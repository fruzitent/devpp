#[derive(Debug, thiserror::Error)]
pub enum Error {
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

    for id in devc.common.features.keys() {
        let reference = devpp_spec::feat::Reference::new(id, &config)?;
        dbg!(&reference);
    }

    Err(Error::NotImplemented)
}
