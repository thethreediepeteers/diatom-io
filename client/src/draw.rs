use web_sys::{CanvasRenderingContext2d, wasm_bindgen::JsValue};
use gloo_utils::window;
use crate::get_game;


pub fn draw_grid(ctx: &CanvasRenderingContext2d, x: f32, y: f32, cell_size: f32) {
    ctx.begin_path();
    let width = window().inner_width().unwrap().as_f64().unwrap();
    let height = window().inner_height().unwrap().as_f64().unwrap();

    for i in (((width / 2.0 - x as f64) % cell_size as f64) as i32..width as i32)
        .step_by(cell_size as usize)
    {
        ctx.move_to(i.into(), 0.0);
        ctx.line_to(i.into(), height);
    }

    for j in (((height / 2.0 - y as f64) % cell_size as f64) as i32..height as i32)
        .step_by(cell_size as usize)
    {
        ctx.move_to(0.0, j.into());
        ctx.line_to(width, j.into());
    }

    ctx.close_path();

    ctx.set_line_width(2.5);
    ctx.set_stroke_style(&JsValue::from_str(get_game().colors.get("grid").unwrap()));

    ctx.stroke();
}