use bevy::ecs::{bundle::Bundle, component::Component};

use crate::game::geometry::{Vec2, Vec3};

use super::{EntityBundle, GameEntity};

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Hatchery;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct HatcheryBundle {
    pub tag: Hatchery,
    pub unit: EntityBundle,
}

impl GameEntity for Hatchery {
    const FOOTPRINT: Vec2 = Vec2::new(5.0, 5.0);
    const SIZE: Vec3 = Self::FOOTPRINT.with_z(2.0);
    const NAME: &'static str = "Hatchery";
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Larva;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct LarvaBundle {
    pub tag: Larva,
    pub unit: EntityBundle,
}
