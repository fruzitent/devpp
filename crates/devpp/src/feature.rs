pub mod generated {
    // @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainerFeature.schema.json
    typify::import_types!("./src/feature.schema.json");
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    #[error(transparent)]
    InvalidJsonc(#[from] std::io::Error),
    #[error("the sub-folder name must match the Feature's id field: {feature_ref:?} expected {id:?}, but got {name:?}")]
    MismatchId {
        feature_ref: String,
        id: String,
        name: std::ffi::OsString,
    },
    #[error("feature is not found: {feature_ref:?}")]
    NotFound { feature_ref: String },
    #[error("the local Feature's sub-folder must contain a install.sh entrypoint script: {feature_ref:?}")]
    NotFoundEntry { feature_ref: String },
    #[error("the local Feature's sub-folder must contain a devcontainer-feature.json file: {feature_ref:?}")]
    NotFoundMetadata { feature_ref: String },
    #[error("a local Feature may not be referenced by absolute path: {feature_ref:?}")]
    PathAbsolute { feature_ref: String },
    #[error(
        "a local Feature's source code must be contained within a sub-folder of the .devcontainer/ folder: \
        feature {feature_ref:?} is resolved to a path {abs_feature:?} outside of {abs_dotdev:?} directory"
    )]
    PathIllegal {
        abs_dotdev: std::path::PathBuf,
        abs_feature: std::path::PathBuf,
        feature_ref: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct FeatureRef {
    pub inner: String,
    pub kind: FeatureRefKind,
}

/// @see: https://containers.dev/implementors/features/#referencing-a-feature
#[derive(Clone, Debug)]
pub enum FeatureRefKind {
    Artifact,
    Local,
    Tarball,
}

#[derive(Clone, Debug)]
pub struct FeatureRefValid(pub FeatureRef);

impl FeatureRefValid {
    pub fn new(abs_config: std::path::PathBuf, abs_dotdev: std::path::PathBuf, feature_ref: String) -> Result<Self> {
        if std::path::Path::new(&feature_ref).is_absolute() {
            return Err(Error::PathAbsolute { feature_ref });
        }

        // @see: https://containers.dev/implementors/features-distribution/#addendum-locally-referenced
        if let Ok(abs_feature) = abs_config.join(&feature_ref).canonicalize() {
            if !abs_feature.starts_with(&abs_dotdev) {
                return Err(Error::PathIllegal {
                    abs_dotdev,
                    abs_feature,
                    feature_ref,
                });
            }
            return Ok(Self(FeatureRef {
                inner: feature_ref,
                kind: FeatureRefKind::Local,
            }));
        }

        match feature_ref {
            _ if feature_ref.starts_with("./") => Err(Error::NotFound { feature_ref }),
            // @see: https://containers.dev/implementors/features-distribution/#directly-reference-tarball
            _ if feature_ref.starts_with("https://") => Ok(Self(FeatureRef {
                inner: feature_ref,
                kind: FeatureRefKind::Tarball,
            })),
            // @see: https://containers.dev/implementors/features-distribution/#oci-registry
            _ => Ok(Self(FeatureRef {
                inner: feature_ref,
                kind: FeatureRefKind::Artifact,
            })),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Feature {
    #[serde(flatten)]
    pub inner: generated::Feature,
}

impl Feature {
    pub fn from_str(s: &mut str) -> Result<Self> {
        json_strip_comments::strip(s).map_err(Error::InvalidJsonc)?;
        serde_json::from_str::<Self>(&s).map_err(Error::InvalidJson)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FeatureValid(pub Feature);

impl FeatureValid {
    pub fn new(ref_valid: FeatureRefValid, abs_config: std::path::PathBuf) -> Result<Self> {
        let abs_feature = abs_config.join(&ref_valid.0.inner).canonicalize().unwrap();

        let metadata_path = abs_feature.join("devcontainer-feature.json");
        if !metadata_path.exists() {
            return Err(Error::NotFoundMetadata {
                feature_ref: ref_valid.0.inner.clone(),
            });
        }

        let entrypoint_path = abs_feature.join("install.sh");
        if !entrypoint_path.exists() {
            return Err(Error::NotFoundEntry {
                feature_ref: ref_valid.0.inner.clone(),
            });
        }

        let mut s = std::fs::read_to_string(metadata_path).unwrap();
        let feature = Feature::from_str(&mut s)?;

        let name = abs_feature.iter().last().unwrap().to_os_string();
        let id = &feature.inner.id;
        if name.to_str().unwrap() != id {
            return Err(Error::MismatchId {
                feature_ref: ref_valid.0.inner,
                id: id.clone(),
                name,
            });
        }

        Ok(Self(feature))
    }
}
