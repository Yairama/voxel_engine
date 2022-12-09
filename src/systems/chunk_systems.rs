use bevy::prelude::{Commands, UVec3, Mesh, shape::Cube, Color, StandardMaterial, Transform, PbrBundle, ResMut, Assets, default, Name};

use crate::{bundles::chunk::{Chunk, CHUNK_SIZE_F32}, components::chunk_components::CartesianCoordinates};

pub const WORLD_SIZE: usize = 2;


pub fn create_chunks(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>){

    let world_width = WORLD_SIZE;
    let world_height = WORLD_SIZE;
    let world_length = WORLD_SIZE;

    let chunk_array = vec![vec![vec![Chunk::default();world_length];world_height];world_width];

    for i in 0..chunk_array.len(){

        for j in 0..chunk_array[i].len(){

            for k in 0..chunk_array[i][j].len(){
                let mut chunk = chunk_array[i][j][k];
                chunk.cartesian_coordinates = CartesianCoordinates(
                    UVec3::new(
                        i as u32,
                        j as u32,
                        k as u32
                    )
                );

                let mesh = Mesh::from(Cube::new(5.));
                let material = StandardMaterial::from(Color::rgb(0.1,0.7,0.62));
                let transform = Transform::from_xyz(
                    (i as f32)*CHUNK_SIZE_F32,
                    (j as f32)*CHUNK_SIZE_F32,
                    (k as f32)*CHUNK_SIZE_F32
                );

                let pbr_bundle = PbrBundle{
                    mesh: meshes.add(mesh),
                    material: materials.add(material),
                    transform,
                    ..default()
                };

                commands.spawn((chunk,pbr_bundle, Name::new("Chunk")));
            }

        }

    }


}