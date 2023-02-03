//curl -X GET "https://quotes.rest/qod?language=en" -H  "accept: application/json"
// https://turreta.com/2019/09/22/rust-convert-struct-instances-to-and-from-json/
// https://jsonformatter.curiousconcept.com/#

mod qod;

use std::thread;
use std::time::Duration;
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

    // print response
    // println!("{:?}", res.clone());

    let quotes: Root = match serde_json::from_str(res.clone().as_ref()) {
        Ok(ok) => ok,
        Err(e) => panic!("{}", res)
    };

    let quote_line = format!("{} ~{}", quotes.contents.quotes.get(0).unwrap().quote, quotes.contents.quotes.get(0).unwrap().author);

    println!("{}", quote_line);

    process::Command::new("cinnamon-screensaver-command").args(vec!["-l","-m",quote_line.as_str()]).spawn().unwrap();
    thread::sleep(Duration::new(4,0));
    process::Command::new("cinnamon-unlock-desktop").spawn().unwrap();

}
