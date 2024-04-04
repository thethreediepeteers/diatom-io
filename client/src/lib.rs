mod draw;
mod entity;
mod game;
mod listeners;
mod protocol;
mod util;

extern crate console_error_panic_hook;

use game::{get_game, new_game};
use gloo_utils::{document, window};
use protocol::Message as ProtocolMessage;
use std::panic;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    wasm_bindgen::{self, closure::Closure, prelude::*, JsCast},
    CanvasRenderingContext2d, Event, HtmlButtonElement, HtmlCanvasElement, HtmlDivElement,
};

#[wasm_bindgen(start)]
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let addr = "ws://0.0.0.0:3000/ws";

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

    let start_button = document
        .get_element_by_id("start")
        .unwrap_throw()
        .dyn_into::<HtmlButtonElement>()
        .unwrap_throw();

    new_game(ctx);
    
    start_button.set_onclick(Some(
        Closure::<dyn FnMut(_)>::new(move |_: Event| {
            document
                .get_element_by_id("startmenu")
                .unwrap()
                .dyn_into::<HtmlDivElement>()
                .unwrap()
                .style()
                .set_property("display", "none")
                .unwrap();
            canvas.style().set_property("display", "flex").unwrap();
            spawn_local(async {
                get_game().start(addr).await;
            });
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));
}
