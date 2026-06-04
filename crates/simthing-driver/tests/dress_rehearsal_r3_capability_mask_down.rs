use simthing_driver::{
    cpu_oracle_dress_rehearsal_r3_capability_mask_down,
    replay_dress_rehearsal_r3_capability_mask_down, run_dress_rehearsal_r3_capability_mask_down,
    DressRehearsalR3Input, DressRehearsalR3Owner, DressRehearsalR3Report, BLOCKADE_DIVERT_MODIFIER,
    COMBAT_BONUS_PLACEHOLDER_MODIFIER, DEFENSIVE_LOGISTICS_MODIFIER, DISRUPTION_DECAY_MODIFIER,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS, MIN_MODIFIER_BPS,
    PATROL_SUPPRESSION_MODIFIER, PIRATE_EMISSION_MODIFIER, RAIDING_LOGISTICS_MODIFIER,
};

fn report() -> DressRehearsalR3Report {
    run_dress_rehearsal_r3_capability_mask_down(&DressRehearsalR3Input::explicit_opt_in())
}

#[test]
fn r3_consumes_r2_and_r1_contracts() {
    let admitted = report();
    assert_eq!(admitted.id, DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID);
    assert!(DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS.contains("IMPLEMENTED / PASS"));
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.r1_contract_consumed);
    assert_eq!(admitted.r1_contract_checksum, 0x17de_0080_304b_3da7);
    assert!(admitted.r1_cpu_oracle_parity);
    assert!(admitted.r2_contract_consumed);
    assert_eq!(admitted.r2_contract_checksum, 0x4fe0_5905_89dd_d975);
    assert!(admitted.r2_cpu_oracle_parity);
    assert!(admitted.store_owner_layout_consumed);
    assert!(admitted.single_galactic_tier);
}

#[test]
fn r3_resolves_terran_capability_tree_to_modifier_overlays() {
    let admitted = report();
    let terran_caps: Vec<_> = admitted
        .capability_rows
        .iter()
        .filter(|row| row.owner == DressRehearsalR3Owner::Terran)
        .collect();
    assert_eq!(terran_caps.len(), 4);
    assert!(terran_caps
        .iter()
        .all(|row| row.faction_simthing_id == "faction-terran"));

    for modifier_id in [
        PATROL_SUPPRESSION_MODIFIER,
        DISRUPTION_DECAY_MODIFIER,
        DEFENSIVE_LOGISTICS_MODIFIER,
        COMBAT_BONUS_PLACEHOLDER_MODIFIER,
    ] {
        let modifier = admitted
            .modifier_overlay_rows
            .iter()
            .find(|row| {
                row.owner == DressRehearsalR3Owner::Terran && row.modifier_id == modifier_id
            })
            .expect("Terran modifier");
        assert!(modifier.multiplier_bps >= MIN_MODIFIER_BPS);
        assert!(modifier.read_side_only);
    }
}

#[test]
fn r3_resolves_pirate_capability_tree_to_modifier_overlays() {
    let admitted = report();
    let pirate_caps: Vec<_> = admitted
        .capability_rows
        .iter()
        .filter(|row| row.owner == DressRehearsalR3Owner::Pirate)
        .collect();
    assert_eq!(pirate_caps.len(), 4);
    assert!(pirate_caps
        .iter()
        .all(|row| row.faction_simthing_id == "faction-pirate"));

    for modifier_id in [
        PIRATE_EMISSION_MODIFIER,
        BLOCKADE_DIVERT_MODIFIER,
        RAIDING_LOGISTICS_MODIFIER,
        COMBAT_BONUS_PLACEHOLDER_MODIFIER,
    ] {
        let modifier = admitted
            .modifier_overlay_rows
            .iter()
            .find(|row| {
                row.owner == DressRehearsalR3Owner::Pirate && row.modifier_id == modifier_id
            })
            .expect("Pirate modifier");
        assert!(modifier.multiplier_bps >= MIN_MODIFIER_BPS);
        assert!(modifier.read_side_only);
    }
}

#[test]
fn r3_masks_modifiers_down_by_owner_column() {
    let admitted = report();
    assert!(admitted
        .owner_mask_application_rows
        .iter()
        .all(|row| row.owner_column_matched));

    let terran_rows: Vec<_> = admitted
        .owner_mask_application_rows
        .iter()
        .filter(|row| row.owner == DressRehearsalR3Owner::Terran)
        .collect();
    let pirate_rows: Vec<_> = admitted
        .owner_mask_application_rows
        .iter()
        .filter(|row| row.owner == DressRehearsalR3Owner::Pirate)
        .collect();

    assert!(terran_rows
        .iter()
        .any(|row| row.modifier_id == PATROL_SUPPRESSION_MODIFIER));
    assert!(terran_rows
        .iter()
        .any(|row| row.modifier_id == DEFENSIVE_LOGISTICS_MODIFIER));
    assert!(pirate_rows
        .iter()
        .any(|row| row.modifier_id == PIRATE_EMISSION_MODIFIER));
    assert!(pirate_rows
        .iter()
        .any(|row| row.modifier_id == BLOCKADE_DIVERT_MODIFIER));
    assert!(!terran_rows
        .iter()
        .any(|row| row.modifier_id == PIRATE_EMISSION_MODIFIER));
    assert!(!pirate_rows
        .iter()
        .any(|row| row.modifier_id == PATROL_SUPPRESSION_MODIFIER));
}

