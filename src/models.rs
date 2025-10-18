use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgQueryResult};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: Utc::now(),
        }
    }
    pub async fn save(&self, pool: &PgPool) -> anyhow::Result<PgQueryResult> {
        let res = sqlx::query!(
            "
            INSERT INTO notes (id, title, content, created_at) 
            VALUES ($1, $2, $3, $4);
            ",
            self.id,
            self.title,
            self.content,
            self.created_at,
        )
        .execute(pool)
        .await?;

        println!("res: {res:?}");
        Ok(res)
    }

    pub async fn get_notes(pool: &PgPool) -> anyhow::Result<Vec<Note>> {
        let res = sqlx::query_as("SELECT * FROM notes;")
            .fetch_all(pool)
            .await?;

        println!("get_notes: {res:?}");
        Ok(res)
    }

    // pub async fn get_note(pool: &PgPool, note_id: Uuid) -> anyhow::Result<Note> {
    //     let res = sqlx::query_as("SELECT * FROM notes WHERE id=$1;")
    //         .bind(note_id)
    //         .fetch_one(pool)
    //         .await?;

    //     Ok(res)
    // }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}
