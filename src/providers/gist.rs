use super::api::prelude::*;
use crate::GistOptions;
use std::{collections::HashMap, path::PathBuf};

pub struct Gist;

impl Provider for Gist {
    type Options = GistOptions;

    fn upload(options: Self::Options) -> anyhow::Result<String> {
        let files = parse_files(&options)?;
        let body = Request::new(options.description, files);
        let res = Client::new()
            .post("https://api.github.com/gists")
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, "Athena: command-line paste uploader")
            .bearer_auth(options.token)
            .json(&body)
            .send()
            .context("Failed to send request")?;
        let res: Response = res.json().context("Failed to parse response")?;
        Ok(res.html_url)
    }
}

fn parse_files(options: &<Gist as Provider>::Options) -> anyhow::Result<Vec<(PathBuf, String)>> {
    let mut files = vec![];
    for path in &options.paths {
        files.push((path.clone(), content(Some(path.clone()))?));
    }
    Ok(files)
}

#[derive(Serialize, Debug)]
struct Request {
    description: String,
    public: bool,
    files: HashMap<String, File>,
}

#[derive(Serialize, Debug)]
struct File {
    content: String,
}

impl Request {
    fn new(description: Option<String>, files: Vec<(PathBuf, String)>) -> Self {
        Self {
            description: description.unwrap_or_default(),
            public: true,
            files: files
                .into_iter()
                .map(|(path, content)| {
                    let filename = String::from(path.file_name().unwrap().to_string_lossy());
                    (filename, File { content })
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

#[derive(Deserialize)]
struct Response {
    html_url: String,
}
