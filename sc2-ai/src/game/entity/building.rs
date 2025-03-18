use bevy_ecs::{bundle::Bundle, component::Component};

use super::EntityBundle;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Hatchery;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct HatcheryBundle {
    pub tag: Hatchery,
    pub unit: EntityBundle,
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Larva;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct LarvaBundle {
    pub tag: Larva,
    pub unit: EntityBundle,
}

