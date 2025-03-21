use bevy::ecs::{bundle::Bundle, component::Component};

use super::EntityBundle;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Worker;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct WorkerBundle {
    pub tag: Worker,
    pub unit: EntityBundle,
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Overlord;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct OverlordBundle {
    pub tag: Overlord,
    pub unit: EntityBundle,
}

