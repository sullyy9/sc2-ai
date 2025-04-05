use bevy::ecs::{bundle::Bundle, component::Component};

use crate::game::geometry::Vec3;

use super::EntityBundle;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Worker;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct WorkerBundle {
    pub tag: Worker,
    pub unit: EntityBundle,
}

impl Worker {
    pub const SIZE: Vec3 = Vec3::new_3d(1.0, 1.0, 1.0);
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Overlord;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct OverlordBundle {
    pub tag: Overlord,
    pub unit: EntityBundle,
}
