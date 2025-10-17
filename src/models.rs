use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}