#[test]
fn r3_colocated_terran_and_pirate_occupants_receive_distinct_modifiers() {
    let admitted = report();
    let colocated: Vec<_> = admitted
        .owner_mask_application_rows
        .iter()
        .filter(|row| row.evidence_group == "galactic-colocation-owner-mask")
        .collect();
    assert_eq!(colocated.len(), 2);
    assert_eq!(colocated[0].cell_index, colocated[1].cell_index);
    assert_eq!(
        (colocated[0].x, colocated[0].y),
        (colocated[1].x, colocated[1].y)
    );
    assert_ne!(colocated[0].owner, colocated[1].owner);
    assert_ne!(colocated[0].modifier_id, colocated[1].modifier_id);
}

#[test]
fn r3_patrol_suppression_modifier_changes_effective_suppression() {
    let admitted = report();
    let row = admitted
        .modified_r1_signal_rows
        .iter()
        .find(|row| row.channel == "PatrolSuppression")
        .expect("patrol suppression source row");
    assert_eq!(row.owner, DressRehearsalR3Owner::Terran);
    assert_eq!(row.modifier_id, PATROL_SUPPRESSION_MODIFIER);
    assert_ne!(row.effective_value, row.base_value);
    assert!(row.effective_value.abs() > row.base_value.abs());
}

#[test]
fn r3_pirate_emission_modifier_changes_effective_disruption() {
    let admitted = report();
    let row = admitted
        .modified_r1_signal_rows
        .iter()
        .find(|row| row.channel == "PirateDisruption")
        .expect("pirate disruption source row");
    assert_eq!(row.owner, DressRehearsalR3Owner::Pirate);
    assert_eq!(row.modifier_id, PIRATE_EMISSION_MODIFIER);
    assert!(row.effective_value > row.base_value);
}

#[test]
fn r3_blockade_or_allocation_modifier_changes_bounded_economy_signal() {
    let admitted = report();
    let blockade = admitted
        .modified_economy_signal_rows
        .iter()
        .find(|row| row.modifier_id == BLOCKADE_DIVERT_MODIFIER)
        .expect("blockade/divert economy signal");
    assert_eq!(blockade.owner, DressRehearsalR3Owner::Pirate);
    assert!(blockade.effective_signal > blockade.base_signal);
    assert!(blockade.bounded_signal <= 100.0);

    let terran_logistics = admitted
        .modified_economy_signal_rows
        .iter()
        .find(|row| row.modifier_id == DEFENSIVE_LOGISTICS_MODIFIER)
        .expect("Terran logistics signal");
    assert!(terran_logistics.effective_signal > terran_logistics.base_signal);
}

#[test]
fn r3_modifier_application_is_read_side_and_does_not_reparent() {
    let admitted = report();
    assert!(admitted
        .modifier_overlay_rows
        .iter()
        .all(|row| row.read_side_only));
    assert!(admitted
        .owner_mask_application_rows
        .iter()
        .all(|row| row.structural_parent_before == row.structural_parent_after));
    assert_eq!(admitted.reparented_occupant_count, 0);
    assert!(!admitted.boundary_request_emitted);
}

#[test]
fn r3_tree_and_occupant_positions_unchanged() {
    let admitted = report();
    assert!(admitted.capability_tree_unchanged);
    assert_eq!(
        admitted.capability_tree_before_checksum,
        admitted.capability_tree_after_checksum
    );
    assert_eq!(
        admitted.occupant_positions_before,
        admitted.occupant_positions_after
    );
    assert!(!admitted.sead_action_emitted);
    assert!(!admitted.gradientxy_consumed);
}

#[test]
fn r3_combat_bonus_resolves_as_data_but_does_not_resolve_combat() {
    let admitted = report();
    assert!(admitted.combat_bonus_resolved_as_data);
    assert!(admitted
        .modifier_overlay_rows
        .iter()
        .any(|row| row.modifier_id == COMBAT_BONUS_PLACEHOLDER_MODIFIER));
    assert_eq!(admitted.combat_resolution_events, 0);
    assert_eq!(admitted.hostile_hp_delta, 0);
}

#[test]
fn r3_deterministic_replay_and_cpu_oracle_parity() {
    let (left, right) = replay_dress_rehearsal_r3_capability_mask_down();
    assert_eq!(left, right);
    assert!(left.cpu_oracle_parity);
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );

    let input = DressRehearsalR3Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r3_capability_mask_down(&input);
    let oracle = cpu_oracle_dress_rehearsal_r3_capability_mask_down(&input);
    assert_eq!(admitted.modifier_overlay_rows, oracle.modifier_rows);
    assert_eq!(admitted.owner_mask_application_rows, oracle.owner_mask_rows);
    assert_eq!(admitted.modified_r1_signal_rows, oracle.modified_r1_rows);
    assert_eq!(
        admitted.modified_economy_signal_rows,
        oracle.modified_economy_rows
    );
    assert_eq!(admitted.summary, oracle.summary);
}

#[test]
fn r3_opt_in_default_off() {
    let disabled =
        run_dress_rehearsal_r3_capability_mask_down(&DressRehearsalR3Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.modifier_overlay_rows.is_empty());
    assert!(!disabled.default_simsession_pass_graph_change);

    let mut default_on = DressRehearsalR3Input::explicit_opt_in();
    default_on.enabled_by_default = true;
    let rejected = run_dress_rehearsal_r3_capability_mask_down(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"r3_default_on_rejected"));
}
