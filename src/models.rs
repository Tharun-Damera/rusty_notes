use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: String,
    title: String,
    content: String,
    created_at: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}
