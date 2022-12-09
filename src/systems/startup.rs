use bevy::{prelude::{Commands, Camera3dBundle, ResMut, Transform, Vec3, default, AmbientLight, Color}, pbr::wireframe::WireframeConfig};
use bevy_flycam::FlyCam;



pub fn startup_system(mut commands: Commands, mut wireframe_config: ResMut<WireframeConfig>,){

    wireframe_config.global = true;
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
    
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 100., 0.).looking_at(Vec3::new(10., 10., 10.), Vec3::Y),
        ..default()
    }).insert(FlyCam);

}