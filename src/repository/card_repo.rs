use crate::error::AppError;
use crate::models::card::{Card, CreateCardDto};
use chrono::Utc;
use sqlx::PgPool;

pub struct CardRepository;

/// 统一的字段查询 SQL 片段（包含 JOIN 逻辑以获取 category_name）
const BASE_SELECT_SQL: &str = "
    SELECT 
        c.id, c.user_id, c.category_id, cat.name as category_name, 
        c.title, c.essence, c.insights, c.difficulty, c.importance, 
        c.interval_days, c.card_type, c.next_review_date, c.created_at 
    FROM cards c
    LEFT JOIN categories cat ON c.category_id = cat.id";

impl CardRepository {
    /// 通过科目 ID 获取该用户的所有卡片（用于科目全局复习和 PDF 导出）
    pub async fn fetch_by_subject(
        pool: &PgPool,
        subject_id: i32,
        user_id: i32,
    ) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "{} WHERE cat.subject_id = $1 AND c.user_id = $2 
             ORDER BY cat.sort_order ASC, c.created_at ASC",
            BASE_SELECT_SQL
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
            "{} WHERE c.category_id = $1 AND c.user_id = $2 
             ORDER BY c.created_at DESC",
            BASE_SELECT_SQL
        );
        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(cat_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 获取该用户单张卡片的详情
    pub async fn find_by_id(pool: &PgPool, id: i32, user_id: i32) -> Result<Card, AppError> {
        let sql = format!("{} WHERE c.id = $1 AND c.user_id = $2", BASE_SELECT_SQL);
        let result = sqlx::query_as::<_, Card>(&sql)
            .bind(id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        result.ok_or(AppError::NotFound)
    }

    /// 全文搜索该用户的卡片内容
    pub async fn search(pool: &PgPool, keyword: &str, user_id: i32) -> Result<Vec<Card>, AppError> {
        let sql = format!(
            "{} WHERE c.search_vector @@ plainto_tsquery('simple', $1) 
             AND c.user_id = $2
             ORDER BY c.importance ASC, c.next_review_date ASC 
             LIMIT 100",
            BASE_SELECT_SQL
        );
        let cards = sqlx::query_as::<_, Card>(&sql)
            .bind(keyword)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(cards)
    }

    /// 创建新卡片并关联用户（即时返回包含章节名的完整对象）
    pub async fn create(pool: &PgPool, user_id: i32, dto: CreateCardDto) -> Result<Card, AppError> {
        let sql = "
            INSERT INTO cards (user_id, category_id, title, essence, insights, difficulty, importance, card_type) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
            RETURNING id, user_id, category_id, title, essence, insights, difficulty, importance, 
                      interval_days, card_type, next_review_date, created_at,
                      (SELECT name FROM categories WHERE id = $2) as category_name";

        let card = sqlx::query_as::<_, Card>(sql)
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

    /// 更新卡片详细内容（支持编辑，即时刷新章节名）
    pub async fn update(
        pool: &PgPool,
        id: i32,
        user_id: i32,
        dto: CreateCardDto,
    ) -> Result<Card, AppError> {
        let sql = "
            UPDATE cards 
            SET title = $1, essence = $2, insights = $3, difficulty = $4, importance = $5, card_type = $6, category_id = $7
            WHERE id = $8 AND user_id = $9
            RETURNING id, user_id, category_id, title, essence, insights, difficulty, importance, 
                      interval_days, card_type, next_review_date, created_at,
                      (SELECT name FROM categories WHERE id = category_id) as category_name";

        let card = sqlx::query_as::<_, Card>(sql)
            .bind(dto.title)
            .bind(dto.essence)
            .bind(dto.insights)
            .bind(dto.difficulty)
            .bind(dto.importance)
            .bind(dto.card_type.unwrap_or_else(|| "qa".to_string()))
            .bind(dto.category_id)
            .bind(id)
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(card)
    }

    /// 更新复习进度
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

    /// 删除卡片
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