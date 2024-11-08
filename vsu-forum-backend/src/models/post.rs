use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i64,
    pub topic_id: i64,
    pub author_id: i64,
    pub text: String,
}
