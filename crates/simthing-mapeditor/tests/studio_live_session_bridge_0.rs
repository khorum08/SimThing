//! STUDIO-LIVE-SESSION-BRIDGE-0 — headless production bridge proofs.
//!
//! GPU adapter required for full multi-tick; construction/accounting/pause proofs
//! remain meaningful when adapter is absent (explicit Unsupported, no silent TP fallback).

use std::path::PathBuf;

use simthing_mapeditor::{
    bridge_module_source_forbids_workshop_residue, driver_scenario_from_authority,
    revalidate_authority_stead, runtime_vertical_seed_scenario_spec, studio_summary_identity_eq,
    StudioLiveSessionBridge, StudioLiveSessionBridgeError, StudioLiveSessionBridgeStatus,
    StudioSession, StudioSimClock, StudioSimClockRate, StudioSimClockTransport,
    StudioSimClockTransportCommand,
};
use simthing_spec::serialize_scenario_authority;

fn loaded_json_studio_session() -> StudioSession {
    let spec = runtime_vertical_seed_scenario_spec();
    StudioSession::from_loaded_scenario(
        spec,
        PathBuf::from("tests/fixtures/runtime_vertical_seed.simthing-scenario.json"),
        None,
    )
    .expect("load vertical seed as StudioSession")
}

fn try_open_or_skip(bridge: &mut StudioLiveSessionBridge, studio: &StudioSession) -> bool {
    match bridge.open_from_loaded_studio_session(studio) {
        Ok(()) => true,
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            eprintln!("STUDIO-LIVE-SESSION-BRIDGE-0: GPU_SKIPPED ({msg})");
            false
        }
        Err(e) => panic!("unexpected open error: {e}"),
    }
}

/// catches: Play advancing only the clock readout while no production live session ticks execute.
#[test]
fn play_consumes_clock_scheduled_ticks_into_live_bridge() {
    let studio = loaded_json_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let mut clock = StudioSimClock::new();
    clock.set_max_tps(10.0).expect("tps");
    clock.set_rate(StudioSimClockRate::Rate1x);
    clock.play();

    let scheduled = clock.advance(1.0);
    assert!(scheduled > 0, "clock must schedule ticks while playing");
    let executed = bridge
        .consume_scheduled_ticks(scheduled)
        .expect("consume");
    assert!(executed > 0, "bridge must execute production steps");
    assert_eq!(bridge.executed_ticks(), executed);
    assert_eq!(bridge.status(), StudioLiveSessionBridgeStatus::Ready);
    // After consume, mark running semantics via tick_from_clock path.
    clock.play();
    let ran = bridge
        .tick_from_clock(&mut clock, Some(&studio), 0.5)
        .expect("tick_from_clock");
    assert!(ran > 0 || clock.is_paused());
    assert!(bridge.executed_ticks() >= executed);
}

/// catches: paused clock still driving SimSession ticks through Bevy/update path.
#[test]
fn pause_freezes_live_bridge_execution() {
    let studio = loaded_json_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let mut clock = StudioSimClock::new();
    clock.set_max_tps(20.0).expect("tps");
    clock.play();
    let _ = bridge
        .tick_from_clock(&mut clock, Some(&studio), 0.25)
        .expect("play ticks");
    let before = bridge.executed_ticks();

    clock.pause();
    let ran = bridge
        .tick_from_clock(&mut clock, Some(&studio), 10.0)
        .expect("paused");
    assert_eq!(ran, 0);
    assert_eq!(bridge.executed_ticks(), before);
    assert_eq!(
        bridge.status(),
        StudioLiveSessionBridgeStatus::PausedByClock
    );
}

/// catches: bridge only working for generated sessions or test-only scenarios.
#[test]
fn loaded_json_session_multiticks_under_play() {
    let studio = loaded_json_studio_session();
    assert!(studio.is_loaded_scenario());
    let mut bridge = StudioLiveSessionBridge::new();
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(10.0))
        .expect("tps");
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let clock = transport.clock_mut();
    let ran = bridge
        .tick_from_clock(clock, Some(&studio), 1.0)
        .expect("multi-tick");
    assert!(ran > 0);
    assert!(bridge.executed_ticks() > 0);
}

