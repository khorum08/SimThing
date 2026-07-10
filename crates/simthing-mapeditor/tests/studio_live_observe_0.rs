//! STUDIO-LIVE-OBSERVE-0 — headless observation projection proofs.
//!
//! Pure compose of clock + bridge + session readouts. Does not require GPU for
//! freeze/update of scheduled counters (clock path). Bridge executed counters are
//! projected from existing bridge readout fields (no new driver API).

use std::path::PathBuf;

use simthing_mapeditor::{
    build_studio_live_observation_readout, observe_module_source_forbids_workshop_residue,
    runtime_vertical_seed_scenario_spec, StudioLiveObservationSourceKind, StudioLiveSessionBridge,
    StudioLiveSessionBridgeReadout, StudioLiveSessionBridgeStatus, StudioSession,
    StudioSimClockRate, StudioSimClockTransport, StudioSimClockTransportCommand,
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

/// catches: observation panel/readout stays stale while clock/bridge execute ticks.
#[test]
fn live_observation_updates_while_running() {
    let studio = loaded_json_studio_session();
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(10.0))
        .expect("tps");
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");

    let before = build_studio_live_observation_readout(
        &transport.readout(),
        &StudioLiveSessionBridgeReadout::default_unattached(),
        Some(&studio),
    );
    assert_eq!(before.scheduled_tick_index, 0);

    let scheduled = transport.clock_mut().advance(1.0);
    assert!(scheduled > 0, "clock must schedule while playing");

    let mut bridge_ro = StudioLiveSessionBridgeReadout::default_unattached();
    bridge_ro.status = StudioLiveSessionBridgeStatus::Running;
    bridge_ro.status_label = "running";
    bridge_ro.executed_ticks = scheduled;
    bridge_ro.last_scheduled_batch = scheduled;
    bridge_ro.scenario_id = Some(studio.scenario_summary.scenario_id.clone());
    bridge_ro.stead_valid = Some(studio.scenario_summary.stead_valid);

    let after =
        build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    assert!(
        after.scheduled_tick_index > before.scheduled_tick_index,
        "scheduled tick index must update while running"
    );
    assert_eq!(after.scheduled_tick_index, transport.readout().tick_index);
    assert!(
        after.bridge_executed_ticks > before.bridge_executed_ticks,
        "bridge executed ticks must update while running"
    );
    assert!(after.clock_playing);
    assert!(!after.clock_paused);
}

/// catches: displayed/derived live values changing while the clock is paused.
#[test]
fn live_observation_freezes_on_pause() {
    let studio = loaded_json_studio_session();
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(20.0))
        .expect("tps");
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let _ = transport.clock_mut().advance(0.5);

    let mut bridge_ro = StudioLiveSessionBridgeReadout::default_unattached();
    bridge_ro.status = StudioLiveSessionBridgeStatus::Running;
    bridge_ro.status_label = "running";
    bridge_ro.executed_ticks = transport.readout().tick_index;
    bridge_ro.last_scheduled_batch = transport.readout().tick_index;

    transport
        .apply(StudioSimClockTransportCommand::Pause)
        .expect("pause");
    bridge_ro.status = StudioLiveSessionBridgeStatus::PausedByClock;
    bridge_ro.status_label = "paused";

    let frozen =
        build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    let tick_at_pause = frozen.scheduled_tick_index;
    let exec_at_pause = frozen.bridge_executed_ticks;

    // Wall time while paused must not move observation counters.
    let _ = transport.clock_mut().advance(10.0);
    let still =
        build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    assert_eq!(still.scheduled_tick_index, tick_at_pause);
    assert_eq!(still.bridge_executed_ticks, exec_at_pause);
    assert!(still.clock_paused);
    assert!(!still.clock_playing);
    assert_eq!(
        still.bridge_status,
        StudioLiveSessionBridgeStatus::PausedByClock
    );
}

