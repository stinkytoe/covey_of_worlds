use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use futures_lite::future;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use crate::assets::entity::EntityAsset;
use crate::assets::entity::EntityAssetError;
use crate::assets::layer::LayerAsset;
use crate::assets::layer::LayerAssetError;
use crate::assets::layer::LayerType;
use crate::assets::level::LevelAsset;
use crate::assets::level::LevelAssetError;
use crate::assets::project::ProjectAsset;
use crate::assets::world::WorldAsset;
use crate::defs::entity_definition::EntityDefinition;
use crate::defs::entity_definition::EntityDefinitionFromError;
use crate::defs::enum_definition::EnumDefinition;
use crate::defs::layer_definition::LayerDefinition;
use crate::defs::layer_definition::LayerDefinitionFromError;
use crate::defs::tileset_definition::TilesetDefinition;
use crate::exports::tile_instance::TileInstance;
use crate::ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_asset_path;
use crate::util::ColorParseError;

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
    #[error(transparent)]
    LevelAssetError(#[from] LevelAssetError),
    #[error(transparent)]
    LayerTypeError(#[from] LayerAssetError),
    #[error(transparent)]
    EntityAssetError(#[from] EntityAssetError),
    #[error(transparent)]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error(transparent)]
    LayerDefinitionFromError(#[from] LayerDefinitionFromError),
    #[error(transparent)]
    EntityDefinitionFromError(#[from] EntityDefinitionFromError),
    #[error("Could not get project directory? {0}")]
    BadProjectDirectory(PathBuf),
    #[error("externalRelPath is None when external_levels is true?")]
    ExternalRelPathIsNone,
    #[error("tile instances in entity type layer!")]
    EntityLayerWithTiles,
    #[error("Value is None in a single world project?")]
    ValueMissingInSingleWorld,
    #[error("Layer Instances is None in a non-external levels project?")]
    LayerInstancesIsNone,
    #[error("Int Grid/Auto Layer should only have auto tiles!")]
    IntGridWithEntitiesOrGridTiles,
    #[error("Tiles Layer should only have grid tiles!")]
    TilesWithAutoLayerOrEntities,
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

            let world_handles = ldtk_worlds
                .iter()
                .map(|ldtk_world| {
                    let level_handles = ldtk_world
                        .levels
                        .iter()
                        .map(|ldtk_level| {
                            let layer_handles = if value.external_levels {
                                let level_path = ldtk_level
                                    .external_rel_path
                                    .as_ref()
                                    .ok_or(ProjectAssetLoaderError::ExternalRelPathIsNone)?;
                                let level_path = Path::new(&level_path);
                                let level_path =
                                    ldtk_path_to_asset_path(&base_directory, level_path);
                                let bytes_result: Result<Vec<u8>, ProjectAssetLoaderError> =
                                    future::block_on(async {
                                        Ok(load_context.read_asset_bytes(level_path).await?)
                                    });
                                let level_json: ldtk::Level =
                                    serde_json::from_slice(&bytes_result?)?;
                                level_json
                                    .layer_instances
                                    .ok_or(ProjectAssetLoaderError::LayerInstancesIsNone)?
                            } else {
                                ldtk_level
                                    .layer_instances
                                    .as_ref()
                                    .ok_or(ProjectAssetLoaderError::LayerInstancesIsNone)?
                                    .to_vec()
                            }
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(index, ldtk_layer)| {
                                let layer_type = LayerType::new(&ldtk_layer.layer_instance_type)?;
                                let (tiles, entity_handles) = match layer_type {
                                    LayerType::IntGrid | LayerType::Autolayer => {
                                            if !ldtk_layer.grid_tiles.is_empty() || !ldtk_layer.entity_instances.is_empty()  {
                                                return Err(ProjectAssetLoaderError::IntGridWithEntitiesOrGridTiles);
                                            }

                                            (ldtk_layer.auto_layer_tiles.iter().map(TileInstance::new).collect(), vec![])
                                        },
                                    LayerType::Tiles => {
                                            if !ldtk_layer.auto_layer_tiles.is_empty()|| !ldtk_layer.entity_instances.is_empty()   {
                                                return Err(ProjectAssetLoaderError::TilesWithAutoLayerOrEntities);
                                            }

                                            (ldtk_layer.grid_tiles.iter().map(TileInstance::new).collect(), vec![])
                                        },

                                    LayerType::Entities => {
                                            if !ldtk_layer.auto_layer_tiles.is_empty() || !ldtk_layer.grid_tiles.is_empty() {
                                                return Err(ProjectAssetLoaderError::EntityLayerWithTiles)
                                            }

                                            let entity_assets = ldtk_layer.entity_instances
                                                .iter()
                                                .map(|ldtk_entity| {
                                                    let label = format!("{}/{}/{}/{}", ldtk_world.identifier, ldtk_level.identifier, ldtk_layer.identifier, ldtk_entity.identifier);
                                                    let asset = EntityAsset::new(ldtk_entity, self_handle.clone())?;
                                                    Ok(load_context.add_loaded_labeled_asset(label, asset.into()))
                                                })
                                                .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?;

                                            (vec![], entity_assets)
                                        },
                                };

                                let label = format!(
                                    "{}/{}/{}",
                                    ldtk_world.identifier,
                                    ldtk_level.identifier,
                                    ldtk_layer.identifier
                                );
                                let asset = LayerAsset::new(
                                    ldtk_layer,
                                    self_handle.clone(),
                                    index,
                                    layer_type,
                                    tiles,
                                    entity_handles,
                                    settings.layer_separation,
                                )?;
                                Ok(load_context.add_loaded_labeled_asset(label, asset.into()))
                            })
                            .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?;

                            let label =
                                format!("{}/{}", ldtk_world.identifier, ldtk_level.identifier);
                            let asset = LevelAsset::new(
                                ldtk_level,
                                self_handle.clone(),
                                settings.level_separation,
                                layer_handles,
                            )?;
                            Ok(load_context.add_loaded_labeled_asset(label, asset.into()))
                        })
                        .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?;

                    let label = ldtk_world.identifier.clone();
                    let asset = WorldAsset::new(ldtk_world, self_handle.clone(), level_handles);
                    Ok(load_context.add_loaded_labeled_asset(label, asset.into()))
                })
                .collect::<Result<Vec<_>, ProjectAssetLoaderError>>()?;

            let background_assets = ldtk_worlds
                .iter()
                .flat_map(|world| world.levels.iter())
                .filter_map(|level| level.bg_rel_path.as_ref())
                .map(|ldtk_path| {
                    let asset_path = Path::new(&ldtk_path);
                    let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
                    let asset_handle = load_context.load(asset_path);
                    (ldtk_path.clone(), asset_handle)
                })
                .collect();

            let tileset_assets = value
                .defs
                .tilesets
                .iter()
                .filter_map(|tileset_definition| tileset_definition.rel_path.as_ref())
                .map(|ldtk_path| {
                    let asset_path = Path::new(&ldtk_path);
                    let asset_path = ldtk_path_to_asset_path(&base_directory, asset_path);
                    let asset_handle = load_context.load(asset_path);
                    (ldtk_path.clone(), asset_handle)
                })
                .collect();
            let layer_defs = value
                .defs
                .layers
                .iter()
                .map(|layer_def| -> Result<_, LayerDefinitionFromError> {
                    Ok((layer_def.uid, LayerDefinition::new(layer_def)?))
                })
                .collect::<Result<_, _>>()?;

            let entity_defs = value
                .defs
                .entities
                .iter()
                .map(|entity_def| -> Result<_, EntityDefinitionFromError> {
                    Ok((entity_def.uid, EntityDefinition::new(entity_def)?))
                })
                .collect::<Result<_, _>>()?;

            let tileset_defs = value
                .defs
                .tilesets
                .iter()
                .map(TilesetDefinition::new)
                .map(|tileset_def| (tileset_def.uid, tileset_def))
                .collect();

            let enum_defs = value
                .defs
                .enums
                .iter()
                .map(EnumDefinition::new)
                .map(|enum_def| (enum_def.uid, enum_def))
                .collect();

            Ok(ProjectAsset {
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                external_levels: value.external_levels,
                iid: value.iid,
                json_version: value.json_version.clone(),
                tileset_assets,
                background_assets,
                layer_defs,
                entity_defs,
                tileset_defs,
                enum_defs,
                self_handle,
                world_handles,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
