
use bevy::prelude::Bundle;
use block_mesh as bm;
use crate::components::{general_components::{DataBaseID, EntityID}, voxel_components::VoxelVisibilityType};

#[derive(Bundle, Clone, Copy, Eq, PartialEq)]
pub struct Voxel {
    pub db_id: DataBaseID,
    pub chunk_id: EntityID,
    pub voxel_visibility: VoxelVisibilityType
}

impl bm::Voxel for Voxel {
    fn get_visibility(&self) -> bm::VoxelVisibility {
        self.voxel_visibility.0
    }
}

impl bm::MergeVoxel for Voxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    // type MergeValueFacingNeighbour = Self;

    // fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
    //     *self
    // }

}

impl Default for Voxel{
    fn default() -> Self {
        Self { 
            db_id: DataBaseID::default(),
            chunk_id: EntityID::default(),
            voxel_visibility: VoxelVisibilityType::opaque() 
        }
    }
}