use crate::Error;
use crate::Result;

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/devc.rs"));
}

#[derive(Clone, Debug)]
pub struct Config {
    pub kind: ConfigKind,
    pub path: std::path::PathBuf,
}

impl Config {
    pub fn find_config(workspace: &std::path::Path, config: Option<&std::path::Path>) -> Result<Config> {
        let entries = Self::find_entries(workspace)?;

        if entries.is_empty() {
            return Err(Error::ConfigNotFound);
        }

        match config {
            Some(config) => {
                for entry in &entries {
                    let lhs = entry.path.canonicalize()?;
                    let rhs = config.canonicalize()?;
                    if lhs.eq(&rhs) {
                        return Ok(entry.to_owned());
                    }
                }
                Err(Error::ConfigPermissionDenied {
                    config: config.to_path_buf(),
                    entries: entries.iter().map(|v| v.path.to_owned()).collect(),
                })
            }
            None => {
                if entries.len() > 1 {
                    return Err(Error::ConfigAmbiguous {
                        entries: entries.iter().map(|v| v.path.to_owned()).collect(),
                    });
                }
                Ok(entries.first().unwrap().to_owned())
            }
        }
    }

    pub fn find_dotdev(&self) -> Result<std::path::PathBuf> {
        match self.kind {
            ConfigKind::Nested => Ok(self.path.parent().ok_or(Error::ParentNotFound)?.to_owned()),
            ConfigKind::Plain => Err(Error::DotdevNotFound),
            ConfigKind::Scoped => Ok(self
                .path
                .parent()
                .and_then(|path| path.parent())
                .ok_or(Error::ParentNotFound)?
                .to_owned()),
        }
    }

    pub fn find_entries(workspace: &std::path::Path) -> Result<Vec<Config>> {
        let mut entries = vec![];

        let path_nested = workspace.join(".devcontainer/devcontainer.json");
        if path_nested.try_exists()? {
            entries.push(Self {
                kind: ConfigKind::Nested,
                path: path_nested,
            });
        }

        let path_plain = workspace.join(".devcontainer.json");
        if path_plain.try_exists()? {
            entries.push(Self {
                kind: ConfigKind::Plain,
                path: path_plain,
            });
        }

        if let Ok(dir) = std::fs::read_dir(workspace.join(".devcontainer")) {
            for entry in dir {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }
                let path_scoped = path.join("devcontainer.json");
                if path_scoped.try_exists()? {
                    entries.push(Self {
                        kind: ConfigKind::Scoped,
                        path: path_scoped,
                    });
                }
            }
        }

        Ok(entries)
    }
}

/// @see: https://containers.dev/implementors/spec/#devcontainerjson
#[derive(Clone, Debug)]
pub enum ConfigKind {
    /// .devcontainer/devcontainer.json
    Nested,
    /// .devcontainer.json
    Plain,
    /// .devcontainer/<folder>/devcontainer.json
    Scoped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_ambiguous() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/config_ambiguous");
        match Config::find_config(&workspace, None) {
            Err(Error::ConfigAmbiguous { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn config_not_found() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/config_not_found");
        match Config::find_config(&workspace, None) {
            Err(Error::ConfigNotFound) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn config_permission_denied() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/config_permission_denied");
        match Config::find_config(&root.join("workspace"), Some(&root.join("devcontainer.json"))) {
            Err(Error::ConfigPermissionDenied { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn dotdev_not_found() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dotdev_not_found");
        let config = Config::find_config(&workspace, None).unwrap();
        match config.find_dotdev() {
            Err(Error::DotdevNotFound) => {}
            other => panic!("{other:?}"),
        }
    }
}
