mod protocol;

use futures_util::StreamExt;
use protocol::Message;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, unbounded_channel, UnboundedSender},
};
use tokio_tungstenite::{accept_async, WebSocketStream};

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let addr = format!("localhost:{}", PORT);

    let listener = TcpListener::bind(&addr).await.unwrap();

    let (broadcast_sender, broadcast_receiver) = unbounded_channel::<BroadcastEvent>();
    // TODO: broadcast loop
    // tokio::spawn();

    let (game_sender, game_receiver) = unbounded_channel::<GameEvent>();
    // TODO: game loop 
    // thread::spawn(move || )

    println!("Listening on {}", addr);

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

async fn listen(
    game_sender: UnboundedSender<GameEvent>,
    ws_stream: WebSocketStream<TcpStream>,
    id: i32,
) {
    let (ws_sender, mut ws_receiver) = ws_stream.split();

    let connection = Connection::new();
    let _ = game_sender.send(GameEvent::Join(connection));

    while let Some(msg) = ws_receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
            } else if msg.is_close() {
                break;
            }
        } else {
            break;
        }
    }

    game_sender.send(GameEvent::Quit(id)).unwrap();
}

struct Connection {}

impl Connection {
    fn new() -> Self {
        Self {}
    }
}

struct GameState {}

struct GameInput {
    x: f32,
    y: f32,
    pressed: bool,
}

enum GameEvent {
    Join(Connection),
    Quit(i32),
    Input(i32, GameInput),
}

enum BroadcastEvent {
    Join(Connection),
    Quit(Connection),
    SendState(GameState),
}
