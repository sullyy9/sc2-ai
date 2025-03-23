pub mod building;
pub mod map;
pub mod unit;

use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, system::Resource},
    utils::HashMap,
};

use super::geometry::Vec3;

/// Starcraft 2's identifier for a unit. This differs from the entity's [`Entity`] ID in the ECS.
#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GameId(pub(super) u64);

impl From<GameId> for u64 {
    fn from(value: GameId) -> Self {
        value.0
    }
}

/// Maps from an entity's [`GameId`] to it's bevy [`Entity`] ID.
#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
pub struct EntityIdMap(HashMap<GameId, Entity>);

impl std::ops::Deref for EntityIdMap {
    type Target = HashMap<GameId, Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EntityIdMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Common components across world entities. An entity may be a unit, building or terrain.
#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct EntityBundle {
    id: GameId,
    position: Vec3,
}

impl From<sc2_proto::raw::Unit> for EntityBundle {
    fn from(value: sc2_proto::raw::Unit) -> Self {
        Self {
            id: GameId(value.tag.unwrap()),
            position: Vec3::from(value.pos.unwrap()),
        }
    }
}
