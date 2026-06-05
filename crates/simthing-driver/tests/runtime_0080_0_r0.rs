use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r0, run_dress_rehearsal_r6c_integrated_run, run_runtime_0080_0_r0,
    DressRehearsalR6cInput, Runtime0080R0Input, Runtime0080R0Report, RUNTIME_0080_0_R0_ID,
    RUNTIME_0080_0_R0_STATUS_PASS, RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_GPU_BLOCKED,
    RUNTIME_R0_R4_F32_BOUND, RUNTIME_R0_WHOLE_RUN_GPU_MEASURED, RUNTIME_R0_WHOLE_RUN_UNMEASURED,
    R6C_CANONICAL_TICK_COUNT,
};

static REPORT: OnceLock<Runtime0080R0Report> = OnceLock::new();

fn report() -> &'static Runtime0080R0Report {
    REPORT.get_or_init(|| run_runtime_0080_0_r0(&Runtime0080R0Input::explicit_opt_in()))
}

#[test]
fn runtime_0080_r0_opt_in_default_off() {
    let default = run_runtime_0080_0_r0(&Runtime0080R0Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.ticks_scheduled, 0);

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert_eq!(admitted.id, RUNTIME_0080_0_R0_ID);
}

#[test]
fn runtime_0080_r0_selects_discrete_gpu_or_blocks_honestly() {
    let default_report = run_runtime_0080_0_r0(&Runtime0080R0Input::default_simsession());
    assert!(default_report.adapter.is_none());

    let measured = report();
    assert!(
        measured.admitted,
        "RUNTIME-0080-0-R0 must fail honestly when no discrete GPU is available: {:?}",
        measured.diagnostics
    );
    let adapter = measured.adapter.as_ref().expect("adapter report");
    assert!(adapter.selected_discrete_gpu);
    assert!(!adapter.adapter_name.is_empty());
}

#[test]
fn runtime_0080_r0_seeds_r6c_world_once() {
    let admitted = report();
    assert_eq!(admitted.ticks_scheduled, R6C_CANONICAL_TICK_COUNT);
    assert!(admitted.single_galactic_tier);
    assert!(admitted.single_resident_theater);
}

#[test]
fn runtime_0080_r0_holds_world_state_gpu_resident_across_ticks() {
    let admitted = report();
    assert!(admitted.gpu_resident_across_ticks);
    assert_eq!(admitted.ticks_scheduled, 100);
}

#[test]
fn runtime_0080_r0_dispatches_r1_disruption_shape_on_gpu() {
    let admitted = report();
    assert_eq!(admitted.dispatch_r1_ticks, 100);
}

#[test]
fn runtime_0080_r0_dispatches_r2_economy_shape_on_gpu() {
    let admitted = report();
    assert_eq!(admitted.dispatch_r2_ticks, 100);
}

#[test]
fn runtime_0080_r0_dispatches_r4_gradient_magnitude_shape_on_gpu() {
    let admitted = report();
    assert_eq!(admitted.dispatch_r4_ticks, 100);
}

#[test]
fn runtime_0080_r0_dispatches_r6_combat_attrition_shape_on_gpu() {
    let admitted = report();
    assert_eq!(admitted.dispatch_r6_ticks, 100);
}

#[test]
fn runtime_0080_r0_dispatches_r6b_construction_fusion_shape_on_gpu() {
    let admitted = report();
    assert_eq!(admitted.dispatch_r6b_ticks, 100);
}

#[test]
fn runtime_0080_r0_avoids_intermediate_cpu_state_readback_between_ticks() {
    let admitted = report();
    assert_eq!(admitted.inter_tick_world_readbacks, 0);
}

#[test]
fn runtime_0080_r0_runs_100_tick_scheduler() {
    let admitted = report();
    assert_eq!(admitted.ticks_scheduled, 100);
    assert_eq!(admitted.residency_trace.len(), 100);
}

#[test]
fn runtime_0080_r0_matches_r6c_cpu_oracle_checksum() {
    let admitted = report();
    assert_eq!(admitted.r6c_checksum_expected, RUNTIME_R0_EXPECTED_R6C_CHECKSUM);
    assert_eq!(admitted.r6c_checksum_observed, RUNTIME_R0_EXPECTED_R6C_CHECKSUM);
}

#[test]
fn runtime_0080_r0_preserves_integer_trajectory_bit_exact() {
    let admitted = report();
    assert!(admitted.integer_trajectory_bit_exact);
    assert!(admitted.cpu_oracle_parity);
}

#[test]
fn runtime_0080_r0_r4_f32_within_accepted_bound() {
    let admitted = report();
    assert!(admitted.r4_within_bound);
    assert!(admitted.r4_max_abs_delta <= RUNTIME_R0_R4_F32_BOUND);
}

#[test]
fn runtime_0080_r0_reports_residency_trace() {
    let admitted = report();
    assert!(!admitted.residency_trace.is_empty());
    assert_eq!(
        admitted.residency_trace.first().map(|row| row.tick),
        Some(0)
    );
    assert_eq!(
        admitted.residency_trace.last().map(|row| row.tick),
        Some(99)
    );
}

#[test]
fn runtime_0080_r0_no_request_atlas_batching() {
    let admitted = report();
    assert!(!admitted.request_atlas_batching);
    assert!(!admitted.m4a_masking_at_scale);
}

#[test]
fn runtime_0080_r0_no_new_semantic_wgsl_or_new_op() {
    let admitted = report();
    assert!(!admitted.new_semantic_wgsl);
    assert!(!admitted.new_accumulator_op);
}

#[test]
fn runtime_0080_r0_no_scenario_reopen_or_behavior_change() {
    let admitted = report();
    assert!(!admitted.scenario_reopened);
    let r6c = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    assert_eq!(r6c.summary.stable_checksum, admitted.r6c_checksum_observed);
}

#[test]
fn runtime_0080_r0_report_checksum_stable() {
    let (left, right) = replay_runtime_0080_0_r0();
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_ne!(left.stable_report_checksum, 0);
}

#[test]
fn runtime_0080_r0_whole_run_gpu_posture_measured_when_pass() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert!(RUNTIME_0080_0_R0_STATUS_PASS.contains("IMPLEMENTED / PASS"));
    assert_eq!(admitted.r6c_whole_run_gpu_posture, RUNTIME_R0_WHOLE_RUN_GPU_MEASURED);
    assert_ne!(admitted.r6c_whole_run_gpu_posture, RUNTIME_R0_WHOLE_RUN_UNMEASURED);
    assert_ne!(admitted.r6c_whole_run_gpu_posture, RUNTIME_R0_GPU_BLOCKED);
}
