pub mod my_list;

use self::my_list::MyList;
use crate::model::{AppCallBacks, Data, DeletionDialogCallBacks, Model, UserId, Users};
use std::rc::Rc;
use yew::{html, Component, Context, ContextProvider, Html};
// use gloo::console;

pub enum Msg {
    // ButtonClick(AttrValue),
    DialogForDelete(DialogForDeleteAction),
}

#[derive(Debug, PartialEq)]
pub enum DialogForDeleteAction {
    Open(UserId),
    Delete(UserId),
    Cancel,
}
/// Our top-level (grandparent) component that holds a reference to the shared state.
pub struct AppLayout {
    state: Rc<Model>,
}
impl Component for AppLayout {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dialog_open = ctx.link().callback(Msg::DialogForDelete);
        let dialog_delete_user = ctx.link().callback(Msg::DialogForDelete);
        let data = Data {
            users: Users(vec![(1, "Alice").into(), (2, "Bob").into()]),
            total_clicks: 0,
        };
        let model = Model {
            last_clicked: None,
            vlist: None,
            data,
            user_to_delete: None,
            callbacks: AppCallBacks {
                deletion_dialog: DeletionDialogCallBacks {
                    dialog_open,
                    dialog_delete_user,
                },
            },
        };
        let state = Rc::new(model);
        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DialogForDelete(action) => match action {
                DialogForDeleteAction::Open(user_id) => {
                    let shared_state = Rc::make_mut(&mut self.state);
                    shared_state.set_user_to_delete(Some(user_id));

                    shared_state.data.total_clicks += 1;
                    shared_state.last_clicked = shared_state.user_to_delete().cloned();
                }
                DialogForDeleteAction::Delete(user_id) => {
                    // let user_to_delete = self.state.user_to_delete().map(|id| id.clone());
                    let shared_state = Rc::make_mut(&mut self.state);
                    shared_state.data.users.delete_user(&user_id);
                    shared_state.set_user_to_delete(None);
                }
                DialogForDeleteAction::Cancel => {
                    let shared_state = Rc::make_mut(&mut self.state);
                    shared_state.set_user_to_delete(None);
                }
            },
        };

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let app_state = self.state.clone();

        let onclick_cancel = self
            .state
            .callbacks
            .deletion_dialog
            .dialog_delete_user
            .reform(move |_| (DialogForDeleteAction::Cancel));

        let maybe_user = app_state
            .clone()
            .user_to_delete()
            .map(|user_id| self.state.data().users.get_user_by_id(&user_id).cloned())
            .flatten();
        let rendered_dialog = if let Some(user) = maybe_user {
            let user_id = user.id().clone();
            let onclick_delete = self
                .state
                .callbacks
                .deletion_dialog
                .dialog_delete_user
                .reform(move |_| (DialogForDeleteAction::Delete(user_id.clone())));
            html! {
                <dialog open=true>
                    <p>{"Dialog for User"}</p>
                    <p>
                        {format!("{:?}",user)}
                    </p>
                    <p>
                    <button onclick={onclick_delete}>{"Delete"}</button>
                    </p>
                    <p>
                    <button onclick={onclick_cancel}>{"Cancel"}</button>
                    </p>
                </dialog>
            }
        } else {
            html! {
                <dialog></dialog>
            }
        };
        html! {
            <ContextProvider<Rc<Model>> context={app_state}>
                <div class="grandparent">
                    <div>
                        <h2 class="title">{ "Admin Panel" }</h2>
                        <div class="grandparent-body">
                            <div class="grandparent-tag">
                                <span>{ "Repositories" }</span>
                            </div>
                            <div class="grandparent-content">
                                <span>{rendered_dialog}</span>
                                <span>{ "Dialog has been opened " }<span>{ self.state.data().total_clicks }</span>{ " times." }</span>
                                <MyList name="Users" />
                            </div>
                        </div>
                    </div>
                </div>

            </ContextProvider<Rc<Model>>>
        }
    }
}
