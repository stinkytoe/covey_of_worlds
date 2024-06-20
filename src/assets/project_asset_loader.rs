use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::reflect::List;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

use crate::assets::level::LevelAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::world::WorldAsset;
use crate::ldtk;
use crate::util::{bevy_color_from_ldtk, ColorParseError};

use super::level::LevelAssetError;

#[derive(Component, Debug, Reflect, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub level_separation: f32,
    pub layer_separation: f32,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            level_separation: 10.0,
            layer_separation: 0.1,
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum ProjectAssetLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    // #[error(transparent)]
    // NewWorldAssetError(#[from] NewWorldAssetError),
    #[error(transparent)]
    LevelAssetError(#[from] LevelAssetError),
    #[error(transparent)]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    // #[error(transparent)]
    // LayerTypeError(#[from] LayerTypeError),
    // #[error(transparent)]
    // NewEntityAssetError(#[from] NewEntityAssetError),
    // #[error(transparent)]
    // LayerDefinitionFromError(#[from] LayerDefinitionFromError),
    // #[error(transparent)]
    // EntityDefinitionFromError(#[from] EntityDefinitionFromError),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("externalRelPath is None when external_levels is true?")]
    ExternalRelPathIsNone,
    // #[error("tile instances in entity type layer!")]
    // NonTileLayerWithTiles,
    #[error("Value is None in a single world project?")]
    ValueMissingInSingleWorld,
    #[error("Layer Instances is None in a non-external levels project?")]
    LayerInstancesIsNone,
    // #[error("Int Grid/Auto Layer should only have auto tiles!")]
    // IntGridWithEntitiesOrGridTiles,
    // #[error("Tiles Layer should only have grid tiles!")]
    // TilesWithAutoLayerOrEntities,
}

#[derive(Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;
    type Settings = ProjectSettings;
    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let asset_path = load_context.path().to_path_buf();

            let self_handle = load_context.load(asset_path.clone());

            info!("Loading LDtk project file: {asset_path:?}");

            let base_directory = asset_path
                .parent()
                .ok_or(ProjectAssetLoaderError::BadProjectDirectory(
                    asset_path.clone(),
                ))?
                .to_path_buf();

            let value: ldtk::LdtkJson = {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let ldtk_worlds = if value.worlds.is_empty() {
                vec![ldtk::World {
                    default_level_height: value
                        .default_level_height
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    default_level_width: value
                        .default_level_width
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    identifier: "World".into(),
                    iid: value.iid.clone(),
                    levels: value.levels,
                    world_grid_height: value
                        .world_grid_height
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    world_grid_width: value
                        .world_grid_width
                        .ok_or(ProjectAssetLoaderError::ValueMissingInSingleWorld)?,
                    world_layout: value.world_layout,
                }]
            } else {
                value.worlds
            };

            let mut world_handles = Vec::new();
            for ldtk_world in ldtk_worlds.iter() {
                let mut level_handles = Vec::new();
                for ldtk_level in ldtk_world.levels.iter() {
                    // let layers = if value.external_levels {
                    //     let level_path = ldtk_level
                    //         .external_rel_path
                    //         .as_ref()
                    //         .ok_or(ProjectAssetLoaderError::ExternalRelPathIsNone)?;
                    //     let level_path = Path::new(&level_path);
                    //     let level_path = _ldtk_path_to_asset_path(&base_directory, level_path);
                    //     let bytes = load_context.read_asset_bytes(level_path).await?;
                    //     let level_json: ldtk::Level = serde_json::from_slice(&bytes)?;
                    //     level_json.layer_instances.unwrap()
                    // } else {
                    //     ldtk_level
                    //         .layer_instances
                    //         .as_ref()
                    //         .ok_or(ProjectAssetLoaderError::LayerInstancesIsNone)?
                    //         .to_vec()
                    // };
                    let label = format!("{}/{}", ldtk_world.identifier, ldtk_level.identifier);
                    let level_asset = LevelAsset::new(
                        ldtk_level,
                        self_handle.clone(),
                        settings.level_separation,
                        Vec::default(),
                    )?;

                    level_handles
                        .push(load_context.add_loaded_labeled_asset(label, level_asset.into()));
                }

                let label = ldtk_world.identifier.clone();
                let world_asset =
                    WorldAsset::new(ldtk_world, self_handle.clone(), level_handles).into();

                world_handles.push(load_context.add_loaded_labeled_asset(label, world_asset));
            }

            Ok(ProjectAsset {
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                external_levels: value.external_levels,
                iid: value.iid,
                json_version: value.json_version.clone(),
                self_handle,
                world_handles,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
