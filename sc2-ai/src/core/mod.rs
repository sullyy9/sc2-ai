use std::net::Ipv4Addr;

use bevy::{
    app::{App, AppExit, First, Last, Plugin, PreStartup},
    ecs::{
        event::EventWriter,
        system::{Res, ResMut, Resource},
    },
};
use protobuf::MessageField;
use tracing::{error, info};

use sc2_proto::{
    common::Race,
    sc2api::{
        self, Difficulty, PlayerSetup, PlayerType, Request, ResponseGameInfo, ResponseObservation,
        Status,
    },
};

mod action;
mod client;
mod command;
mod process;

use client::Client;
use process::Process;

pub use action::Actions;
pub use command::DebugCommands;

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

        app.init_resource::<Actions>();
        app.init_resource::<DebugCommands>();

        app.init_resource::<ApiMapInfo>();
        app.init_resource::<ApiObservation>();
        app.init_resource::<PlayerCommon>();

        app.add_systems(PreStartup, fetch_game_info);
        app.add_systems(PreStartup, fetch_world_state);

        app.add_systems(First, fetch_world_state);
        app.add_systems(Last, send_request);
    }

    fn cleanup(&self, app: &mut App) {
        if let Some(mut process) = app.world_mut().remove_resource::<Process>() {
            process.kill().expect("Failed to kill process");
            process.wait().expect("Failed to wait on process exit");
        }
    }
}

#[derive(Resource, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct PlayerId(u32);

/// Observation provided by the game API.
///
/// This contains things like visibile units, effects and events. It is stored as a resource in the
/// ECS in order for other systems to read and generate other entities from it.
#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct ApiObservation(sc2_proto::raw::ObservationRaw);

impl std::ops::Deref for ApiObservation {
    type Target = sc2_proto::raw::ObservationRaw;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct PlayerCommon(sc2_proto::sc2api::PlayerCommon);

impl std::ops::Deref for PlayerCommon {
    type Target = sc2_proto::sc2api::PlayerCommon;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct ApiMapInfo(sc2_proto::raw::StartRaw);

impl std::ops::Deref for ApiMapInfo {
    type Target = sc2_proto::raw::StartRaw;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn fetch_game_info(mut client: ResMut<Client>, mut api_map: ResMut<ApiMapInfo>) {
    let request = {
        let mut request = Request::new();
        request.mut_game_info();
        request
    };

    let mut response = client.send(request).inspect_err(|e| error!("{e}")).unwrap();

    let ResponseGameInfo {
        start_raw: MessageField(Some(start_raw)),
        ..
    } = response.take_game_info()
    else {
        error!("Api response contains unexpected pattern");
        return;
    };

    *api_map = ApiMapInfo(*start_raw);
}

fn fetch_world_state(
    player: Res<PlayerId>,
    mut client: ResMut<Client>,
    mut api_observation: ResMut<ApiObservation>,
    mut player_resources: ResMut<PlayerCommon>,
    mut exit: EventWriter<AppExit>,
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
        exit.send(AppExit::Success);
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

    *api_observation = ApiObservation(*observation);
    *player_resources = PlayerCommon(*player);
}

fn send_request(
    mut client: ResMut<Client>,
    mut actions: ResMut<Actions>,
    mut commands: ResMut<DebugCommands>,
) {
    tracing::info!(
        "Sending Request | actions: {} | commands: {}",
        actions.len(),
        commands.len()
    );

    let request = {
        let mut complete_request = Request::new();

        let request = &mut complete_request.mut_action();
        request.actions.append(&mut actions);

        complete_request
    };

    let _response = client.send(request).inspect_err(|e| error!("{e}")).unwrap();

    let request = {
        let mut complete_request = Request::new();

        let request = &mut complete_request.mut_debug().debug;
        request.append(&mut commands);

        complete_request
    };

    let _response = client.send(request).inspect_err(|e| error!("{e}")).unwrap();
}
