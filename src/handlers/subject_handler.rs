use crate::AppState;
use crate::error::AppError;
use crate::handlers::auth_handler::AuthUser; // 必须导入提取器
use crate::models::subject::{CreateSubjectDto, Subject};
use crate::repository::subject_repo::SubjectRepository;
use axum::{extract::{State, Path}, Json, http::StatusCode};
use tracing::{info, instrument};
use validator::Validate;

#[instrument(skip(state, user_id))]
pub async fn list_subjects(
    AuthUser(user_id): AuthUser, // 1. 提取当前用户ID
    State(state): State<AppState>,
) -> Result<Json<Vec<Subject>>, AppError> {
    // 2. 传递 user_id 给 Repo
    let subjects = SubjectRepository::fetch_all(&state.db, user_id).await?;
    Ok(Json(subjects))
}

#[instrument(skip(state, user_id))]
pub async fn create_subject(
    AuthUser(user_id): AuthUser, // 1. 提取当前用户ID
    State(state): State<AppState>,
    Json(payload): Json<CreateSubjectDto>,
) -> Result<(StatusCode, Json<Subject>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    // 2. 传递 user_id 给 Repo
    let subject = SubjectRepository::create(&state.db, user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(subject)))
}

#[instrument(skip(state, user_id))]
pub async fn delete_subject(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    SubjectRepository::delete(&state.db, id, user_id).await?;
    info!("🗑️ 用户 {} 删除了科目 {}", user_id, id);
    Ok(StatusCode::NO_CONTENT)
}
