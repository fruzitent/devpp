#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    StableTopoSort(#[from] stable_topo_sort::error::Error),
    #[error("dependencies of merge features are not supported")]
    NestedMergeNotSupported,
    #[error("target stage must be set")]
    TargetNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
