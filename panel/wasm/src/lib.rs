#[macro_use]
mod macros;
mod traits;

use crate::traits::Name;
use gloo::console::log;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    marker::PhantomData,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use yew::{html, Component, Context, Html, ToHtml};

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

const MODEL_INIT: Model = Model { data: None };

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
    UpdateData(EnvelopeWrapper),
}

#[derive(Debug, Clone, Serialize)]
pub struct App {
    model: Model,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Model {
    data: Option<EnvelopeWrapper>,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct EnvelopeMetada {
    msg: String,
    msg_id: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Envelope {
    metadata: EnvelopeMetada,
    model: Value,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvelopeWrapper {
    envelope: Envelope,
    recipient: String,
}

impl ToHtml for EnvelopeWrapper {
    fn to_html(&self) -> Html {
        log!(format!("View"));
        log!(format!("{:?}", &self));
        let html = serde_json::to_string_pretty(self)
            .map_or("See the logs".into(), |error| error.to_string());

        html!(
            <div>
                {html}
            </div>
        )
    }
}

impl App {
    fn send_to_debugger(envelope: Value) {
        log!("send_to_debugger(_)");
        let recipient = "yew-debugger";
        let dbg_msg = json! {
            {
                "recipient": recipient,
                "envelope": envelope
            }
        };
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("panel create");

        let ctx_clone = ctx.link().clone();

        let closure = Closure::wrap(Box::new(move |message: JsValue, _: JsValue, _: JsValue| {
            log!("create closure -> message");

            match serde_wasm_bindgen::from_value::<EnvelopeWrapper>(message) {
                Ok(envelope) => {
                    log!("&envelope");
                    log!(format!("{:?}", &envelope));
                    ctx_clone.send_message(Msg::UpdateData(envelope.clone()));
                }
                Err(error) => {
                    log!("ERROR");
                    log!(error.to_string());
                }
            };
        }) as Box<dyn FnMut(JsValue, JsValue, JsValue)>);

        addListener(&closure.as_ref().unchecked_ref());

        // Prevent the closure from being dropped
        closure.forget();

        Self { model: MODEL_INIT }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("panel update");

        let outcome = match msg {
            Msg::UpdateData(new_data) => {
                log!("Update - new_data");
                log!(format!("{:?}", &new_data));

                self.model.data = Some(new_data);
                true
            }
            Msg::Nil => true,
        };

        // DEBBUGER_APP.with(|inner| {
        //     let mut debbuger = inner.borrow_mut();
        //     let cur_msg_number = debbuger.cur_msg_number().clone();
        //     debbuger.set_cur_msg(msg.clone());
        //     debbuger.set_cur_msg_number(cur_msg_number + 1);
        //     debbuger.set_cur_model(self.model.clone());
        // });
        // let debugger = DEBBUGER_APP.with(|inner| inner.borrow().clone());
        // App::send_to_debugger(debugger.into());
        outcome
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let debugger = DEBBUGER_APP.with(|inner| inner.borrow().clone());

        html!(
            <div>
                <div>
                    <p>{ "Bozo" }</p>
                </div>
                <div>
                    <p>{ self.model.data.clone() }</p>
                </div>
                // <pre>
                //      {
                //          debugger.to_string()
                //      }
                //  </pre>
            </div>
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();

    // let yew_main = gloo::utils::document()
    //     .get_element_by_id("devtools-panel-main")
    //     .unwrap();
    // yew::Renderer::<App>::with_root(yew_main).render();
}
