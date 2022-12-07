
use bevy::prelude::Bundle;
use crate::components::{general_components::{DataBaseID, EntityID}, voxel_components::VoxelVisibilityType};

#[derive(Bundle, Clone, Copy, Eq, PartialEq)]
struct Voxel {
    db_id: DataBaseID,
    chunk_id: EntityID,
    voxel_visibility: VoxelVisibilityType
}
