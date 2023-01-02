#![deny(clippy::pedantic)]
use crate::{
    log::info,
    providers::{Gist, Provider, SourceBin, TSPlay},
};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

mod log;
mod providers;

/// The Athena CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// The provider to upload the paste to.
    #[clap(subcommand)]
    provider: ProviderChoice,
}

#[derive(Args, Clone)]
pub struct PathOptions {
    /// The file to paste.
    path: Option<PathBuf>,
}

#[derive(Args, Clone)]
pub struct GistOptions {
    /// A GitHub personal access token.
    #[arg(required = true, short, long)]
    token: String,

    /// The files to add to this gist.
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// The description of the gist.
    #[arg(short, long)]
    description: Option<String>,
}

#[derive(Clone, Subcommand)]
enum ProviderChoice {
    /// Uploads to https://sourceb.in
    #[clap(name = "sourcebin")]
    SourceBin(PathOptions),

    /// Creates a link for https://www.typescriptlang.org/play and shortens it using https://tsplay.dev
    #[clap(name = "tsplay")]
    TSPlay(PathOptions),

    /// Uploads to https://gist.github.com (requires authentication)
    #[clap(name = "gist")]
    Gist(GistOptions),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let url = match cli.provider {
        ProviderChoice::SourceBin(options) => upload::<SourceBin>(options),
        ProviderChoice::TSPlay(options) => upload::<TSPlay>(options),
        ProviderChoice::Gist(options) => upload::<Gist>(options),
    }?;

    info(&format!("uploaded to {url}"));

    Ok(())
}

/// A helper function for calling [`Provider::upload`].
fn upload<P: Provider>(options: P::Options) -> anyhow::Result<String> {
    P::upload(options)
}
