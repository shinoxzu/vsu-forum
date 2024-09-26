use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{
        common::ObjectCreatedDTO,
        user_dtos::{LoginDTO, RegisterDTO, UserDTO},
    },
    state::ApplicationState,
};

#[axum::debug_handler]
pub async fn register_user(
    State(state): State<ApplicationState>,
    Json(register_dto): Json<RegisterDTO>,
) -> (StatusCode, Json<ObjectCreatedDTO>) {
    (StatusCode::CREATED, Json(ObjectCreatedDTO { id: 1 }))
}

pub async fn login_user(
    State(state): State<ApplicationState>,
    Json(login_dto): Json<LoginDTO>,
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
