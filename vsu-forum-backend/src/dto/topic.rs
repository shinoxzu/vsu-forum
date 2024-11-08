use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicDTO {
    pub id: i64,
    pub author_id: i64,
    pub category_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTopicDTO {
    pub category_id: i64,
    #[validate(length(min = 6, max = 30))]
    pub name: String,
}
