use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorDTO<T: Serialize> {
    pub err: String,
    pub data: T,
}
