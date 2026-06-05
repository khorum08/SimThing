use simthing_driver::{
    cpu_oracle_dress_rehearsal_r6c_integrated_run, replay_dress_rehearsal_r6c_integrated_run,
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cDetectorStatus,
    DressRehearsalR6cInput, DressRehearsalR6cOwner, DressRehearsalR6cReport,
    DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID, DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS,
    R6C_CANONICAL_TICK_COUNT, R6C_GPU_POSTURE, R6C_TIE_BREAKER_POLICY,
};
use std::sync::OnceLock;

static REPORT: OnceLock<DressRehearsalR6cReport> = OnceLock::new();

fn report() -> &'static DressRehearsalR6cReport {
    REPORT.get_or_init(|| {
        run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in())
    })
}

#[test]
fn r6c_opt_in_default_off() {
    let default =
        run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.tick_count, 0);
    assert!(default.movement_rows.is_empty());

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert_eq!(admitted.id, DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID);
    assert!(DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS.contains("IMPLEMENTED / PASS"));
}

#[test]
fn r6c_seeds_one_mutable_world_state() {
    let admitted = report();
    let initial = admitted.initial_world.as_ref().expect("initial world");
    assert!(admitted.single_mutable_world_state);
    assert_eq!(initial.systems.len(), 13);
    assert_eq!(admitted.world_seed_summary.terran_system_count, 10);
    assert_eq!(admitted.world_seed_summary.pirate_system_count, 3);
    assert_eq!(admitted.world_seed_summary.starport_count, 4);
    assert_eq!(admitted.world_seed_summary.initial_fleet_cohort_count, 13);
    assert_eq!(admitted.world_seed_summary.initial_pirate_ships, 10);
    assert_eq!(admitted.world_seed_summary.initial_terran_ships, 3);
}

