mod game;
mod network;

use futures_util::stream::SplitSink;
use game::game::{Game, GameState};
use network::{events::*, messages::*, protocol::Message, server::*};
use std::thread;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::unbounded_channel,
};
use tokio_tungstenite::{accept_async, tungstenite::Message as SocketMessage, WebSocketStream};

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let addr = format!("0.0.0.0:{}", PORT);

    let listener = TcpListener::bind(&addr).await.unwrap();

    let (broadcast_sender, broadcast_receiver) = unbounded_channel::<BroadcastEvent>();
    tokio::spawn(broadcast(broadcast_receiver));

    let (game_sender, game_receiver) = unbounded_channel::<GameEvent>();
    thread::spawn(move || run(broadcast_sender, game_receiver));

    println!("WebSocket server listening on {}", addr);

    let mut client_counter = 0;

    while let Ok((stream, addr)) = listener.accept().await {
        match accept_async(stream).await {
            Err(e) => println!("Error: {}", e),
            Ok(ws_stream) => {
                client_counter += 1;
                println!("Client {} connected from {}", client_counter, addr);
                tokio::spawn(listen(game_sender.clone(), ws_stream, client_counter));
            }
        }
    }
}

struct Connection {
    id: i32,
    sender: SplitSink<WebSocketStream<TcpStream>, SocketMessage>,
}

impl Connection {
    fn new(id: i32, sender: SplitSink<WebSocketStream<TcpStream>, SocketMessage>) -> Self {
        Self { id, sender }
    }
}
