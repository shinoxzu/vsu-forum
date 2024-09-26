use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectCreatedDTO {
    pub id: i32,
}
