use simthing_driver::{
    replay_disruption_decay_0080_2, run_disruption_decay_0080_2, DisruptionDecay0082DecayWeights,
    DisruptionDecay0082Input, DisruptionDecay0082Report, DisruptionDecay0082RetentionFactor,
    DISRUPTION_DECAY_0080_2_ID, DISRUPTION_DECAY_0080_2_SCENARIO,
    DISRUPTION_DECAY_0080_2_STATUS_PASS, DISRUPTION_MAX,
};

fn report() -> DisruptionDecay0082Report {
    run_disruption_decay_0080_2(&DisruptionDecay0082Input::explicit_opt_in())
}

#[test]
fn disruption_decay_0080_2_explicit_opt_in_only() {
    let disabled = run_disruption_decay_0080_2(&DisruptionDecay0082Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.rows.is_empty());
    assert_eq!(disabled.node_count, 0);

    let mut default_on = DisruptionDecay0082Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_disruption_decay_0080_2(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"disruption_decay_default_on_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn disruption_decay_0080_2_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.id, DISRUPTION_DECAY_0080_2_ID);
    assert_eq!(admitted.status, DISRUPTION_DECAY_0080_2_STATUS_PASS);
    assert_eq!(admitted.scenario_name, DISRUPTION_DECAY_0080_2_SCENARIO);
}

#[test]
fn disruption_decay_0080_2_composes_bounded_coefficient() {
    let admitted = report();
    // base 9/10 * tech 95/100 = 855/1000 = 171/200.
    assert_eq!(admitted.effective_retain_num, 171);
    assert_eq!(admitted.effective_retain_den, 200);
    assert!(admitted.decay_coefficient_bounded);
    assert!(admitted.effective_retain_num < admitted.effective_retain_den);
}

#[test]
fn disruption_decay_0080_2_accumulates_with_presence() {
    let admitted = report();
    assert!(admitted.accumulates_with_presence);
    // node 0 sees disruption rise across its early pirate-present ticks.
    let node0_t0 = admitted
        .rows
        .iter()
        .find(|r| r.node == 0 && r.tick == 0)
        .unwrap();
    assert!(node0_t0.disruption_after > 0);
    assert!(node0_t0.gained > 0);
}

#[test]
fn disruption_decay_0080_2_decays_to_zero_without_input() {
    let admitted = report();
    assert!(admitted.decays_to_zero_without_input);
    // node 0 after the pirate leaves (tick 6, no patrol yet) strictly decreases via base decay.
    let row = admitted
        .rows
        .iter()
        .find(|r| r.node == 0 && r.tick == 6)
        .unwrap();
    assert_eq!(row.mover_presence_units, 0);
    assert_eq!(row.patrol_presence, 0);
    assert!(row.disruption_after < row.disruption_before);
    assert_eq!(row.disruption_after, row.retained);
}

#[test]
fn disruption_decay_0080_2_patrol_accelerates_decay() {
    let admitted = report();
    assert!(admitted.patrol_accelerates_decay);
    // node 0 once the patrol arrives (tick 12) removes more than base decay alone.
    let row = admitted
        .rows
        .iter()
        .find(|r| r.node == 0 && r.tick == 12)
        .unwrap();
    assert!(row.patrol_presence > 0);
    assert!(row.suppressed > 0);
    assert!(row.disruption_after < row.retained);
}

#[test]
fn disruption_decay_0080_2_clean_systems_stay_zero() {
    let admitted = report();
    // node 1 never sees presence; node 2 only ever sees a patrol (no pirate) — both stay 0.
    assert_eq!(admitted.final_disruption[1], 0);
    assert_eq!(admitted.peak_disruption[1], 0);
    assert_eq!(admitted.final_disruption[2], 0);
    assert_eq!(admitted.peak_disruption[2], 0);
}

#[test]
fn disruption_decay_0080_2_saturates_at_ceiling() {
    let admitted = report();
    // node 3 (continuous pirate) saturates against the bounded ceiling.
    assert!(admitted.saturates_at_ceiling);
    assert!(admitted.peak_disruption[3] <= DISRUPTION_MAX);
    assert_eq!(admitted.peak_disruption[3], DISRUPTION_MAX);
}

#[test]
fn disruption_decay_0080_2_replay_deterministic() {
    let (a, b) = replay_disruption_decay_0080_2();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

#[test]
fn disruption_decay_0080_2_emits_export() {
    let admitted = report();
    assert!(!admitted.text_export.is_empty());
    assert!(admitted.text_export.contains("DISRUPTION-DECAY-0080-2"));
    assert!(admitted.text_export.contains("TICK|"));
    assert!(admitted.text_export.contains("FINAL|"));
}

#[test]
fn disruption_decay_0080_2_rejects_unbounded_decay() {
    let mut input = DisruptionDecay0082Input::explicit_opt_in();
    // retention >= 1 is an unbounded recurrence.
    input.decay_weights = DisruptionDecay0082DecayWeights {
        base_retain_num: 10,
        base_retain_den: 10,
        modifiers: Vec::new(),
    };
    let rejected = run_disruption_decay_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"unbounded_decay_coefficient"));
}

