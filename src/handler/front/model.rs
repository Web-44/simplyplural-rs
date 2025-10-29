use serde::{Deserialize, Serialize};
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct FrontData {
    pub uid: String,
    pub member: String,
    pub custom: bool,
    pub live: bool,
    #[serde(rename = "customStatus")]
    pub custom_status: String,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
    #[serde(rename = "startTime")]
    pub start_time: UnixTimestamp,
    #[serde(rename = "endTime")]
    pub end_time: Option<UnixTimestamp>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FrontCreateData {
    /// Id of the member or custom front this entry belongs to
    pub member: String,
    /// Whether this front history entry tracks a custom front or member
    pub custom: bool,
    /// Whether this front history entry is live
    pub live: bool,
    /// Time in Milliseconds since epoch
    #[serde(rename = "startTime")]
    pub start_time: UnixTimestamp,
    /// Time in Milliseconds since epoch
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<UnixTimestamp>,
    /// Custom status of the front entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "customStatus")]
    pub custom_status: Option<String>,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct FrontUpdateData {
    /// Id of the member or custom front this entry belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// Whether this front history entry tracks a custom front or member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    /// Whether this front history entry is live
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    /// Time in Milliseconds since epoch
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<UnixTimestamp>,
    /// Time in Milliseconds since epoch
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<UnixTimestamp>,
    /// Custom status of the front entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "customStatus")]
    pub custom_status: Option<String>,
}