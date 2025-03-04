use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::dto::errors::ErrorDTO;

pub enum ApiError {
    InternalServerError,
    BadRequest(String),
    NotFound(String),
    OtherError(StatusCode, String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorDTO {
                    err: "sorry, try again later".to_string(),
                }),
            ),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, Json(ErrorDTO { err: msg })),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, Json(ErrorDTO { err: msg })),
            ApiError::OtherError(code, msg) => (code, Json(ErrorDTO { err: msg })),
        }
        .into_response()
    }
}
