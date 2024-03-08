mod protocol;

extern crate console_error_panic_hook;

use gloo::utils::{document, window};
use protocol::Message as ProtocolMessage;
use wasm_bindgen::convert::IntoWasmAbi;
use web_sys::js_sys::parse_int;
use std::panic;
use std::{collections::HashMap, f64::consts::PI, hash::Hash};
use web_sys::{
    console,
    js_sys::Uint8Array,
    wasm_bindgen::{closure::Closure, prelude::*, JsCast},
    BinaryType, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MessageEvent,
    WebSocket,
};

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

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

    let window = window();

    let document = document();

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

        if cloned_socket.ready_state() == 1 {
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
        }
    })
    .forget();

    gloo_events::EventListener::new(&window, "keyup", move |event: &Event| {
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();

        let char = event.key().chars().next().unwrap();

        let keys = &mut get_game().keys;

        if keys.contains_key(&char) {
            keys.insert(char, false);
        }

        if socket.ready_state() == 1 {
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
        }
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
    index: Option<i32>,
    entities: Entities,
    ctx: CanvasRenderingContext2d,
    keys: HashMap<char, bool>,
    colors: HashMap<&'static str, &'static str>,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d) -> Self {
        let colors: HashMap<&str, &str> =
            HashMap::from([("grey", "#808080"), ("blue", "#00B0E1"), ("red", "#C83737")]);

        Self {
            index: None,
            entities: Entities::new(),
            ctx,
            keys: HashMap::from([('a', false), ('d', false), ('w', false), ('s', false)]),
            colors,
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
                        }

                        self.entities.retain(|id, _| {
                            vec.iter().any(|m| match m {
                                ProtocolMessage::Array(v) if v.len() == 3 => {
                                    if let [ProtocolMessage::Int32(i), _, _] = v.as_slice() {
                                        i == id
                                    } else {
                                        false
                                    }
                                }
                                _ => false,
                            })
                        });
                    }
                }
            }
        } else if let ProtocolMessage::Int32(id) = message {
            self.index = Some(id);
        }
    }

    fn tick(&mut self) {
        self.render();

        let closure = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            let game = get_game();
            game.tick();
        }));
        window()
            .request_animation_frame(closure.into_js_value().as_ref().unchecked_ref())
            .unwrap_throw();
    }

    fn render(&self) {
        if let None = self.index {
            return;
        }
        if let None = self.entities.get(&self.index.unwrap_throw()) {
            return;
        }

        let me = self.entities.get(&self.index.unwrap_throw()).unwrap();

        let ctx = &self.ctx;

        let width = window().inner_width().unwrap().as_f64().unwrap();
        let height = window().inner_height().unwrap().as_f64().unwrap();

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&JsValue::from_str("#bfbfbf"));
        ctx.fill_rect(0.0, 0.0, width, height);

        ctx.save();

        draw_grid(
            ctx,
            (width / 2.0) as f32 + me.pos.x,
            (height / 2.0) as f32 + me.pos.y,
            32.0,
        );

        ctx.translate(
            width / 2.0 - me.pos.x as f64,
            height / 2.0 - me.pos.y as f64,
        )
        .unwrap_throw();

        self.entities.values().for_each(|entity| {
            ctx.set_global_alpha(1.0);

            ctx.begin_path();
            ctx.arc(
                entity.pos.x.into(),
                entity.pos.y.into(),
                (entity.size / 2.0).into(),
                0.0,
                2.0 * PI,
            )
            .unwrap();

            let color: &str;

            if entity.id == self.index.unwrap() {
                color = self.colors.get("blue").unwrap();
            } else {
                color = self.colors.get("red").unwrap();
            }
            ctx.set_fill_style(&JsValue::from_str(color));
            ctx.fill();

            ctx.set_line_width(5.0);

            ctx.set_stroke_style(&JsValue::from_str(&offset_hex(color, 30)));
            ctx.stroke();
        });

        ctx.restore();
    }
}

fn draw_grid(ctx: &CanvasRenderingContext2d, x: f32, y: f32, cell_size: f32) {
    ctx.begin_path();
    let width = window().inner_width().unwrap().as_f64().unwrap();
    let height = window().inner_height().unwrap().as_f64().unwrap();

    for i in (((width / 2.0 - x as f64) % cell_size as f64) as i32..width as i32).step_by(cell_size as usize) {
        ctx.move_to(i.into(), 0.0);
        ctx.line_to(i.into(), height);
    }

    for j in (((height / 2.0 - y as f64) % cell_size as f64) as i32..height as i32).step_by(cell_size as usize) {
        ctx.move_to(0.0, j.into());
        ctx.line_to(width, j.into());
    }

    ctx.close_path();

    ctx.set_line_width(2.5);
    ctx.set_stroke_style(&JsValue::from_str("#8f8f8f"));

    ctx.stroke();
}

fn offset_hex(hex_color: &str, offset: u8) -> String {
    let mut r = u8::from_str_radix(&hex_color[1..3], 16).unwrap();
    let mut g = u8::from_str_radix(&hex_color[3..5], 16).unwrap();
    let mut b = u8::from_str_radix(&hex_color[5..7], 16).unwrap();

    r = r.saturating_sub(offset);
    g = g.saturating_sub(offset);
    b = b.saturating_sub(offset);

    format!("#{:02X}{:02X}{:02X}", r, g, b)
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
