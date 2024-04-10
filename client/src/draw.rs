use crate::{entity::Entity, get_game, util::offset_hex};
use gloo_console::console_dbg;
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

    ctx.set_line_width(1.0);

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

    let mockup = game
        .mockups
        .as_array()
        .unwrap()
        .get(entity.mockup_id as usize)
        .unwrap();

    ctx.set_line_width(5.);
    ctx.set_global_alpha(1.0);

    for gun in mockup["guns"].as_array().unwrap() {
        let width = gun["width"].as_f64().unwrap();
        let height = gun["height"].as_f64().unwrap();

        let x = entity.pos.x + gun["x"].as_f64().unwrap();
        let y = entity.pos.y + gun["y"].as_f64().unwrap();
        let angle = gun["angle"].as_f64().unwrap() * PI / 180.0;

        let x1 = x + angle.cos() * height;
        let y1 = y + angle.sin() * height;

        let color = gun["color"].as_str().unwrap();

        draw_trapezoid(
            ctx,
            x1,
            y1,
            width / 2.0,
            height / 2.0,
            angle + entity.angle,
            gun["aspect"].as_f64().unwrap(),
            color,
        );
    }

    let color = mockup["color"].as_str().unwrap();

    draw_shape(
        &ctx,
        mockup["shape"].as_u64().unwrap() as u8,
        entity.pos.x,
        entity.pos.y,
        mockup["width"].as_f64().unwrap(),
        mockup["height"].as_f64().unwrap(),
        entity.angle,
        JsValue::from_str(color),
        JsValue::from_str(&offset_hex(color, 30)),
        5.0,
    );
}

fn draw_shape(
    ctx: &CanvasRenderingContext2d,
    shape: u8,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    color: JsValue,
    stroke_color: JsValue,
    stroke_width: f64,
) {
    let height = height / 2.0;
    let width = width / 2.0;

    ctx.begin_path();
    if shape == 0 {
        ctx.ellipse(x, y, width, height, angle, 0.0, 2.0 * PI)
            .unwrap();
    } else {
        let r = (height / width).atan();
        let l = (width * width + height * height).sqrt();

        ctx.begin_path();
        ctx.move_to(x + l * (angle + r).cos(), y + l * (angle + r).sin());
        ctx.line_to(
            x + l * (angle + PI - r).cos(),
            y + l * (angle + PI - r).sin(),
        );
        ctx.line_to(
            x + l * (angle + PI + r).cos(),
            y + l * (angle + PI + r).sin(),
        );
        ctx.line_to(x + l * (angle - r).cos(), y + l * (angle - r).sin());
        ctx.close_path();
    }
    ctx.close_path();
    ctx.set_fill_style(&color);
    ctx.fill();

    ctx.set_line_width(stroke_width);
    ctx.set_stroke_style(&stroke_color);
    ctx.stroke();
}

fn draw_trapezoid(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    aspect: f64,
    color: &str,
) {
    let h = if aspect > 0.0 {
        [width * aspect, width]
    } else {
        [width, -width * aspect]
    };

    let r = [h[0].atan2(height), h[1].atan2(height)];
    let l = [
        (height * height + h[1] * h[1]).sqrt(),
        (height * height + h[1] * h[1]).sqrt(),
    ];

    ctx.begin_path();
    ctx.line_to(
        x + l[0] * (angle + r[0]).cos(),
        y + l[0] * (angle + r[0]).sin(),
    );
    ctx.line_to(
        x + l[1] * (angle + PI - r[1]).cos(),
        y + l[1] * (angle + PI - r[1]).sin(),
    );
    ctx.line_to(
        x + l[1] * (angle + PI + r[1]).cos(),
        y + l[1] * (angle + PI + r[1]).sin(),
    );
    ctx.line_to(
        x + l[0] * (angle - r[0]).cos(),
        y + l[0] * (angle - r[0]).sin(),
    );
    ctx.close_path();

    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill();

    ctx.set_line_width(5.0);
    ctx.set_line_cap("round");
    ctx.set_stroke_style(&JsValue::from_str(&offset_hex(color, 30)));
    ctx.stroke();
}
