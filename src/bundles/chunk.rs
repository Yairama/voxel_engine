use bevy::prelude::{Bundle, UVec3};
use block_mesh::ndshape::ConstShape3u32;

use crate::components::{general_components::{DataBaseID}, chunk_components::{CartesianCoordinates, ChunkVisibility}};

pub const CHUNK_SIZE_USIZE: usize = 32;
pub const CHUNK_SIZE_U32: u32 = 32;
pub const CHUNK_SIZE_F32: f32 = 32.;

#[derive(Bundle, Clone, Copy, Eq, PartialEq)]
pub struct Chunk{
    pub db_id: DataBaseID,
    pub cartesian_coordinates: CartesianCoordinates,
    pub chunk_visibility: ChunkVisibility

}

pub type ChunkShape = ConstShape3u32<CHUNK_SIZE_U32,CHUNK_SIZE_U32,CHUNK_SIZE_U32>;

impl Default for Chunk{
    fn default() -> Self {
        Self {
            db_id: DataBaseID(0),
            cartesian_coordinates: CartesianCoordinates(UVec3::ZERO),
            chunk_visibility: ChunkVisibility(true)
        }
    }
}


