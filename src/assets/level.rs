use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::sprite::Anchor;
use thiserror::Error;

use crate::assets::layer::LayerAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::exports::field_instance::{FieldInstance, FieldInstanceValueParseError};
use crate::exports::level_background_position::LevelBackgroundPosition;
use crate::exports::neighbors::Neighbour;
use crate::exports::neighbors::NeighbourError;
use crate::ldtk;
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
    #[reflect(ignore)]
    pub(crate) _project: Handle<ProjectAsset>,
    #[reflect(ignore)]
    pub(crate) layer_handles: Vec<Handle<LayerAsset>>,
}

impl LevelAsset {
    pub(crate) fn new(
        value: &ldtk::Level,
        project: Handle<ProjectAsset>,
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
            _project: project,
            layer_handles,
        })
    }

    pub(crate) fn level_bg_system(
        mut commands: Commands,
        mut events: EventReader<LdtkAssetLoadEvent<LevelAsset>>,
        layer_assets: Res<Assets<LevelAsset>>,
        mut image_assets: ResMut<Assets<Image>>,
    ) -> Result<(), LevelAssetError> {
        for LdtkAssetLoadEvent { entity, handle } in events.read() {
            let asset = layer_assets.get(handle).ok_or(LevelAssetError::BadHandle)?;
            let color = asset.bg_color.as_rgba_u8();

            let bg_image = Image::new_fill(
                Extent3d {
                    width: asset.size.x as u32,
                    height: asset.size.y as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &color,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );

            let image_handle = image_assets.add(bg_image);

            commands.entity(*entity).insert((
                image_handle,
                Sprite {
                    // color: todo!(),
                    // flip_x: todo!(),
                    // flip_y: todo!(),
                    // custom_size: todo!(),
                    // rect: todo!(),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
            ));
        }
        Ok(())
    }
}

impl LdtkAssetChildLoader<LayerAsset> for LevelAsset {
    fn children(&self) -> Vec<Handle<LayerAsset>> {
        self.layer_handles.clone()
    }
}

impl LdtkAsset for LevelAsset {}

impl LdtkComponent<LevelAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LevelAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands
            .entity(entity)
            .insert(Name::from(asset.identifier.clone()));

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
        commands.entity(entity).insert(Iid(asset.iid.clone()));

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
        info!("assigning {}", asset.location);
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = asset.location;
        } else {
            commands
                .entity(entity)
                .insert(SpatialBundle::from_transform(Transform::from_translation(
                    asset.location,
                )));
        }
        Ok(())
    }
}
