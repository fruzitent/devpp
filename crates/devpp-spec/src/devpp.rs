use serde::Deserialize;
use serde::Serialize;

use crate::devpp::generated::DevppCustomizations;
use crate::feat::Feature;

#[allow(clippy::all)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/devpp.rs"));
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customizations(pub DevppCustomizations);

impl Customizations {
    pub fn new(feature: &Feature) -> Self {
        // TODO: hack
        Self(serde_json::from_value(serde_json::to_value(&feature.inner.customizations).unwrap()).unwrap())
    }
}
