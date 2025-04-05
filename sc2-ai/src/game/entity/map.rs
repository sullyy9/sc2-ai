use bevy::ecs::{bundle::Bundle, component::Component};

use crate::game::geometry::Vec3;

use super::EntityBundle;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MineralPatch;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct MineralPatchBundle {
    pub tag: MineralPatch,
    pub unit: EntityBundle,
}

impl MineralPatch {
    pub const SIZE: Vec3 = Vec3::new_3d(2.0, 1.0, 2.0);
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VespeneGeyser;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct VespeneGeyserBundle {
    pub tag: VespeneGeyser,
    pub unit: EntityBundle,
}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct DestructibleRock;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct DestructibleRockBundle {
    pub tag: DestructibleRock,
    pub unit: EntityBundle,
}
