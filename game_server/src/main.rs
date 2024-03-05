use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use protocol::Message as ProtocolMessage;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

mod protocol;

type Clients = Arc<Mutex<HashMap<i32, Client>>>;

#[tokio::main]
async fn main() {
    let addr = "localhost:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    let clients: Clients = Clients::new(Mutex::new(HashMap::new()));

    println!("WebSocket server listening on {}", addr);

    tokio::spawn(game_loop(clients.clone()));

    loop {
        let (stream, addr) = listener.accept().await.unwrap();

        let clients = clients.clone();

        tokio::spawn(handle_connection(clients, stream, addr));
    }
}

async fn game_loop(clients: Clients) {
    loop {
        let unlocked_clients = clients.lock().unwrap();

        for client in unlocked_clients.values() {
            client.send_buks(&clients);
        }
        tokio::time::sleep(Duration::from_millis(1000 / 60)).await;
    }
}

static mut COUNTER: i32 = 0;

async fn handle_connection(clients: Clients, stream: TcpStream, addr: SocketAddr) {
    if let Ok(ws_stream) = accept_async(stream).await {
        let (tx, rx) = unbounded();

        let index: i32;

        unsafe {
            index = COUNTER;
            COUNTER += 1;
        }

        clients
            .lock()
            .unwrap()
            .insert(index, Client::new(index, tx, addr));

        println!("Client {} connected", index);

        let (outgoing, incoming) = ws_stream.split();

        let broadcasted = incoming.try_for_each(|msg| {
            let clients = clients.lock().unwrap();

            clients.get(&index).unwrap().handle_message(msg);

            future::ok(())
        });

        let received = rx.map(Ok).forward(outgoing);

        pin_mut!(broadcasted, received);
        future::select(broadcasted, received).await;

        println!("Client {} disconnected", index);
        clients.lock().unwrap().remove(&index);
    }
}

#[allow(dead_code)]
struct Client {
    index: i32,
    tx: UnboundedSender<Message>,
    addr: SocketAddr,
    entity: Entity,
}

#[allow(dead_code)]
impl Client {
    fn new(index: i32, tx: UnboundedSender<Message>, addr: SocketAddr) -> Self {
        Self {
            index,
            tx,
            addr,
            entity: Entity::new(0.0, 100.0, 0.0, 5),
        }
    }

    fn talk(&self, msg: ProtocolMessage) {
        self.tx
            .unbounded_send(Message::Binary(msg.encode()))
            .unwrap();
    }

    fn handle_message(&self, msg: Message) {
        if let Message::Close(_) = msg {
            return;
        }

        println!(
            "{}: {:?}",
            self.index,
            ProtocolMessage::decode(&msg.into_data())
        );
    }
}

trait Messageable {
    fn encode(&self) -> ProtocolMessage;
}

struct XY {
    x: f32,
    y: f32,
}

impl Messageable for XY {
    fn encode(&self) -> ProtocolMessage {
        ProtocolMessage::Array(vec![
            ProtocolMessage::Float32(self.x),
            ProtocolMessage::Float32(self.y),
        ])
    }
}

struct Entity {
    id: i32,
    pos: XY,
    size: f32,
    angle: f32,
    shape: u8,
}

impl Entity {
    pub fn new(x: f32, y: f32, size: f32, shape: u8) -> Self {
        Self {
            id: 0,
            pos: XY { x, y },
            size,
            angle: 0.0,
            shape,
        }
    }
}

impl Messageable for Entity {
    fn encode(&self) -> ProtocolMessage {
        ProtocolMessage::Array(vec![
            ProtocolMessage::Int32(self.id),
            self.pos.encode(),
            ProtocolMessage::Float32(self.size),
            ProtocolMessage::Float32(self.angle),
            ProtocolMessage::Uint8(self.shape),
        ])
    }
}
