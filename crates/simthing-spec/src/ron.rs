use crate::designer_admission::{
    ClauseSpecFrontierV2Scenario, DesignerAdmissionPreflightManifest, V78LineScenarioPack,
};
use crate::error::SpecError;
use crate::spec::capability::CapabilityTreeSpec;
use crate::spec::eml_gadget::EmlGadgetStackSpec;
use crate::spec::first_slice_scenario::FirstSliceScenarioSpec;
use crate::spec::game_mode::GameModeSpec;
use crate::spec::region_field::RegionFieldSpec;

pub fn deserialize_game_mode_ron(text: &str) -> Result<GameModeSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_capability_tree_ron(text: &str) -> Result<CapabilityTreeSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_region_field_ron(text: &str) -> Result<RegionFieldSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_first_slice_scenario_ron(
    text: &str,
) -> Result<FirstSliceScenarioSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_eml_gadget_stack_ron(text: &str) -> Result<EmlGadgetStackSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_designer_admission_preflight_manifest_ron(
    text: &str,
) -> Result<DesignerAdmissionPreflightManifest, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_clause_spec_frontier_v2_scenario_ron(
    text: &str,
) -> Result<ClauseSpecFrontierV2Scenario, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn serialize_clause_spec_frontier_v2_scenario_ron(
    scenario: &ClauseSpecFrontierV2Scenario,
) -> Result<String, SpecError> {
    ron::ser::to_string_pretty(scenario, ron::ser::PrettyConfig::default())
        .map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_v7_8_line_scenario_pack_ron(
    text: &str,
) -> Result<V78LineScenarioPack, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn serialize_v7_8_line_scenario_pack_ron(
    pack: &V78LineScenarioPack,
) -> Result<String, SpecError> {
    ron::ser::to_string_pretty(pack, ron::ser::PrettyConfig::default())
        .map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn serialize_designer_admission_preflight_manifest_ron(
    manifest: &DesignerAdmissionPreflightManifest,
) -> Result<String, SpecError> {
    ron::ser::to_string_pretty(manifest, ron::ser::PrettyConfig::default())
        .map_err(|e| SpecError::RonParse(e.to_string()))
}
