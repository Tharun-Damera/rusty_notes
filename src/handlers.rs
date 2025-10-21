use anyhow::Result;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Note, NotePayload},
};

pub async fn create_note(
    State(pool): State<PgPool>,
    Json(payload): Json<NotePayload>,
) -> Result<impl IntoResponse, AppError> {
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (id, title, content, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *;",
    )
    .bind(Uuid::new_v4())
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get_notes(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes;")
        .fetch_all(&pool)
        .await?;

    Ok((StatusCode::OK, Json(notes)))
}

pub async fn get_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let maybe_note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id=$1;")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match maybe_note {
        Some(note) => Ok((StatusCode::OK, Json(note))),
        None => Err(AppError::NotFound(format!("Note {id} not found"))),
    }
}

pub async fn update_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<NotePayload>,
) -> Result<impl IntoResponse, AppError> {
    let note =
        sqlx::query_as::<_, Note>("UPDATE notes SET title=$1, content=$2 WHERE id=$3 RETURNING *;")
            .bind(&payload.title)
            .bind(&payload.content)
            .bind(id)
            .fetch_one(&pool)
            .await?;

    Ok((StatusCode::OK, Json(note)))
}

pub async fn delete_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query("DELETE FROM notes WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await?
        .rows_affected();

    if rows == 0 {
        Err(AppError::NotFound(format!("Note {id} not found")))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
