use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn welcome_to_background() {
    log!("[From background module]: Hello from YewDebugger");
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
    // log!("WASM - [{}]: yew_debugger_collector", file!());
    // log!(&input);
    match serde_json::from_str::<Event>(&input) {
        Ok(inner_envelope) => serde_json::to_string(&inner_envelope).map_err(|err| err.to_string()),
        Err(error) => {
            let envelope = json!({
                "inputError": error.to_string()
            });

            serde_json::to_string(&envelope).map_err(|err| err.to_string())
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
    // log!("WASM - [{}]: yew_debugger_panel", file!());
    // log!(&input);
    match serde_json::from_str::<EventMetadata>(&input) {
        Ok(inner_envelope) => serde_json::to_string(&inner_envelope).map_err(|err| err.to_string()),
        Err(error) => {
            let envelope = json!({
                "error": error.to_string()
            });

            serde_json::to_string(&envelope).map_err(|err| err.to_string())
        }
    }
}
