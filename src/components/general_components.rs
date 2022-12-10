use crate::Component;
use bevy::prelude::Entity;

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct DataBaseID(pub u32);

impl Default for DataBaseID {
    fn default() -> Self {
        Self(0)
    }
}



#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct EntityID(pub Entity);

impl Default for EntityID {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}