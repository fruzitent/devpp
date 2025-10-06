use std::io::Write;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppContainerfile(#[from] devpp_containerfile::Error),
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unexpected cycle")]
    GraphCycle,
    #[error("not implemented")]
    NotImplemented,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn build(workspace: &std::path::Path, config: Option<&std::path::Path>) -> Result<()> {
    let config = devpp_spec::devc::Config::find_config(workspace, config)?;
    let devc = devpp_spec::devc::DevContainer::new(std::fs::read_to_string(&config.path)?)?;
    let build_info = devpp_spec::devc::BuildInfo::new(&config, &devc)?;

    let mut features = std::collections::BTreeMap::new();
    let mut graph = topo_sort::TopoSort::new();

    for (id, options) in &devc.common.features {
        let reference = devpp_spec::feat::Reference::new(id, &config)?;
        let feature = devpp_spec::feat::Feature::new(&reference)?;
        graph.insert(id.to_owned(), feature.inner.installs_after.to_vec());
        features.insert(id, (feature, options));
    }

    let items = match graph.into_vec_nodes() {
        topo_sort::SortResults::Full(items) => items,
        topo_sort::SortResults::Partial(_) => return Err(Error::GraphCycle),
    };

    for id in items {
        let (feature, options) = features.get(&id).unwrap();
        devpp_containerfile::write_feature(&mut std::io::stdout(), &build_info, &config, &devc, feature, options)?;
        writeln!(&mut std::io::stdout())?;
    }

    Err(Error::NotImplemented)
}
