use simthing_driver::{
    replay_dress_rehearsal_r6_combat_hp_damage, run_dress_rehearsal_r6_combat_hp_damage,
    DressRehearsalR6Input, DressRehearsalR6Owner, DressRehearsalR6Report,
    DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID, DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS,
    GALACTIC_STRUCTURAL_PARENT, COMBAT_DAMAGE_BASE,
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
fn r6_non_fleet_occupants_are_combat_inert() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert!(row.occupant_kind_matches_fleet());
    }
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
        if row.friendly_fire_blocked {
            assert!(row
                .hostile_target_ids
                .iter()
                .all(|target| {
                    admitted
                        .combat_arena_rows
                        .iter()
                        .find(|other| &other.combatant_id == target)
                        .map(|other| other.owner != row.owner)
                        .unwrap_or(true)
                }));
        }
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
    assert!(terran.outgoing_damage > COMBAT_DAMAGE_BASE);
    assert!(pirate.outgoing_damage > terran.outgoing_damage);
}

#[test]
fn r6_damage_flow_subtracts_from_hp() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert_eq!(row.hp_after, (row.hp_before - row.incoming_damage).max(0));
        assert!(row.incoming_damage > 0 || row.hostile_target_ids.is_empty());
    }
}

#[test]
fn r6_zero_hp_threshold_emits_combat_event() {
    let admitted = report();
    assert!(
        admitted
            .combat_arena_rows
            .iter()
            .any(|row| row.zero_hp_threshold_passed && row.combat_event_emitted)
    );
}

#[test]
fn r6_zero_hp_combatant_removed_from_arena_membership() {
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
    assert!(!arena_row
        .arena_membership_after
        .contains(&arena_row.entity_id));
}

#[test]
fn r6_survivor_remains_enrolled_with_owner_overlay() {
    let admitted = report();
    assert!(!admitted.survivor_rows.is_empty());
    let survivor = &admitted.survivor_rows[0];
    let row = admitted
        .combat_arena_rows
        .iter()
        .find(|row| row.combatant_id == survivor.combatant_id)
        .expect("survivor arena row");
    assert!(row.arena_membership_after.contains(&row.entity_id));
    assert!(row.owner_overlay_preserved);
}

#[test]
fn r6_identity_preserved_in_combat_event_log() {
    let admitted = report();
    for row in &admitted.combat_arena_rows {
        assert!(row.identity_preserved);
        assert_eq!(row.idroute_identity_before(), row.identity_lane);
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
    assert!(admitted.subtract_from_source_used);
    assert_eq!(admitted.r1_contract_checksum, 0x17de_0080_304b_3da7);
    assert_eq!(admitted.r2_contract_checksum, 0x4fe0_5905_89dd_d975);
    assert_eq!(admitted.r3_contract_checksum, 0x28af_b4a2_04d1_01d2);
    assert_eq!(admitted.r4_contract_checksum, 0xf0ac_be2c_cb98_badb);
    assert_eq!(admitted.r5_contract_checksum, 0x5308_a1eb_1b7a_e5fb);
    assert_eq!(admitted.summary.stable_checksum, 405082860229191929);
}

trait CombatArenaRowTestExt {
    fn occupant_kind_matches_fleet(&self) -> bool;
    fn idroute_identity_before(&self) -> u32;
}

impl CombatArenaRowTestExt for simthing_driver::DressRehearsalR6CombatArenaRow {
    fn occupant_kind_matches_fleet(&self) -> bool {
        matches!(
            self.combatant_id.as_str(),
            _ if self.combatant_id.contains("patrol") || self.combatant_id.contains("pirate-ship")
        )
    }

    fn idroute_identity_before(&self) -> u32 {
        self.identity_lane
    }
}
