use bevy::prelude::*;
use thiserror::Error;

use crate::{
    components::{field_instances::FieldInstances, iid::Iid, traits::LdtkComponent},
    exports::{
        field_instance::{FieldInstance, FieldInstanceValueParseError},
        level_background_position::LevelBackgroundPosition,
        neighbors::{Neighbour, NeighbourError},
    },
    ldtk,
    util::{bevy_color_from_ldtk, ColorParseError},
};

use super::{
    layer::LayerAsset,
    project::ProjectAsset,
    traits::{LdtkAsset, LdtkAssetChildLoader},
};

#[derive(Debug, Error)]
pub enum LevelAssetError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    FieldInstanceValueParseErrpr(#[from] FieldInstanceValueParseError),
    #[error(transparent)]
    NeighbourError(#[from] NeighbourError),
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
    pub world_location: Vec3,
    #[reflect(ignore)]
    pub(crate) project: Handle<ProjectAsset>,
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
            bg_pos: value
                .bg_pos
                .as_ref()
                .map(|bg_pos| LevelBackgroundPosition::new(bg_pos)),
            neighbours: value
                .neighbours
                .iter()
                .map(|neighbour| Neighbour::new(neighbour))
                .collect::<Result<_, _>>()?,
            bg_rel_path: value.bg_rel_path.clone(),
            field_instances: value
                .field_instances
                .iter()
                .map(|field_instance| FieldInstance::new(field_instance))
                .collect::<Result<_, _>>()?,
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            world_location: (
                value.world_x as f32,
                -value.world_y as f32,
                (value.world_depth as f32) * level_separation,
            )
                .into(),
            project,
            layer_handles,
        })
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
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = asset.world_location;
        } else {
            commands
                .entity(entity)
                .insert(SpatialBundle::from_transform(Transform::from_translation(
                    asset.world_location,
                )));
        }
        Ok(())
    }
}
