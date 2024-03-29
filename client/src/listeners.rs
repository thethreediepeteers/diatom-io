use crate::ProtocolMessage;
use gloo_events::{EventListener, EventListenerOptions};
use gloo_utils::{document, window};
use web_sys::{
    wasm_bindgen::{prelude::*, JsCast}, Event, HtmlCanvasElement, KeyboardEvent, WebSocket
};

pub fn add_event_listeners(socket: WebSocket) {
    let window = window();
    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let cloned_canvas = canvas.clone();

    let cloned_window = window.clone();
    EventListener::new(&window, "resize", move |_| {
        canvas.set_width(cloned_window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(cloned_window.inner_height().unwrap().as_f64().unwrap() as u32);
    })
    .forget();

    let prevented = EventListenerOptions::enable_prevent_default();
    EventListener::new_with_options(
        &cloned_canvas,
        "contextmenu",
        prevented,
        move |event: &Event| {
            event.prevent_default();
        },
    )
    .forget();

    let cloned_socket = socket.clone();

    EventListener::new(&window, "keydown", move |event: &Event| {
        if cloned_socket.ready_state() != 1 {
            return;
        }
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();
        let key = event.key();

        let num: u8;

        match key.as_str() {
            "w" | "ArrowUp" => num = 0,
            "a" | "ArrowLeft" => num = 1,
            "s" | "ArrowDown" => num = 2,
            "d" | "ArrowRight" => num = 3,
            _ => return,
        }

        cloned_socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Uint8(0),
                    ProtocolMessage::Uint8(num),
                ])
                .encode(),
            )
            .unwrap_throw();
    })
    .forget();

    EventListener::new(&window, "keyup", move |event: &Event| {
        if socket.ready_state() != 1 {
            return;
        }
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();
        let key = event.key();

        let num: u8;

        match key.as_str() {
            "w" | "ArrowUp" => num = 0,
            "a" | "ArrowLeft" => num = 1,
            "s" | "ArrowDown" => num = 2,
            "d" | "ArrowRight" => num = 3,
            _ => return,
        }

        socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Uint8(1),
                    ProtocolMessage::Uint8(num),
                ])
                .encode(),
            )
            .unwrap_throw();
    })
    .forget();
}
