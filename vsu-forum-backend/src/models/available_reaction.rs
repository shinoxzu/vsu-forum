use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableReaction {
    pub id: i64,
    pub reaction: String,
}
