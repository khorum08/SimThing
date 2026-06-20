//! LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 — driver compile plan for loaded scenario runtime report chain.

use simthing_spec::{
    evaluate_loaded_scenario_runtime_report_chain_from_json_str,
    LoadedScenarioRuntimeReportChainReport, SpecError,
};

use crate::loaded_scenario_recursive_rf_runtime_compile::{
    compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str,
    LoadedScenarioRecursiveRfRuntimePlan,
};

/// Driver compile plan for loaded scenario runtime report chain.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRuntimeReportChainPlan {
    pub recursive_rf_runtime_plan: LoadedScenarioRecursiveRfRuntimePlan,
    pub runtime_report_chain: LoadedScenarioRuntimeReportChainReport,

    pub full_chain_ready: bool,
    pub owner_silo_ready: bool,
    pub local_allocation_ready: bool,
    pub local_effects_ready: bool,
    pub semantic_projection_ready: bool,
    pub semantic_execution_records_ready: bool,
    pub semantic_delta_preview_ready: bool,
    pub runtime_participant_state_rows_ready: bool,
    pub runtime_property_view_rows_ready: bool,

    pub gpu_compatible_row_table_surface: bool,
    pub cpu_oracle_only: bool,
    pub explicit_runtime_report_mode_only: bool,

    pub scenario_authority_mutation_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Compile loaded scenario runtime report chain plan from JSON, composing #838 recursive RF runtime.
pub fn compile_loaded_scenario_runtime_report_chain_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioRuntimeReportChainPlan, SpecError> {
    let recursive_rf_runtime_plan =
        compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str(source_label, json)?;
    let runtime_report_chain =
        evaluate_loaded_scenario_runtime_report_chain_from_json_str(source_label, json)?;

    let full_chain_ready = runtime_report_chain.recursive_rf_runtime_ready
        && runtime_report_chain.owner_silo_ready
        && runtime_report_chain.local_allocation_ready
        && runtime_report_chain.local_effects_ready
        && runtime_report_chain.semantic_projection_ready
        && runtime_report_chain.semantic_execution_records_ready
        && runtime_report_chain.semantic_delta_preview_ready
        && runtime_report_chain.runtime_participant_state_rows_ready
        && runtime_report_chain.runtime_property_view_rows_ready;

    Ok(LoadedScenarioRuntimeReportChainPlan {
        full_chain_ready,
        owner_silo_ready: runtime_report_chain.owner_silo_ready,
        local_allocation_ready: runtime_report_chain.local_allocation_ready,
        local_effects_ready: runtime_report_chain.local_effects_ready,
        semantic_projection_ready: runtime_report_chain.semantic_projection_ready,
        semantic_execution_records_ready: runtime_report_chain.semantic_execution_records_ready,
        semantic_delta_preview_ready: runtime_report_chain.semantic_delta_preview_ready,
        runtime_participant_state_rows_ready: runtime_report_chain
            .runtime_participant_state_rows_ready,
        runtime_property_view_rows_ready: runtime_report_chain.runtime_property_view_rows_ready,
        gpu_compatible_row_table_surface: runtime_report_chain.gpu_compatible_row_table_surface,
        cpu_oracle_only: runtime_report_chain.cpu_oracle_only,
        explicit_runtime_report_mode_only: runtime_report_chain.explicit_runtime_report_mode_only,
        scenario_authority_mutation_deferred: runtime_report_chain
            .scenario_authority_mutation_deferred,
        runtime_mutation_deferred: runtime_report_chain.runtime_mutation_deferred,
        savefile_persistence_deferred: runtime_report_chain.savefile_persistence_deferred,
        persistent_history_deferred: runtime_report_chain.persistent_history_deferred,
        studio_ui_wiring_deferred: runtime_report_chain.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: runtime_report_chain.gpu_dispatch_deferred,
        recursive_rf_runtime_plan,
        runtime_report_chain,
    })
}
