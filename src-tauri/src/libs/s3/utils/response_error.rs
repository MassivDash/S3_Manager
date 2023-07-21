use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResponseError {
    pub name: String,
    pub message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error: {} - {}", self.name, self.message)
    }
}

pub fn create_error(name: String, message: String) -> ResponseError {
    return ResponseError { name, message };
}
