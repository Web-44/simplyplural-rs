use crate::handler::members::model::{MemberCreateData, MemberData};
use crate::handler::model::IdExists;

pub mod model;

pub type MemberResponse = IdExists<MemberData>;

document_handler!(member, "member", MemberResponse, MemberCreateData, MemberCreateData);
document_handler_all!(members, "members", MemberResponse);