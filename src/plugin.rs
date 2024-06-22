use bevy::prelude::*;
use bevy::utils::error;

use crate::assets::entity::EntityAsset;
use crate::assets::layer::LayerAsset;
use crate::assets::level::LevelAsset;
use crate::assets::project::ProjectAsset;
use crate::assets::project_asset_loader::ProjectAssetLoader;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::assets::traits::LdtkAssetLoadEvent;
use crate::assets::world::WorldAsset;
use crate::components::iid::Iid;
use crate::components::tiles::Tiles;
use crate::components::tileset_rectangle::TilesetRectangle;
use crate::components::traits::LdtkComponent;

pub struct CoveyOfWorldsPlugin;

impl Plugin for CoveyOfWorldsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .register_type::<Iid>()
            .register_type::<Tiles>();

        app //
            .init_asset::<ProjectAsset>()
            .init_asset_loader::<ProjectAssetLoader>()
            .add_event::<LdtkAssetLoadEvent<ProjectAsset>>()
            .register_asset_reflect::<ProjectAsset>()
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
                    <Iid as LdtkComponent<WorldAsset>>::on_ldtk_asset_event_system.map(error),
                    <Transform as LdtkComponent<WorldAsset>>::on_ldtk_asset_event_system.map(error),
                    WorldAsset::on_create_system,
                    WorldAsset::on_modified_system,
                    WorldAsset::with_load_stub_system,
                    WorldAsset::load_children_system.map(error),
                ),
            );

        app //
            .init_asset::<LevelAsset>()
            .add_event::<LdtkAssetLoadEvent<LevelAsset>>()
            .register_asset_reflect::<LevelAsset>()
            .add_systems(
                Update,
                (
                    <Name as LdtkComponent<LevelAsset>>::on_ldtk_asset_event_system.map(error),
                    <Iid as LdtkComponent<LevelAsset>>::on_ldtk_asset_event_system.map(error),
                    <Transform as LdtkComponent<LevelAsset>>::on_ldtk_asset_event_system.map(error),
                    LevelAsset::on_create_system,
                    LevelAsset::on_modified_system,
                    LevelAsset::with_load_stub_system,
                    LevelAsset::load_children_system.map(error),
                    LevelAsset::level_bg_system.map(error),
                ),
            );

        app //
            .init_asset::<LayerAsset>()
            .add_event::<LdtkAssetLoadEvent<LayerAsset>>()
            .register_asset_reflect::<LayerAsset>()
            .add_systems(
                Update,
                (
                    <Name as LdtkComponent<LayerAsset>>::on_ldtk_asset_event_system.map(error),
                    <Iid as LdtkComponent<LayerAsset>>::on_ldtk_asset_event_system.map(error),
                    <Transform as LdtkComponent<LayerAsset>>::on_ldtk_asset_event_system.map(error),
                    <Tiles as LdtkComponent<LayerAsset>>::on_ldtk_asset_event_system.map(error),
                    LayerAsset::on_create_system,
                    LayerAsset::on_modified_system,
                    LayerAsset::with_load_stub_system,
                    LayerAsset::load_children_system.map(error),
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
