use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::world_layout::WorldLayout;
use crate::ldtk;
use crate::{assets::level::LevelAsset, components::traits::LdtkComponent};

use super::project::{LevelChildrenToLoad, WorldChildrenToLoad};

#[derive(Asset, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: I64Vec2,
    pub world_layout: WorldLayout,
    #[reflect(ignore)]
    pub(crate) project: Handle<ProjectAsset>,
    pub(crate) level_handles: Vec<(String, String, Handle<LevelAsset>)>,
    pub(crate) levels_to_load: WorldChildrenToLoad,
}

impl WorldAsset {
    pub fn new(
        value: &ldtk::World,
        project: Handle<ProjectAsset>,
        level_handles: Vec<(String, String, Handle<LevelAsset>)>,
        levels_to_load: WorldChildrenToLoad,
    ) -> Self {
        Self {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            world_grid_size: (value.world_grid_width, value.world_grid_height).into(),
            world_layout: value.world_layout.clone().unwrap_or(WorldLayout::Free),
            project,
            level_handles,
            levels_to_load,
        }
    }
}

impl LdtkAsset for WorldAsset {}

impl LdtkComponent<WorldAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Self>,
        asset: &WorldAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands
            .entity(entity)
            .insert(Name::from(asset.identifier.clone()));

        Ok(())
    }
    //     fn try_from_ldtk_asset(asset: &WorldAsset) -> Result<Self, LdtkAssetComponentError> {
    //         Ok(Name::from(asset.identifier.clone()))
    //     }
}
//
// impl LdtkAssetComponent<WorldAsset> for Iid {
//     fn try_from_ldtk_asset(asset: &WorldAsset) -> Result<Self, LdtkAssetComponentError> {
//         Ok(Iid(asset.iid.clone()))
//     }
// }
