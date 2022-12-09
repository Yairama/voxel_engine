
mod bundles;
mod components;
mod systems;
mod utils;

use bevy::{prelude::*, pbr::wireframe::WireframePlugin, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_flycam::{MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use systems::{startup::startup_system, chunk_systems::{create_chunks}, create_voxels_chunk::{create_voxels_chunk, create_voxels_for_new_chunk}};


pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;


fn main() {

    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
              width: WIDTH,
              height: HEIGHT,
              title: "Voxel_Engine".to_string(),
              ..default()
            },
        ..default()
    }))
    .insert_resource(ClearColor(Color::GRAY))
    .insert_resource(MovementSettings {
        sensitivity: 0.00015, // default: 0.00012
        speed: 120.0, // default: 12.0
    })
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(NoCameraPlayerPlugin)
    .add_plugin(WireframePlugin)
    .add_startup_system(startup_system)
    .add_startup_system(create_chunks)
    .add_system(create_voxels_for_new_chunk)
    // .add_system(create_voxels_chunk)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())


    .run();
}
