use core::{CorePlugin, StartupMode};
use std::net::Ipv4Addr;

use bevy::app::{App, Update};
use bevy::ecs::{
    event::EventWriter,
    query::With,
    system::{Commands, Query, Res, Resource},
};
use clap::Parser;

use num_traits::FromPrimitive as _;
use tracing::{info, warn};

mod core;
mod game;

use game::{
    ApiObservation,
    action::MoveEvent,
    entity::{
        self, EntityBundle, Position,
        building::{HatcheryBundle, LarvaBundle},
        map::{DestructibleRockBundle, MineralPatchBundle, VespeneGeyserBundle},
        unit::{OverlordBundle, Worker, WorkerBundle},
    },
};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
struct Args {
    #[arg(short, long)]
    start_process: bool,

    #[arg(short, long)]
    map: String,
}

#[derive(Resource, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct PlayerId(u32);

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();

    let args = Args::parse();

    let core = if args.start_process {
        CorePlugin::new(StartupMode::Launch, args.map)
    } else {
        CorePlugin::new(
            StartupMode::Connect {
                addr: Ipv4Addr::new(127, 0, 0, 1),
                port: 8167,
            },
            args.map,
        )
    };

    info!("Setting up ECS");

    let mut app = App::new();

    app.add_plugins(core);

    app.add_systems(Update, move_workers);

    app.set_runner(|mut app| {
        loop {
            app.update();
            if let Some(exit) = app.should_exit() {
                return exit;
            }
        }
    });

    info!("Running game");
    app.run();

    Ok(())
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

fn move_workers(mut events: EventWriter<MoveEvent>, query: Query<&entity::Id, With<Worker>>) {
    for worker in query.iter() {
        events.send(MoveEvent::new(&[*worker], Position::new(0.0, 0.0)));
    }
}
