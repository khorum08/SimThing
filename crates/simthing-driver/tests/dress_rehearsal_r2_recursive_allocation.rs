use simthing_driver::{
    cpu_oracle_dress_rehearsal_r2_recursive_allocation,
    replay_dress_rehearsal_r2_recursive_allocation, run_dress_rehearsal_r1_disruption_heatmap,
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR1Channel, DressRehearsalR1Input,
    DressRehearsalR1OccupantContribution, DressRehearsalR1OccupantKind, DressRehearsalR1Owner,
    DressRehearsalR1Scenario, DressRehearsalR2Input, DressRehearsalR2Owner, DressRehearsalR2Report,
    BLOCKADE_THRESHOLD, DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_ID,
    DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_STATUS_PASS, FACTORY_UNIT_COST_LABOR,
    GALAXY_CELL_COUNT, PIRATE_EMIT, POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE,
    STARPORT_PRODUCTION_NEED,
};

fn report() -> DressRehearsalR2Report {
    run_dress_rehearsal_r2_recursive_allocation(&DressRehearsalR2Input::explicit_opt_in())
}

fn r2_with_terran_blockade() -> DressRehearsalR2Report {
    let mut scenario = DressRehearsalR1Scenario::canonical();
    let target = scenario
        .occupants
        .iter()
        .find(|occupant| {
            occupant.kind == DressRehearsalR1OccupantKind::System
                && occupant.owner == DressRehearsalR1Owner::Terran
        })
        .expect("canonical Terran system")
        .clone();

    for idx in 0..10 {
        scenario
            .occupants
            .push(DressRehearsalR1OccupantContribution {
                source_id: format!("r2-test-pirate-{idx:02}"),
                kind: DressRehearsalR1OccupantKind::PirateFleet,
                owner: DressRehearsalR1Owner::Pirate,
                x: target.x,
                y: target.y,
                cell_index: target.cell_index,
                channel: DressRehearsalR1Channel::PirateDisruption,
                value: PIRATE_EMIT,
            });
    }

    let r1 = run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::with_scenario(
        scenario, 8,
    ));
    assert!(r1.admitted, "{:?}", r1.diagnostics);
    assert!(r1.cpu_oracle_parity);
    assert_eq!(
        r1.final_disruption[target.cell_index as usize],
        BLOCKADE_THRESHOLD
    );

    run_dress_rehearsal_r2_recursive_allocation(&DressRehearsalR2Input::with_r1_report(r1))
}

#[test]
fn r2_opening_status_matches_track() {
    let admitted = report();
    assert_eq!(admitted.id, DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_ID);
    assert!(DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_STATUS_PASS.contains("IMPLEMENTED / PASS"));
    assert!(admitted.status.contains("recursive allocation"));

    let track = include_str!("../../../docs/design_0_0_8_0_consumer_pulled_production_track.md");
    assert!(track.contains("R2 — Recursive allocation + faction economy + blockade/divert"));
    assert!(
        track.contains("IMPLEMENTED / PASS") || track.contains("OPEN / AUTHORED 2026-06-04"),
        "production track must name the R2 rung status"
    );
}

#[test]
fn r2_consumes_accepted_r1_heatmap() {
    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.r1_heatmap_consumed);
    assert!(admitted.r1_cpu_oracle_parity);
    assert_eq!(admitted.r1_final_disruption_cells, GALAXY_CELL_COUNT);
    assert_eq!(admitted.r1_input_contract_checksum, 0x17de_0080_304b_3da7);
    assert!(admitted
        .production_rows
        .iter()
        .any(|row| row.disruption >= BLOCKADE_THRESHOLD));
}

#[test]
fn r2_factory_recipe_converts_labor_to_production() {
    let admitted = report();
    let first = admitted.production_rows.first().expect("production row");
    assert_eq!(first.labor_generated, POP_LABOR_PER_TICK);
    assert_eq!(first.labor_consumed, FACTORY_UNIT_COST_LABOR);
    assert_eq!(first.labor_remaining, 0);
    assert_eq!(first.production_generated, PRODUCTION_PER_RECIPE);
    assert_eq!(
        admitted.factory_recipe.input_consumption,
        "SubtractFromAllInputs"
    );
    assert!(admitted
        .factory_recipe
        .recipe_shape
        .contains("ConjunctiveCrossing"));
    assert!(admitted.factory_recipe.no_new_op);
}

#[test]
fn r2_production_reduces_up_to_faction_stockpile_owner_masked() {
    let admitted = report();
    let terran_sum: i64 = admitted
        .production_rows
        .iter()
        .filter(|row| row.effective_outflow_owner == DressRehearsalR2Owner::Terran)
        .map(|row| row.outflow_to_effective_owner)
        .sum();
    let pirate_sum: i64 = admitted
        .production_rows
        .iter()
        .filter(|row| row.effective_outflow_owner == DressRehearsalR2Owner::Pirate)
        .map(|row| row.outflow_to_effective_owner)
        .sum();
    let terran_ledger = admitted
        .stockpile_ledger
        .iter()
        .find(|row| row.owner == DressRehearsalR2Owner::Terran)
        .expect("Terran ledger");
    let pirate_ledger = admitted
        .stockpile_ledger
        .iter()
        .find(|row| row.owner == DressRehearsalR2Owner::Pirate)
        .expect("Pirate ledger");

    assert_eq!(terran_ledger.reduced_in, terran_sum);
    assert_eq!(pirate_ledger.reduced_in, pirate_sum);
    assert_eq!(terran_sum + pirate_sum, admitted.summary.total_production);
    assert_ne!(terran_ledger.owner, pirate_ledger.owner);
}

