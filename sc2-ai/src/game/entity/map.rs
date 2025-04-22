use bevy::ecs::{bundle::Bundle, component::Component};

use crate::game::geometry::{Vec2, Vec3};

use super::{EntityBundle, GameEntity, MapEntity};

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MineralPatch;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct RichMinerals;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct MineralPatchBundle {
    pub tag: MineralPatch,
    pub unit: EntityBundle,
}

impl GameEntity for MineralPatch {
    const FOOTPRINT: Vec2 = Vec2::new(2.0, 1.0);
    const SIZE: Vec3 = Self::FOOTPRINT.with_z(2.0);
    const NAME: &'static str = "Minerals";
}

impl MapEntity for MineralPatch {}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VespeneGeyser;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct VespeneGeyserBundle {
    pub tag: VespeneGeyser,
    pub unit: EntityBundle,
}

impl GameEntity for VespeneGeyser {
    const FOOTPRINT: Vec2 = Vec2::new(3.0, 3.0);
    const SIZE: Vec3 = Self::FOOTPRINT.with_z(1.0);
    const NAME: &'static str = "Vespene";
}

impl MapEntity for VespeneGeyser {}

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct DestructibleRock;

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct DestructibleRockBundle {
    pub tag: DestructibleRock,
    pub unit: EntityBundle,
}
