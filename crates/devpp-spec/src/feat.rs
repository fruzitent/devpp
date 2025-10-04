use crate::Error;
use crate::Result;
use crate::devc;
use crate::normalize;

#[derive(Clone, Debug)]
pub struct Reference {
    pub id: String,
    pub kind: ReferenceKind,
}

impl Reference {
    pub fn new(id: &str, config: &devc::Config, workspace: &std::path::Path) -> Result<Self> {
        Ok(Self {
            id: id.to_string(),
            kind: ReferenceKind::new(id, config, workspace)?,
        })
    }
}

/// @see: https://containers.dev/implementors/features/#referencing-a-feature
#[derive(Clone, Debug)]
pub enum ReferenceKind {
    /// @see: https://containers.dev/implementors/features-distribution/#oci-registry
    Artifact {
        reference: oci_spec::distribution::Reference,
    },
    /// @see: https://containers.dev/implementors/features-distribution/#addendum-locally-referenced
    Local { path: std::path::PathBuf },
    /// @see: https://containers.dev/implementors/features-distribution/#directly-reference-tarball
    Tarball { url: url::Url },
}

impl ReferenceKind {
    pub fn new(id: &str, config: &devc::Config, workspace: &std::path::Path) -> Result<Self> {
        if std::path::Path::new(id).is_absolute() {
            return Err(Error::ReferencePathAbsolute { id: id.to_string() });
        }

        let config_dir = config.path.parent().unwrap();
        let path_feature = config_dir.join(id);

        if let Ok(path) = path_feature.canonicalize() {
            let dotdev = config.find_dotdev()?.canonicalize()?;
            if !path.starts_with(&dotdev) {
                return Err(Error::ReferencePathIllegal {
                    dotdev,
                    id: id.to_string(),
                    path,
                });
            }
            return Ok(Self::Local {
                path: normalize(workspace, &path_feature)?,
            });
        }

        if let Ok(reference) = id.parse() {
            return Ok(Self::Artifact { reference });
        }

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

    fn run(workspace: &std::path::Path) -> Result<()> {
        let config = devc::Config::find_config(workspace, None)?;
        let devc = devc::DevContainer::new(std::fs::read_to_string(&config.path)?)?;
        for id in devc.common.features.keys() {
            Reference::new(id, &config, workspace)?;
        }
        Ok(())
    }

    #[test]
    fn reference_invalid_argument() {
        let workspace = tests::root("tests/fixtures/reference_invalid_argument");
        match run(&workspace) {
            Err(Error::ReferenceInvalidArgument { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_not_found() {
        let workspace = tests::root("tests/fixtures/reference_not_found");
        match run(&workspace) {
            Err(Error::ReferenceNotFound { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_absolute() {
        let workspace = tests::root("tests/fixtures/reference_path_absolute");
        match run(&workspace) {
            Err(Error::ReferencePathAbsolute { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_illegal() {
        let workspace = tests::root("tests/fixtures/reference_path_illegal");
        match run(&workspace) {
            Err(Error::ReferencePathIllegal { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_path_traversal() {
        let workspace = tests::root("tests/fixtures/reference_path_traversal");
        match run(&workspace) {
            Ok(_) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn reference_scheme_mismatch() {
        let workspace = tests::root("tests/fixtures/reference_scheme_mismatch");
        match run(&workspace) {
            Err(Error::ReferenceSchemeMismatch { .. }) => {}
            other => panic!("{other:?}"),
        }
    }
}
