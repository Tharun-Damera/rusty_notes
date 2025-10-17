use axum::{
    Router,
    routing::{get, post},
};

mod models;
mod routes;

use crate::routes::{create_note, get_note};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/notes", post(create_note))
        .route("/notes/{note_id}", get(get_note));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
