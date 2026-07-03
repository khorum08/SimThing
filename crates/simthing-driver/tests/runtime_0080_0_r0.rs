use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r0, run_dress_rehearsal_r6c_integrated_run, run_runtime_0080_0_r0,
    DressRehearsalR6cInput, Runtime0080R0Input, Runtime0080R0Report, R6C_CANONICAL_TICK_COUNT,
    RUNTIME_0080_0_R0_ID, RUNTIME_0080_0_R0_STATUS_PARTIAL, RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
    RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND, RUNTIME_R0_SUBSTRATE_GAP,
    RUNTIME_R0_WHOLE_RUN_GPU_MEASURED, RUNTIME_R0_WHOLE_RUN_PARTIAL,
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
}

#[test]
fn runtime_0080_r0_foreground_diagnostics_documented() {
    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(
        admitted.foreground_capture_method,
        RUNTIME_R0_FOREGROUND_CAPTURE
    );
    assert!(admitted
        .foreground_capture_method
        .contains("no stdout/stderr redirection"));
}

#[test]
fn runtime_0080_r0_cpu_oracle_is_not_observed_run_driver_if_claiming_gpu_measured() {
    let admitted = report();
    assert!(admitted.cpu_is_tick_authority);
    if admitted.r6c_whole_run_gpu_posture == RUNTIME_R0_WHOLE_RUN_GPU_MEASURED {
        panic!("whole-run GPU-measured claim requires CPU to not be tick authority");
    }
}

#[test]
fn runtime_0080_r0_gpu_state_feeds_next_tick_if_claiming_gpu_measured() {
    let admitted = report();
    if admitted.r6c_whole_run_gpu_posture == RUNTIME_R0_WHOLE_RUN_GPU_MEASURED {
        assert!(
            admitted.gpu_state_feeds_next_tick,
            "GPU-measured claim requires gpu_state_feeds_next_tick"
        );
    } else {
        assert!(
            !admitted.gpu_state_feeds_next_tick,
            "mirror dispatch must not claim GPU feeds next tick"
        );
    }
}

#[test]
fn runtime_0080_r0_upload_after_cpu_tick_is_not_sufficient_for_whole_run_measured() {
    let admitted = report();
    assert!(admitted.mirror_dispatch_after_cpu_tick);
    assert!(admitted.cpu_is_tick_authority);
    assert_ne!(
        admitted.r6c_whole_run_gpu_posture,
        RUNTIME_R0_WHOLE_RUN_GPU_MEASURED
    );
}

#[test]
fn runtime_0080_r0_reports_partial_when_cpu_world_is_tick_authority() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PARTIAL");
    assert_eq!(admitted.status, RUNTIME_0080_0_R0_STATUS_PARTIAL);
    assert!(admitted.cpu_is_tick_authority);
    assert_eq!(
        admitted.r6c_whole_run_gpu_posture,
        RUNTIME_R0_WHOLE_RUN_PARTIAL
    );
}

#[test]
fn runtime_0080_r0_reports_measured_only_when_gpu_state_is_tick_authority() {
    let admitted = report();
    if admitted.gpu_state_feeds_next_tick {
        assert_eq!(
            admitted.r6c_whole_run_gpu_posture,
            RUNTIME_R0_WHOLE_RUN_GPU_MEASURED
        );
    } else {
        assert_ne!(
            admitted.r6c_whole_run_gpu_posture,
            RUNTIME_R0_WHOLE_RUN_GPU_MEASURED
        );
    }
}

#[test]
fn runtime_0080_r0_preserves_existing_gpu_shape_dispatch_evidence() {
    let admitted = report();
    assert!(admitted.gpu_resident_across_ticks);
    assert_eq!(admitted.ticks_scheduled, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(admitted.dispatch_r1_ticks, 100);
    assert_eq!(admitted.dispatch_r2_ticks, 100);
    assert_eq!(admitted.dispatch_r4_ticks, 100);
    assert_eq!(admitted.dispatch_r6_ticks, 100);
    assert_eq!(admitted.dispatch_r6b_ticks, 100);
    assert_eq!(admitted.inter_tick_world_readbacks, 0);
    assert_eq!(admitted.residency_trace.len(), 100);
}

#[test]
fn runtime_0080_r0_r6c_checksum_parity_still_verified() {
    let admitted = report();
    assert_eq!(
        admitted.r6c_checksum_expected,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert_eq!(
        admitted.r6c_checksum_observed,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert!(admitted.integer_trajectory_bit_exact);
    assert!(admitted.cpu_oracle_parity);
}

#[test]
fn runtime_0080_r0_holds_world_state_gpu_resident_across_ticks() {
    let admitted = report();
    assert!(admitted.gpu_resident_across_ticks);
}

#[test]
fn runtime_0080_r0_r4_f32_within_accepted_bound() {
    let admitted = report();
    assert!(admitted.r4_within_bound);
    assert!(admitted.r4_max_abs_delta <= RUNTIME_R0_R4_F32_BOUND);
}

#[test]
fn runtime_0080_r0_no_new_semantic_wgsl_or_new_op() {
    let admitted = report();
    assert!(!admitted.new_semantic_wgsl);
    assert!(!admitted.new_accumulator_op);
    assert!(!admitted.scenario_reopened);
    let r6c = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    assert_eq!(r6c.summary.stable_checksum, admitted.r6c_checksum_observed);
}

#[test]
fn runtime_0080_r0_no_request_atlas_batching_or_m4a() {
    let admitted = report();
    assert!(!admitted.request_atlas_batching);
    assert!(!admitted.m4a_masking_at_scale);
    assert_eq!(admitted.id, RUNTIME_0080_0_R0_ID);
    assert!(!admitted.substrate_gap_for_true_pass.is_empty());
    assert_eq!(
        admitted.substrate_gap_for_true_pass,
        RUNTIME_R0_SUBSTRATE_GAP
    );
}

#[test]
fn runtime_0080_r0_report_checksum_stable() {
    let (left, right) = replay_runtime_0080_0_r0();
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_ne!(left.stable_report_checksum, 0);
    assert_eq!(left.verdict, "PARTIAL");
    assert_eq!(right.verdict, "PARTIAL");
}
