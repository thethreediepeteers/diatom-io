use crate::{
    context::Context,
    draw::{draw_connecting, draw_disconnect, draw_entity, draw_grid},
    entity::Entity,
    listeners::add_event_listeners,
    mockup::Mockups,
    util::lerp,
    ProtocolMessage
};
use gloo_console::console_dbg;
use gloo_utils::window;
use std::collections::HashMap;
use web_sys::{
    js_sys::Uint8Array,
    wasm_bindgen::{closure::Closure, prelude::*},
    BinaryType, CanvasRenderingContext2d, CloseEvent, MessageEvent, WebSocket,
};

type Entities = HashMap<u16, Entity>;

struct Map {
    width: f64,
    height: f64,
    server_width: f64,
    server_height: f64
}

impl Map {
    pub fn new_empty() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            server_width: 0.0,
            server_height: 0.0
        }
    }
}

pub struct Game {
    pub index: Option<u16>,
    entities: Entities,
    pub ctx: Context,
    pub colors: HashMap<&'static str, &'static str>,
    pub disconnected: Option<String>,
    map: Map,
    pub mockups: Mockups,
    pub window_scale: f64,
    pub mouse_angle: f64
}

fn decode_angle(angle: i16) -> f64 {
    angle as f64 / i16::MAX as f64 * 360.
}

impl Game {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        let colors: HashMap<&str, &str> = HashMap::from([
            ("grey", "#808080"),
            ("blue", "#00B0E1"),
            ("red", "#C83737"),
            ("bg", "#d4d4d4"),
            ("grid", "#0000000a")
        ]);

        Self {
            index: None,
            entities: Entities::new(),
            ctx: Context::new(ctx),
            colors,
            disconnected: None,
            map: Map::new_empty(),
            mockups: Mockups::new(),
            window_scale: 1.0,
            mouse_angle: 0.0
        }
    }

    pub fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            for msg in &vec {
                if let ProtocolMessage::Array(v) = msg {
                    match v.as_slice() {
                        [ProtocolMessage::Float64(w), ProtocolMessage::Float64(h)] =>
                        // map size
                        {
                            self.map.server_width = *w;
                            self.map.server_height = *h;
                        }
                        [ProtocolMessage::Uint16(id), ProtocolMessage::Uint16(mockup_id), ProtocolMessage::Int16(angle), ProtocolMessage::Array(bounds)] =>
                        // entity update
                        {
                            if let [ProtocolMessage::Float64(min_x), ProtocolMessage::Float64(min_y), ProtocolMessage::Float64(max_x), ProtocolMessage::Float64(max_y)] =
                                bounds.as_slice()
                            {
                                let x = (min_x + max_x) / 2.0;
                                let y = (min_y + max_y) / 2.0;

                                self.entities
                                    .entry(*id)
                                    .and_modify(|e| {
                                        e.set_predict(x, y, *max_x - *min_x, decode_angle(*angle));
                                    })
                                    .or_insert(Entity::new(*id, x, y, 0.0, *mockup_id, self.index.unwrap_or(u16::MAX) == *id));
                            }

                            self.entities.retain(|id, _| {
                                vec.iter().any(|m| match m {
                                    ProtocolMessage::Array(v) if v.len() == 4 => {
                                        if let [ProtocolMessage::Uint16(i), _, _, _] = v.as_slice() {
                                            i == id
                                        } else {
                                            false
                                        }
                                    }
                                    _ => false,
                                })
                            });
                        }
                        _ => {
                            console_dbg!(v);
                        }
                    };
                }
            }
        } else if let ProtocolMessage::Uint16(id) = message {
            self.index = Some(id);
            //if let Some((_, entity)) = self.entities.iter_mut().find(|e| e.1.id == id) {
            //    entity.is_player = true;
            //}
        }
    }

    pub async fn start(&mut self, addr: &str) {
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

        if let Err(e) = self.get_mockups().await {
            console_dbg!(format!("Failed to get mockups: {:?}", e));
        };

        console_dbg!(format!(
            "mockups: {:?}",
            self.mockups.as_vec()
        ));
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

        let width = ctx.canvas_width();
        let height = ctx.canvas_height();

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.fill_style("#c9c9c9");
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
        ctx.line_cap("round");
        ctx.line_join("round");

        self.map.width = lerp(self.map.width, self.map.server_width, 0.1);
        self.map.height = lerp(self.map.height, self.map.server_height, 0.1);

        ctx.fill_style(self.colors.get("bg").unwrap());
        ctx.fill_rect(
            width / 2.0 - me.pos.x * self.window_scale,
            height / 2.0 - me.pos.y * self.window_scale,
            self.map.width * self.window_scale,
            self.map.height * self.window_scale
        );

        draw_grid(
            ctx,
            width / 2.0 - me.pos.x * self.window_scale,
            height / 2.0 - me.pos.y * self.window_scale,
            32.0 * self.window_scale
        );

        ctx.translate(width / 2.0, height / 2.0);
        ctx.scale(self.window_scale);
        ctx.translate(-me.pos.x, -me.pos.y);

        for entity in self.entities.values_mut() {
            draw_entity(ctx, entity);
            entity.predict();
        }

        ctx.restore();
    }

    async fn get_mockups(&mut self) -> Result<(), reqwest::Error> {
        let addr = format!("http://localhost:3000/mockups.json");

        self.mockups.load(serde_json::from_str(reqwest::get(addr).await?.text().await?.trim()).unwrap());
        Ok(())
    }
}

pub static mut GAME: Option<Box<Game>> = None;

pub fn new_game(ctx: CanvasRenderingContext2d) {
    unsafe { GAME = Some(Box::new(Game::new(ctx))) }
}

pub fn get_game() -> &'static mut Game {
    unsafe { GAME.as_mut().unwrap() }
}