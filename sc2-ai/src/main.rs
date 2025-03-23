use core::{CorePlugin, StartupMode};
use std::net::Ipv4Addr;

use bevy::app::{App, Update};
use bevy::ecs::event::EventReader;
use bevy::ecs::schedule::IntoSystemConfigs as _;
use bevy::ecs::system::Commands;
use bevy::ecs::{event::EventWriter, query::With, system::Query};
use clap::Parser;

use game::debug::Color;
use game::geometry::{Line, Rect};
use tracing::{info, warn};

mod core;
mod game;

use game::{
    GamePlugin,
    action::MoveEvent,
    debug::DrawCommandsExt,
    entity::{self, unit::Worker},
    geometry::Vec3,
};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
struct Args {
    #[arg(short, long = "start-process")]
    start_process: bool,

    #[arg(short, long)]
    map: String,
}

use rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct WorkerRush;
impl Player for WorkerRush {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Protoss)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        for worker in &self.units.my.workers {
            worker.attack(Target::Pos(self.enemy_start), false);
        }

        // self.debug.draw_box(p0, p1, color);

        Ok(())
    }
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

fn move_workers(mut events: EventWriter<MoveEvent>, query: Query<&entity::GameId, With<Worker>>) {
    for worker in query.iter() {
        events.send(MoveEvent::new(&[*worker], Vec3::new_2d(100.0, 100.0)));
    }
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
    query: Query<(&entity::GameId, &Vec3)>,
) {
    for action in actions.read() {
        for unit_id in action.units() {
            if let Some((_, position)) = query.iter().find(|(id, _)| *id == unit_id) {
                commands.draw_line(
                    Line::new(*position, *action.destination()),
                    Color::default(),
                );
            } else {
                warn!("Unable to find unit with id {unit_id:?} referenced by move action");
            }
        }
    }
}
