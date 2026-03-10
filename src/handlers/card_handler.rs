use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use crate::AppState;
use crate::models::card::{Card, CreateCardDto, ReviewCardDto};
use crate::repository::card_repo::CardRepository;
use crate::error::AppError;
// 确保 auth_handler 已正确定义 AuthUser 提取器
use crate::handlers::auth_handler::AuthUser; 
use serde::Deserialize;
use tracing::{info, warn, instrument};
use validator::Validate;

// 定义查询参数结构体并派生 Debug，以供 tracing 使用
#[derive(Deserialize, Debug)]
pub struct CardSearchQuery {
    pub keyword: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct SubjectQuery {
    pub subject_id: i32,
}

/// 创建卡片 (已关联用户)
#[instrument(skip(state, user_id))]
pub async fn create_card(
    AuthUser(user_id): AuthUser, 
    State(state): State<AppState>,
    Json(payload): Json<CreateCardDto>,
) -> Result<(StatusCode, Json<Card>), AppError> {

    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let card = CardRepository::create(&state.db, user_id, payload).await?;
    info!("✅ 用户 {} 成功创建卡片 ID: {}, 标题: {}", user_id, card.id, card.title);
    Ok((StatusCode::CREATED, Json(card)))
}

/// 更新/编辑卡片内容 (仅限本人所有)
#[instrument(skip(state, user_id))]
pub async fn update_card(
    AuthUser(user_id): AuthUser,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<CreateCardDto>,
) -> Result<Json<Card>, AppError> {

    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let card = CardRepository::update(&state.db, id, user_id, payload).await?;
    info!("📝 用户 {} 更新了卡片 ID: {}", user_id, id);
    Ok(Json(card))
}

/// 获取章节下的卡片列表 (仅限本人的卡片)
#[instrument(skip(state, user_id))]
pub async fn list_cards(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Query(params): Query<CardSearchQuery>,
) -> Result<Json<Vec<Card>>, AppError> {
    let cat_id = params.category_id.ok_or_else(|| AppError::BadRequest("必须提供章节ID".into()))?;
    let cards = CardRepository::fetch_by_category(&state.db, cat_id, user_id).await?;
    info!("📂 用户 {} 获取章节 {} 下的卡片, 数量: {}", user_id, cat_id, cards.len());
    Ok(Json(cards))
}

/// 全文搜索 (仅在本人卡片内搜索)
#[instrument(skip(state, user_id))]
pub async fn search_cards(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Query(params): Query<CardSearchQuery>,
) -> Result<Json<Vec<Card>>, AppError> {
    let keyword = params.keyword.as_deref().unwrap_or_default();
    if keyword.trim().is_empty() {
        return Ok(Json(vec![]));
    }
    let cards = CardRepository::search(&state.db, keyword, user_id).await?;
    info!("🔎 用户 {} 搜索完成, 关键词: '{}', 结果数: {}", user_id, keyword, cards.len());
    Ok(Json(cards))
}

/// 获取卡片详情
#[instrument(skip(state, user_id))]
pub async fn get_card_detail(
    AuthUser(user_id): AuthUser,
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Card>, AppError> {
    let card = CardRepository::find_by_id(&state.db, id, user_id).await?;
    Ok(Json(card))
}

/// 记录复习进度 (更新下一次复习时间)
#[instrument(skip(state, user_id))]
pub async fn review_card(
    AuthUser(user_id): AuthUser,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<ReviewCardDto>,
) -> Result<StatusCode, AppError> {
    if payload.interval_days < 0 {
        warn!("⚠️ 用户 {} 提交了非法的复习间隔: {}", user_id, payload.interval_days);
        return Err(AppError::BadRequest("复习间隔不能为负数".into()));
    }
    
    CardRepository::update_review(&state.db, id, user_id, payload.interval_days).await?;
    info!("📅 用户 {} 更新了卡片 {} 的复习进度", user_id, id);
    Ok(StatusCode::OK)
}

/// 按科目加载全量复习卡片 (用于首页一键进入复习)
#[instrument(skip(state, user_id))]
pub async fn list_cards_by_subject(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Query(params): Query<SubjectQuery>,
) -> Result<Json<Vec<Card>>, AppError> {
    info!("🚀 用户 {} 开始加载科目 {} 的全量卡片", user_id, params.subject_id);
    let cards = CardRepository::fetch_by_subject(&state.db, params.subject_id, user_id).await?;
    Ok(Json(cards))
}

#[instrument(skip(state, user_id))]
pub async fn delete_card(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    CardRepository::delete(&state.db, id, user_id).await?;
    info!("🗑️ 用户 {} 删除了卡片 {}", user_id, id);
    Ok(StatusCode::NO_CONTENT)
}