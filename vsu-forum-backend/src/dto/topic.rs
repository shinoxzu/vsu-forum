use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{topic_category::TopicCategoryDTO, user::UserDTO};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicDTO {
    pub id: i64,
    pub name: String,
    pub category: TopicCategoryDTO,
    pub creator: UserDTO,
    pub posts_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortTopicDTO {
    pub id: i64,
    pub author_id: i64,
    pub category_id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTopicDTO {
    pub category_id: i64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTopicDTO {
    pub category_id: Option<i64>,
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}
