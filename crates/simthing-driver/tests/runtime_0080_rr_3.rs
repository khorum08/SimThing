use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, replay_runtime_0080_rr_3, run_runtime_0080_rr_3, Runtime0080Rr3Input,
    Runtime0080Rr3Report, Runtime0080Rr3TierTransition, RR_3_SLOTS_PER_SYSTEM,
    RUNTIME_0080_RR_3_ID, RUNTIME_0080_RR_3_STATUS_PASS, RUNTIME_RR_3_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr3Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr3Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_3(&Runtime0080Rr3Input::explicit_opt_in()))
}

#[test]
fn rr_3_opt_in_default_off() {
    let default = run_runtime_0080_rr_3(&Runtime0080Rr3Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.verdict, "BLOCKED");
}

#[test]
fn rr_3_consumes_rr_0_recursive_world() {
    let admitted = report();
    assert!(admitted.rr_0_world_consumed);
    assert!(admitted.not_flattened_scalar);
}

#[test]
fn rr_3_consumes_rr_1_nested_residency() {
    let admitted = report();
    assert!(admitted.rr_1_residency_consumed);
}

#[test]
fn rr_3_consumes_rr_2_gpu_surface_production() {
    let admitted = report();
    assert!(admitted.rr_2_surface_production_consumed);
}

#[test]
fn rr_3_proves_terran_recursive_path() {
    let admitted = report();
    assert!(admitted.terran_path_proven);
}

#[test]
fn rr_3_proves_pirate_recursive_path() {
    let admitted = report();
    assert!(admitted.pirate_path_proven);
}

#[test]
fn rr_3_gpu_reduces_surface_to_planet() {
    let admitted = report();
    assert!(admitted
        .reduce_up_rows
        .iter()
        .any(|row| row.transition == Runtime0080Rr3TierTransition::SurfaceToPlanet && row.parity));
}

#[test]
fn rr_3_gpu_reduces_planet_to_system() {
    let admitted = report();
    assert!(admitted
        .reduce_up_rows
        .iter()
        .any(|row| row.transition == Runtime0080Rr3TierTransition::PlanetToSystem && row.parity));
}

#[test]
fn rr_3_gpu_reduces_system_to_galaxy() {
    let admitted = report();
    assert!(admitted
        .reduce_up_rows
        .iter()
        .any(|row| row.transition == Runtime0080Rr3TierTransition::SystemToGalaxy && row.parity));
}

#[test]
fn rr_3_gpu_reduces_galaxy_to_faction_stockpile() {
    let admitted = report();
    assert!(admitted.reduce_up_rows.iter().any(|row| row.transition
        == Runtime0080Rr3TierTransition::GalaxyToStockpile
        && row.parity));
}

#[test]
fn rr_3_gpu_disburses_faction_to_galaxy() {
    let admitted = report();
    assert!(admitted.disburse_down_rows.iter().any(|row| {
        row.transition == Runtime0080Rr3TierTransition::StockpileToGalaxy && row.parity
    }));
}

#[test]
fn rr_3_gpu_disburses_galaxy_to_system() {
    let admitted = report();
    assert!(admitted.disburse_down_rows.iter().any(|row| {
        row.transition == Runtime0080Rr3TierTransition::GalaxyToSystem && row.parity
    }));
}

#[test]
fn rr_3_gpu_disburses_system_to_surface_or_starport() {
    let admitted = report();
    assert!(admitted.disburse_down_rows.iter().any(|row| {
        row.transition == Runtime0080Rr3TierTransition::SystemToStarport && row.parity
    }));
}

#[test]
fn rr_3_disabled_surface_to_planet_reduce_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_surface_to_planet_fails_parity);
}

#[test]
fn rr_3_reenabled_surface_to_planet_reduce_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_surface_to_planet_restores_parity);
}

#[test]
fn rr_3_disabled_galaxy_to_faction_reduce_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_galaxy_to_stockpile_fails_parity);
}

#[test]
fn rr_3_reenabled_galaxy_to_faction_reduce_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_galaxy_to_stockpile_restores_parity);
}

#[test]
fn rr_3_disabled_disburse_down_fails_parity() {
    let admitted = report();
    assert!(admitted.disabled_disburse_down_fails_parity);
}

#[test]
fn rr_3_reenabled_disburse_down_restores_parity() {
    let admitted = report();
    assert!(admitted.reenabled_disburse_down_restores_parity);
}

#[test]
fn rr_3_no_cross_owner_leakage() {
    let admitted = report();
    assert!(admitted.no_cross_owner_leakage);
}

#[test]
fn rr_3_no_cross_tier_shortcut() {
    let admitted = report();
    assert!(admitted.no_cross_tier_shortcut);
}

#[test]
fn rr_3_inactive_surfaces_do_not_reduce() {
    let admitted = report();
    assert!(admitted.inactive_surfaces_do_not_reduce);
}

#[test]
fn rr_3_inactive_systems_do_not_disburse() {
    let admitted = report();
    assert!(admitted.inactive_systems_do_not_disburse);
}

#[test]
fn rr_3_not_flattened_to_surface_to_faction_scalar() {
    let admitted = report();
    assert!(admitted.not_flattened_scalar);
    assert_eq!(admitted.system_bindings.len(), 13);
    let world = build_recursive_world(0x0080_2000);
    assert_eq!(world.galaxy.systems.len(), 13);
    assert!(admitted
        .system_bindings
        .iter()
        .all(|b| b.planet_slot + 1 == b.system_slot && b.system_slot + 1 == b.galaxy_slot));
    let _ = RR_3_SLOTS_PER_SYSTEM;
}

#[test]
fn rr_3_scope_ledger_contains_all_required_rows() {
    let admitted = report();
    assert_eq!(admitted.scope_ledger.len(), 27);
}

#[test]
fn rr_3_pass_requires_required_scope_rows_implemented() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.status, RUNTIME_0080_RR_3_STATUS_PASS);
    for row in admitted.scope_ledger.iter().take(25) {
        assert_eq!(row.status, "implemented", "row: {}", row.spec_element);
    }
}

#[test]
fn rr_3_deviation_record_required_for_any_proxy() {
    let admitted = report();
    assert!(admitted.deviation_records.is_empty());
}

#[test]
fn rr_3_no_rr_4_integrated_rehearsal_claim() {
    let admitted = report();
    assert!(!admitted.rr_4_claimed);
}

#[test]
fn rr_3_no_standalone_m4a_claim() {
    let admitted = report();
    assert!(!admitted.standalone_m4a_claimed);
}

#[test]
fn rr_3_no_invariant_edit() {
    let admitted = report();
    assert!(!admitted.invariant_edit);
    let invariants = include_str!("../../../docs/invariants.md");
    assert!(invariants.contains("Specification Fidelity & Anti-Ceremony"));
}

#[test]
fn rr_3_report_checksum_stable() {
    let admitted = report();
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_RR_3_EXPECTED_REPORT_CHECKSUM
    );
}

#[test]
fn rr_3_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_3();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(left.id, RUNTIME_0080_RR_3_ID);
    assert!(!left.reduce_up_rows.is_empty());
    assert!(!left.disburse_down_rows.is_empty());
}
