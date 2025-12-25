pub mod error;

use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use devpp_containerfile::Containerfile;
use devpp_spec::devc::BuildInfo;
use devpp_spec::devc::Config;
use devpp_spec::devc::DevContainer;
use devpp_spec::feat::Feature;
use devpp_spec::feat::Features;
use devpp_spec::feat::Reference;
use topo_sort::SortResults;
use topo_sort::TopoSort;

use crate::error::Error;
use crate::error::Result;

pub fn build(mut w: impl Write, workspace: &Path, config: Option<&Path>) -> Result<()> {
    let config = Config::find_config(workspace, config)?;
    let devc = DevContainer::new(read_to_string(&config.path)?)?;
    let build_info = BuildInfo::new(&config, &devc)?;

    let mut features = Features::new();
    let mut ids = TopoSort::new();

    for (id, options) in &devc.common.features {
        let reference = Reference::new(id, &config)?;
        let feature = Feature::new(&reference)?;
        ids.insert(id.to_string(), feature.inner.installs_after.to_vec());
        features.insert(id, (feature, options));
    }

    let ids = match ids.into_vec_nodes() {
        SortResults::Full(ids) => ids,
        SortResults::Partial(_) => return Err(Error::GraphCycle),
    };

    let mut cf = Containerfile::new(&build_info)?;
    cf.patch_base()?;

    for id in &ids {
        let (feature, options) = features.get(id).unwrap();
        cf.apply_feature(feature, options, &features)?;
    }

    Ok(writeln!(w, "{cf}")?)
}
