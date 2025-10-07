use crate::feat;

#[allow(clippy::all)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/devpp.rs"));
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Customizations(pub generated::DevppCustomizations);

impl Customizations {
    pub fn new(feature: &feat::Feature) -> Self {
        // TODO: hack
        Self(serde_json::from_value(serde_json::to_value(&feature.inner.customizations).unwrap()).unwrap())
    }
}
