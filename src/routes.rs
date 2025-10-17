use axum::{debug_handler, extract::Json};

use crate::models::{CreateNote, Note};

#[debug_handler]
pub async fn create_note(Json(payload): Json<CreateNote>) -> Json<Note> {
    let note = Note::new(payload.title, payload.content);
    println!("Called create_note handler| note: {:?}", note);
    Json::from(note)
}
