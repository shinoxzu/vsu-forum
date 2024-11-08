use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    dto::{claims::Claims, error::ErrorDTO},
    ApplicationState,
};

pub async fn auth_middleware(
    State(state): State<ApplicationState>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header.replace("Bearer ", "")
    } else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorDTO {
                err: "pass token".to_string(),
            }),
        )
            .into_response();
    };

    if let Some(claims) = authorize_current_user(auth_header, &state.config.jwt.secret_key).await {
        req.extensions_mut().insert(claims);
        next.run(req).await.into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorDTO {
                err: "token is incorrect".to_string(),
            }),
        )
            .into_response()
    }
}

async fn authorize_current_user(auth_token: String, secret_key: &str) -> Option<Claims> {
    decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )
    .ok()
    .map(|d| d.claims)
}
