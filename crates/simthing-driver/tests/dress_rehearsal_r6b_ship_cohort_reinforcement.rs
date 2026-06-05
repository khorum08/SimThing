use simthing_driver::{
    construction_threshold_emission, damage_output_for_cohort, hp_to_retire_for_cohort,
    replay_dress_rehearsal_r6b_ship_cohort_reinforcement,
    run_dress_rehearsal_r6b_ship_cohort_reinforcement, run_r6_combat_with_r6b_cohorts,
    DressRehearsalR6bInput, DressRehearsalR6bOwner, DressRehearsalR6bReport,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS, FLEET_COHORT_NUM_SHIPS,
    FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP, R6B_FUSION_FIXTURE_CELL, R6B_FUSION_LEFT_ID,
    R6B_FUSION_RIGHT_ID, SHIP_COST,
};

fn report() -> DressRehearsalR6bReport {
    run_dress_rehearsal_r6b_ship_cohort_reinforcement(&DressRehearsalR6bInput::explicit_opt_in())
}

#[test]
fn r6b_consumes_r5_and_r6a_contracts() {
    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.r5_contract_checksum, 0x5308_a1eb_1b7a_e5fb);
    assert_eq!(admitted.r6_contract_checksum, 7528695422102681985);
}

#[test]
fn r6b_ship_production_is_threshold_emission_band() {
    let admitted = report();
    assert!(admitted
        .construction_rows
        .iter()
        .any(|row| row.threshold_passed));
    assert!(admitted.gpu_substrate_posture_only);
}

#[test]
fn r6b_construction_progress_emits_ship_count_delta() {
    let (_, passed, delta, remainder) = construction_threshold_emission(0, SHIP_COST, SHIP_COST);
    assert!(passed);
    assert_eq!(delta, 1);
    assert_eq!(remainder, 0);
    let admitted = report();
    assert!(admitted
        .construction_rows
        .iter()
        .any(|row| row.ship_count_delta_emitted >= 1));
}

#[test]
fn r6b_starport_production_targets_starbase_gridcell() {
    let admitted = report();
    assert!(admitted
        .construction_rows
        .iter()
        .all(|row| row.cell_index < 400));
    assert!(!admitted.construction_rows.is_empty());
}

#[test]
fn r6b_masked_selection_finds_existing_compatible_friendly_fleet() {
    let admitted = report();
    assert!(!admitted.reinforcement_rows.is_empty());
    let row = &admitted.reinforcement_rows[0];
    assert!(admitted
        .cohort_rows
        .iter()
        .any(|c| c.fleet_id == row.target_fleet_id));
}

#[test]
fn r6b_reinforcement_increments_num_ships_without_boundary_request() {
    let admitted = report();
    for row in &admitted.reinforcement_rows {
        assert!(!row.movement_boundary_request_used);
        assert_eq!(
            row.num_ships_after,
            row.num_ships_before + row.ship_count_delta
        );
    }
    assert!(!admitted.movement_boundary_request_used);
}

#[test]
fn r6b_reinforcement_recomputes_hp_to_retire() {
    let admitted = report();
    for row in &admitted.reinforcement_rows {
        assert_eq!(
            row.hp_to_retire_after,
            hp_to_retire_for_cohort(row.num_ships_after, FLEET_HP_PER_SHIP)
        );
    }
}

#[test]
fn r6b_reinforcement_recomputes_damage_output() {
    let admitted = report();
    for row in &admitted.reinforcement_rows {
        assert_eq!(
            row.damage_output_after,
            damage_output_for_cohort(row.num_ships_after, FLEET_DAMAGE_PER_SHIP_PER_TICK)
        );
    }
}

#[test]
fn r6b_new_fleet_birth_only_when_no_compatible_cohort_exists() {
    let admitted = report();
    assert!(!admitted.birth_rows.is_empty());
    for birth in &admitted.birth_rows {
        let cell = birth.cell_index;
        let compatible_before = admitted
            .cohort_rows
            .iter()
            .filter(|c| {
                c.cell_index == cell
                    && c.owner == birth.owner
                    && c.profile.hp_per_ship == FLEET_HP_PER_SHIP
                    && c.fleet_id != birth.created_fleet_id
            })
            .count();
        assert_eq!(
            compatible_before, 0,
            "birth implies no prior compatible cohort"
        );
    }
}

#[test]
fn r6b_new_fleet_birth_uses_local_alloc_enrollment_not_movement_boundary() {
    let admitted = report();
    for row in &admitted.birth_rows {
        assert!(row.alloc_enrollment_applied);
        assert!(!row.movement_boundary_request_used);
        assert_eq!(row.shadow_table_update_kind, "AllocArrivalEnrollment");
    }
}

#[test]
fn r6b_friendly_colocated_compatible_fleets_fuse_by_masked_reduction() {
    let admitted = report();
    let fusion = admitted
        .fusion_rows
        .iter()
        .find(|row| row.cell_index == R6B_FUSION_FIXTURE_CELL && row.fused_num_ships == 14)
        .expect("fixture cell fusion");
    assert_eq!(fusion.surviving_fleet_id, R6B_FUSION_LEFT_ID);
}

#[test]
fn r6b_fusion_sums_num_ships() {
    let admitted = report();
    let fusion = admitted
        .fusion_rows
        .iter()
        .find(|row| row.surviving_fleet_id == R6B_FUSION_LEFT_ID)
        .expect("left survivor fusion");
    assert_eq!(
        fusion.left_num_ships + fusion.right_num_ships,
        fusion.fused_num_ships
    );
    assert_eq!(fusion.fused_num_ships, 14);
}

