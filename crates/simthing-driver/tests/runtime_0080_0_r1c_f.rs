use std::sync::OnceLock;

use simthing_driver::{
    run_runtime_0080_0_r1c_f, Runtime0080R1cFInput, Runtime0080R1cFReport, RUNTIME_0080_0_R1C_F_ID,
    RUNTIME_0080_0_R1C_F_PRIMITIVE, RUNTIME_0080_0_R1C_F_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_F_STATUS_PASS, RUNTIME_R1C_F_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_F_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1cFReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1cFReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1c_f(&Runtime0080R1cFInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1cFReport) -> bool {
    report.verdict == "BLOCKED"
}

#[test]
fn r1c_f_opt_in_default_off() {
    let default = run_runtime_0080_0_r1c_f(&Runtime0080R1cFInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.id, RUNTIME_0080_0_R1C_F_ID);
    assert!(!default.admitted);
    assert_eq!(default.zero_cohort_row_count, 0);
}

#[test]
fn r1c_f_gpu_decides_zero_cohort_from_resident_num_ships() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_zero_cohort_decision_from_resident_num_ships);
    assert!(admitted.zero_cohort_row_count > 0);
    assert_eq!(
        admitted.zero_cohort_resident_input_column, "num_ships",
        "decision must read resident num_ships"
    );
    assert!(
        admitted
            .zero_cohort_rows
            .iter()
            .all(|row| row.gpu_num_ships_value < 0.5),
        "GPU threshold downward@0.5 requires post-crossing num_ships < 0.5"
    );
    assert!(
        admitted
            .zero_cohort_rows
            .iter()
            .all(|row| row.gpu_previous_num_ships >= 0.5),
        "GPU threshold downward@0.5 requires pre-crossing num_ships >= 0.5"
    );
}

#[test]
fn r1c_f_zero_cohort_uses_threshold_or_emission_band_not_identity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.zero_cohort_decision_op_is_threshold_or_emission_band);
    assert!(admitted.zero_cohort_decision_gate.contains("Threshold"));
    assert_eq!(admitted.zero_cohort_decision_consume_mode, "EmitEvent");
    assert!(!admitted.zero_cohort_decision_op_is_identity_copy);
}

#[test]
fn r1c_f_cpu_witness_does_not_decide_zero_cohort() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.cpu_witness_decides_zero_cohort);
}

#[test]
fn r1c_f_zero_cohort_rows_read_from_gpu_values() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.zero_cohort_rows_read_from_gpu_values);
    assert!(!admitted.zero_cohort_rows.is_empty());
}

#[test]
fn r1c_f_zero_cohort_parity_matches_r6c_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.zero_cohort_rows_match_r6c_oracle);
    assert_eq!(
        admitted.zero_cohort_row_count, admitted.oracle_zero_cohort_row_count,
        "GPU ZeroCohort row count must match oracle"
    );
    assert!(admitted.event_journal_parity);
}

#[test]
fn r1c_f_disabled_zero_cohort_emitter_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.disabled_zero_cohort_emitter_fails_parity);
    let check = admitted
        .disabled_zero_cohort_emitter_check
        .as_ref()
        .expect("disabled emitter negative control must run with admitted report");
    assert_eq!(check.emitter_disabled_rows, 0);
    assert!(!check.emitter_disabled_oracle_parity);
    assert!(check.negative_control_detected);
}

#[test]
fn r1c_f_reenabled_zero_cohort_emitter_restores_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.reenabled_zero_cohort_emitter_restores_parity);
    let check = admitted
        .disabled_zero_cohort_emitter_check
        .as_ref()
        .expect("negative control must run");
    assert!(check.negative_control_detected);
    assert!(check.emitter_enabled_oracle_parity);
    assert!(!check.emitter_disabled_oracle_parity);
}

#[test]
fn r1c_f_sets_structural_decisions_gpu_emitted_zero_cohort_true() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1C_F_STATUS_PASS);
    assert!(admitted.structural_decisions_gpu_emitted_zero_cohort);
}

#[test]
fn r1c_f_umbrella_structural_decisions_gpu_emitted_remains_false_until_all_classes() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.structural_decisions_gpu_emitted);
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"DamageDelta"));
    assert!(admitted
        .remaining_cpu_decided_classes
        .contains(&"MoveRequest"));
}

#[test]
fn r1c_f_preserves_r1a_tier_a_source_of_truth() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted
        .r1a_preservation
        .as_ref()
        .expect("R1a preservation summary");
    assert!(summary.preserved, "R1a verdict {}", summary.verdict);
}

#[test]
fn r1c_f_preserves_r1b_event_journal_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted
        .r1b_preservation
        .as_ref()
        .expect("R1b preservation summary");
    assert!(summary.preserved, "R1b verdict {}", summary.verdict);
}

#[test]
fn r1c_f_preserves_r1c_a_b_c_d_contracts() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for label in ["R1c-a", "R1c-b", "R1c-c", "R1c-d"] {
        let summary = match label {
            "R1c-a" => admitted.r1c_a_preservation.as_ref(),
            "R1c-b" => admitted.r1c_b_preservation.as_ref(),
            "R1c-c" => admitted.r1c_c_preservation.as_ref(),
            "R1c-d" => admitted.r1c_d_preservation.as_ref(),
            _ => None,
        }
        .expect(label);
        assert!(summary.preserved, "{} verdict {}", label, summary.verdict);
    }
}

#[test]
fn r1c_f_preserves_r1c_e_contract_if_present() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted
        .r1c_e_preservation
        .as_ref()
        .expect("R1c-e landed on master");
    assert!(summary.preserved, "R1c-e verdict {}", summary.verdict);
}

#[test]
fn r1c_f_preserves_r1c_complete_shadow_contract() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let summary = admitted
        .r1c_shadow_preservation
        .as_ref()
        .expect("R1c shadow preservation summary");
    assert!(summary.preserved, "R1c verdict {}", summary.verdict);
}

#[test]
fn r1c_f_no_identity_copy_substitution_for_decision() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.zero_cohort_decision_op_is_identity_copy);
    assert!(admitted.zero_cohort_decision_gate.contains("Threshold"));
    assert!(admitted.zero_cohort_decision_per_tick);
}

#[test]
fn r1c_f_no_m4a_or_multi_atlas() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.resident_m4a_authority);
    assert!(!admitted.multi_atlas_authority);
}

#[test]
fn r1c_f_no_invariant_edit_or_scenario_reopen() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.docs_invariants_edit_required);
    assert!(!admitted.scenario_reopen_required);
}

#[test]
fn r1c_f_domain_neutral_terms_only() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.scope, RUNTIME_R1C_F_SCOPE);
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1C_F_PRIMITIVE);
    for term in &admitted.domain_terms {
        assert!(
            !term.contains("wgpu") && !term.contains("WGSL"),
            "domain term must stay neutral: {}",
            term
        );
    }
}

#[test]
fn r1c_f_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1C_F_STATUS_BLOCKED);
        return;
    }
    assert_ne!(admitted.stable_report_checksum, 0);
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_R1C_F_EXPECTED_REPORT_CHECKSUM
    );
}
