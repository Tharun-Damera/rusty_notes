mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = db::init().await?;
    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
