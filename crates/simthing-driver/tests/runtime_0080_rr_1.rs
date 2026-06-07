use std::sync::OnceLock;

use simthing_driver::{
    build_recursive_world, canonical_access_pattern, replay_runtime_0080_rr_1,
    run_runtime_0080_rr_1, try_access_surface_for_system, try_access_system_at_galaxy_cell,
    Runtime0080Rr0Owner, Runtime0080Rr1Input, Runtime0080Rr1Report, Runtime0080Rr1ResidencyRequest,
    RR_1_GALAXY_CELL_COUNT, RR_1_SURFACE_CELL_COUNT, RR_1_SYSTEM_CELL_COUNT, RR_1_SYSTEM_COUNT,
    RUNTIME_0080_RR_1_ID, RUNTIME_0080_RR_1_STATUS_PASS, RUNTIME_RR_1_EXPECTED_REPORT_CHECKSUM,
};

static REPORT: OnceLock<Runtime0080Rr1Report> = OnceLock::new();

fn report() -> &'static Runtime0080Rr1Report {
    REPORT.get_or_init(|| run_runtime_0080_rr_1(&Runtime0080Rr1Input::explicit_opt_in()))
}

#[test]
fn rr_1_opt_in_default_off() {
    let default = run_runtime_0080_rr_1(&Runtime0080Rr1Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.verdict, "BLOCKED");
}

#[test]
fn rr_1_consumes_rr_0_recursive_world() {
    let admitted = report();
    assert!(admitted.rr_0_world_consumed);
    assert!(!admitted.rr_0_is_flattened);
    assert_ne!(admitted.rr_0_structural_checksum, 0);
    assert_eq!(admitted.tier_counts.addressable_system_handles, 13);
}

#[test]
fn rr_1_galaxy_always_resident() {
    let admitted = report();
    assert_eq!(
        admitted.tier_counts.galaxy_resident_rows,
        RR_1_GALAXY_CELL_COUNT
    );
    assert!(admitted
        .residency_trace
        .iter()
        .all(|row| row.galaxy_materialized_rows == RR_1_GALAXY_CELL_COUNT));
}

#[test]
fn rr_1_has_13_addressable_system_nodes() {
    let admitted = report();
    assert_eq!(admitted.system_handles.len(), RR_1_SYSTEM_COUNT);
    assert_eq!(admitted.system_handles.len(), 13);
}

#[test]
fn rr_1_descend_materializes_system_10x10() {
    let admitted = report();
    assert!(admitted.residency_trace.iter().any(|row| {
        row.system_materialized_rows == RR_1_SYSTEM_CELL_COUNT && row.surface_materialized_rows == 0
    }));
}

#[test]
fn rr_1_ascend_deactivates_system() {
    let admitted = report();
    assert!(admitted.residency_trace.iter().any(|row| {
        row.request == Some(Runtime0080Rr1ResidencyRequest::AscendToGalaxy)
            && row.system_materialized_rows == 0
            && row.surface_materialized_rows == 0
    }));
}

#[test]
fn rr_1_descend_materializes_planet_surface_10x10() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .any(|row| row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT));
}

#[test]
fn rr_1_ascend_deactivates_planet_surface() {
    let admitted = report();
    assert!(admitted.residency_trace.iter().any(|row| {
        matches!(
            row.request,
            Some(Runtime0080Rr1ResidencyRequest::AscendToSystem { .. })
        ) && row.surface_materialized_rows == 0
            && row.system_materialized_rows == RR_1_SYSTEM_CELL_COUNT
    }));
}

#[test]
fn rr_1_starport_visible_only_when_system_resident() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .any(|row| row.child_visibility.starport_visible));
    assert!(admitted.residency_trace.iter().any(|row| {
        row.system_materialized_rows == 0 && !row.child_visibility.starport_visible
    }));
}

