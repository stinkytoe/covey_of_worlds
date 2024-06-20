use bevy::prelude::*;

use crate::components::tileset_rectangle::TilesetRectangle;
use crate::ldtk;

#[derive(Debug, Default, Reflect)]
pub struct EnumValueDefinition {
    // ?? No idea what this is...
    pub color: i64,
    pub id: String,
    pub tile_rect: Option<TilesetRectangle>,
}

impl EnumValueDefinition {
    pub(crate) fn new(value: &ldtk::EnumValueDefinition) -> Self {
        EnumValueDefinition {
            color: value.color,
            id: value.id.clone(),
            tile_rect: value.tile_rect.as_ref().map(TilesetRectangle::new),
        }
    }
}
