extern crate tokio;

const URL: &'static str = "http://esummarizer.com/main/getsummary";

#[tokio::main]
pub async fn summarize_text(text: &str) -> String {
    let params = [
        ("text", text),
        ("nbsentences", "5")
    ];
    let client = reqwest::Client::new();
    client.post(URL)
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}