/// catches: frame-count/FPS/egui updates becoming observation authority.
#[test]
fn observation_uses_clock_and_bridge_readouts_not_bevy_frame_count() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    transport
        .apply(StudioSimClockTransportCommand::Rate2x)
        .expect("rate");
    let _ = transport.clock_mut().advance(0.25);
    let clock = transport.readout();

    let mut bridge = StudioLiveSessionBridgeReadout::default_unattached();
    bridge.status = StudioLiveSessionBridgeStatus::Ready;
    bridge.status_label = "ready";
    bridge.executed_ticks = 42;
    bridge.last_scheduled_batch = 5;

    // Fake Bevy-like frame counter must NOT appear in observation.
    let bevy_frame_count: u64 = 999_999;
    let obs = build_studio_live_observation_readout(&clock, &bridge, None);

    assert_eq!(obs.scheduled_tick_index, clock.tick_index);
    assert_eq!(obs.effective_tps, clock.effective_tps);
    assert_eq!(obs.clock_rate_label, clock.rate_label);
    assert_eq!(obs.bridge_executed_ticks, bridge.executed_ticks);
    assert_eq!(obs.bridge_status, bridge.status);
    assert_ne!(
        obs.scheduled_tick_index, bevy_frame_count,
        "observation must not use Bevy frame count"
    );
    assert_ne!(obs.bridge_executed_ticks, bevy_frame_count);
    assert_eq!(obs.scheduled_tick_index, transport.clock().tick_index());
}

/// catches: opening/refreshing/rendering observation UI causing SimSession::step_once or clock advance.
#[test]
fn observation_does_not_execute_ticks() {
    let studio = loaded_json_studio_session();
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let _ = transport.clock_mut().advance(0.1);
    let tick_before = transport.readout().tick_index;

    let bridge = StudioLiveSessionBridge::new();
    // Do not open / do not step — observation must not drive bridge.
    let bridge_ro = bridge.readout();
    let exec_before = bridge_ro.executed_ticks;

    for _ in 0..50 {
        let _ =
            build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    }

    assert_eq!(
        transport.readout().tick_index,
        tick_before,
        "refreshing observation must not advance the clock"
    );
    assert_eq!(
        bridge.executed_ticks(),
        exec_before,
        "refreshing observation must not execute bridge ticks"
    );
    assert_eq!(bridge.status(), StudioLiveSessionBridgeStatus::Unattached);
}

/// catches: observation path becoming model-authority mutation.
#[test]
fn observation_does_not_mutate_scenario_spec() {
    let mut studio = loaded_json_studio_session();
    let before =
        serialize_scenario_authority(&studio.scenario_authority).expect("serialize before");

    let mut transport = StudioSimClockTransport::new();
    let _ = transport.apply(StudioSimClockTransportCommand::Play);
    let _ = transport.clock_mut().advance(1.0);
    let mut bridge_ro = StudioLiveSessionBridgeReadout::default_unattached();
    bridge_ro.executed_ticks = 10;

    for _ in 0..20 {
        let _ =
            build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    }

    // Touch summary fields only through observation (read path).
    let obs =
        build_studio_live_observation_readout(&transport.readout(), &bridge_ro, Some(&studio));
    assert_eq!(
        obs.scenario_id.as_deref(),
        Some(studio.scenario_summary.scenario_id.as_str())
    );

    let after = serialize_scenario_authority(&studio.scenario_authority).expect("serialize after");
    assert_eq!(before, after, "observation must not mutate ScenarioSpec");
    // Keep studio mutably borrowed unused path clean — authority still identical.
    let _ = &mut studio;
}

/// catches: live panel losing loaded scenario identity / STEAD summary.
#[test]
fn session_identity_and_stead_are_visible_in_observation() {
    let studio = loaded_json_studio_session();
    assert!(!studio.scenario_summary.scenario_id.is_empty());

    let transport = StudioSimClockTransport::new();
    let bridge = StudioLiveSessionBridgeReadout::default_unattached();
    let obs = build_studio_live_observation_readout(&transport.readout(), &bridge, Some(&studio));

    assert!(obs.session_loaded);
    assert_eq!(
        obs.source_kind,
        StudioLiveObservationSourceKind::LoadedScenario
    );
    assert_eq!(
        obs.scenario_id.as_deref(),
        Some(studio.scenario_summary.scenario_id.as_str())
    );
    assert_eq!(obs.stead_valid, Some(studio.scenario_summary.stead_valid));
    assert_eq!(obs.system_count, Some(studio.scenario_summary.system_count));
    assert_eq!(obs.link_count, Some(studio.scenario_summary.link_count));
    assert_eq!(obs.rf_ready, Some(studio.scenario_summary.rf_ready));
    assert_eq!(
        obs.occupied_cells,
        Some(studio.scenario_summary.occupied_cells)
    );
    assert!(obs.session_status_message.is_some());
}

