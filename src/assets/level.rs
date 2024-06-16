use bevy::{prelude::*, utils::HashMap};

use crate::{
    components::field_instances::FieldInstances,
    exports::{level_background_position::LevelBackgroundPosition, neighbors::Neighbour},
};

use super::{
    layer::{EntitiesToLoad, LayerAsset},
    project::ProjectAsset,
    traits::LdtkAsset,
};

#[derive(Asset, Debug, Reflect)]
pub struct LevelAsset {
    pub bg_color: Color,
    pub bg_pos: Option<LevelBackgroundPosition>,
    pub neighbours: Vec<Neighbour>,
    pub bg_rel_path: Option<String>,
    pub field_instances: FieldInstances,
    pub identifier: String,
    pub iid: String,
    pub size: Vec2,
    // (worldX, worldY, and worldDepth)
    // In Bevy coordinate system, not necessarily the same as Bevy transform!
    pub world_location: Vec3,
    #[reflect(ignore)]
    pub(crate) _project: Handle<ProjectAsset>,
    pub(crate) layer_handles: Vec<(String, String, Handle<LayerAsset>)>,
    pub(crate) layers_to_load: LayersToLoad,
}

#[derive(Debug, Reflect)]
pub enum LayersToLoad {
    None,
    ByIid(HashMap<String, EntitiesToLoad>),
    All(EntitiesToLoad),
}

impl Default for LayersToLoad {
    fn default() -> Self {
        Self::All(EntitiesToLoad::default())
    }
}

impl LdtkAsset for LevelAsset {}
