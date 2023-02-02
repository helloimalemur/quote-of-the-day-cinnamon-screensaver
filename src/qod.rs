use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub error: Error,
}