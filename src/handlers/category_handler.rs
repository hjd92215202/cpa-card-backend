use axum::{extract::{State, Query}, Json, http::StatusCode};
use crate::AppState;
use crate::models::category::{Category, CreateCategoryDto};
use crate::repository::category_repo::CategoryRepository;
use crate::error::AppError;
use serde::Deserialize;
use tracing::instrument;

// 必须实现 Debug 才能被 #[instrument] 记录
#[derive(Deserialize, Debug)]
pub struct CategoryQuery {
    pub subject_id: Option<i32>,
}

#[instrument(skip(state))]
pub async fn list_by_subject(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<Vec<Category>>, AppError> {
    let subject_id = params.subject_id.ok_or_else(|| AppError::BadRequest("未提供学科ID".into()))?;
    let categories = CategoryRepository::find_by_subject(&state.db, subject_id).await?;
    Ok(Json(categories))
}

#[instrument(skip(state))]
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>,
) -> Result<(StatusCode, Json<Category>), AppError> {
    let category = CategoryRepository::create(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(category)))
}