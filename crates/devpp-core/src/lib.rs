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
use stable_topo_sort::stable_topo_sort;

use crate::error::Result;

pub fn build(mut w: impl Write, workspace: &Path, config: Option<&Path>) -> Result<()> {
    let config = Config::find_config(workspace, config)?;
    let devc = DevContainer::new(read_to_string(&config.path)?)?;
    let build_info = BuildInfo::new(&config, &devc)?;

    let mut features = Features::new();
    for (id, options) in &devc.common.features {
        let reference = Reference::new(id, &config)?;
        let feature = Feature::new(&reference)?;
        features.insert(id, (feature, options));
    }

    let (mut nodes, mut edges) = (vec![], vec![]);
    for (id, (feature, _options)) in &features {
        nodes.push(*id);
        for dep_id in &feature.inner.installs_after {
            edges.push((dep_id, *id));
        }
    }
    let ids = stable_topo_sort(&nodes, &edges)?;

    let mut cf = Containerfile::new(&build_info)?;
    cf.patch_base()?;

    for id in &ids {
        let (feature, options) = features.get(id).unwrap();
        cf.apply_feature(feature, options, &features)?;
    }

    cf.apply_merger(&features, &ids)?;

    Ok(writeln!(w, "{cf}")?)
}
