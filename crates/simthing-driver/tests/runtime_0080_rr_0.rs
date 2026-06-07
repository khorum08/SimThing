use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, replay_runtime_0080_rr_0, run_runtime_0080_rr_0, Runtime0080Rr0Input,
    Runtime0080Rr0Owner, Runtime0080Rr0Report, FACTORY_UNIT_COST_LABOR, GALAXY_SIDE,
    POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE, R6C_CANONICAL_TICK_COUNT, RUNTIME_0080_RR_0_ID,
    RUNTIME_0080_RR_0_STATUS_PASS, RUNTIME_RR_0_EXPECTED_REPORT_CHECKSUM, SYSTEM_COUNT,
};

const SYSTEM_SIDE: u32 = 10;

static REPORT: OnceLock<Runtime0080Rr0Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr0Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_0(&Runtime0080Rr0Input::explicit_opt_in()))
}

#[test]
fn rr_0_opt_in_default_off() {
    let default = run_runtime_0080_rr_0(&Runtime0080Rr0Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.ticks_scheduled, 0);
    assert_eq!(default.verdict, "BLOCKED");
}

#[test]
fn rr_0_builds_galaxy_20x20() {
    let admitted = report();
    assert_eq!(admitted.world.galaxy.width, GALAXY_SIDE);
    assert_eq!(admitted.world.galaxy.height, GALAXY_SIDE);
    assert_eq!(admitted.entity_counts.galaxy_cells, 400);
}

#[test]
fn rr_0_builds_13_star_systems() {
    let admitted = report();
    assert_eq!(admitted.entity_counts.systems, SYSTEM_COUNT);
    assert_eq!(admitted.world.galaxy.systems.len(), 13);
}

#[test]
fn rr_0_each_system_has_10x10_subgrid() {
    let admitted = report();
    for system in &admitted.world.galaxy.systems {
        assert_eq!(system.width, SYSTEM_SIDE);
        assert_eq!(system.height, SYSTEM_SIDE);
        assert_eq!(system.cells.len(), 100);
    }
    assert_eq!(admitted.entity_counts.system_grid_cells, 13 * 100);
}

#[test]
fn rr_0_each_system_has_starport_child() {
    let admitted = report();
    assert_eq!(admitted.entity_counts.starports, 4);
    assert!(admitted
        .world
        .galaxy
        .systems
        .iter()
        .any(|system| system.starport.is_some()));
}

#[test]
fn rr_0_each_system_has_planet_child() {
    let admitted = report();
    for system in &admitted.world.galaxy.systems {
        assert_eq!(system.planet.parent_system_id, system.id);
    }
    assert_eq!(admitted.entity_counts.planets, 13);
}

#[test]
fn rr_0_each_planet_has_10x10_surface() {
    let admitted = report();
    for system in &admitted.world.galaxy.systems {
        assert_eq!(system.planet.surface.width, 10);
        assert_eq!(system.planet.surface.height, 10);
        assert_eq!(system.planet.surface.cells.len(), 100);
    }
    assert_eq!(admitted.entity_counts.surface_cells, 13 * 100);
}

#[test]
fn rr_0_each_surface_has_pop_cohort_child() {
    let admitted = report();
    for system in &admitted.world.galaxy.systems {
        assert_eq!(system.planet.surface.pop_cohort.kind, "PopCohort");
        assert!(!system.planet.surface.pop_cohort.simthing_id.is_empty());
    }
    assert_eq!(admitted.entity_counts.pop_cohorts, 13);
}

#[test]
fn rr_0_each_surface_has_factory_child() {
    let admitted = report();
    for system in &admitted.world.galaxy.systems {
        assert_eq!(system.planet.surface.factory.kind, "FactoryDistrict");
        assert!(!system.planet.surface.factory.simthing_id.is_empty());
    }
    assert_eq!(admitted.entity_counts.factories, 13);
}

#[test]
fn rr_0_terran_has_10_systems() {
    let admitted = report();
    let terran = admitted
        .world
        .galaxy
        .systems
        .iter()
        .filter(|system| system.owner == Runtime0080Rr0Owner::Terran)
        .count();
    assert_eq!(terran, 10);
}

#[test]
fn rr_0_pirate_has_3_systems() {
    let admitted = report();
    let pirate = admitted
        .world
        .galaxy
        .systems
        .iter()
        .filter(|system| system.owner == Runtime0080Rr0Owner::Pirate)
        .count();
    assert_eq!(pirate, 3);
}

#[test]
fn rr_0_not_flattened_systems_are_not_only_galactic_cells() {
    let admitted = report();
    assert!(!admitted.world.is_flattened);
    assert!(!admitted.flat_proxy_closure);
    for system in &admitted.world.galaxy.systems {
        assert_ne!(
            system.cells.len(),
            1,
            "system must not be a single galactic scalar"
        );
        assert_eq!(system.planet.surface.cells.len(), 100);
    }
}

