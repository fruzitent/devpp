use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::devc::generated::ComposeContainer;
use crate::devc::generated::DevContainerCommon;
use crate::devc::generated::DockerfileContainer;
use crate::devc::generated::ImageContainer;
use crate::devc::generated::NonComposeBase;
use crate::error::Error;
use crate::error::Result;

#[allow(clippy::all)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/devc.rs"));
}

#[derive(Clone, Debug)]
pub struct Config {
    pub kind: ConfigKind,
    pub path: PathBuf,
}

impl Config {
    pub fn find_config(workspace: &Path, config: Option<&Path>) -> Result<Config> {
        let entries = Self::find_entries(workspace)?;

        if entries.is_empty() {
            return Err(Error::ConfigNotFound);
        }

        match config {
            Some(config) => {
                for entry in &entries {
                    let lhs = &entry.path;
                    let rhs = &config.canonicalize()?;
                    if lhs.eq(rhs) {
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

    pub fn find_dotdev(&self) -> Result<PathBuf> {
        match &self.kind {
            ConfigKind::Nested { dotdev } => Ok(dotdev.clone()),
            ConfigKind::Plain => Err(Error::DotdevNotFound),
            ConfigKind::Scoped { dotdev } => Ok(dotdev.clone()),
        }
    }

    pub fn find_entries(workspace: &Path) -> Result<Vec<Config>> {
        let mut entries = vec![];
        let dotdev = workspace.join(".devcontainer");

        if let Ok(path_nested) = dotdev.join("devcontainer.json").canonicalize() {
            entries.push(Self {
                kind: ConfigKind::Nested {
                    dotdev: dotdev.canonicalize()?,
                },
                path: path_nested,
            });
        }

        if let Ok(path_plain) = workspace.join(".devcontainer.json").canonicalize() {
            entries.push(Self {
                kind: ConfigKind::Plain,
                path: path_plain,
            });
        }

        if let Ok(dir) = std::fs::read_dir(&dotdev) {
            for entry in dir {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }
                if let Ok(path_scoped) = path.join("devcontainer.json").canonicalize() {
                    entries.push(Self {
                        kind: ConfigKind::Scoped {
                            dotdev: dotdev.canonicalize()?,
                        },
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
    Nested { dotdev: PathBuf },
    /// .devcontainer.json
    Plain,
    /// .devcontainer/<folder>/devcontainer.json
    Scoped { dotdev: PathBuf },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DevContainer {
    #[serde(flatten)]
    pub common: DevContainerCommon,
    #[serde(flatten)]
    pub is_compose: IsCompose,
}

impl DevContainer {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let mut s = s.into();
        json_strip_comments::strip(&mut s)?;
        Ok(serde_json::from_str::<Self>(&s)?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IsCompose {
    Compose(ComposeContainer),
    NonCompose(NonCompose),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonCompose {
    #[serde(flatten)]
    pub base: NonComposeBase,
    #[serde(flatten)]
    pub is_image: IsImage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IsImage {
    Dockerfile(DockerfileContainer),
    Image(ImageContainer),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    #[test]
    fn config_ambiguous() {
        let workspace = tests::root("tests/fixtures/config_ambiguous");
        match Config::find_config(&workspace, None) {
            Err(Error::ConfigAmbiguous { .. }) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn config_not_found() {
        let workspace = tests::root("tests/fixtures/config_not_found");
        match Config::find_config(&workspace, None) {
            Err(Error::ConfigNotFound) => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn config_permission_denied() {
        let root = tests::root("tests/fixtures/config_permission_denied");
        match Config::find_config(&root.join("workspace"), Some(&root.join("devcontainer.json"))) {
            Err(Error::ConfigPermissionDenied { .. }) => {}
            other => panic!("{other:?}"),
        }
    }
}
