use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorWithDataDTO<T: Serialize> {
    pub err: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct ErrorDTO {
    pub err: String,
}
