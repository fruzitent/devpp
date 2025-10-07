use crate::Error;
use crate::Result;
use crate::devc;

#[allow(clippy::all)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/feat.rs"));
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Feature {
    #[serde(skip)]
    pub entrypoint: std::path::PathBuf,
    #[serde(flatten)]
    pub inner: generated::Feature,
    #[cfg(feature = "devpp")]
    #[serde(skip)]
    pub merger: std::path::PathBuf,
    #[serde(skip)]
    pub metadata: std::path::PathBuf,
}

impl Feature {
    pub fn new(reference: &Reference) -> Result<Self> {
        let this = match &reference.kind {
            #[cfg(feature = "artifact")]
            ReferenceKind::Artifact { .. } => unimplemented!(),
            // @see: https://containers.dev/implementors/features/#folder-structure
            ReferenceKind::Local { path } => {
                let path_metadata = path.join("devcontainer-feature.json");
                if !path_metadata.try_exists()? {
                    return Err(Error::FeatureMetadataNotFound {
                        id: reference.id.clone(),
                    });
                };

                let path_entrypoint = path.join("install.sh");
                if !path_entrypoint.try_exists()? {
                    return Err(Error::FeatureEntrypointNotFound {
                        id: reference.id.clone(),
                    });
                }

                #[cfg(feature = "devpp")]
                let path_merger = path.join("configure.sh");
                #[cfg(feature = "devpp")]
                if !path_merger.try_exists()? {
                    return Err(Error::FeatureMergerNotFound {
                        id: reference.id.clone(),
                    });
                }

                let mut s = std::fs::read_to_string(&path_metadata)?;
                json_strip_comments::strip(&mut s)?;
                Self {
                    #[cfg(feature = "devpp")]
                    merger: path_merger,
                    entrypoint: path_entrypoint,
                    inner: serde_json::from_str(&s)?,
                    metadata: path_metadata,
                }
            }
            #[cfg(feature = "tarball")]
            ReferenceKind::Tarball { .. } => unimplemented!(),
        };
        reference.validate(&this.inner.id)?;
        Ok(this)
    }
}

#[derive(Clone, Debug)]
pub struct Reference {
    pub id: String,
    pub kind: ReferenceKind,
}

impl Reference {
    pub fn new(id: &str, config: &devc::Config) -> Result<Self> {
        Ok(Self {
            id: id.to_string(),
            kind: ReferenceKind::new(id, config)?,
        })
    }

    pub fn validate(&self, id: &str) -> Result<()> {
        match &self.kind {
            #[cfg(feature = "artifact")]
            ReferenceKind::Artifact { .. } => unimplemented!(),
            ReferenceKind::Local { path } => {
                let got = path.iter().next_back().unwrap();
                if got.to_str().unwrap() != id {
                    return Err(Error::FeatureIdMismatch {
                        expected: id.to_string(),
                        got: got.to_os_string(),
                        id: self.id.clone(),
                    });
                }
            }
            #[cfg(feature = "tarball")]
            ReferenceKind::Tarball { .. } => unimplemented!(),
        };
        Ok(())
    }
}

/// @see: https://containers.dev/implementors/features/#referencing-a-feature
#[derive(Clone, Debug)]
pub enum ReferenceKind {
    /// @see: https://containers.dev/implementors/features-distribution/#oci-registry
    #[cfg(feature = "artifact")]
    Artifact {
        reference: oci_spec::distribution::Reference,
    },
    /// @see: https://containers.dev/implementors/features-distribution/#addendum-locally-referenced
    Local { path: std::path::PathBuf },
    /// @see: https://containers.dev/implementors/features-distribution/#directly-reference-tarball
    #[cfg(feature = "tarball")]
    Tarball { url: url::Url },
}

