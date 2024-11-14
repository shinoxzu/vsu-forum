use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionDTO {
    pub author_id: i64,
    pub reaction: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddReactionDTO {
    pub post_id: i64,
    pub reaction: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RemoveReactionDTO {
    pub post_id: i64,
    pub reaction: String,
}
