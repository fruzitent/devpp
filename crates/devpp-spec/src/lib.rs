pub mod devc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("config is not specified, found within search path {entries:?}")]
    ConfigAmbiguous { entries: Vec<std::path::PathBuf> },
    #[error(
        "the config file could not be found in one the following locations: \
        [.devcontainer/devcontainer.json, .devcontainer.json, .devcontainer/<folder>/devcontainer.json]"
    )]
    ConfigNotFound,
    #[error("config {config:?} is not found within search path {entries:?}")]
    ConfigPermissionDenied {
        config: std::path::PathBuf,
        entries: Vec<std::path::PathBuf>,
    },
    #[error("the project must have a .devcontainer/ folder at the root of the project workspace folder")]
    DotdevNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