#[test]
fn r2_faction_disburses_surplus_to_deficit_system() {
    let admitted = report();
    let disbursement = admitted
        .deficit_disbursements
        .iter()
        .find(|row| row.disbursed > 0)
        .expect("deficit disbursement");
    assert_eq!(disbursement.requested, STARPORT_PRODUCTION_NEED);
    assert_eq!(disbursement.remaining_deficit, 0);

    let ledger = admitted
        .stockpile_ledger
        .iter()
        .find(|row| row.owner == disbursement.owner)
        .expect("ledger for disbursing owner");
    assert!(ledger.after_disburse_down < ledger.after_reduce_up);
    assert_eq!(
        admitted.summary.total_disbursed,
        admitted
            .deficit_disbursements
            .iter()
            .map(|row| row.disbursed)
            .sum::<i64>()
    );
}

#[test]
fn r2_blockade_threshold_gates_outflow_at_100() {
    let admitted = r2_with_terran_blockade();
    let row = admitted
        .production_rows
        .iter()
        .find(|row| row.original_owner == DressRehearsalR2Owner::Terran && row.blockaded)
        .expect("blockaded Terran row");
    assert_eq!(row.disruption, BLOCKADE_THRESHOLD);
    assert_eq!(row.outflow_to_original_owner, 0);
    assert_eq!(row.diverted_production, row.production_generated);
}

#[test]
fn r2_divert_flips_production_owner_column_to_blockader() {
    let admitted = r2_with_terran_blockade();
    let diverted = admitted
        .diverted_production_rows
        .iter()
        .find(|row| row.original_owner == DressRehearsalR2Owner::Terran)
        .expect("Terran production diverted to blockader");
    assert_eq!(diverted.blockader_owner, DressRehearsalR2Owner::Pirate);
    assert_eq!(diverted.owner_column_before, DressRehearsalR2Owner::Terran);
    assert_eq!(diverted.owner_column_after, DressRehearsalR2Owner::Pirate);
    assert_eq!(diverted.production, PRODUCTION_PER_RECIPE);
}

#[test]
fn r2_divert_is_owner_column_flip_not_reparenting() {
    let admitted = r2_with_terran_blockade();
    let row = admitted
        .production_rows
        .iter()
        .find(|row| row.original_owner == DressRehearsalR2Owner::Terran && row.blockaded)
        .expect("blockaded Terran row");
    assert!(row.owner_column_flipped);
    assert_ne!(row.original_owner, row.effective_outflow_owner);
    assert_eq!(row.structural_parent_before, row.structural_parent_after);
    assert_eq!(admitted.reparented_system_count, 0);
}

#[test]
fn r2_no_occupant_relocated_and_no_boundary_request() {
    let admitted = report();
    assert_eq!(
        admitted.occupant_positions_before,
        admitted.occupant_positions_after
    );
    assert!(!admitted.boundary_request_emitted);
}

#[test]
fn r2_no_combat_resolution() {
    let admitted = report();
    assert_eq!(admitted.combat_resolution_events, 0);
    assert_eq!(admitted.hostile_hp_delta, 0);
    assert!(admitted
        .production_rows
        .iter()
        .any(|row| row.blockader == Some(DressRehearsalR2Owner::Pirate)));
}

#[test]
fn r2_single_tier_no_interior_subtile_materialization() {
    let admitted = report();
    assert!(admitted.single_galactic_tier);
    assert!(!admitted.interior_subtile_materialized);
    assert_eq!(admitted.interior_subtile_count, 0);
    assert_eq!(admitted.galaxy_side, 20);
    assert_eq!(admitted.production_rows.len(), 13);
}

#[test]
fn r2_deterministic_replay_and_cpu_oracle_parity() {
    let (left, right) = replay_dress_rehearsal_r2_recursive_allocation();
    assert_eq!(left, right);
    assert!(left.cpu_oracle_parity);
    assert_eq!(
        left.deterministic_replay_checksum,
        right.deterministic_replay_checksum
    );

    let input = DressRehearsalR2Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r2_recursive_allocation(&input);
    let oracle = cpu_oracle_dress_rehearsal_r2_recursive_allocation(&input);
    assert_eq!(admitted.production_rows, oracle.production_rows);
    assert_eq!(admitted.stockpile_ledger, oracle.stockpile_ledger);
    assert_eq!(admitted.summary, oracle.summary);
}

#[test]
fn r2_opt_in_default_off() {
    let disabled =
        run_dress_rehearsal_r2_recursive_allocation(&DressRehearsalR2Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.production_rows.is_empty());
    assert!(!disabled.default_simsession_pass_graph_change);

    let mut default_on = DressRehearsalR2Input::explicit_opt_in();
    default_on.enabled_by_default = true;
    let rejected = run_dress_rehearsal_r2_recursive_allocation(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"r2_default_on_rejected"));
}
