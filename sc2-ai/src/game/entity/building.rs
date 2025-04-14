use bevy::ecs::{bundle::Bundle, component::Component};

use crate::game::geometry::Vec3;

use super::{EntityBundle, GameEntity};

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Hatchery;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct HatcheryBundle {
    pub tag: Hatchery,
    pub unit: EntityBundle,
}

impl GameEntity for Hatchery {
    const SIZE: Vec3 = Vec3::new_3d(5.0, 5.0, 2.0);
    const NAME: &'static str = "Hatchery";
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Larva;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct LarvaBundle {
    pub tag: Larva,
    pub unit: EntityBundle,
}
