use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub user_id: i64,
    pub topic_id: i64,
}
