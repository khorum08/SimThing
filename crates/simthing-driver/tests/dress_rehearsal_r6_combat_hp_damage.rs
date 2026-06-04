use simthing_driver::{
    damage_output_for_cohort, emission_band_ship_attrition, hp_to_kill_for_cohort,
    replay_dress_rehearsal_r6_combat_hp_damage, run_dress_rehearsal_r6_combat_hp_damage,
    DressRehearsalR6Input, DressRehearsalR6Owner, DressRehearsalR6Report,
    DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID, DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS,
    FLEET_COHORT_NUM_SHIPS, FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP,
    GALACTIC_STRUCTURAL_PARENT,
};

fn report() -> DressRehearsalR6Report {
    run_dress_rehearsal_r6_combat_hp_damage(&DressRehearsalR6Input::explicit_opt_in())
}

#[test]
fn r6_consumes_r5_post_movement_contract() {
    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.r5_contract_checksum, 0x5308_a1eb_1b7a_e5fb);
    assert!(admitted.summary.hostile_colocation_detected);
}

#[test]
fn r6_detects_hostile_colocation_in_cell_arena() {
    let admitted = report();
    assert!(!admitted.combat_arena_rows.is_empty());
    let cell = admitted.combat_arena_rows[0].cell_index;
    let owners: std::collections::BTreeSet<_> = admitted
        .combat_arena_rows
        .iter()
        .filter(|row| row.cell_index == cell)
        .map(|row| row.owner)
        .collect();
    assert!(owners.contains(&DressRehearsalR6Owner::Terran));
    assert!(owners.contains(&DressRehearsalR6Owner::Pirate));
}

#[test]
fn r6_fleet_is_simthing_cohort_with_ten_ships() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert_eq!(row.num_ships_before, FLEET_COHORT_NUM_SHIPS);
    }
}

#[test]
fn r6_hp_to_kill_equals_num_ships_times_hp_per_ship() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert_eq!(row.hp_per_ship, FLEET_HP_PER_SHIP);
        assert_eq!(
            row.hp_to_kill_before,
            hp_to_kill_for_cohort(row.num_ships_before, row.hp_per_ship)
        );
    }
}

#[test]
fn r6_damage_output_equals_num_ships_times_damage_per_ship_per_tick() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert_eq!(
            row.damage_output,
            damage_output_for_cohort(row.num_ships_before, row.damage_per_ship_per_tick)
        );
    }
}

#[test]
fn r6_combat_uses_adversarial_resource_flow_arena() {
    let admitted = report();
    assert!(admitted.adversarial_resource_flow_arena_used);
    assert!(!admitted.reduce_up_rows.is_empty());
    assert!(!admitted.disburse_down_rows.is_empty());
}

#[test]
fn r6_damage_reduces_up_by_owner_channel() {
    let admitted = report();
    let cell = admitted.combat_arena_rows[0].cell_index;
    let terran_total: i64 = admitted
        .reduce_up_rows
        .iter()
        .filter(|row| row.cell_index == cell && row.owner == DressRehearsalR6Owner::Terran)
        .map(|row| row.damage_output)
        .sum();
    let pirate_total: i64 = admitted
        .reduce_up_rows
        .iter()
        .filter(|row| row.cell_index == cell && row.owner == DressRehearsalR6Owner::Pirate)
        .map(|row| row.damage_output)
        .sum();
    assert!(terran_total > 0);
    assert!(pirate_total > 0);
    assert_eq!(
        admitted
            .reduce_up_rows
            .iter()
            .find(|row| row.owner == DressRehearsalR6Owner::Terran)
            .expect("terran reduce-up")
            .owner_channel_total_after_reduce_up,
        terran_total
    );
}

#[test]
fn r6_damage_disburses_down_to_hostile_cohort() {
    let admitted = report();
    assert!(admitted.disburse_down_rows.iter().any(|row| {
        row.attacker_owner != row.target_owner
            && row.damage_disbursed > 0
            && admitted
                .combat_arena_rows
                .iter()
                .any(|c| c.combatant_id == row.target_id)
    }));
}

#[test]
fn r6_owner_mask_blocks_friendly_fire() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        for target in &row.hostile_target_ids {
            let target_row = admitted
                .combat_arena_rows
                .iter()
                .find(|other| &other.combatant_id == target)
                .expect("hostile target row");
            assert_ne!(target_row.owner, row.owner);
        }
        assert!(!admitted.disburse_down_rows.iter().any(|d| {
            d.attacker_owner == d.target_owner
                && d.attacker_id == row.combatant_id
                && d.damage_disbursed > 0
        }));
    }
}

#[test]
fn r6_non_fleet_occupants_are_combat_inert() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert!(
            row.combatant_id.contains("patrol") || row.combatant_id.contains("pirate-ship")
        );
    }
}

#[test]
fn r6_r3_combat_modifier_changes_bounded_damage_or_hp() {
    let admitted = report();
    let terran = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR6Owner::Terran)
        .expect("terran combatant");
    let pirate = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR6Owner::Pirate)
        .expect("pirate combatant");
    assert_eq!(terran.r3_combat_modifier_bps, 10_500);
    assert_eq!(pirate.r3_combat_modifier_bps, 11_500);
    let base_output = damage_output_for_cohort(
        FLEET_COHORT_NUM_SHIPS,
        FLEET_DAMAGE_PER_SHIP_PER_TICK,
    );
    assert!(terran.damage_output > base_output);
    assert!(pirate.damage_output > terran.damage_output);
}

