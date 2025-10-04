pub mod devc;
pub mod feat;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Oci(#[from] oci_spec::distribution::ParseError),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
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
    ReferencePathIllegal {
        dotdev: std::path::PathBuf,
        id: String,
        path: std::path::PathBuf,
    },
    #[error("feature URI scheme must be https: {id:?}")]
    ReferenceSchemeMismatch { id: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    pub(crate) fn root(path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}
