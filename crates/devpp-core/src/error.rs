#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppContainerfile(#[from] devpp_containerfile::error::Error),
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    StableTopoSort(#[from] stable_topo_sort::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
