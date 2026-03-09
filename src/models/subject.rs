use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub theme_color: Option<String>,
    pub icon_type: Option<String>,   // 新增字段
    pub visibility: Option<String>,  // 新增字段
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubjectDto {
    pub name: String,
    pub description: Option<String>,
    pub theme_color: Option<String>,
    pub icon_type: Option<String>,   // 报错点：之前这里漏掉了
    pub visibility: Option<String>,  // 报错点：之前这里漏掉了
}