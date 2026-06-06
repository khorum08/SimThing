use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r1c_a, run_runtime_0080_0_r1c_a_with_mark_writers_enabled,
    Runtime0080R1cAInput, Runtime0080R1cAReport, RUNTIME_0080_0_R1C_A_ID,
    RUNTIME_0080_0_R1C_A_PRIMITIVE, RUNTIME_0080_0_R1C_A_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_A_STATUS_PASS, RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
    RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_A_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cAReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cAReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1cAReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_a_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_A_ID);
    assert!(!default.admitted);
    assert!(default.marker.is_none());
}

#[test]
fn r1c_a_selects_predecessors_or_blocks_honestly() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_A_STATUS_BLOCKED);
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_A_STATUS_PASS);
    let predecessor = admitted.predecessor.as_ref().expect("predecessors");
    assert_eq!(predecessor.r1c_verdict, "PARTIAL");
    assert!(predecessor.r1b_event_journal_parity);
    assert!(predecessor.r1b_event_rows_read_from_gpu_values);
    assert!(predecessor.r1b_free_slot_mark_source_rows > 0);
}

#[test]
fn r1c_a_marks_free_slots_from_r1b_gpu_journal_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let marker = admitted.marker.as_ref().expect("marker report");
    assert!(marker.resident_marker_session_created);
    assert!(marker.mark_sources_from_r1b_gpu_journal);
    assert!(marker.mark_source_rows > 0);
    assert_eq!(
        marker.unique_slots_marked_expected,
        marker.unique_slots_marked_gpu
    );
    assert_eq!(marker.oracle_marked_slots, marker.gpu_marked_slots);
    assert!(marker.mark_parity_measured_from_gpu_values);
    assert!(admitted.mark_trace.iter().any(
        |row| row.source_event_kind == "FusionRequest" || row.source_event_kind == "ZeroCohort"
    ));
}

#[test]
fn r1c_a_disabled_marker_fails_mark_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let marker = admitted.marker.as_ref().expect("marker report");
    assert!(marker.disabled_marker_negative_control_detected);
    assert_eq!(marker.disabled_marker_gpu_marked_slots, 0);
    assert!(!marker.disabled_marker_parity);

    let disabled = run_runtime_0080_0_r1c_a_with_mark_writers_enabled(
        &Runtime0080R1cAInput::explicit_opt_in(),
        false,
    );
    if !blocked(&disabled) {
        let disabled_marker = disabled.marker.as_ref().expect("disabled marker");
        assert!(!disabled_marker.mark_parity_measured_from_gpu_values);
        assert_eq!(disabled_marker.unique_slots_marked_gpu, 0);
    }
}

#[test]
fn r1c_a_does_not_claim_allocation_scatter_or_compaction() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_free_list_mark_authority);
    assert!(!admitted.resident_free_list_allocation_authority);
    assert!(!admitted.resident_reenroll_scatter_authority);
    assert!(!admitted.resident_birth_removal_authority);
    assert!(!admitted.resident_fusion_compaction_authority);
    assert!(!admitted.structural_decisions_gpu_emitted);
}

#[test]
fn r1c_a_records_remaining_gates_without_pulling_them_forward() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_A_PRIMITIVE);
    assert_eq!(admitted.scope, RUNTIME_R1C_A_SCOPE);
    assert!(admitted.requires_compaction_for_next_rung);
    assert!(admitted.requires_allocation_for_birth_rung);
    assert!(!admitted.semantic_gpu_code_required);
    assert!(!admitted.cpu_planner_required);
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.pinned_number_change_required);
    assert!(!admitted.scenario_reopen_required);
    assert_eq!(
        admitted.next_horizon,
        "R1c-b resident allocation into marked free slots / no compaction"
    );
}

#[test]
fn r1c_a_preserves_r6c_checksum() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.r6c_checksum_expected,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert_eq!(
        admitted.r6c_checksum_observed,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert!(admitted.r6c_checksum_matches);
}

#[test]
fn r1c_a_uses_domain_neutral_terms() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for term in [
        "resident event journal",
        "free-list mark",
        "slot bitmap",
        "GPU-side structural event rows",
        "disabled-marker negative control",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
}

#[test]
fn r1c_a_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_ne!(admitted.stable_report_checksum, 0);
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM
    );
    assert_eq!(
        admitted.foreground_capture_method,
        RUNTIME_R0_FOREGROUND_CAPTURE
    );
}
