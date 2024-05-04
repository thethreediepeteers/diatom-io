use web_sys::{wasm_bindgen::{JsValue, UnwrapThrowExt}, CanvasRenderingContext2d};

pub struct Context {
    ctx: CanvasRenderingContext2d
}

impl Context {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }

    pub fn scale<T: Into<f64>>(&self, x: T) {
        let s = x.into();
        self.ctx.scale(s, s).unwrap_throw();
    }

    pub fn fill_style(&self, color: &str) {
        self.ctx.set_fill_style(&JsValue::from_str(color));
    }

    pub fn stroke_style(&self, color: &str) {
        self.ctx.set_stroke_style(&JsValue::from_str(color));
    }

    pub fn line_width<T: Into<f64>>(&self, width: T) {
        self.ctx.set_line_width(width.into());
    }

    pub fn translate<T: Into<f64>>(&self, x: T, y: T) {
        self.ctx.translate(x.into(), y.into()).unwrap_throw();
    }

    pub fn fill_rect<T: Into<f64>>(&self, x: T, y: T, w: T, h: T) {
        self.ctx.fill_rect(x.into(), y.into(), w.into(), h.into());
    }

    pub fn clear_rect<T: Into<f64>>(&self, x: T, y: T, w: T, h: T) {
        self.ctx.clear_rect(x.into(), y.into(), w.into(), h.into());
    }

    pub fn fill_text<T: Into<f64>>(&self, text: &str, x: T, y: T) {
        let x = x.into();
        let y = y.into();
        let width = self.ctx.line_width();
        self.line_width(8.0);
        self.stroke_style("#000");
        self.ctx.stroke_text(text, x, y).unwrap_throw();
        self.line_width(width);
        self.ctx.fill_text(text, x, y).unwrap_throw();
    }

    pub fn font(&self, font: &str) {
        self.ctx.set_font(font);
    }

    pub fn text_align(&self, align: &str) {
        self.ctx.set_text_align(align);
    }

    pub fn text_baseline(&self, baseline: &str) {
        self.ctx.set_text_baseline(baseline);
    }

    pub fn begin_path(&self) {
        self.ctx.begin_path();
    }

    pub fn close_path(&self) {
        self.ctx.close_path();
    }

    pub fn fill(&self) {
        self.ctx.fill();
    }

    pub fn stroke(&self) {
        self.ctx.stroke();
    }

    pub fn line_to<T: Into<f64>>(&self, x: T, y: T) {
        self.ctx.line_to(x.into(), y.into());
    }

    pub fn move_to<T: Into<f64>>(&self, x: T, y: T) {
        self.ctx.move_to(x.into(), y.into());
    }

    pub fn canvas_width<T: From<u32>>(&self) -> T {
        self.ctx.canvas().unwrap_throw().width().into()
    }

    pub fn canvas_height<T: From<u32>>(&self) -> T {
        self.ctx.canvas().unwrap_throw().height().into()
    }

    pub fn save(&self) {
        self.ctx.save();
    }

    pub fn restore(&self) {
        self.ctx.restore();
    }

    pub fn global_alpha(&self, alpha: f64) {
        self.ctx.set_global_alpha(alpha);
    }

    pub fn ellipse<T: Into<f64>, A: Into<f64>>(&self, x: T, y: T, width: T, height: T, angle: A) {
        self.ctx.ellipse(x.into(), y.into(), width.into(), height.into(), angle.into(), 0.0, 2.0 * std::f64::consts::PI).unwrap_throw();
    }

    pub fn line_cap(&self, cap: &str) {
        self.ctx.set_line_cap(cap);
    }

    pub fn line_join(&self, join: &str) {
        self.ctx.set_line_join(join);
    }
}