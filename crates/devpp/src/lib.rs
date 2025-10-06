#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppContainerfile(#[from] devpp_containerfile::Error),
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("not implemented")]
    NotImplemented,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn build(workspace: &std::path::Path, config: Option<&std::path::Path>) -> Result<()> {
    let config = devpp_spec::devc::Config::find_config(workspace, config)?;
    dbg!(&config);
    let dotdev = config.find_dotdev()?;
    dbg!(&dotdev);
    let devc = devpp_spec::devc::DevContainer::new(std::fs::read_to_string(&config.path)?)?;
    dbg!(&devc);

    let build_info = devpp_spec::devc::BuildInfo::new(&config, &devc)?;
    dbg!(&build_info);

    for (id, options) in &devc.common.features {
        let reference = devpp_spec::feat::Reference::new(id, &config)?;
        dbg!(&reference);
        let feature = devpp_spec::feat::Feature::new(&reference)?;
        dbg!(&feature);

        devpp_containerfile::write_feature(&mut std::io::stdout(), &build_info, &feature, options)?;
    }

    Err(Error::NotImplemented)
}
