use crate::error::AppError;
use crate::models::card::{Card, CreateCardDto};
use chrono::Utc;
use sqlx::PgPool;

pub struct CardRepository;

// 定义统一的字段查询列表，包含新增的 user_id 和 card_type
const CARD_COLUMNS: &str = "id, user_id, category_id, title, essence, insights, difficulty, importance, interval_days, card_type, next_review_date, created_at";

impl CardRepository {
    /// 通过科目 ID 获取该科目下属于该用户的所有卡片
    pub async fn fetch_by_subject(
        pool: &PgPool,
        subject_id: i32,
        user_id: i32,
    ) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "SELECT c.id, c.user_id, c.category_id, c.title, c.essence, c.insights, 
                    c.difficulty, c.importance, c.interval_days, c.card_type,
                    c.next_review_date, c.created_at 
             FROM cards c
             JOIN categories cat ON c.category_id = cat.id
             WHERE cat.subject_id = $1 AND c.user_id = $2
             ORDER BY c.next_review_date ASC, c.importance ASC"
        );

        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(subject_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 通过章节 ID 获取该用户的卡片列表
    pub async fn fetch_by_category(
        pool: &PgPool,
        cat_id: i32,
        user_id: i32,
    ) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "SELECT {} FROM cards 
             WHERE category_id = $1 AND user_id = $2 
             ORDER BY created_at DESC",
            CARD_COLUMNS
        );
        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(cat_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 获取该用户单张卡片的详情（确保越权防护）
    pub async fn find_by_id(pool: &PgPool, id: i32, user_id: i32) -> Result<Card, AppError> {
        let sql = format!(
            "SELECT {} FROM cards WHERE id = $1 AND user_id = $2",
            CARD_COLUMNS
        );
        let result = sqlx::query_as::<_, Card>(&sql)
            .bind(id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(card) => Ok(card),
            None => Err(AppError::NotFound),
        }
    }

    /// 全文搜索该用户的卡片内容
    pub async fn search(pool: &PgPool, keyword: &str, user_id: i32) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "SELECT {} FROM cards 
             WHERE search_vector @@ plainto_tsquery('simple', $1) 
             AND user_id = $2
             ORDER BY importance ASC, next_review_date ASC 
             LIMIT 100",
            CARD_COLUMNS
        );
        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(keyword)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 更新复习进度（仅限本人卡片）
    pub async fn update_review(
        pool: &PgPool,
        card_id: i32,
        user_id: i32,
        days: i32,
    ) -> Result<(), AppError> {
        let next_date = Utc::now().date_naive() + chrono::Duration::days(days as i64);

        let result = sqlx::query(
            "UPDATE cards 
             SET interval_days = $1, 
                 next_review_date = $2, 
                 last_review_date = CURRENT_DATE, 
                 review_count = review_count + 1 
             WHERE id = $3 AND user_id = $4",
        )
        .bind(days)
        .bind(next_date)
        .bind(card_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    /// 更新卡片详细内容（编辑功能实现）
    pub async fn update(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        dto: CreateCardDto,
    ) -> Result<Card, AppError> {
        let sql = format!(
            "UPDATE cards 
             SET title = $1, essence = $2, insights = $3, difficulty = $4, importance = $5, card_type = $6
             WHERE id = $7 AND user_id = $8
             RETURNING {}",
            CARD_COLUMNS
        );

        let card = sqlx::query_as::<_, Card>(&sql)
            .bind(dto.title)
            .bind(dto.essence)
            .bind(dto.insights)
            .bind(dto.difficulty)
            .bind(dto.importance)
            .bind(dto.card_type.unwrap_or_else(|| "qa".to_string()))
            .bind(id)
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(card)
    }

    /// 创建新卡片并关联用户
    pub async fn create(pool: &PgPool, user_id: i32, dto: CreateCardDto) -> Result<Card, AppError> {
        let sql = format!(
            "INSERT INTO cards (user_id, category_id, title, essence, insights, difficulty, importance, card_type) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING {}",
            CARD_COLUMNS
        );

        let card = sqlx::query_as::<_, Card>(&sql)
            .bind(user_id)
            .bind(dto.category_id)
            .bind(dto.title)
            .bind(dto.essence)
            .bind(dto.insights)
            .bind(dto.difficulty)
            .bind(dto.importance)
            .bind(dto.card_type.unwrap_or_else(|| "qa".to_string()))
            .fetch_one(pool)
            .await?;
        Ok(card)
    }

    pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM cards WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}
