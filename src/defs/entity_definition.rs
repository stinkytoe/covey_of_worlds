use bevy::prelude::*;
use bevy::sprite::Anchor;
use thiserror::Error;

use crate::components::tileset_rectangle::TilesetRectangle;
use crate::defs::tile_render_mode::TileRenderMode;
use crate::ldtk;
use crate::util::bevy_anchor_from_ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::AnchorIntoError;
use crate::util::ColorParseError;

#[derive(Debug, Reflect)]
pub struct EntityDefinition {
    // pub color: String,
    pub color: Color,
    // pub height: i64,
    // pub width: i64,
    pub size: Vec2,
    pub identifier: String,
    pub nine_slice_borders: Vec<i64>,
    // pub pivot_x: f64,
    // pub pivot_y: f64,
    pub anchor: Anchor,
    pub tile_rect: Option<TilesetRectangle>,
    pub tile_render_mode: TileRenderMode,
    pub tileset_id: Option<i64>,
    pub ui_tile_rect: Option<TilesetRectangle>,
    pub uid: i64,
}

#[derive(Debug, Error)]
pub enum EntityDefinitionFromError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    AnchorIntoError(#[from] AnchorIntoError),
}

impl EntityDefinition {
    pub(crate) fn new(value: &ldtk::EntityDefinition) -> Result<Self, EntityDefinitionFromError> {
        Ok(Self {
            color: bevy_color_from_ldtk(&value.color)?,
            size: (value.width as f32, value.height as f32).into(),
            identifier: value.identifier.clone(),
            nine_slice_borders: value.nine_slice_borders.clone(),
            anchor: bevy_anchor_from_ldtk(&[value.pivot_x, value.pivot_y])?,
            tile_rect: value.tile_rect.as_ref().map(TilesetRectangle::new),
            tile_render_mode: TileRenderMode::new(&value.tile_render_mode),
            tileset_id: value.tileset_id,
            ui_tile_rect: value.ui_tile_rect.as_ref().map(TilesetRectangle::new),
            uid: value.uid,
        })
    }
}
