use axum::{
    Router,
    routing::{get, post, delete},
};
use sqlx::PgPool;

use crate::handlers::*;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/notes", post(create_note).get(get_notes))
        .route("/notes/{id}", get(get_note).put(update_note).delete(delete_note))
        .with_state(pool)
}
