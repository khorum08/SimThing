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
