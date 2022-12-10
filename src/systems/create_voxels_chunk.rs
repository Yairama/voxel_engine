
use bevy::{prelude::{Commands, Query, Entity, Mesh, Assets, ResMut, Handle, default, Added, Res, StandardMaterial, Image}, math::Vec3A, render::{render_resource::{PrimitiveTopology, AddressMode, SamplerDescriptor}, mesh::{VertexAttributeValues, Indices}, texture::ImageSampler}};
use block_mesh::{ndshape::{ ConstShape}, RIGHT_HANDED_Y_UP_CONFIG, GreedyQuadsBuffer, greedy_quads};

use crate::{bundles::{chunk::{CHUNK_SIZE_U32, ChunkShape}, voxel::Voxel}, components::{chunk_components::{CartesianCoordinates, ChunkVisibility, VoxelArray}, general_components::{DataBaseID, EntityID}, voxel_components::VoxelVisibilityType}};

use super::startup::TextureLoading;

const UV_SCALE: f32 = 1.0 / 16.0;

#[allow(dead_code)]
pub fn create_voxels_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity,
        &DataBaseID,
        &CartesianCoordinates,
        &ChunkVisibility
        )>){

    for (entity,
        _db_id,
        _cartesian_coordinates,
        _chunk_visibility
    ) in query.iter(){

        if true{


            let voxels = [
                Voxel::default(); ChunkShape::USIZE];
                
            let mesh = greedy_mesh(voxels);
            commands.entity(entity).insert(meshes.add(mesh));
            commands.entity(entity).insert(VoxelArray(voxels.to_vec()));

        }                  

    }

}

pub fn create_voxels_for_new_chunk<'a>(mut commands: Commands,
    query: Query<(Entity, &ChunkVisibility),
    Added<ChunkVisibility>>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_handle: Res<TextureLoading>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ){
    
    for (entity, _chunk_visibility) in query.iter(){

        let voxels = [Voxel::default(); ChunkShape::USIZE];
        
        let mesh = greedy_mesh(voxels);
        commands.entity(entity).insert( meshes.add(mesh));
        commands.entity(entity).insert(materials.add(texture_handle.0.clone().into()));
        // commands.entity(entity).insert(VoxelArray(voxels.to_vec()));


    }

}

#[allow(dead_code)]
fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0/ array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

#[allow(dead_code)]
fn sphere(radius: f32, p: Vec3A) -> u8 {
    if p.length() < radius{
        1
    }else {
        0
    }
}

fn greedy_mesh(
    mut voxels: [Voxel; ChunkShape::USIZE]
    ) -> Mesh{

    // for z in 1..31{
    //     for y in 1..31{
    //         for x in 1..31{
    //             let i = ChunkShape::linearize([x, y, z]);
    //             voxels[i as usize].voxel_visibility = VoxelVisibilityType::opaque();
    //         }
    //     }
    // }

    // Code to generate Sphere
    for i in 0u32..(ChunkShape::SIZE) {
        let p = into_domain(31, ChunkShape::delinearize(i));
        voxels[i as usize].0 = sphere(0.9,p);
    }
    // println!("{}", voxels.len());

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let mut buffer = GreedyQuadsBuffer::new(voxels.len());
    greedy_quads(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [CHUNK_SIZE_U32-1; 3],
        &faces,
        &mut buffer,
    );



    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);


    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &quad,
            ));
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    for uv in tex_coords.iter_mut() {
        for c in uv.iter_mut() {
            *c *= UV_SCALE;
        }
    }


    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL, 
        VertexAttributeValues::Float32x3(normals)
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(tex_coords),
    );
    render_mesh.set_indices(Some(Indices::U32(indices)));

    render_mesh

}

