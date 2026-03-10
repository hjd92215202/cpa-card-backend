use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: i32,
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub title: String,
    pub essence: Option<String>,
    pub insights: Option<String>,
    pub difficulty: i16,
    pub importance: String,
    pub interval_days: i32,
    pub card_type: Option<String>, // 新增字段：用于支持 问答/笔记/代码 模式
    pub next_review_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCardDto {
    pub category_id: i32,

    #[validate(length(min = 1, message = "卡片标题不能为空"))]
    pub title: String,
    
    #[validate(length(min = 1, message = "知识精华不能为空"))]
    pub essence: String,
    
    pub insights: Option<String>,
    
    #[validate(range(min = 1, max = 5, message = "难度必须在 1-5 之间"))]
    pub difficulty: i16,
    
    // 限制只能传入 A, B, C
    #[validate(custom(function = "validate_importance"))]
    pub importance: String,
    
    pub card_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewCardDto {
    pub interval_days: i32,
}

fn validate_importance(val: &str) -> Result<(), validator::ValidationError> {
    if ["A", "B", "C"].contains(&val) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("重要度只能是 A, B 或 C"))
    }
}