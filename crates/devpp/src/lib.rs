pub mod devcontainer;
pub mod extension;
pub mod feature;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevContainer(#[from] devcontainer::Error),
    #[error(transparent)]
    Feature(#[from] feature::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn build(workspace: std::path::PathBuf, config: Option<std::path::PathBuf>) -> Result<()> {
    let config = devcontainer::find_config(&workspace, config.as_ref())?;

    let mut s = std::fs::read_to_string(&config.path).unwrap();
    let devcontainer = devcontainer::DevContainer::from_str(&mut s)?;

    let abs_config = config.path.parent().unwrap().canonicalize().unwrap();
    let abs_dotdev = devcontainer::find_dotdev(&config)?.canonicalize().unwrap();

    let build_info = devcontainer.get_build_info();
    dbg!(&build_info);

    for (feature_ref, _options) in devcontainer.common.features {
        let ref_valid = feature::FeatureRefValid::new(abs_config.clone(), abs_dotdev.clone(), feature_ref.clone())?;

        if !matches!(ref_valid.0.kind, feature::FeatureRefKind::Local) {
            continue;
        }

        let feature_valid = feature::FeatureValid::new(ref_valid, abs_config.clone())?;
        dbg!(&feature_valid);

        let customizations = extension::Customizations::new(feature_valid);
        if let Some(devpp) = customizations.0.devpp {
            dbg!(&devpp);
        }
    }

    Ok(())
}
