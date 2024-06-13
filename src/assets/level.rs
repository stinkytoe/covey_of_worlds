use bevy::prelude::*;

use super::project::ProjectAsset;

#[derive(Asset, Debug, Reflect)]
pub struct LevelAsset {
    pub bg_color: Color,
    // pub bg_pos: Option<LevelBackgroundPosition>,
    // pub neighbours: Neighbours,
    // pub bg_rel_path: Option<PathBuf>,
    // pub field_instances: FieldInstances,
    pub identifier: String,
    pub iid: String,
    pub size: Vec2,
    // (worldX, worldY, and worldDepth)
    // In Bevy coordinate system, not necessarily the same as Bevy transform!
    pub world_location: Vec3,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
    // pub layer_assets_by_identifier: HashMap<String, Handle<LayerAsset>>,
    // pub layer_assets_by_iid: HashMap<String, Handle<LayerAsset>>,
}
