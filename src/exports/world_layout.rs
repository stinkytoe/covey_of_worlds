use crate::ldtk;
use bevy::reflect::impl_reflect;

// Re-export
pub(crate) use ldtk::WorldLayout;

impl_reflect!(
    #[reflect(Debug)]
    #[type_path = "crate::ldtk"]
    pub enum WorldLayout {
        Free,
        GridVania,
        LinearHorizontal,
        LinearVertical,
    }
);
