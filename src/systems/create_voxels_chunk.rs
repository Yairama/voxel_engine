
use bevy::{prelude::{Commands, Query, Entity, Mesh, PbrBundle, Assets, ResMut, Handle, default, Component, Added}, math::Vec3A, render::{render_resource::PrimitiveTopology, mesh::{VertexAttributeValues, Indices}}};
use block_mesh::{ndshape::{ConstShape3u32, ConstShape}, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG, GreedyQuadsBuffer, greedy_quads};

use crate::{bundles::{chunk::{Chunk, CHUNK_SIZE_U32, ChunkShape}, voxel::Voxel}, components::{chunk_components::{CartesianCoordinates, ChunkVisibility}, general_components::{DataBaseID, EntityID}, voxel_components::VoxelVisibilityType}};



pub fn create_voxels_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity,
        &DataBaseID,
        &CartesianCoordinates,
        &ChunkVisibility
        )>){
    
    

    for (entity,
        db_id,
        cartesian_coordinates,
        chunk_visibility
    ) in query.iter(){

        if *chunk_visibility == true{

            // commands.entity(entity).remove::<Handle<Mesh>>();
            let mesh = greedy_mesh(&mut meshes, entity);
            commands.entity(entity).insert(mesh);

        }                  

    }

}

pub fn create_voxels_for_new_chunk(mut commands: Commands,
    query: Query<(Entity, &ChunkVisibility),
    Added<ChunkVisibility>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    
    for (entity, chunk_visibility) in query.iter(){
        
        let mesh = greedy_mesh(&mut meshes, entity);
        commands.entity(entity).insert(mesh);
    }

}


fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0/ array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

fn sphere(radius: f32, p: Vec3A) -> VoxelVisibilityType {
    if(p.length() < radius){
        VoxelVisibilityType::opaque()
    }else {
        VoxelVisibilityType::empty()
    }
}

fn greedy_mesh(meshes: &mut Assets<Mesh>, entity: Entity) -> Handle<Mesh>{



    let mut voxels = [
        Voxel{ 
            chunk_id: EntityID(entity),
            voxel_visibility: VoxelVisibilityType::empty(),
            ..default() }; ChunkShape::USIZE];

    for z in 1..31{
        for y in 1..31{
            for x in 1..31{
                let i = ChunkShape::linearize([x, y, z]);
                voxels[i as usize].voxel_visibility = VoxelVisibilityType::opaque();
            }
        }
    }

    /// Code for generate Sphere
    // for i in 0u32..(ChunkShape::SIZE) {
    //     let p = into_domain(31, ChunkShape::delinearize(i));
    //     voxels[i as usize].voxel_visibility = sphere(0.9,p);
    // }
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
    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
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
        VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
    );
    render_mesh.set_indices(Some(Indices::U32(indices.clone())));

    meshes.add(render_mesh)

}