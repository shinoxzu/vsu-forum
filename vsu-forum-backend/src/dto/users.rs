use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterDTO {
    #[validate(length(min = 3, max = 40))]
    pub login: String,
    #[validate(length(min = 3, max = 40))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginDTO {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizedUserDTO {
    pub id: i64,
    pub token: String,
}
