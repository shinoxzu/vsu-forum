use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicCategory {
    pub id: i64,
    pub name: String,
}
