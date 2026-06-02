use simthing_driver::{
    replay_compound_field_0080_2, run_compound_field_0080_2, CompoundField0082Input,
    CompoundField0082Report, CompoundField0082Weights, BASE_DESIRABILITY, COMPOUND_FIELD_0080_2_ID,
    COMPOUND_FIELD_0080_2_SCENARIO, COMPOUND_FIELD_0080_2_STATUS_PASS, DESIRABILITY_MAX,
};

fn report() -> CompoundField0082Report {
    run_compound_field_0080_2(&CompoundField0082Input::explicit_opt_in())
}

#[test]
fn compound_field_0080_2_explicit_opt_in_only() {
    let disabled = run_compound_field_0080_2(&CompoundField0082Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.snapshots.is_empty());
    assert_eq!(disabled.node_count, 0);

    let mut default_on = CompoundField0082Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_compound_field_0080_2(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"compound_field_default_on_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn compound_field_0080_2_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.id, COMPOUND_FIELD_0080_2_ID);
    assert_eq!(admitted.status, COMPOUND_FIELD_0080_2_STATUS_PASS);
    assert_eq!(admitted.scenario_name, COMPOUND_FIELD_0080_2_SCENARIO);
}

#[test]
fn compound_field_0080_2_produces_snapshots_for_all_ticks_and_nodes() {
    let admitted = report();
    assert!(admitted.tick_count >= 20);
    assert_eq!(admitted.node_count, 4);
    assert_eq!(
        admitted.snapshots.len(),
        admitted.tick_count as usize * admitted.node_count
    );
}

#[test]
fn compound_field_0080_2_patrol_repels() {
    let admitted = report();
    assert!(admitted.patrol_repels);
    // node 2 has patrol throughout — desirability must be below base.
    let node2_snap = admitted
        .snapshots
        .iter()
        .find(|s| s.node == 2 && s.tick == 0)
        .unwrap();
    assert!(node2_snap.patrol_field > 0);
    assert!(node2_snap.desirability < BASE_DESIRABILITY);
}

#[test]
fn compound_field_0080_2_patrolled_node_floors_to_zero() {
    let admitted = report();
    // node 2 (canonical patrol_repulsion=15_000, 1 patrol unit, base=50_000):
    // desirability = 50_000 - 15_000 - 0 = 35_000 > 0.
    // Confirm it's reduced but above zero (no disruption present on this node).
    let final_node2 = admitted.final_desirability[2];
    assert!(final_node2 < BASE_DESIRABILITY);
    assert!(final_node2 >= 0);
}

#[test]
fn compound_field_0080_2_disrupted_still_passable() {
    let admitted = report();
    assert!(admitted.disrupted_still_passable);
    // node 3 (pirate-saturated) at final tick: disruption = DISRUPTION_MAX, no patrol.
    // desirability = 50_000 - 0 - 300*(100_000/1_000) = 50_000 - 30_000 = 20_000 > 0.
    assert!(admitted.final_desirability[3] > 0);
}

#[test]
fn compound_field_0080_2_clean_node_reaches_base() {
    let admitted = report();
    assert!(admitted.clean_node_reaches_base);
    // node 1 never sees presence — should reach BASE_DESIRABILITY throughout.
    let node1_any = admitted
        .snapshots
        .iter()
        .filter(|s| s.node == 1)
        .all(|s| s.desirability == BASE_DESIRABILITY);
    assert!(node1_any);
}

#[test]
fn compound_field_0080_2_final_field_ordering_correct() {
    let admitted = report();
    assert!(admitted.final_field_ordering_correct);
    // node 1 (clean) >= node 0 (partially decayed) > node 3 (disrupted) AND node 2 (patrolled) < node 1
    let d = &admitted.final_desirability;
    assert!(d[1] >= d[0], "node1 >= node0: {} >= {}", d[1], d[0]);
    assert!(d[0] > d[3], "node0 > node3: {} > {}", d[0], d[3]);
    assert!(d[2] < d[1], "node2 < node1: {} < {}", d[2], d[1]);
}

#[test]
fn compound_field_0080_2_desirability_never_exceeds_max() {
    let admitted = report();
    assert!(admitted.snapshots.iter().all(|s| s.desirability <= DESIRABILITY_MAX));
    assert!(admitted.snapshots.iter().all(|s| s.desirability >= 0));
}

#[test]
fn compound_field_0080_2_node_positions_carried_through() {
    let admitted = report();
    assert_eq!(admitted.node_positions.len(), 4);
    assert_eq!(admitted.node_positions[0].x, 0);
    assert_eq!(admitted.node_positions[1].x, 1);
    assert_eq!(admitted.node_positions[2].x, 2);
    assert_eq!(admitted.node_positions[3].x, 3);
    assert!(admitted.node_positions.iter().all(|p| p.y == 0));
}

#[test]
fn compound_field_0080_2_emits_export() {
    let admitted = report();
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("COMPOUND-FIELD-0080-2"));
    assert!(admitted.text_export.contains("FIELD|"));
    assert!(admitted.text_export.contains("NODE_POS|"));
    assert!(admitted.text_export.contains("FINAL|"));
}

