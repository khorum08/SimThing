use crate::metadata::DisplayMeta;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainPackSpec {
    pub id:           String,
    pub display_name: String,
    #[serde(default)]
    pub metadata:     DisplayMeta,
    #[serde(default)]
    pub properties:   Vec<super::property::PropertySpec>,
    #[serde(default)]
    pub overlays:     Vec<super::overlay::OverlaySpec>,
    #[serde(default)]
    pub capability_trees: Vec<super::capability::CapabilityTreeSpec>,
}
