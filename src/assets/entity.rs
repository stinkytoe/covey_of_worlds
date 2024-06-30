use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::tileset_rectangle::TilesetRectangle;
use crate::components::traits::LdtkComponent;
use crate::exports::field_instance::FieldInstance;
use crate::exports::field_instance::FieldInstanceValueParseError;
use crate::ldtk;
use crate::system_params::project::LdtkProjectCommands;
use crate::system_params::project::LdtkProjectCommandsEx;
use crate::util::bevy_anchor_from_ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::AnchorIntoError;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum EntityAssetError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    AnchorIntoError(#[from] AnchorIntoError),
    #[error(transparent)]
    FieldInstanceValueError(#[from] FieldInstanceValueParseError),
    #[error("One world coord is Some(...) and the other is None!")]
    WorldCoordMixedOption,
    #[error("Bad handle?")]
    BadHandle,
    #[error("Bad Iid?")]
    BadIid,
    #[error("Bad Tileset Uid?")]
    BadTilesetUid,
    #[error("Missing Tileset Path!")]
    MissingTilesetPath,
    #[error("Bad Tileset Path!")]
    BadTilesetPath,
}

#[derive(Asset, Debug, Reflect)]
pub struct EntityAsset {
    pub grid: I64Vec2,
    pub identifier: String,
    pub anchor: Anchor,
    pub smart_color: Color,
    pub tags: Vec<String>,
    pub tile: Option<TilesetRectangle>,
    pub world_location: Option<Vec2>,
    pub def_uid: i64,
    pub field_instances: Vec<FieldInstance>,
    pub size: Vec2,
    pub iid: String,
    pub location: Vec3,
    // #[reflect(ignore)]
    pub(crate) project_iid: String,
}

impl EntityAsset {
    pub(crate) fn new(
        value: &ldtk::EntityInstance,
        project_iid: String,
    ) -> Result<Self, EntityAssetError> {
        Ok(Self {
            grid: (value.grid[0], value.grid[1]).into(),
            identifier: value.identifier.clone(),
            anchor: bevy_anchor_from_ldtk(&value.pivot)?,
            smart_color: bevy_color_from_ldtk(&value.smart_color)?,
            tags: value.tags.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::new),
            world_location: match (value.world_x, value.world_y) {
                (None, None) => None,
                (Some(world_x), Some(world_y)) => Some((world_x as f32, world_y as f32).into()),
                (None, Some(_)) | (Some(_), None) => {
                    return Err(EntityAssetError::WorldCoordMixedOption)
                }
            },
            def_uid: value.def_uid,
            field_instances: value
                .field_instances
                .iter()
                .map(FieldInstance::new)
                .collect::<Result<_, _>>()?,
            size: (value.width as f32, value.height as f32).into(),
            iid: value.iid.clone(),
            location: (value.px[0] as f32, -value.px[1] as f32, 0.0).into(),
            project_iid,
        })
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn entity_tile_system(
        mut commands: Commands,
        project_commands: LdtkProjectCommands,
        mut query: Query<
            (
                Entity,
                &Handle<EntityAsset>,
                &TilesetRectangle,
                Option<&mut Sprite>,
            ),
            Changed<TilesetRectangle>,
        >,
        mut removed_tile: RemovedComponents<TilesetRectangle>,
        entity_assets: Res<Assets<EntityAsset>>,
    ) -> Result<(), EntityAssetError> {
        for (entity, handle, tile, mut sprite) in query.iter_mut() {
            let entity_asset = entity_assets
                .get(handle)
                .ok_or(EntityAssetError::BadHandle)?;

            let project_asset = project_commands
                .iter()
                .with_iid(&entity_asset.project_iid)
                .ok_or(EntityAssetError::BadIid)?;

            let tileset_definition = project_asset
                .tileset_defs
                .get(&tile.tileset_uid)
                .ok_or(EntityAssetError::BadTilesetUid)?;

            let custom_size = Some(tile.size);

            let rect = Some(Rect::from_corners(tile.location, tile.location + tile.size));

            let anchor = entity_asset.anchor;

            let texture = project_asset
                .tileset_assets
                .get(
                    tileset_definition
                        .rel_path
                        .as_ref()
                        .ok_or(EntityAssetError::MissingTilesetPath)?,
                )
                .ok_or(EntityAssetError::BadTilesetPath)?
                .clone();

            if let Some(mut sprite) = sprite {
                sprite.custom_size = custom_size;
                sprite.rect = rect;
                sprite.anchor = anchor;
            } else {
                commands.entity(entity).insert(Sprite {
                    color: Color::WHITE,
                    custom_size,
                    rect,
                    anchor,
                    ..default()
                });
            };

            commands.entity(entity).insert(texture);
        }

        removed_tile.read().for_each(|entity| {
            commands
                .entity(entity)
                .remove::<Handle<Image>>()
                .remove::<Sprite>();
        });
        Ok(())
    }
}

impl LdtkAsset for EntityAsset {
    fn iid(&self) -> String {
        self.iid.clone()
    }
}

impl LdtkComponent<EntityAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        let component = Name::new(asset.identifier.clone());
        commands.entity(entity).try_insert(component);
        Ok(())
    }
}

impl LdtkComponent<EntityAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Iid>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).try_insert(component);
        Ok(())
    }
}

impl LdtkComponent<EntityAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Self>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        if let Ok(_transform) = query.get_mut(entity) {
            // transform.translation = asset.location;
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

impl LdtkComponent<EntityAsset> for TilesetRectangle {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut TilesetRectangle>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        match asset.tile.as_ref() {
            Some(tile) => {
                commands.entity(entity).try_insert(tile.clone());
            }
            None => {
                commands.entity(entity).remove::<TilesetRectangle>();
                //     commands.entity(entity).remove::<Sprite>();
                //     commands.entity(entity).remove::<Handle<Image>>();
            }
        };

        commands.entity(entity);

        Ok(())
    }
}
