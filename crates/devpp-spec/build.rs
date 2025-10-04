// @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainer.base.schema.json
fn generate_devc() {
    let j = std::fs::read_to_string("./src/devc.schema.json").unwrap();
    let p = std::fs::read_to_string("./src/devc.schema.json.patch").unwrap();
    let s = diffy::apply(&j, &diffy::Patch::from_str(&p).unwrap()).unwrap();

    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&s).unwrap();
    let mut type_space = typify::TypeSpace::new(&typify::TypeSpaceSettings::default());
    type_space.add_root_schema(schema).unwrap();

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    std::fs::write(out_dir.join("devc.rs"), contents).unwrap();
}

// @see: https://raw.githubusercontent.com/devcontainers/spec/113500f4125e0f14c9293adf4d8d94a861e0ec11/schemas/devContainerFeature.schema.json
fn generate_feat() {
    let j = std::fs::read_to_string("./src/feat.schema.json").unwrap();
    let p = std::fs::read_to_string("./src/feat.schema.json.patch").unwrap();
    let s = diffy::apply(&j, &diffy::Patch::from_str(&p).unwrap()).unwrap();

    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&s).unwrap();
    let mut type_space = typify::TypeSpace::new(&typify::TypeSpaceSettings::default());
    type_space.add_root_schema(schema).unwrap();

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    std::fs::write(out_dir.join("feat.rs"), contents).unwrap();
}

fn main() {
    generate_devc();
    generate_feat();
}
