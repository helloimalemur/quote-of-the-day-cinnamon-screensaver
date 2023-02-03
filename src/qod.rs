use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rooty {
    pub success: Success,
    pub contents: Contents,
    pub baseurl: String,
    pub copyright: Copyright
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    total: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quotes {
    quote: String,
    author: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Contents {
    pub quotes: Quotes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Copyright {
    year: i32,
    url: String
}
