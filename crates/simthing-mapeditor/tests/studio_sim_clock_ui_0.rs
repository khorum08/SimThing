//! STUDIO-SIM-CLOCK-UI-0 — headless transport UI contract proofs.
//!
//! Exercises the same `StudioSimClockTransport` / `StudioSimClockTransportCommand` path
//! the Studio egui panel uses. No Bevy window, no GPU, no live SimSession bridge.

use simthing_mapeditor::{
    runtime_vertical_seed_scenario_spec, StudioSimClockRate, StudioSimClockTransport,
    StudioSimClockTransportCommand,
};
use simthing_spec::serialize_scenario_authority;

/// catches: the UI Pause action changing only widget state while the underlying
/// StudioSimClock continues scheduling ticks.
#[test]
fn pause_action_freezes_clock() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::Play).expect("play");
    t.apply(StudioSimClockTransportCommand::SetMaxTps(10.0))
        .expect("tps");
    let scheduled = t.clock_mut().advance(1.0);
    assert!(scheduled > 0);
    let frozen = t.readout().tick_index;
    assert_eq!(frozen, scheduled);

    t.apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");
    assert!(t.readout().paused);
    assert!(!t.readout().playing);
    assert_eq!(t.clock_mut().advance(10.0), 0);
    assert_eq!(t.clock_mut().advance(100.0), 0);
    assert_eq!(t.readout().tick_index, frozen);
    // Rate and max TPS preserved across pause.
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate1x);
    assert_eq!(t.readout().max_tps, 10.0);
}

/// catches: the UI Play action failing to transition the underlying clock into the running state.
#[test]
fn play_action_enables_clock() {
    let mut t = StudioSimClockTransport::new();
    assert!(t.readout().paused);
    assert_eq!(t.clock_mut().advance(1.0), 0);

    t.apply(StudioSimClockTransportCommand::Play).expect("play");
    assert!(t.readout().playing);
    assert!(!t.readout().paused);
    // Does not silently change rate or max TPS.
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate1x);
    let n = t.clock_mut().advance(1.0);
    assert!(n > 0, "play must enable scheduling on the underlying clock");
}

/// catches: 1×/2×/4× controls mapping to the wrong StudioSimClockRate or a UI-only rate.
#[test]
fn rate_actions_select_landed_clock_rates() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::Rate2x).expect("2x");
    assert_eq!(t.clock().rate(), StudioSimClockRate::Rate2x);
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate2x);
    assert_eq!(t.readout().rate_label, "2×");

    t.apply(StudioSimClockTransportCommand::Rate4x).expect("4x");
    assert_eq!(t.clock().rate(), StudioSimClockRate::Rate4x);
    assert_eq!(t.readout().rate_label, "4×");

    t.apply(StudioSimClockTransportCommand::Rate1x).expect("1x");
    assert_eq!(t.clock().rate(), StudioSimClockRate::Rate1x);
    assert_eq!(t.readout().rate_label, "1×");
}

/// catches: transport bypassing StudioSimClock validation, accepting invalid TPS, or
/// corrupting the prior valid setting.
#[test]
fn invalid_max_tps_preserves_last_valid_value() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::SetMaxTpsText("20".into()))
        .expect("valid max tps");
    assert_eq!(t.clock().max_tps(), 20.0);
    assert_eq!(t.readout().max_tps, 20.0);

    let err = t
        .apply(StudioSimClockTransportCommand::SetMaxTpsText("0".into()))
        .expect_err("zero max tps must fail via clock path");
    assert!(err.to_string().contains("max_tps"));
    assert!(t.last_error().is_some());
    assert_eq!(t.clock().max_tps(), 20.0);
    assert_eq!(t.readout().max_tps, 20.0);

    let err = t
        .apply(StudioSimClockTransportCommand::SetMaxTpsText(
            "not-a-number".into(),
        ))
        .expect_err("parse fail must reject");
    assert!(err.to_string().contains("max_tps"));
    assert_eq!(t.clock().max_tps(), 20.0);

    let err = t
        .apply(StudioSimClockTransportCommand::SetMaxTps(-5.0))
        .expect_err("negative");
    assert!(err.to_string().contains("max_tps"));
    assert_eq!(t.clock().max_tps(), 20.0);
}

/// catches: paused/rate/max-TPS/effective-rate/tick readout becoming stale or sourced from
/// duplicated presentation state rather than the underlying clock.
#[test]
fn clock_readout_tracks_underlying_state() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::SetMaxTps(10.0))
        .expect("tps");
    t.apply(StudioSimClockTransportCommand::Rate2x)
        .expect("rate");
    t.apply(StudioSimClockTransportCommand::Play).expect("play");
    let scheduled = t.clock_mut().advance(1.0);
    let r = t.readout();
    assert!(!r.paused);
    assert!(r.playing);
    assert_eq!(r.rate, StudioSimClockRate::Rate2x);
    assert_eq!(r.max_tps, 10.0);
    assert_eq!(r.effective_tps, t.clock().effective_tps());
    assert_eq!(r.effective_tps, 20.0);
    assert_eq!(r.tick_index, t.clock().tick_index());
    assert_eq!(r.tick_index, scheduled);

    t.apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");
    let r2 = t.readout();
    assert!(r2.paused);
    assert!(!r2.playing);
    assert_eq!(r2.tick_index, scheduled);
    assert_eq!(r2.rate, StudioSimClockRate::Rate2x);
}

/// catches: presentation controls acquiring an unintended model-authority mutation path.
#[test]
fn transport_actions_do_not_mutate_scenario_spec() {
    let mut spec = runtime_vertical_seed_scenario_spec();
    let before_id = spec.scenario_id.clone();
    let before_json = serialize_scenario_authority(&spec).expect("serialize before");

    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::Play).expect("play");
    t.apply(StudioSimClockTransportCommand::Rate4x)
        .expect("rate");
    t.apply(StudioSimClockTransportCommand::SetMaxTps(20.0))
        .expect("tps");
    let _ = t.clock_mut().advance(5.0);
    t.apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");
    t.apply(StudioSimClockTransportCommand::Play).expect("play");
    let _ = t.clock_mut().advance(1.0);
    let _ = t.apply(StudioSimClockTransportCommand::SetMaxTpsText("0".into()));

    // Transport API has no Spec handle; prove adjacent Spec bytes unchanged.
    assert_eq!(spec.scenario_id, before_id);
    let after_json = serialize_scenario_authority(&spec).expect("serialize after");
    assert_eq!(before_json, after_json);
    let _ = &mut spec; // keep mut for parity with substrate proof style
}
