use sqlx::PgPool;
use crate::models::subject::{Subject, CreateSubjectDto};
use crate::error::AppError;

pub struct SubjectRepository;

impl SubjectRepository {
    pub async fn fetch_all(pool: &PgPool) -> Result<Vec<Subject>, AppError> {
        let subjects = sqlx::query_as::<_, Subject>(
            "SELECT id, name, description, theme_color, icon_type, visibility, created_at FROM subjects ORDER BY id DESC"
        )
        .fetch_all(pool)
        .await?;
        Ok(subjects)
    }

    pub async fn create(pool: &PgPool, dto: CreateSubjectDto) -> Result<Subject, AppError> {
        let subject = sqlx::query_as::<_, Subject>(
            "INSERT INTO subjects (name, description, theme_color, icon_type, visibility) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, name, description, theme_color, icon_type, visibility, created_at"
        )
        .bind(dto.name)        
        .bind(dto.description) 
        .bind(dto.theme_color) 
        .bind(dto.icon_type.unwrap_or_else(|| "Book".to_string())) 
        .bind(dto.visibility.unwrap_or_else(|| "private".to_string()))
        .fetch_one(pool)
        .await?;
        Ok(subject)
    }
}