#[macro_use]
mod macros;
mod traits;

use gloo::{console::log, utils::format::JsValueSerdeExt};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use yew::{html, Component, Context, Html};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome.runtime"])]
    fn sendMessage(message: JsValue, callback: &Function);
}

#[derive(Debug, Clone, Serialize)]
pub enum Msg {
    Nil,
    GetEvents,
    UpdateDebuggerInView(MessageOutcome),
}

#[derive(Debug, Default, Clone)]
pub struct App {
    model: Model,
}

impl App {
    pub fn model(&self) -> &Model {
        &self.model
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Model {
    message_outcome: MessageOutcome,
}

impl Model {
    pub fn message_outcome(&self) -> &MessageOutcome {
        &self.message_outcome
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageOutcome {
    #[serde(rename = "isOk")]
    is_ok: bool,
    data: Option<Vec<Event>>,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    metadata: EventMetadata,
    model: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMetadata {
    msg: String,
    msg_id: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // let command: Option<()> = None;
        // let mut self_clone = self.clone();
        let command = json!(
            {
                "name": "GetEvents",
                "data": null
            }
        );

        let should_render = match msg {
            Msg::GetEvents => {
                log!("Get events");
                let message = json!(
                    {
                        "command": command,
                        "api": "yew-debugger-panel",
                    }
                );
                // TODO: Implement error handler
                let js_value = JsValue::from_serde(&message).unwrap_or_default();

                let ctx_clone = ctx.link().clone();

                let closure =
                    Closure::wrap(Box::new(move |message: JsValue, _: JsValue, _: JsValue| {
                        log!("create closure -> message");

                        match serde_wasm_bindgen::from_value::<MessageOutcome>(message) {
                            Ok(envelope) => {
                                log!("&envelope");
                                log!(format!("{:?}", &envelope));
                                ctx_clone.send_message(Msg::UpdateDebuggerInView(envelope));
                            }
                            Err(error) => {
                                log!("ERROR");
                                log!(error.to_string());
                            }
                        };
                    })
                        as Box<dyn FnMut(JsValue, JsValue, JsValue)>);

                // addListener(&closure.as_ref().unchecked_ref());

                // Prevent the closure from being dropped
                // closure.forget();

                log!("panel_wasm::sendMessage::js_value");
                log!(&js_value);
                sendMessage(js_value, &closure.as_ref().unchecked_ref());

                closure.forget();

                true
            }
            Msg::Nil => false,
            Msg::UpdateDebuggerInView(input_inner) => {
                self.model = Model {
                    message_outcome: input_inner,
                };

                true
            }
        };
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let model = self.model();
        let model_view = serde_json::to_string_pretty(&model).unwrap_or_default();
        let MessageOutcome { is_ok, data, error } = model.message_outcome();
        html!(
            <div class="h-full w-full p-4">
                <div class="flex flex-col w-full gap-y-2">
                    <button class="w-full btn btn-sm btn-accent" onclick={ctx.link().callback(|_| Msg::GetEvents)}>
                        { "[ Get Events ]" }
                    </button>
                    if is_ok.clone() == false && data.is_none() && error.is_none() {
                        <pre class="text-primary-content"><code>{model_view}</code></pre>
                    } else {
                        <div class="flex flex-col gap-y-2">
                            {
                                if is_ok.clone() == true && error.is_none() {
                                    if let Some(message_outcome_inner_data) = data {
                                        if message_outcome_inner_data.is_empty() {
                                            html!(<h1 class="!text-4xl text-primary-content font-bold font-mono uppercase">{"No events"}</h1>)
                                        } else {
                                            html!(
                                                {
                                                    message_outcome_inner_data.into_iter().rev().map(|event_item| {
                                                        let event_item_metadata = event_item.metadata.clone();
                                                        let event_item_model = event_item.model.clone();
                                                        let event_item_model_view = serde_json::to_string_pretty(&event_item_model).unwrap_or_default();
                                                        html!(
                                                            <div class="collapse bg-base-200">
                                                                <input type="checkbox" />
                                                                <div class="collapse-title text-xl font-medium">
                                                                    {format!("{} :: {}", event_item_metadata.msg, event_item_metadata.msg_id)}
                                                                </div>
                                                                <div class="collapse-content">
                                                                    <pre class="text-primary-content"><code>{event_item_model_view}</code></pre>
                                                                </div>
                                                            </div>
                                                        )
                                                    }).collect::<Html>()
                                                }
                                            )
                                        }
                                    } else {
                                        html!(
                                            <div>
                                                <h1 class="text-primary-content font-bold font-mono uppercase">{"Impossible state"}</h1>
                                                <pre class="text-primary-content"><code>{model_view}</code></pre>
                                            </div>
                                        )
                                    }
                                } else if is_ok.clone() == false && error.is_some() {
                                    if let Some(message_outcome_error) = error {
                                        html!(<h1 class="text-error-content font-bold font-mono uppercase">{message_outcome_error}</h1>)
                                    } else {
                                        html!(
                                            <div>
                                                <h1 class="text-primary-content font-bold font-mono uppercase">{"Impossible state"}</h1>
                                                <pre class="text-primary-content"><code>{model_view}</code></pre>
                                            </div>
                                        )
                                    }
                                } else {
                                    html!(
                                        <div>
                                            <h1 class="text-primary-content font-bold font-mono uppercase">{"Impossible state"}</h1>
                                            <pre class="text-primary-content"><code>{model_view}</code></pre>
                                        </div>
                                    )
                                }
                            }
                        </div>
                    }
                </div>
            </div>
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
