use bevy::ecs::system::Resource;

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct Actions(Vec<sc2_proto::sc2api::Action>);

impl std::ops::Deref for Actions {
    type Target = Vec<sc2_proto::sc2api::Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Actions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
