#[macro_use]
mod macros;
mod traits;

use std::rc::Rc;

use gloo::{console::log, timers::callback::Interval, utils::format::JsValueSerdeExt};
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
    ChangeEventCotentOnClick(Event),
}

#[derive(Debug)]
pub struct App {
    model: Model,
    current_event: Option<Event>,
    tick: Interval,
}

impl App {
    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn current_event(&self) -> Option<&Event> {
        self.current_event.as_ref()
    }

    pub fn set_current_event(&mut self, current_event: Event) {
        self.current_event = Some(current_event);
    }
}

#[derive(Debug, Default, Clone)]
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
    #[serde(rename = "data")]
    maybe_data: Option<Vec<Event>>,
    maybe_error: Option<String>,
}

impl MessageOutcome {
    pub fn is_ok(&self) -> bool {
        self.is_ok
    }

    pub fn maybe_outcome(&self) -> Option<&Vec<Event>> {
        self.maybe_data.as_ref()
    }

    pub fn maybe_error(&self) -> Option<&String> {
        self.maybe_error.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    metadata: EventMetadata,
    model: Value,
}

impl Event {
    pub fn model(&self) -> &Value {
        &self.model
    }

    pub fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMetadata {
    msg: String,
    msg_id: u32,
}

impl EventMetadata {
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn msg_id(&self) -> u32 {
        self.msg_id
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let tick = {
            let link = ctx.link().clone();
            Interval::new(100, move || link.send_message(Msg::GetEvents))
        };
        Self {
            tick,
            model: Default::default(),
            current_event: Default::default(),
        }
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
                // log!("Get events");
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

                // log!("panel_wasm::sendMessage::js_value");
                // log!(&js_value);
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
            Msg::ChangeEventCotentOnClick(input_inner) => {
                self.set_current_event(input_inner);
                true
            }
        };
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let model = self.model().clone();
        let MessageOutcome {
            is_ok,
            maybe_data,
            maybe_error,
        } = model.message_outcome().clone();

        html!(
            <main class="h-full w-full p-4">
                <div class="flex flex-col w-full gap-y-2">
                    <div class="relative">
                        // <button class="text-accent-content font-mono fixed top-2 px-4 w-full btn btn-sm btn-accent" onclick={ctx.link().callback(|_| Msg::GetEvents)}>
                        //     { "[ Get Events ]" }
                        // </button>
                    </div>
                    // if is_ok.clone() == false && data.is_none() && error.is_none() {
                    //     <pre class="pt-6 text-primary-content font-mono"><code>{model_view}</code></pre>
                    // } else {
                        <div class="flex pt-6">
                            <div class="w-[30%]">
                                <>
                                {

                                    if is_ok && maybe_data.is_some() {
                                        if let Some(events) = maybe_data {
                                            if events.is_empty() {
                                                html!(<h1 class="!text-4xl text-base-content font-bold font-mono uppercase">{"No events"}</h1>)
                                            } else {
                                                html!(
                                                    // TODO
                                                    /* FIX - [JADSN]: Height base on user height size */
                                                    <div class="pr-1 h-[600px] overflow-y-auto">
                                                        <div class="flex flex-col  w-full gap-y-2 ">
                                                        {
                                                            events.into_iter().rev().map(|event_item| {
                                                                let cur_event_item = event_item.clone();
                                                                let my_callback = move |_| Msg::ChangeEventCotentOnClick(cur_event_item.clone());
                                                                let msg = event_item.metadata().msg();
                                                                let msg_id = event_item.metadata().msg_id();
                                                                if let Some(current_event) = self.current_event() {
                                                                    if current_event.metadata().msg_id() == msg_id {
                                                                        html!(
                                                                            <button class="flex items-center justify-between h-12 p-2 rounded-lg bg-warning text-base-content hover:bg-warning-focus" onclick={ctx.link().callback(my_callback)}>
                                                                                <span class="font-mono font-bold uppercase">{msg}</span>
                                                                                <span class="font-mono font-bold uppercase">{msg_id}</span>
                                                                            </button>
                                                                        )
                                                                    } else {
                                                                        html!(
                                                                            <button class="flex items-center justify-between h-12 p-2 rounded-lg bg-primary text-primary-content hover:bg-primary-focus" onclick={ctx.link().callback(my_callback)}>
                                                                                <span class="font-mono font-bold uppercase">{msg}</span>
                                                                                <span class="font-mono font-bold uppercase">{msg_id}</span>
                                                                            </button>
                                                                        )
                                                                    }
                                                                } else {
                                                                    html!(
                                                                        <button class="flex items-center justify-between h-12 p-2 rounded-lg bg-primary text-primary-content hover:bg-primary-focus" onclick={ctx.link().callback(my_callback)}>
                                                                            <span class="font-mono font-bold uppercase">{msg}</span>
                                                                            <span class="font-mono font-bold uppercase">{msg_id}</span>
                                                                        </button>
                                                                    )
                                                                }

                                                            }).collect::<Html>()
                                                        }
                                                        </div>
                                                    </div>
                                                )
                                            }
                                        } else {
                                            html!(
                                                <div>
                                                    <h1 class="text-error-content font-bold font-mono uppercase">{"Impossible state"}</h1>
                                                </div>
                                            )
                                        }
                                    } else {
                                        if let Some(message_outcome_error) = maybe_error {
                                            html!(<h1 class="text-error-content font-bold font-mono uppercase">{message_outcome_error}</h1>)
                                        } else {
                                            html!(
                                                <div>
                                                    <h1 class="text-error-content font-bold font-mono uppercase">{"Impossible state"}</h1>

                                                </div>
                                            )
                                        }
                                    }
                                }
                                </>
                            </div>
                            // TODO
                            /* FIX - [JADSN]: Height base on user height size */
                            <div class="flex flex-col flex-grow pl-1 overflow-y-auto h-[600px]">
                               <div class="flex flex-col">
                                    <pre class="text-primary-content uppercase font-mono font-bold">
                                        <span>{"-- Message "}</span>
                                        {self.current_event().map(|cur_event|
                                            html!(<span class="text-base-content">{cur_event.metadata().msg_id()}</span>))}
                                    </pre>
                                    <pre class="text-base-content py-2 font-mono">
                                        {self.current_event().map(|cur_event| html!(<code>{cur_event.metadata().msg()}</code>))}
                                    </pre>
                                </div>
                                <div class="flex flex-col">
                                    <pre class="text-primary-content uppercase font-mono font-bold"><code>{"-- Model"}</code></pre>
                                    <pre class="text-base-content pt-2 font-mono">
                                        {
                                            self.current_event()
                                                .map(|cur_event| html!(
                                                        <code>
                                                            {
                                                                serde_json::to_string_pretty(cur_event.model()).unwrap_or_default()
                                                            }
                                                        </code>
                                                    )
                                                )
                                        }
                                    </pre>
                                </div>
                            </div>
                        </div>
                    // }
                </div>
            </main>
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
