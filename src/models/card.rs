use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: i32,
    pub category_id: Option<i32>,
    pub title: String,
    pub essence: Option<String>,
    pub insights: Option<String>,
    pub difficulty: i16,
    pub importance: String,
    pub interval_days: i32,
    pub next_review_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardDto {
    pub category_id: i32,
    pub title: String,
    pub essence: String,
    pub insights: Option<String>,
    pub difficulty: i16,
    pub importance: String,
}

#[derive(Debug, Deserialize)]
pub struct ReviewCardDto {
    pub interval_days: i32, // 用户标记的下一次复习间隔
}