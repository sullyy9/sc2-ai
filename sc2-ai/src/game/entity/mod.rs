pub mod building;
pub mod map;
pub mod unit;

use bevy_ecs::{bundle::Bundle, component::Component};

pub use super::position::Position;

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
    position: Position,
}

impl From<sc2_proto::raw::Unit> for EntityBundle {
    fn from(value: sc2_proto::raw::Unit) -> Self {
        Self {
            id: Id(value.tag.unwrap()),
            position: Position::from(value.pos.unwrap()),
        }
    }
}
