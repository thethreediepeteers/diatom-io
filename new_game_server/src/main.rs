mod protocol;

use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};
use futures_util::{stream::SplitSink, FutureExt, SinkExt, StreamExt};
use protocol::Message;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::unconstrained,
};
use tokio_tungstenite::{accept_async, tungstenite::Message as SocketMessage, WebSocketStream};

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let addr = format!("localhost:{}", PORT);

    let listener = TcpListener::bind(&addr).await.unwrap();

    let (broadcast_sender, broadcast_receiver) = unbounded_channel::<BroadcastEvent>();
    tokio::spawn(broadcast(broadcast_receiver));

    let (game_sender, game_receiver) = unbounded_channel::<GameEvent>();
    thread::spawn(move || run(broadcast_sender, game_receiver));

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

fn run(sender: UnboundedSender<BroadcastEvent>, mut receiver: UnboundedReceiver<GameEvent>) {
    let mut game = Game::new();

    let mut accum = 0.0;
    let mut dt = 0.0;

    loop {
        let start_time = Instant::now();

        while let Some(is_event) = unconstrained(receiver.recv()).now_or_never() {
            if let Some(event) = is_event {
                match event {
                    GameEvent::Join(conn) => {
                        game.add_player(conn.id);
                        let _ = sender.send(BroadcastEvent::Join(conn));
                    }
                    GameEvent::Quit(id) => {
                        game.remove_player(id);
                        let _ = sender.send(BroadcastEvent::Quit(id));
                    }
                    GameEvent::Input(id, input) => {
                        game.set_input(id, input);
                    }
                }
            }
        }

        accum += dt;
        while accum >= 0.016 {
            accum -= 0.016;

            game.update();

            let _ = sender.send(BroadcastEvent::SendState(game.get_state()));
        }

        thread::sleep(Duration::from_millis(1000 / 60));
        dt = start_time.elapsed().as_secs_f32();
    }
}

async fn broadcast(mut receiver: UnboundedReceiver<BroadcastEvent>) {
    let mut connections: HashMap<i32, Connection> = HashMap::new();

    while let Some(event) = receiver.recv().await {
        match event {
            BroadcastEvent::Join(conn) => {
                connections.insert(conn.id, conn);
            }

            BroadcastEvent::Quit(id) => {
                connections.remove(&id);
                println!("Client {} disconnected", id);
            }

            BroadcastEvent::SendState(state) => {
                for conn in connections.values_mut() {
                    let data = state.encode();
                    let _ = conn
                        .sender
                        .send(SocketMessage::Binary(data.encode()))
                        .await;
                }
            }
        }
    }
}

async fn listen(
    game_sender: UnboundedSender<GameEvent>,
    ws_stream: WebSocketStream<TcpStream>,
    id: i32,
) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    ws_sender.send(SocketMessage::Binary(Message::Int32(id).encode())).await.unwrap();

    let connection = Connection::new(id, ws_sender);
    let _ = game_sender.send(GameEvent::Join(connection));

    while let Some(msg) = ws_receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                let decoded_message = Message::decode(&msg.into_data());
                if let Message::Array(vec) = decoded_message {
                    if let Some(input) = GameInput::from_vec(vec) {
                        let _ = game_sender.send(GameEvent::Input(id, input));
                    }
                }
            } else if msg.is_close() {
                break;
            }
        } else {
            break;
        }
    }

    game_sender.send(GameEvent::Quit(id)).unwrap();
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

struct GameState {
    entities: Vec<Entity>,
}

#[derive(Debug)]
struct GameInput {
    keys: HashMap<char, bool>,
}

enum GameEvent {
    Join(Connection),
    Quit(i32),
    Input(i32, GameInput),
}

enum BroadcastEvent {
    Join(Connection),
    Quit(i32),
    SendState(GameState),
}

#[derive(Clone)]
#[derive(Debug)]
struct XY {
    x: f32,
    y: f32,
}

impl XY {
    fn add(&mut self, vec: &Self) {
        self.x += vec.x;
        self.y += vec.y;
    }
}

#[derive(Clone)]
struct Entity {
    id: i32,
    pos: XY,
    vel: XY,
    size: f32,
    keys: HashMap<char, bool>,
}

trait WriteMessage {
    fn encode(&self) -> Message;
}

trait ReadMessage {
    fn from_vec(vec: Vec<Message>) -> Option<Self>
    where
        Self: Sized;
}

impl WriteMessage for XY {
    fn encode(&self) -> Message {
        Message::Array(vec![Message::Float32(self.x), Message::Float32(self.y)])
    }
}

impl WriteMessage for Entity {
    fn encode(&self) -> Message {
        Message::Array(vec![
            Message::Int32(self.id),
            self.pos.encode(),
            Message::Float32(self.size),
        ])
    }
}

impl ReadMessage for GameInput {
    fn from_vec(vec: Vec<Message>) -> Option<Self> {
        if let [Message::Bool(w), Message::Bool(a), Message::Bool(s), Message::Bool(d)] =
            vec.as_slice()
        {
            let mut input = Self {
                keys: HashMap::new(),
            };

            input.keys.insert('w', *w);
            input.keys.insert('a', *a);
            input.keys.insert('s', *s);
            input.keys.insert('d', *d);

            Some(input)
        } else {
            None
        }
    }
}

impl WriteMessage for GameState {
    fn encode(&self) -> Message {
        let mut message = vec![];
        for entity in &self.entities {
            message.push(entity.encode());
        }
        Message::Array(message)
    }
}

struct Game {
    players: HashMap<i32, Entity>,
}

impl Game {
    fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    fn add_player(&mut self, id: i32) {
        self.players.insert(
            id,
            Entity {
                id,
                pos: XY { x: 0.0, y: 0.0 },
                vel: XY { x: 0.0, y: 0.0 },
                size: 65.0,
                keys: HashMap::from([('w', false), ('a', false), ('s', false), ('d', false)]),
            },
        );
    }

    fn remove_player(&mut self, id: i32) {
        self.players.remove(&id);
    }

    fn set_input(&mut self, id: i32, input: GameInput) {
        if let Some(entity) = self.players.get_mut(&id) {
            entity.keys = input.keys;
        }
    }

    fn update(&mut self) {
        for entity in self.players.values_mut() {
            if entity.keys[&'w'] {
                entity.vel.y -= 1.0;
            }
            if entity.keys[&'a'] {
                entity.vel.x -= 1.0;
            }
            if entity.keys[&'s'] {
                entity.vel.y += 1.0;
            }
            if entity.keys[&'d'] {
                entity.vel.x += 1.0;
            }

            entity.pos.add(&entity.vel);
            entity.vel.x *= 0.9;
            entity.vel.y *= 0.9;
        }
    }

    fn get_state(&self) -> GameState {
        let mut state = GameState {
            entities: Vec::new(),
        };

        for entity in self.players.values() {
            state.entities.push(entity.clone());
        }

        state
    }
}
