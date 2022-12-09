use bevy::prelude::{Component, UVec3};


#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct CartesianCoordinates(pub UVec3);


#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct ChunkVisibility(pub bool);


impl PartialEq<bool> for ChunkVisibility {

    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}