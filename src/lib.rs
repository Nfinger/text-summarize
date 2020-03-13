use std::error::Error;
use reqwest::blocking::multipart;
const URL: &'static str = "http://esummarizer.com/main/getsummary";

pub fn summarize_text(text: &str) -> Result<String, Box<dyn Error>> {
    let form = multipart::Form::new()
        .text("text", text.to_string())
        .text("nbsentences", "5");

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(URL)
        .multipart(form)
        .send()?;
    let text = resp.text()?;
    Ok(text)
}