impl ReferenceKind {
    pub fn new(id: &str, config: &devc::Config) -> Result<Self> {
        if std::path::Path::new(id).is_absolute() {
            return Err(Error::ReferencePathAbsolute { id: id.to_string() });
        }

        if let Ok(path) = config.path.parent().unwrap().join(id).canonicalize() {
            let dotdev = config.find_dotdev()?;
            if !path.starts_with(&dotdev) {
                return Err(Error::ReferencePathIllegal {
                    dotdev,
                    id: id.to_string(),
                    path,
                });
            }
            return Ok(Self::Local { path });
        }

        #[cfg(feature = "artifact")]
        if let Ok(reference) = id.parse() {
            return Ok(Self::Artifact { reference });
        }

        #[cfg(feature = "tarball")]
        if let Ok(url) = url::Url::parse(id) {
            if url.scheme() != "https" {
                return Err(Error::ReferenceSchemeMismatch { id: id.to_string() });
            }

            let path = std::path::Path::new(url.path());
            let file_name = path.file_name().unwrap().to_str().unwrap();

            // TODO: check featureId
            if !(file_name.starts_with("devcontainer-feature-") && file_name.ends_with(".tgz")) {
                return Err(Error::ReferenceInvalidArgument { id: id.to_string() });
            }

            return Ok(Self::Tarball { url });
        }

        Err(Error::ReferenceNotFound { id: id.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devc;
    use crate::tests;

    fn run_feature(workspace: &std::path::Path) -> Result<()> {
        let config = devc::Config::find_config(workspace, None)?;
        let devc = devc::DevContainer::new(std::fs::read_to_string(&config.path)?)?;
        for id in devc.common.features.keys() {
            let reference = Reference::new(id, &config)?;
            Feature::new(&reference)?;
        }
        Ok(())
    }

    #[test]
    fn feature_entrypoint_not_found() {
        let workspace = tests::root("tests/fixtures/feature_entrypoint_not_found");
        match run_feature(&workspace) {
            Err(Error::FeatureEntrypointNotFound { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn feature_id_mismatch() {
        let workspace = tests::root("tests/fixtures/feature_id_mismatch");
        match run_feature(&workspace) {
            Err(Error::FeatureIdMismatch { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn feature_metadata_not_found() {
        let workspace = tests::root("tests/fixtures/feature_metadata_not_found");
        match run_feature(&workspace) {
            Err(Error::FeatureMetadataNotFound { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    fn run_reference(workspace: &std::path::Path) -> Result<()> {
        let config = devc::Config::find_config(workspace, None)?;
        let devc = devc::DevContainer::new(std::fs::read_to_string(&config.path)?)?;
        for id in devc.common.features.keys() {
            Reference::new(id, &config)?;
        }
        Ok(())
    }

    #[cfg(feature = "tarball")]
    #[test]
    fn reference_invalid_argument() {
        let workspace = tests::root("tests/fixtures/reference_invalid_argument");
        match run_reference(&workspace) {
            Err(Error::ReferenceInvalidArgument { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_not_found() {
        let workspace = tests::root("tests/fixtures/reference_not_found");
        match run_reference(&workspace) {
            Err(Error::ReferenceNotFound { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_absolute() {
        let workspace = tests::root("tests/fixtures/reference_path_absolute");
        match run_reference(&workspace) {
            Err(Error::ReferencePathAbsolute { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_illegal() {
        let workspace = tests::root("tests/fixtures/reference_path_illegal");
        match run_reference(&workspace) {
            Err(Error::ReferencePathIllegal { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_traversal() {
        let workspace = tests::root("tests/fixtures/reference_path_traversal");
        match run_reference(&workspace) {
            Ok(_) => {}
            other => panic!("{other:?}"),
        }
    }

    #[cfg(feature = "tarball")]
    #[test]
    fn reference_scheme_mismatch() {
        let workspace = tests::root("tests/fixtures/reference_scheme_mismatch");
        match run_reference(&workspace) {
            Err(Error::ReferenceSchemeMismatch { .. }) => {}
            other => panic!("{other:?}"),
        }
    }
}
