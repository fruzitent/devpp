pub mod devc;
pub mod devpp;
pub mod error;
pub mod feat;

use serde_json::json;

use crate::devc::DevContainer;
use crate::error::Result;
use crate::feat::Feature;

pub fn get_metadata(devc: &DevContainer, features: &[&Feature]) -> Result<String> {
    let mut metadata = features.iter().fold(vec![], |mut acc, feature| {
        acc.push(json!({
            "customizations": feature.inner.customizations,
        }));
        acc
    });
    metadata.push(json!({
        "customizations": devc.common.customizations,
    }));
    Ok(serde_json::to_string(&metadata)?)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    pub(crate) fn root(path: impl AsRef<Path>) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}
