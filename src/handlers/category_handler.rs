use axum::{extract::{State, Query, Path}, Json, http::StatusCode};
use crate::AppState;
use crate::models::category::{Category, CreateCategoryDto};
use crate::repository::category_repo::CategoryRepository;
use crate::error::AppError;
use crate::handlers::auth_handler::AuthUser;
use serde::Deserialize;
use tracing::{info, instrument};
use validator::Validate;

/// 章节查询参数
#[derive(Deserialize, Debug)]
pub struct CategoryQuery {
    pub subject_id: Option<i32>,
}

/// 获取某个学科下的章节列表 (带用户隔离)
#[instrument(skip(state, user_id))]
pub async fn list_by_subject(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<Vec<Category>>, AppError> {
    let subject_id = params.subject_id.ok_or_else(|| AppError::BadRequest("必须提供学科ID以加载目录".into()))?;
    
    // 调用仓库层，内部已实现 JOIN subjects 校验所有权
    let categories = CategoryRepository::find_by_subject(&state.db, subject_id, user_id).await?;
    
    info!("📂 用户 {} 加载了学科 {} 的章节列表, 共 {} 项", user_id, subject_id, categories.len());
    Ok(Json(categories))
}

/// 创建新章节 (增加所有权校验)
#[instrument(skip(state, user_id))]
pub async fn create_category(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>,
) -> Result<(StatusCode, Json<Category>), AppError> {
    // 1. DTO 合法性校验 (由 validator 处理)
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // 2. 执行创建 (Repo 内部会校验 payload.subject_id 是否属于该 user_id)
    let category = CategoryRepository::create(&state.db, user_id, payload).await?;
    
    info!("➕ 用户 {} 在科目 {} 下创建了新章节: {}", user_id, category.subject_id, category.name);
    Ok((StatusCode::CREATED, Json(category)))
}

/// 删除章节 (带本人所有权校验)
#[instrument(skip(state, user_id))]
pub async fn delete_category(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    // 调用 Repo，内部通过子查询确保该 category 属于该用户拥有的科目
    CategoryRepository::delete(&state.db, id, user_id).await?;
    
    info!("🗑️ 用户 {} 删除了章节节点 ID: {}", user_id, id);
    Ok(StatusCode::NO_CONTENT)
}