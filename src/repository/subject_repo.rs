use sqlx::PgPool;
use crate::models::subject::{Subject, CreateSubjectDto};
use crate::error::AppError;

pub struct SubjectRepository;

impl SubjectRepository {
    pub async fn fetch_all(pool: &PgPool) -> Result<Vec<Subject>, AppError> {
        // 使用 query_as 函数代替 query_as! 宏
        let subjects = sqlx::query_as::<_, Subject>("SELECT id, name, description, theme_color, created_at FROM subjects ORDER BY id DESC")
            .fetch_all(pool)
            .await?;
        Ok(subjects)
    }

    pub async fn create(pool: &PgPool, dto: CreateSubjectDto) -> Result<Subject, AppError> {
        // 使用 query_as 函数并手动 bind 参数
        let subject = sqlx::query_as::<_, Subject>(
            "INSERT INTO subjects (name, description, theme_color) VALUES ($1, $2, $3) RETURNING id, name, description, theme_color, created_at"
        )
        .bind(dto.name)        
        .bind(dto.description) 
        .bind(dto.theme_color) 
        .fetch_one(pool)
        .await?;
        Ok(subject)
    }
}