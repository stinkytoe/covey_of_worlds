use bevy::prelude::*;

use crate::exports::field_instance::FieldInstance;

#[derive(Clone, Component, Debug, Reflect)]
pub struct FieldInstances {
    pub field_instances: Vec<FieldInstance>,
}
