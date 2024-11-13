use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookmarkDTO {
    pub topic_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateBookmarkDTO {
    pub topic_id: i64,
}
