// use gloo::console::log;
use js_sys::Function;
use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
use wasm_bindgen::{
    prelude::wasm_bindgen,
    // JsCast,
    JsValue,
};
use web_sys::console::{self};

// TODO:
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome.runtime"])]
    fn sendMessage(message: JsValue);

    #[wasm_bindgen(js_namespace = ["chrome.runtime.onMessage"])]
    fn addListener(callback: &Function);

    #[wasm_bindgen(js_namespace = ["chrome.storage.local.get"])]
    fn get(keys: JsValue);

    #[wasm_bindgen(js_namespace = ["chrome.storage.local.set"])]
    fn set(items: JsValue, func: &Function);
}

#[wasm_bindgen]
pub fn print() {
    console::log_1(&"[from wasm] Hello World!".into());
}

#[wasm_bindgen]
pub fn print_with_value(value: &str) {
    console::log_2(&"[from wasm] Hello".into(), &value.into());
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    message: String,
}

#[wasm_bindgen]
pub fn receive_evelope(input: JsValue) -> Result<JsValue, JsValue> {
    match serde_wasm_bindgen::from_value::<Envelope>(input) {
        Ok(inner_envelope) => Ok(serde_wasm_bindgen::to_value(&inner_envelope)?),
        Err(error) => {
            let mut envelope = Envelope {
                message: error.to_string(),
            };
            envelope.message = error.to_string();

            Ok(serde_wasm_bindgen::to_value(&envelope)?)
        }
    }
}
