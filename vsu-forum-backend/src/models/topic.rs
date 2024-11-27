use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Topic {
    pub id: i64,
    pub author_id: i64,
    pub category_id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
