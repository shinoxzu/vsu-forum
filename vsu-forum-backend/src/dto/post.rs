use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDTO {
    pub id: i64,
    pub author_id: i64,
    pub topic_id: i64,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePostDTO {
    pub topic_id: i64,
    #[validate(length(min = 1, max = 1000))]
    pub text: String,
}