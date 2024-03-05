use protocol::Message as ProtocolMessage;
use std::collections::HashMap;
use web_sys::{
    console,
    js_sys::Uint8Array,
    wasm_bindgen::{closure::Closure, prelude::*, JsCast},
    window, BinaryType, CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, WebSocket,
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

    let game = get_game();
    game.tick();
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
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            entities: Entities::new(),
            ctx,
        }
    }

    fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            if let [ProtocolMessage::Int32(id), ProtocolMessage::Array(pos), ProtocolMessage::Float32(size), ProtocolMessage::Float32(angle), ProtocolMessage::Uint8(shape)] =
                vec.as_slice()
            {
                if let [ProtocolMessage::Float32(x), ProtocolMessage::Float32(y)] = pos.as_slice() {
                    self.entities
                        .entry(*id)
                        .or_insert(Entity::new(*id, *x, *y, *size, *angle, *shape));
                }
                console::log_1(&format!("{:?}", &self.entities).into());
            }
        }
    }

    fn tick(&mut self) {
        let ctx = &self.ctx;
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
        web_sys::window()
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
    angle: f32,
    shape: u8,
}

impl Entity {
    fn new(id: i32, x: f32, y: f32, size: f32, angle: f32, shape: u8) -> Self {
        Self {
            id,
            pos: XY { x, y },
            size,
            angle,
            shape,
        }
    }
}
