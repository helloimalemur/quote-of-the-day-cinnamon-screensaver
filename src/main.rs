//curl -X GET "https://quotes.rest/qod?language=en" -H  "accept: application/json"
// https://turreta.com/2019/09/22/rust-convert-struct-instances-to-and-from-json/
// https://jsonformatter.curiousconcept.com/#

mod qod;

use std::time::Duration;
use tokio::*;
use crate::qod::*;


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = client.get("https://quotes.rest/qod?language=en")
        .timeout(Duration::new(3,0))
        .send()
        .await
        .expect("unable to fetch qod")
        .text()
        .await
        .expect("unable to parse text");

    let quotes: Root = match serde_json::from_str(res.clone().as_ref()) {
        Ok(ok) => ok,
        Err(_e) => json_failure()
    };

    let quote_line = format!("{} ~{}", quotes.contents.quotes.get(0).unwrap().quote, quotes.contents.quotes.get(0).unwrap().author);

    println!("{}", quote_line);

    process::Command::new("cinnamon-screensaver-command").args(vec!["-l","-m",quote_line.as_str()]).spawn().unwrap();
}

// if request fails we'll use this quote
fn json_failure() -> Root {
    let data = r#"
        {
  "success": {
    "total": 1
  },
  "contents": {
    "quotes": [
      {
        "quote": "x,x",
        "length": "3",
        "author": "",
        "tags": [
          "inspire"
        ],
        "category": "",
        "language": "en",
        "date": "",
        "permalink": "",
        "id": "",
        "background": "",
        "title": ""
      }
    ]
  },
  "baseurl": "https://theysaidso.com",
  "copyright": {
    "year": 2025,
    "url": "https://theysaidso.com"
  }
}"#;
    serde_json::from_str(data).unwrap()
}

pub fn failure() -> &'static str {
    "message"
}
