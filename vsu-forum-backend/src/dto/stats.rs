use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForumStatsDTO {
    pub posts_count: i64,
    pub users_count: i64,
    pub topics_count: i64,
}
