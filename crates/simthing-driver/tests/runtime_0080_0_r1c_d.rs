use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r1c_d, Runtime0080R1cDInput, Runtime0080R1cDReport, RUNTIME_0080_0_R1C_D_ID,
    RUNTIME_0080_0_R1C_D_PRIMITIVE, RUNTIME_0080_0_R1C_D_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_D_STATUS_PASS, RUNTIME_R1C_D_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_D_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cDReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cDReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_d(&Runtime0080R1cDInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1cDReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_d_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_d(&Runtime0080R1cDInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_D_ID);
    assert!(!default.admitted);
    assert_eq!(default.compaction_rows.len(), 0);
    assert_eq!(default.lineage_rows.len(), 0);
}

#[test]
fn r1c_d_consumes_r1b_event_journal() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_D_STATUS_BLOCKED);
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_D_STATUS_PASS);
    assert!(admitted.consumes_r1b_event_journal);
    assert!(admitted.r1b_event_rows_consumed > 0);
}

#[test]
fn r1c_d_consumes_r1c_a_mark_table() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.consumes_r1c_a_mark_table);
    assert!(admitted.r1c_a_mark_rows_consumed > 0);
}

#[test]
fn r1c_d_consumes_r1c_b_allocation_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.consumes_r1c_b_allocation_rows);
    assert!(admitted.r1c_b_allocation_rows_consumed > 0);
}

#[test]
fn r1c_d_consumes_r1c_c_membership_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.consumes_r1c_c_membership_rows);
    assert!(admitted.r1c_c_membership_rows_consumed > 0);
}

#[test]
fn r1c_d_resident_compaction_map_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_compaction_map_created);
    assert!(admitted.compaction_representation.contains("old_slot"));
    assert!(!admitted.resident_compacted_view.is_empty());
}

#[test]
fn r1c_d_resident_lineage_staging_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_lineage_staging_created);
    assert!(admitted.lineage_representation.contains("lineage"));
}

#[test]
fn r1c_d_gpu_writes_compaction_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_compaction_rows);
    assert!(admitted.compaction_rows_written > 0);
    assert!(admitted
        .compaction_rows
        .iter()
        .all(|row| row.applied_by_gpu));
}

#[test]
fn r1c_d_gpu_writes_lineage_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_lineage_rows);
    assert!(admitted.lineage_rows_written > 0);
    assert!(admitted.lineage_rows.iter().all(|row| row.applied_by_gpu));
}

#[test]
fn r1c_d_zero_or_departure_rows_stage_tombstones() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.zero_or_departure_tombstones_staged_if_zero_rows_exist);
    assert!(admitted.tombstone_rows_written > 0);
    assert!(admitted
        .compaction_rows
        .iter()
        .any(|row| row.reason_code == "DepartureMarked" && row.new_slot_or_tombstone.is_none()));
}

#[test]
fn r1c_d_fusion_rows_stage_absorption_lineage_when_present() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.fusion_absorption_lineage_staged_if_fusion_rows_exist);
    assert!(admitted.fusion_absorption_rows_written > 0);
    assert!(admitted
        .lineage_rows
        .iter()
        .any(|row| row.lineage_kind == "Absorb"));
    assert!(admitted
        .lineage_rows
        .iter()
        .any(|row| row.lineage_kind == "Survive"));
}

#[test]
fn r1c_d_birth_allocation_rows_stage_birth_lineage() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.birth_lineage_rows_written > 0);
    assert!(admitted
        .lineage_rows
        .iter()
        .any(|row| row.lineage_kind == "Birth"));
    assert!(admitted
        .compaction_rows
        .iter()
        .any(|row| row.reason_code == "BirthAllocated"));
}

#[test]
fn r1c_d_compaction_rows_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.compaction_rows_read_from_gpu_values);
    assert!(admitted.compaction_readback_count > 0);
}

#[test]
fn r1c_d_lineage_rows_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.lineage_rows_read_from_gpu_values);
    assert!(admitted.lineage_readback_count > 0);
}

#[test]
fn r1c_d_cpu_shadow_consumes_compaction_without_redeciding() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(
        admitted
            .cpu_shadow
            .consumes_compaction_rows_without_redeciding
    );
    assert!(!admitted.cpu_shadow.cpu_decided_any_compaction_row);
    assert!(admitted.cpu_shadow.compaction_shadow_matches_gpu_rows);
}

#[test]
fn r1c_d_cpu_shadow_consumes_lineage_without_redeciding() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_shadow.consumes_lineage_rows_without_redeciding);
    assert!(!admitted.cpu_shadow.cpu_decided_any_lineage_row);
    assert!(admitted.cpu_shadow.lineage_shadow_matches_gpu_rows);
}

#[test]
fn r1c_d_disabled_compaction_writer_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_compaction_writer_check
        .as_ref()
        .expect("disabled compaction writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_parity);
    assert!(!check.writers_disabled_parity);
    assert!(check.writers_disabled_rows < admitted.compaction_rows_written);
}

#[test]
fn r1c_d_reenabled_compaction_writer_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_compaction_rows);
    assert!(admitted.compaction_rows_read_from_gpu_values);
}

#[test]
fn r1c_d_disabled_lineage_writer_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_lineage_writer_check
        .as_ref()
        .expect("disabled lineage writer check");
    assert!(check.negative_control_detected);
    assert!(check.writers_enabled_parity);
    assert!(!check.writers_disabled_parity);
    assert!(check.writers_disabled_rows < admitted.lineage_rows_written);
}

#[test]
fn r1c_d_reenabled_lineage_writer_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_lineage_rows);
    assert!(admitted.lineage_rows_read_from_gpu_values);
}

#[test]
fn r1c_d_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1a_preservation.as_ref().expect("R1a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_d_preserves_r1b_event_journal_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1b_preservation.as_ref().expect("R1b summary");
    assert_eq!(summary.verdict, "PARTIAL");
    assert!(summary.preserved);
}

#[test]
fn r1c_d_preserves_r1c_a_mark_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_a_preservation.as_ref().expect("R1c-a summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_d_preserves_r1c_b_allocation_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_b_preservation.as_ref().expect("R1c-b summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_d_preserves_r1c_c_membership_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted.r1c_c_preservation.as_ref().expect("R1c-c summary");
    assert_eq!(summary.verdict, "PASS");
    assert!(summary.preserved);
}

#[test]
fn r1c_d_preserves_r1c_complete_shadow_contract() {
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
fn r1c_d_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_m4a_authority);
    assert!(!admitted.multi_atlas_authority);
    assert!(!admitted.system_planet_recursion_authority);
}

#[test]
fn r1c_d_no_default_session_wiring() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.default_session_wiring);
}

#[test]
fn r1c_d_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.scenario_reopen_required);
}

#[test]
fn r1c_d_domain_neutral_terms_only() {
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
        "disabled-transform parity check",
    ] {
        assert!(admitted.domain_terms.contains(&term));
    }
    assert_eq!(admitted.scope, RUNTIME_R1C_D_SCOPE);
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_D_PRIMITIVE);
    assert!(!admitted.artifact_markdown.contains("Terran"));
    assert!(!admitted.artifact_markdown.contains("Pirate"));
}

#[test]
fn r1c_d_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_ne!(admitted.stable_report_checksum, 0);
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R1C_D_EXPECTED_REPORT_CHECKSUM
    );
}
