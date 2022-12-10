use bevy::prelude::{Commands, UVec3, Mesh, shape::Cube, Color, StandardMaterial, Transform, PbrBundle, ResMut, Assets, default, Name, debug};
use block_mesh::ndshape::{ConstShape3u32, ConstShape};

use crate::{bundles::chunk::{Chunk, CHUNK_SIZE_F32}, components::chunk_components::CartesianCoordinates};

pub const WORLD_SIZE: u32 = 20;
pub type WorldShape = ConstShape3u32<WORLD_SIZE,WORLD_SIZE,WORLD_SIZE>;

pub fn create_chunks_not_optimal(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>){

    debug!("Creating Chunks!");

    let mut chunk_array = [Chunk::default();WorldShape::USIZE];

    for i in 0..WorldShape::USIZE{
        let a = WorldShape::delinearize(i as u32);
        chunk_array[i].cartesian_coordinates = CartesianCoordinates(UVec3::from_array(a));
    
        
        let mesh = Mesh::from(Cube::new(5.));
        let material = StandardMaterial::from(Color::rgb(0.1,0.7,0.62));
        let transform = Transform::from_xyz(
            (a[0] as f32)*CHUNK_SIZE_F32,
            (a[1] as f32)*CHUNK_SIZE_F32,
            (a[2] as f32)*CHUNK_SIZE_F32
        );

        let pbr_bundle = PbrBundle{
            mesh: meshes.add(mesh),
            material: materials.add(material),
            transform,
            ..default()
        };

        commands.spawn((chunk_array[i],pbr_bundle));
    }

    

}



