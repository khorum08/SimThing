use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1b, run_runtime_0080_0_r1b, run_runtime_0080_0_r1b_with_event_writers_enabled,
    Runtime0080R1bInput, Runtime0080R1bReport, RUNTIME_0080_0_R1B_ID, RUNTIME_0080_0_R1B_PRIMITIVE,
    RUNTIME_0080_0_R1B_STATUS_BLOCKED, RUNTIME_0080_0_R1B_STATUS_PASS,
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE,
};

static REPORT: OnceLock<Runtime0080R1bReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1bReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1bReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1b_opt_in_default_off() {
    let default = run_runtime_0080_0_r1b(&Runtime0080R1bInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.trace.len(), 0);
    assert_eq!(default.id, RUNTIME_0080_0_R1B_ID);
    assert!(!default.resident_event_journal_created);
}

#[test]
fn r1b_resident_event_journal_created() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.resident_event_journal_created);
    assert_eq!(admitted.journal_tick_count, 100);
}

#[test]
fn r1b_gpu_writes_movement_request_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_event_rows);
    let movement = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "MoveRequest")
        .map(|row| row.rows)
        .unwrap_or(0);
    assert!(movement > 0, "expected MoveRequest rows from GPU journal");
}

#[test]
fn r1b_gpu_writes_combat_event_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let damage = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "DamageDelta")
        .map(|row| row.rows)
        .unwrap_or(0);
    let zero = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "ZeroCohort")
        .map(|row| row.rows)
        .unwrap_or(0);
    assert!(damage > 0 || zero > 0, "expected combat journal rows");
}

#[test]
fn r1b_gpu_writes_construction_reinforcement_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let reinforcement = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "ShipCountDelta")
        .map(|row| row.rows)
        .unwrap_or(0);
    assert!(
        reinforcement > 0,
        "expected ShipCountDelta reinforcement rows"
    );
}

#[test]
fn r1b_gpu_writes_fusion_or_compaction_rows_when_applicable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let fusion = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "FusionRequest")
        .map(|row| row.rows)
        .unwrap_or(0);
    assert!(fusion > 0, "R6C trajectory includes fusion events");
}

#[test]
fn r1b_gpu_writes_blockade_divert_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let blockade = admitted
        .per_kind_row_counts
        .iter()
        .find(|row| row.kind == "OwnerCodeFlip")
        .map(|row| row.rows)
        .unwrap_or(0);
    assert!(blockade > 0, "expected OwnerCodeFlip journal rows");
}

#[test]
fn r1b_event_rows_are_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.event_rows_read_from_gpu_values);
}

#[test]
fn r1b_boundary_pass_consumes_gpu_event_rows() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_boundary_pass_consumes_event_rows);
    assert!(admitted
        .trace
        .iter()
        .any(|row| row.boundary_rows_applied > 0));
}

#[test]
fn r1b_boundary_pass_does_not_rederive_movement_decisions() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.boundary_pass_invoked_movement_tick);
    assert!(admitted.cpu_boundary_pass_does_not_rederive_decisions);
}

#[test]
fn r1b_boundary_pass_does_not_rederive_combat_decisions() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.boundary_pass_invoked_combat_tick);
}

#[test]
fn r1b_boundary_pass_does_not_rederive_production_decisions() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.boundary_pass_invoked_production_tick);
}

#[test]
fn r1b_boundary_pass_does_not_rederive_blockade_decisions() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_boundary_pass_does_not_rederive_decisions);
}

#[test]
fn r1b_event_rows_match_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.event_journal_parity_measured_from_gpu_values);
    assert_eq!(admitted.status, RUNTIME_0080_0_R1B_STATUS_PASS);
}

#[test]
fn r1b_disabled_event_writer_fails_event_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_transform_event_writer_check
        .as_ref()
        .expect("disabled-transform parity check");
    assert!(!check.writers_disabled_oracle_parity);
}

#[test]
fn r1b_reenabled_event_writer_restores_event_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let check = admitted
        .disabled_transform_event_writer_check
        .as_ref()
        .expect("disabled-transform parity check");
    assert!(check.writers_enabled_oracle_parity);
    assert!(check.negative_control_detected);
}

#[test]
fn r1b_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.r1a_tier_a_preservation);
    assert_eq!(admitted.r1a_tier_a_preservation_verdict, "PASS");
}

#[test]
fn r1b_preserves_r6c_checksum_or_reports_expected_structural_delta() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.r6c_checksum_expected,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert!(admitted.r6c_checksum_matches);
}

#[test]
fn r1b_no_cpu_planner() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_boundary_pass_does_not_rederive_decisions);
    assert!(!admitted.boundary_pass_invoked_movement_tick);
    assert!(!admitted.boundary_pass_invoked_combat_tick);
    assert!(!admitted.boundary_pass_invoked_production_tick);
}

#[test]
fn r1b_no_scenario_specific_event_kind() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let allowed = [
        "MoveRequest",
        "DamageDelta",
        "ShipCountDelta",
        "ZeroCohort",
        "LocalBirthRequest",
        "FusionRequest",
        "OwnerCodeFlip",
    ];
    for row in &admitted.per_kind_row_counts {
        assert!(
            allowed.contains(&row.kind),
            "unexpected event kind {}",
            row.kind
        );
    }
}

#[test]
fn r1b_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1B_PRIMITIVE);
    assert!(!admitted.artifact_markdown.contains("M-4A"));
    assert!(!admitted.artifact_markdown.contains("multi-atlas"));
}

#[test]
fn r1b_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.artifact_markdown.contains("invariants.md"));
    assert!(!admitted.artifact_markdown.contains("scenario reopen"));
}

#[test]
fn r1b_uses_domain_neutral_terms() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.domain_terms.contains(&"FieldPolicy"));
    assert!(admitted.domain_terms.contains(&"field_agent"));
    assert!(admitted.domain_terms.contains(&"selection"));
    assert!(admitted.domain_terms.contains(&"extraction"));
}

#[test]
fn r1b_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let (first, second) = replay_runtime_0080_0_r1b();
    assert_eq!(first.stable_report_checksum, second.stable_report_checksum);
    assert_eq!(
        first.stable_report_checksum,
        admitted.stable_report_checksum
    );
    assert_eq!(
        first.foreground_capture_method,
        RUNTIME_R0_FOREGROUND_CAPTURE
    );
}

#[test]
fn r1b_selects_discrete_gpu_or_blocks_honestly() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1B_STATUS_BLOCKED);
        assert!(!admitted.diagnostics.is_empty());
        assert!(admitted.adapter.is_none());
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1B_STATUS_PASS);
    assert_eq!(admitted.verdict, "PASS");
    let adapter = admitted.adapter.as_ref().expect("R1b adapter");
    assert!(adapter.selected_discrete_gpu);
}

#[test]
fn r1b_negative_control_event_writers_disabled() {
    let disabled = run_runtime_0080_0_r1b_with_event_writers_enabled(
        &Runtime0080R1bInput::explicit_opt_in(),
        false,
    );
    if disabled.verdict == "BLOCKED" {
        return;
    }
    assert!(!disabled.gpu_writes_event_rows);
    assert!(!disabled.event_journal_parity_measured_from_gpu_values);
}
