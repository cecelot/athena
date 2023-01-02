#![deny(clippy::pedantic)]
use crate::{
    log::{abort, info},
    providers::{sourcebin::SourceBin, Provider},
};
use anyhow::Context;
use clap::{Parser, ValueEnum};
use std::{env, fs, path::PathBuf, process};
use uuid::Uuid;

mod log;
mod providers;

/// The Athena CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// The provider to upload the paste to.
    provider: ProviderChoice,

    /// The file to paste. If not specified, the contents of the file opened by $EDITOR
    /// is used as the contents of the paste.
    file: Option<PathBuf>,
}

#[derive(Copy, Clone, ValueEnum)]
enum ProviderChoice {
    /// https://sourceb.in
    Sourcebin,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let content = match cli.file {
        Some(file) => {
            fs::read_to_string(&file).context(format!("Failed to read file: {}", file.display()))
        }
        None => input(),
    }?;
    let url = match cli.provider {
        ProviderChoice::Sourcebin => <SourceBin as Provider>::upload(content)?,
    };

    info(&format!("uploaded to {url}"));

    Ok(())
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
