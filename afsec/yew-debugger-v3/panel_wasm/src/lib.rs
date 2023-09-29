mod layout;
mod model;

use crate::layout::AppLayout;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<AppLayout>::new().render();
}
