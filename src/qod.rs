use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub success: Success,
    pub contents: Contents,
    pub baseurl: String,
    pub copyright: Copyright,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub total: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contents {
    pub quotes: Vec<Qts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qts {
    pub quote: String,
    pub length: String,
    pub author: String,
    pub tags: Vec<String>,
    pub category: String,
    pub language: String,
    pub date: String,
    pub permalink: String,
    pub id: String,
    pub background: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Copyright {
    pub year: i64,
    pub url: String,
}
