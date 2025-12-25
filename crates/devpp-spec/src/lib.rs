pub mod devc;
#[cfg(feature = "devpp")]
pub mod devpp;
pub mod error;
pub mod feat;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    pub(crate) fn root(path: impl AsRef<Path>) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}