#[test]
fn rr_1_pop_visible_only_when_surface_resident() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .any(|row| row.child_visibility.pop_cohort_visible));
    assert!(admitted.residency_trace.iter().any(|row| {
        row.surface_materialized_rows == 0 && !row.child_visibility.pop_cohort_visible
    }));
}

#[test]
fn rr_1_factory_visible_only_when_surface_resident() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .any(|row| row.child_visibility.factory_visible));
    assert!(admitted.residency_trace.iter().any(|row| {
        row.surface_materialized_rows == 0 && !row.child_visibility.factory_visible
    }));
}

#[test]
fn rr_1_terran_residency_path_proven() {
    let admitted = report();
    assert!(admitted.terran_path_proven);
    let terran_id = admitted
        .system_handles
        .iter()
        .find(|handle| handle.owner == Runtime0080Rr0Owner::Terran)
        .expect("terran handle")
        .system_id;
    assert!(admitted.residency_trace.iter().any(|row| {
        row.active_system_id == Some(terran_id)
            && row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT
    }));
}

#[test]
fn rr_1_pirate_residency_path_proven() {
    let admitted = report();
    assert!(admitted.pirate_path_proven);
    let pirate_id = admitted
        .system_handles
        .iter()
        .find(|handle| handle.owner == Runtime0080Rr0Owner::Pirate)
        .expect("pirate handle")
        .system_id;
    assert!(admitted.residency_trace.iter().any(|row| {
        row.active_system_id == Some(pirate_id)
            && row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT
    }));
}

#[test]
fn rr_1_no_galaxy_wrong_system_leakage() {
    let admitted = report();
    assert!(admitted.leakage_proof.wrong_galaxy_cell_rejected);
    let world = build_recursive_world(0x0080_2000);
    let system = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.id != 0)
        .expect("alt");
    assert!(try_access_system_at_galaxy_cell(&world, system.id, 0, 0).is_err());
}

#[test]
fn rr_1_no_system_wrong_planet_leakage() {
    let admitted = report();
    assert!(admitted.leakage_proof.wrong_system_surface_rejected);
    let world = build_recursive_world(0x0080_2000);
    let terran = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.owner == Runtime0080Rr0Owner::Terran)
        .expect("terran")
        .id;
    let pirate = world
        .galaxy
        .systems
        .iter()
        .find(|s| s.owner == Runtime0080Rr0Owner::Pirate)
        .expect("pirate")
        .id;
    assert!(try_access_surface_for_system(&world, terran, Some(pirate)).is_err());
}

#[test]
fn rr_1_no_inactive_surface_child_leakage() {
    let admitted = report();
    assert_eq!(admitted.leakage_proof.inactive_surface_child_count, 0);
    assert!(admitted.residency_trace.iter().any(|row| {
        row.system_materialized_rows == 0
            && row.surface_materialized_rows == 0
            && row.child_visibility.visible_child_count == 0
    }));
}

#[test]
fn rr_1_sparse_accounting_scales_with_active_tiers() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .all(|row| row.sparse_only_active_tiers));
    let galaxy_only = admitted.residency_trace.first().expect("first step");
    assert!(galaxy_only.inert_cell_count > 0);
    assert!(galaxy_only.resident_cell_count < 3000);
    let deepest = admitted
        .residency_trace
        .iter()
        .max_by_key(|row| row.resident_cell_count)
        .expect("deepest");
    assert_eq!(
        deepest.resident_cell_count,
        RR_1_GALAXY_CELL_COUNT + RR_1_SYSTEM_CELL_COUNT + RR_1_SURFACE_CELL_COUNT
    );
}

#[test]
fn rr_1_inactive_systems_have_zero_materialized_rows() {
    let admitted = report();
    assert!(admitted.residency_trace.iter().any(|row| {
        row.system_materialized_rows == 0 && row.galaxy_materialized_rows == RR_1_GALAXY_CELL_COUNT
    }));
    assert_eq!(
        admitted.tier_counts.inactive_system_rows,
        RR_1_SYSTEM_COUNT as u32 * RR_1_SYSTEM_CELL_COUNT - admitted.tier_counts.active_system_rows
    );
}

