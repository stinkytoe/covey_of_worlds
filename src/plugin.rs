use bevy::prelude::*;
use bevy::utils::error;

use crate::{
    assets::{
        entity::EntityAsset,
        project::ProjectAsset,
        project_asset_loader::ProjectAssetLoader,
        traits::{LdtkAsset, LdtkAssetLoadEvent},
    },
    components::{iid::Iid, tileset_rectangle::TilesetRectangle, traits::LdtkComponent},
};

pub struct CoveyOfWorldsPlugin;

impl Plugin for CoveyOfWorldsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_event::<LdtkAssetLoadEvent<ProjectAsset>>()
            .register_asset_reflect::<ProjectAsset>()
            .register_type::<Iid>()
            .add_systems(
                Update,
                (
                    <Iid as LdtkComponent<ProjectAsset>>::on_ldtk_asset_event_system.map(error),
                    <Transform as LdtkComponent<ProjectAsset>>::on_ldtk_asset_event_system
                        .map(error),
                    ProjectAsset::on_create_system,
                    ProjectAsset::on_modified_system,
                    ProjectAsset::with_load_stub_system,
                ),
            );

        app //
            .init_asset::<EntityAsset>()
            .add_event::<LdtkAssetLoadEvent<EntityAsset>>()
            .register_asset_reflect::<EntityAsset>()
            .add_systems(
                Update,
                (
                    <Iid as LdtkComponent<EntityAsset>>::on_ldtk_asset_event_system.map(error),
                    <TilesetRectangle as LdtkComponent<EntityAsset>>::on_ldtk_asset_event_system
                        .map(error),
                    EntityAsset::on_create_system,
                    EntityAsset::on_modified_system,
                    EntityAsset::with_load_stub_system,
                ),
            );
    }
}
