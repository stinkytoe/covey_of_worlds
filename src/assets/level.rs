use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::sprite::Anchor;
use image::imageops::crop_imm;
use image::imageops::overlay;
use image::imageops::resize;
use image::imageops::FilterType;
use image::RgbaImage;
use thiserror::Error;

use crate::assets::layer::LayerAsset;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::exports::field_instance::{FieldInstance, FieldInstanceValueParseError};
use crate::exports::level_background_position::LevelBackgroundPosition;
use crate::exports::neighbors::Neighbour;
use crate::exports::neighbors::NeighbourError;
use crate::ldtk;
use crate::system_params::project::LdtkProjectCommands;
use crate::system_params::project::LdtkProjectCommandsEx;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

use super::traits::LdtkAssetLoadEvent;

#[derive(Debug, Error)]
pub enum LevelAssetError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    FieldInstanceValueParseErrpr(#[from] FieldInstanceValueParseError),
    #[error(transparent)]
    NeighbourError(#[from] NeighbourError),
    #[error("Bad handle?")]
    BadHandle,
    #[error("bg_pos is Some but bg_rel_path is none?")]
    BgPosWithBgRelPathNone,
    #[error("bg_pos is none but bg_rel_path Some?")]
    BgPosNoneWithBgRelPath,
    #[error("bg_rel_path not found!")]
    BgRelPathNotFound,
    #[error("Bad Project Iid!")]
    BadProjectIid,
}

#[derive(Asset, Debug, Reflect)]
pub struct LevelAsset {
    pub bg_color: Color,
    pub bg_pos: Option<LevelBackgroundPosition>,
    pub neighbours: Vec<Neighbour>,
    pub bg_rel_path: Option<String>,
    pub field_instances: Vec<FieldInstance>,
    pub identifier: String,
    pub iid: String,
    pub size: Vec2,
    // (worldX, worldY, and worldDepth)
    // In Bevy coordinate system, not necessarily the same as Bevy transform!
    pub location: Vec3,
    pub project_iid: String,
    // #[reflect(ignore)]
    pub(crate) layer_handles: Vec<Handle<LayerAsset>>,
}

impl LevelAsset {
    pub(crate) fn new(
        value: &ldtk::Level,
        project_iid: String,
        level_separation: f32,
        layer_handles: Vec<Handle<LayerAsset>>,
    ) -> Result<Self, LevelAssetError> {
        Ok(Self {
            bg_color: bevy_color_from_ldtk(&value.bg_color)?,
            bg_pos: value.bg_pos.as_ref().map(LevelBackgroundPosition::new),
            neighbours: value
                .neighbours
                .iter()
                .map(Neighbour::new)
                .collect::<Result<_, _>>()?,
            bg_rel_path: value.bg_rel_path.clone(),
            field_instances: value
                .field_instances
                .iter()
                .map(FieldInstance::new)
                .collect::<Result<_, _>>()?,
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            location: (
                value.world_x as f32,
                -value.world_y as f32,
                (value.world_depth as f32) * level_separation,
            )
                .into(),
            project_iid,
            layer_handles,
        })
    }

    pub(crate) fn level_bg_system(
        mut commands: Commands,
        mut events: EventReader<LdtkAssetLoadEvent<LevelAsset>>,
        // project_assets: Res<Assets<ProjectAsset>>,
        project_commands: LdtkProjectCommands,
        level_assets: Res<Assets<LevelAsset>>,
        mut image_assets: ResMut<Assets<Image>>,
    ) -> Result<(), LevelAssetError> {
        for LdtkAssetLoadEvent { entity, handle } in events.read() {
            let level_asset = level_assets.get(handle).ok_or(LevelAssetError::BadHandle)?;

            // let project_asset = project_assets
            //     .get(&level_asset.project)
            //     .ok_or(LevelAssetError::BadHandle)?;
            let project_asset = project_commands
                .iter()
                .with_iid(&level_asset.project_iid)
                .ok_or(LevelAssetError::BadProjectIid)?;

            match (
                level_asset.bg_pos.as_ref(),
                level_asset.bg_rel_path.as_ref(),
            ) {
                (None, Some(_)) => return Err(LevelAssetError::BgPosNoneWithBgRelPath),
                (Some(_), None) => return Err(LevelAssetError::BgPosWithBgRelPathNone),
                (None, None) => {
                    let color = level_asset.bg_color.as_rgba_u8();

                    let background_image = Image::new_fill(
                        Extent3d {
                            width: level_asset.size.x as u32,
                            height: level_asset.size.y as u32,
                            depth_or_array_layers: 1,
                        },
                        TextureDimension::D2,
                        &color,
                        TextureFormat::Rgba8UnormSrgb,
                        RenderAssetUsages::default(),
                    );

                    let image_handle = image_assets.add(background_image);

                    commands.entity(*entity).try_insert((
                        image_handle,
                        Sprite {
                            anchor: Anchor::TopLeft,
                            ..default()
                        },
                    ));
                }
                (Some(bg_pos), Some(bg_rel_path)) => {
                    let background_handle = project_asset
                        .background_assets
                        .get(bg_rel_path)
                        .ok_or(LevelAssetError::BgRelPathNotFound)?;

                    let background_image = image_assets
                        .get(background_handle)
                        .ok_or(LevelAssetError::BadHandle)?
                        .clone()
                        .try_into_dynamic()
                        // FIXME: remove this expect and return a proper Err(..) value
                        // once bevy 0.14 drops
                        .expect("a dynamic image");

                    let cropped = crop_imm(
                        &background_image,
                        bg_pos.crop_top_left.x as u32,
                        bg_pos.crop_top_left.y as u32,
                        (bg_pos.crop_top_left.x + bg_pos.crop_bottom_right.x) as u32,
                        (bg_pos.crop_top_left.y + bg_pos.crop_bottom_right.y) as u32,
                    );

                    let new_size = ((bg_pos.crop_bottom_right - bg_pos.crop_top_left)
                        * bg_pos.scale)
                        .as_uvec2();

                    let scaled = resize(
                        &cropped.to_image(),
                        new_size.x,
                        new_size.y,
                        FilterType::Gaussian,
                    );

                    let color = level_asset.bg_color.as_rgba_u8();

                    let mut background_color =
                        RgbaImage::new(level_asset.size.x as u32, level_asset.size.y as u32);

                    for (_, _, p) in background_color.enumerate_pixels_mut() {
                        *p = image::Rgba(color);
                    }

                    let dynamic_image = image::DynamicImage::from(scaled);

                    overlay(
                        &mut background_color,
                        &dynamic_image,
                        bg_pos.top_left.x as i64,
                        bg_pos.top_left.y as i64,
                    );

                    let background_image = Image::from_dynamic(
                        background_color.into(),
                        true,
                        RenderAssetUsages::default(),
                    );

                    let background_handle = image_assets.add(background_image);

                    commands.entity(*entity).try_insert((
                        background_handle,
                        Sprite {
                            anchor: Anchor::TopLeft,
                            ..default()
                        },
                    ));
                }
            };
        }
        Ok(())
    }
}

impl LdtkAssetChildLoader<LayerAsset> for LevelAsset {
    fn children(&self) -> Vec<Handle<LayerAsset>> {
        self.layer_handles.clone()
    }
}

impl LdtkAsset for LevelAsset {
    fn iid(&self) -> String {
        self.iid.clone()
    }
}

impl LdtkComponent<LevelAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LevelAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands
            .entity(entity)
            .try_insert(Name::from(asset.identifier.clone()));

        Ok(())
    }
}

impl LdtkComponent<LevelAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LevelAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands.entity(entity).try_insert(Iid(asset.iid.clone()));

        Ok(())
    }
}

impl LdtkComponent<LevelAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Self>,
        asset: &LevelAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = asset.location;
        } else {
            commands
                .entity(entity)
                .try_insert(SpatialBundle::from_transform(Transform::from_translation(
                    asset.location,
                )));
        }
        Ok(())
    }
}
