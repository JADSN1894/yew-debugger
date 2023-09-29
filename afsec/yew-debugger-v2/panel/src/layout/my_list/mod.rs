pub mod item;

use std::rc::Rc;
use yew::AttrValue;
use yew::Component;
use yew::Context;
use yew::ContextHandle;
use yew::Properties;
use yew::{html, Html};

use crate::model::Model;
use crate::model::VItem;
use crate::model::Vlist;

/// The `MyList` component is the child of the `Parent` component, and will send and receive updates
/// to/from the grandparent using the context.
pub struct MyList {
    state: Rc<Model>,
    _listener: ContextHandle<Rc<Model>>,
}

pub enum MyListMsg {
    ContextChanged(Rc<Model>),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct MyListProps {
    pub name: AttrValue,
}

impl Component for MyList {
    type Message = MyListMsg;
    type Properties = MyListProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Here we fetch the shared state from the context. For a demonstration on the use of
        // context in a functional component, have a look at the `examples/contexts` code.
        let (mut state, _listener) = ctx
            .link()
            .context::<Rc<Model>>(ctx.link().callback(MyListMsg::ContextChanged))
            .expect("context to be set");
        let vec_vitem = state
            .data
            .users.0
            .iter()
            .map(|user| user.into())
            .collect::<Vec<VItem>>();

        {
            let vlist = Vlist(vec_vitem);
            let binding = Rc::make_mut(&mut state);
            binding.vlist = Some(vlist);
        }

        // let vlist: Vlist = state.data.users.iter().map(|user| user.into()).collect();
        // state.data.users.
        Self { state, _listener }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MyListMsg::ContextChanged(mut state) => {
                let vec_vitem = state
                    .data
                    .users.0
                    .iter()
                    .map(|user| user.into())
                    .collect::<Vec<VItem>>();

                {
                    let vlist = Vlist(vec_vitem);
                    let binding = Rc::make_mut(&mut state);
                    binding.vlist = Some(vlist);
                }

                self.state = state;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let component_name = ctx.props().name.clone();
        // let name = forma{ for }t!("I'm {my_name}.");

        // Here we emit the callback to the grandparent component, whenever the button is clicked.
        // let onclick = self.state.child_clicked.reform(move |_| (my_name.clone()));
        if let Some(vdom) = self.state.vlist.clone() {
            let users = vdom.0;

            html! {
            <div class="parent-body">
                <div class="parent-tag">
                    <span>{ component_name }</span>
                </div>
                <div class="parent-content">
                { for users.iter().map(|vitem| vitem.render()) }


                </div>
            </div>
            }
        } else {
            html! {
            <div class="parent-body">
                <div class="parent-tag">
                    <span>{ component_name }</span>
                </div>
                <div class="parent-content">



                </div>
            </div>
            }
        }
    }
}
