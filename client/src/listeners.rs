use crate::{game::get_game, ProtocolMessage};
use gloo_events::{EventListener, EventListenerOptions};
use gloo_utils::{document, window};
use web_sys::{
    wasm_bindgen::{prelude::*, JsCast},
    Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebSocket,
};

pub fn add_event_listeners(socket: WebSocket) {
    let window = window();
    let canvas = document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let resize = move || {
        let game = get_game();
        let window = web_sys::window().unwrap();
        let ratio = window.device_pixel_ratio();
        let width = window.inner_width().unwrap().as_f64().unwrap() * ratio;
        let height = window.inner_height().unwrap().as_f64().unwrap() * ratio;
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let a = width / 1920.0;
        let b = height / 1080.0;
        game.window_scale = if a > b { a } else { b };
    };
    resize();
    EventListener::new(&window, "resize", move |_| resize()).forget();

    let prevented = EventListenerOptions::enable_prevent_default();
    EventListener::new_with_options(
        &web_sys::window().unwrap(),
        "contextmenu",
        prevented,
        move |event: &Event| event.prevent_default()
    ).forget();

    let cloned_socket = socket.clone();
    let cloned_socket_1 = socket.clone();
    let cloned_socket_2 = socket.clone();
    let cloned_socket_3 = socket.clone();

    EventListener::new(&window, "keydown", move |event: &Event| {
        if cloned_socket.ready_state() != 1 {
            return;
        }
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();
        let key = event.code();

        let num: u8;

        match key.as_str() {
            "KeyW" | "ArrowUp" => num = 0,
            "KeyA" | "ArrowLeft" => num = 1,
            "KeyS" | "ArrowDown" => num = 2,
            "KeyD" | "ArrowRight" => num = 3,
            _ => return
        };

        cloned_socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Uint8(0),
                    ProtocolMessage::Uint8(num),
                ])
                .encode(),
            )
            .unwrap_throw();
    }).forget();

    EventListener::new(&window, "keyup", move |event: &Event| {
        if socket.ready_state() != 1 {
            return;
        }
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap_throw();
        let key = event.code();

        let num: u8;

        match key.as_str() {
            "KeyW" | "ArrowUp" => num = 0,
            "KeyA" | "ArrowLeft" => num = 1,
            "KeyS" | "ArrowDown" => num = 2,
            "KeyD" | "ArrowRight" => num = 3,
            _ => return
        };

        socket
            .send_with_u8_array(
                &ProtocolMessage::Array(vec![
                    ProtocolMessage::Uint8(1),
                    ProtocolMessage::Uint8(num),
                ])
                .encode(),
            )
            .unwrap_throw();
    }).forget();

    EventListener::new(&window, "mousedown", move |_: &Event| {
        if cloned_socket_2.ready_state() != 1 {
            return;
        }
        cloned_socket_2
            .send_with_u8_array(&ProtocolMessage::Array(vec![ProtocolMessage::Bool(true)]).encode())
            .unwrap_throw();
    }).forget();
    EventListener::new(&window, "mouseup", move |_: &Event| {
        if cloned_socket_3.ready_state() != 1 {
            return;
        }
        cloned_socket_3
            .send_with_u8_array(&ProtocolMessage::Array(vec![ProtocolMessage::Bool(false)]).encode())
            .unwrap_throw();
    }).forget();

    EventListener::new(&window, "mousemove", move |event: &Event| {
        let game = get_game();
        let width: f64 = game.ctx.canvas_width();
        let height: f64 = game.ctx.canvas_height();
        let event = event.clone().dyn_into::<MouseEvent>().unwrap();
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;

        if cloned_socket_1.ready_state() != 1 {
            return;
        }

        let window = web_sys::window().unwrap();
        let ratio = window.device_pixel_ratio();

        let delta_x = x - width / 2.0 / ratio;
        let delta_y = y - height / 2.0 / ratio;
        let rad = -delta_x.atan2(delta_y);
        game.mouse_angle = rad;
        cloned_socket_1
            .send_with_u8_array(&ProtocolMessage::Array(vec![ProtocolMessage::Float64(rad)]).encode())
            .unwrap();
    }).forget();
}