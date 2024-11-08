use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReportDTO {
    pub id: i64,
    pub reported_user_id: i64,
    pub author_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateReportDTO {
    pub reported_user_id: i64,
}