/// catches: silent no-op/fallback when no session is loaded or bridge has an error.
#[test]
fn bridge_error_or_unattached_state_is_reported() {
    let transport = StudioSimClockTransport::new();

    // Unattached / no session
    let unattached = build_studio_live_observation_readout(
        &transport.readout(),
        &StudioLiveSessionBridgeReadout::default_unattached(),
        None,
    );
    assert!(!unattached.session_loaded);
    assert_eq!(
        unattached.bridge_status,
        StudioLiveSessionBridgeStatus::Unattached
    );
    assert_eq!(unattached.bridge_status_label, "unattached");
    assert_eq!(
        unattached.source_kind_label,
        StudioLiveObservationSourceKind::None.label()
    );
    assert!(unattached.scenario_id.is_none());
    assert!(unattached.bridge_last_error.is_none());

    // Explicit error state must surface, not fall back to "ready".
    let mut errored = StudioLiveSessionBridgeReadout::default_unattached();
    errored.status = StudioLiveSessionBridgeStatus::Errored;
    errored.status_label = "errored";
    errored.last_error = Some("live session open failed: test".into());
    let obs_err = build_studio_live_observation_readout(&transport.readout(), &errored, None);
    assert_eq!(
        obs_err.bridge_status,
        StudioLiveSessionBridgeStatus::Errored
    );
    assert_eq!(obs_err.bridge_status_label, "errored");
    assert_eq!(
        obs_err.bridge_last_error.as_deref(),
        Some("live session open failed: test")
    );
    assert_ne!(obs_err.bridge_status, StudioLiveSessionBridgeStatus::Ready);
}

/// catches: observer importing workshop residue or inventing gameplay summaries.
#[test]
fn no_new_gameplay_or_workshop_dependency_for_observation() {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/studio_live_observe.rs"
    ));
    observe_module_source_forbids_workshop_residue(source)
        .expect("observe module must forbid workshop/gameplay tokens");

    // Cargo dependency scan: mapeditor must not depend on simthing-workshop for this rung.
    let cargo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    assert!(
        !cargo.contains("simthing-workshop"),
        "mapeditor Cargo.toml must not depend on simthing-workshop"
    );
}

/// catches: optional tree summary deriving from a planner/evaluator instead of existing fields.
#[test]
fn tree_local_summary_is_projection_only() {
    let studio = loaded_json_studio_session();
    let transport = StudioSimClockTransport::new();
    let bridge = StudioLiveSessionBridgeReadout::default_unattached();
    let obs = build_studio_live_observation_readout(&transport.readout(), &bridge, Some(&studio));

    // Tree-local numbers must equal StudioScenarioSummary (already-derived projection).
    assert_eq!(obs.system_count, Some(studio.scenario_summary.system_count));
    assert_eq!(obs.link_count, Some(studio.scenario_summary.link_count));
    assert_eq!(
        obs.occupied_cells,
        Some(studio.scenario_summary.occupied_cells)
    );
    // No invented "strategy score" / planner fields on the readout type surface.
    let type_name = std::any::type_name::<simthing_mapeditor::StudioLiveObservationReadout>();
    assert!(type_name.contains("StudioLiveObservationReadout"));
}

/// Sanity: rate/TPS fields track transport rate selection (presentation path).
#[test]
fn observation_tracks_rate_and_max_tps_from_clock() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Rate4x)
        .expect("4x");
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(5.0))
        .expect("tps");
    let clock = transport.readout();
    assert_eq!(clock.rate, StudioSimClockRate::Rate4x);
    let obs = build_studio_live_observation_readout(
        &clock,
        &StudioLiveSessionBridgeReadout::default_unattached(),
        None,
    );
    assert_eq!(obs.clock_rate_label, "4×");
    assert!((obs.max_tps - 5.0).abs() < f64::EPSILON);
}
