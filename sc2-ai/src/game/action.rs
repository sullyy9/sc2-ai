use bevy::ecs::event::Event;
use num_traits::ToPrimitive;

use rust_sc2::ids::AbilityId;

use super::geometry::Vec3;

#[derive(Event, Default, Clone, Debug, PartialEq)]
pub struct MoveEvent {
    units: Box<[super::entity::Id]>,
    destination: Vec3,
}

impl MoveEvent {
    pub fn new(units: &[super::entity::Id], destination: Vec3) -> Self {
        Self {
            units: units.to_owned().into_boxed_slice(),
            destination,
        }
    }

    pub fn units(&self) -> &[super::entity::Id] {
        &self.units
    }

    pub fn destination(&self) -> &Vec3 {
        &self.destination
    }
}

impl From<MoveEvent> for sc2_proto::sc2api::Action {
    fn from(event: MoveEvent) -> Self {
        let mut action = sc2_proto::sc2api::Action::new();
        let unit_command = action.action_raw.mut_or_insert_default().mut_unit_command();

        unit_command.set_ability_id(AbilityId::Move.to_i32().unwrap());
        unit_command.set_target_world_space_pos(event.destination.into());
        unit_command
            .unit_tags
            .extend(event.units.iter().map(|id| u64::from(*id)));
        unit_command.set_queue_command(false);

        action
    }
}
