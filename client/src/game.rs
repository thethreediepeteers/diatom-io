use crate::{draw::draw_grid, util::offset_hex, entity::Entity, ProtocolMessage};
use gloo_utils::window;
use std::{collections::HashMap, f64::consts::PI};
use web_sys::{
    wasm_bindgen::{closure::Closure, prelude::*},
    CanvasRenderingContext2d,
};

type Entities = HashMap<i32, Entity>;

pub struct Game {
    index: Option<i32>,
    entities: Entities,
    ctx: CanvasRenderingContext2d,
    pub keys: HashMap<char, bool>,
    pub colors: HashMap<&'static str, &'static str>,
    pub disconnected: Option<String>,
}

impl Game {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        let colors: HashMap<&str, &str> = HashMap::from([
            ("grey", "#808080"),
            ("blue", "#00B0E1"),
            ("red", "#C83737"),
            ("bg", "#BFBFBF"),
            ("grid", "#8F8F8F"),
        ]);

        Self {
            index: None,
            entities: Entities::new(),
            ctx,
            keys: HashMap::from([('a', false), ('d', false), ('w', false), ('s', false)]),
            colors,
            disconnected: None,
        }
    }

    pub fn handle_message(&mut self, message: ProtocolMessage) {
        if let ProtocolMessage::Array(vec) = message {
            for msg in &vec {
                if let ProtocolMessage::Array(v) = msg {
                    if let [ProtocolMessage::Int32(id), ProtocolMessage::Array(pos), ProtocolMessage::Float32(size)] =
                        v.as_slice()
                    {
                        if let [ProtocolMessage::Float32(x), ProtocolMessage::Float32(y)] =
                            pos.as_slice()
                        {
                            if let None = self.entities.get(id) {
                                self.entities.insert(*id, Entity::new(*id, *x, *y, *size));
                            } else {
                                self.entities.entry(*id).and_modify(|e| {
                                    e.set_predict(*x, *y);
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
        ctx.set_fill_style(&JsValue::from_str(self.colors.get("bg").unwrap()));
        ctx.fill_rect(0.0, 0.0, width, height);

        if self.index.is_none() || self.entities.get(&self.index.unwrap_throw()).is_none() {
            ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
            ctx.set_font("bold 48px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.fill_text("Connecting...", width / 2.0, height / 2.0)
                .unwrap_throw();
            return;
        }

        if let Some(reason) = &self.disconnected {
            ctx.set_fill_style(&JsValue::from_str("#FF0000"));
            ctx.set_font("bold 48px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");

            if reason != "" {
                ctx.fill_text(
                    &format!("Disconnected: {}", reason),
                    width / 2.0,
                    height / 2.0,
                )
                .unwrap_throw();
            } else {
                ctx.fill_text("Disconnected", width / 2.0, height / 2.0)
                    .unwrap_throw();
            }

            return;
        }

        let me = self.entities.get(&self.index.unwrap()).unwrap();

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

        for entity in self.entities.values_mut() {
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
    unsafe { GAME.as_mut().unwrap_throw() }
}