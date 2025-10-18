use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::{create_note, get_note};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/notes/{note_id}", get(get_note))
        .route("/notes", post(create_note))
        .with_state(pool)
}
