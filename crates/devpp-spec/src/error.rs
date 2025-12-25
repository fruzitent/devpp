use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(feature = "artifact")]
    #[error(transparent)]
    Oci(#[from] oci_spec::distribution::ParseError),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[cfg(feature = "tarball")]
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("config is not specified, found within search path {entries:?}")]
    ConfigAmbiguous { entries: Vec<PathBuf> },
    #[error(
        "the config file could not be found in one the following locations: \
        [.devcontainer/devcontainer.json, .devcontainer.json, .devcontainer/<folder>/devcontainer.json]"
    )]
    ConfigNotFound,
    #[error("config {config:?} is not found within search path {entries:?}")]
    ConfigPermissionDenied { config: PathBuf, entries: Vec<PathBuf> },
    #[error("the project must have a .devcontainer/ folder at the root of the project workspace folder")]
    DotdevNotFound,
    #[error("the local feature's sub-folder must contain a install.sh entrypoint script: {id:?}")]
    FeatureEntrypointNotFound { id: String },
    #[error("the local feature's sub-folder must contain a devcontainer-feature.json file: {id:?}")]
    FeatureMetadataNotFound { id: String },
    #[error("the sub-folder name must match the feature's id field: {id:?} expected {expected:?}, but got {got:?}")]
    FeatureIdMismatch {
        expected: String,
        got: std::ffi::OsString,
        id: String,
    },
    #[cfg(feature = "tarball")]
    #[error("the .tgz archive file must be named devcontainer-feature-<featureId>.tgz: {id:?}")]
    ReferenceInvalidArgument { id: String },
    #[error("feature is not found: {id:?}")]
    ReferenceNotFound { id: String },
    #[error("a local feature may not be referenced by absolute path: {id:?}")]
    ReferencePathAbsolute { id: String },
    #[error(
        "a local feature's source code must be contained within a sub-folder of the .devcontainer/ folder: \
        feature {id:?} is resolved to a path {path:?} outside of {dotdev:?} directory"
    )]
    ReferencePathIllegal { dotdev: PathBuf, id: String, path: PathBuf },
    #[cfg(feature = "tarball")]
    #[error("feature URI scheme must be https: {id:?}")]
    ReferenceSchemeMismatch { id: String },
}

pub type Result<T> = std::result::Result<T, Error>;
