use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicCategoryDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTopicCategoryDTO {
    pub id: i64,
    #[validate(length(min = 6, max = 30))]
    pub name: String,
}
