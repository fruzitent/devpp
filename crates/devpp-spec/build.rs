use std::env::var;
use std::fs::read_to_string;
use std::path::PathBuf;

use diffy::Patch;
use schemars::schema::RootSchema;
use typify::TypeSpace;
use typify::TypeSpaceSettings;

fn typify(name: &str, s: &str, settings: &TypeSpaceSettings) {
    let mut type_space = TypeSpace::new(settings);

    let schema = serde_json::from_str::<RootSchema>(s).unwrap();
    type_space.add_root_schema(schema).unwrap();

    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    std::fs::write(out_dir.join(name), contents).unwrap();
}

// @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainer.base.schema.json
fn generate_devc() {
    let j = read_to_string("./src/devc.schema.json").unwrap();
    let p = read_to_string("./src/devc.schema.json.patch").unwrap();
    let s = diffy::apply(&j, &Patch::from_str(&p).unwrap()).unwrap();
    let mut settings = TypeSpaceSettings::default();
    let settings = settings.with_map_type("std::collections::BTreeMap");
    typify("devc.rs", &s, settings);
}

#[cfg(feature = "devpp")]
fn generate_devpp() {
    let s = read_to_string("./src/devpp.schema.json").unwrap();
    let mut settings = TypeSpaceSettings::default();
    let settings = settings.with_map_type("std::collections::BTreeMap");
    typify("devpp.rs", &s, settings);
}

// @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainerFeature.schema.json
fn generate_feat() {
    let j = read_to_string("./src/feat.schema.json").unwrap();
    let p = read_to_string("./src/feat.schema.json.patch").unwrap();
    let s = diffy::apply(&j, &Patch::from_str(&p).unwrap()).unwrap();
    let mut settings = TypeSpaceSettings::default();
    let settings = settings.with_map_type("std::collections::BTreeMap");
    typify("feat.rs", &s, settings);
}

fn main() {
    generate_devc();
    #[cfg(feature = "devpp")]
    generate_devpp();
    generate_feat();
}
