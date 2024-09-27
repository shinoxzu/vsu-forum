use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{
        common::ObjectCreatedDTO,
        user::{LoginDTO, RegisterDTO, UserDTO},
    },
    extractors::ValidatedJson,
    state::ApplicationState,
    tools::hash_text,
};

pub async fn register_user(
    State(state): State<ApplicationState>,
    ValidatedJson(register_dto): ValidatedJson<RegisterDTO>,
) -> (StatusCode, Json<ObjectCreatedDTO>) {
    let result = sqlx::query_scalar!(
        "insert into users(login, password_hash) values ($1, $2) returning id",
        register_dto.login,
        hash_text(register_dto.password)
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(ObjectCreatedDTO { id: result }))
}

pub async fn login_user(
    State(state): State<ApplicationState>,
    ValidatedJson(login_dto): ValidatedJson<LoginDTO>,
) -> (StatusCode, Json<ObjectCreatedDTO>) {
    (StatusCode::CREATED, Json(ObjectCreatedDTO { id: 1 }))
}

pub async fn get_user(
    Path(user_id): Path<i32>,
    State(state): State<ApplicationState>,
) -> Json<UserDTO> {
    Json(UserDTO {
        id: 1,
        login: "".to_string(),
    })
}
