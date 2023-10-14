#[macro_use]
mod macros;
mod traits;

use crate::traits::Name;
use gloo::console;
use gloo::utils::format::JsValueSerdeExt;
use js_sys::Date;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::fmt::{Debug, Display};
use std::{cell::RefCell, marker::PhantomData};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use yew::{html, Component, Context, Html};

const MODEL_INIT: Model = Model { counter: 0 };

thread_local! {
    static DEBBUGER_APP: RefCell<MVUDebbuger<App>> = const {
        RefCell::new(MVUDebbuger {
            component: PhantomData,
            cur_msg_number: 0,
            cur_msg: Msg::Nil,
            cur_model: MODEL_INIT })};

}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["chrome.runtime"])]
    fn sendMessage(message: JsValue);
}

impl_name! {App}

#[derive(Debug, Clone)]
pub struct MVUDebbuger<C> {
    component: PhantomData<C>,
    cur_msg_number: usize,
    cur_msg: Msg,
    cur_model: Model,
}

impl<C: Component + Name> From<MVUDebbuger<C>> for Value {
    fn from(debugger: MVUDebbuger<C>) -> Self {
        let model = serde_json::to_value(debugger.cur_model()).unwrap_or_default();
        json! {
            {
                "component": C::NAME,
                "cur_msg_number": debugger.cur_msg_number(),
                "cur_msg": debugger.cur_msg(),
                "cur_model": model,
            }

        }
    }
}

impl<C: Component + Name> Display for MVUDebbuger<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = serde_json::to_value(self.cur_model()).unwrap_or_default();
        let json = json! {
            {
                "component": C::NAME,
                "cur_msg_number": self.cur_msg_number(),
                "cur_msg": self.cur_msg(),
                "cur_model": model,
            }

        };
        write!(f, "{}", json.to_string())
    }
}
impl<C: Component + Name> MVUDebbuger<C> {
    // Getters
    pub fn component(&self) -> &'static str {
        C::NAME
    }
    pub fn cur_msg_number(&self) -> usize {
        self.cur_msg_number
    }
    pub fn cur_msg(&self) -> &Msg {
        &self.cur_msg
    }
    pub fn cur_model(&self) -> &Model {
        &self.cur_model
    }

    // Setters
    pub fn set_cur_msg_number(&mut self, cur_msg_number: usize) {
        self.cur_msg_number = cur_msg_number;
    }

    pub fn set_cur_msg(&mut self, cur_msg: Msg) {
        self.cur_msg = cur_msg;
    }

    pub fn set_cur_model(&mut self, cur_model: Model) {
        self.cur_model = cur_model;
    }
}

// Define the possible messages which can be sent to the component
#[derive(Debug, Clone, Serialize)]
pub enum Msg {
    Nil,
    GetEvents,
}

#[derive(Debug, Clone)]
pub struct App {
    model: Model, // This will store the counter value
}
#[derive(Debug, Clone, Serialize)]
pub struct Model {
    counter: i32,
}

impl App {
    fn send_to_debugger(envelope: Value) {
        let recipient = "yew-debugger";
        let dbg_msg = json! {
        {
            "recipient": recipient,
            "envelope": envelope
        }

        };
        // let global_scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
        // let
        // Send the first message from Rust to the main thread
        // let _ = global_scope.post_message(&JsValue::from_str("Hello from Rust"));
        // let _ = global_scope.post_message(&JsValue::from_serde(&dbg_msg).unwrap_or_default());

        // eval("yourJsFunction();");
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { model: MODEL_INIT }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let envelope: Option<()> = None;

        let should_render = match msg {
            Msg::GetEvents => {
                console::log!("Get events");
                let message = json! {
                    {
                        "envelope": envelope,
                        "recipient": "yew-debugger-panel",
                    }
                };
                // TODO: Implement error handler
                let js_value = JsValue::from_serde(&message).unwrap_or_default();
                let outcome = sendMessage(js_value);
                true // Return true to cause the displayed change to update
            }
            Msg::Nil => false,
        };
        DEBBUGER_APP.with(|inner| {
            let mut debbuger = inner.borrow_mut();
            let cur_msg_number = debbuger.cur_msg_number().clone();
            debbuger.set_cur_msg(msg.clone());
            debbuger.set_cur_msg_number(cur_msg_number + 1);
            debbuger.set_cur_model(self.model.clone());
        });
        let debugger = DEBBUGER_APP.with(|inner| inner.borrow().clone());
        App::send_to_debugger(debugger.into());
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let debugger = DEBBUGER_APP.with(|inner| inner.borrow().clone());

        html! {

            <div class="h-full w-full m-4">
                <div class="flex flex-col w-full gap-y-2">
                    <button class="w-full btn btn-sm btn-accent" onclick={ctx.link().callback(|_| Msg::GetEvents)}>
                        { "[ Get Events ]" }
                    </button>
                    <div class="flex flex-col gap-y-2">
                        {(0..=50).enumerate().map(|(index, item)| {
                            html!(
                                <div class="collapse bg-base-200">
                                    <input type="checkbox" /> 
                                    <div class="collapse-title text-xl font-medium">
                                     {format!("{index} Item - {item}")}
                                    </div>
                                    <div class="collapse-content"> 
                                      <p>{ debugger.to_string() }</p>
                                    </div>
                                  </div>
                            )
                        }).collect::<Html>()}
                    </div>
                </div>

                // Display the current value of the counter
                // <p class="counter">
                //     { self.model.counter}
                // </p>

                // Display the current date and time the page was rendered
                // <p class="footer">
                //     <p>
                //     { "Rendered: " }
                //     { String::from(Date::new_0().to_string()) }
                //     </p>
                //     <hr/>
                //     <h2>{ "Debugger: " }</h2>
                    <pre>
                    {
                        debugger.to_string()
                    }
                    </pre>
                //     // <p>
                //     // { "COMPONENT: " }
                //     // {
                //     //     debugger.component()
                //     // }
                //     // </p>
                //     // <p>
                //     // { "CURRENT_MSG_NUMBER: " }
                //     // {
                //     //     format!("{:?}",debugger.cur_msg_number())
                //     // }
                //     // </p>
                //     // <p>
                //     // { "CURRENT_MSG: " }
                //     // {
                //     //     format!("{:?}",debugger.cur_msg())
                //     // }
                //     // </p>
                //     // <p>
                //     // { "CURRENT_MODEL: " }
                //     // {
                //     //     format!("{:?}",debugger.cur_model())
                //     // }
                //     // </p>
                // </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
