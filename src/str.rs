use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Root {
    pub success: Success,
    pub contents: Contents,
    pub baseurl: String,
    pub copyright: Copyright,
}

#[derive(Serialize, Deserialize, Debug)]
struct Success {
    pub total: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contents {
    pub quotes: Vec<Struct>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Struct {
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
struct Copyright {
    pub year: i64,
    pub url: String,
}
