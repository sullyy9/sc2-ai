use bevy::ecs::{
    event::{Event, EventReader},
    system::{ResMut, Resource},
};

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct Actions(pub Vec<sc2_proto::sc2api::Action>);

pub fn action_handler<T>(mut event: EventReader<T>, mut actions: ResMut<Actions>)
where
    T: Event + Into<sc2_proto::sc2api::Action> + Clone,
{
    actions.0.extend(event.read().map(|e| e.clone().into()));
}
