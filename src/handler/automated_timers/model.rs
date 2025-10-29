use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct AutomatedTimerData {
    /// Name of the reminder
    pub name: String,
    /// Message the reminder sends
    pub message: String,
    /// Delay in hours since the action (eg: front change)
    #[serde(rename = "delayInHours")]
    pub delay: u32,
    /// When this reminder should trigger
    pub r#type: AutomatedTimerAction,
    pub uid: String,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
}

#[derive(Serialize, Debug, Clone)]
pub struct AutomatedTimerCreateData {
    /// Name of the reminder
    pub name: String,
    /// Message the reminder sends
    pub message: String,
    /// Delay in hours since the action (eg: front change)
    #[serde(rename = "delayInHours")]
    pub delay: u32,
    /// When this reminder should trigger
    pub r#type: AutomatedTimerAction,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AutomatedTimerAction {
    MemberFrontChange = 0,
    CustomFrontChange = 1,
    AnyFrontChange = 2
}