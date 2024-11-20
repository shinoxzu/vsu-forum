use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BookmarkDTO {
    pub topic_id: i64,
}
