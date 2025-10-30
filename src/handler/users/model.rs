use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::handler::model::Frame;
use crate::UnixTimestamp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    #[serde(skip_serializing)]
    pub uid: String,
    /// Whether the user is a system
    #[serde(rename = "isAsystem")]
    pub is_a_system: bool,
    #[serde(skip_serializing, rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
    /// Name of the user
    #[serde(skip_serializing)]
    pub username: String,
    /// URL for the avatar
    #[serde(skip_serializing_if = "Option::is_none", rename = "avatarUrl")]
    pub avatar: Option<String>,
    /// Color of the user in hex format (eg: #ffffff)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Description of the user
    #[serde(skip_serializing_if = "Option::is_none", rename = "desc")]
    pub description: Option<String>,
    /// Whether the description supports markdown
    #[serde(skip_serializing_if = "Option::is_none", rename = "supportDescMarkdown")]
    pub description_markdown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<Frame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, UserField>>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserField {
    pub name: String,
    pub order: usize,
    pub private: bool,
    #[serde(rename = "preventTrusted")]
    pub prevent_trusted: bool,
    pub r#type: usize,
    #[serde(rename = "supportMarkdown")]
    pub support_markdown: Option<bool>,
}

#[derive(Serialize)]
pub(in crate::handler::users) struct UsernameData {
    pub username: String,
}