use crate::handler::model::IdExists;
use crate::handler::automated_timers::model::{AutomatedTimerCreateData, AutomatedTimerData};

pub mod model;

pub type AutomatedTimerResponse = IdExists<AutomatedTimerData>;

document_handler!(automated_timer, "timer/automated", AutomatedTimerResponse, AutomatedTimerCreateData, AutomatedTimerCreateData);
document_handler_all!(automated_timers, "timers/automated", AutomatedTimerResponse);