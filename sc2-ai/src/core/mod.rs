use std::net::Ipv4Addr;

use bevy::{
    app::{App, First, Last, Plugin, PostUpdate, PreUpdate},
    ecs::system::{Res, ResMut},
};
use process::Process;
use protobuf::MessageField;
use tracing::{error, info};

use sc2_proto::{
    common::Race,
    sc2api::{self, Difficulty, PlayerSetup, PlayerType, Request, ResponseObservation, Status},
};

mod client;
mod process;

use crate::{
    PlayerId,
    game::{ApiObservation, PlayerResources, action::MoveEvent},
};
use client::Client;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartupMode {
    Launch,
    Connect { addr: Ipv4Addr, port: u16 },
}

/// Core plugin for managing requests/responses to/from game api and translating the game state to the ECS.
///
/// Game startup is done through this plugin.
/// Systems are added to read the game state at the start of each tick and send actions back at the end of each tick.
#[derive(Debug)]
pub struct CorePlugin {
    mode: StartupMode,
    map: String,
}

impl CorePlugin {
    pub fn new(mode: StartupMode, map: String) -> Self {
        Self { mode, map }
    }
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        info!("Launching client");
        let (process, mut client) = match self.mode {
            StartupMode::Launch => process::launch_client().map(|(p, c)| (Some(p), c)),
            StartupMode::Connect { addr, port } => {
                Client::connect(&addr.to_string(), port.into()).map(|c| (None, c))
            }
        }
        .expect("Failed to start client");

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

        info!("Starting game");
        client
            .start_game(format!("{}.SC2Map", self.map), player.clone(), opponent)
            .expect("Failed to start game");

        info!("Joining game");
        let bot_id = client.join_game(player).expect("Failed to join game");

        app.insert_resource(client);
        app.insert_resource(PlayerId(bot_id));
        if let Some(process) = process {
            app.insert_resource(process);
        }

        app.init_resource::<crate::game::action::Actions>();
        app.init_resource::<ApiObservation>();
        app.init_resource::<PlayerResources>();

        app.add_event::<MoveEvent>();

        app.add_systems(First, fetch_world_state);
        app.add_systems(PreUpdate, super::create_entities);

        app.add_systems(PostUpdate, crate::game::action::action_handler::<MoveEvent>);
        app.add_systems(Last, send_actions);
    }

    fn cleanup(&self, app: &mut App) {
        if let Some(mut process) = app.world_mut().remove_resource::<Process>() {
            process.kill().expect("Failed to kill process");
            process.wait().expect("Failed to wait on process exit");
        }
    }
}

fn fetch_world_state(
    player: Res<PlayerId>,
    mut client: ResMut<Client>,
    mut api_observation: ResMut<ApiObservation>,
    mut player_resources: ResMut<PlayerResources>,
) {
    let request = {
        let mut request = Request::new();
        request.mut_observation().set_disable_fog(false);
        request
    };

    let mut response = client.send(request).inspect_err(|e| error!("{e}")).unwrap();

    if matches!(response.status(), Status::ended) {
        let result = response.observation().player_result[player.0 as usize - 1].result();
        info!("Game finished. Result: {:?}", result);
        // TODO: end app
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
        return;
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
        return;
    };

    *api_observation = ApiObservation::from(*observation);
    *player_resources = PlayerResources::from(*player);
}

fn send_actions(mut client: ResMut<Client>, mut actions: ResMut<crate::game::action::Actions>) {
    let request = {
        let mut request = Request::new();
        let api_actions = &mut request.mut_action().actions;

        // Move this tick's actions into the request.
        api_actions.append(&mut actions.0);
        request
    };

    let _response = client.send(request).inspect_err(|e| error!("{e}")).unwrap();
}
