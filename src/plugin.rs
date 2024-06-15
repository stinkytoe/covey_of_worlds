use bevy::prelude::*;
use bevy::utils::error;

use crate::{
    assets::{
        entity::EntityAsset,
        project::ProjectAsset,
        project_asset_loader::ProjectAssetLoader,
        traits::{LdtkAsset, LdtkAssetChildLoader, LdtkAssetLoadEvent},
        world::WorldAsset,
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
                    <Name as LdtkComponent<ProjectAsset>>::on_ldtk_asset_event_system.map(error),
                    <Iid as LdtkComponent<ProjectAsset>>::on_ldtk_asset_event_system.map(error),
                    <Transform as LdtkComponent<ProjectAsset>>::on_ldtk_asset_event_system
                        .map(error),
                    ProjectAsset::on_create_system,
                    ProjectAsset::on_modified_system,
                    ProjectAsset::with_load_stub_system,
                    ProjectAsset::load_children_system.map(error),
                ),
            );

        app //
            .init_asset::<WorldAsset>()
            .add_event::<LdtkAssetLoadEvent<WorldAsset>>()
            .register_asset_reflect::<WorldAsset>()
            .add_systems(
                Update,
                (
                    <Name as LdtkComponent<WorldAsset>>::on_ldtk_asset_event_system.map(error),
                    WorldAsset::on_create_system,
                    WorldAsset::on_modified_system,
                    WorldAsset::with_load_stub_system,
                    // WorldAsset::load_children_system.map(error),
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
