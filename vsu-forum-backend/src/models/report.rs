use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub id: i64,
    pub reported_user_id: i64,
    pub author_id: i64,
    pub reason: String,
}
