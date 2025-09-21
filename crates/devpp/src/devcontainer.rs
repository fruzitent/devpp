pub mod generated {
    // @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainer.base.schema.json
    typify::import_types!("./src/devcontainer.schema.json");
}

// TODO: https://github.com/oxidecomputer/typify/issues/896
// diff --git a/devcontainer.schema.json b/devcontainer.schema.json
// index 0000000..0000000 100644
// --- a/devcontainer.schema.json
// +++ b/devcontainer.schema.json
// @@ -451,18 +451,6 @@
//                                                 },
//                                                 "gpu": {
//                                                         "oneOf": [
// -                                                               {
// -                                                                       "type": [
// -                                                                               "boolean",
// -                                                                               "string"
// -                                                                       ],
// -                                                                       "enum": [
// -                                                                               true,
// -                                                                               false,
// -                                                                               "optional"
// -                                                                       ],
// -                                                                       "description": "Indicates whether a GPU is required. The string \"optional\" indicates that a GPU is optional. An object value can be used to configure more detailed requirements."
// -                                                               },
//                                                                 {
//                                                                         "type": "object",
//                                                                         "properties": {

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config is not found")]
    NotFoundConfig,
    #[error("config {config:?} is not found within search path {entries:?}")]
    NotFoundConfigArg {
        config: std::path::PathBuf,
        entries: Vec<std::path::PathBuf>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

// @see: https://containers.dev/implementors/spec/#devcontainerjson
pub fn find_configs<P: AsRef<std::path::Path>>(workspace: P) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut entries = vec![];

    let case_nested = workspace.as_ref().join(".devcontainer/devcontainer.json");
    if case_nested.exists() {
        entries.push(case_nested);
    }

    let case_plain = workspace.as_ref().join(".devcontainer.json");
    if case_plain.exists() {
        entries.push(case_plain);
    }

    if let Ok(dir) = std::fs::read_dir(workspace.as_ref().join(".devcontainer")) {
        for entry in dir {
            let path = entry?.path();
            let case_scoped = path.join("devcontainer.json");
            if case_scoped.exists() {
                entries.push(case_scoped);
            }
        }
    }

    Ok(entries)
}

pub fn find_config<P: AsRef<std::path::Path>>(workspace: P, config: Option<P>) -> Result<std::path::PathBuf> {
    let entries = find_configs(workspace).unwrap();

    if entries.is_empty() {
        return Err(Error::NotFoundConfig);
    }

    match config {
        Some(config) => {
            for entry in &entries {
                let lhs = entry.canonicalize().unwrap();
                let rhs = config.as_ref().canonicalize().unwrap();
                if lhs.eq(&rhs) {
                    return Ok(entry.to_owned());
                }
            }
            Err(Error::NotFoundConfigArg {
                config: config.as_ref().to_path_buf(),
                entries,
            })
        }
        None => Ok(entries.first().unwrap().clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_unexpected_location() {
        let temp_dir = tempfile::tempdir().unwrap();

        let config = temp_dir.path().join("foo/bar.json");
        std::fs::create_dir_all(config.parent().unwrap()).unwrap();
        std::fs::File::create(&config).unwrap();

        std::fs::File::create(temp_dir.path().join(".devcontainer.json")).unwrap();
        assert!(matches!(
            find_config(temp_dir.path().to_path_buf(), Some(config)),
            Err(Error::NotFoundConfigArg { .. })
        ));
    }
}
