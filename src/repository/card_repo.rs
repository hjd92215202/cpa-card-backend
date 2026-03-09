use crate::error::AppError;
use crate::models::card::{Card, CreateCardDto};
use chrono::Utc;
use sqlx::PgPool;

pub struct CardRepository;

// 定义统一的字段查询列表，避免在多个方法中重复编写且容易漏掉字段
const CARD_COLUMNS: &str = "id, category_id, title, essence, insights, difficulty, importance, interval_days, card_type, next_review_date, created_at";

impl CardRepository {
    /// 通过科目 ID 获取该科目下所有卡片（用于全科目复习模式）
    pub async fn fetch_by_subject(pool: &PgPool, subject_id: i32) -> Result<Vec<Card>, AppError> {
        // 明确指定 c.id 以免和 category 的 id 冲突
        // 确保包含你新增的 card_type 字段
        let sql = "
        SELECT 
            c.id, c.category_id, c.title, c.essence, c.insights, 
            c.difficulty, c.importance, c.interval_days, c.card_type,
            c.next_review_date, c.created_at 
        FROM cards c
        JOIN categories cat ON c.category_id = cat.id
        WHERE cat.subject_id = $1
        ORDER BY c.next_review_date ASC, c.importance ASC";

        let cards = sqlx::query_as::<_, Card>(sql)
            .bind(subject_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 通过章节 ID 获取卡片列表
    pub async fn fetch_by_category(pool: &PgPool, cat_id: i32) -> Result<Vec<Card>, AppError> {
        // 同样，这里的 SELECT * 可能会因为字段顺序或缺失 card_type 报错
        // 建议显式列出所有字段
        let sql = "
        SELECT id, category_id, title, essence, insights, difficulty, importance, interval_days, card_type, next_review_date, created_at 
        FROM cards 
        WHERE category_id = $1 
        ORDER BY created_at DESC";
        let cards = sqlx::query_as::<_, Card>(sql)
            .bind(cat_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 获取单张卡片详情
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Card, AppError> {
        let sql = format!("SELECT {} FROM cards WHERE id = $1", CARD_COLUMNS);
        let result = sqlx::query_as::<_, Card>(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(card) => Ok(card),
            None => Err(AppError::NotFound),
        }
    }

    /// 全文搜索卡片内容
    pub async fn search(pool: &PgPool, keyword: &str) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "SELECT {} FROM cards 
             WHERE search_vector @@ plainto_tsquery('simple', $1) 
             ORDER BY importance ASC, next_review_date ASC 
             LIMIT 100",
            CARD_COLUMNS
        );
        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(keyword)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 更新复习进度
    pub async fn update_review(pool: &PgPool, card_id: i32, days: i32) -> Result<(), AppError> {
        // 计算下一次复习日期
        let next_date = Utc::now().date_naive() + chrono::Duration::days(days as i64);

        sqlx::query(
            "UPDATE cards 
             SET interval_days = $1, 
                 next_review_date = $2, 
                 last_review_date = CURRENT_DATE, 
                 review_count = review_count + 1 
             WHERE id = $3",
        )
        .bind(days)
        .bind(next_date)
        .bind(card_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// 创建新卡片
    pub async fn create(pool: &PgPool, dto: CreateCardDto) -> Result<Card, AppError> {
        let sql = format!(
            "INSERT INTO cards (category_id, title, essence, insights, difficulty, importance, card_type) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING {}",
            CARD_COLUMNS
        );

        let card = sqlx::query_as::<_, Card>(&sql)
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
}
