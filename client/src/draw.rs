use crate::{entity::Entity, get_game, util::offset_hex};
use gloo_utils::window;
use std::f64::consts::PI;
use web_sys::{
    wasm_bindgen::{prelude::UnwrapThrowExt, JsValue},
    CanvasRenderingContext2d,
};

pub fn draw_grid(ctx: &CanvasRenderingContext2d, x: f64, y: f64, cell_size: f64) {
    ctx.begin_path();
    let width = window().inner_width().unwrap().as_f64().unwrap();
    let height = window().inner_height().unwrap().as_f64().unwrap();

    for i in (((width / 2.0 - x) % cell_size) as i32..width as i32).step_by(cell_size as usize) {
        ctx.move_to(i.into(), 0.0);
        ctx.line_to(i.into(), height);
    }

    for j in (((height / 2.0 - y) % cell_size) as i32..height as i32).step_by(cell_size as usize) {
        ctx.move_to(0.0, j.into());
        ctx.line_to(width, j.into());
    }

    ctx.close_path();

    ctx.set_line_width(2.5);

    ctx.set_stroke_style(&JsValue::from_str(get_game().colors.get("grid").unwrap()));

    ctx.stroke();
}

pub fn draw_connecting(ctx: &CanvasRenderingContext2d) {
    let window = window();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
    ctx.set_font("bold 48px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    ctx.fill_text("Connecting...", width / 2.0, height / 2.0)
        .unwrap_throw();
}

pub fn draw_disconnect(reason: &String, ctx: &CanvasRenderingContext2d) {
    let window = window();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    ctx.set_fill_style(&JsValue::from_str("#FF0000"));
    ctx.set_font("bold 48px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    if !reason.is_empty() {
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
}

pub fn draw_entity(ctx: &CanvasRenderingContext2d, entity: &mut Entity) {
    let game = get_game();

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

    let color: &str = if entity.id == game.index.unwrap() {
        game.colors.get("blue").unwrap()
    } else {
        game.colors.get("red").unwrap()
    };

    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill();

    ctx.set_line_width(5.0);

    ctx.set_stroke_style(&JsValue::from_str(&offset_hex(color, 30)));
    ctx.stroke();
}
