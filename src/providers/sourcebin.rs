use super::api::prelude::*;

pub struct SourceBin;

impl Provider for SourceBin {
    type Options = PathOptions;

    fn upload(options: Self::Options) -> anyhow::Result<String> {
        let content = content(options.path)?;
        let body = Request::new(content);
        let res = Client::new()
            .post("https://sourceb.in/api/bins")
            .json(&body)
            .send()
            .context("Failed to send request")?;
        let res: Response = res.json().context("Failed to parse response")?;
        Ok(format!("https://sourceb.in/{}", res.key))
    }
}

#[derive(Serialize)]
struct Request {
    files: Vec<File>,
}

impl Request {
    fn new(content: String) -> Self {
        Self {
            files: vec![File { content }],
        }
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    key: String,
}

#[derive(Serialize)]
pub struct File {
    content: String,
}
