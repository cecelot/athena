use super::api::prelude::*;

pub struct RSPlay;

impl Provider for RSPlay {
    type Options = PathOptions;

    fn upload(options: Self::Options) -> anyhow::Result<String> {
        let content = content(options.path)?;
        let body = Request::new(content);
        let res = Client::new()
            .post("https://play.rust-lang.org/meta/gist")
            .json(&body)
            .send()
            .context("Failed to send request")?;
        let res: Response = res.json().context("Failed to parse response")?;
        Ok(format!("https://play.rust-lang.org/?gist={}", res.id))
    }
}

#[derive(Serialize)]
struct Request {
    code: String,
}

impl Request {
    fn new(code: String) -> Self {
        Self { code }
    }
}

#[derive(Deserialize)]
struct Response {
    id: String,
}
