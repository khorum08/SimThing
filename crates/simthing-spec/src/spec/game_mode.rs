use crate::metadata::DisplayMeta;
use crate::version::SpecVersion;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    /// Authored finite order-weight classes (ORDER-WEIGHT-CLASS-0).
    /// Operator directives that claim dominance must reference a class id.
    #[serde(default)]
    pub order_weight_classes: Vec<super::order_weight::OrderWeightClassSpec>,
    #[serde(default)]
    pub capability_trees: Vec<super::capability::CapabilityTreeSpec>,
    #[serde(default)]
    pub events: Vec<super::event::EventSpec>,
    /// Resource Flow authored override graph (E-10). Default admission derives
    /// from populated resource properties and topology at session build.
    #[serde(default)]
    pub resource_flow: Option<super::resource_flow::ResourceFlowSpec>,
    /// Production transfer / recipe / emission / threshold-emit registrations (Phase T).
    #[serde(default)]
    pub resource_economy: Option<super::resource_economy::ResourceEconomySpec>,
    /// Sparse RegionCell mapping field declarations (Phase M-3). Structure only; does not enable execution.
    #[serde(default)]
    pub region_fields: Vec<super::region_field::RegionFieldSpec>,
    /// Mapping execution opt-in profile. Default Disabled; spec presence alone does not enable runtime.
    #[serde(default)]
    pub mapping_execution_profile: super::region_field::MappingExecutionProfile,
}
