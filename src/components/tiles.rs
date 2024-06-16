use bevy::prelude::*;

use crate::exports::tile_instance::TileInstance;

#[derive(Clone, Component, Debug, Reflect)]
pub struct Tiles {
    pub tiles: Vec<TileInstance>,
}
