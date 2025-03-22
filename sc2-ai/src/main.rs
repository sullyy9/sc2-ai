use core::{CorePlugin, StartupMode};
use std::net::Ipv4Addr;

use bevy::app::{App, Update};
use bevy::ecs::{event::EventWriter, query::With, system::Query};
use clap::Parser;

use game::GamePlugin;
use tracing::info;

mod core;
mod game;

use game::{
    action::MoveEvent,
    entity::{self, Position, unit::Worker},
};

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
struct Args {
    #[arg(short, long)]
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

fn move_workers(mut events: EventWriter<MoveEvent>, query: Query<&entity::Id, With<Worker>>) {
    for worker in query.iter() {
        events.send(MoveEvent::new(&[*worker], Position::new(0.0, 0.0)));
    }
}
