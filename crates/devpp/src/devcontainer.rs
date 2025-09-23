pub mod generated {
    // @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainer.base.schema.json
    typify::import_types!("./src/devcontainer.schema.json");
}

// TODO: https://github.com/oxidecomputer/typify/issues/896
// diff --git a/devcontainer.schema.json b/devcontainer.schema.json
// index 0000000..0000000 100644
// --- a/devcontainer.schema.json
// +++ b/devcontainer.schema.json
// @@ -19,29 +19,12 @@
//  				"features": {
//  					"type": "object",
//  					"description": "Features to add to the dev container.",
// -					"properties": {
// -						"fish": {
// -							"deprecated": true,
// -							"deprecationMessage": "Legacy feature not supported. Please check https://containers.dev/features for replacements."
// -						},
// -						"maven": {
// -							"deprecated": true,
// -							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/java` has an option to install Maven."
// -						},
// -						"gradle": {
// -							"deprecated": true,
// -							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/java` has an option to install Gradle."
// -						},
// -						"homebrew": {
// -							"deprecated": true,
// -							"deprecationMessage": "Legacy feature not supported. Please check https://containers.dev/features for replacements."
// -						},
// -						"jupyterlab": {
// -							"deprecated": true,
// -							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/python` has an option to install JupyterLab."
// +					"additionalProperties": {
// +						"type": "object",
// +							"additionalProperties": {
// +							"type": "string"
//  						}
// -					},
// -					"additionalProperties": true
// +                    }
//  				},
//  				"overrideFeatureInstallOrder": {
//  					"type": "array",
// @@ -451,18 +434,6 @@
//  						},
//  						"gpu": {
//  							"oneOf": [
// -								{
// -									"type": [
// -										"boolean",
// -										"string"
// -									],
// -									"enum": [
// -										true,
// -										false,
// -										"optional"
// -									],
// -									"description": "Indicates whether a GPU is required. The string \"optional\" indicates that a GPU is optional. An object value can be used to configure more detailed requirements."
// -								},
//  								{
//  									"type": "object",
//  									"properties": {

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    #[error(transparent)]
    InvalidJsonc(#[from] std::io::Error),
    #[error("config is not found")]
    NotFoundConfig,
    #[error("config {config:?} is not found within search path {entries:?}")]
    NotFoundConfigArg {
        config: std::path::PathBuf,
        entries: Vec<std::path::PathBuf>,
    },
    #[error("the project must have a .devcontainer/ folder at the root of the project workspace folder")]
    NotFoundDotDev,
    #[error("parent directory is not found")]
    NotFoundParent,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DevContainer {
    #[serde(flatten)]
    pub common: generated::DevContainerCommon,
    #[serde(flatten)]
    pub is_compose: IsCompose,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum IsCompose {
    Compose(generated::ComposeContainer),
    NonCompose(NonCompose),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NonCompose {
    #[serde(flatten)]
    pub base: generated::NonComposeBase,
    #[serde(flatten)]
    pub is_image: IsImage,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum IsImage {
    Dockerfile(generated::DockerfileContainer),
    Image(generated::ImageContainer),
}

impl DevContainer {
    pub fn from_str(s: &mut str) -> Result<Self> {
        json_strip_comments::strip(s).map_err(Error::InvalidJsonc)?;
        serde_json::from_str::<Self>(&s).map_err(Error::InvalidJson)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub kind: ConfigKind,
    pub path: std::path::PathBuf,
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

pub fn find_configs<P: AsRef<std::path::Path>>(workspace: P) -> std::io::Result<Vec<Config>> {
    let mut entries = vec![];

    let case_nested = workspace.as_ref().join(".devcontainer/devcontainer.json");
    if case_nested.exists() {
        entries.push(Config {
            kind: ConfigKind::Nested,
            path: case_nested,
        });
    }

    let case_plain = workspace.as_ref().join(".devcontainer.json");
    if case_plain.exists() {
        entries.push(Config {
            kind: ConfigKind::Plain,
            path: case_plain,
        });
    }

    if let Ok(dir) = std::fs::read_dir(workspace.as_ref().join(".devcontainer")) {
        for entry in dir {
            let path = entry?.path();
            let case_scoped = path.join("devcontainer.json");
            if case_scoped.exists() {
                entries.push(Config {
                    kind: ConfigKind::Scoped,
                    path: case_scoped,
                });
            }
        }
    }

    Ok(entries)
}

pub fn find_config<P: AsRef<std::path::Path>>(workspace: P, config: Option<P>) -> Result<Config> {
    let entries = find_configs(workspace).unwrap();

    if entries.is_empty() {
        return Err(Error::NotFoundConfig);
    }

    match config {
        Some(config) => {
            for entry in &entries {
                let lhs = entry.path.canonicalize().unwrap();
                let rhs = config.as_ref().canonicalize().unwrap();
                if lhs.eq(&rhs) {
                    return Ok(entry.to_owned());
                }
            }
            Err(Error::NotFoundConfigArg {
                config: config.as_ref().to_path_buf(),
                entries: entries.iter().map(|v| v.path.to_owned()).collect(),
            })
        }
        None => Ok(entries.first().unwrap().clone()),
    }
}

pub fn find_dotdev(config: &Config) -> Result<std::path::PathBuf> {
    match config.kind {
        ConfigKind::Nested => Ok(config.path.parent().ok_or(Error::NotFoundDotDev)?.to_owned()),
        ConfigKind::Plain => Err(Error::NotFoundDotDev),
        ConfigKind::Scoped => Ok(config
            .path
            .parent()
            .ok_or(Error::NotFoundParent)?
            .parent()
            .ok_or(Error::NotFoundParent)?
            .to_owned()),
    }
}
