use yew::{html, Component, Properties};

use crate::models::CounterModel;

#[derive(Clone, Debug, Default, Properties, PartialEq)]
pub struct CounterComponentProps {
    pub counter: CounterModel,
}

#[derive(Clone, Debug, Default)]
pub struct CounterComponent;

#[derive(Clone, Debug)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

impl Component for CounterComponent {
    type Message = CounterMsg;

    type Properties = CounterComponentProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        html!(
            <div class="flex flex-col flex-grow items-center justify-center gap-y-2 border-warning border-dashed border-2 w-full p-2">

                // Display the current value of the counter
                <div>
                    <p class="text-3xl font-bold border-warning border-dashed border-2 w-full p-2">
                        { props.counter.clone().take() }
                    </p>
                </div>

                <div class="flex items-center justify-center gap-x-2 border-warning border-dashed border-2 w-full p-2">
                    // A button to send the Increment message
                    <button class="btn btn-primary" onclick={ctx.link().callback(|_| CounterMsg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button class="btn btn-primary" onclick={ctx.link().callback(|_| CounterMsg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button class="btn btn-primary" onclick={ctx.link().batch_callback(|_| vec![CounterMsg::Increment, CounterMsg::Increment])}>
                        { "*2" }
                    </button>
                </div>
            </div>

        )
    }
}
