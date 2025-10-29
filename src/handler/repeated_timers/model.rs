use serde::{Deserialize, Serialize};
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct RepeatedTimerData {
    /// Name of the reminder
    pub name: String,
    /// Message the reminder sends
    pub message: String,
    /// Day interval between the reminders are sent
    #[serde(rename = "dayInterval")]
    pub day_interval: u32,
    /// Time of day this reminder should go off
    pub time: RepeatedTimerTime,
    /// First day of this reminder (used for day intervals)
    #[serde(rename = "startTime")]
    pub start_time: RepeatedTimerStartTime,
    pub uid: String,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
}

#[derive(Serialize, Debug, Clone)]
pub struct RepeatedTimerCreateData {
    /// Name of the reminder
    pub name: String,
    /// Message the reminder sends
    pub message: String,
    /// Day interval between the reminders are sent
    #[serde(rename = "dayInterval")]
    pub day_interval: u32,
    /// Time of day this reminder should go off
    pub time: RepeatedTimerTime,
    /// First day of this reminder (used for day intervals)
    #[serde(rename = "startTime")]
    pub start_time: RepeatedTimerStartTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepeatedTimerTime {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepeatedTimerStartTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}