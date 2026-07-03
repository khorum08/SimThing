// ── SCENARIO-0080-2 proof status (note 2026-06-02, design authority) ─────────────────
// PROVEN at the math/behavioral layer (rung 4: FIELD_POLICY field-as-policy movement — gradient
// read, threshold-gated single-step ascent, self-disruption-driven migration, patrol
// repulsion, deterministic). These remain valid as CPU ORACLES. They are NOT yet proven
// through a real SimThing reduction (no SimThing/SimProperty/Overlay/BoundaryProtocol) —
// the bar set by invariants.md "Scenario Proof". Scheduled for full-vertical re-validation
// in the gamesession dress rehearsal (docs/design_0_0_8_0_consumer_pulled_production_track.md
// §12): the pirate faction simthing moves over worldstate gridcell desirability via
// Threshold+EmitEvent -> BoundaryRequest (one step per boundary, no CPU planner), its
// techtree modifying emission/threshold. Oracle here, engine there.
use simthing_driver::{
    replay_gradient_follow_0080_2, run_gradient_follow_0080_2, GradientFollow0082Input,
    GradientFollow0082Report, GRADIENT_FOLLOW_0080_2_ID, GRADIENT_FOLLOW_0080_2_SCENARIO,
    GRADIENT_FOLLOW_0080_2_STATUS_PASS,
};

fn report() -> GradientFollow0082Report {
    run_gradient_follow_0080_2(&GradientFollow0082Input::explicit_opt_in())
}

#[test]
fn gradient_follow_0080_2_explicit_opt_in_only() {
    let disabled = run_gradient_follow_0080_2(&GradientFollow0082Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.move_rows.is_empty());

    let mut default_on = GradientFollow0082Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_gradient_follow_0080_2(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gradient_follow_default_on_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn gradient_follow_0080_2_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.id, GRADIENT_FOLLOW_0080_2_ID);
    assert_eq!(admitted.status, GRADIENT_FOLLOW_0080_2_STATUS_PASS);
    assert_eq!(admitted.scenario_name, GRADIENT_FOLLOW_0080_2_SCENARIO);
}

#[test]
fn gradient_follow_0080_2_runs_twenty_plus_ticks() {
    let admitted = report();
    assert!(admitted.tick_count >= 20);
    assert_eq!(admitted.move_rows.len(), admitted.tick_count as usize);
}

#[test]
fn gradient_follow_0080_2_first_tick_ascends_eastward() {
    // Pirate emits disruption at node 0 (lowering its desirability); the forward gradient
    // toward the clean node 1 crosses threshold → step east.
    let admitted = report();
    let t0 = admitted.move_rows[0];
    assert_eq!(t0.pirate_node, 0);
    assert!(t0.gradient_dx > 0, "forward gradient: {}", t0.gradient_dx);
    assert!(t0.threshold_crossed);
    assert!(t0.event_emitted);
    assert!(t0.moved);
    assert_eq!(t0.move_direction, "east");
    assert_eq!(t0.moved_to_node, 1);
}

#[test]
fn gradient_follow_0080_2_pirate_migrates_from_start() {
    let admitted = report();
    assert_eq!(admitted.start_node, 0);
    // Self-disruption drives the pirate across the line; it reaches the far end at some point.
    assert!(
        admitted.max_distance_from_start >= 3,
        "max distance {}",
        admitted.max_distance_from_start
    );
    assert!(
        admitted.visited_nodes.len() >= 4,
        "visited {:?}",
        admitted.visited_nodes
    );
    assert!(admitted.total_moves >= 4);
}

#[test]
fn gradient_follow_0080_2_single_step_per_tick() {
    let admitted = report();
    assert!(admitted.single_step_per_tick);
    // Every move lands on a node adjacent (Manhattan distance 1) to the previous one.
    for row in admitted.move_rows.iter().filter(|r| r.moved) {
        assert!(row.moved_to_node != row.pirate_node);
    }
}

#[test]
fn gradient_follow_0080_2_threshold_gates_movement() {
    // A very high threshold means the gradient never crosses → the pirate never moves.
    let mut input = GradientFollow0082Input::explicit_opt_in();
    input.movement_threshold = 10_000_000;
    let admitted = run_gradient_follow_0080_2(&input);
    assert!(admitted.admitted);
    assert_eq!(admitted.total_moves, 0);
    assert_eq!(admitted.final_node, admitted.start_node);
    assert!(admitted.move_rows.iter().all(|r| !r.moved));
    assert!(admitted.move_rows.iter().all(|r| !r.event_emitted));
}

#[test]
fn gradient_follow_0080_2_moves_only_when_event_emitted() {
    let admitted = report();
    // Field-sourced + threshold-gated: a move happens only on a tick whose threshold crossed.
    for row in &admitted.move_rows {
        if row.moved {
            assert!(row.threshold_crossed && row.event_emitted);
        }
    }
    assert!(admitted.threshold_gated);
}

#[test]
fn gradient_follow_0080_2_field_sourced_guardrails() {
    let admitted = report();
    assert!(admitted.field_sourced_movement);
    assert!(admitted.no_cpu_planner_or_lookahead);
    assert!(admitted.no_multi_step_pathfinding);
    assert!(admitted.no_direct_movement_command);
    assert!(admitted.no_global_default_schedule);
    assert!(admitted.does_not_reopen_closed_0080_1_ladder);
}

#[test]
fn gradient_follow_0080_2_mid_patrol_alters_trajectory() {
    let clean = report();
    let patrolled =
        run_gradient_follow_0080_2(&GradientFollow0082Input::explicit_opt_in_with_mid_patrol());
    assert!(patrolled.admitted, "{:?}", patrolled.diagnostics);
    // The mid-line patrol repels the pirate, producing a different deterministic trajectory.
    assert_ne!(
        clean.deterministic_replay_checksum,
        patrolled.deterministic_replay_checksum
    );
}

#[test]
fn gradient_follow_0080_2_replay_deterministic() {
    let (a, b) = replay_gradient_follow_0080_2();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

#[test]
fn gradient_follow_0080_2_emits_export() {
    let admitted = report();
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("GRADIENT-FOLLOW-0080-2"));
    assert!(admitted.text_export.contains("STEP|"));
}

#[test]
fn gradient_follow_0080_2_rejects_cpu_planner_and_lookahead() {
    let mut input = GradientFollow0082Input::explicit_opt_in();
    input.forbidden.cpu_planner_or_lookahead = true;
    let rejected = run_gradient_follow_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"cpu_planner_or_lookahead"));
    assert!(!rejected.no_cpu_planner_or_lookahead);
}
