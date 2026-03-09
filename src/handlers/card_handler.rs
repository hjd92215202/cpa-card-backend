use axum::{extract::{State, Path, Query}, Json, http::StatusCode};
use crate::AppState;
use crate::models::card::{Card, CreateCardDto, ReviewCardDto};
use crate::repository::card_repo::CardRepository;
use crate::error::AppError;
use serde::Deserialize;
use tracing::{info, warn, instrument};

// 必须实现 Debug 才能被 #[instrument] 记录
#[derive(Deserialize, Debug)]
pub struct CardSearchQuery {
    pub keyword: Option<String>,
    pub category_id: Option<i32>,
}

#[instrument(skip(state))]
pub async fn create_card(
    State(state): State<AppState>,
    Json(payload): Json<CreateCardDto>,
) -> Result<(StatusCode, Json<Card>), AppError> {
    if payload.title.is_empty() {
        return Err(AppError::BadRequest("卡片标题不能为空".into()));
    }
    let card = CardRepository::create(&state.db, payload).await?;
    info!("✅ 成功创建卡片: {}", card.title);
    Ok((StatusCode::CREATED, Json(card)))
}

#[instrument(skip(state))]
pub async fn list_cards(
    State(state): State<AppState>,
    Query(params): Query<CardSearchQuery>,
) -> Result<Json<Vec<Card>>, AppError> {
    // 如果传了 category_id，则按分类查；否则返回空或报错
    let cat_id = params.category_id.ok_or_else(|| AppError::BadRequest("必须提供章节ID".into()))?;
    let cards = CardRepository::fetch_by_category(&state.db, cat_id).await?;
    info!("📂 获取章节 {} 下的卡片，数量: {}", cat_id, cards.len());
    Ok(Json(cards))
}

#[instrument(skip(state))]
pub async fn search_cards(
    State(state): State<AppState>,
    Query(params): Query<CardSearchQuery>,
) -> Result<Json<Vec<Card>>, AppError> {
    let keyword = params.keyword.as_deref().unwrap_or_default();
    if keyword.is_empty() {
        return Ok(Json(vec![]));
    }
    let cards = CardRepository::search(&state.db, keyword).await?;
    info!("🔎 搜索完成，关键词: '{}', 结果数: {}", keyword, cards.len());
    Ok(Json(cards))
}

#[instrument(skip(state))]
pub async fn get_card_detail(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Card>, AppError> {
    let card = CardRepository::find_by_id(&state.db, id).await?;
    Ok(Json(card))
}

#[instrument(skip(state))]
pub async fn review_card(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ReviewCardDto>,
) -> Result<StatusCode, AppError> {
    if payload.interval_days < 0 {
        warn!("⚠️ 收到非法的复习间隔: {}", payload.interval_days);
        return Err(AppError::BadRequest("复习间隔不能为负数".into()));
    }
    CardRepository::update_review(&state.db, id, payload.interval_days).await?;
    info!("📅 卡片 {} 复习进度已更新", id);
    Ok(StatusCode::OK)
}