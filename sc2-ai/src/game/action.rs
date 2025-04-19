use bevy::ecs::{entity::Entity, event::Event, world::Command};
use num_traits::ToPrimitive;

use rust_sc2::ids::AbilityId;

use crate::core::Actions;

use super::{entity::GameId, geometry::Vec2};

struct MoveCommand {
    units: Box<[Entity]>,
    destination: Vec2,
}

#[derive(Event, Default, Clone, Debug, PartialEq)]
pub struct MoveEvent {
    units: Box<[Entity]>,
    destination: Vec2,
}

impl MoveEvent {
    pub fn units(&self) -> &[Entity] {
        &self.units
    }

    pub fn destination(&self) -> &Vec2 {
        &self.destination
    }
}

impl Command for MoveCommand {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let units = self
            .units
            .into_iter()
            .map(|e| {
                world
                    .entity(e)
                    .get::<GameId>()
                    .expect("Found missing GameId for entity")
            })
            .map(|id| u64::from(*id))
            .collect::<Box<_>>();

        let mut action = sc2_proto::sc2api::Action::new();
        let unit_command = action.action_raw.mut_or_insert_default().mut_unit_command();

        unit_command.set_ability_id(AbilityId::Move.to_i32().unwrap());
        unit_command.set_target_world_space_pos(self.destination.into());
        unit_command.unit_tags.extend(units);
        unit_command.set_queue_command(false);

        let mut actions = world.resource_mut::<Actions>();
        actions.push(action);
    }
}

pub trait ActionCommandsExt {
    /// Request that a collection of units move to a location.
    ///
    /// Dispatches a [`MoveEvent`].
    fn move_units(&mut self, units: &[Entity], destination: Vec2);

    /// Request that a single unit move to a location.
    ///
    /// Dispatches a [`MoveEvent`].
    fn move_unit(&mut self, unit: Entity, destination: Vec2) {
        self.move_units(&[unit], destination);
    }
}

impl ActionCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn move_units(&mut self, units: &[Entity], destination: Vec2) {
        let units: Box<[_]> = Box::from(units);

        self.queue(MoveCommand {
            units: units.clone(),
            destination,
        });

        self.send_event(MoveEvent { units, destination });
    }
}
