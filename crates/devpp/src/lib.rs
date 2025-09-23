pub mod devcontainer;
pub mod feature;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevContainer(#[from] devcontainer::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn build(workspace: std::path::PathBuf, config: Option<std::path::PathBuf>) -> Result<()> {
    let config = devcontainer::find_config(&workspace, config.as_ref())?;
    let mut s = std::fs::read_to_string(&config.path).unwrap();
    let devcontainer = devcontainer::DevContainer::from_str(&mut s)?;
    dbg!(&devcontainer);
    Ok(())
}
