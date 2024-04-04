mod game;
mod network;

use futures_util::stream::SplitSink;
use game::game::{Game, GameState};
use network::{events::*, messages::*, protocol::Message, server::*};
use serde_json::{json, Value};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    thread,
};
use tokio::sync::mpsc::unbounded_channel;
use warp::{
    ws::{Message as SocketMessage, WebSocket},
    Filter,
};

const PORT: u16 = 3000;
static mut MOCKUPS: Value = Value::Null;
static mut CLIENT_COUNTER: i32 = 0;

#[tokio::main]
async fn main() {
    unsafe {
        MOCKUPS = json!({"test": "lkjfaslkjfd"});
    }
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), PORT);

    let (broadcast_sender, broadcast_receiver) = unbounded_channel::<BroadcastEvent>();
    tokio::spawn(broadcast(broadcast_receiver));

    let (game_sender, game_receiver) = unbounded_channel::<GameEvent>();
    thread::spawn(move || run(broadcast_sender, game_receiver));

    println!("WebSocket server listening on {}", addr);

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let game_sender = game_sender.clone();
            ws.on_upgrade(|socket| async {
                tokio::spawn(listen(game_sender, socket));
            })
        })
        .or(warp::path("mockups.json")
            .map(|| unsafe { warp::reply::json(&MOCKUPS) })
            .with(warp::cors().allow_any_origin()));

    warp::serve(routes).run(addr).await;
}

struct Connection {
    id: i32,
    sender: SplitSink<WebSocket, SocketMessage>,
}

impl Connection {
    fn new(sender: SplitSink<WebSocket, SocketMessage>) -> Self {
        unsafe {
            CLIENT_COUNTER += 1;
            Self {
                id: CLIENT_COUNTER,
                sender,
            }
        }
    }
}
