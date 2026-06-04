use simthing_driver::{
    replay_dress_rehearsal_r5_movement_reenroll, run_dress_rehearsal_r4_sead_field_consumption,
    run_dress_rehearsal_r5_movement_reenroll, DressRehearsalR4Decision, DressRehearsalR4Input,
    DressRehearsalR4Report, DressRehearsalR5Input, DressRehearsalR5Report,
    DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID, DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS,
    GALACTIC_STRUCTURAL_PARENT,
};
fn report() -> DressRehearsalR5Report {
    run_dress_rehearsal_r5_movement_reenroll(&DressRehearsalR5Input::explicit_opt_in())
}

fn r4_report() -> DressRehearsalR4Report {
    run_dress_rehearsal_r4_sead_field_consumption(&DressRehearsalR4Input::explicit_opt_in())
}

#[test]
fn r5_consumes_r4_step_opportunity_contract() {
    let admitted = report();
    let r4 = r4_report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert_eq!(admitted.r4_contract_checksum, r4.summary.stable_checksum);
    assert_eq!(admitted.r4_contract_checksum, 0xf0ac_be2c_cb98_badb);
    assert!(
        admitted
            .movement_rows
            .iter()
            .all(|row| row.r4_decision_consumed == "StepOpportunity")
    );
}

#[test]
fn r5_sit_still_rows_do_not_emit_boundary_request() {
    let admitted = report();
    let r4 = r4_report();
    let sit_movers: Vec<_> = r4
        .mover_rows
        .iter()
        .filter(|row| row.decision == DressRehearsalR4Decision::SitStill)
        .map(|row| row.mover_id.as_str())
        .collect();
    for mover_id in sit_movers {
        assert!(
            !admitted
                .boundary_request_rows
                .iter()
                .any(|row| row.mover_id == mover_id),
            "sit-still mover {mover_id} must not emit boundary request"
        );
        assert!(
            admitted
                .sit_still_rows
                .iter()
                .any(|row| row.mover_id == mover_id)
        );
    }
}

#[test]
fn r5_step_opportunity_emits_event_and_boundary_request() {
    let admitted = report();
    assert!(!admitted.boundary_request_rows.is_empty());
    for row in &admitted.boundary_request_rows {
        assert!(row.event_emitted);
        assert!(row.materialized_from_r4_step_opportunity);
        assert_ne!(row.source_cell_index, row.destination_cell_index);
    }
}

#[test]
fn r5_routes_boundary_request_through_mobility_substrate() {
    let admitted = report();
    assert!(
        admitted.mobility_substrate_admitted,
        "mobility diagnostics {:?} report {:?}",
        admitted.mobility_substrate_diagnostics,
        admitted.diagnostics
    );
    assert!(!admitted.direct_movement_command);
    assert!(!admitted.external_boundary_request);
}

#[test]
fn r5_reenrolls_mover_from_source_cell_to_destination_cell() {
    let admitted = report();
    for row in &admitted.movement_rows {
        assert!(row.movement_applied, "mover {}", row.mover_id);
        assert_eq!(row.post_move_cell_index, row.destination_cell_index);
        assert!(
            !row
                .source_arena_membership_after
                .contains(&row.entity_id)
        );
        assert!(
            row.destination_arena_membership_after
                .contains(&row.entity_id)
        );
    }
}

#[test]
fn r5_preserves_idroute_identity() {
    let admitted = report();
    for row in &admitted.movement_rows {
        assert_eq!(
            row.idroute_identity_before,
            row.idroute_identity_after,
            "{}",
            row.mover_id
        );
    }
}

#[test]
fn r5_preserves_owner_overlay() {
    let admitted = report();
    for row in &admitted.movement_rows {
        assert_eq!(
            row.owner_faction_id_before,
            row.owner_faction_id_after,
            "{}",
            row.mover_id
        );
    }
}

#[test]
fn r5_updates_arena_membership_without_reparenting() {
    let admitted = report();
    for row in &admitted.movement_rows {
        assert_eq!(row.structural_parent_before, GALACTIC_STRUCTURAL_PARENT);
        assert_eq!(row.structural_parent_after, GALACTIC_STRUCTURAL_PARENT);
    }
}

#[test]
fn r5_movement_changes_field_membership_for_next_step() {
    let admitted = report();
    for row in &admitted.movement_rows {
        assert_ne!(row.source_cell_index, row.post_move_cell_index);
    }
}

#[test]
fn r5_no_direct_movement_command() {
    let admitted = report();
    assert!(!admitted.direct_movement_command);
}

#[test]
fn r5_no_external_boundary_request() {
    let admitted = report();
    assert!(!admitted.external_boundary_request);
    for row in &admitted.boundary_request_rows {
        assert!(row.materialized_from_r4_step_opportunity);
    }
}

#[test]
fn r5_no_cpu_planner() {
    let admitted = report();
    assert!(!admitted.cpu_planner_used);
}

#[test]
fn r5_starport_fission_emits_new_fleet_if_substrate_available() {
    let admitted = report();
    assert!(admitted.fission_substrate_available);
    assert!(!admitted.fission_rows.is_empty());
    assert!(admitted
        .fission_rows
        .iter()
        .any(|row| row.fission_applied && !row.new_fleet_id.is_empty()));
}

#[test]
fn r5_new_fleet_enrolled_with_owner_overlay_if_fission_available() {
    let admitted = report();
    let row = admitted
        .fission_rows
        .first()
        .expect("fission row");
    assert!(row.fission_applied);
    assert!(row.owner_faction_id > 0);
    assert!(!row.starport_id.is_empty());
}

#[test]
fn r5_deterministic_replay_and_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let (a, b) = replay_dress_rehearsal_r5_movement_reenroll();
    assert_eq!(a.summary.stable_checksum, b.summary.stable_checksum);
    assert_eq!(a.summary.stable_checksum, admitted.summary.stable_checksum);
}

#[test]
fn r5_opt_in_default_off() {
    let default = run_dress_rehearsal_r5_movement_reenroll(&DressRehearsalR5Input::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.summary.movement_row_count, 0);

    let admitted = report();
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
    assert_eq!(admitted.id, DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID);
    assert!(
        DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS.contains("IMPLEMENTED / PASS")
    );
}

#[test]
fn r5_canonical_checksum_pin() {
    let admitted = report();
    assert!(admitted.admitted);
    assert!(admitted.summary.movement_row_count > 0);
    assert_eq!(admitted.r1_contract_checksum, 0x17de_0080_304b_3da7);
    assert_eq!(admitted.r2_contract_checksum, 0x4fe0_5905_89dd_d975);
    assert_eq!(admitted.r3_contract_checksum, 0x28af_b4a2_04d1_01d2);
    assert_eq!(admitted.r4_contract_checksum, 0xf0ac_be2c_cb98_badb);
    // Pin after first green run — update if fixture inputs change intentionally.
    assert_eq!(admitted.summary.stable_checksum, 0x5308_a1eb_1b7a_e5fb);
}
