use bevy::{math::I64Vec2, prelude::*};
use thiserror::Error;

use crate::components::tiles::Tiles;

use super::{entity::EntityAsset, project::ProjectAsset, traits::LdtkAsset};

#[derive(Debug, Error)]
pub enum LayerTypeError {
    #[error("Unknown LDtk layer type! {0}")]
    UnknownLayerType(String),
}

#[derive(Clone, Copy, Debug, Reflect)]
pub enum LayerType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

impl LayerType {
    pub fn new(ldtk_type: &str) -> Result<LayerType, LayerTypeError> {
        Ok(match ldtk_type {
            "IntGrid" => LayerType::IntGrid,
            "Entities" => LayerType::Entities,
            "Tiles" => LayerType::Tiles,
            "AutoLayer" => LayerType::Autolayer,
            _ => return Err(LayerTypeError::UnknownLayerType(ldtk_type.to_string())),
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct LayerAsset {
    // from LDtk
    pub grid_size: I64Vec2,
    pub grid_cell_size: i64,
    pub identifier: String,
    pub opacity: f64,
    pub px_total_offset: I64Vec2,
    pub tileset_def_uid: Option<i64>,
    pub tileset_rel_path: Option<String>,
    pub layer_type: LayerType,
    pub iid: String,
    pub int_grid_csv: Vec<i64>,
    pub layer_def_uid: i64,
    pub level_id: i64,
    pub override_tileset_uid: Option<i64>,
    pub px_offset: I64Vec2,
    pub visible: bool,

    // for us!
    pub index: usize,
    pub tiles: Tiles,
    #[reflect(ignore)]
    pub(crate) _project: Handle<ProjectAsset>,
    pub(crate) entity_handles: Vec<(String, String, Handle<EntityAsset>)>,
    pub(crate) entities_to_load: EntitiesToLoad,
}

impl LayerAsset {
    // pub(crate) fn new(
    //     value: &ldtk::LayerInstance,
    //     project: Handle<ProjectAsset>,
    //     index: usize,
    //     // tiles: Vec<Tile>,
    //     // entity_assets_by_identifier: HashMap<String, Handle<EntityAsset>>,
    //     // entity_assets_by_iid: HashMap<String, Handle<EntityAsset>>,
    // ) -> Result<Self, LayerTypeError> {
    //     Ok(Self {
    //         grid_size: (value.c_wid, value.c_hei).into(),
    //         grid_cell_size: value.grid_size,
    //         identifier: value.identifier.clone(),
    //         opacity: value.opacity,
    //         px_total_offset: (value.px_total_offset_x, -value.px_total_offset_y).into(),
    //         tileset_def_uid: value.tileset_def_uid,
    //         tileset_rel_path: value.tileset_rel_path.clone(),
    //         layer_type: LayerType::new(&value.layer_instance_type)?,
    //         iid: value.iid.clone(),
    //         int_grid_csv: value.int_grid_csv.clone(),
    //         layer_def_uid: value.layer_def_uid,
    //         level_id: value.level_id,
    //         override_tileset_uid: value.override_tileset_uid,
    //         px_offset: (value.px_offset_x, -value.px_offset_y).into(),
    //         visible: value.visible,
    //         index,
    //         // tiles,
    //         project,
    //         // entity_assets_by_identifier,
    //         // entity_assets_by_iid,
    //     })
    // }
}
#[derive(Default, Debug, Reflect)]
pub enum EntitiesToLoad {
    None,
    ByIid(Vec<String>),
    #[default]
    All,
}

impl LdtkAsset for LayerAsset {}
