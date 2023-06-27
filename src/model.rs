use serde_derive::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Type, Debug, PartialEq)]
pub enum TodoState {
    Opened,
    Closed,
}

#[derive(Serialize, Clone, FromRow, Debug, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub state: TodoState,
}

#[derive(Deserialize, Clone, Validate)]
pub struct TodoInput {
    #[validate(length(
        min = 1,
        max = 200,
        message = "Can not be empty or longer then 200 characters"
    ))]
    pub text: String,
    pub state: TodoState,
}
