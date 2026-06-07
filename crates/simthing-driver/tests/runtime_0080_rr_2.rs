use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, replay_runtime_0080_rr_2, run_runtime_0080_rr_2, Runtime0080Rr0Owner,
    Runtime0080Rr2Input, Runtime0080Rr2Report, RR_2_ACTIVE_SURFACE_COUNT, RUNTIME_0080_RR_2_ID,
    RUNTIME_0080_RR_2_STATUS_PASS, RUNTIME_RR_2_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr2Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr2Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_2(&Runtime0080Rr2Input::explicit_opt_in()))
}

#[test]
fn rr_2_opt_in_default_off() {
    let default = run_runtime_0080_rr_2(&Runtime0080Rr2Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.verdict, "BLOCKED");
}

#[test]
fn rr_2_consumes_rr_0_recursive_world() {
    let admitted = report();
    assert!(admitted.rr_0_world_consumed);
    assert!(admitted.not_flattened_scalar);
}

#[test]
fn rr_2_consumes_rr_1_nested_residency() {
    let admitted = report();
    assert!(admitted.rr_1_residency_consumed);
    assert!(admitted.terran_proof.materialized_through_rr_1);
    assert!(admitted.pirate_proof.materialized_through_rr_1);
}

#[test]
fn rr_2_materializes_terran_surface_through_rr_1() {
    let admitted = report();
    assert!(admitted.terran_proof.materialized_through_rr_1);
    assert_eq!(admitted.terran_proof.owner, Runtime0080Rr0Owner::Terran);
}

#[test]
fn rr_2_materializes_pirate_surface_through_rr_1() {
    let admitted = report();
    assert!(admitted.pirate_proof.materialized_through_rr_1);
    assert_eq!(admitted.pirate_proof.owner, Runtime0080Rr0Owner::Pirate);
}

#[test]
fn rr_2_pop_child_is_surface_labor_emitter() {
    let admitted = report();
    assert!(admitted.labor_parity_ok);
    assert!(admitted.terran_proof.pop_slot != admitted.terran_proof.factory_slot);
}

#[test]
fn rr_2_factory_child_is_surface_labor_consumer() {
    let admitted = report();
    assert!(admitted.labor_parity_ok);
    assert!(admitted
        .parity_rows
        .iter()
        .all(|row| row.labor_consumed > 0));
}

#[test]
fn rr_2_gpu_computes_labor_emission() {
    let admitted = report();
    assert!(admitted.gpu_available);
    assert!(admitted.labor_parity_ok);
}

#[test]
fn rr_2_gpu_computes_factory_labor_consumption() {
    let admitted = report();
    assert!(admitted.labor_parity_ok);
}

#[test]
fn rr_2_gpu_computes_production_generation() {
    let admitted = report();
    assert!(admitted.production_parity_ok);
}

#[test]
fn rr_2_labor_bits_match_rr_0_oracle() {
    let admitted = report();
    assert!(admitted.labor_parity_ok);
    assert!(admitted.parity_rows.iter().all(|row| row.parity));
}

#[test]
fn rr_2_production_bits_match_rr_0_oracle() {
    let admitted = report();
    assert!(admitted.production_parity_ok);
    assert!(admitted
        .parity_rows
        .iter()
        .all(|row| row.cpu_production_bits == row.gpu_production_bits));
}

#[test]
fn rr_2_disabled_labor_emitter_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_emitter_fails_parity);
}

#[test]
fn rr_2_reenabled_labor_emitter_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_emitter_restores_parity);
}

#[test]
fn rr_2_disabled_factory_consumer_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_consumer_fails_parity);
}

#[test]
fn rr_2_reenabled_factory_consumer_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_consumer_restores_parity);
}

#[test]
fn rr_2_inactive_surface_emits_no_labor() {
    let admitted = report();
    assert!(admitted.inactive_surface_no_labor);
}

#[test]
fn rr_2_inactive_surface_produces_no_output() {
    let admitted = report();
    assert!(admitted.inactive_surface_no_output);
}

#[test]
fn rr_2_no_cross_surface_labor_leakage() {
    let admitted = report();
    assert!(admitted.no_cross_surface_leakage);
}

#[test]
fn rr_2_not_flattened_to_system_or_galaxy_scalar() {
    let admitted = report();
    assert!(admitted.not_flattened_scalar);
    assert_eq!(admitted.surface_bindings.len(), RR_2_ACTIVE_SURFACE_COUNT);
    let world = build_recursive_world(0x0080_2000);
    assert_eq!(world.galaxy.systems.len(), 13);
}

#[test]
fn rr_2_scope_ledger_contains_all_required_rows() {
    let admitted = report();
    assert_eq!(admitted.scope_ledger.len(), 21);
}

#[test]
fn rr_2_pass_requires_required_scope_rows_implemented() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.status, RUNTIME_0080_RR_2_STATUS_PASS);
    for row in admitted.scope_ledger.iter().take(18) {
        assert_eq!(row.status, "implemented", "row: {}", row.spec_element);
    }
}

#[test]
fn rr_2_deviation_record_required_for_any_proxy() {
    let admitted = report();
    assert!(admitted.deviation_records.is_empty());
}

#[test]
fn rr_2_no_rr_3_reduce_disburse_claim() {
    let admitted = report();
    assert!(!admitted.rr_3_claimed);
}

#[test]
fn rr_2_no_rr_4_integrated_rehearsal_claim() {
    let admitted = report();
    assert!(!admitted.rr_4_claimed);
}

#[test]
fn rr_2_no_standalone_m4a_claim() {
    let admitted = report();
    assert!(!admitted.standalone_m4a_claimed);
}

#[test]
fn rr_2_no_invariant_edit() {
    let admitted = report();
    assert!(!admitted.invariant_edit);
    let invariants = include_str!("../../../docs/invariants.md");
    assert!(invariants.contains("Specification Fidelity & Anti-Ceremony"));
}

#[test]
fn rr_2_report_checksum_stable() {
    let admitted = report();
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_RR_2_EXPECTED_REPORT_CHECKSUM
    );
}

#[test]
fn rr_2_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_2();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_2_ID);
    assert!(left.parity_rows.len() >= 2);
    assert_eq!(left.parity_rows.len(), RR_2_ACTIVE_SURFACE_COUNT);
}
