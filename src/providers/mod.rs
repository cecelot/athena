pub mod sourcebin;

pub trait Provider {
    /// Uploads the content to the service this provider implements. This should return
    /// a url to the uploaded paste.
    fn upload(content: String) -> anyhow::Result<String>;
}
