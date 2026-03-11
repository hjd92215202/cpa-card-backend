use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};
use validator::Validate;

/// 知识卡片核心模型
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: i32,
    pub user_id: i32,
    pub category_id: Option<i32>,
    /// 章节名称：由 Repository 通过 JOIN categories 表填充，用于前端分组和 PDF 导出
    pub category_name: Option<String>, 
    pub title: String,
    pub essence: Option<String>,
    pub insights: Option<String>,
    pub difficulty: i16,
    pub importance: String,
    pub interval_days: i32,
    /// 卡片类型：qa(问答), note(笔记), code(代码)
    pub card_type: Option<String>, 
    pub next_review_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
}

/// 创建/更新卡片时的数据传输对象
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
    
    /// 考试重要程度：必须是 A, B 或 C
    #[validate(custom(function = "validate_importance"))]
    pub importance: String,
    
    pub card_type: Option<String>,
}

/// 更新复习进度时的请求载荷
#[derive(Debug, Deserialize)]
pub struct ReviewCardDto {
    pub interval_days: i32,
}

/// 自定义校验函数：确保重要度等级符合 CPA 体系规范
fn validate_importance(val: &str) -> Result<(), validator::ValidationError> {
    if ["A", "B", "C"].contains(&val) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("重要度等级必须是 A、B 或 C"))
    }
}