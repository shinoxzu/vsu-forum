use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicCategoryDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTopicCategoryDTO {
    #[validate(length(min = 2, max = 30))]
    pub name: String,
}
