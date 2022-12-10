
use bevy::prelude::Component;
use block_mesh::VoxelVisibility;


#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct VoxelVisibilityType(pub VoxelVisibility);

impl VoxelVisibilityType {
    pub fn empty() -> Self {
        Self(VoxelVisibility::Empty)
    }
    #[allow(dead_code)]
    pub fn translucent() -> Self {
        Self(VoxelVisibility::Translucent)
    }

    pub fn opaque() -> Self {
        Self(VoxelVisibility::Opaque)
    }

}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct VoxelCoordinates(pub [u8;3]);
impl Default for VoxelCoordinates {
    fn default() -> Self {
        Self([0,0,0])
    }
}