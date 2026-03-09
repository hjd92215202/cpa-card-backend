use sqlx::PgPool;
use crate::models::category::{Category, CreateCategoryDto};
use crate::error::AppError;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn find_by_subject(pool: &PgPool, subject_id: i32) -> Result<Vec<Category>, AppError> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT id, subject_id, parent_id, name, sort_order FROM categories WHERE subject_id = $1 ORDER BY sort_order ASC"
        )
        .bind(subject_id)
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    pub async fn create(pool: &PgPool, dto: CreateCategoryDto) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            "INSERT INTO categories (subject_id, parent_id, name, sort_order) 
             VALUES ($1, $2, $3, $4) RETURNING id, subject_id, parent_id, name, sort_order"
        )
        .bind(dto.subject_id)
        .bind(dto.parent_id)
        .bind(dto.name)
        .bind(dto.sort_order.unwrap_or(0))
        .fetch_one(pool)
        .await?;
        Ok(category)
    }
}