use bevy::{math::I64Vec2, prelude::*};
use thiserror::Error;

use crate::assets::entity::EntityAsset;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::components::iid::Iid;
use crate::components::tiles::Tiles;
use crate::components::traits::LdtkComponent;
use crate::exports::tile_instance::TileInstance;
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LayerAssetError {
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
    pub fn new(ldtk_type: &str) -> Result<LayerType, LayerAssetError> {
        Ok(match ldtk_type {
            "IntGrid" => LayerType::IntGrid,
            "Entities" => LayerType::Entities,
            "Tiles" => LayerType::Tiles,
            "AutoLayer" => LayerType::Autolayer,
            _ => return Err(LayerAssetError::UnknownLayerType(ldtk_type.to_string())),
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
    #[reflect(ignore)]
    pub int_grid_csv: Vec<i64>,
    pub layer_def_uid: i64,
    pub level_id: i64,
    pub override_tileset_uid: Option<i64>,
    pub location: Vec3,
    pub visible: bool,

    // for us!
    pub index: usize,
    #[reflect(ignore)]
    pub tiles: Vec<TileInstance>,
    #[reflect(ignore)]
    pub(crate) entity_handles: Vec<Handle<EntityAsset>>,
}

impl LayerAsset {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        index: usize,
        layer_type: LayerType,
        tiles: Vec<TileInstance>,
        entity_handles: Vec<Handle<EntityAsset>>,
        layer_separation: f32,
    ) -> Result<Self, LayerAssetError> {
        Ok(Self {
            grid_size: (value.c_wid, value.c_hei).into(),
            grid_cell_size: value.grid_size,
            identifier: value.identifier.clone(),
            opacity: value.opacity,
            px_total_offset: (value.px_total_offset_x, -value.px_total_offset_y).into(),
            tileset_def_uid: value.tileset_def_uid,
            tileset_rel_path: value.tileset_rel_path.clone(),
            layer_type,
            iid: value.iid.clone(),
            int_grid_csv: value.int_grid_csv.clone(),
            layer_def_uid: value.layer_def_uid,
            level_id: value.level_id,
            override_tileset_uid: value.override_tileset_uid,
            // px_offset: (value.px_offset_x, -value.px_offset_y).into(),
            location: (
                value.px_offset_x as f32,
                -value.px_offset_y as f32,
                (index as f32 + 2.0) * layer_separation,
            )
                .into(),
            visible: value.visible,
            index,
            tiles,
            entity_handles,
            // entity_assets_by_identifier,
            // entity_assets_by_iid,
        })
    }
}

impl LdtkAsset for LayerAsset {
    fn iid(&self) -> String {
        self.iid.clone()
    }
}

impl LdtkAssetChildLoader<EntityAsset> for LayerAsset {
    fn children(&self) -> Vec<Handle<EntityAsset>> {
        self.entity_handles.clone()
    }
}

impl LdtkComponent<LayerAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LayerAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands
            .entity(entity)
            .try_insert(Name::new(asset.identifier.clone()));
        Ok(())
    }
}

impl LdtkComponent<LayerAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LayerAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        commands.entity(entity).try_insert(Iid(asset.iid.clone()));
        Ok(())
    }
}

impl LdtkComponent<LayerAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Self>,
        asset: &LayerAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = asset.location;
        } else {
            commands
                .entity(entity)
                .try_insert(SpatialBundle::from_transform(Transform::from_translation(
                    asset.location,
                )));
        }
        Ok(())
    }
}

impl LdtkComponent<LayerAsset> for Tiles {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &LayerAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        if asset.tiles.is_empty() {
            commands.entity(entity).remove::<Tiles>();
        } else {
            commands.entity(entity).try_insert(Tiles {
                tiles: asset.tiles.clone(),
            });
        }
        Ok(())
    }
}
