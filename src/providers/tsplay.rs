use super::api::prelude::*;

pub struct TSPlay;

impl Provider for TSPlay {
    type Options = PathOptions;

    fn upload(options: Self::Options) -> anyhow::Result<String> {
        let content = content(options.path)?;
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
