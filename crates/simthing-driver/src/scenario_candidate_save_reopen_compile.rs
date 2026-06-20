//! SCENARIO-CANDIDATE-SAVE-REOPEN-0 — driver compile plan for candidate ScenarioSpec save/reopen.

use simthing_spec::{
    evaluate_scenario_candidate_save_reopen_from_json_str, ScenarioCandidateSaveReopenReport,
    SpecError,
};

use crate::scenario_candidate_from_runtime_compile::{
    compile_scenario_candidate_from_runtime_plan_from_json_str, ScenarioCandidateFromRuntimePlan,
};

/// Driver compile plan for candidate ScenarioSpec canonical save/reopen.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCandidateSaveReopenPlan {
    pub candidate_from_runtime_plan: ScenarioCandidateFromRuntimePlan,
    pub save_reopen_report: ScenarioCandidateSaveReopenReport,

    pub candidate_save_reopen_ready: bool,
    pub original_authority_preserved: bool,
    pub candidate_digest_stable_after_reopen: bool,
    pub stead_tree_projection_ready: bool,

    pub canonical_scenario_json_only: bool,
    pub no_distinct_savefile_format_introduced: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Compile candidate ScenarioSpec save/reopen plan from JSON, composing #842 candidate path.
pub fn compile_scenario_candidate_save_reopen_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCandidateSaveReopenPlan, SpecError> {
    let candidate_from_runtime_plan =
        compile_scenario_candidate_from_runtime_plan_from_json_str(source_label, json)?;
    let save_reopen_report =
        evaluate_scenario_candidate_save_reopen_from_json_str(source_label, json)?;

    Ok(ScenarioCandidateSaveReopenPlan {
        candidate_save_reopen_ready: save_reopen_report.candidate_save_reopen_ready,
        original_authority_preserved: save_reopen_report.original_authority_preserved,
        candidate_digest_stable_after_reopen: save_reopen_report
            .candidate_digest_stable_after_reopen,
        stead_tree_projection_ready: save_reopen_report.candidate_stead_tree_projection_ready,
        canonical_scenario_json_only: save_reopen_report.canonical_scenario_json_only,
        no_distinct_savefile_format_introduced: save_reopen_report
            .no_distinct_savefile_format_introduced,
        persistent_history_deferred: save_reopen_report.persistent_history_deferred,
        studio_ui_wiring_deferred: save_reopen_report.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: save_reopen_report.gpu_dispatch_deferred,
        candidate_from_runtime_plan,
        save_reopen_report,
    })
}
