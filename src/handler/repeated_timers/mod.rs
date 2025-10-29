use crate::handler::model::IdExists;
use crate::handler::repeated_timers::model::{RepeatedTimerCreateData, RepeatedTimerData};

pub mod model;

pub type RepeatedTimerResponse = IdExists<RepeatedTimerData>;

document_handler!(repeated_timer, "timer/repeated", RepeatedTimerResponse, RepeatedTimerCreateData, RepeatedTimerCreateData);
document_handler_all!(repeated_timers, "timers/repeated", RepeatedTimerResponse);