#[test]
fn disruption_decay_0080_2_rejects_retention_increasing_modifier() {
    let mut input = DisruptionDecay0082Input::explicit_opt_in();
    input.decay_weights.modifiers = vec![DisruptionDecay0082RetentionFactor {
        label: "bad_persistence_tech",
        num: 110,
        den: 100,
    }];
    let rejected = run_disruption_decay_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"decay_modifier_increases_retention"));
}

#[test]
fn disruption_decay_0080_2_rejects_global_decay_overlay_write() {
    let mut input = DisruptionDecay0082Input::explicit_opt_in();
    input.surface.global_decay_overlay_write = true;
    let rejected = run_disruption_decay_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"global_decay_overlay_write_rejected"));
    assert!(!rejected.no_global_decay_overlay_write);
}

#[test]
fn disruption_decay_0080_2_rejects_gradient_follow_movement_in_this_rung() {
    let mut input = DisruptionDecay0082Input::explicit_opt_in();
    input.forbidden.gradient_follow_movement = true;
    let rejected = run_disruption_decay_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"gradient_follow_movement_not_in_this_rung"));
}

#[test]
fn disruption_decay_0080_2_rejects_closed_ladder_reopen_and_kernel_and_planner() {
    for mutate in [
        |i: &mut DisruptionDecay0082Input| i.forbidden.reopen_closed_0080_1_ladder = true,
        |i: &mut DisruptionDecay0082Input| i.forbidden.new_shader_or_gpu_kernel = true,
        |i: &mut DisruptionDecay0082Input| i.forbidden.cpu_planner_urgency_commitment = true,
        |i: &mut DisruptionDecay0082Input| i.surface.global_default_schedule = true,
        |i: &mut DisruptionDecay0082Input| i.forbidden.semantic_or_raw_wgsl = true,
    ] {
        let mut input = DisruptionDecay0082Input::explicit_opt_in();
        mutate(&mut input);
        let rejected = run_disruption_decay_0080_2(&input);
        assert!(!rejected.admitted);
        assert!(!rejected.diagnostics.is_empty());
    }
}

#[test]
fn disruption_decay_0080_2_rejects_presence_schedule_shape_mismatch() {
    let mut input = DisruptionDecay0082Input::explicit_opt_in();
    input.presence_schedule.pop();
    let rejected = run_disruption_decay_0080_2(&input);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"presence_schedule_tick_shape_mismatch"));
}

#[test]
fn disruption_decay_0080_2_runs_twenty_plus_ticks() {
    let admitted = report();
    assert!(admitted.tick_count >= 20);
    assert_eq!(admitted.rows.len(), admitted.tick_count as usize * admitted.node_count);
}