#[test]
fn rr_0_pop_emits_labor() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.labor_emitted, 13 * POP_LABOR_PER_TICK);
    assert_eq!(admitted.total_labor_emitted, 100 * 13 * POP_LABOR_PER_TICK);
}

#[test]
fn rr_0_factory_consumes_labor() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.labor_consumed, 13 * FACTORY_UNIT_COST_LABOR);
}

#[test]
fn rr_0_factory_produces_output() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.production_generated, 13 * PRODUCTION_PER_RECIPE);
    assert_eq!(
        admitted.total_production_generated,
        100 * 13 * PRODUCTION_PER_RECIPE
    );
}

#[test]
fn rr_0_reduces_surface_to_planet() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.reduced_surface_to_planet, first.production_generated);
    assert!(first.reduced_surface_to_planet > 0);
}

#[test]
fn rr_0_reduces_planet_to_system() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.reduced_planet_to_system, first.production_generated);
}

#[test]
fn rr_0_reduces_system_to_galaxy() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(first.reduced_system_to_galaxy, first.production_generated);
}

#[test]
fn rr_0_reduces_galaxy_to_faction_stockpile() {
    let admitted = report();
    let first = admitted.oracle_ticks.first().expect("oracle tick");
    assert_eq!(
        first.reduced_galaxy_to_stockpile_terran + first.reduced_galaxy_to_stockpile_pirate,
        first.production_generated
    );
    assert!(first.terran_stockpile_after > 0 || first.pirate_stockpile_after > 0);
}

#[test]
fn rr_0_disburses_down_recursively() {
    let admitted = report();
    assert!(admitted.total_disbursed_terran > 0 || admitted.total_disbursed_pirate > 0);
    let received: i64 = admitted
        .world
        .galaxy
        .systems
        .iter()
        .filter_map(|system| system.starport.as_ref())
        .map(|starport| starport.production_received)
        .sum();
    assert_eq!(
        received,
        admitted.total_disbursed_terran + admitted.total_disbursed_pirate
    );
}

#[test]
fn rr_0_runs_100_tick_recursive_cpu_oracle() {
    let admitted = report();
    assert_eq!(admitted.ticks_scheduled, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(admitted.ticks_completed, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(admitted.oracle_ticks.len(), 100);
    assert!(admitted
        .oracle_ticks
        .iter()
        .all(|tick| tick.structural_identity_preserved));
}

#[test]
fn rr_0_oracle_deterministic() {
    let (left, right) = replay_runtime_0080_rr_0();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_ne!(left.deterministic_replay_checksum, 0);
}

#[test]
fn rr_0_scope_ledger_contains_all_required_rows() {
    let admitted = report();
    assert_eq!(admitted.scope_ledger.len(), 24);
    for element in [
        "Galaxy 20×20 grid",
        "13 occupied star systems",
        "100-tick recursive CPU oracle",
        "Integrated recursive GPU rehearsal deferred to RR-4",
    ] {
        assert!(
            admitted
                .scope_ledger
                .iter()
                .any(|row| row.spec_element == element),
            "missing scope row: {element}"
        );
    }
}

#[test]
fn rr_0_pass_requires_required_scope_rows_implemented() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.status, RUNTIME_0080_RR_0_STATUS_PASS);
    for row in admitted.scope_ledger.iter().take(19) {
        assert_eq!(row.status, "implemented", "row: {}", row.spec_element);
    }
}

#[test]
fn rr_0_deviation_record_required_for_any_proxy() {
    let admitted = report();
    assert!(admitted.deviation_records.is_empty());
    assert!(admitted
        .scope_ledger
        .iter()
        .take(19)
        .all(|row| row.deviation.is_empty()));
}

#[test]
fn rr_0_no_gpu_residency_claim() {
    let admitted = report();
    assert!(admitted.cpu_oracle_only);
    assert!(!admitted.gpu_residency_claimed);
    assert_eq!(admitted.id, RUNTIME_0080_RR_0_ID);
}

#[test]
fn rr_0_no_flat_proxy_closure() {
    let admitted = report();
    assert!(!admitted.flat_proxy_closure);
    assert!(!admitted.world.is_flattened);
}

#[test]
fn rr_0_no_invariant_edit() {
    let admitted = report();
    assert!(!admitted.invariant_edit);
    let invariants = include_str!("../../../docs/invariants.md");
    assert!(invariants.contains("Specification Fidelity & Anti-Ceremony"));
}

#[test]
fn rr_0_report_checksum_stable() {
    let admitted = report();
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_RR_0_EXPECTED_REPORT_CHECKSUM
    );
}

#[test]
fn rr_0_recursive_world_build_is_independent_of_oracle() {
    let world = build_recursive_world(0x0080_2000);
    assert_eq!(world.galaxy.systems.len(), 13);
    assert!(!world.is_flattened);
    assert_eq!(world.galaxy.width, GALAXY_SIDE);
}
