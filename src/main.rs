//curl -X GET "https://quotes.rest/qod?language=en" -H  "accept: application/json"
// https://turreta.com/2019/09/22/rust-convert-struct-instances-to-and-from-json/
// https://jsonformatter.curiousconcept.com/#

mod qod;
use tokio::*;
use crate::qod::*;
use reqwest::Client;


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = match match client.get("https://quotes.rest/qod?language=en")
        .send()
        .await {
        Ok(x) => x,
        Err(_) => request_failure().await,
    }
        .text()
        .await {
        Ok(x) => x,
        Err(_) => failure().to_string(),
    };

    // print response
    // println!("{:?}", res.clone());

    let quotes: Root = match serde_json::from_str(res.clone().as_ref()) {
        Ok(ok) => ok,
        Err(_e) => json_failure()
    };

    let quote_line = format!("{} ~{}", quotes.contents.quotes.get(0).unwrap().quote, quotes.contents.quotes.get(0).unwrap().author);

    println!("{}", quote_line);

    process::Command::new("cinnamon-screensaver-command").args(vec!["-l","-m",quote_line.as_str()]).spawn().unwrap();
}

async fn request_failure() -> reqwest::Response {
    let response = reqwest::Client::get(&Client::new(), "google.com").send().await.unwrap();
    response
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
        "quote": "If I work as hard as I can, I wonder how much I can do in a day?",
        "length": "64",
        "author": "Ezra Taft Benson",
        "tags": [
          "inspire"
        ],
        "category": "inspire",
        "language": "en",
        "date": "2023-02-02",
        "permalink": "https://theysaidso.com/quote/ezra-taft-benson-if-i-work-as-hard-as-i-can-i-wonder-how-much-i-can-do-in-a-day",
        "id": "I6A4ptotLn_wOFHoq4jiuAeF",
        "background": "https://theysaidso.com/img/qod/qod-inspire.jpg",
        "title": "Inspiring Quote of the day"
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
