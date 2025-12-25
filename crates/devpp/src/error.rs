#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppCore(#[from] devpp_core::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
