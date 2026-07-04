// ── SCENARIO-0080-2 proof status (note 2026-06-02, design authority) ─────────────────
// PROVEN at the math/behavioral layer (rung 2: compound desirability field — patrol
// repulsion + disruption penalty, read-only over rung 1, deterministic ordering).
// These remain valid as CPU ORACLES. They are NOT yet proven through a real SimThing
// reduction (no SimThing/SimProperty/Overlay/BoundaryProtocol) — the bar set by
// invariants.md "Scenario Proof". Scheduled for full-vertical re-validation in the
// gamesession dress rehearsal (docs/design_0_0_8_0_consumer_pulled_production_track.md
// §12): desirability becomes a derived column on worldstate gridcell simthings, read-only
// over the `disruption` column. Oracle here, engine there.
use simthing_driver::{
    replay_compound_field_0080_2, run_compound_field_0080_2, CompoundField0082Input,
    CompoundField0082Report, CompoundField0082Weights, BASE_DESIRABILITY, COMPOUND_FIELD_0080_2_ID,
    COMPOUND_FIELD_0080_2_SCENARIO, COMPOUND_FIELD_0080_2_STATUS_PASS, DESIRABILITY_MAX,
};

fn report() -> CompoundField0082Report {
    run_compound_field_0080_2(&CompoundField0082Input::explicit_opt_in())
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
