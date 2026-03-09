use sqlx::PgPool;
use crate::models::card::{Card, CreateCardDto};
use crate::error::AppError;
use chrono::Utc;

pub struct CardRepository;

impl CardRepository {
    pub async fn fetch_by_category(pool: &PgPool, cat_id: i32) -> Result<Vec<Card>, AppError> {
        let sql = "
            SELECT id, category_id, title, essence, insights, difficulty, importance, interval_days, next_review_date, created_at 
            FROM cards WHERE category_id = $1 ORDER BY created_at DESC";
        let cards = sqlx::query_as::<_, Card>(sql).bind(cat_id).fetch_all(pool).await?;
        Ok(cards)
    }

        pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Card, AppError> {
        let sql = "SELECT * FROM cards WHERE id = $1";
        let result = sqlx::query_as::<_, Card>(sql)
            .bind(id)
            .fetch_optional(pool) // 使用 fetch_optional 而不是 fetch_one
            .await?;

        // 如果没有找到结果，主动触发 NotFound 错误
        match result {
            Some(card) => Ok(card),
            None => Err(AppError::NotFound), 
        }
    }

    pub async fn search(pool: &PgPool, keyword: &str) -> Result<Vec<Card>, AppError> {
        // 使用 plainto_tsquery 优化搜索，增加 100 条限制防止数据过载
        let sql = "
            SELECT id, category_id, title, essence, insights, difficulty, importance, interval_days, next_review_date, created_at 
            FROM cards 
            WHERE search_vector @@ plainto_tsquery('simple', $1) 
            ORDER BY importance ASC, next_review_date ASC 
            LIMIT 100";
        let cards = sqlx::query_as::<_, Card>(sql).bind(keyword).fetch_all(pool).await?;
        Ok(cards)
    }

    pub async fn update_review(pool: &PgPool, card_id: i32, days: i32) -> Result<(), AppError> {
        let next_date = Utc::now().date_naive() + chrono::Duration::days(days as i64);
        sqlx::query(
            "UPDATE cards SET interval_days = $1, next_review_date = $2, last_review_date = CURRENT_DATE, review_count = review_count + 1 WHERE id = $3"
        )
        .bind(days)
        .bind(next_date)
        .bind(card_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn create(pool: &PgPool, dto: CreateCardDto) -> Result<Card, AppError> {
        let sql = "
            INSERT INTO cards (category_id, title, essence, insights, difficulty, importance) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id, category_id, title, essence, insights, difficulty, importance, interval_days, next_review_date, created_at";
        let card = sqlx::query_as::<_, Card>(sql)
            .bind(dto.category_id)
            .bind(dto.title)
            .bind(dto.essence)
            .bind(dto.insights)
            .bind(dto.difficulty)
            .bind(dto.importance)
            .fetch_one(pool)
            .await?;
        Ok(card)
    }
}