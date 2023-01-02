mod sourcebin;
mod tsplay;

pub use sourcebin::SourceBin;
pub use tsplay::TSPlay;

pub trait Provider {
    /// Uploads the content to the service this provider implements. This should return
    /// a url to the uploaded paste.
    fn upload(content: String) -> anyhow::Result<String>;
}
