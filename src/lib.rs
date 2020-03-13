extern crate tokio;
use reqwest::Error;

const URL: &'static str = "http://esummarizer.com/main/getsummary";

#[tokio::main]
pub async fn summarize_text(text: &str) -> Result<String, Error> {
    let params = [
        ("text", text),
        ("nbsentences", "5")
    ];
    let client = reqwest::Client::new();
    client.post(URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await
}