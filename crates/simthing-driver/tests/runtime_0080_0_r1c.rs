use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r1c, Runtime0080R1cInput, Runtime0080R1cReport, RUNTIME_0080_0_R1C_ID,
    RUNTIME_0080_0_R1C_PRIMITIVE, RUNTIME_0080_0_R1C_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_STATUS_PARTIAL, RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
    RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c(&Runtime0080R1cInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1cReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c(&Runtime0080R1cInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_ID);
    assert!(!default.admitted);
    assert!(default.predecessor.is_none());
}

#[test]
fn r1c_selects_r1b_predecessor_or_blocks_honestly() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_STATUS_BLOCKED);
        assert!(admitted
            .diagnostics
            .contains(&"r1b_predecessor_blocked_or_no_discrete_gpu".to_string()));
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_STATUS_PARTIAL);
    let predecessor = admitted.predecessor.as_ref().expect("R1b predecessor");
    assert_eq!(predecessor.r1b_verdict, "PARTIAL");
    assert!(predecessor.r1b_event_journal_parity);
    assert_eq!(
        predecessor.r1b_gpu_event_rows,
        predecessor.r1b_oracle_event_rows
    );
    assert!(predecessor.r1b_checksum_matches);
}

#[test]
fn r1c_does_not_overclaim_structural_decision_authority() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.verdict, "PARTIAL");
    assert!(!admitted.structural_decisions_gpu_emitted);
    assert!(!admitted.resident_reenroll_scatter_authority);
    assert!(!admitted.resident_birth_removal_authority);
    assert!(!admitted.resident_fusion_compaction_authority);
    assert!(admitted.cpu_decision_witness_still_authority);
}

#[test]
fn r1c_complete_cpu_shadow_is_serializable_and_pausable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let shadow = admitted
        .shadow_contract
        .as_ref()
        .expect("complete shadow contract");
    assert!(shadow.complete_cpu_shadow_retained);
    assert!(shadow.includes_tier_a_values);
    assert!(shadow.includes_membership);
    assert!(shadow.includes_positions);
    assert!(shadow.includes_birth_removal_state);
    assert!(shadow.includes_fusion_lineage);
    assert!(shadow.includes_slot_allocation);
    assert!(shadow.serialize_reload_continue_roundtrip);
    assert!(shadow.reloaded_from_serialized_snapshot);
    assert_ne!(shadow.serialized_snapshot_hash, 0);
    assert!(shadow.continue_hash_matches_after_reload);
    assert_eq!(shadow.roundtrip_hash_before, shadow.roundtrip_hash_after);
    assert_eq!(shadow.initial_snapshot, shadow.reloaded_snapshot);
}

#[test]
fn r1c_never_reconstructs_structural_state_from_value_projection() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let shadow = admitted
        .shadow_contract
        .as_ref()
        .expect("complete shadow contract");
    assert!(shadow.no_structural_reconstruction_from_value_projection);
    assert!(admitted.event_journal_remains_only_structural_handoff);
}

#[test]
fn r1c_stop_line_defines_next_smaller_rung() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.stop_line.stop_line_triggered);
    assert!(admitted.stop_line.requires_resident_free_list_scatter);
    assert!(
        admitted
            .stop_line
            .requires_resident_compaction_or_lineage_update
    );
    assert_eq!(
        admitted.stop_line.next_smaller_rung,
        "R1c-a resident free-list mark-only / no compaction"
    );
}

#[test]
fn r1c_does_not_pull_forbidden_gates_forward() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_PRIMITIVE);
    assert_eq!(admitted.scope, RUNTIME_R1C_SCOPE);
    assert!(!admitted.stop_line.requires_m4a_or_multi_atlas_now);
    assert!(!admitted.stop_line.semantic_gpu_code_required);
    assert!(!admitted.stop_line.cpu_planner_required);
    assert!(!admitted.stop_line.docs_invariants_edit_required);
    assert!(!admitted.stop_line.pinned_number_change_required);
    assert!(!admitted.stop_line.scenario_reopen_required);
}

#[test]
fn r1c_backpressure_policy_keeps_gpu_value_loop_off_cpu_hot_path() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.backpressure_policy.gpu_value_loop_may_run_ahead);
    assert!(
        admitted
            .backpressure_policy
            .cpu_boundary_consumer_is_not_hot_path_gate
    );
    assert!(
        admitted
            .backpressure_policy
            .per_tick_decision_readback_forbidden
    );
    assert_eq!(
        admitted
            .backpressure_policy
            .max_unserialized_ticks_documented,
        1
    );
}

#[test]
fn r1c_preserves_r6c_checksum() {
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
fn r1c_uses_domain_neutral_terms() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for term in [
        "FieldPolicy",
        "field_agent",
        "selection",
        "extraction",
        "resident event journal",
        "GPU-side structural event rows",
        "disabled-transform parity check",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
}

#[test]
fn r1c_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_ne!(admitted.stable_report_checksum, 0);
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM
    );
    assert_eq!(
        admitted.foreground_capture_method,
        RUNTIME_R0_FOREGROUND_CAPTURE
    );
}
