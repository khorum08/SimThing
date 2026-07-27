use crate::designer_admission::{
    ClauseSpecFrontierV2Scenario, DesignerAdmissionPreflightManifest, MobilityScenario0Packet,
    V78LineScenarioPack,
};
use crate::error::SpecError;
use crate::spec::capability::CapabilityTreeSpec;
use crate::spec::eml_gadget::EmlGadgetStackSpec;
use crate::spec::first_slice_scenario::FirstSliceScenarioSpec;
use crate::spec::game_mode::GameModeSpec;
use crate::spec::region_field::RegionFieldSpec;

pub fn deserialize_game_mode_ron(text: &str) -> Result<GameModeSpec, SpecError> {
    let mut game_mode: GameModeSpec =
        ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))?;
    for class in &mut game_mode.order_weight_classes {
        class.source_span_token = record_scalar_position(text, &class.id, "magnitude:");
    }
    for overlay in &mut game_mode.overlays {
        overlay.source_span_token = if overlay.order_weight_class.is_some() {
            record_scalar_position(text, &overlay.id, "order_weight_class:")
        } else {
            record_transform_scalar_position(text, &overlay.id)
        };
    }
    for pack in &mut game_mode.domain_packs {
        for overlay in &mut pack.overlays {
            overlay.source_span_token = if overlay.order_weight_class.is_some() {
                record_scalar_position(text, &overlay.id, "order_weight_class:")
            } else {
                record_transform_scalar_position(text, &overlay.id)
            };
        }
    }
    Ok(game_mode)
}

fn record_start(text: &str, id: &str) -> Option<usize> {
    text.find(&format!("id: \"{id}\""))
}

fn record_scalar_position(text: &str, id: &str, field: &str) -> Option<usize> {
    let start = record_start(text, id)?;
    let relative = text[start..].find(field)?;
    let field_end = start + relative + field.len();
    Some(
        field_end
            + text[field_end..]
                .bytes()
                .position(|byte| !byte.is_ascii_whitespace())?,
    )
}

fn record_transform_scalar_position(text: &str, id: &str) -> Option<usize> {
    let start = record_start(text, id)?;
    let deltas = text[start..].find("sub_field_deltas:")?;
    let search_start = start + deltas;
    ["Add(", "Set(", "Multiply("]
        .into_iter()
        .filter_map(|op| text[search_start..].find(op).map(|at| (at, op.len())))
        .min_by_key(|(at, _)| *at)
        .map(|(at, op_len)| search_start + at + op_len)
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

pub fn deserialize_mobility_scenario0_packet_ron(
    text: &str,
) -> Result<MobilityScenario0Packet, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn serialize_mobility_scenario0_packet_ron(
    packet: &MobilityScenario0Packet,
) -> Result<String, SpecError> {
    ron::ser::to_string_pretty(packet, ron::ser::PrettyConfig::default())
        .map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn serialize_designer_admission_preflight_manifest_ron(
    manifest: &DesignerAdmissionPreflightManifest,
) -> Result<String, SpecError> {
    ron::ser::to_string_pretty(manifest, ron::ser::PrettyConfig::default())
        .map_err(|e| SpecError::RonParse(e.to_string()))
}
