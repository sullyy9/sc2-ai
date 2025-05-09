pub mod building;
pub mod map;
pub mod unit;

use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::Event,
        query::With,
        system::{Commands, Query, Resource},
    },
    utils::HashMap,
};

use super::{
    debug::{Color, DrawCommandsExt as _},
    geometry::{Rect, Vec2, Vec3},
};

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

pub trait GameEntity: Component {
    const FOOTPRINT: Vec2;
    const SIZE: Vec3;
    const NAME: &'static str;
}

pub trait MapEntity: GameEntity {
    fn draw_debug_info(mut commands: Commands, query: Query<&Vec3, With<Self>>)
    where
        Self: Sized,
    {
        for &pos in query.iter() {
            commands.draw_surface_box(
                Rect::from_center(pos.without_z(), Self::FOOTPRINT),
                Self::SIZE.z,
                Color::BLUE,
            );
            commands.draw_text(Self::NAME, pos, Color::default());
        }
    }
}

pub trait UnitEntity: GameEntity {}
pub trait BuildingEntity: GameEntity {
    const BUILD_ID: sc2_proto::ability::AbilityId;
}

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityFound<T: GameEntity> {
    pub entity: Entity,
    tag: T,
}

impl<T> From<Entity> for EntityFound<T>
where
    T: GameEntity + Default,
{
    fn from(entity: Entity) -> Self {
        Self {
            entity,
            tag: Default::default(),
        }
    }
}
