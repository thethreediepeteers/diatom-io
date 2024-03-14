use crate::{get_game, ProtocolMessage};
use gloo_events::EventListener;
use gloo_utils::{document, window};
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, HtmlCanvasElement, KeyboardEvent, WebSocket};

pub fn add_event_listeners(socket: WebSocket) {
    let window = window();
    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let cloned_window = window.clone();
    EventListener::new(&window, "resize", move |_| {
        canvas.set_width(cloned_window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(cloned_window.inner_height().unwrap().as_f64().unwrap() as u32);
    })
    .forget();

    let cloned_socket = socket.clone();

    EventListener::new(&window, "keydown", move |event: &Event| {
        if cloned_socket.ready_state() != 1 {
            return;
        }
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();

        let char = event.key().chars().next().unwrap().to_ascii_lowercase();

        let keys = &mut get_game().keys;

        if keys.contains_key(&char) {
            keys.insert(char, true);
        }

        cloned_socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Bool(*keys.get(&'w').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'a').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'s').unwrap()),
                    ProtocolMessage::Bool(*keys.get(&'d').unwrap()),
                ])
                .encode(),
            )
            .unwrap_throw();
    })
    .forget();

    EventListener::new(&window, "keyup", move |event: &Event| {
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();

        let char = event.key().chars().next().unwrap().to_ascii_lowercase();

        let keys = &mut get_game().keys;

        if keys.contains_key(&char) {
            keys.insert(char, false);
        }

        if socket.ready_state() == 1 {
            socket
                .send_with_u8_array(
                    &ProtocolMessage::Array(vec![
                        ProtocolMessage::Bool(*keys.get(&'w').unwrap()),
                        ProtocolMessage::Bool(*keys.get(&'a').unwrap()),
                        ProtocolMessage::Bool(*keys.get(&'s').unwrap()),
                        ProtocolMessage::Bool(*keys.get(&'d').unwrap()),
                    ])
                    .encode(),
                )
                .unwrap_throw();
        }
    })
    .forget();
}
