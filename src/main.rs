//curl -X GET "https://quotes.rest/qod?language=en" -H  "accept: application/json"
// https://turreta.com/2019/09/22/rust-convert-struct-instances-to-and-from-json/

mod qod;
use qod::*;

use tokio::*;
use crate::qod::*;
use serde::{Deserialize, Serialize};
use reqwest::Response;
use serde_json::json;


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = client.get("https://quotes.rest/qod?language=en")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", res.clone());


    let quotes: Vec<Root> = match serde_json::from_str(res.clone().as_ref()) {
        Ok(ok) => ok,
        Err(e) => panic!("{}", res)
    };

    // println!("{:?}", quotes);
}
