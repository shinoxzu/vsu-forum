#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub login: Option<String>,
    pub password_hash: Vec<u8>,
}