#[test]
fn r6c_runs_canonical_100_ticks() {
    let admitted = report();
    assert_eq!(admitted.tick_count, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(admitted.summary.tick_count, R6C_CANONICAL_TICK_COUNT);
    assert_eq!(
        admitted.summary.tick_row_count,
        R6C_CANONICAL_TICK_COUNT as usize
    );
    assert_eq!(
        admitted.race_curve.len(),
        R6C_CANONICAL_TICK_COUNT as usize + 1
    );
}

#[test]
fn r6c_tick_order_matches_spec() {
    let admitted = report();
    assert_eq!(
        admitted.tick_order,
        vec![
            "R1 disruption recurrence from current fleet positions",
            "R2 labor-to-production reduce-up disburse-down blockade/divert",
            "R3 capability overlays owner-mask down",
            "R4 SEAD field read GradientXY exact-mag2 Candidate-F threshold",
            "R5 movement BoundaryRequest REENROLL fresh-read substeps",
            "R6 combat from movement-produced co-location",
            "R6B production reinforcement birth fusion",
            "write_back positions ships stockpiles disruption production blockade/divert",
        ]
    );
}

#[test]
fn r6c_write_back_changes_next_tick_inputs() {
    let admitted = report();
    assert!(admitted.write_back_confirmed);

    let mut carried_stockpile = false;
    for row in &admitted.stockpile_ledger_rows {
        if let Some(next) = admitted
            .stockpile_ledger_rows
            .iter()
            .find(|next| next.owner == row.owner && next.tick == row.tick + 1)
        {
            assert_eq!(row.after_disburse_down, next.before_reduce_up);
            carried_stockpile = true;
            break;
        }
    }
    assert!(carried_stockpile);

    let movement_feedback = admitted.movement_rows.iter().any(|movement| {
        admitted.disruption_source_rows.iter().any(|source| {
            source.tick == movement.tick + 1
                && source.fleet_id == movement.mover_id
                && source.cell_index == movement.destination_cell_index
        })
    });
    assert!(
        movement_feedback,
        "a moved fleet should feed the next R1 source cell"
    );
}

#[test]
fn r6c_disruption_uses_current_fleet_positions() {
    let admitted = report();
    assert!(!admitted.disruption_source_rows.is_empty());
    assert!(admitted.disruption_source_rows.iter().any(|row| {
        row.owner == DressRehearsalR6cOwner::Pirate && row.input_cell > 0.0 && row.num_ships > 0
    }));
    assert!(admitted.disruption_source_rows.iter().any(|row| {
        row.owner == DressRehearsalR6cOwner::Terran && row.input_cell < 0.0 && row.num_ships > 0
    }));
}

#[test]
fn r6c_economy_and_stockpiles_carry_forward() {
    let admitted = report();
    assert!(admitted
        .economy_rows
        .iter()
        .any(|row| row.production_generated == 1));
    assert!(admitted
        .stockpile_ledger_rows
        .iter()
        .any(|row| row.after_disburse_down > 0));
    assert!(admitted
        .economy_rows
        .iter()
        .any(|row| row.blockaded && row.owner_column_flipped && row.diverted_production > 0));
}

#[test]
fn r6c_capability_overlays_feed_field_consumption() {
    let admitted = report();
    assert!(admitted
        .capability_overlay_rows
        .iter()
        .any(|row| row.consumed_by_field && row.multiplier_bps != 10_000));
    assert!(admitted
        .field_read_rows
        .iter()
        .any(|row| row.capability_component_bps != 10_000));
}

#[test]
fn r6c_movement_is_field_attributable() {
    let admitted = report();
    assert!(!admitted.movement_rows.is_empty());
    for row in admitted
        .field_read_rows
        .iter()
        .filter(|row| row.decision == "StepOpportunity")
    {
        assert!(row.real_signal_gradient_magnitude_bits > 0);
        assert!(
            row.disruption_component.abs() > 0.0 || row.economy_component.abs() > 0.0,
            "movement should be attributable to live field components"
        );
    }
}

#[test]
fn r6c_tiebreaker_dropped_or_dominated() {
    let admitted = report();
    assert_eq!(admitted.tie_breaker_policy, R6C_TIE_BREAKER_POLICY);
    assert_eq!(admitted.tie_breaker_policy, "Dropped");
    assert!(admitted
        .field_read_rows
        .iter()
        .all(|row| row.tie_breaker_gradient_magnitude_bits == 0));
}

#[test]
fn r6c_movement_uses_boundary_request_and_reenroll() {
    let admitted = report();
    assert_eq!(
        admitted.boundary_request_rows.len(),
        admitted.movement_rows.len()
    );
    assert!(admitted
        .boundary_request_rows
        .iter()
        .all(|row| { row.event_emitted && row.materialized_from_step_opportunity }));
    assert!(admitted.movement_rows.iter().all(|row| {
        row.r4_decision_consumed == "StepOpportunity"
            && row.event_emitted
            && row.movement_applied
            && row.idroute_identity_before == row.idroute_identity_after
            && row.owner_faction_id_before == row.owner_faction_id_after
            && !row.destination_arena_membership_after.is_empty()
    }));
}

#[test]
fn r6c_combat_colocation_is_movement_produced() {
    let admitted = report();
    assert!(!admitted.combat_rows.is_empty());
    assert!(admitted
        .combat_rows
        .iter()
        .all(|row| row.movement_produced_colocation));
}

#[test]
fn r6c_combat_uses_fleet_cohort_resource_flow() {
    let admitted = report();
    assert!(!admitted.combat_reduce_rows.is_empty());
    assert!(!admitted.combat_disburse_rows.is_empty());
    assert!(admitted.combat_rows.iter().any(|row| {
        let bps = match row.owner {
            DressRehearsalR6cOwner::Terran => 10_500,
            DressRehearsalR6cOwner::Pirate => 11_500,
        };
        row.damage_output == row.num_ships_before * row.damage_per_ship_per_tick * bps / 10_000
            && !row.hostile_target_ids.is_empty()
    }));
}

#[test]
fn r6c_production_reinforces_or_births_fleet_cohorts() {
    let admitted = report();
    assert!(admitted
        .construction_rows
        .iter()
        .any(|row| row.threshold_passed));
    assert!(!admitted.reinforcement_rows.is_empty() || !admitted.birth_rows.is_empty());
    for row in &admitted.reinforcement_rows {
        assert_eq!(
            row.num_ships_after,
            row.num_ships_before + row.ship_count_delta
        );
        assert!(!row.movement_boundary_request_used);
    }
    for row in &admitted.birth_rows {
        assert!(row.alloc_enrollment_applied);
        assert!(!row.movement_boundary_request_used);
    }
}

#[test]
fn r6c_friendly_fleets_fuse_when_compatible() {
    let admitted = report();
    assert!(!admitted.fusion_rows.is_empty());
    for row in &admitted.fusion_rows {
        assert_eq!(
            row.left_num_ships + row.right_num_ships,
            row.fused_num_ships
        );
        assert!(row.identity_lineage_recorded);
        assert!(row.owner_overlay_preserved);
        assert!(!row.movement_boundary_request_used);
    }
}

#[test]
fn r6c_ship_conservation_only_combat_or_production() {
    let admitted = report();
    assert_eq!(
        admitted.conservation_rows.len(),
        R6C_CANONICAL_TICK_COUNT as usize
    );
    for row in &admitted.conservation_rows {
        assert!(row.ship_delta_explained);
        assert!(row.positions_changed_by_r5_only);
        if row.ships_before != row.ships_after {
            assert!(row.ships_destroyed_by_combat > 0 || row.ships_created_by_production > 0);
        }
    }
}

#[test]
fn r6c_identity_and_owner_preserved() {
    let admitted = report();
    assert!(admitted.movement_rows.iter().all(|row| {
        row.idroute_identity_before == row.idroute_identity_after
            && row.owner_faction_id_before == row.owner_faction_id_after
    }));
    assert!(admitted
        .combat_rows
        .iter()
        .all(|row| { row.identity_preserved && row.owner_overlay_preserved }));
    assert!(admitted
        .final_world
        .as_ref()
        .expect("final world")
        .fleets
        .iter()
        .all(|fleet| fleet.owner_faction_id == fleet.owner.stable_code()));
}

#[test]
fn r6c_detector_table_populated() {
    let admitted = report();
    assert_eq!(admitted.detector_rows.len(), 14);
    assert_eq!(admitted.summary.detector_row_count, 14);
    assert!(admitted
        .detector_rows
        .iter()
        .any(|row| { row.status == DressRehearsalR6cDetectorStatus::Emerged }));
    assert!(admitted.detector_rows.iter().any(|row| {
        row.status == DressRehearsalR6cDetectorStatus::NotObserved
            && row.cause_if_not_observed.is_some()
    }));
}

#[test]
fn r6c_race_equilibrium_curve_emitted() {
    let admitted = report();
    assert_eq!(
        admitted.race_curve.len(),
        R6C_CANONICAL_TICK_COUNT as usize + 1
    );
    let first = admitted.race_curve.first().expect("first sample");
    let last = admitted.race_curve.last().expect("last sample");
    assert_eq!(first.terran_ships, 3);
    assert_eq!(first.pirate_ships, 10);
    assert_eq!(last.sample, R6C_CANONICAL_TICK_COUNT);
    assert!(admitted
        .race_curve
        .iter()
        .any(|row| row.blockaded_system_count > 0));
}

#[test]
fn r6c_deterministic_replay_checksum_stable() {
    let admitted = report();
    assert_ne!(admitted.deterministic_replay_checksum, 0);
    let (a, b) = replay_dress_rehearsal_r6c_integrated_run();
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
    assert_eq!(
        a.deterministic_replay_checksum,
        admitted.deterministic_replay_checksum
    );
}

#[test]
fn r6c_cpu_oracle_parity() {
    let admitted = report();
    assert!(admitted.cpu_oracle_parity);
    let oracle =
        cpu_oracle_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    assert_eq!(
        oracle.summary.stable_checksum,
        admitted.summary.stable_checksum
    );
    assert_eq!(
        oracle.final_world,
        admitted.final_world.as_ref().expect("final world").clone()
    );
}

#[test]
fn r6c_reports_gpu_conformance_without_unmeasured_gpu_claim() {
    let admitted = report();
    assert_eq!(admitted.gpu_posture, R6C_GPU_POSTURE);
    assert_eq!(
        admitted.gpu_posture,
        "GPU-conformant; GPU execution not yet measured"
    );
    assert!(!admitted.artifact_markdown.contains("validated on GPU"));
}
