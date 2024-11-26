use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableReactionDTO {
    pub id: i64,
    pub reaction: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateAvailableReactionDTO {
    #[validate(length(min = 1, max = 10))]
    pub reaction: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateAvailableReactionDTO {
    #[validate(length(min = 1, max = 10))]
    pub reaction: Option<String>,
}
