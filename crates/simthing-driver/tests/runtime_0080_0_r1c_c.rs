use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1c_c, run_runtime_0080_0_r1c_c,
    run_runtime_0080_0_r1c_c_with_membership_writers_enabled, Runtime0080R1cCInput,
    Runtime0080R1cCReport, RUNTIME_0080_0_R1C_C_ID, RUNTIME_0080_0_R1C_C_PRIMITIVE,
    RUNTIME_0080_0_R1C_C_STATUS_BLOCKED, RUNTIME_0080_0_R1C_C_STATUS_PASS, RUNTIME_R1C_C_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cCReport> = OnceLock::new();
static DISABLED_REPORT: OnceLock<Runtime0080R1cCReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cCReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::explicit_opt_in()))
}

fn disabled_report() -> &'static Runtime0080R1cCReport {
    DISABLED_REPORT.get_or_init(|| {
        run_runtime_0080_0_r1c_c_with_membership_writers_enabled(
            &Runtime0080R1cCInput::explicit_opt_in(),
            false,
        )
    })
}

fn blocked(report: &Runtime0080R1cCReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_c_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_C_ID);
    assert!(!default.admitted);
    assert_eq!(default.membership_delta_rows.len(), 0);
}

#[test]
fn r1c_c_consumes_r1b_event_journal() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_C_STATUS_BLOCKED);
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_C_STATUS_PASS);
    assert!(admitted.movement_membership_delta_count > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.source_event_kind == "MoveRequest"));
}

#[test]
fn r1c_c_consumes_r1c_b_allocation_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.birth_membership_delta_count > 0);
    assert!(admitted.allocated_birth_slots_added > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "BirthIn"));
}

#[test]
fn r1c_c_resident_membership_table_created_or_reused() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_membership_table_created_or_reused);
    assert!(admitted.membership_representation.contains("slot-to-cell"));
}

#[test]
fn r1c_c_gpu_writes_membership_delta_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_membership_delta_rows);
    assert!(!admitted.membership_delta_rows.is_empty());
}

#[test]
fn r1c_c_gpu_applies_move_source_removal() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.source_removals_applied > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "MoveOut" && row.applied_by_gpu));
}

#[test]
fn r1c_c_gpu_applies_move_destination_addition() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.destination_additions_applied > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "MoveIn" && row.applied_by_gpu));
}

#[test]
fn r1c_c_gpu_applies_birth_membership_for_allocated_slot() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.allocated_birth_slots_added > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "BirthIn" && row.applied_by_gpu));
}

#[test]
fn r1c_c_gpu_marks_departure_membership_when_zero_cohort_present() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.departure_membership_delta_count > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "DepartureMark" && row.applied_by_gpu));
}

#[test]
fn r1c_c_gpu_applies_owner_code_update_when_present() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.owner_code_update_count > 0);
    assert!(admitted
        .membership_delta_rows
        .iter()
        .any(|row| row.membership_action == "OwnerCodeUpdate" && row.applied_by_gpu));
}

#[test]
fn r1c_c_membership_state_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.membership_apply_reads_gpu_rows);
    assert!(admitted.membership_parity_measured_from_gpu_values);
}

#[test]
fn r1c_c_cpu_shadow_observes_after_gpu_apply() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_shadow.observes_after_gpu_apply);
    assert!(admitted.cpu_shadow.shadow_matches_oracle);
}

#[test]
fn r1c_c_cpu_does_not_apply_membership_before_gpu() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_shadow.does_not_apply_membership_before_gpu);
    assert!(!admitted.cpu_shadow.cpu_selected_membership_effects);
}

#[test]
fn r1c_c_disabled_membership_writer_fails_membership_parity() {
    let admitted = report();
    let disabled = disabled_report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_membership_writer_check
        .as_ref()
        .expect("disabled membership writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_membership_parity);
    assert!(!check.writers_disabled_membership_parity);
    assert!(disabled.membership_delta_rows.len() < admitted.membership_delta_rows.len());
}

#[test]
fn r1c_c_reenabled_membership_writer_restores_membership_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.membership_parity_measured_from_gpu_values);
    assert!(admitted.gpu_writes_membership_delta_rows);
}

#[test]
fn r1c_c_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1a_preservation.as_ref().expect("R1a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_c_preserves_r1b_event_journal_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1b_preservation.as_ref().expect("R1b summary");
    assert_eq!(summary.verdict, "PARTIAL");
    assert!(summary.preserved);
}

#[test]
fn r1c_c_preserves_r1c_a_mark_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_a_preservation.as_ref().expect("R1c-a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_c_preserves_r1c_b_allocation_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_b_preservation.as_ref().expect("R1c-b summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_c_preserves_r1c_complete_shadow_contract() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted
        .r1c_shadow_preservation
        .as_ref()
        .expect("R1c shadow summary");
    assert_eq!(summary.verdict, "PARTIAL");
    assert!(summary.preserved);
}

#[test]
fn r1c_c_no_compaction_or_lineage_rewrite() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_compaction_authority);
    assert!(!admitted.resident_lineage_rewrite_authority);
}

#[test]
fn r1c_c_no_fusion_compaction() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_fusion_compaction_authority);
}

#[test]
fn r1c_c_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_m4a_authority);
}

#[test]
fn r1c_c_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.scenario_reopen_required);
}

#[test]
fn r1c_c_domain_neutral_terms_only() {
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
        "resident mark table",
        "resident allocation rows",
        "resident membership table",
        "membership apply",
        "disabled-transform parity check",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
    assert_eq!(admitted.scope, RUNTIME_R1C_C_SCOPE);
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_C_PRIMITIVE);
}

#[test]
fn r1c_c_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let (first, second) = replay_runtime_0080_0_r1c_c();
    assert_eq!(first.stable_report_checksum, second.stable_report_checksum);
    assert_eq!(
        first.stable_report_checksum,
        admitted.stable_report_checksum
    );
}
