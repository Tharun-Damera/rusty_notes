use anyhow::Result;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init() -> Result<Pool<Postgres>> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}
