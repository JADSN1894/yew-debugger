use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

#[wasm_bindgen]
pub fn print() {
    console::log_1(&"[from wasm] Hello World!".into());
}

#[wasm_bindgen]
pub fn print_with_value(value: &str) {
    // with 2-args log function
    console::log_2(&"[from wasm] Hello".into(), &value.into());
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    message: String,
}

#[wasm_bindgen]
pub fn receive_evelope(input: JsValue) -> Result<JsValue, JsValue> {
    match serde_wasm_bindgen::from_value::<Envelope>(input) {
        Ok(evelope) => Ok(serde_wasm_bindgen::to_value(&evelope)?),
        Err(error) => {
            let mut envelope = Envelope { message: "".into() };
            envelope.message = error.to_string();

            Ok(serde_wasm_bindgen::to_value(&envelope)?)
        }
    }
}
