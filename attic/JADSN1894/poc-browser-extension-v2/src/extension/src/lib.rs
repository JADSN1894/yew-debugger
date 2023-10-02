use gloo::console::log;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen(start)]
pub fn main() {
    log!(&JsValue::from("Hello, World!"));
}