use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReportDTO {
    pub id: i64,
    pub reported_user_name: String,
    pub author_id: i64,
    pub reason: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateReportDTO {
    pub reported_user_name: String,
    pub reason: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateReportDTO {
    pub reason: Option<String>,
}
