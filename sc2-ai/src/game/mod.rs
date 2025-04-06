//! Deals with transforming data between the SC2 API and types moe suitable for use in ECS systems.

use action::MoveEvent;
use bevy::{
    app::{App, MainScheduleOrder, Plugin, Startup, Update},
    ecs::{
        schedule::{IntoSystemConfigs, ScheduleLabel},
        system::{Commands, Res, ResMut},
    },
};
use entity::{
    EntityBundle, EntityFound, EntityIdMap, GameId,
    building::{HatcheryBundle, LarvaBundle},
    map::{
        DestructibleRockBundle, MineralPatch, MineralPatchBundle, VespeneGeyser,
        VespeneGeyserBundle,
    },
    unit::{OverlordBundle, WorkerBundle},
};
use geometry::Vec3;
use map::{HeightMap, PlacementGrid};
use num_traits::FromPrimitive;
use tracing::warn;

use crate::core::ApiObservation;

pub mod action;
pub mod debug;
pub mod entity;
pub mod geometry;
pub mod map;
mod player;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GamePlugin;

#[derive(ScheduleLabel, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct DataInit;

#[derive(ScheduleLabel, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct DataUpdate;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(DataInit);
        app.init_schedule(DataUpdate);

        let mut schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        schedule_order.insert_startup_before(Startup, DataInit);
        schedule_order.insert_before(Update, DataUpdate);

        app.init_resource::<EntityIdMap>();

        app.add_event::<MoveEvent>();
        app.add_event::<EntityFound<MineralPatch>>();
        app.add_event::<EntityFound<VespeneGeyser>>();

        app.add_systems(
            DataInit,
            (
                create_entities,
                HeightMap::init,
                PlacementGrid::init,
                PlacementGrid::entity_found_handler::<MineralPatch>,
                PlacementGrid::entity_found_handler::<VespeneGeyser>,
            )
                .chain(),
        );
        app.add_systems(DataUpdate, update_entities);
    }
}

/// Create entities by evaluating the [`Observation`] resource.
fn create_entities(
    mut commands: Commands,
    observation: Res<ApiObservation>,
    mut map: ResMut<EntityIdMap>,
) {
    for unit in &observation.units {
        use sc2_proto::unit::TypeId;
        let unit_type = TypeId::from_u32(unit.unit_type()).unwrap();

        let entity = match unit_type {
            TypeId::MineralField | TypeId::MineralField450 | TypeId::MineralField750 => {
                let entity = MineralPatchBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };

                let entity = commands.spawn(entity).id();
                commands.send_event(EntityFound::<MineralPatch>::from(entity));
                entity
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

                let entity = commands.spawn(entity).id();
                commands.send_event(EntityFound::<VespeneGeyser>::from(entity));
                entity
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
                commands.spawn(entity).id()
            }

            TypeId::Hatchery => {
                let entity = HatcheryBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity).id()
            }
            TypeId::Larva => {
                let entity = LarvaBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };
                commands.spawn(entity).id()
            }
            TypeId::SCV | TypeId::Probe | TypeId::Drone => {
                let entity = WorkerBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };

                commands.spawn(entity).id()
            }
            TypeId::Overlord => {
                let entity = OverlordBundle {
                    unit: EntityBundle::from(unit.clone()),
                    ..Default::default()
                };

                commands.spawn(entity).id()
            }

            _ => {
                warn!("Unhandled unit: {unit:?}");
                continue;
            }
        };

        map.insert(GameId(unit.tag()), entity);
    }
}

fn update_entities(
    mut commands: Commands,
    observation: Res<ApiObservation>,
    map: Res<EntityIdMap>,
) {
    for unit in &observation.units {
        use sc2_proto::unit::TypeId;
        let unit_type = TypeId::from_u32(unit.unit_type()).unwrap();

        match unit_type {
            TypeId::MineralField | TypeId::MineralField450 | TypeId::MineralField750 => {}
            TypeId::VespeneGeyser
            | TypeId::SpacePlatformGeyser
            | TypeId::RichVespeneGeyser
            | TypeId::ProtossVespeneGeyser
            | TypeId::PurifierVespeneGeyser
            | TypeId::ShakurasVespeneGeyser => {}
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
            | TypeId::UnbuildablePlatesDestructible => {}

            TypeId::Hatchery => {}

            TypeId::Larva | TypeId::Overlord | TypeId::SCV | TypeId::Probe | TypeId::Drone => {
                let entity = map
                    .get(&GameId(unit.tag()))
                    .expect("Missing entity in EntityIdMap");
                let mut entity = commands.entity(*entity);
                entity.insert(Vec3::from(unit.pos.get_or_default().clone()));
            }

            _ => (),
        }
    }
}
