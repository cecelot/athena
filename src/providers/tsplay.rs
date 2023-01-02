use super::Provider;
use anyhow::Context;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub struct TSPlay;

impl Provider for TSPlay {
    fn upload(content: String) -> anyhow::Result<String> {
        let compressed = lz_str::compress_to_base64(&content).replace('/', "-");
        let body = Request::new(format!(
            "https://www.typescriptlang.org/play?#code/{compressed}"
        ));
        let res = Client::new()
            .post("https://tsplay.dev/api/short")
            .json(&body)
            .send()
            .context("Failed to send request")?;
        let res: Response = res.json().context("Failed to parse response")?;
        Ok(res.shortened)
    }
}

#[derive(Serialize)]
struct Request {
    url: String,
    #[serde(rename = "createdOn")]
    created_on: String,
    expires: bool,
}

impl Request {
    fn new(url: String) -> Self {
        Self {
            url,
            created_on: "client".into(),
            expires: false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    shortened: String,
}
