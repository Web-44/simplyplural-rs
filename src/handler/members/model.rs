use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::handler::model::Frame;
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct MemberData {
    pub uid: String,
    /// Name of the member
    pub name: String,
    /// Description of the member
    #[serde(rename = "desc")]
    pub description: String,
    /// Whether the description supports markdown
    #[serde(rename = "supportDescMarkdown")]
    pub description_markdown: bool,
    /// Pronouns of the member
    pub pronouns: String,
    /// Pluralkit Id of the member, 5 or 6 letter string
    #[serde(rename = "pkId")]
    pub pluralkit_id: String,
    /// URL for the avatar
    #[serde(rename = "avatarUrl")]
    pub avatar: String,
    #[serde(rename = "avatarUuid")]
    pub avatar_uuid: Option<String>,
    /// Whether this member is private
    pub private: bool,
    /// Whether trusted friends are prevented from seeing this private member
    #[serde(rename = "preventTrusted")]
    pub prevent_trusted: bool,
    /// Whether to prefer front notification changes
    #[serde(rename = "preventsFrontNotifs")]
    pub prevents_front_notifications: bool,
    /// Whether this member is archived
    pub archived: bool,
    #[serde(rename = "archivedReason")]
    pub archived_reason: String,
    #[serde(rename = "receiveMessageBoardNotifs")]
    pub receive_message_board_notifications: bool,
    /// Color of the member in hex format (eg: #ffffff)
    pub color: String,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
    pub buckets: Vec<String>,
    pub frame: Option<Frame>,
    pub info: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MemberCreateData {
    /// Name of the member
    pub name: String,
    /// Description of the member
    #[serde(skip_serializing_if = "Option::is_none", rename = "desc")]
    pub description: Option<String>,
    /// Whether the description supports markdown
    #[serde(skip_serializing_if = "Option::is_none", rename = "supportDescMarkdown")]
    pub description_markdown: Option<bool>,
    /// Pronouns of the member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    /// Pluralkit Id of the member, 5 or 6 letter string
    #[serde(skip_serializing_if = "Option::is_none", rename = "pkId")]
    pub pluralkit_id: Option<String>,
    /// URL for the avatar
    #[serde(skip_serializing_if = "Option::is_none", rename = "avatarUrl")]
    pub avatar: Option<String>,
    /// Whether this member is private
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Whether trusted friends are prevented from seeing this private member
    #[serde(skip_serializing_if = "Option::is_none", rename = "preventTrusted")]
    pub prevent_trusted: Option<bool>,
    /// Whether to prefer front notification changes
    #[serde(skip_serializing_if = "Option::is_none", rename = "preventsFrontNotifs")]
    pub prevents_front_notifications: Option<bool>,
    /// Color of the member in hex format (eg: #ffffff, #ffffffff or ffffff)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<HashMap<String, String>>,
}