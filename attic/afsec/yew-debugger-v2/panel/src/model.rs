use std::num::ParseIntError;

use yew::{html, AttrValue, Callback, Html};

use crate::layout::{my_list::item::Item, DialogForDeleteAction};

/// This is the shared state between the parent and child components.
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub vlist: Option<Vlist>,

    // * Dialog Rendering
    pub user_to_delete: Option<UserId>,
    /// Callback used when a child is clicked. The AttrValue is the name of the child that was
    /// clicked.
    pub callbacks: AppCallBacks,
    pub data: Data,

    /// The name of the child that was last clicked.
    pub last_clicked: Option<UserId>,
}

impl Model {
    pub fn user_to_delete(&self) -> Option<&UserId> {
        self.user_to_delete.as_ref()
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn set_user_to_delete(&mut self, maybe_user_id: Option<UserId>) {
        self.user_to_delete = maybe_user_id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppCallBacks {
    pub deletion_dialog: DeletionDialogCallBacks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeletionDialogCallBacks {
    pub dialog_open: Callback<DialogForDeleteAction>,
    pub dialog_delete_user: Callback<DialogForDeleteAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub users: Users,
    /// Total number of clicks received.
    pub total_clicks: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Users(pub Vec<User>);

impl Users {
    // * Public
    pub fn get_user_by_id(&self, id: &UserId) -> Option<&User> {
        self.0.iter().find(|user| &user.id == id)
    }

    pub fn delete_user(&mut self, id: &UserId) {
        let maybe_idx = self.get_user_idx_by_id(id);
        maybe_idx.map(|index| self.0.remove(index));
    }
    // Private
    fn get_user_idx_by_id(&self, id: &UserId) -> Option<usize> {
        self.0.iter().position(|user| &user.id == id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }
}

impl From<(u16, &'static str)> for User {
    fn from((id, name): (u16, &'static str)) -> Self {
        Self {
            id: UserId(id),
            name: UserName(name.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserId(pub u16);
impl TryFrom<AttrValue> for UserId {
    type Error = ParseIntError;

    fn try_from(value: AttrValue) -> Result<Self, Self::Error> {
        Ok(UserId(value.parse()?))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserName(pub String);

/// This is the shared state between the parent and child components.
#[derive(Debug, Clone, PartialEq)]
pub struct Vlist(pub Vec<VItem>);

#[derive(Debug, Clone, PartialEq)]
pub struct VItem {
    id: UserId,
    name: UserName,
}

impl From<&User> for VItem {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name.clone(),
        }
    }
}

impl VItem {
    pub fn render(&self) -> Html {
        html! {
            <Item id={self.id.0.to_string()} name={self.name.0.clone()} />
        }
    }
}
