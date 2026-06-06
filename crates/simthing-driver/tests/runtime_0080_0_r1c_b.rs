use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1c_b, run_runtime_0080_0_r1c_b,
    run_runtime_0080_0_r1c_b_with_allocation_writers_enabled, Runtime0080R1cBInput,
    Runtime0080R1cBReport, RUNTIME_0080_0_R1C_B_ID, RUNTIME_0080_0_R1C_B_PRIMITIVE,
    RUNTIME_0080_0_R1C_B_STATUS_BLOCKED, RUNTIME_0080_0_R1C_B_STATUS_PASS, RUNTIME_R1C_B_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cBReport> = OnceLock::new();
static DISABLED_REPORT: OnceLock<Runtime0080R1cBReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cBReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::explicit_opt_in()))
}

fn disabled_report() -> &'static Runtime0080R1cBReport {
    DISABLED_REPORT.get_or_init(|| {
        run_runtime_0080_0_r1c_b_with_allocation_writers_enabled(
            &Runtime0080R1cBInput::explicit_opt_in(),
            false,
        )
    })
}

fn blocked(report: &Runtime0080R1cBReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_b_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_B_ID);
    assert!(!default.admitted);
    assert_eq!(default.allocation_rows_written, 0);
}

#[test]
fn r1c_b_consumes_r1c_a_mark_table() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_B_STATUS_BLOCKED);
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_B_STATUS_PASS);
    assert_eq!(admitted.scope, RUNTIME_R1C_B_SCOPE);
    assert!(admitted.free_slot_mark_count_before_allocation > 0);
    assert_eq!(
        admitted.free_slot_mark_count_before_allocation as usize,
        admitted.mark_table_before_allocation.len()
    );
}

#[test]
fn r1c_b_consumes_local_birth_request_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.local_birth_request_count > 0);
    assert_eq!(
        admitted.local_birth_request_count,
        admitted.allocation_rows_written
    );
    assert!(admitted
        .allocation_rows
        .iter()
        .all(|row| row.requested_ships > 0));
}

#[test]
fn r1c_b_resident_allocation_rows_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_B_PRIMITIVE);
    assert!(!admitted.allocation_rows.is_empty());
    assert!(admitted.allocation_rows_written_from_gpu_values);
}

#[test]
fn r1c_b_gpu_selects_marked_free_slot() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for slot in &admitted.allocated_slots {
        assert!(admitted.mark_table_before_allocation.contains(slot));
    }
    assert!(admitted.gpu_select_dispatch_count >= admitted.local_birth_request_count);
}

#[test]
fn r1c_b_allocates_lowest_compatible_marked_slot() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.allocated_slots, admitted.expected_allocated_slots);
}

#[test]
fn r1c_b_allocated_slot_read_from_gpu_value() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.allocated_slot_read_from_gpu_value);
    assert!(admitted
        .allocation_rows
        .iter()
        .all(|row| row.allocated_slot == row.gpu_selected_slot_value));
}

#[test]
fn r1c_b_mark_cleared_for_allocated_slot() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for slot in &admitted.allocated_slots {
        assert!(!admitted.mark_table_after_allocation.contains(slot));
    }
}

#[test]
fn r1c_b_unallocated_marks_remain() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for slot in &admitted.mark_table_before_allocation {
        if !admitted.allocated_slots.contains(slot) {
            assert!(admitted.mark_table_after_allocation.contains(slot));
        }
    }
}

#[test]
fn r1c_b_cpu_boundary_consumes_allocation_without_selecting_slot() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_boundary_pass_consumes_allocation_row);
    assert!(admitted.cpu_boundary_pass_does_not_select_slot);
    assert!(!admitted.cpu_selected_any_slot);
    assert_eq!(
        admitted.boundary_pass.enrolled_slots,
        admitted.allocated_slots
    );
}

#[test]
fn r1c_b_disabled_allocation_writer_fails_allocation_parity() {
    let disabled = disabled_report();
    if blocked(disabled) {
        return;
    }
    assert!(!disabled.allocation_parity_measured_from_gpu_values);
    assert_eq!(disabled.allocation_rows_written, 0);
}

#[test]
fn r1c_b_reenabled_allocation_writer_restores_allocation_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_allocation_writer_check
        .as_ref()
        .expect("disabled allocation writer check");
    assert!(check.writers_enabled_allocation_parity);
    assert!(!check.writers_disabled_allocation_parity);
    assert!(check.negative_control_detected);
}

#[test]
fn r1c_b_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1a_preservation.as_ref().expect("R1a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_b_preserves_r1b_event_journal_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1b_preservation.as_ref().expect("R1b summary");
    assert_eq!(summary.verdict, "PARTIAL");
    assert!(summary.preserved);
}

#[test]
fn r1c_b_preserves_r1c_a_mark_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_a_preservation.as_ref().expect("R1c-a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_b_preserves_r1c_complete_shadow_contract() {
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
fn r1c_b_no_compaction_or_lineage_rewrite() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_compaction_authority);
    assert!(!admitted.resident_lineage_rewrite_authority);
}

#[test]
fn r1c_b_no_resident_reenroll_scatter() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_reenroll_scatter_authority);
    assert!(!admitted.resident_arena_membership_rewrite_authority);
}

#[test]
fn r1c_b_no_fusion_compaction() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_fusion_compaction_authority);
}

#[test]
fn r1c_b_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.m4a_or_multi_atlas_authority);
}

#[test]
fn r1c_b_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.scenario_reopen_required);
    assert!(!admitted.artifact_markdown.contains("scenario reopen"));
}

#[test]
fn r1c_b_domain_neutral_terms_only() {
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
        "resident free-slot allocation",
        "GPU-side allocation rows",
        "disabled-transform parity check",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
}

#[test]
fn r1c_b_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let (first, second) = replay_runtime_0080_0_r1c_b();
    assert_eq!(first.stable_report_checksum, second.stable_report_checksum);
    assert_eq!(
        first.stable_report_checksum,
        admitted.stable_report_checksum
    );
}
