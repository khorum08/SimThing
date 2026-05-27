use crate::metadata::DisplayMeta;
use crate::version::SpecVersion;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameModeSpec {
    pub id: String,
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    pub spec_version: SpecVersion,
    #[serde(default)]
    pub metadata: DisplayMeta,
    #[serde(default)]
    pub domain_packs: Vec<super::domain_pack::DomainPackSpec>,
    #[serde(default)]
    pub properties: Vec<super::property::PropertySpec>,
    #[serde(default)]
    pub overlays: Vec<super::overlay::OverlaySpec>,
    #[serde(default)]
    pub capability_trees: Vec<super::capability::CapabilityTreeSpec>,
    #[serde(default)]
    pub events: Vec<super::event::EventSpec>,
    /// Resource Flow arena admission graph (E-10). Validated at session build.
    #[serde(default)]
    pub resource_flow: Option<super::resource_flow::ResourceFlowSpec>,
    /// Production transfer / recipe / emission / threshold-emit registrations (Phase T).
    #[serde(default)]
    pub resource_economy: Option<super::resource_economy::ResourceEconomySpec>,
    /// RF-T4: scenario-class flat-star Resource Flow execution profile at session open.
    ///
    /// Precedence: explicit spec `FlatStarOptIn` wins over profile enablement. Profile does
    /// not enable GPU from `ResourceFlowSpec` presence alone — arenas must be authored.
    #[serde(default)]
    pub resource_flow_execution_profile: super::resource_flow::ResourceFlowExecutionProfile,
}
