use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::handlers::*;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        // .route("/notes/{note_id}", get(get_note))
        .with_state(pool)
}
