//! LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 — driver compile plan for loaded scenario session envelope.

use simthing_spec::{
    evaluate_loaded_scenario_studio_session_envelope_from_json_str,
    LoadedScenarioStudioSessionEnvelope, SpecError,
};

use crate::scenario_canonical_io_compile::ScenarioCanonicalIoPlan;
use crate::scenario_stead_map_roundtrip_compile::{
    compile_scenario_stead_map_roundtrip_plan_from_json_str, ScenarioSteadMapRoundtripPlan,
};

/// Driver compile plan for loaded Scenario Studio session envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioStudioSessionEnvelopePlan {
    pub canonical_io_plan: ScenarioCanonicalIoPlan,
    pub stead_map_roundtrip_plan: ScenarioSteadMapRoundtripPlan,
    pub session_envelope: LoadedScenarioStudioSessionEnvelope,
    pub scenario_import_ready: bool,
    pub scenario_export_ready: bool,
    pub studio_projection_rebuild_ready: bool,
    pub recursive_rf_prerequisites_ready: bool,
    pub runtime_tick_execution_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Compile loaded Scenario Studio session envelope plan from JSON, composing #828 and #834.
pub fn compile_loaded_scenario_studio_session_envelope_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioStudioSessionEnvelopePlan, SpecError> {
    let stead_map_roundtrip_plan =
        compile_scenario_stead_map_roundtrip_plan_from_json_str(source_label, json)?;
    let canonical_io_plan = stead_map_roundtrip_plan.canonical_io_plan.clone();
    let session_envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str(source_label, json)?;

    Ok(LoadedScenarioStudioSessionEnvelopePlan {
        scenario_import_ready: session_envelope.authority.scenario_import_ready,
        scenario_export_ready: session_envelope.authority.scenario_export_ready,
        studio_projection_rebuild_ready: session_envelope.authority.studio_projection_rebuild_ready,
        recursive_rf_prerequisites_ready: session_envelope
            .authority
            .recursive_rf_prerequisites_ready,
        runtime_tick_execution_deferred: session_envelope
            .runtime_sidecar
            .runtime_tick_execution_deferred,
        runtime_mutation_deferred: session_envelope.runtime_sidecar.runtime_mutation_deferred,
        savefile_persistence_deferred: session_envelope
            .runtime_sidecar
            .savefile_persistence_deferred,
        persistent_history_deferred: session_envelope.runtime_sidecar.persistent_history_deferred,
        studio_ui_wiring_deferred: session_envelope.runtime_sidecar.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: session_envelope.runtime_sidecar.gpu_dispatch_deferred,
        canonical_io_plan,
        stead_map_roundtrip_plan,
        session_envelope,
    })
}
