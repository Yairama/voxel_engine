use bevy::{prelude::{Commands, Camera3dBundle, ResMut, Transform, Vec3, default, AmbientLight, Color, AssetServer, Res, debug, Handle, Image, Resource, State, Assets}, pbr::wireframe::WireframeConfig, asset::LoadState, render::{texture::ImageSampler, render_resource::{SamplerDescriptor, AddressMode}}};
use bevy_flycam::FlyCam;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppState {
    Loading,
    Run,
}

#[derive(Resource)]
pub struct TextureLoading(pub Handle<Image>);

pub fn startup_system(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    texture_handle: Res<TextureLoading>,
    mut textures: ResMut<Assets<Image>>,
    ){

    
    let mut texture = textures.get_mut(&texture_handle.0).unwrap();
    // Set the texture to tile over the entire quad
    texture.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        ..Default::default()
    });

    wireframe_config.global = false;
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
    
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    }).insert(FlyCam);

}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // debug!("load");
    let handle: Handle<Image> = asset_server.load("uv_checker.png");
    commands.insert_resource(TextureLoading(handle));
}

/// Make sure that our texture is loaded so we can change some settings on it later
pub fn check_loaded(
    mut state: ResMut<State<AppState>>,
    handle: Res<TextureLoading>,
    asset_server: Res<AssetServer>,
) {
    debug!("check loaded");
    if let LoadState::Loaded = asset_server.get_load_state(&handle.0) {
        state.set(AppState::Run).unwrap();
    }
}