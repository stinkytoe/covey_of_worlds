use std::path::PathBuf;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use thiserror::Error;

use crate::{
    ldtk,
    util::{bevy_color_from_ldtk, ColorParseError},
};

use super::project::{ProjectAsset, ProjectChildrenToLoad};

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
    // #[error(transparent)]
    // NewLevelAssetError(#[from] NewLevelAssetError),
    // #[error(transparent)]
    // ReadAssetBytesError(#[from] ReadAssetBytesError),
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
    // #[error("externalRelPath is None when external levels is true?")]
    // ExternalRelPathIsNone,
    // #[error("tile instances in entity type layer!")]
    // NonTileLayerWithTiles,
    // #[error("Value is None in a single world project?")]
    // ValueMissingInSingleWorld,
    // #[error("Layer Instances is None in a non-external levels project?")]
    // LayerInstancesIsNone,
    // #[error("Int Grid/Auto Layer should only have auto tiles!")]
    // IntGridWithEntitiesOrGridTiles,
    // #[error("Tiles Layer should only have grid tiles!")]
    // TilesWithAutoLayerOrEntities,
}

#[derive(Default)]
pub(crate) struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    type Asset = ProjectAsset;
    type Settings = ();
    type Error = ProjectAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
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

            Ok(ProjectAsset {
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                external_levels: value.external_levels,
                iid: value.iid.clone(),
                json_version: value.json_version.clone(),
                self_handle,
                world_handles: Vec::default(),
                worlds_to_load: ProjectChildrenToLoad::default(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
