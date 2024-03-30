mod draw;
mod entity;
mod game;
mod listeners;
mod protocol;
mod util;

extern crate console_error_panic_hook;

use game::{get_game, new_game, GAME};
use gloo_utils::{document, window};
use gloo_console::console_dbg;
use listeners::add_event_listeners;
use protocol::Message as ProtocolMessage;
use std::panic;
use web_sys::{
    js_sys::Uint8Array,
    wasm_bindgen::{self, closure::Closure, prelude::*, JsCast},
    BinaryType, CanvasRenderingContext2d, CloseEvent, HtmlCanvasElement, MessageEvent, WebSocket,
};

#[wasm_bindgen(start)]
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let addr = "ws://localhost:3000/";

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

    add_event_listeners(socket);

    new_game(ctx);

    get_game().tick();
}
