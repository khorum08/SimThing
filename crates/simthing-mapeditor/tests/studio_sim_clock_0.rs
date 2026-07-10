//! STUDIO-SIM-CLOCK-0 — headless Studio sim clock substrate proofs.
//!
//! No Bevy window, no GPU adapter, no SimSession bridge, no library UI.

use simthing_mapeditor::{
    runtime_vertical_seed_scenario_spec, StudioSimClock, StudioSimClockRate,
    STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE,
};
use simthing_spec::serialize_scenario_authority;

#[test]
fn pause_freezes_tick_index() {
    let mut clock = StudioSimClock::new();
    clock.set_max_tps(10.0).expect("max_tps");
    clock.set_rate(StudioSimClockRate::Rate1x);
    clock.play();
    let scheduled = clock.advance(1.0);
    assert_eq!(scheduled, 10);
    assert_eq!(clock.tick_index(), 10);

    clock.pause();
    let before = clock.tick_index();
    assert_eq!(clock.advance(10.0), 0);
    assert_eq!(clock.advance(100.0), 0);
    assert_eq!(clock.tick_index(), before);
    assert!(clock.is_paused());

    clock.play();
    let resumed = clock.advance(0.5);
    assert_eq!(resumed, 5);
    assert_eq!(clock.tick_index(), before + 5);
}

#[test]
fn rate_2x_4x_ratios() {
    // Same wall elapsed + same max_tps: 2× ≈ 2× ticks, 4× ≈ 4× ticks (tolerance).
    let elapsed = 2.0;
    let max_tps = 10.0;

    let mut c1 = StudioSimClock::new();
    c1.set_max_tps(max_tps).expect("max_tps");
    c1.set_rate(StudioSimClockRate::Rate1x);
    c1.play();
    let t1 = c1.advance(elapsed) as f64;

    let mut c2 = StudioSimClock::new();
    c2.set_max_tps(max_tps).expect("max_tps");
    c2.set_rate(StudioSimClockRate::Rate2x);
    c2.play();
    let t2 = c2.advance(elapsed) as f64;

    let mut c4 = StudioSimClock::new();
    c4.set_max_tps(max_tps).expect("max_tps");
    c4.set_rate(StudioSimClockRate::Rate4x);
    c4.play();
    let t4 = c4.advance(elapsed) as f64;

    assert!(t1 > 0.0, "1× must schedule ticks");
    let ratio_2 = t2 / t1;
    let ratio_4 = t4 / t1;
    assert!(
        (ratio_2 - 2.0).abs() <= STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE,
        "2× ratio {ratio_2} outside tolerance vs 2.0 (t1={t1}, t2={t2})"
    );
    assert!(
        (ratio_4 - 4.0).abs() <= STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE,
        "4× ratio {ratio_4} outside tolerance vs 4.0 (t1={t1}, t4={t4})"
    );
}

#[test]
fn max_tps_cap_holds() {
    // For a given rate, scheduled ticks for dt equal floor(rate * max_tps * dt)
    // (plus at most prior fractional accumulator). Never an unbounded storm.
    let mut clock = StudioSimClock::new();
    clock.set_max_tps(5.0).expect("max_tps");
    clock.set_rate(StudioSimClockRate::Rate2x); // effective 10 TPS
    clock.play();

    let dt = 3.0;
    let scheduled = clock.advance(dt);
    let expected = (2.0_f64 * 5.0 * dt).floor() as u64;
    assert_eq!(scheduled, expected);
    assert_eq!(clock.tick_index(), expected);

    // Huge elapsed still scales with max_tps * rate * dt — not an open loop.
    let mut storm = StudioSimClock::new();
    storm.set_max_tps(1.0).expect("max_tps");
    storm.set_rate(StudioSimClockRate::Rate1x);
    storm.play();
    let big = storm.advance(1_000.0);
    assert_eq!(big, 1_000);
    assert_eq!(storm.tick_index(), 1_000);

    // Reject invalid caps.
    let mut bad = StudioSimClock::new();
    assert!(bad.set_max_tps(0.0).is_err());
    assert!(bad.set_max_tps(-1.0).is_err());
    assert!(bad.set_max_tps(f64::NAN).is_err());
}

#[test]
fn clock_does_not_mutate_scenario_spec() {
    let mut spec = runtime_vertical_seed_scenario_spec();
    let before_id = spec.scenario_id.clone();
    let before_json = serialize_scenario_authority(&spec).expect("serialize before");

    let mut clock = StudioSimClock::new();
    clock.set_max_tps(20.0).expect("max_tps");
    clock.set_rate(StudioSimClockRate::Rate4x);
    clock.play();
    let _ = clock.advance(5.0);
    clock.pause();
    let _ = clock.advance(5.0);
    clock.play();
    let _ = clock.advance(1.0);

    // Clock API has no Spec handle; prove adjacent Spec bytes unchanged after scheduling.
    assert_eq!(spec.scenario_id, before_id);
    let after_json = serialize_scenario_authority(&spec).expect("serialize after");
    assert_eq!(before_json, after_json);
    // Touch Spec only via local binding — clock never received it.
    spec.scenario_id = before_id;
}
