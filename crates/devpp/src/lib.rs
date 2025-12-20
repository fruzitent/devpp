use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use devpp_spec::devc::BuildInfo;
use devpp_spec::devc::Config;
use devpp_spec::devc::DevContainer;
#[cfg(feature = "devpp")]
use devpp_spec::devpp::Customizations;
use devpp_spec::feat::Feature;
use devpp_spec::feat::Reference;
use topo_sort::SortResults;
use topo_sort::TopoSort;

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

pub fn build(workspace: &Path, config: Option<&Path>) -> Result<()> {
    let config = Config::find_config(workspace, config)?;
    let devc = DevContainer::new(read_to_string(&config.path)?)?;
    let build_info = BuildInfo::new(&config, &devc)?;

    let mut features = BTreeMap::new();
    let mut graph = TopoSort::new();

    for (id, options) in &devc.common.features {
        let reference = Reference::new(id, &config)?;
        let feature = Feature::new(&reference)?;
        graph.insert(id.to_owned(), feature.inner.installs_after.to_vec());
        features.insert(id, (feature, options));
    }

    let items = match graph.into_vec_nodes() {
        SortResults::Full(items) => items,
        SortResults::Partial(_) => return Err(Error::GraphCycle),
    };

    devpp_containerfile::write_base(&mut std::io::stdout(), &build_info)?;
    writeln!(&mut std::io::stdout())?;

    for id in items {
        let (feature, options) = features.get(&id).unwrap();
        #[cfg(feature = "devpp")]
        let customizations = Customizations::new(feature);
        devpp_containerfile::write_feature(
            &mut std::io::stdout(),
            &build_info,
            #[cfg(feature = "devpp")]
            &customizations,
            feature,
            &features,
            options,
        )?;
        writeln!(&mut std::io::stdout())?;
    }

    Err(Error::NotImplemented)
}
