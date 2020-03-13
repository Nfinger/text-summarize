extern crate tokio;
use reqwest::{Error};
use serde_json::{Value};

const URL: &'static str = "http://esummarizer.com/main/getsummary";

#[tokio::main]
pub async fn summarize_text(text: &str) -> Result<Value, Error> {
    let params = [
        ("text", text),
        ("nbsentences", "5")
    ];
    let client = reqwest::Client::new();
    let res = client.post(URL)
        .form(&params)
        .send()
        .await?;
    let text = res.text().await?;
    let json_str: Value = serde_json::from_str(&text).unwrap();
    Ok(json_str)
}