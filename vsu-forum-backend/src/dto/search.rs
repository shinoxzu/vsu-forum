use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQueryParamsDTO {
    pub query: String,
}
