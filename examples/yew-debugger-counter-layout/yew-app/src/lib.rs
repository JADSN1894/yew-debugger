mod models;

use gloo::{
    console::{self, log},
    utils::document,
};
use js_sys::Date;
use models::{CounterModel, ThemeMode};
use serde::Serialize;
use std::ops::{AddAssign, SubAssign};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::{html, Component, Context, Html};
use yew_debugger::impl_yew_debugger;
use yew_debugger_derive::{YewMessage, YewModel};

#[derive(Debug, Serialize, YewMessage)]
pub enum Msg {
    Increment,
    Decrement,
    ToggleThemeMode,
}

#[derive(Debug, Serialize, YewModel)]
pub struct App {
    counter: CounterModel,
    current_theme_mode: ThemeMode,
}

impl App {
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

// * Yew Debugger Setup: 1 of 2
impl_yew_debugger!(App, Msg);

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // * Yew Debugger Setup: 2 of 2
        #[cfg(debug_assertions)]
        App::send_to_debbuger(self, &msg);

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
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="h-screen w-screen flex items-center justify-center overflow-hidden">
                <div class="flex flex-col items-center justify-center">

                        // Header
                        <div class="flex items-center justify-end mr-2">
                            <input type="checkbox" class="toggle toggle-primary" onchange={ctx.link().callback(|_| Msg::ToggleThemeMode)} checked={self.current_theme_mode.clone().into()} />
                        </div>

                        // Content
                        <div class="flex flex-col flex-grow items-center justify-center gap-y-2 w-full p-2">

                            // Display the current value of the counter
                            <div>
                                <p class="text-3xl font-bold w-full p-2">
                                    { self.counter().clone().take() }
                                </p>
                            </div>

                            <div class="flex items-center justify-center gap-x-2 w-full p-2">
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

                        // Footer
                        <div class="flex flex-col items-center justify-center">
                            <p>{ "Rendered" }</p>
                            <p class="text-xs">
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