#[test]
fn r6b_fusion_recomputes_hp_to_retire_and_damage_output() {
    let admitted = report();
    let fusion = admitted
        .fusion_rows
        .iter()
        .find(|row| row.fused_num_ships == 14)
        .expect("canonical 7+7 fusion");
    assert_eq!(
        fusion.hp_to_retire_after,
        hp_to_retire_for_cohort(fusion.fused_num_ships, fusion.hp_per_ship)
    );
    assert_eq!(
        fusion.damage_output_after,
        damage_output_for_cohort(fusion.fused_num_ships, fusion.damage_per_ship_per_tick)
    );
}

#[test]
fn r6b_fusion_records_identity_lineage() {
    let admitted = report();
    assert!(admitted
        .fusion_rows
        .iter()
        .all(|row| row.identity_lineage_recorded && !row.fusion_event_id.is_empty()));
}

#[test]
fn r6b_fusion_preserves_owner_overlay() {
    let admitted = report();
    assert!(admitted
        .fusion_rows
        .iter()
        .all(|row| row.owner_overlay_preserved));
}

#[test]
fn r6b_fusion_updates_shadow_table_or_membership_with_minimal_boundary() {
    let admitted = report();
    let fusion = admitted.fusion_rows.first().expect("fusion row");
    assert_eq!(fusion.shadow_table_update_kind, "CohortCompactionDeparture");
    assert!(!fusion.movement_boundary_request_used);
    assert!(!fusion.arena_membership_after.is_empty());
}

#[test]
fn r6b_hostile_colocated_fleets_do_not_fuse() {
    let admitted = report();
    let hostile_fusion = admitted.fusion_rows.iter().any(|row| {
        (row.surviving_fleet_id.contains("pirate") && row.absorbed_fleet_id.contains("patrol"))
            || (row.surviving_fleet_id.contains("patrol")
                && row.absorbed_fleet_id.contains("pirate"))
    });
    assert!(!hostile_fusion);
    let at_fixture: Vec<_> = admitted
        .cohort_rows
        .iter()
        .filter(|c| c.cell_index == R6B_FUSION_FIXTURE_CELL && !c.destroyed)
        .collect();
    assert!(at_fixture
        .iter()
        .any(|c| c.owner == DressRehearsalR6bOwner::Terran));
    assert!(at_fixture
        .iter()
        .any(|c| c.owner == DressRehearsalR6bOwner::Pirate));
}

#[test]
fn r6b_incompatible_friendly_fleets_do_not_fuse() {
    let admitted = report();
    let incompatible_fused = admitted.fusion_rows.iter().any(|row| {
        row.absorbed_fleet_id.contains("incompatible")
            || row.surviving_fleet_id.contains("incompatible")
    });
    assert!(!incompatible_fused);
}

#[test]
fn r6b_non_fleet_occupants_do_not_fuse() {
    let admitted = report();
    for row in &admitted.fusion_rows {
        assert!(
            row.surviving_fleet_id.contains("fleet")
                || row.surviving_fleet_id.contains("patrol")
                || row.surviving_fleet_id.contains("pirate")
                || row.surviving_fleet_id.contains("dress-rehearsal")
        );
    }
}

#[test]
fn r6b_combat_consumes_reinforced_or_fused_num_ships() {
    let admitted = report();
    let combat = admitted
        .combat_with_r6b_cohorts
        .as_ref()
        .expect("combat with R6B cohort overrides");
    assert!(combat.admitted);
    let reinforced = admitted
        .reinforcement_rows
        .first()
        .expect("reinforcement row");
    let entity = reinforced.entity_id;
    let arena = combat
        .combat_arena_rows
        .iter()
        .find(|row| row.entity_id == entity);
    if let Some(arena_row) = arena {
        assert_eq!(arena_row.num_ships_before, reinforced.num_ships_after);
    } else {
        let override_ships = admitted
            .fleet_cohort_overrides
            .get(&entity)
            .expect("override entry");
        assert_eq!(override_ships.num_ships, reinforced.num_ships_after);
    }
    let fused = admitted
        .fusion_rows
        .iter()
        .find(|row| row.fused_num_ships == 14)
        .expect("fusion 14");
    let fused_entity = entity_id_for_fusion_survivor(&admitted, fused.surviving_fleet_id.as_str());
    let o = admitted
        .fleet_cohort_overrides
        .get(&fused_entity)
        .expect("fused override");
    assert_eq!(o.num_ships, 14);
    let _ = run_r6_combat_with_r6b_cohorts(&admitted);
}

fn entity_id_for_fusion_survivor(report: &DressRehearsalR6bReport, fleet_id: &str) -> u64 {
    report
        .cohort_rows
        .iter()
        .find(|c| c.fleet_id == fleet_id)
        .map(|c| c.entity_id)
        .expect("survivor cohort")
}

#[test]
fn r6b_no_cpu_side_fleet_manager_decision_path() {
    let admitted = report();
    assert!(admitted.table_driven_masked_scan_used);
    assert!(!admitted.cpu_fleet_manager_decision_path);
}

#[test]
fn r6b_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r6b_ship_cohort_reinforcement();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_ne!(admitted.summary.stable_checksum, 0);
}

#[test]
fn r6b_opt_in_default_off() {
    let default = run_dress_rehearsal_r6b_ship_cohort_reinforcement(
        &DressRehearsalR6bInput::default_simsession(),
    );
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert!(default.disabled_no_op);
    assert!(default.construction_rows.is_empty());
}

#[test]
fn r6b_canonical_checksum_pin() {
    let admitted = report();
    assert_eq!(
        admitted.id,
        DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID
    );
    assert_eq!(
        admitted.status,
        DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS
    );
    assert_eq!(admitted.summary.stable_checksum, 18001790122452668567);
}
