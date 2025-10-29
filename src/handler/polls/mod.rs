use crate::handler::model::IdExists;
use crate::handler::polls::model::{PollCreateData, PollData};

pub mod model;

pub type PollResponse = IdExists<PollData>;

document_handler!(poll, "poll", PollResponse, PollCreateData, PollCreateData);
document_handler_all!(polls, "polls", PollResponse);