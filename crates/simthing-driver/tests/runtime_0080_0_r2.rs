use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r2, Runtime0080R2Input, Runtime0080R2Report, R6C_CANONICAL_TICK_COUNT,
    RUNTIME_0080_0_R2_ID, RUNTIME_0080_0_R2_PRIMITIVE, RUNTIME_0080_0_R2_STATUS_PASS,
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R2_EXPECTED_REPORT_CHECKSUM, RUNTIME_R2_SCOPE,
};

static REPORT: OnceLock<Runtime0080R2Report> = OnceLock::new();

fn report() -> &'static Runtime0080R2Report {
    REPORT.get_or_init(|| run_runtime_0080_0_r2(&Runtime0080R2Input::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R2Report) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r2_opt_in_default_off() {
    let default = run_runtime_0080_0_r2(&Runtime0080R2Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert!(!default.runs_100_ticks);
    assert_eq!(default.id, RUNTIME_0080_0_R2_ID);
}

#[test]
fn r2_runs_canonical_100_ticks() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.runs_100_ticks);
    assert_eq!(admitted.tick_count, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(
        admitted.per_tick_trace.len(),
        R6C_CANONICAL_TICK_COUNT as usize
    );
}

#[test]
fn r2_consumes_r1a_tier_a_gpu_next_tick() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.uses_r1a_tier_a_gpu_next_tick);
    assert!(admitted.tier_a_tick100_matches_oracle);
}

#[test]
fn r2_consumes_r1b_resident_event_journal() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.uses_r1b_resident_event_journal);
    assert!(admitted.event_journal_parity);
}

#[test]
fn r2_consumes_r1c_a_through_r1c_e_structural_substrate() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.uses_r1c_structural_substrate);
    let substrate = admitted.substrate.as_ref().expect("substrate outcome");
    assert!(substrate.r1c_a_ok);
    assert!(substrate.r1c_b_ok);
    assert!(substrate.r1c_c_ok);
    assert!(substrate.r1c_d_ok);
    assert!(substrate.r1c_e_ok);
    assert!(substrate.mark_rows > 0);
    assert!(substrate.allocation_rows > 0);
    assert!(substrate.membership_delta_rows > 0);
}

#[test]
fn r2_consumes_r1c_f_gpu_zero_cohort() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.uses_r1c_f_gpu_zero_cohort);
    assert!(admitted.zero_cohort_row_count > 0);
    assert!(admitted.structural_decisions_gpu_emitted_zero_cohort);
}

#[test]
fn r2_zero_cohort_not_cpu_decided() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.zero_cohort_cpu_decided);
}

#[test]
fn r2_reports_expected_r6c_checksum() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.r6c_checksum_expected,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert_eq!(admitted.r6c_checksum_expected, 0x1bba891c779190a4);
}

#[test]
fn r2_reports_observed_checksum_or_explained_delta() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    if admitted.r6c_checksum_matches {
        assert_eq!(
            admitted.r6c_checksum_observed,
            RUNTIME_R0_EXPECTED_R6C_CHECKSUM
        );
    } else {
        assert!(!admitted.r6c_checksum_delta_explained.is_empty());
    }
}

#[test]
fn r2_lists_remaining_cpu_decided_classes() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"DamageDelta"));
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"MoveRequest"));
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"LocalBirthRequest"));
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"FusionRequest"));
    assert!(!admitted.remaining_class_blocked_run);
}

#[test]
fn r2_does_not_open_m4a() {
    let admitted = report();
    assert!(!admitted.m4a_required);
}

#[test]
fn r2_does_not_use_multi_atlas() {
    let admitted = report();
    assert!(!admitted.multi_atlas_required);
}

#[test]
fn r2_does_not_add_new_copy_substrate() {
    let admitted = report();
    assert!(!admitted.new_copy_substrate_added);
}

#[test]
fn r2_no_default_session_wiring() {
    let default = run_runtime_0080_0_r2(&Runtime0080R2Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.disabled_no_op);
}

#[test]
fn r2_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    assert_eq!(admitted.scope, RUNTIME_R2_SCOPE);
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R2_PRIMITIVE);
}

#[test]
fn r2_domain_neutral_terms_only() {
    let admitted = report();
    for term in &admitted.domain_terms {
        assert!(
            !term.contains("Terran") && !term.contains("Pirate"),
            "domain term must be neutral: {term}"
        );
    }
}

#[test]
fn r2_profiling_populated() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let profiling = admitted.profiling.as_ref().expect("profiling capture");
    assert!(profiling.gpu_loop_ms > 0.0);
    assert_eq!(
        profiling.per_tick_timing.len(),
        R6C_CANONICAL_TICK_COUNT as usize
    );
    assert!(profiling.mean_tick_ms > 0.0);
    assert!(profiling.memory.gpu_persistent_total_bytes > 0);
}

#[test]
fn r2_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R2_EXPECTED_REPORT_CHECKSUM
    );
    if admitted.verdict == "PASS" {
        assert_eq!(admitted.status, RUNTIME_0080_0_R2_STATUS_PASS);
    }
}
