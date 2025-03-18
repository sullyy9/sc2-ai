use std::net::TcpStream;

use anyhow::anyhow;
use protobuf::Message as _;
use sc2_proto::sc2api::{InterfaceOptions, PlayerSetup, Request, Response, response_create_game};
use thiserror::Error;
use tungstenite::{Message, WebSocket, stream::MaybeTlsStream};

use crate::process;

pub struct Client(WebSocket<MaybeTlsStream<TcpStream>>);

#[derive(Error, Debug, Clone)]
#[error("{error:?}: {detail}")]
pub struct ApiError {
    error: response_create_game::Error,
    detail: String,
}

impl Client {
    pub fn connect(host: &str, port: i32) -> Result<Self, anyhow::Error> {
        let (ws, _rs) = loop {
            if let Ok(result) = tungstenite::connect(format!("ws://{}:{}/sc2api", host, port)) {
                break result;
            }
        };
        Ok(Self(ws))
    }

    pub fn send(&mut self, request: Request) -> Result<Response, anyhow::Error> {
        self.0.send(Message::binary(request.write_to_bytes()?))?;

        let msg = self.0.read()?;
        let mut response = Response::new();
        response.merge_from_bytes(&msg.into_data())?;

        Ok(response)
    }

    pub fn start_game(
        &mut self,
        map: &str,
        player: PlayerSetup,
        opponent: PlayerSetup,
    ) -> Result<(), anyhow::Error> {
        let request = {
            let mut request = Request::new();
            let req_create_game = request.mut_create_game();

            let map = process::map_path(map);

            req_create_game.mut_local_map().set_map_path(map);
            req_create_game.player_setup = vec![player, opponent];
            req_create_game.set_realtime(true);
            request
        };

        let response = self.send(request)?;
        {
            let res_create_game = response.create_game();
            if res_create_game.has_error() {
                return Err(ApiError {
                    error: res_create_game.error(),
                    detail: res_create_game.error_details().to_owned(),
                }
                .into());
            }
        }

        Ok(())
    }

    pub fn join_game(&mut self, player: PlayerSetup) -> Result<u32, anyhow::Error> {
        let mut request = Request::new();

        let game = request.mut_join_game();
        game.set_race(player.race());
        *game.mut_player_name() = player.player_name().to_owned();

        game.options.0 = Some(Box::new(InterfaceOptions {
            raw: Some(true),
            score: Some(true),
            show_cloaked: Some(true),
            show_burrowed_shadows: Some(true),
            show_placeholders: Some(true),
            raw_affects_selection: Some(false),
            raw_crop_to_playable_area: Some(false),
            ..Default::default()
        }));

        let response = self.send(request)?;
        let response = response.join_game();

        if response.has_error() {
            return Err(anyhow!(
                "{:?}: {}",
                response.error(),
                response.error_details().to_owned(),
            ));
        };

        Ok(response.player_id())
    }
}
