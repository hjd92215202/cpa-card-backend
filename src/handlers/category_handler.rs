use axum::{extract::{State, Query, Path}, Json, http::StatusCode};
use crate::AppState;
use crate::models::category::{Category, CreateCategoryDto};
use crate::repository::category_repo::CategoryRepository;
use crate::error::AppError;
use crate::handlers::auth_handler::AuthUser; // 仅从这里引入身份提取器
use serde::Deserialize;
use tracing::{info, instrument};
use validator::Validate;

// 1. 在这里定义查询参数结构体
#[derive(Deserialize, Debug)]
pub struct CategoryQuery {
    pub subject_id: Option<i32>,
}

/// 获取某个学科下的章节列表 (带用户隔离)
#[instrument(skip(state, user_id))]
pub async fn list_by_subject(
    AuthUser(user_id): AuthUser, // 从 JWT 提取用户 ID
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<Vec<Category>>, AppError> {
    let subject_id = params.subject_id.ok_or_else(|| AppError::BadRequest("未提供学科ID".into()))?;
    
    // 调用 Repo，传入 user_id 确保权限安全
    let categories = CategoryRepository::find_by_subject(&state.db, subject_id, user_id).await?;
    Ok(Json(categories))
}

/// 创建新章节
#[instrument(skip(state, user_id))]
pub async fn create_category(
    AuthUser(user_id): AuthUser, // 获取当前用户
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>,
) -> Result<(StatusCode, Json<Category>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // 传入 user_id 进行越权校验
    let category = CategoryRepository::create(&state.db, user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

#[instrument(skip(state, user_id))]
pub async fn delete_category(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    CategoryRepository::delete(&state.db, id, user_id).await?;
    info!("🗑️ 用户 {} 删除了章节 {}", user_id, id);
    Ok(StatusCode::NO_CONTENT)
}