#[test]
fn rr_1_inactive_surfaces_have_zero_materialized_rows() {
    let admitted = report();
    assert!(admitted
        .residency_trace
        .iter()
        .any(|row| { row.surface_materialized_rows == 0 && row.system_materialized_rows > 0 }));
}

#[test]
fn rr_1_mapping_parity_matches_rr_0() {
    let admitted = report();
    assert!(admitted.mapping_parity_ok);
    assert_eq!(admitted.mapping_parity_rows.len(), RR_1_SYSTEM_COUNT);
    for row in &admitted.mapping_parity_rows {
        assert!(row.owner_matches_rr_0);
        assert!(row.parent_galaxy_matches_rr_0);
        assert!(row.system_dims_match_rr_0);
        assert!(row.surface_dims_match_rr_0);
        assert!(row.pop_placement_matches_rr_0);
        assert!(row.factory_placement_matches_rr_0);
    }
}

#[test]
fn rr_1_scope_ledger_contains_all_required_rows() {
    let admitted = report();
    assert_eq!(admitted.scope_ledger.len(), 21);
    for element in [
        "RR-0 recursive world consumed",
        "Galaxy 20×20 always resident",
        "GPU economy deferred to RR-2",
        "Integrated recursive 100-tick rehearsal deferred to RR-4",
    ] {
        assert!(
            admitted
                .scope_ledger
                .iter()
                .any(|row| row.spec_element == element),
            "missing: {element}"
        );
    }
}

#[test]
fn rr_1_pass_requires_required_scope_rows_implemented() {
    let admitted = report();
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.status, RUNTIME_0080_RR_1_STATUS_PASS);
    for row in admitted.scope_ledger.iter().take(18) {
        assert_eq!(row.status, "implemented", "row: {}", row.spec_element);
    }
}

#[test]
fn rr_1_deviation_record_required_for_any_proxy() {
    let admitted = report();
    assert!(admitted.deviation_records.is_empty());
}

#[test]
fn rr_1_no_flat_proxy_closure() {
    let admitted = report();
    assert!(!admitted.flat_proxy_closure);
    assert!(!admitted.rr_0_is_flattened);
}

#[test]
fn rr_1_no_gpu_economy_claim() {
    let admitted = report();
    assert!(!admitted.gpu_economy_claimed);
    assert_eq!(admitted.id, RUNTIME_0080_RR_1_ID);
}

#[test]
fn rr_1_no_rr_2_rr_3_rr_4_claim() {
    let admitted = report();
    assert!(!admitted.rr_2_claimed);
    assert!(!admitted.rr_3_claimed);
    assert!(!admitted.rr_4_claimed);
}

#[test]
fn rr_1_no_invariant_edit() {
    let admitted = report();
    assert!(!admitted.invariant_edit);
    let invariants = include_str!("../../../docs/invariants.md");
    assert!(invariants.contains("Specification Fidelity & Anti-Ceremony"));
}

#[test]
fn rr_1_report_checksum_stable() {
    let admitted = report();
    assert_eq!(
        admitted.stable_report_checksum,
        RUNTIME_RR_1_EXPECTED_REPORT_CHECKSUM
    );
}

#[test]
fn rr_1_canonical_access_pattern_covers_both_factions() {
    let world = build_recursive_world(0x0080_2000);
    let pattern = canonical_access_pattern(&world);
    assert_eq!(pattern.len(), 8);
    assert!(pattern
        .iter()
        .any(|req| matches!(req, Runtime0080Rr1ResidencyRequest::DescendToSurface { .. })));
}

#[test]
fn rr_1_replay_deterministic() {
    let (left, right) = replay_runtime_0080_rr_1();
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_ne!(left.deterministic_replay_checksum, 0);
}
