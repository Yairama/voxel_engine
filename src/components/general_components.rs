use crate::Component;
use bevy::prelude::Entity;
use bevy_inspector_egui::{Inspectable};

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct DataBaseID(pub i32);

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct EntityID(pub Entity);