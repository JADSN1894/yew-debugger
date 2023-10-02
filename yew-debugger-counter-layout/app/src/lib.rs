use std::ops::{AddAssign, SubAssign};

use gloo::{
    console::{self, log},
    utils::document,
};
use js_sys::Date;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::{html, Component, Context, Html};

pub enum Msg {
    Increment,
    Decrement,
    ToggleThemeMode,
}

#[derive(Clone, Debug, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl From<ThemeMode> for bool {
    fn from(value: ThemeMode) -> Self {
        match value {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
        }
    }
}

impl From<ThemeMode> for &str {
    fn from(value: ThemeMode) -> Self {
        match value {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Counter(i64);

impl Counter {
    pub fn take(self) -> i64 {
        self.0
    }

    pub fn set_counter(mut self, new_value: i64) {
        self.0 = new_value
    }
}

impl From<i64> for Counter {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl AddAssign for Counter {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Counter {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

#[derive(Debug)]
pub struct App {
    value: Counter,
    current_theme_mode: ThemeMode,
}

impl App {
    pub fn value(&self) -> &Counter {
        &self.value
    }

    pub fn set_value(&mut self, value: Counter) {
        self.value = value;
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
            value: Default::default(),
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value.add_assign(1.into());
                console::log!("plus one");
                true
            }

            Msg::Decrement => {
                self.value.sub_assign(1.into());
                console::log!("minus one");
                true
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

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex justify-center items-center h-screen w-screen font-mono border-success border-dashed border-2">
                <div class="flex flex-col gap-y-4 border-info border-dashed border-2 p-2">
                // Display the current value of the counter
                    <div class="flex items-center justify-between border-warning border-dashed border-2 w-full p-2">
                        <p class="text-3xl font-bold">
                            { self.value().clone().take() }
                        </p>
                        <input type="checkbox" class="toggle toggle-primary" onchange={ctx.link().callback(|_| Msg::ToggleThemeMode)} checked={self.current_theme_mode.clone().into()} />
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
                        <button class="btn btn-primary" onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                            { "+1, +1" }
                        </button>
                    </div>
                    <div class="border-warning border-dashed border-2 w-full p-2">
                        // Display the current date and time the page was rendered
                        <p class="text-xl">
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
