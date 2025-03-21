//! Deals with transforming data between the SC2 API and types moe suitable for use in ECS systems.

pub mod action;
pub mod entity;
mod player;
mod position;

use bevy::{
    app::{Plugin, PostUpdate, PreUpdate},
    ecs::system::{Commands, Res},
};
use entity::{
    EntityBundle,
    building::{HatcheryBundle, LarvaBundle},
    map::{DestructibleRockBundle, MineralPatchBundle, VespeneGeyserBundle},
    unit::{OverlordBundle, WorkerBundle},
};
use num_traits::FromPrimitive;
use tracing::warn;

use crate::core::{ApiObservation, action_handler};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PreUpdate, create_entities);
        app.add_systems(PostUpdate, action_handler::<action::MoveEvent>);
    }
}

/// Create entities by evaluating the [`Observation`] resource.
fn create_entities(mut commands: Commands, observation: Res<ApiObservation>) {
    for unit in &observation.units {
        use sc2_proto::unit::TypeId;
        let unit_type = TypeId::from_u32(unit.unit_type()).unwrap();

        match unit_type {
            TypeId::MineralField | TypeId::MineralField450 | TypeId::MineralField750 => {
                let entity = MineralPatchBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }
            TypeId::VespeneGeyser
            | TypeId::SpacePlatformGeyser
            | TypeId::RichVespeneGeyser
            | TypeId::ProtossVespeneGeyser
            | TypeId::PurifierVespeneGeyser
            | TypeId::ShakurasVespeneGeyser => {
                let entity = VespeneGeyserBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }
            TypeId::DestructibleCityDebris2x4Vertical
            | TypeId::DestructibleCityDebris2x4Horizontal
            | TypeId::DestructibleCityDebris2x6Vertical
            | TypeId::DestructibleCityDebris2x6Horizontal
            | TypeId::DestructibleCityDebris4x4
            | TypeId::DestructibleCityDebris6x6
            | TypeId::DestructibleRockEx12x4Vertical
            | TypeId::DestructibleRockEx12x4Horizontal
            | TypeId::DestructibleRockEx12x6Vertical
            | TypeId::DestructibleRockEx12x6Horizontal
            | TypeId::DestructibleRockEx14x4
            | TypeId::DestructibleRockEx16x6
            | TypeId::UnbuildableRocksDestructible
            | TypeId::UnbuildableBricksDestructible
            | TypeId::UnbuildablePlatesDestructible => {
                let entity = DestructibleRockBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }

            TypeId::Hatchery => {
                let entity = HatcheryBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }
            TypeId::Larva => {
                let entity = LarvaBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }
            TypeId::SCV | TypeId::Probe | TypeId::Drone => {
                let entity = WorkerBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }
            TypeId::Overlord => {
                let entity = OverlordBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity);
            }

            _ => warn!("Unhandled unit: {unit:?}"),
        }
    }
}
