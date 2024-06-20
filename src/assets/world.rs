use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::assets::level::LevelAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::traits::{LdtkComponent, LdtkComponentError};
use crate::exports::world_layout::WorldLayout;
use crate::ldtk;

use super::traits::LdtkAssetChildLoader;

#[derive(Asset, Debug, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: I64Vec2,
    pub world_layout: WorldLayout,
    #[reflect(ignore)]
    pub(crate) _project: Handle<ProjectAsset>,
    pub(crate) level_handles: Vec<Handle<LevelAsset>>,
}

impl WorldAsset {
    pub(crate) fn new(
        value: &ldtk::World,
        project: Handle<ProjectAsset>,
        level_handles: Vec<Handle<LevelAsset>>,
    ) -> Self {
        Self {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            world_grid_size: (value.world_grid_width, value.world_grid_height).into(),
            world_layout: value.world_layout.clone().unwrap_or(WorldLayout::Free),
            _project: project,
            level_handles,
        }
    }
}

impl LdtkAssetChildLoader<LevelAsset> for WorldAsset {
    fn children(&self) -> Vec<Handle<LevelAsset>> {
        self.level_handles.clone()
    }
}

impl LdtkAsset for WorldAsset {}

impl LdtkComponent<WorldAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &WorldAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands
            .entity(entity)
            .insert(Name::from(asset.identifier.clone()));

        Ok(())
    }
}

impl LdtkComponent<WorldAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &WorldAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).insert(component);
        Ok(())
    }
}

impl LdtkComponent<WorldAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Transform>,
        _: &WorldAsset,
    ) -> Result<(), LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = Vec3::ZERO;
        } else {
            commands.entity(entity).insert(SpatialBundle::default());
        }
        Ok(())
    }
}
