use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use thiserror::Error;

use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::tileset_rectangle::TilesetRectangle;
use crate::components::traits::LdtkComponent;
use crate::exports::field_instance::FieldInstance;
use crate::exports::field_instance::FieldInstanceValueParseError;
use crate::ldtk;
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
    pub location: Vec2,
    // #[reflect(ignore)]
    // pub project: Handle<ProjectAsset>,
}

impl EntityAsset {
    pub(crate) fn new(
        value: &ldtk::EntityInstance,
        // project: Handle<ProjectAsset>,
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
            location: (value.px[0] as f32, -value.px[1] as f32).into(),
            // project,
        })
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
                commands.entity(entity).remove::<Sprite>();
                commands.entity(entity).remove::<Handle<Image>>();
            }
        };

        commands.entity(entity);

        Ok(())
    }
}
