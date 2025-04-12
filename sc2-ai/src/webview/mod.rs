use bevy::app::{App, Plugin, PostUpdate, Startup};

use bevy::ecs::system::{Commands, Res, ResMut, Resource};
use bevy::tasks::futures_lite::{
    StreamExt as _,
    io::{AsyncRead, AsyncWrite},
};
use futures::SinkExt as _;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use tungstenite::Message;

use crate::core::ApiMapInfo;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WebViewPlugin;

impl Plugin for WebViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_service);
        app.add_systems(PostUpdate, handle_webview_messages);
    }
}

struct AsyncTcpStream(async_net::TcpStream);

impl hyper::rt::Read for AsyncTcpStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        // Fill the read buffer with initialized data.
        let read_slice = unsafe {
            let buffer = buf.as_mut();
            buffer.as_mut_ptr().write_bytes(0, buffer.len());
            std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len())
        };

        // Read bytes from the underlying source.
        let n = match std::pin::pin!(&mut self.0).poll_read(cx, read_slice) {
            std::task::Poll::Ready(Ok(n)) => n,
            std::task::Poll::Ready(Err(e)) => return std::task::Poll::Ready(Err(e)),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        };

        // Advance the buffer.
        unsafe {
            buf.advance(n);
        }

        std::task::Poll::Ready(Ok(()))
    }
}

impl hyper::rt::Write for AsyncTcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        std::pin::pin!(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::pin!(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::pin!(&mut self.0).poll_close(cx)
    }
}

fn start_service(mut commands: Commands) {
    let (in_send, in_recv) = crossbeam_channel::unbounded();
    let (out_send, out_recv) = async_channel::unbounded();

    let internal_channels = InternalChannels {
        receiver: in_recv,
        sender: out_send,
    };

    let external_channels = ExternalChannels {
        receiver: out_recv,
        sender: in_send,
    };

    commands.insert_resource(internal_channels);

    let exec = bevy::tasks::IoTaskPool::get_or_init(|| bevy::tasks::TaskPool::new());

    exec.spawn(async move {
        let listener = async_net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let stream = AsyncTcpStream(stream);

            if let Err(error) = hyper::server::conn::http1::Builder::new()
                .serve_connection(
                    stream,
                    hyper::service::service_fn(|req| {
                        webview_handler(req, external_channels.clone())
                    }),
                )
                .with_upgrades()
                .await
            {
                tracing::error!("Web view service error: {error}")
            }
        }
    })
    .detach();
}

async fn webview_handler(
    request: hyper::Request<hyper::body::Incoming>,
    channels: ExternalChannels,
) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, anyhow::Error> {
    tracing::info!("Got request for {}", request.uri().path());

    if request
        .headers()
        .get("Upgrade")
        .is_some_and(|value| value == "websocket")
    {
        info!("Got Websocket upgrade request");
        let (response, socket) = hyper_tungstenite::upgrade(request, None).unwrap();

        let exec = bevy::tasks::IoTaskPool::get_or_init(|| bevy::tasks::TaskPool::new());
        exec.spawn(async move {
            socket_handler(socket, channels).await;
            error!("Websocket handler exited");
        })
        .detach();

        return Ok(response);
    }

    let body = match request.uri().path() {
        "/actions.html" => {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../html/actions.html"))
        }
        "/controls.html" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../html/controls.html"
        )),
        "/details.html" => {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../html/details.html"))
        }
        "/host.html" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../html/host.html")),
        "/map.html" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../html/map.html")),
        "/play.html" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../html/play.html")),
        "/progress.html" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../html/progress.html"
        )),
        _ => return Err(anyhow::anyhow!("Invalid path")),
    };

    tracing::info!("Sending response");
    Ok(hyper::Response::new(hyper::body::Bytes::from(body).into()))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum IncomingMessage {
    Ready,
    Pause,
    Scroll { x: i32, y: i32 },
    Wheel { delta: i32, x: i32, y: i32 },
    Click { x: i32, y: i32 },
    Resize { width: i32, height: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Focus {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Size2DI {
    x: i32,
    y: i32,
}

impl From<sc2_proto::common::Size2DI> for Size2DI {
    fn from(value: sc2_proto::common::Size2DI) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ImageData {
    bits_per_pixel: i32,
    size: Size2DI,
    data: Box<[u8]>,
}

impl From<sc2_proto::common::ImageData> for ImageData {
    fn from(value: sc2_proto::common::ImageData) -> Self {
        Self {
            bits_per_pixel: value.bits_per_pixel(),
            size: (*value.size.0.clone().unwrap()).into(),
            data: Box::from(value.data()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Grid {
    placement: ImageData,
    pathing: ImageData,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RenderData {
    // focus: Focus,
    grid: Grid,
    // mapbox: (),
    // viewbox: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OutgoingMessage {
    Focus { focus: Focus },
    Icons { path: String },
    Render { data: RenderData },
    Reset,
}

/// Communicate to this from a bevy system via channels.
async fn socket_handler(websocket: hyper_tungstenite::HyperWebsocket, channels: ExternalChannels) {
    info!("Starting websocket handler");
    let websocket = websocket.await.unwrap();

    #[derive(Debug)]
    enum Msg {
        Ws(Result<Message, tungstenite::Error>),
        Chan(OutgoingMessage),
    }

    let (mut ws_sink, ws_stream) = futures::StreamExt::split(websocket);

    let messages = channels.receiver.map(Msg::Chan).or(ws_stream.map(Msg::Ws));
    let mut messages = std::pin::pin!(messages);

    while let Some(message) = messages.next().await {
        info!("Handling websocket message: {message:?}");
        match message {
            Msg::Ws(Ok(Message::Ping(data))) => ws_sink.send(Message::Pong(data)).await.unwrap(),

            Msg::Ws(Ok(Message::Pong(_))) => (),

            Msg::Ws(Ok(Message::Text(bytes))) => {
                let message = serde_json::from_str::<IncomingMessage>(bytes.as_str()).unwrap();
                info!("Received websocket message: {message:?}");
                channels.sender.send(message).unwrap();
            }
            Msg::Ws(Ok(Message::Binary(bytes))) => {
                let message = serde_json::from_slice::<IncomingMessage>(&bytes).unwrap();
                info!("Received websocket message: {message:?}");
                channels.sender.send(message).unwrap();
            }
            Msg::Ws(Ok(Message::Frame(_))) => todo!(),
            Msg::Ws(Ok(Message::Close(_))) => break,
            Msg::Ws(Err(_)) => todo!(),

            Msg::Chan(msg) => {}
        }
    }
}

#[derive(Resource, Clone, Debug)]
struct ExternalChannels {
    sender: crossbeam_channel::Sender<IncomingMessage>,
    receiver: async_channel::Receiver<OutgoingMessage>,
}

#[derive(Resource, Clone, Debug)]
struct InternalChannels {
    sender: async_channel::Sender<OutgoingMessage>,
    receiver: crossbeam_channel::Receiver<IncomingMessage>,
}

fn handle_webview_messages(channels: ResMut<InternalChannels>, map_info: Res<ApiMapInfo>) {
    let message = OutgoingMessage::Render {
        data: RenderData {
            grid: Grid {
                pathing: (*map_info.pathing_grid.0.clone().unwrap()).into(),
                placement: (*map_info.placement_grid.0.clone().unwrap()).into(),
            },
        },
    };

    channels.sender.try_send(message).ok();
}
