use crate::error::AppError;
use crate::models::subject::{CreateSubjectDto, Subject};
use sqlx::PgPool;

pub struct SubjectRepository;

impl SubjectRepository {
    pub async fn fetch_all(pool: &PgPool, user_id: i32) -> Result<Vec<Subject>, AppError> {
        let subjects = sqlx::query_as::<_, Subject>(
            "SELECT id, name, description, theme_color, icon_type, visibility, created_at 
             FROM subjects WHERE user_id = $1 ORDER BY id DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(subjects)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        dto: CreateSubjectDto,
    ) -> Result<Subject, AppError> {
        let subject = sqlx::query_as::<_, Subject>(
            "INSERT INTO subjects (name, description, theme_color, icon_type, visibility, user_id) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING id, name, description, theme_color, icon_type, visibility, created_at",
        )
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.theme_color)
        .bind(dto.icon_type.unwrap_or_else(|| "Book".to_string()))
        .bind(dto.visibility.unwrap_or_else(|| "private".to_string()))
        .bind(user_id) // 绑定用户 ID
        .fetch_one(pool)
        .await?;
        Ok(subject)
    }

    pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM subjects WHERE id = $1 AND user_id = $2")
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
