use crate::{network::events::*, Connection, Game, Message, WriteMessage};
use futures_util::{FutureExt, SinkExt, StreamExt};
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::unconstrained,
};
use warp::filters::ws::{Message as SocketMessage, WebSocket};

pub fn run(sender: UnboundedSender<BroadcastEvent>, mut receiver: UnboundedReceiver<GameEvent>) {
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
                        game.remove_entity_at_id(id);
                        let _ = sender.send(BroadcastEvent::Quit(id));
                    }
                    GameEvent::Input(id, input) => match input {
                        Input::Keys(key, value) => {
                            game.set_input(id, key, value);
                        }
                        Input::Mouse(rad) => {
                            game.set_mouse(id, rad);
                        }
                        Input::MouseClick(b) => {
                            game.set_mouse_click(id, b);
                        }
                    },
                }
            }
        }

        accum += dt;
        while accum >= 0.016 {
            accum -= 0.016;

            game.update();

            let _ = sender.send(BroadcastEvent::SendState(game.get_state()));
        }

        thread::sleep(Duration::from_millis(1000 / 30));
        dt = start_time.elapsed().as_secs_f32();
        //println!("MSPT (Milliseconds per tick): {:.3}", dt);
    }
}

pub async fn broadcast(mut receiver: UnboundedReceiver<BroadcastEvent>) {
    let mut connections: HashMap<u16, Connection> = HashMap::new();

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
                    let _ = conn.sender.send(SocketMessage::binary(data.encode())).await;
                }
            }
        }
    }
}

pub async fn listen(game_sender: UnboundedSender<GameEvent>, ws_stream: WebSocket) {
    let (ws_sender, mut ws_receiver) = ws_stream.split();

    let mut connection = Connection::new(ws_sender);
    let id = connection.id;
    println!("Client {} connected", id);
    connection
        .sender
        .send(SocketMessage::binary(Message::Uint16(id).encode()))
        .await
        .unwrap();

    let _ = game_sender.send(GameEvent::Join(connection));

    while let Some(msg) = ws_receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                let decoded_message = Message::decode(&msg.into_bytes());
                if let Message::Array(vec) = decoded_message {
                    match vec.as_slice() {
                        [Message::Uint8(upordown), Message::Uint8(key)] => match upordown {
                            0 => {
                                let _ =
                                    game_sender.send(GameEvent::Input(id, Input::Keys(*key, true)));
                            }
                            1 => {
                                let _ = game_sender
                                    .send(GameEvent::Input(id, Input::Keys(*key, false)));
                            }
                            _ => {}
                        },
                        [Message::Float64(rad)] => {
                            let _ = game_sender.send(GameEvent::Input(id, Input::Mouse(*rad)));
                        }
                        [Message::Bool(b)] => {
                            let _ = game_sender.send(GameEvent::Input(id, Input::MouseClick(*b)));
                        }
                        _ => {}
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
