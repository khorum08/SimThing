// ── SCENARIO-0080-2 proof status (note 2026-06-02, design authority) ─────────────────
// PROVEN at the math/behavioral layer (rung 1: disruption BoundedFeedback decay —
// accumulate / natural decay / patrol-accelerated decay / saturation, deterministic).
// These remain valid as CPU ORACLES. They are NOT yet proven through a real SimThing
// reduction (no SimThing/SimProperty/Overlay/BoundaryProtocol) — the bar set by
// invariants.md "Scenario Proof". Scheduled for full-vertical re-validation in the
// gamesession dress rehearsal (docs/design_0_0_8_0_consumer_pulled_production_track.md
// §12): `disruption` becomes a SimProperty column on worldstate gridcell simthings,
// advanced by AccumulatorOp with the decay weight carried on the gamesession root overlay
// and composed with faction-techtree capability modifiers. Oracle here, engine there.
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
fn disruption_decay_0080_2_replay_deterministic() {
    let (a, b) = replay_disruption_decay_0080_2();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}
