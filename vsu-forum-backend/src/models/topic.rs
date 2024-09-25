use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Topic {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
}
