use axum::{
    Router,
    routing::{get, post},
};
use dotenvy;

mod db;
mod models;
mod routes;

use crate::routes::{create_note, get_note};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::init().await?;
    dbg!(pool);

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/notes", post(create_note))
        .route("/notes/{note_id}", get(get_note));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
