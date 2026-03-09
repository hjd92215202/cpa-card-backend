// src/handlers/subject_handler.rs
use axum::{extract::State, Json, http::StatusCode};
use crate::AppState;
use crate::models::subject::{Subject, CreateSubjectDto};
use crate::repository::subject_repo::SubjectRepository;
use crate::error::AppError;

pub async fn list_subjects(
    State(state): State<AppState>,
) -> Result<Json<Vec<Subject>>, AppError> {
    let subjects = SubjectRepository::fetch_all(&state.db).await?;
    Ok(Json(subjects))
}

pub async fn create_subject(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubjectDto>,
) -> Result<(StatusCode, Json<Subject>), AppError> {
    let subject = SubjectRepository::create(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(subject)))
}