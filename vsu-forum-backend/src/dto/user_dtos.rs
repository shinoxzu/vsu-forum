use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub login: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterDTO {
    pub login: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginDTO {
    pub login: String,
    pub password: String
}