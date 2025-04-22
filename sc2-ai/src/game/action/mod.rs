use bevy::ecs::entity::Entity;

use super::{entity::BuildingEntity, geometry::Vec2};

mod build;
mod r#move;

pub use build::BuildCommandsExt;
pub use r#move::{MoveCommandsExt, MoveEvent};

pub trait ActionCommandsExt: MoveCommandsExt + BuildCommandsExt {
    fn move_units(&mut self, units: &[Entity], destination: Vec2) {
        MoveCommandsExt::move_units(self, units, destination)
    }

    fn move_unit(&mut self, unit: Entity, destination: Vec2) {
        MoveCommandsExt::move_unit(self, unit, destination)
    }

    fn build<T>(&mut self, location: Vec2, builder: Entity)
    where
        T: BuildingEntity,
    {
        BuildCommandsExt::build::<T>(self, location, builder)
    }
}

impl ActionCommandsExt for bevy::ecs::system::Commands<'_, '_> {}
