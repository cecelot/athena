use crate::log::abort;
use anyhow::Context;
use std::{env, fs, path::PathBuf, process};
use uuid::Uuid;

mod gist;
mod rsplay;
mod sourcebin;
mod tsplay;

pub use gist::Gist;
pub use rsplay::RSPlay;
pub use sourcebin::SourceBin;
pub use tsplay::TSPlay;

pub trait Provider {
    type Options;

    /// Uploads the content to the service this provider implements. This should return
    /// a url to the uploaded paste.
    fn upload(options: Self::Options) -> anyhow::Result<String>;
}

/// Reads the file at `path` if if is [`Option::Some`], or fetches input from `$EDITOR` if it is [`Option::None`].
fn content(path: Option<PathBuf>) -> anyhow::Result<String> {
    match path {
        Some(path) => {
            fs::read_to_string(&path).context(format!("Failed to read file: {}", path.display()))
        }
        None => input(),
    }
}

/// Reads the contents of the temporary file /tmp/athena/xxx.paste and returns it as a [`String`](std::str::String).
fn input() -> anyhow::Result<String> {
    fs::create_dir_all("/tmp/athena").context("Failed to create tmp directory")?;

    let path = format!("/tmp/athena/{}.paste", Uuid::new_v4());
    let editor = env::var("EDITOR").context("No $EDITOR set")?;

    let mut cmd = process::Command::new(editor)
        .arg(&path)
        .spawn()
        .context("failed to start $EDITOR")?;
    cmd.wait().context("$EDITOR wasn't running")?;

    let input = fs::read_to_string(&path).unwrap_or_else(|_| abort("no input specified"));

    Ok(input)
}
