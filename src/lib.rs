mod ldtk;
mod plugin;
mod system_params;
mod util;

pub mod prelude {
    pub use crate::assets::entity::EntityAsset;
    pub use crate::assets::project::ProjectAsset;
    pub use crate::plugin::CoveyOfWorldsPlugin;
}

mod assets;
mod components;
