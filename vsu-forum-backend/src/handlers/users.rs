use axum::{extract::Path, http::StatusCode, Json};

use crate::dto::{
    common::ObjectCreatedDTO, 
    user_dtos::{LoginDTO, RegisterDTO, UserDTO}
};

pub async fn register_user(Json(register_dto): Json<RegisterDTO>) -> (StatusCode, Json<ObjectCreatedDTO>) {
    (StatusCode::CREATED, Json(ObjectCreatedDTO { id: 1 }))
}

pub async fn login_user(Json(login_dto): Json<LoginDTO>) -> (StatusCode, Json<ObjectCreatedDTO>) {
    (StatusCode::CREATED, Json(ObjectCreatedDTO { id: 1 }))
}

pub async fn get_user(Path(user_id): Path<i32>) -> Json<UserDTO> {
    Json(UserDTO { id: 1, login: "".to_string() })
}