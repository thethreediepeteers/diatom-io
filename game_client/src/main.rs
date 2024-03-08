use protocol::Message as ProtocolMessage;
use std::collections::HashMap;
use web_sys::{
    console,
    js_sys::Uint8Array,
    wasm_bindgen::{closure::Closure, prelude::*, JsCast},
    window, BinaryType, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent,
    MessageEvent, WebSocket,
};

mod protocol;

fn main() {
    let addr = "ws://localhost:3000";

    let socket = WebSocket::new(addr).unwrap();

    socket.set_binary_type(BinaryType::Arraybuffer);

    socket.clone().set_onmessage(Some(
        Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            let buf = event.data();
            let array = Uint8Array::new(&buf);
            let message = ProtocolMessage::decode(&array.to_vec());

            get_game().handle_message(message);
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));

    socket.clone().set_onopen(Some(
        Closure::<dyn FnMut()>::new(move || {
            // onopen
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));

    let window = window().unwrap();

    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    new_game(ctx);

    let cloned_socket = socket.clone();

    gloo_events::EventListener::new(&window, "keydown", move |event: &Event| {
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();

        let char = event.key().chars().next().unwrap();

        let keys = &mut get_game().keys;

        if keys.contains_key(&char) {
            keys.insert(char, true);
        }

        cloned_socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Bool(*keys.get(&'w').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'a').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'s').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'d').unwrap()),
                ])
                .encode(),
            )
            .unwrap_throw();
    })
    .forget();

    gloo_events::EventListener::new(&window, "keyup", move |event: &Event| {
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();

        let char = event.key().chars().next().unwrap();

        let keys = &mut get_game().keys;

        if keys.contains_key(&char) {
            keys.insert(char, false);
        }

        socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Bool(*keys.get(&'w').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'a').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'s').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'d').unwrap()),
                ])
                .encode(),
            )
            .unwrap_throw();
    })
    .forget();

    get_game().tick();
}

static mut GAME: Option<Box<Game>> = None;

fn new_game(ctx: CanvasRenderingContext2d) {
    unsafe { GAME = Some(Box::new(Game::new(ctx))) }
}

fn get_game() -> &'static mut Game {
    unsafe { GAME.as_mut().unwrap_throw() }
}

type Entities = HashMap<i32, Entity>;

struct Game {
    entities: Entities,
    ctx: CanvasRenderingContext2d,
    keys: HashMap<char, bool>,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            entities: Entities::new(),
            ctx,
            keys: HashMap::from([('a', false), ('d', false), ('w', false), ('s', false)]),
        }
    }

    fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            for msg in &vec {
                if let ProtocolMessage::Array(v) = msg {
                    if let [ProtocolMessage::Int32(id), ProtocolMessage::Array(pos), ProtocolMessage::Float32(size)] =
                        v.as_slice()
                    {
                        if let [ProtocolMessage::Float32(x), ProtocolMessage::Float32(y)] =
                            pos.as_slice()
                        {
                            self.entities.insert(*id, Entity::new(*id, *x, *y, *size));
                            console::log_1(&format!("{:?}", self.keys).into());
                        }
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        let ctx = &self.ctx;
        ctx.clear_rect(
            0.0,
            0.0,
            window()
                .unwrap_throw()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap(),
            window()
                .unwrap_throw()
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap(),
        );
        self.entities.values().for_each(|entity| {
            ctx.fill_rect(
                entity.pos.x.into(),
                entity.pos.y.into(),
                entity.size.into(),
                entity.size.into(),
            );
        });
        let closure = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            let game = get_game();
            game.tick();
        }));
        window()
            .unwrap_throw()
            .request_animation_frame(closure.into_js_value().as_ref().unchecked_ref())
            .unwrap_throw();
    }
}

#[derive(Debug)]
struct XY {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Entity {
    id: i32,
    pos: XY,
    size: f32,
}

impl Entity {
    fn new(id: i32, x: f32, y: f32, size: f32) -> Self {
        Self {
            id,
            pos: XY { x, y },
            size,
        }
    }
}
