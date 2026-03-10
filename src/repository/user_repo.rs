use crate::error::AppError;
use crate::models::user::{User, RegisterRequest};
use sqlx::PgPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, req: RegisterRequest, hash: String) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(req.username)
        .bind(hash)
        .bind(req.email)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }
}