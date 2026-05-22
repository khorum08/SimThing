use crate::metadata::DisplayMeta;
use crate::version::SpecVersion;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameModeSpec {
    pub id:           String,
    pub display_name: String,
    #[serde(default)]
    pub description:  String,
    pub spec_version: SpecVersion,
    #[serde(default)]
    pub metadata:     DisplayMeta,
    #[serde(default)]
    pub domain_packs: Vec<super::domain_pack::DomainPackSpec>,
    #[serde(default)]
    pub properties:   Vec<super::property::PropertySpec>,
    #[serde(default)]
    pub overlays:     Vec<super::overlay::OverlaySpec>,
    #[serde(default)]
    pub capability_trees: Vec<super::capability::CapabilityTreeSpec>,
}
