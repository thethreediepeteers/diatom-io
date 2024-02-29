use gloo::console::console;
use gloo_render::request_animation_frame;
use protocol::Message as ProtocolMessage;
use std::collections::HashMap;
use web_sys::{
    console,
    js_sys::{Reflect::get, Uint8Array},
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

type Entities = HashMap<i32, Entity>;

static mut GAME: Option<Box<Game>> = None;

fn new_game(ctx: CanvasRenderingContext2d) {
    unsafe { GAME = Some(Box::new(Game::new(ctx))) }
}

fn get_game() -> &'static mut Game {
    unsafe { GAME.as_mut().unwrap_throw() }
}

struct Game {
    entities: Entities,
    counter: i32,
    ctx: CanvasRenderingContext2d,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            entities: Entities::new(),
            counter: 0,
            ctx,
        }
    }

    fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            console::log_1(&format!("handle_message: {:?}", vec).into());
        }
    }

    fn tick(&mut self) {
        let ctx = &self.ctx;
        self.entities.values().for_each(|entity| {
            ctx.fill_rect(entity.pos.x, entity.pos.y, 50.0, 50.0);
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

struct Entity {
    pos: XY,
    size: u8,
}

struct XY {
    x: f64,
    y: f64,
}
