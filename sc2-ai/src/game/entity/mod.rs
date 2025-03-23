pub mod building;
pub mod map;
pub mod unit;

use bevy::ecs::{bundle::Bundle, component::Component};

use super::geometry::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Id(u64);

impl From<Id> for u64 {
    fn from(value: Id) -> Self {
        value.0
    }
}

#[derive(Bundle, Default, Clone, Copy, Debug, PartialEq)]
pub struct EntityBundle {
    id: Id,
    position: Vec3,
}

impl From<sc2_proto::raw::Unit> for EntityBundle {
    fn from(value: sc2_proto::raw::Unit) -> Self {
        Self {
            id: Id(value.tag.unwrap()),
            position: Vec3::from(value.pos.unwrap()),
        }
    }
}
