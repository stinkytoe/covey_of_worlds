use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::components::traits::LdtkComponentError;

use super::traits::LdtkAssetChildLoader;
use super::world::WorldAsset;

#[derive(Asset, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,
    #[reflect(ignore)]
    pub(crate) self_handle: Handle<ProjectAsset>,
    pub(crate) world_handles: Vec<(String, String, Handle<WorldAsset>)>,
    pub(crate) worlds_to_load: ProjectChildrenToLoad,
}

#[derive(Component, Reflect)]
pub enum ProjectChildrenToLoad {
    None,
    ByIid(HashMap<String, WorldChildrenToLoad>),
    All(WorldChildrenToLoad),
}

impl LdtkAssetChildLoader<WorldAsset> for ProjectAsset {
    fn children(&self) -> Vec<Handle<WorldAsset>> {
        match &self.worlds_to_load {
            ProjectChildrenToLoad::None => vec![],
            ProjectChildrenToLoad::ByIid(ids) => self
                .world_handles
                .iter()
                .filter(|(_, iid, _)| ids.contains_key(iid))
                .map(|(_, _, handle)| handle.clone())
                .collect(),
            ProjectChildrenToLoad::All(_) => self
                .world_handles
                .iter()
                .map(|(_, _, handle)| handle.clone())
                .collect(),
        }
    }
}

impl Default for ProjectChildrenToLoad {
    fn default() -> Self {
        Self::All(WorldChildrenToLoad::default())
    }
}

#[derive(Reflect)]
pub enum WorldChildrenToLoad {
    None,
    ByIid(HashMap<String, LevelChildrenToLoad>),
    // #[default = LayerChildrenToLoad::default()]
    All(LevelChildrenToLoad),
}

impl Default for WorldChildrenToLoad {
    fn default() -> Self {
        Self::All(LevelChildrenToLoad::default())
    }
}

#[derive(Reflect)]
pub enum LevelChildrenToLoad {
    None,
    ByIid(HashMap<String, LayerChildrenToLoad>),
    All(LayerChildrenToLoad),
}

impl Default for LevelChildrenToLoad {
    fn default() -> Self {
        Self::All(LayerChildrenToLoad::default())
    }
}

#[derive(Default, Reflect)]
pub enum LayerChildrenToLoad {
    None,
    ByIid(Vec<String>),
    #[default]
    All,
}

impl LdtkAsset for ProjectAsset {}

impl LdtkComponent<ProjectAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        let name = asset
            .self_handle
            .path()
            .ok_or(LdtkComponentError::BadPath)?
            .to_string();
        let component = Name::new(name);
        commands.entity(entity).insert(component);
        Ok(())
    }
}

impl LdtkComponent<ProjectAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Iid>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).insert(component);
        Ok(())
    }
}

impl LdtkComponent<ProjectAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Transform>,
        _: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = Vec3::ZERO;
        } else {
            commands.entity(entity).insert(SpatialBundle::default());
        }
        Ok(())
    }
}
