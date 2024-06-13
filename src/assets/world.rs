use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::level::LevelAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;

#[derive(Asset, Reflect)]
pub struct WorldAsset {
    pub identifier: String,
    pub iid: String,
    pub world_grid_size: Vec2,
    // pub world_layout: WorldLayout,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
    pub level_assets: HashMap<String, Handle<LevelAsset>>,
}

impl LdtkAsset for WorldAsset {}

// impl LdtkAssetComponent<WorldAsset> for Name {
//     fn try_from_ldtk_asset(asset: &WorldAsset) -> Result<Self, LdtkAssetComponentError> {
//         Ok(Name::from(asset.identifier.clone()))
//     }
// }
//
// impl LdtkAssetComponent<WorldAsset> for Iid {
//     fn try_from_ldtk_asset(asset: &WorldAsset) -> Result<Self, LdtkAssetComponentError> {
//         Ok(Iid(asset.iid.clone()))
//     }
// }
