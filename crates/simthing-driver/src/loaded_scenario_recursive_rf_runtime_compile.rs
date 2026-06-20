//! LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 — driver compile plan for loaded scenario recursive RF runtime.

use simthing_spec::{
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str,
    LoadedScenarioRecursiveRfRuntimeReport, SpecError,
};

use crate::loaded_scenario_studio_session_envelope_compile::{
    compile_loaded_scenario_studio_session_envelope_plan_from_json_str,
    LoadedScenarioStudioSessionEnvelopePlan,
};

/// Driver compile plan for loaded scenario recursive RF runtime surface.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRecursiveRfRuntimePlan {
    pub session_envelope_plan: LoadedScenarioStudioSessionEnvelopePlan,
    pub recursive_rf_runtime_report: LoadedScenarioRecursiveRfRuntimeReport,
    pub recursive_rf_runtime_ready: bool,
    pub local_parent_node_resolution_first: bool,
    pub sibling_settlement_before_upward_bubbling: bool,
    pub owner_scope_not_spatial_parentage: bool,
    pub gpu_compatible_row_table_surface: bool,
    pub cpu_oracle_only: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Compile loaded scenario recursive RF runtime plan from JSON, composing #836 session envelope.
pub fn compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioRecursiveRfRuntimePlan, SpecError> {
    let session_envelope_plan =
        compile_loaded_scenario_studio_session_envelope_plan_from_json_str(source_label, json)?;
    let recursive_rf_runtime_report =
        evaluate_loaded_scenario_recursive_rf_runtime_from_json_str(source_label, json)?;

    Ok(LoadedScenarioRecursiveRfRuntimePlan {
        recursive_rf_runtime_ready: recursive_rf_runtime_report.recursive_rf_runtime_ready,
        local_parent_node_resolution_first: recursive_rf_runtime_report
            .local_parent_node_resolution_first,
        sibling_settlement_before_upward_bubbling: recursive_rf_runtime_report
            .sibling_settlement_before_upward_bubbling,
        owner_scope_not_spatial_parentage: recursive_rf_runtime_report
            .owner_scope_not_spatial_parentage,
        gpu_compatible_row_table_surface: recursive_rf_runtime_report
            .gpu_compatible_row_table_surface,
        cpu_oracle_only: recursive_rf_runtime_report.cpu_oracle_only,
        scenario_authority_mutation_deferred: recursive_rf_runtime_report
            .scenario_authority_mutation_deferred,
        runtime_mutation_deferred: recursive_rf_runtime_report.runtime_mutation_deferred,
        savefile_persistence_deferred: recursive_rf_runtime_report.savefile_persistence_deferred,
        persistent_history_deferred: recursive_rf_runtime_report.persistent_history_deferred,
        studio_ui_wiring_deferred: recursive_rf_runtime_report.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: recursive_rf_runtime_report.gpu_dispatch_deferred,
        session_envelope_plan,
        recursive_rf_runtime_report,
    })
}
