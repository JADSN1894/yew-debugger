// use super::*;

use std::rc::Rc;

use yew::html;
use yew::AttrValue;
use yew::Component;
use yew::Context;
use yew::ContextHandle;
use yew::Html;
use yew::Properties;

use crate::model::Model;
use crate::model::UserId;

/// The `Item` component is the child of the `Parent` component, and will send and receive updates
/// to/from the grandparent using the context.
pub struct Item {
    state: Rc<Model>,
    _listener: ContextHandle<Rc<Model>>,
}

pub enum ItemMsg {
    ContextChanged(Rc<Model>),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct ItemProps {
    pub id: AttrValue,
    pub name: AttrValue,
}

impl Component for Item {
    type Message = ItemMsg;
    type Properties = ItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Here we fetch the shared state from the context. For a demonstration on the use of
        // context in a functional component, have a look at the `examples/contexts` code.
        let (state, _listener) = ctx
            .link()
            .context::<Rc<Model>>(ctx.link().callback(ItemMsg::ContextChanged))
            .expect("context to be set");

        Self { state, _listener }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ItemMsg::ContextChanged(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        let name = ctx.props().name.clone();
        let user_id_emit: UserId = ctx.props().id.clone().try_into().unwrap();

        // Here we emit the callback to the grandparent component, whenever the button is clicked.
        let onclick = self
            .state
            .callbacks.deletion_dialog.dialog_open
            .reform(move |_| (crate::layout::DialogForDeleteAction::Open(user_id_emit.clone())));

        html! {
            <div class="child-body">
                <div class="child-tag">
                    <span>{ "Item" }</span>
                </div>
                <div>
                    <span>{ "We've been clicked " }<span>{ self.state.data.total_clicks }</span>{ " times." }</span>
                </div>
                <div class="child-content">
                <span>{ id }</span>
                    <span>{ name }</span>
                    <button onclick={onclick}>{"Delete"}</button>
                </div>
            </div>
        }
    }
}
