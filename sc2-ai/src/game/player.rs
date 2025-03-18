use bevy_ecs::system::Resource;

/// Current player resources.
///
/// This contains the current minerals, vepene and unit capacity.
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct PlayerResources {
    minerals: u32,
    vespene: u32,
    unit_capacity: u32,
    used_capacity: u32,
}

impl From<sc2_proto::sc2api::PlayerCommon> for PlayerResources {
    fn from(value: sc2_proto::sc2api::PlayerCommon) -> Self {
        Self {
            minerals: value.minerals(),
            vespene: value.vespene(),
            unit_capacity: value.food_cap(),
            used_capacity: value.food_used(),
        }
    }
}
