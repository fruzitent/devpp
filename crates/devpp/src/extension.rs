pub mod generated {
    // TODO: https://github.com/oxidecomputer/typify/issues/313
    typify::import_types!("./src/extension.schema.json");
}

pub struct Customizations(pub generated::ExtensionCustomizations);

impl Customizations {
    pub fn new(feature_valid: crate::feature::FeatureValid) -> Self {
        Self(serde_json::from_value(serde_json::to_value(feature_valid.0.inner.customizations).unwrap()).unwrap())
    }
}
