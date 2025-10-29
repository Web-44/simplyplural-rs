use crate::handler::notes::model::{NoteCreateData, NoteData, NoteUpdateData};
use crate::handler::model::IdExists;

pub mod model;

pub type NoteResponse = IdExists<NoteData>;

document_handler!(note, "note", NoteResponse, NoteCreateData, NoteUpdateData);
document_handler_for!(notes, "notes", member, NoteResponse);