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
    Increment,
    Decrement,
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
        let global_scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
        // let
        // Send the first message from Rust to the main thread
        // let _ = global_scope.post_message(&JsValue::from_str("Hello from Rust"));
        let _ = global_scope.post_message(&JsValue::from_serde(&dbg_msg).unwrap_or_default());

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
        let outcome = match msg {
            Msg::Increment => {
                self.model.counter += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.model.counter -= 1;
                console::log!("minus one");
                true
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
        outcome
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let debugger = DEBBUGER_APP.with(|inner| inner.borrow().clone());

        html! {

            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.model.counter}
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    <p>
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                    </p>
                    <hr/>
                    <h2>{ "Debugger: " }</h2>
                    <pre>
                    {
                        debugger.to_string()
                    }
                    </pre>
                    // <p>
                    // { "COMPONENT: " }
                    // {
                    //     debugger.component()
                    // }
                    // </p>
                    // <p>
                    // { "CURRENT_MSG_NUMBER: " }
                    // {
                    //     format!("{:?}",debugger.cur_msg_number())
                    // }
                    // </p>
                    // <p>
                    // { "CURRENT_MSG: " }
                    // {
                    //     format!("{:?}",debugger.cur_msg())
                    // }
                    // </p>
                    // <p>
                    // { "CURRENT_MODEL: " }
                    // {
                    //     format!("{:?}",debugger.cur_model())
                    // }
                    // </p>
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
