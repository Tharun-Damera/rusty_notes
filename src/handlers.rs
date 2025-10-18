use axum::{
    debug_handler,
    extract::{Json, Path},
};
use uuid::Uuid;

use crate::models::{CreateNote, Note};

#[debug_handler]
pub async fn create_note(Json(payload): Json<CreateNote>) -> Json<Note> {
    let note = Note::new(payload.title, payload.content);
    println!("Called create_note handler| note: {:?}", note);
    Json::from(note)
}

#[debug_handler]
pub async fn get_note(Path(note_id): Path<Uuid>) -> Json<Option<Note>> {
    let note = if note_id == Uuid::parse_str("74517d89-f682-463b-86d1-3c129acb0b27").unwrap() {
        Some(Note::new(
            "Rust is great!".to_string(),
            "Axum is a rust web framework".to_string(),
        ))
    } else {
        None
    };
    Json::from(note)
}
