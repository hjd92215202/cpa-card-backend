use serde::{Deserialize, Serialize};
use sqlx::FromRow; // 必须引入这个 Trait
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)] // 增加 FromRow
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub theme_color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubjectDto {
    pub name: String,
    pub description: Option<String>,
    pub theme_color: Option<String>,
}