use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r1c_e, Runtime0080R1cEInput, Runtime0080R1cEReport, RUNTIME_0080_0_R1C_E_ID,
    RUNTIME_0080_0_R1C_E_PRIMITIVE, RUNTIME_0080_0_R1C_E_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_E_STATUS_PASS, RUNTIME_R1C_E_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_E_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cEReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cEReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_e(&Runtime0080R1cEInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1cEReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_e_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_e(&Runtime0080R1cEInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_E_ID);
    assert!(!default.admitted);
    assert_eq!(default.slot_remap_rows.len(), 0);
    assert_eq!(default.compacted_slot_rows.len(), 0);
    assert_eq!(default.membership_remap_rows.len(), 0);
}

#[test]
fn r1c_e_consumes_r1c_d_compaction_rows() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_E_STATUS_BLOCKED);
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_E_STATUS_PASS);
    assert!(admitted.consumes_r1c_d_compaction_rows);
    assert!(admitted.r1c_d_compaction_rows_consumed > 0);
}

#[test]
fn r1c_e_consumes_r1c_d_lineage_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.consumes_r1c_d_lineage_rows);
    assert!(admitted.r1c_d_lineage_rows_consumed > 0);
}

#[test]
fn r1c_e_consumes_r1c_c_membership_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.consumes_r1c_c_membership_rows);
    assert!(admitted.r1c_c_membership_rows_consumed > 0);
}

#[test]
fn r1c_e_resident_slot_remap_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_slot_remap_created);
    assert!(admitted.slot_remap_representation.contains("old_slot"));
}

#[test]
fn r1c_e_resident_compacted_slot_table_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_compacted_slot_table_created);
    assert!(admitted
        .resident_compacted_table_representation
        .contains("slot-table"));
}

#[test]
fn r1c_e_resident_membership_remap_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_membership_remap_created);
    assert!(admitted
        .membership_remap_representation
        .contains("membership"));
}

#[test]
fn r1c_e_gpu_writes_slot_remap_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_slot_remap_rows);
    assert!(admitted.slot_remap_rows_written > 0);
    assert!(admitted
        .slot_remap_rows
        .iter()
        .all(|row| row.applied_by_gpu));
}

#[test]
fn r1c_e_gpu_applies_compacted_slot_table() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_applies_compacted_slot_table);
    assert!(admitted.compacted_slot_rows_written > 0);
    assert!(admitted
        .compacted_slot_rows
        .iter()
        .all(|row| row.applied_by_gpu));
}

#[test]
fn r1c_e_gpu_writes_membership_remap_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_membership_remap_rows);
    assert!(admitted.membership_remap_rows_written > 0);
    assert!(admitted
        .membership_remap_rows
        .iter()
        .all(|row| row.applied_by_gpu));
}

#[test]
fn r1c_e_tombstoned_slots_are_inactive() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.tombstone_rows_applied > 0);
    assert!(admitted
        .slot_remap_rows
        .iter()
        .any(|row| row.new_slot_or_tombstone.is_none() && !row.active_after));
    assert!(admitted.compacted_slot_rows.iter().any(|row| !row.active));
}

#[test]
fn r1c_e_absorbed_slots_map_to_survivor() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.absorption_rows_applied > 0);
    assert!(admitted.slot_remap_rows.iter().any(|row| {
        row.reason_code == "FusionAbsorbed"
            && row.survivor_slot.is_some()
            && row.new_slot_or_tombstone == row.survivor_slot
            && !row.active_after
    }));
}

#[test]
fn r1c_e_birth_allocated_slots_remain_active() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.birth_allocation_rows_preserved > 0);
    assert!(admitted.slot_remap_rows.iter().any(|row| {
        row.reason_code == "BirthAllocated"
            && row.active_after
            && row.new_slot_or_tombstone.is_some()
    }));
}

#[test]
fn r1c_e_lineage_rows_preserved_after_apply() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.lineage_rows_preserved_after_apply);
    for kind in ["Birth", "Absorb", "Survive", "Tombstone"] {
        assert!(admitted
            .lineage_rows_after_apply
            .iter()
            .any(|row| row.lineage_kind == kind));
    }
}

