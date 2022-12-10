
mod bundles;
mod components;
mod systems;
mod utils;

use bevy::{prelude::*, pbr::wireframe::WireframePlugin, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_flycam::{MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use systems::{startup::{startup_system, AppState, load_assets, check_loaded}, chunk_systems::{create_chunks_not_optimal}, create_voxels_chunk::{create_voxels_chunk, create_voxels_for_new_chunk}};


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
    .insert_resource(State::new(AppState::Loading))
    .add_state(AppState::Loading)
    .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
    .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loaded))
    .add_system_set(SystemSet::on_enter(AppState::Run).with_system(startup_system))
    .add_system_set(SystemSet::on_enter(AppState::Run).with_system(create_chunks_not_optimal))
    .add_system_set(SystemSet::on_update(AppState::Run).with_system(create_voxels_for_new_chunk))

    // .add_startup_system(startup_system)
    // .add_startup_system(create_chunks)
    // .add_system(create_voxels_for_new_chunk)
    // .add_system(create_voxels_chunk)****
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())


    .run();
}
