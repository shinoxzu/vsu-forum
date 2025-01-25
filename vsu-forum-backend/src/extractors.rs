use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use crate::dto::errors::ErrorWithDataDTO;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = JsonValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum JsonValidationError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for JsonValidationError {
    fn into_response(self) -> Response {
        match self {
            JsonValidationError::ValidationError(validation_errors) => {
                let error = ErrorWithDataDTO {
                    err: "validation error occurred".to_string(),
                    data: validation_errors,
                };
                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            }
            JsonValidationError::AxumJsonRejection(reason) => {
                let error = ErrorWithDataDTO {
                    err: "passed json is invalid".to_string(),
                    data: reason.to_string(),
                };
                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            }
        }
    }
}
