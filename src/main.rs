
mod bundles;
mod components;
mod systems;
mod utils;

use bevy::{prelude::*, pbr::wireframe::WireframePlugin};
use bevy_flycam::{MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;


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


    .run();
}