#[test]
fn r6_emission_band_converts_damage_to_ships_destroyed() {
    let (destroyed, after, hp_after, _) =
        emission_band_ship_attrition(250, FLEET_COHORT_NUM_SHIPS, FLEET_HP_PER_SHIP);
    assert_eq!(destroyed, 2);
    assert_eq!(after, 8);
    assert_eq!(hp_after, 800);
}

#[test]
fn r6_partial_damage_below_hp_per_ship_kills_zero_ships() {
    let (destroyed, after, hp_after, zero) =
        emission_band_ship_attrition(75, FLEET_COHORT_NUM_SHIPS, FLEET_HP_PER_SHIP);
    assert_eq!(destroyed, 0);
    assert_eq!(after, FLEET_COHORT_NUM_SHIPS);
    assert_eq!(hp_after, 1000);
    assert!(!zero);
}

#[test]
fn r6_canonical_500_damage_kills_five_of_ten_ships() {
    let (destroyed, after, hp_after, _) =
        emission_band_ship_attrition(500, FLEET_COHORT_NUM_SHIPS, FLEET_HP_PER_SHIP);
    assert_eq!(destroyed, 5);
    assert_eq!(after, 5);
    assert_eq!(hp_after, 500);
    let admitted = report();
    assert!(admitted.combat_arena_rows.iter().any(|row| {
        row.hostile_damage_received >= 500 && row.ships_destroyed == 5 && row.num_ships_after == 5
    }));
}

#[test]
fn r6_overkill_clamps_ships_destroyed_to_num_ships() {
    let (destroyed, after, hp_after, zero) =
        emission_band_ship_attrition(1200, FLEET_COHORT_NUM_SHIPS, FLEET_HP_PER_SHIP);
    assert_eq!(destroyed, 10);
    assert_eq!(after, 0);
    assert_eq!(hp_after, 0);
    assert!(zero);
}

#[test]
fn r6_partial_attrition_reduces_num_ships_without_removing_cohort() {
    let admitted = report();
    let partial = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.num_ships_after > 0 && row.ships_destroyed > 0)
        .expect("partial attrition row");
    assert!(partial.num_ships_after < partial.num_ships_before);
    assert!(!partial.zero_cohort_event_emitted);
}

#[test]
fn r6_zero_remaining_ships_emits_removal_event() {
    let admitted = report();
    assert!(
        admitted
            .combat_arena_rows
            .iter()
            .any(|row| row.zero_cohort_event_emitted && row.num_ships_after == 0)
    );
}

#[test]
fn r6_zero_remaining_ships_removed_from_arena_membership() {
    let admitted = report();
    let removed = admitted
        .defeated_rows
        .iter()
        .find(|row| row.removal_applied)
        .expect("defeated removal row");
    let arena_row = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.combatant_id == removed.combatant_id)
        .expect("arena row");
    assert_eq!(arena_row.num_ships_after, 0);
    assert!(!arena_row
        .arena_membership_after
        .contains(&arena_row.entity_id));
}

#[test]
fn r6_survivor_remains_enrolled_with_owner_overlay_and_reduced_num_ships() {
    let admitted = report();
    assert!(!admitted.survivor_rows.is_empty());
    let survivor = admitted
        .survivor_rows
        .iter()
        .find(|row| row.num_ships_after > 0)
        .expect("survivor with ships");
    let row = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.combatant_id == survivor.combatant_id)
        .expect("survivor arena row");
    assert!(row.arena_membership_after.contains(&row.entity_id));
    assert!(row.owner_overlay_preserved);
    assert!(row.num_ships_after < row.num_ships_before);
}

#[test]
fn r6_identity_preserved_in_combat_event_log() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert!(row.identity_preserved);
        assert_eq!(row.identity_lane, identity_lane_for_owner(row.owner));
    }
}

#[test]
fn r6_no_reparenting_during_combat_or_removal() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert_eq!(row.structural_parent, GALACTIC_STRUCTURAL_PARENT);
    }
}

#[test]
fn r6_no_new_boundary_request_or_movement_command() {
    let admitted = report();
    assert!(!admitted.new_boundary_request);
    assert!(!admitted.direct_movement_command);
}

#[test]
fn r6_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r6_combat_hp_damage();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_eq!(a.summary.stable_checksum, admitted.summary.stable_checksum);
}

#[test]
fn r6_opt_in_default_off() {
    let default = run_dress_rehearsal_r6_combat_hp_damage(&DressRehearsalR6Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.summary.combat_arena_row_count, 0);

    let admitted = report();
    assert!(admitted.explicit_opt_in);
    assert_eq!(admitted.id, DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID);
    assert!(
        DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS.contains("IMPLEMENTED / PASS")
    );
}

#[test]
fn r6_canonical_checksum_pin() {
    let admitted = report();
    assert!(admitted.admitted);
    assert!(admitted.adversarial_resource_flow_arena_used);
    assert_eq!(admitted.r1_contract_checksum, 0x17de_0080_304b_3da7);
    assert_eq!(admitted.r2_contract_checksum, 0x4fe0_5905_89dd_d975);
    assert_eq!(admitted.r3_contract_checksum, 0x28af_b4a2_04d1_01d2);
    assert_eq!(admitted.r4_contract_checksum, 0xf0ac_be2c_cb98_badb);
    assert_eq!(admitted.r5_contract_checksum, 0x5308_a1eb_1b7a_e5fb);
    assert_eq!(admitted.summary.stable_checksum, 7528695422102681985);
}

fn identity_lane_for_owner(owner: DressRehearsalR6Owner) -> u32 {
    match owner {
        DressRehearsalR6Owner::Terran => 0,
        DressRehearsalR6Owner::Pirate => 1,
    }
}
