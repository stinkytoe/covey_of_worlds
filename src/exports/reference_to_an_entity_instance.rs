use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug, Default, Reflect)]
pub struct ReferenceToAnEntityInstance {
    pub entity_iid: String,
    pub layer_iid: String,
    pub level_iid: String,
    pub world_iid: String,
}

impl ReferenceToAnEntityInstance {
    pub(crate) fn _new(value: &ldtk::ReferenceToAnEntityInstance) -> Self {
        Self {
            entity_iid: value.entity_iid.clone(),
            layer_iid: value.layer_iid.clone(),
            level_iid: value.level_iid.clone(),
            world_iid: value.world_iid.clone(),
        }
    }
}
