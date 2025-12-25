pub mod error;

use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use devpp_containerfile::write_base;
use devpp_containerfile::write_feature;
use devpp_spec::devc::BuildInfo;
use devpp_spec::devc::Config;
use devpp_spec::devc::DevContainer;
#[cfg(feature = "devpp")]
use devpp_spec::devpp::Customizations;
use devpp_spec::feat::Feature;
use devpp_spec::feat::Reference;
use topo_sort::SortResults;
use topo_sort::TopoSort;

use crate::error::Error;
use crate::error::Result;

pub fn build(mut w: impl Write, workspace: &Path, config: Option<&Path>) -> Result<()> {
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

    write_base(&mut w, &build_info)?;
    writeln!(&mut w)?;

    for id in items {
        let (feature, options) = features.get(&id).unwrap();
        #[cfg(feature = "devpp")]
        let customizations = Customizations::new(feature);
        write_feature(
            &mut w,
            &build_info,
            #[cfg(feature = "devpp")]
            &customizations,
            feature,
            &features,
            options,
        )?;
        writeln!(&mut w)?;
    }

    Ok(())
}
