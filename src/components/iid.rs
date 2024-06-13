use bevy::prelude::*;

use crate::assets::traits::LdtkAsset;
use crate::prelude::ProjectAsset;

#[derive(Component, Reflect)]
pub struct Iid(pub String);
