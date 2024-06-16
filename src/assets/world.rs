use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::level::LevelAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::traits::LdtkComponent;
use crate::exports::world_layout::WorldLayout;
use crate::ldtk;

use super::level::LayersToLoad;

#[derive(Asset, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: I64Vec2,
    pub world_layout: WorldLayout,
    #[reflect(ignore)]
    pub(crate) _project: Handle<ProjectAsset>,
    pub(crate) level_handles: Vec<(String, String, Handle<LevelAsset>)>,
    pub(crate) levels_to_load: LevelsToLoad,
}

impl WorldAsset {
    pub(crate) fn new(
        value: &ldtk::World,
        project: Handle<ProjectAsset>,
        level_handles: Vec<(String, String, Handle<LevelAsset>)>,
        levels_to_load: LevelsToLoad,
    ) -> Self {
        Self {
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            world_grid_size: (value.world_grid_width, value.world_grid_height).into(),
            world_layout: value.world_layout.clone().unwrap_or(WorldLayout::Free),
            _project: project,
            level_handles,
            levels_to_load,
        }
    }
}

#[derive(Reflect)]
pub enum LevelsToLoad {
    None,
    ByIid(HashMap<String, LayersToLoad>),
    // #[default = LayerChildrenToLoad::default()]
    All(LayersToLoad),
}

impl Default for LevelsToLoad {
    fn default() -> Self {
        Self::All(LayersToLoad::default())
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
