use bevy_ecs::{bundle::Bundle, component::Component};

use super::EntityBundle;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MineralPatch;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct MineralPatchBundle {
    pub tag: MineralPatch,
    pub unit: EntityBundle,
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
