use bevy::ecs::system::Resource;

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct DebugCommands(pub Vec<sc2_proto::debug::DebugCommand>);

impl std::ops::Deref for DebugCommands {
    type Target = Vec<sc2_proto::debug::DebugCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DebugCommands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