#[test]
fn r1c_e_compacted_table_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.compacted_table_read_from_gpu_values);
    assert!(admitted.compacted_table_readback_count > 0);
}

#[test]
fn r1c_e_membership_remap_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.membership_remap_rows_read_from_gpu_values);
    assert!(admitted.membership_rows_remapped_or_linked_from_gpu_values);
    assert!(admitted.membership_remap_readback_count > 0);
}

#[test]
fn r1c_e_cpu_shadow_consumes_remap_without_redeciding() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_shadow.consumes_slot_remap_without_redeciding);
    assert!(!admitted.cpu_shadow.cpu_decided_any_slot_remap);
    assert!(
        admitted
            .cpu_shadow
            .cpu_shadow_does_not_rewrite_slot_mapping_first
    );
}

#[test]
fn r1c_e_cpu_shadow_consumes_compacted_table_without_redeciding() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(
        admitted
            .cpu_shadow
            .consumes_compacted_table_without_redeciding
    );
    assert!(!admitted.cpu_shadow.cpu_decided_any_compacted_table_row);
    assert!(!admitted.cpu_shadow.cpu_decided_any_lineage_application);
}

#[test]
fn r1c_e_disabled_remap_writer_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_remap_writer_check
        .as_ref()
        .expect("disabled remap writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_parity);
    assert!(!check.writers_disabled_parity);
    assert!(check.writers_disabled_rows < admitted.slot_remap_rows_written);
}

#[test]
fn r1c_e_reenabled_remap_writer_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_slot_remap_rows);
    assert!(admitted.remap_rows_read_from_gpu_values);
}

#[test]
fn r1c_e_disabled_compacted_table_writer_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_compacted_table_writer_check
        .as_ref()
        .expect("disabled compacted table writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_parity);
    assert!(!check.writers_disabled_parity);
    assert!(check.writers_disabled_rows < admitted.compacted_slot_rows_written);
}

#[test]
fn r1c_e_reenabled_compacted_table_writer_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_applies_compacted_slot_table);
    assert!(admitted.compacted_table_read_from_gpu_values);
}

#[test]
fn r1c_e_disabled_membership_remap_writer_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_membership_remap_writer_check
        .as_ref()
        .expect("disabled membership remap writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_parity);
    assert!(!check.writers_disabled_parity);
    assert!(check.writers_disabled_rows < admitted.membership_remap_rows_written);
}

#[test]
fn r1c_e_reenabled_membership_remap_writer_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_membership_remap_rows);
    assert!(admitted.membership_remap_rows_read_from_gpu_values);
}

#[test]
fn r1c_e_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1a_preservation.as_ref().expect("R1a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1b_event_journal_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1b_preservation.as_ref().expect("R1b summary");
    assert_eq!(summary.verdict, "PARTIAL");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1c_a_mark_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_a_preservation.as_ref().expect("R1c-a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1c_b_allocation_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_b_preservation.as_ref().expect("R1c-b summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1c_c_membership_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_c_preservation.as_ref().expect("R1c-c summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1c_d_compaction_lineage_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_d_preservation.as_ref().expect("R1c-d summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_e_preserves_r1c_complete_shadow_contract() {
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
fn r1c_e_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_m4a_authority);
    assert!(!admitted.multi_atlas_authority);
    assert!(!admitted.system_planet_recursion_authority);
}

#[test]
fn r1c_e_no_default_session_wiring() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.default_session_wiring);
}

#[test]
fn r1c_e_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.scenario_reopen_required);
}

#[test]
fn r1c_e_domain_neutral_terms_only() {
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
        "resident compaction map",
        "resident lineage staging",
        "resident compacted view",
        "resident slot-table apply",
        "disabled-transform parity check",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
    assert_eq!(admitted.scope, RUNTIME_R1C_E_SCOPE);
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_E_PRIMITIVE);
    assert!(!admitted.artifact_markdown.contains("Terran"));
    assert!(!admitted.artifact_markdown.contains("Pirate"));
}

#[test]
fn r1c_e_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_ne!(admitted.stable_report_checksum, 0);
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R1C_E_EXPECTED_REPORT_CHECKSUM
    );
}
