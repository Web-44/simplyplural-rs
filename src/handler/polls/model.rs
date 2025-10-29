use serde::{Deserialize, Serialize};
use crate::UnixTimestamp;

#[derive(Deserialize, Debug, Clone)]
pub struct PollData {
    pub uid: String,
    /// Name of the poll
    pub name: String,
    /// Description of the poll
    #[serde(rename = "desc")]
    pub description: String,
    /// Whether the description supports markdown
    #[serde(rename = "supportDescMarkdown")]
    pub description_markdown: Option<bool>,
    /// Whether this poll is a custom poll with custom answers
    pub custom: bool,
    /// Time in Milliseconds since epoch
    #[serde(rename = "endTime")]
    pub end_time: UnixTimestamp,
    #[serde(rename = "lastOperationTime")]
    pub last_operation_time: UnixTimestamp,
    /// Whether members can abstain from voting, only for non-custom polls
    #[serde(rename = "allowAbstain")]
    pub allow_abstain: Option<bool>,
    /// Whether members can veto, only for non-custom polls
    #[serde(rename = "allowVeto")]
    pub allow_veto: Option<bool>,
    /// Available answers for this poll, only for custom polls
    pub options: Option<Vec<PollCustomAnswer>>,
    /// All the votes for this poll
    pub votes: Option<Vec<PollVote>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PollCreateData {
    /// Name of the poll
    pub name: String,
    /// Description of the poll
    #[serde(rename = "desc")]
    pub description: String,
    /// Whether this poll is a custom poll with custom answers
    pub custom: bool,
    /// Time in Milliseconds since epoch
    #[serde(rename = "endTime")]
    pub end_time: UnixTimestamp,
    /// Whether members can abstain from voting, only for non-custom polls
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowAbstain")]
    pub allow_abstain: Option<bool>,
    /// Whether members can veto, only for non-custom polls
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowVeto")]
    pub allow_veto: Option<bool>,
    /// Available answers for this poll, only for custom polls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<PollCustomAnswer>>,
    /// All the votes for this poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes: Option<Vec<PollVote>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollCustomAnswer {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollVote {
    /// Id of the member who voted
    pub id: String,
    /// Vote of the member, can be "yes", "no", "abstain" or "veto" for non-custom polls, or a custom answer for custom polls
    pub vote: String,
    pub comment: String,
}