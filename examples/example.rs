use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use covey_of_worlds::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "covey_of_worlds=debug,example=trace".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            CoveyOfWorldsPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            // good scale for a 1920x1080 canvas/window
            scale: Vec3::splat(0.3),
            ..default()
        },
        ..default()
    });

    commands.spawn(asset_server.load::<ProjectAsset>("ldtk/top_down.ldtk"));
}

fn update() {}
