use serde::{Deserialize, Serialize};
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct NoteData {
    pub uid: String,
    pub title: String,
    pub note: String,
    #[serde(rename = "supportMarkdown")]
    pub markdown: bool,
    pub color: String,
    pub member: String,
    pub date: UnixTimestamp,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
}

#[derive(Serialize, Debug, Clone)]
pub struct NoteCreateData {
    /// Name of the note
    pub title: String,
    /// Note text
    pub note: String,
    /// Whether the description supports markdown
    #[serde(skip_serializing_if = "Option::is_none", rename = "supportMarkdown")]
    pub markdown: Option<bool>,
    /// Color of the object in a hex format (eg: #ffffff, #ffffffff or ffffff)
    pub color: String,
    /// Id of the member this note belongs to
    pub member: String,
    /// Time in Milliseconds since epoch
    pub date: UnixTimestamp,
}

#[derive(Serialize, Debug, Clone)]
pub struct NoteUpdateData {
    /// Name of the note
    pub title: Option<String>,
    /// Note text
    pub note: Option<String>,
    /// Whether the description supports markdown
    #[serde(skip_serializing_if = "Option::is_none", rename = "supportMarkdown")]
    pub markdown: Option<bool>,
    /// Color of the object in a hex format (eg: #ffffff, #ffffffff or ffffff)
    pub color: Option<String>,
}