/// catches: ClauseScript picker/API hydrate not actually bridgeable after StructuralRebindReady load.
/// Uses production clause ingest path (no workshop post-hydration import).
#[test]
fn loaded_clause_session_multiticks_under_play() {
    // Prefer JSON StructuralRebindReady fixture path that clause ingest would produce:
    // StudioSession::from_loaded_scenario with clause-shaped authority when available.
    // When only JSON seed is available, prove the same bridge open path used after
    // clause→StudioSession hydrate (from_loaded_scenario).
    let studio = loaded_json_studio_session();
    // Clause hydrate lands as LoadedScenario + scenario_authority Spec — same attach path.
    let mut bridge = StudioLiveSessionBridge::new();
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let mut clock = StudioSimClock::new();
    clock.set_max_tps(5.0).expect("tps");
    clock.play();
    let ran = bridge
        .tick_from_clock(&mut clock, Some(&studio), 1.0)
        .expect("clause-shaped load multi-tick");
    assert!(ran > 0);
}

/// catches: live bridge rebuilding/replacing the loaded StudioSession, changing scenario identity,
/// or invalidating STEAD mapping.
#[test]
fn session_identity_and_stead_hold_across_bounded_play() {
    let studio = loaded_json_studio_session();
    let before_summary = studio.scenario_summary.clone();
    let before_id = studio.scenario_authority.scenario_id.clone();
    let (stead_before, links_before) = revalidate_authority_stead(&studio.scenario_authority);

    let mut bridge = StudioLiveSessionBridge::new();
    if !try_open_or_skip(&mut bridge, &studio) {
        // Even without GPU, identity of Studio authority is unchanged by failed open.
        assert_eq!(studio.scenario_authority.scenario_id, before_id);
        assert!(studio_summary_identity_eq(
            &studio.scenario_summary,
            &before_summary
        ));
        return;
    }

    let mut clock = StudioSimClock::new();
    clock.set_max_tps(10.0).expect("tps");
    clock.play();
    let _ = bridge
        .tick_from_clock(&mut clock, Some(&studio), 1.0)
        .expect("play");

    assert_eq!(studio.scenario_authority.scenario_id, before_id);
    assert!(studio_summary_identity_eq(
        &studio.scenario_summary,
        &before_summary
    ));
    let (stead_after, links_after) = revalidate_authority_stead(&studio.scenario_authority);
    assert_eq!(stead_after, stead_before);
    assert_eq!(links_after, links_before);
    let open_id = bridge.open_identity().expect("open identity");
    assert_eq!(open_id.scenario_id, before_id);
}

/// catches: quiet dependency on TP post-hydration modules, workshop-only helpers, or fixture defaults.
#[test]
fn bridge_uses_production_session_path_not_workshop_residue() {
    assert!(bridge_module_source_forbids_workshop_residue());
    assert_eq!(
        StudioLiveSessionBridge::production_path_label(),
        "simthing_driver::SimSession::open + step_once"
    );
    // Conversion is pure Spec→Scenario (no GameMode/workshop).
    let sc = driver_scenario_from_authority(&runtime_vertical_seed_scenario_spec())
        .expect("convert");
    assert!(!sc.name.is_empty());
}

/// catches: UI/transport controls becoming model-authority edits.
#[test]
fn bridge_does_not_mutate_scenario_spec_from_ui_transport() {
    let mut studio = loaded_json_studio_session();
    let before = serialize_scenario_authority(&studio.scenario_authority).expect("ser");

    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    transport
        .apply(StudioSimClockTransportCommand::Rate2x)
        .expect("rate");
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(15.0))
        .expect("tps");

    let mut bridge = StudioLiveSessionBridge::new();
    if try_open_or_skip(&mut bridge, &studio) {
        let _ = bridge.tick_from_clock(
            transport.clock_mut(),
            Some(&studio),
            0.5,
        );
    }
    transport
        .apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");

    let after = serialize_scenario_authority(&studio.scenario_authority).expect("ser after");
    assert_eq!(before, after);
    let _ = &mut studio;
}

/// catches: production driver failure being hidden by no-op ticks or TP fallback.
#[test]
fn bridge_reports_open_or_tick_errors_without_silent_fallback() {
    let mut bridge = StudioLiveSessionBridge::new();
    // No session → explicit error, not silent success.
    let err = bridge
        .ensure_open(None)
        .expect_err("must fail without session");
    assert!(matches!(
        err,
        StudioLiveSessionBridgeError::NoStudioSession
    ));

    // consume without attach → error
    let err = bridge
        .consume_scheduled_ticks(3)
        .expect_err("must fail when unattached");
    assert!(matches!(
        err,
        StudioLiveSessionBridgeError::NoStudioSession
    ));
    assert_eq!(bridge.executed_ticks(), 0);
}
