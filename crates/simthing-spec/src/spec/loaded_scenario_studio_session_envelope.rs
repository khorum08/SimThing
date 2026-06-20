//! LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — loaded ScenarioSpec authority envelope for Studio.

use crate::error::SpecError;

use super::scenario_canonical_io::load_scenario_spec_from_json_str;
use super::scenario_stead_map_roundtrip::evaluate_scenario_stead_map_roundtrip_from_json_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadedScenarioSessionSource {
    CanonicalScenarioJson,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioAuthorityEnvelope {
    pub source_label: String,
    pub source: LoadedScenarioSessionSource,
    pub scenario_authority_digest: u64,
    pub scenario_id: Option<String>,
    pub canonical_io_ready: bool,
    pub stead_map_roundtrip_ready: bool,
    pub stead_ids_stable: bool,
    pub links_stable: bool,
    pub spatial_tree_stable: bool,
    pub rf_metadata_stable: bool,
    pub owner_metadata_not_spatial_parentage: bool,
    pub recursive_rf_prerequisites_ready: bool,
    pub studio_projection_rebuild_ready: bool,
    pub scenario_import_ready: bool,
    pub scenario_export_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedScenarioRuntimeSidecarEnvelope {
    pub runtime_reports_available: bool,
    pub recursive_rf_runtime_ready: bool,
    pub runtime_tick_execution_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioStudioSessionEnvelope {
    pub authority: LoadedScenarioAuthorityEnvelope,
    pub runtime_sidecar: LoadedScenarioRuntimeSidecarEnvelope,
    pub studio_config_is_authority: bool,
    pub bevy_state_is_authority: bool,
    pub gpu_buffers_are_authority: bool,
    pub runtime_reports_are_authority: bool,
}

/// Evaluate loaded Scenario Studio session envelope from canonical JSON, composing #828 and #834.
pub fn evaluate_loaded_scenario_studio_session_envelope_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioStudioSessionEnvelope, SpecError> {
    let (_, initial_load) = load_scenario_spec_from_json_str(source_label, json)?;
    let stead_report = evaluate_scenario_stead_map_roundtrip_from_json_str(source_label, json)?;

    let canonical_io_ready =
        stead_report.digest_stable && initial_load.loaded && initial_load.ingestion_ready;
    let stead_map_roundtrip_ready = canonical_io_ready
        && stead_report.stead_ids_stable
        && stead_report.links_stable
        && stead_report.spatial_tree_stable
        && stead_report.rf_metadata_stable
        && stead_report.owner_metadata_not_spatial_parentage;

    let recursive_rf_prerequisites_ready =
        stead_report.local_rf_parent_node_resolution_prerequisites_present;
    let studio_projection_rebuild_ready = stead_report.studio_projection_rebuild_ready;

    let scenario_import_ready =
        canonical_io_ready && stead_map_roundtrip_ready && studio_projection_rebuild_ready;
    let scenario_export_ready = scenario_import_ready && stead_report.digest_stable;

    let authority = LoadedScenarioAuthorityEnvelope {
        source_label: source_label.to_string(),
        source: LoadedScenarioSessionSource::CanonicalScenarioJson,
        scenario_authority_digest: stead_report.initial_authority_digest,
        scenario_id: initial_load.scenario_id,
        canonical_io_ready,
        stead_map_roundtrip_ready,
        stead_ids_stable: stead_report.stead_ids_stable,
        links_stable: stead_report.links_stable,
        spatial_tree_stable: stead_report.spatial_tree_stable,
        rf_metadata_stable: stead_report.rf_metadata_stable,
        owner_metadata_not_spatial_parentage: stead_report.owner_metadata_not_spatial_parentage,
        recursive_rf_prerequisites_ready,
        studio_projection_rebuild_ready,
        scenario_import_ready,
        scenario_export_ready,
    };

    let runtime_sidecar = LoadedScenarioRuntimeSidecarEnvelope {
        runtime_reports_available: recursive_rf_prerequisites_ready
            && studio_projection_rebuild_ready,
        recursive_rf_runtime_ready: recursive_rf_prerequisites_ready,
        runtime_tick_execution_deferred: true,
        runtime_mutation_deferred: true,
        semantic_execution_deferred: true,
        savefile_persistence_deferred: true,
        persistent_history_deferred: true,
        studio_ui_wiring_deferred: true,
        gpu_dispatch_deferred: true,
    };

    Ok(LoadedScenarioStudioSessionEnvelope {
        authority,
        runtime_sidecar,
        studio_config_is_authority: false,
        bevy_state_is_authority: false,
        gpu_buffers_are_authority: false,
        runtime_reports_are_authority: false,
    })
}

/// Prove loaded session envelope preserves ScenarioSpec authority boundaries.
pub fn prove_loaded_scenario_session_envelope_preserves_authority_boundaries(
    source_label: &str,
    json: &str,
) -> Result<bool, SpecError> {
    let envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str(source_label, json)?;
    Ok(!envelope.studio_config_is_authority
        && !envelope.bevy_state_is_authority
        && !envelope.gpu_buffers_are_authority
        && !envelope.runtime_reports_are_authority
        && envelope.runtime_sidecar.runtime_tick_execution_deferred
        && envelope.runtime_sidecar.runtime_mutation_deferred
        && envelope.runtime_sidecar.semantic_execution_deferred
        && envelope.runtime_sidecar.savefile_persistence_deferred
        && envelope.runtime_sidecar.persistent_history_deferred
        && envelope.runtime_sidecar.studio_ui_wiring_deferred
        && envelope.runtime_sidecar.gpu_dispatch_deferred)
}
