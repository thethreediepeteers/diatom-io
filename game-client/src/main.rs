use protocol::Message as ProtocolMessage;
use web_sys::{
    console, js_sys::Uint8Array, wasm_bindgen::{closure::Closure, JsCast, JsValue}, BinaryType, MessageEvent, WebSocket
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

            console::log_1(&format!("Message: {:?}", message).as_str().into());
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
}
