use crate::error::AppError;
use crate::models::category::{Category, CreateCategoryDto};
use sqlx::PgPool;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn is_owner(pool: &sqlx::PgPool, subject_id: i32, user_id: i32) -> bool {
        let result = sqlx::query!(
            "SELECT id FROM subjects WHERE id = $1 AND user_id = $2",
            subject_id,
            user_id
        )
        .fetch_optional(pool)
        .await;
        result.is_ok() && result.unwrap().is_some()
    }
    pub async fn find_by_subject(
        pool: &PgPool,
        subject_id: i32,
        user_id: i32,
    ) -> Result<Vec<Category>, AppError> {
        let sql = "
            SELECT c.id, c.subject_id, c.parent_id, c.name, c.sort_order 
            FROM categories c
            JOIN subjects s ON c.subject_id = s.id
            WHERE c.subject_id = $1 AND s.user_id = $2
            ORDER BY c.sort_order ASC";

        let categories = sqlx::query_as::<_, Category>(sql)
            .bind(subject_id)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(categories)
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        user_id: i32,
        dto: CreateCategoryDto,
    ) -> Result<Category, AppError> {
        // 1. 强制检查科目所属权
        if !Self::is_owner(pool, dto.subject_id, user_id).await {
            return Err(AppError::BadRequest("无权操作此科目".into()));
        }

        let category = sqlx::query_as::<_, Category>(
            "INSERT INTO categories (subject_id, parent_id, name, sort_order) 
             VALUES ($1, $2, $3, $4) RETURNING id, subject_id, parent_id, name, sort_order",
        )
        .bind(dto.subject_id)
        .bind(dto.parent_id)
        .bind(dto.name)
        .bind(dto.sort_order.unwrap_or(0))
        .fetch_one(pool)
        .await?;
        Ok(category)
    }
    pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<(), AppError> {
        let sql = "
            DELETE FROM categories 
            WHERE id = $1 AND subject_id IN (SELECT id FROM subjects WHERE user_id = $2)";
        let result = sqlx::query(sql)
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
