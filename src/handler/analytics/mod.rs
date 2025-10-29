use crate::handler::analytics::model::AnalyticsResponse;
use crate::handler::request::get;
use crate::{SPClient, SPResult, UnixTimestamp};

pub mod model;

pub(crate) async fn get_analytics(client: &SPClient, start_time: UnixTimestamp, end_time: UnixTimestamp) -> SPResult<AnalyticsResponse> {
    get(client, format!("/user/analytics?startTime={start_time}&endTime={end_time}")).await
}