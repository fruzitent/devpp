#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("target stage must be set")]
    TargetNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
