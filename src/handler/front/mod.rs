use std::fmt::Display;
use crate::{SPClient, SPResult, UnixTimestamp};
use crate::error::SPError;
use crate::handler::front::model::{FrontCreateData, FrontData, FrontUpdateData};
use crate::handler::request::{delete_without_response, get, patch_without_response, post_clear};
use crate::handler::model::IdExists;

pub mod model;

pub type FrontResponse = IdExists<FrontData>;

pub(crate) async fn get_current_fronters(client: &SPClient) -> SPResult<Vec<FrontResponse>> {
    get(client, "/fronters").await
}

pub(crate) async fn get_front_history(client: &SPClient, user_id: impl Display, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<Vec<FrontResponse>> {
    get(client, format!("/frontHistory/{user_id}?startTime={start_time}&endTime={end_time}")).await
}

pub(crate) async fn get_member_front_history(client: &SPClient, member_id: impl Display) -> SPResult<Vec<FrontResponse>> {
    get(client, format!("/frontHistory/member/{member_id}")).await
}

pub(crate) async fn get_member_front_history_in_interval(client: &SPClient, member_id: impl Display, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<Vec<FrontResponse>> {
    get(client, format!("/frontHistory/member/{member_id}?startTime={start_time}&endTime={end_time}")).await
}

pub(crate) async fn get_front_history_entry(client: &SPClient, user_id: impl Display, entry_id: impl Display) -> SPResult<FrontResponse> {
    get(client, format!("/frontHistory/{user_id}/{entry_id}")).await
}

pub(crate) async fn add_front_entry(client: &SPClient, entry: &FrontCreateData) -> Result<String, SPError> {
    if entry.live && entry.end_time.is_some() {
        return Err(SPError::Validation("A live front entry cannot have an end time.".to_string()));
    } else if !entry.live && entry.end_time.is_none() {
        return Err(SPError::Validation("A non-live front entry must have an end time.".to_string()));
    }
    post_clear(client, "/frontHistory", entry, true).await.map_err(|err| SPError::Reqwest(err))
}

pub(crate) async fn update_front_entry(client: &SPClient, entry_id: impl Display, entry: &FrontUpdateData) -> SPResult<()> {
    patch_without_response(client, format!("/frontHistory/{entry_id}"), entry).await
}

pub(crate) async fn delete_front_entry(client: &SPClient, entry_id: impl Display) -> SPResult<()> {
    delete_without_response(client, format!("/frontHistory/{entry_id}")).await
}