use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub subject_id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryDto {
    pub subject_id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub sort_order: Option<i32>,
}