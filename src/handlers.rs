use axum::extract::{Json, State};
use sqlx::PgPool;

use crate::models::{CreateNote, Note};

pub async fn create_note(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNote>,
) -> Json<Note> {
    let note = Note::new(payload.title, payload.content);
    note.save(&pool).await.unwrap();
    Json::from(note)
}

pub async fn get_notes(State(pool): State<PgPool>) -> Json<Vec<Note>> {
    Json(Note::get_notes(&pool).await.unwrap())
}
