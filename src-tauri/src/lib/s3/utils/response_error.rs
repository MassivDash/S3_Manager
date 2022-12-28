use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseError {
    pub name: String,
    pub message: String,
}

pub fn create_error(name: String, message: String) -> ResponseError {
    return ResponseError { name, message };
}
