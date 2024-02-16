use protocol::Message as ProtocolMessage;
use web_sys::{
    console,
    js_sys::{Math::random, Uint8Array},
    wasm_bindgen::{closure::Closure, JsCast},
    window, BinaryType, CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, WebSocket,
};

mod protocol;

fn main() {
    let addr = "ws://localhost:3000";

    let socket = WebSocket::new(addr).unwrap();

    socket.set_binary_type(BinaryType::Arraybuffer);

    socket.clone().set_onmessage(Some(
        Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            let buf = event.data();
            let array = Uint8Array::new(&buf);
            let message = ProtocolMessage::decode(&array.to_vec());

            handle_message(message);
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));

    socket.clone().set_onopen(Some(
        Closure::<dyn FnMut()>::new(move || {
            // onopen
        })
        .into_js_value()
        .as_ref()
        .unchecked_ref(),
    ));

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    game_loop();
}

fn game_loop() {
    let window = window().unwrap();
    let document = window.document().unwrap();
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

    ctx.set_fill_style(
        &format!(
            "rgb({}, {}, {})",
            random() * 255.0,
            random() * 255.0,
            random() * 255.0
        )
        .into(),
    );
    ctx.fill_rect(
        random() * canvas.width() as f64,
        random() * canvas.height() as f64,
        100.0,
        100.0,
    );

    let closure = Closure::wrap(Box::new(move |ctx: CanvasRenderingContext2d| {
        game_loop();
    }) as Box<dyn FnMut(_)>);

    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn handle_message(msg: ProtocolMessage) {
    if let ProtocolMessage::Object(pos) = msg {
        console::log_1(&format!("pos: {:?}", pos).into());
    }
}
