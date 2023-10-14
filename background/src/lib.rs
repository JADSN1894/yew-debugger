use gloo::console::log;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use serde_json::{json, Value};
use wasm_bindgen::{
    prelude::wasm_bindgen,
    // JsCast,
    JsValue,
};
use web_sys::console::{self};

// TODO:
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["chrome.runtime"])]
//     fn sendMessage(message: JsValue);

//     #[wasm_bindgen(js_namespace = ["chrome.runtime.onMessage"])]
//     fn addListener(callback: &Function);

//     #[wasm_bindgen(js_namespace = ["chrome.storage.local.get"])]
//     fn get(keys: JsValue);

//     #[wasm_bindgen(js_namespace = ["chrome.storage.local.set"])]
//     fn set(items: JsValue, func: &Function);
// }

#[wasm_bindgen]
pub fn print() {
    console::log_1(&"[from wasm] Hello World!".into());
}

#[wasm_bindgen]
pub fn print_with_value(value: &str) {
    console::log_2(&"[from wasm] Hello".into(), &value.into());
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    metadata: EventMetadata,
    model: Value,
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize)]
pub struct EventMetadata {
    msg: String,
    msg_id: u32,
}

#[wasm_bindgen]
pub fn yew_debugger_collector(input: String) -> Result<String, String> {
    log!("WASM - [{}]: yew_debugger_collector", file!());
    log!(&input);
    match serde_json::from_str::<EventMetadata>(&input) {
        Ok(inner_envelope) => {
            log!(format!("{:?}", &inner_envelope));
            serde_json::to_string(&inner_envelope).map_err(|err| err.to_string())
        }
        Err(error) => {
            let envelope = json!({
                "inputError": error.to_string()
            });

            serde_json::to_string(&envelope).map_err(|err| err.to_string())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Command {
    name: PanelCommand,
    data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum PanelCommand {
    GetEvents,
}
#[wasm_bindgen]
pub fn yew_debugger_panel(input: String) -> Result<String, String> {
    log!("WASM - [{}]: yew_debugger_collector", file!());
    log!(&input);
    match serde_json::from_str::<EventMetadata>(&input) {
        Ok(inner_envelope) => {
            log!(format!("{:?}", &inner_envelope));
            serde_json::to_string(&inner_envelope).map_err(|err| err.to_string())
        }
        Err(error) => {
            let envelope = json!({
                "error": error.to_string()
            });

            serde_json::to_string(&envelope).map_err(|err| err.to_string())
        }
    }
}

// #[wasm_bindgen]
// pub fn receive_envelope(input: String) -> Result<String, String> {
//     log!("WASM - [{}]: receive_envelope", file!());
//     log!(&input);
//     match serde_json::from_str::<Envelope>(&input) {
//         Ok(inner_envelope) => {
//             log!(format!("{:?}", &inner_envelope));
//             serde_json::to_string(&inner_envelope).map_err(|err| err.to_string())
//         }
//         Err(error) => {
//             let mut envelope = Envelope {
//                 message: Some(error.to_string()),
//             };
//             envelope.message = Some(error.to_string());

//             serde_json::to_string(&envelope).map_err(|err| err.to_string())
//         }
//     }
// }

// match serde_wasm_bindgen::from_value::<Envelope>(input) {
// }

// fn do_something() {
// let closure = Closure::wrap(Box::new(move |message: JsValue, _: JsValue, _: JsValue| {
//     log!("create closure -> message");

//     match serde_wasm_bindgen::from_value::<EnvelopeWrapper>(message) {
//         Ok(envelope) => {
//             log!("&envelope");
//             log!(format!("{:?}", &envelope));
//             ctx_clone.send_message(Msg::UpdateData(envelope.clone()));
//         }
//         Err(error) => {
//             log!("ERROR");
//             log!(error.to_string());
//         }
//     };
// }) as Box<dyn FnMut(JsValue, JsValue, JsValue)>);

// addListener(&closure.as_ref().unchecked_ref());

// Prevent the closure from being dropped
// closure.forget();
// }
