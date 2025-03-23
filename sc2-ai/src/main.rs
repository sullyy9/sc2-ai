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
    entity::{building::Hatchery, unit::Worker},
    geometry::{Line, Rect, Vec3},
};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
struct Args {
    #[arg(short, long = "start-process")]
    start_process: bool,

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

    app.add_plugins(core).add_plugins(GamePlugin);

    app.add_systems(
        Update,
        (move_workers, highlight_workers, draw_move_actions).chain(),
    );

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

fn move_workers(mut commands: Commands, query: Query<Entity, With<Worker>>) {
    let workers = query.iter().collect::<Box<_>>();
    commands.move_units(&workers, Vec3::new_2d(100.0, 100.0));
}

fn highlight_workers(mut commands: Commands, query: Query<&Vec3, With<Worker>>) {
    let box_size = Vec3::new_3d(1.0, 1.0, 1.0);
    let box_offset = Vec3::new_3d(0.0, 0.0, box_size.z / 2.0);

    for pos in query.iter().map(|pos| pos + box_offset) {
        commands.draw_box(Rect::from_center(pos, box_size), Color::GREEN);
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
