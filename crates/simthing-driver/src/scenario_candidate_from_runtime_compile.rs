//! SCENARIO-CANDIDATE-FROM-RUNTIME-0 — driver compile plan for candidate ScenarioSpec from runtime.

use simthing_spec::{
    evaluate_scenario_candidate_from_runtime_from_json_str, ScenarioCandidateFromRuntimeReport,
    SpecError,
};

use crate::loaded_scenario_runtime_report_chain_compile::{
    compile_loaded_scenario_runtime_report_chain_plan_from_json_str,
    LoadedScenarioRuntimeReportChainPlan,
};

/// Driver compile plan for candidate ScenarioSpec from loaded runtime property-view rows.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateFromRuntimePlan {
    pub runtime_report_chain_plan: LoadedScenarioRuntimeReportChainPlan,
    pub candidate_report: ScenarioCandidateFromRuntimeReport,

    pub candidate_scenario_spec_ready: bool,
    pub original_authority_preserved: bool,
    pub candidate_authority_changed: bool,
    pub mutation_record_count: u32,

    pub gpu_compatible_source_rows: bool,
    pub cpu_candidate_serialization_only: bool,

    pub candidate_save_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Compile candidate ScenarioSpec plan from JSON, composing #840 runtime report chain.
pub fn compile_scenario_candidate_from_runtime_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCandidateFromRuntimePlan, SpecError> {
    let runtime_report_chain_plan =
        compile_loaded_scenario_runtime_report_chain_plan_from_json_str(source_label, json)?;
    let candidate_report =
        evaluate_scenario_candidate_from_runtime_from_json_str(source_label, json)?;

    Ok(ScenarioCandidateFromRuntimePlan {
        candidate_scenario_spec_ready: candidate_report.candidate_scenario_spec_ready,
        original_authority_preserved: candidate_report.original_authority_preserved,
        candidate_authority_changed: candidate_report.candidate_authority_changed,
        mutation_record_count: candidate_report.mutation_record_count,
        gpu_compatible_source_rows: candidate_report.gpu_compatible_source_rows,
        cpu_candidate_serialization_only: candidate_report.cpu_candidate_serialization_only,
        candidate_save_deferred: candidate_report.candidate_save_deferred,
        savefile_persistence_deferred: candidate_report.savefile_persistence_deferred,
        persistent_history_deferred: candidate_report.persistent_history_deferred,
        studio_ui_wiring_deferred: candidate_report.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: candidate_report.gpu_dispatch_deferred,
        runtime_report_chain_plan,
        candidate_report,
    })
}
