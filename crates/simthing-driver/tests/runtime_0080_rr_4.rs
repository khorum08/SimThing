use std::sync::OnceLock;

use simthing_driver::{
    dress_rehearsal_r6c_integrated_run::R6C_CANONICAL_TICK_COUNT, replay_runtime_0080_rr_4,
    run_runtime_0080_rr_4, Runtime0080Rr4Input, Runtime0080Rr4Report, RUNTIME_0080_RR_4_ID,
    RUNTIME_0080_RR_4_STATUS_PASS, RUNTIME_RR_4_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr4Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr4Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_4(&Runtime0080Rr4Input::explicit_opt_in()))
}

#[test]
fn rr_4_opt_in_default_off() {
    let default = run_runtime_0080_rr_4(&Runtime0080Rr4Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.verdict, "BLOCKED");
}

#[test]
fn rr_4_consumes_rr_0_recursive_world_and_oracle() {
    let admitted = report();
    assert!(admitted.rr_0_world_consumed);
    assert_eq!(admitted.ticks_completed, R6C_CANONICAL_TICK_COUNT);
}

#[test]
fn rr_4_consumes_rr_1_nested_residency() {
    assert!(report().rr_1_residency_consumed);
}

#[test]
fn rr_4_consumes_rr_2_gpu_surface_economy() {
    assert!(report().rr_2_surface_economy_consumed);
}

#[test]
fn rr_4_consumes_rr_3_recursive_gpu_transfers() {
    assert!(report().rr_3_recursive_transfers_consumed);
}

#[test]
fn rr_4_runs_100_recursive_ticks() {
    let admitted = report();
    assert_eq!(admitted.tick_count, 100);
    assert_eq!(admitted.ticks_completed, 100);
}

#[test]
fn rr_4_tick_state_feeds_next_tick() {
    assert!(report().tick_state_feeds_next_tick);
}

#[test]
fn rr_4_integrates_terran_path_for_100_ticks() {
    assert!(report().terran_path_integrated);
}

#[test]
fn rr_4_integrates_pirate_path_for_100_ticks() {
    assert!(report().pirate_path_integrated);
}

#[test]
fn rr_4_gpu_computes_labor_each_tick() {
    assert!(report().per_tick_labor_parity_ok);
}

#[test]
fn rr_4_gpu_computes_factory_consumption_each_tick() {
    assert!(report().per_tick_labor_parity_ok);
}

#[test]
fn rr_4_gpu_computes_production_each_tick() {
    assert!(report().per_tick_production_parity_ok);
}

#[test]
fn rr_4_gpu_reduces_surface_to_planet_each_tick() {
    assert!(report().per_tick_reduce_up_parity_ok);
}

#[test]
fn rr_4_gpu_reduces_planet_to_system_each_tick() {
    assert!(report().per_tick_reduce_up_parity_ok);
}

#[test]
fn rr_4_gpu_reduces_system_to_galaxy_each_tick() {
    assert!(report().per_tick_reduce_up_parity_ok);
}

#[test]
fn rr_4_gpu_reduces_galaxy_to_stockpile_each_tick() {
    assert!(report().per_tick_reduce_up_parity_ok);
}

#[test]
fn rr_4_gpu_disburses_stockpile_to_galaxy_each_tick() {
    assert!(report().per_tick_disburse_down_parity_ok);
}

#[test]
fn rr_4_gpu_disburses_galaxy_to_system_each_tick() {
    assert!(report().per_tick_disburse_down_parity_ok);
}

#[test]
fn rr_4_gpu_disburses_system_to_surface_or_starport_each_tick() {
    assert!(report().per_tick_disburse_down_parity_ok);
}

#[test]
fn rr_4_per_tick_labor_bits_match_rr_0_oracle() {
    assert!(report().per_tick_labor_parity_ok);
}

#[test]
fn rr_4_per_tick_production_bits_match_rr_0_oracle() {
    assert!(report().per_tick_production_parity_ok);
}

#[test]
fn rr_4_per_tick_reduce_up_bits_match_rr_0_oracle() {
    assert!(report().per_tick_reduce_up_parity_ok);
}

#[test]
fn rr_4_per_tick_disburse_down_bits_match_rr_0_oracle() {
    assert!(report().per_tick_disburse_down_parity_ok);
}

#[test]
fn rr_4_final_stockpile_bits_match_rr_0_oracle() {
    assert!(report().final_stockpile_parity_ok);
}

#[test]
fn rr_4_final_starport_or_target_bits_match_rr_0_oracle() {
    assert!(report().final_starport_parity_ok);
}

#[test]
fn rr_4_no_cross_owner_leakage_over_100_ticks() {
    assert!(report().no_cross_owner_leakage);
}

#[test]
fn rr_4_no_cross_tier_shortcut_over_100_ticks() {
    assert!(report().no_cross_tier_shortcut);
}

#[test]
fn rr_4_inactive_surfaces_do_not_emit_or_reduce() {
    assert!(report().inactive_surfaces_no_op);
}

#[test]
fn rr_4_inactive_systems_do_not_disburse() {
    assert!(report().inactive_systems_no_op);
}

#[test]
fn rr_4_not_flattened_to_surface_to_faction_scalar() {
    assert!(report().not_flattened_scalar);
}

#[test]
fn rr_4_scope_ledger_contains_all_required_rows() {
    assert_eq!(report().scope_ledger.len(), 33);
}

#[test]
fn rr_4_pass_requires_required_scope_rows_implemented() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.status, RUNTIME_0080_RR_4_STATUS_PASS);
    assert!(admitted.recursive_horizon_reached);
    for row in admitted.scope_ledger.iter().take(30) {
        assert_eq!(row.status, "implemented", "row: {}", row.spec_element);
    }
}

#[test]
fn rr_4_deviation_record_required_for_any_proxy() {
    assert!(report().deviation_records.is_empty());
}

#[test]
fn rr_4_no_standalone_m4a_claim() {
    assert!(!report().standalone_m4a_claimed);
}

#[test]
fn rr_4_no_default_session_wiring() {
    assert!(!report().default_session_wiring);
}

#[test]
fn rr_4_no_invariant_edit() {
    assert!(!report().invariant_edit);
    let invariants = include_str!("../../../docs/invariants.md");
    assert!(invariants.contains("Specification Fidelity & Anti-Ceremony"));
}

#[test]
fn rr_4_report_checksum_stable() {
    let admitted = report();
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_RR_4_EXPECTED_REPORT_CHECKSUM
    );
}

#[test]
fn rr_4_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_4();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_4_ID);
    assert_eq!(left.tick_parity_rows.len(), 200);
}
