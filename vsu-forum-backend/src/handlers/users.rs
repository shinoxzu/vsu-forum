use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    dto::{
        claims::Claims,
        user::{AuthorizedUserDTO, LoginDTO, RegisterDTO, UserDTO},
    },
    errors::ApiError,
    extractors::ValidatedJson,
    models::User,
    state::ApplicationState,
    tools::hash_text,
};

pub async fn register_user(
    State(state): State<ApplicationState>,
    ValidatedJson(register_dto): ValidatedJson<RegisterDTO>,
) -> Result<(StatusCode, Json<AuthorizedUserDTO>), ApiError> {
    let user = sqlx::query_as!(
        User,
        "select * from users where login = $1",
        register_dto.login
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match user {
        Some(_) => Err(ApiError::BadRequest(
            "user with this login already registered".to_string(),
        )),
        None => {
            let result = sqlx::query_scalar!(
                "insert into users(login, password_hash) values ($1, $2) returning id",
                register_dto.login,
                hash_text(register_dto.password)
            )
            .fetch_one(&state.db_pool)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

            let claims = Claims {
                exp: (Utc::now() + Duration::days(31)).timestamp() as usize,
                sub: register_dto.login,
                user_id: result,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.config.jwt.secret_key.as_ref()),
            )
            .map_err(|_| ApiError::InternalServerError)?;

            Ok((
                StatusCode::CREATED,
                Json(AuthorizedUserDTO { token, id: result }),
            ))
        }
    }
}

pub async fn login_user(
    State(state): State<ApplicationState>,
    ValidatedJson(login_dto): ValidatedJson<LoginDTO>,
) -> Result<(StatusCode, Json<AuthorizedUserDTO>), ApiError> {
    let user = sqlx::query_as!(
        User,
        "select * from users where login = $1 and password_hash = $2",
        login_dto.login,
        hash_text(login_dto.password)
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| ApiError::InternalServerError)?;

    match user {
        Some(user) => {
            let claims = Claims {
                exp: (Utc::now() + Duration::days(31)).timestamp() as usize,
                sub: user.login,
                user_id: user.id,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.config.jwt.secret_key.as_ref()),
            )
            .map_err(|_| ApiError::InternalServerError)?;

            Ok((
                StatusCode::OK,
                Json(AuthorizedUserDTO { token, id: user.id }),
            ))
        }
        None => Err(ApiError::OtherError(
            StatusCode::UNAUTHORIZED,
            "login or password is incorrect".to_string(),
        )),
    }
}

pub async fn get_user(
    Path(user_id): Path<i64>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<UserDTO>), ApiError> {
    let user = sqlx::query_as!(User, "select * from users where id = $1", user_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    match user {
        Some(user) => Ok((
            StatusCode::OK,
            Json(UserDTO {
                id: user.id,
                login: user.login,
            }),
        )),
        None => Err(ApiError::NotFound("user not found".to_string())),
    }
}

pub async fn get_me(
    Extension(claims): Extension<Claims>,
    State(state): State<ApplicationState>,
) -> Result<(StatusCode, Json<UserDTO>), ApiError> {
    let user = sqlx::query_as!(User, "select * from users where id = $1", claims.user_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    match user {
        Some(user) => Ok((
            StatusCode::OK,
            Json(UserDTO {
                id: user.id,
                login: user.login,
            }),
        )),
        None => Err(ApiError::NotFound("user not found".to_string())),
    }
}
