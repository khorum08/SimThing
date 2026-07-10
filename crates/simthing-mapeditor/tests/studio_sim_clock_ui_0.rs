//! STUDIO-SIM-CLOCK-UI-0 — headless transport UI contract proofs.
//!
//! No Bevy window, no GPU, no live SimSession bridge. Proves the programmatic
//! transport façade that the Studio egui panel projects over.

use simthing_mapeditor::{
    StudioSimClockRate, StudioSimClockTransport, StudioSimClockTransportCommand,
};

#[test]
fn transport_pause_play_toggles_clock_state() {
    let mut t = StudioSimClockTransport::new();
    assert!(t.readout().paused);

    t.apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    assert!(t.readout().playing);
    assert!(!t.readout().paused);

    // While playing, schedule ticks under default max TPS.
    let n = t.clock_mut().advance(1.0);
    assert!(n > 0);
    assert_eq!(t.readout().tick_index, n);

    t.apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");
    assert!(t.readout().paused);
    let frozen = t.readout().tick_index;
    assert_eq!(t.clock_mut().advance(10.0), 0);
    assert_eq!(t.readout().tick_index, frozen);
}

#[test]
fn transport_rate_buttons_select_1x_2x_4x() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::Rate2x)
        .expect("2x");
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate2x);
    assert_eq!(t.readout().rate_label, "2×");

    t.apply(StudioSimClockTransportCommand::Rate4x)
        .expect("4x");
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate4x);
    assert_eq!(t.readout().rate_label, "4×");

    t.apply(StudioSimClockTransportCommand::Rate1x)
        .expect("1x");
    assert_eq!(t.readout().rate, StudioSimClockRate::Rate1x);
    assert_eq!(t.readout().rate_label, "1×");
}

#[test]
fn transport_max_tps_uses_clock_validation() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::SetMaxTpsText("20".into()))
        .expect("valid max tps");
    assert_eq!(t.readout().max_tps, 20.0);
    assert!(t.last_error().is_none());

    // Invalid input is rejected by the clock path (not UI-only sanitization).
    let err = t
        .apply(StudioSimClockTransportCommand::SetMaxTpsText("0".into()))
        .expect_err("zero max tps must fail");
    assert!(err.to_string().contains("max_tps"));
    assert!(t.last_error().is_some());
    // Clock retains last valid max_tps.
    assert_eq!(t.readout().max_tps, 20.0);

    let err = t
        .apply(StudioSimClockTransportCommand::SetMaxTpsText("not-a-number".into()))
        .expect_err("parse fail");
    assert!(err.to_string().contains("max_tps"));
    assert_eq!(t.readout().max_tps, 20.0);
}

#[test]
fn transport_readout_exposes_effective_tps_and_tick() {
    let mut t = StudioSimClockTransport::new();
    t.apply(StudioSimClockTransportCommand::SetMaxTps(10.0))
        .expect("tps");
    t.apply(StudioSimClockTransportCommand::Rate2x)
        .expect("rate");
    t.apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let scheduled = t.clock_mut().advance(1.0);
    let r = t.readout();
    assert!(!r.paused);
    assert_eq!(r.effective_tps, 20.0);
    assert_eq!(r.tick_index, scheduled);
    assert_eq!(scheduled, 20);
}
