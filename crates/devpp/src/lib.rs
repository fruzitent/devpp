#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not implemented")]
    NotImplemented,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn build(_workspace: &std::path::Path, _config: Option<&std::path::Path>) -> Result<()> {
    Err(Error::NotImplemented)
}
