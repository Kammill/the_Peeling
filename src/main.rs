use bevy::{prelude::*, window::{Cursor, WindowResolution}};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use asset_loader::AssetLoaderPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};

use camera::MainCameraPlugin;
use inputs::InputsPlugin;
use world::WorldPlugin;

mod inputs;
mod flat_mesh;
mod asset_loader;
mod camera;
mod world;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::Fullscreen,
            position: WindowPosition::At(IVec2{x: 100, y: 100}), 
            cursor: Cursor {
                visible: false,
                grab_mode: bevy::window::CursorGrabMode::Confined,
                ..default()
            },
            resolution: WindowResolution::new(1920.0/2., 1080.0 /2.0),
            title: "ToHell".to_string(),
            ..default()
        }),
        ..default()
    }))
    
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(LogDiagnosticsPlugin::default())
    //Rapier
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(WorldPlugin)
    .add_plugins(InputsPlugin)
    //.add_plugins(SpawnerPlugin)

    .add_plugins(MainCameraPlugin)
    .add_systems(Update, bevy::window::close_on_esc)

    .run();
}
