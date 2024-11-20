use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reaction {
    pub post_id: i64,
    pub author_id: i64,
    pub reaction_id: i64,
}
