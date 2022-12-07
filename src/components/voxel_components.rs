
use bevy::prelude::Component;
use block_mesh::VoxelVisibility;

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct VoxelVisibilityType(pub VoxelVisibility);

impl VoxelVisibilityType {
    pub fn empty() -> Self {
        Self(VoxelVisibility::Empty)
    }

    pub fn translucent() -> Self {
        Self(VoxelVisibility::Translucent)
    }

    pub fn opaque() -> Self {
        Self(VoxelVisibility::Opaque)
    }

}