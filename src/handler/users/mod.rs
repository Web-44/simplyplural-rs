use crate::handler::users::model::{UserData, UsernameData};
use crate::handler::request::{get, patch, patch_without_response};
use crate::{SPClient, SPResult};
use std::fmt::Display;
use crate::handler::model::{IdExists, ResultWithErrorMessage};

pub mod model;

pub type UserResponse = IdExists<UserData>;
pub type UsernameResponse = ResultWithErrorMessage;

pub(crate) async fn get_self_user(client: &SPClient) -> SPResult<UserResponse> {
    get(client, "/me").await
}

pub(crate) async fn get_user(client: &SPClient, user_id: impl Display) -> SPResult<UserResponse> {
    get(client, format!("/user/{user_id}")).await
}

pub(crate) async fn update_user(client: &SPClient, user_id: impl Display, user: &UserData) -> SPResult<()> {
    patch_without_response(client, format!("/user/{user_id}"), user).await
}

pub(crate) async fn set_username(client: &SPClient, user_id: impl Display, username: impl ToString) -> SPResult<UsernameResponse> {
    patch(client, format!("/user/username/{user_id}"), &UsernameData { username: username.to_string() }).await
}