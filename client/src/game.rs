use crate::{
    draw::{draw_connecting, draw_disconnect, draw_entity, draw_grid},
    entity::Entity,
    listeners::add_event_listeners,
    util::lerp,
    ProtocolMessage,
};
use gloo_utils::window;
use std::collections::HashMap;
use web_sys::{
    js_sys::Uint8Array,
    wasm_bindgen::{closure::Closure, prelude::*},
    BinaryType, CanvasRenderingContext2d, CloseEvent, MessageEvent, WebSocket,
};

type Entities = HashMap<i32, Entity>;

struct Map {
    width: f32,
    height: f32,
    server_width: f32,
    server_height: f32,
}

pub struct Game {
    pub index: Option<i32>,
    entities: Entities,
    ctx: CanvasRenderingContext2d,
    pub colors: HashMap<&'static str, &'static str>,
    pub disconnected: Option<String>,
    map: Map,
}

impl Game {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        let colors: HashMap<&str, &str> = HashMap::from([
            ("grey", "#808080"),
            ("blue", "#00B0E1"),
            ("red", "#C83737"),
            ("bg", "#d4d4d4"),
            ("grid", "#0000001a"),
        ]);

        Self {
            index: None,
            entities: Entities::new(),
            ctx,
            colors,
            disconnected: None,
            map: Map {
                width: 0.0,
                height: 0.0,
                server_width: 0.0,
                server_height: 0.0,
            },
        }
    }

    pub fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            for msg in &vec {
                if let ProtocolMessage::Array(v) = msg {
                    if let [ProtocolMessage::Float32(map_width), ProtocolMessage::Float32(map_height)] =
                        v.as_slice()
                    {
                        self.map.server_width = *map_width;
                        self.map.server_height = *map_height;
                    }
                    if let [ProtocolMessage::Int32(id), ProtocolMessage::Array(pos), ProtocolMessage::Float32(size)] =
                        v.as_slice()
                    {
                        if let [ProtocolMessage::Float32(x), ProtocolMessage::Float32(y)] =
                            pos.as_slice()
                        {
                            if self.entities.get(id).is_none() {
                                self.entities.insert(*id, Entity::new(*id, *x, *y, 0.0));
                            } else {
                                self.entities.entry(*id).and_modify(|e| {
                                    e.set_predict(*x, *y, *size);
                                });
                            }
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

    pub fn start(&mut self, addr: &str) {
        let socket = WebSocket::new(addr).unwrap();

        socket.set_binary_type(BinaryType::Arraybuffer);

        socket.set_onmessage(Some(
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

        socket.set_onclose(Some(
            Closure::<dyn FnMut(_)>::new(move |event: CloseEvent| {
                unsafe {
                    if GAME.is_none() {
                        return;
                    }
                }
                get_game().disconnected = Some(event.reason());
            })
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
        ));

        add_event_listeners(socket);
        self.tick();
    }

    pub fn tick(&mut self) {
        self.update();

        let closure = Closure::once_into_js(|| {
            let game = get_game();
            game.tick();
        });

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap_throw();
    }

    pub fn update(&mut self) {
        let ctx = &self.ctx;

        let width = window().inner_width().unwrap().as_f64().unwrap();
        let height = window().inner_height().unwrap().as_f64().unwrap();

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&JsValue::from_str("#c9c9c9"));
        ctx.fill_rect(0.0, 0.0, width, height);

        if self.index.is_none() || self.entities.get(&self.index.unwrap_throw()).is_none() {
            draw_connecting(ctx);

            return;
        }

        if let Some(reason) = &self.disconnected {
            draw_disconnect(reason, ctx);

            return;
        }

        let me = self.entities.get(&self.index.unwrap()).unwrap();

        ctx.save();

        self.map.width = lerp(self.map.width, self.map.server_width);
        self.map.height = lerp(self.map.height, self.map.server_height);

        ctx.set_fill_style(&JsValue::from_str(self.colors.get("bg").unwrap()));
        ctx.fill_rect(
            width / 2.0 - me.pos.x as f64,
            height / 2.0 - me.pos.y as f64,
            self.map.width.into(),
            self.map.height.into(),
        );

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

        for entity in self.entities.values_mut() {
            draw_entity(ctx, entity);

            entity.predict();
        }

        ctx.restore();
    }
}

pub static mut GAME: Option<Box<Game>> = None;

pub fn new_game(ctx: CanvasRenderingContext2d) {
    unsafe { GAME = Some(Box::new(Game::new(ctx))) }
}

pub fn get_game() -> &'static mut Game {
    unsafe { GAME.as_mut().unwrap() }
}
