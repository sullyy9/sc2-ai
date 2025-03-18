use bevy_ecs::{
    event::{EventWriter, Events}, query::With, schedule::{Schedule, ScheduleLabel}, system::{Commands, Query, Res}, world::World
};
use num_traits::FromPrimitive as _;
use protobuf::MessageField;
use sc2_proto::{
    common::Race,
    sc2api::{self, Difficulty, PlayerSetup, PlayerType, Request, ResponseObservation, Status},
};
use tracing::{error, info, warn};

mod client;
mod game;
mod process;

use game::{
    action::MoveEvent, entity::{
        self, building::{HatcheryBundle, LarvaBundle}, map::{DestructibleRockBundle, MineralPatchBundle, VespeneGeyserBundle}, unit::{OverlordBundle, Worker, WorkerBundle}, EntityBundle, Position
    }, ApiObservation, PlayerResources
};

#[derive(ScheduleLabel, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Startup;

#[derive(ScheduleLabel, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct StepRun;

#[derive(ScheduleLabel, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct StepEnd;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();

    info!("starting process");
    let (mut process, mut client) = process::launch_client()?;

    info!("Starting game");
    let player = PlayerSetup {
        type_: Some(PlayerType::Participant.into()),
        race: Some(Race::Zerg.into()),
        player_name: Some("Tomobot".to_owned()),
        ..Default::default()
    };

    let opponent = PlayerSetup {
        type_: Some(PlayerType::Computer.into()),
        race: Some(Race::Terran.into()),
        difficulty: Some(Difficulty::Medium.into()),
        ..Default::default()
    };

    client.start_game("Ladder2019Season3/TritonLE", player.clone(), opponent)?;

    info!("Joining game");
    let bot_id = client.join_game(player)?;

    info!("Setting up ECS");
    let mut world = World::new();

    let startup = {
        let mut schedule = Schedule::new(Startup);
        schedule.add_systems(create_entities);
        schedule
    };

    let step_run = {
        let mut schedule = Schedule::new(StepRun);
        schedule.add_systems(move_workers);
        schedule
    };

    let step_end = {
        let mut schedule = Schedule::new(StepEnd);
        schedule.add_systems(game::action::action_handler::<MoveEvent>);
        schedule
    };

    world.add_schedule(startup);
    world.add_schedule(step_run);
    world.add_schedule(step_end);

    world.insert_resource(Events::<MoveEvent>::default());
    world.insert_resource(game::action::Actions::default());

    info!("Running game");
    for step in 0.. {
        // info!("Playing step {step}");
        let request = {
            let mut request = Request::new();
            request.mut_observation().set_disable_fog(false);
            request
        };

        let mut response = client.send(request)?;
        if matches!(response.status(), Status::ended) {
            let result = response.observation().player_result[bot_id as usize - 1].result();
            info!("Game finished. Result: {:?}", result);
            break;
        }

        let ResponseObservation {
            actions: _,
            action_errors: _,
            observation: MessageField(Some(observation)),
            chat: _,
            ..
        } = response.take_observation()
        else {
            error!("Api response contains unexpected pattern");
            continue;
        };

        let sc2api::Observation {
            game_loop: _,
            player_common: MessageField(Some(player)),
            alerts: _,
            abilities: _,
            score: _,
            raw_data: MessageField(Some(observation)),
            ..
        } = *observation
        else {
            error!("Observation contains unexpected pattern");
            continue;
        };

        // Transfer data into ECS.
        world.insert_resource::<ApiObservation>(ApiObservation::from(*observation));
        world.insert_resource::<PlayerResources>(PlayerResources::from(*player));

        // Run startup systems on first step
        if step == 0 {
            world.run_schedule(Startup);
        }

        world.run_schedule(StepRun);

        world.run_schedule(StepEnd);

        let request = {
            let mut request = Request::new();
            let api_actions = &mut request.mut_action().actions;

            // Move this tick's actions into the request.
            let mut actions = world.resource_mut::<game::action::Actions>();
            api_actions.append(&mut actions.0);

            request
        };

        let _response = client.send(request)?;
        // Check response here.
    }

    process.kill()?;
    process.wait()?;
    Ok(())
}

/// Create entities by evaluating the [`Observation`] resource.
fn create_entities(mut commands: Commands, observation: Res<ApiObservation>) {
    for unit in &observation.into_inner().units {
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
