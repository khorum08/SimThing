//! SCENARIO-CANDIDATE-FROM-RUNTIME-0 — candidate ScenarioSpec from loaded runtime property-view rows.
//!
//! GPU-residency doctrine: runtime property-view rows are GPU-compatible source data.
//! CPU space is limited to candidate clone serialization/bookkeeping — not production simulation authority.

use crate::error::SpecError;

use super::channel_key::{OwnerRef, ResourceKey, ScopeId};
use super::loaded_scenario_runtime_report_chain::evaluate_loaded_scenario_runtime_report_chain_from_json_str;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario_canonical_io::load_scenario_spec_from_json_str;
use super::scenario_property_mutation_authority_boundary::{
    clone_scenario_candidate_with_runtime_property_view,
    evaluate_scenario_property_mutation_authority_boundary,
    prove_scenario_property_mutation_boundary_preserves_original_authority,
    ScenarioPropertyMutationRecord, ScenarioPropertyMutationSourceMode,
};
use super::scenario_stead_map_roundtrip::{
    evaluate_scenario_stead_map_roundtrip_from_json_str, extract_scenario_spatial_tree_rows,
    extract_scenario_stead_id_rows, extract_scenario_stead_link_rows,
};

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);
const REPLAY_ONE: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioCandidateFromRuntimeSource {
    LoadedScenarioRuntimeReportChain,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidatePropertyMutationRecord {
    pub participant_simthing_id_raw: u32,
    pub property_id: String,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub scope_id: Option<ScopeId>,
    pub before_value: Option<f64>,
    pub runtime_value: f64,
    pub after_value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateFromRuntimeReport {
    pub source: ScenarioCandidateFromRuntimeSource,
    pub source_label: String,

    pub original_authority_digest_before: u64,
    pub original_authority_digest_after: u64,
    pub original_authority_preserved: bool,

    pub candidate_authority_digest_before: u64,
    pub candidate_authority_digest_after: u64,
    pub candidate_authority_changed: bool,

    pub runtime_report_chain_ready: bool,
    pub runtime_property_view_rows_ready: bool,

    pub mutation_records: Vec<ScenarioCandidatePropertyMutationRecord>,
    pub mutation_record_count: u32,

    pub candidate_scenario_spec_ready: bool,
    pub candidate_stead_ids_preserved: bool,
    pub candidate_links_preserved: bool,
    pub candidate_spatial_tree_preserved: bool,
    pub owner_metadata_not_spatial_parentage: bool,

    pub gpu_compatible_source_rows: bool,
    pub cpu_candidate_serialization_only: bool,

    pub candidate_save_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Evaluate candidate ScenarioSpec report from canonical JSON, composing #840 runtime report chain.
pub fn evaluate_scenario_candidate_from_runtime_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCandidateFromRuntimeReport, SpecError> {
    let runtime_chain =
        evaluate_loaded_scenario_runtime_report_chain_from_json_str(source_label, json)?;
    if !runtime_chain.runtime_property_view_rows_ready {
        return Err(SpecError::ValidationFailed);
    }

    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let mutation_report = evaluate_scenario_property_mutation_authority_boundary(
        &scenario,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    let candidate =
        clone_scenario_candidate_with_runtime_property_view(&scenario, TICK_ONE, REPLAY_ONE)
            .map_err(|_| SpecError::ValidationFailed)?;

    let stead_report = evaluate_scenario_stead_map_roundtrip_from_json_str(source_label, json)?;
    let original_stead_ids = extract_scenario_stead_id_rows(&scenario)?;
    let original_links = extract_scenario_stead_link_rows(&scenario)?;
    let original_spatial_tree = extract_scenario_spatial_tree_rows(&scenario)?;
    let candidate_stead_ids = extract_scenario_stead_id_rows(&candidate)?;
    let candidate_links = extract_scenario_stead_link_rows(&candidate)?;
    let candidate_spatial_tree = extract_scenario_spatial_tree_rows(&candidate)?;

    let mutation_records = mutation_report
        .mutation_records
        .iter()
        .map(map_mutation_record)
        .collect::<Vec<_>>();

    let candidate_authority_digest_before = mutation_report.original_before_authority_digest;
    let candidate_authority_changed = mutation_report.candidate_scenario_mutated
        && mutation_report.mutation_record_count > 0
        && mutation_report.candidate_after_authority_digest != candidate_authority_digest_before;

    Ok(ScenarioCandidateFromRuntimeReport {
        source: ScenarioCandidateFromRuntimeSource::LoadedScenarioRuntimeReportChain,
        source_label: source_label.to_string(),
        original_authority_digest_before: mutation_report.original_before_authority_digest,
        original_authority_digest_after: mutation_report.original_after_authority_digest,
        original_authority_preserved: mutation_report.original_scenario_unchanged,
        candidate_authority_digest_before,
        candidate_authority_digest_after: mutation_report.candidate_after_authority_digest,
        candidate_authority_changed,
        runtime_report_chain_ready: runtime_chain.recursive_rf_runtime_ready
            && runtime_chain.owner_silo_ready
            && runtime_chain.local_allocation_ready
            && runtime_chain.local_effects_ready
            && runtime_chain.semantic_projection_ready
            && runtime_chain.semantic_execution_records_ready
            && runtime_chain.semantic_delta_preview_ready
            && runtime_chain.runtime_participant_state_rows_ready
            && runtime_chain.runtime_property_view_rows_ready,
        runtime_property_view_rows_ready: runtime_chain.runtime_property_view_rows_ready,
        mutation_record_count: mutation_report.mutation_record_count,
        mutation_records,
        candidate_scenario_spec_ready: mutation_report.candidate_property_mutation_applied
            && mutation_report.runtime_property_view_ready,
        candidate_stead_ids_preserved: original_stead_ids == candidate_stead_ids,
        candidate_links_preserved: original_links == candidate_links,
        candidate_spatial_tree_preserved: original_spatial_tree == candidate_spatial_tree,
        owner_metadata_not_spatial_parentage: stead_report.owner_metadata_not_spatial_parentage,
        gpu_compatible_source_rows: runtime_chain.gpu_compatible_row_table_surface
            && runtime_chain.runtime_property_view_rows_ready,
        cpu_candidate_serialization_only: true,
        candidate_save_deferred: true,
        savefile_persistence_deferred: mutation_report.savefile_mutation_deferred,
        persistent_history_deferred: mutation_report.persistent_history_deferred,
        studio_ui_wiring_deferred: runtime_chain.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: runtime_chain.gpu_dispatch_deferred,
    })
}

/// Prove loaded original ScenarioSpec authority is preserved after candidate evaluation.
pub fn prove_scenario_candidate_from_runtime_preserves_original_authority(
    source_label: &str,
    json: &str,
) -> Result<bool, SpecError> {
    let report = evaluate_scenario_candidate_from_runtime_from_json_str(source_label, json)?;
    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let mutation_proof = prove_scenario_property_mutation_boundary_preserves_original_authority(
        &scenario,
        TICK_ONE,
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable,
        REPLAY_ONE,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    Ok(mutation_proof
        && report.original_authority_preserved
        && report.original_authority_digest_before == report.original_authority_digest_after
        && report.candidate_save_deferred
        && report.cpu_candidate_serialization_only)
}

fn map_mutation_record(
    record: &ScenarioPropertyMutationRecord,
) -> ScenarioCandidatePropertyMutationRecord {
    ScenarioCandidatePropertyMutationRecord {
        participant_simthing_id_raw: record.participant_simthing_id_raw,
        property_id: record.property_id.clone(),
        owner_ref: Some(record.owner_ref.clone()),
        resource_key: Some(record.resource_key.clone()),
        scope_id: record.scope_id.map(|scope| ScopeId::new(scope.to_string())),
        before_value: record.before_value,
        runtime_value: record.runtime_property_view_value,
        after_value: record.candidate_after_value,
    }
}
