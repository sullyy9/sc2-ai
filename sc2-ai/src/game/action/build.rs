use std::marker::PhantomData;

use bevy::ecs::{
    entity::Entity,
    world::{Command, World},
};
use num_traits::ToPrimitive;

use crate::{
    core::Actions,
    game::{
        entity::{BuildingEntity, GameId},
        geometry::Vec2,
    },
};

struct BuildCommand<T> {
    unit: Entity,
    location: Vec2,
    _building: PhantomData<T>,
}

impl<T> BuildCommand<T>
where
    T: BuildingEntity,
{
    fn into_proto(self, world: &World) -> sc2_proto::raw::ActionRaw {
        let unit_id = world
            .entity(self.unit)
            .get::<GameId>()
            .expect("Every game entity should have a GameId component");

        let mut action = sc2_proto::raw::ActionRaw::new();
        let command = action.mut_unit_command();

        command.set_ability_id(T::BUILD_ID.to_i32().unwrap());
        command.set_target_world_space_pos(self.location.into());
        command.unit_tags.push(u64::from(*unit_id));
        command.set_queue_command(false);

        action
    }
}

impl<T> Command for BuildCommand<T>
where
    T: BuildingEntity + Send,
{
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let mut action = sc2_proto::sc2api::Action::new();
        action.action_raw = protobuf::MessageField::some(self.into_proto(world));

        let mut actions = world.resource_mut::<Actions>();
        actions.push(action);
    }
}

pub trait BuildCommandsExt {
    /// Request that a unit construct a building at a given location.
    fn build<T>(&mut self, location: Vec2, builder: Entity)
    where
        T: BuildingEntity;
}

impl BuildCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn build<T>(&mut self, location: Vec2, builder: Entity)
    where
        T: BuildingEntity,
    {
        self.queue(BuildCommand::<T> {
            unit: builder,
            location,
            _building: Default::default(),
        });
    }
}
