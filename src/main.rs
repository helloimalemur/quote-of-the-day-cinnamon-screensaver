//curl -X GET "https://quotes.rest/qod?language=en" -H  "accept: application/json"
mod qod;

use tokio::*;

#[tokio::main]
fn main() {
    let client = reqwest::Client::new();
    // let res = client.get("http://koonts.net")

}
