use crate::{context::Context, entity::Entity, get_game, util::offset_hex};
use gloo_utils::window;
use std::f64::consts::PI;

pub fn draw_grid(ctx: &Context, x: f64, y: f64, cell_size: f64) {
    let width: f64 = ctx.canvas_width();
    let height: f64 = ctx.canvas_height();

    let mut x = x % cell_size;
    while x < width {
        ctx.move_to(x, 0.0);
        ctx.line_to(x, height);
        x += cell_size;
    }

    let mut y = y % cell_size;
    while y < height {
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
        y += cell_size;
    }

    ctx.close_path();

    ctx.line_width(2);
    ctx.stroke_style(get_game().colors.get("grid").unwrap());
    ctx.stroke();
}

pub fn draw_connecting(ctx: &Context) {
    let width: f64 = ctx.canvas_width();
    let height: f64 = ctx.canvas_height();

    ctx.fill_style("#ffffff");
    ctx.font("bold 48px sans-serif");
    ctx.text_align("center");
    ctx.text_baseline("middle");
    ctx.fill_text("Connecting...", width / 2.0, height / 2.0);
}

pub fn draw_disconnect(reason: &String, ctx: &Context) {
    let window = window();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    ctx.fill_style("#ff0000");
    ctx.font("bold 48px sans-serif");
    ctx.text_align("center");
    ctx.text_baseline("middle");

    if !reason.is_empty() {
        ctx.fill_text(
            &format!("Disconnected: {}", reason),
            width / 2.0,
            height / 2.0,
        );
    } else {
        ctx.fill_text("Disconnected", width / 2.0, height / 2.0);
    }
}

pub fn draw_entity(ctx: &Context, entity: &mut Entity) {
    let game = get_game();

    let mockup = game.mockups.get(entity.mockup_id);

    ctx.line_width(5);
    ctx.global_alpha(1.0);

    for gun in mockup.guns.iter() {
        let x = entity.pos.x + gun.x;
        let y = entity.pos.y + gun.y;
        let angle = gun.angle * PI / 180.0 + entity.angle + PI / 2.0;

        draw_trapezoid(
            ctx,
            x + angle.cos() * gun.width,
            y + angle.sin() * gun.height,
            gun.width / 2.0,
            gun.height / 2.0,
            angle,
            gun.aspect,
            &gun.color
        );
    }

    draw_shape(
        &ctx,
        mockup.shape,
        entity.pos.x,
        entity.pos.y,
        mockup.width,
        mockup.height,
        entity.angle,
        &mockup.color,
        &offset_hex(&mockup.color, 30),
        5.0,
    );
}

fn draw_shape(
    ctx: &Context,
    shape: u8,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    color: &str,
    stroke_color: &str,
    stroke_width: f64
) {
    let height = height / 2.0;
    let width = width / 2.0;

    ctx.begin_path();
    if shape == 0 {
        ctx.ellipse(x, y, width, height, angle);
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
    ctx.fill_style(&color);
    ctx.fill();

    ctx.line_width(stroke_width);
    ctx.stroke_style(&stroke_color);
    ctx.stroke();
}

fn draw_trapezoid(
    ctx: &Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    angle: f64,
    aspect: f64,
    color: &str
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

    ctx.fill_style(color);
    ctx.fill();

    ctx.line_width(5);
    ctx.stroke_style(&offset_hex(color, 30));
    ctx.stroke();
}