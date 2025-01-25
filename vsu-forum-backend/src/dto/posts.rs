use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::users::UserDTO;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDTO {
    pub id: i64,
    pub topic_id: i64,
    pub text: String,
    pub sender: UserDTO,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePostDTO {
    pub topic_id: i64,
    #[validate(length(min = 1, max = 1000))]
    pub text: String,
}

#[derive(Deserialize)]
pub struct GetPostsDTO {
    pub topic_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePostDTO {
    #[validate(length(min = 1, max = 1000))]
    pub text: Option<String>,
}