#[test]
fn compound_field_0080_2_replay_deterministic() {
    let (a, b) = replay_compound_field_0080_2();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

#[test]
fn compound_field_0080_2_rejects_gradient_follow_movement_in_this_rung() {
    let mut input = CompoundField0082Input::explicit_opt_in();
    input.forbidden.gradient_follow_movement = true;
    let rejected = run_compound_field_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gradient_follow_movement_not_in_this_rung"));
    assert!(!rejected.no_gradient_follow_movement);
}

#[test]
fn compound_field_0080_2_rejects_write_to_disruption_column() {
    let mut input = CompoundField0082Input::explicit_opt_in();
    input.forbidden.write_to_disruption_column = true;
    let rejected = run_compound_field_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"write_to_disruption_column_rejected"));
}

#[test]
fn compound_field_0080_2_rejects_kernel_planner_global_schedule_wgsl() {
    for mutate in [
        |i: &mut CompoundField0082Input| i.forbidden.new_shader_or_gpu_kernel = true,
        |i: &mut CompoundField0082Input| i.forbidden.cpu_planner_urgency_commitment = true,
        |i: &mut CompoundField0082Input| i.surface.global_default_schedule = true,
        |i: &mut CompoundField0082Input| i.forbidden.semantic_or_raw_wgsl = true,
        |i: &mut CompoundField0082Input| i.forbidden.reopen_closed_0080_1_ladder = true,
    ] {
        let mut input = CompoundField0082Input::explicit_opt_in();
        mutate(&mut input);
        let rejected = run_compound_field_0080_2(&input);
        assert!(!rejected.admitted, "expected rejection for forbidden flag");
        assert!(!rejected.diagnostics.is_empty());
    }
}

#[test]
fn compound_field_0080_2_rejects_node_positions_shape_mismatch() {
    let mut input = CompoundField0082Input::explicit_opt_in();
    input.node_positions.pop(); // 3 positions for 4 nodes
    let rejected = run_compound_field_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"node_positions_count_mismatch"));
}

#[test]
fn compound_field_0080_2_disruption_field_reflects_rung1_state() {
    let admitted = report();
    // node 3 ends with DISRUPTION_MAX (saturated, proven in rung 1).
    // This rung's final_disruption[3] must agree.
    use simthing_driver::DISRUPTION_MAX;
    assert_eq!(admitted.final_disruption[3], DISRUPTION_MAX);
    // node 1 ends with 0 disruption (no presence, never gained).
    assert_eq!(admitted.final_disruption[1], 0);
}

#[test]
fn compound_field_0080_2_desirability_formula_spot_check() {
    let admitted = report();
    // node 1, any tick: disruption=0, patrol=0 → desirability = BASE_DESIRABILITY exactly.
    let snap = admitted
        .snapshots
        .iter()
        .find(|s| s.node == 1 && s.tick == 0)
        .unwrap();
    assert_eq!(snap.disruption, 0);
    assert_eq!(snap.patrol_field, 0);
    assert_eq!(snap.desirability, BASE_DESIRABILITY);

    // node 2, tick 0: patrol=1, disruption=0 → 50_000 - 15_000 = 35_000.
    let snap2 = admitted
        .snapshots
        .iter()
        .find(|s| s.node == 2 && s.tick == 0)
        .unwrap();
    assert_eq!(snap2.patrol_field, 1);
    assert_eq!(snap2.disruption, 0);
    assert_eq!(snap2.desirability, BASE_DESIRABILITY - 15_000);
}
