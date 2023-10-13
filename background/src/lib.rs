use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console::{self};

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
