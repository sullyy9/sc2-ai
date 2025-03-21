use std::ops::Deref;

use bevy::ecs::system::Resource;

/// Observation provided by the game API.
/// 
/// This contains things like visibile units, effects and events. It is stored as a resource in the
/// ECS in order for other systems to read and generate other entities from it.
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct ApiObservation(sc2_proto::raw::ObservationRaw);

impl Deref for ApiObservation {
    type Target = sc2_proto::raw::ObservationRaw;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<sc2_proto::raw::ObservationRaw> for ApiObservation {
    fn from(value: sc2_proto::raw::ObservationRaw) -> Self {
        Self(value)
    }
}

impl From<ApiObservation> for sc2_proto::raw::ObservationRaw {
    fn from(value: ApiObservation) -> Self {
        value.0
    }
}
