use base::BaseAiPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod base;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AiPluginGroup;

impl PluginGroup for AiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(BaseAiPlugin)
    }
}
