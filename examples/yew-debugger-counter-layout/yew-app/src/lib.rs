mod components;
mod models;

use crate::components::CounterComponent;
use gloo::{
    console::{self, log},
    utils::{document, format::JsValueSerdeExt},
};
use js_sys::Date;
use models::{CounterModel, ThemeMode};
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::cell::RefCell;
use std::ops::{AddAssign, SubAssign};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::DedicatedWorkerGlobalScope;
use web_sys::HtmlElement;
use yew::{html, Component, Context, Html};

thread_local! {
    static MSG_ID: RefCell<AtomicUsize> = const {
        RefCell::new(AtomicUsize::new(0))};

}

#[derive(Debug, Serialize)]
pub enum Msg {
    Increment,
    Decrement,
    ToggleThemeMode,
}

#[derive(Debug, Serialize)]
pub struct App {
    counter: CounterModel,
    current_theme_mode: ThemeMode,
}

impl App {
    fn send_to_debugger(event: Value) {
        // let msg_id = MSG_ID.with(|inner| inner.borrow().load(Ordering::SeqCst));

        let api = "yew-debugger-collector";
        let dbg_msg = json! {
        {
            "api": api,
            "event": event,
        }

        };
        let global_scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
        // let
        // Send the first message from Rust to the main thread
        // let _ = global_scope.post_message(&JsValue::from_str("Hello from Rust"));
        let _ = global_scope.post_message(&JsValue::from_serde(&dbg_msg).unwrap_or_default());

        // eval("yourJsFunction();");
    }
    pub fn counter(&self) -> &CounterModel {
        &self.counter
    }

    pub fn set_counter(&mut self, value: CounterModel) {
        self.counter = value;
    }

    pub fn current_theme_mode(&self) -> &ThemeMode {
        &self.current_theme_mode
    }

    pub fn set_current_theme_mode(&mut self, current_theme_mode: ThemeMode) {
        self.current_theme_mode = current_theme_mode;
    }
}

impl Default for App {
    fn default() -> Self {
        let html_element = document()
            .query_selector("html")
            .transpose()
            .ok_or("Error on get html tag")
            .unwrap_throw()
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        html_element
            .set_attribute("data-theme", ThemeMode::default().into())
            .unwrap_throw();

        Self {
            counter: Default::default(),
            current_theme_mode: Default::default(),
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.counter.add_assign(1.into());
                console::log!("plus one");
            }

            Msg::Decrement => {
                self.counter.sub_assign(1.into());
                console::log!("minus one");
            }

            Msg::ToggleThemeMode => {
                match self.current_theme_mode {
                    ThemeMode::Dark => self.current_theme_mode = ThemeMode::Light,
                    ThemeMode::Light => self.current_theme_mode = ThemeMode::Dark,
                };

                let html_element = document()
                    .query_selector("html")
                    .transpose()
                    .ok_or("Error on get html tag")
                    .unwrap_throw()
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();

                html_element
                    .set_attribute("data-theme", self.current_theme_mode.clone().into())
                    .unwrap_throw();

                log!(format!("{:?}", html_element));
            }
        };
        let model = serde_json::to_value(&self).unwrap_or_default();
        let msg_id = MSG_ID.with(|inner| inner.borrow_mut().fetch_add(1, Ordering::SeqCst));
        let envelope = json! {
            {
                "model": model,
                "metadata": {
                    "msg_id": msg_id,
                    "msg": msg,
                }
            }
        };
        App::send_to_debugger(envelope.into());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="h-screen w-screen flex items-center justify-center border-info border-dashed border-2">
                <div class="w-[400px] h-[750px] flex flex-col items-center justify-center border-success border-dashed border-2">

                        // Header
                        <div class="flex items-center justify-end mr-2 border-success border-dashed border-2">
                            <input type="checkbox" class="toggle toggle-primary" onchange={ctx.link().callback(|_| Msg::ToggleThemeMode)} checked={self.current_theme_mode.clone().into()} />
                        </div>

                        // Content
                        <div class="flex flex-col flex-grow items-center justify-center gap-y-2 border-warning border-dashed border-2 w-full p-2">

                            // Display the current value of the counter
                            <div>
                                <p class="text-3xl font-bold border-warning border-dashed border-2 w-full p-2">
                                    { self.counter().clone().take() }
                                </p>
                            </div>

                            <div class="flex items-center justify-center gap-x-2 border-warning border-dashed border-2 w-full p-2">
                                // A button to send the Increment message
                                <button class="btn btn-primary" onclick={ctx.link().callback(|_| Msg::Increment)}>
                                    { "+1" }
                                </button>

                                // A button to send the Decrement message
                                <button class="btn btn-primary" onclick={ctx.link().callback(|_| Msg::Decrement)}>
                                    { "-1" }
                                </button>

                                // A button to send two Increment messages
                                <button class="btn btn-success" onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                                    { "*2" }
                                </button>
                            </div>
                        </div>

                        // <CounterComponent counter={self.counter().clone()} />

                        // Footer
                        <div class="flex items-center justify-center border-success border-dashed border-2">
                            <p class="text-xs">
                                    { "Rendered: " }
                                    { String::from(Date::new_0().to_string()) }
                                </p>
                        </div>
                </div>
        </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();

    Ok(())
}
