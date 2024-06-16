use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::assets::world::LevelsToLoad;
use crate::assets::world::WorldAsset;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::components::traits::LdtkComponentError;

#[derive(Asset, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,
    #[reflect(ignore)]
    pub(crate) self_handle: Handle<ProjectAsset>,
    pub(crate) world_handles: Vec<(String, String, Handle<WorldAsset>)>,
    pub(crate) worlds_to_load: WorldsToLoad,
}

#[derive(Reflect)]
pub enum WorldsToLoad {
    None,
    ByIid(HashMap<String, LevelsToLoad>),
    All(LevelsToLoad),
}

impl Default for WorldsToLoad {
    fn default() -> Self {
        Self::All(LevelsToLoad::default())
    }
}

impl LdtkAssetChildLoader<WorldAsset> for ProjectAsset {
    fn children(&self) -> Vec<Handle<WorldAsset>> {
        match &self.worlds_to_load {
            WorldsToLoad::None => vec![],
            WorldsToLoad::ByIid(ids) => self
                .world_handles
                .iter()
                .filter(|(_, iid, _)| ids.contains_key(iid))
                .map(|(_, _, handle)| handle.clone())
                .collect(),
            WorldsToLoad::All(_) => self
                .world_handles
                .iter()
                .map(|(_, _, handle)| handle.clone())
                .collect(),
        }
    }
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
