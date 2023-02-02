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

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    pub author: String,
    pub quote: String,
    pub tags: Vec<String>,
    pub id: String,
    pub image: String,
    pub length: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contents {
    pub quotes: Vec<Quote>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rooty {
    pub success: String,
    pub contents: Contents,
}