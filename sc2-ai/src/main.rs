use bevy::{
    app::{App, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs as _,
        system::{Commands, Query},
    },
};
use clap::Parser;
use core::{CorePlugin, StartupMode};
use std::net::Ipv4Addr;
use tracing::{info, warn};

mod core;
mod game;

use game::{
    GamePlugin,
    action::{ActionCommandsExt, MoveEvent},
    debug::{Color, DrawCommandsExt},
    geometry::{Line, Cuboid, Sphere, Vec3},
};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
struct Args {
    #[arg(long = "start-process")]
    start_process: bool,

    #[arg(long, default_value_t = 8167)]
    port: u16,

    #[arg(long = "step-rate", group = "step-rate", default_value_t = 22)]
    step_rate: u64,

    #[arg(long, group = "step-rate")]
    realtime: bool,

    #[arg(short, long)]
    map: String,
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();

    let args = Args::parse();

    let core = if args.start_process {
        CorePlugin::new(StartupMode::Launch, args.map, args.realtime)
    } else {
        CorePlugin::new(
            StartupMode::Connect {
                addr: Ipv4Addr::new(127, 0, 0, 1),
                port: args.port,
            },
            args.map,
            args.realtime,
        )
    };

    info!("Setting up ECS");

    let mut app = App::new();

    app.add_plugins(core).add_plugins(GamePlugin);

    app.add_systems(
        Update,
        (move_workers, highlight_workers, draw_move_actions).chain(),
    );

    app.set_runner(move |mut app| {
        let step_period = std::time::Duration::from_millis(1000 / args.step_rate);
        let mut next_step = std::time::Instant::now() + step_period;

        loop {
            if !args.realtime {
                let now = std::time::Instant::now();
                std::thread::sleep(next_step.duration_since(now));
                next_step = now + step_period;
            }

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

fn move_workers(mut commands: Commands, query: Query<Entity, With<Worker>>) {
    let workers = query.iter().collect::<Box<_>>();
    commands.move_units(&workers, Vec3::new_2d(100.0, 100.0));
}

fn highlight_workers(mut commands: Commands, query: Query<&Vec3, With<Worker>>) {
    for &pos in query.iter() {
        commands.draw_box(Cuboid::from_base_center(pos, Worker::SIZE), Color::GREEN);
        commands.draw_text("Worker", pos, Color::default());
    }
}

fn draw_move_actions(
    mut commands: Commands,
    mut actions: EventReader<MoveEvent>,
    query: Query<(Entity, &Vec3)>,
) {
    for action in actions.read() {
        let destination = *action.destination();

        for &unit in action.units() {
            let Ok((_, &position)) = query
                .get(unit)
                .inspect_err(|e| warn!("When querying entity: {e}"))
            else {
                continue;
            };

            commands.draw_line(Line::new(position, destination), Color::default());
        }
    }
}
