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
fn gradient_follow_0080_2_replay_deterministic() {
    let (a, b) = replay_gradient_follow_0080_2();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}
