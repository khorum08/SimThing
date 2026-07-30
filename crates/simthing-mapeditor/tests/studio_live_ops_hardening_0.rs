//! STUDIO-LIVE-OPS-HARDENING-0 edge-case regression proofs.

use simthing_mapeditor::{
    apply_live_bridge_reset_before_tick, create_blank_studio_session,
    request_live_bridge_reset_after_session_replacement, runtime_vertical_seed_scenario_spec,
    save_current_session_scenario_to_path, StudioLiveSessionBridge, StudioScenarioLibraryModel,
    StudioScenarioLibraryTab, StudioSession, StudioSimClockRate, StudioSimClockTransport,
    StudioSimClockTransportCommand,
};
use simthing_spec::serialize_scenario_authority;
use tempfile::TempDir;

const UI_SOURCE: &str = include_str!("../src/app/ui.rs");

fn loaded_session() -> StudioSession {
    StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        "hardening.simthing-scenario.json".into(),
        None,
    )
    .expect("loaded session")
}

fn action_branch(start: &str, end: &str) -> &'static str {
    let start = UI_SOURCE.find(start).expect("action branch start");
    let end = UI_SOURCE[start + 1..]
        .find(end)
        .map(|offset| start + 1 + offset)
        .expect("action branch end");
    &UI_SOURCE[start..end]
}

/// Pairing invariant over EVERY session-replacement call site.
///
/// The former window-slice form (`action_branch(start, end)`) asserted over a
/// text range bracketed by two `do_*` marker strings. That is reorder-brittle:
/// once the clause-load branch stopped containing the literal
/// `do_open_clause_scenario_picker` (its trigger id is `do_load_clause_scenario`),
/// the slice silently retargeted the `data.remove(..)` cleanup block and the
/// referee asserted over source that never contained either call. This form is
/// immune to ordering and strictly stronger: it catches a NEW unpaired site,
/// which window slicing structurally cannot.
fn assert_every_replacement_requests_reset() {
    const RESET: &str = "request_live_bridge_reset_after_session_replacement";
    let mut idx = 0usize;
    let mut sites = 0usize;
    while let Some(found) = UI_SOURCE[idx..].find("adopt_loaded_scenario_session(") {
        let at = idx + found;
        let end = (at + 600).min(UI_SOURCE.len());
        assert!(
            UI_SOURCE[at..end].contains(RESET),
            "session replacement at byte {at} does not request a live-bridge reset"
        );
        sites += 1;
        idx = at + 1;
    }
    assert!(
        sites >= 5,
        "expected at least 5 session-replacement sites, found {sites}"
    );
}

#[test]
fn hardening_json_load_requests_bridge_reset() {
    assert_every_replacement_requests_reset();
}

#[test]
fn hardening_clause_load_requests_bridge_reset() {
    assert_every_replacement_requests_reset();
}

#[test]
fn hardening_create_still_requests_bridge_reset() {
    assert_every_replacement_requests_reset();
}

#[test]
fn hardening_bridge_reset_detaches_before_next_tick() {
    let mut requested = false;
    let mut bridge = StudioLiveSessionBridge::new();
    request_live_bridge_reset_after_session_replacement(&mut requested);

    assert!(apply_live_bridge_reset_before_tick(
        &mut requested,
        &mut bridge
    ));
    assert!(!requested);
    assert!(!bridge.is_attached());

    let reset = UI_SOURCE.find("apply_live_bridge_reset_before_tick");
    let tick = UI_SOURCE.find("tick_from_clock");
    assert!(reset.is_none(), "reset is owned by app/mod.rs, not UI");
    let app_source = include_str!("../src/app/mod.rs");
    assert!(
        app_source.find("apply_live_bridge_reset_before_tick") < app_source.find("tick_from_clock")
    );
    assert!(
        tick.is_none(),
        "live ticking is owned by app/mod.rs, not UI"
    );
}

#[test]
fn hardening_modal_cancel_keeps_pause_and_no_autoplay() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);

    library.cancel(&mut transport);

    assert!(!library.visible);
    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock_mut().advance(60.0), 0);
}

#[test]
fn hardening_modal_cancel_does_not_fire_pending_actions() {
    let clear = action_branch(
        "fn clear_scenario_library_pending_actions",
        "fn open_scenario_library",
    );
    for action in [
        "do_save_scenario",
        "do_load_scenario_manual",
        "do_load_scenario_picker",
        "do_open_clause_scenario_picker",
        "do_create_blank_scenario",
    ] {
        assert!(clear.contains(action), "cancel does not clear {action}");
    }
    let close = action_branch(
        "if response.inner || response.should_close()",
        "fn clear_scenario",
    );
    assert!(close.contains("cancel_scenario_library"));
    assert!(close.contains("clear_scenario_library_pending_actions"));
}

#[test]
fn hardening_double_open_is_idempotent() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();

    library.open(&mut transport);
    let first = library.clone();
    library.open(&mut transport);

    assert_eq!(library, first);
    assert!(transport.clock().is_paused());
}

#[test]
fn hardening_double_open_preserves_operator_inputs() {
    let mut transport = StudioSimClockTransport::new();
    let mut library = StudioScenarioLibraryModel {
        selected_tab: StudioScenarioLibraryTab::Create,
        create_scenario_id: "operator_draft".into(),
        ..Default::default()
    };

    library.open(&mut transport);
    library.open(&mut transport);

    assert_eq!(library.selected_tab, StudioScenarioLibraryTab::Create);
    assert_eq!(library.create_scenario_id, "operator_draft");
}

fn run_rapid_rate_sequence() -> (u64, StudioSimClockRate) {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::SetMaxTps(8.0))
        .expect("tps");
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut scheduled = 0;
    for command in [
        StudioSimClockTransportCommand::Rate4x,
        StudioSimClockTransportCommand::Rate1x,
        StudioSimClockTransportCommand::Rate2x,
        StudioSimClockTransportCommand::Rate4x,
        StudioSimClockTransportCommand::Rate2x,
    ] {
        transport.apply(command).expect("rate command");
        scheduled += transport.clock_mut().advance(0.125);
    }
    (scheduled, transport.clock().rate())
}

#[test]
fn hardening_rapid_rate_changes_remain_deterministic() {
    let first = run_rapid_rate_sequence();
    let second = run_rapid_rate_sequence();
    assert_eq!(first, second);
    assert_eq!(first, (13, StudioSimClockRate::Rate2x));
}

#[test]
fn hardening_modal_visible_rate_changes_do_not_tick() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);
    transport
        .apply(StudioSimClockTransportCommand::Rate4x)
        .expect("rate");
    library.enforce_pause(&mut transport);
    let mut bridge = StudioLiveSessionBridge::new();

    let ran = bridge
        .tick_from_clock(transport.clock_mut(), None, 10.0)
        .expect("paused tick");
    assert_eq!(ran, 0);
    assert_eq!(transport.clock().tick_index(), 0);
}

#[test]
fn hardening_save_while_paused_does_not_tick_or_autoplay() {
    let dir = TempDir::new().expect("tempdir");
    let session = loaded_session();
    let mut transport = StudioSimClockTransport::new();
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);

    save_current_session_scenario_to_path(
        &session,
        &dir.path().join("paused-save.simthing-scenario.json"),
    )
    .expect("save");
    let mut bridge = StudioLiveSessionBridge::new();
    let ran = bridge
        .tick_from_clock(transport.clock_mut(), Some(&session), 10.0)
        .expect("paused tick");

    assert_eq!(ran, 0);
    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock().tick_index(), 0);
}

#[test]
fn hardening_save_error_is_fail_loud_and_state_safe() {
    let dir = TempDir::new().expect("tempdir");
    let session = loaded_session();
    let before = serialize_scenario_authority(&session.scenario_authority).expect("before");
    let mut transport = StudioSimClockTransport::new();

    let error = save_current_session_scenario_to_path(
        &session,
        &dir.path()
            .join("missing-parent/fail.simthing-scenario.json"),
    )
    .expect_err("missing parent must fail");

    assert!(!error.to_string().is_empty());
    assert_eq!(
        serialize_scenario_authority(&session.scenario_authority).expect("after"),
        before
    );
    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock_mut().advance(10.0), 0);
}

#[test]
fn hardening_no_tick_on_modal_for_all_library_actions() {
    for tab in [
        StudioScenarioLibraryTab::Json,
        StudioScenarioLibraryTab::Clause,
        StudioScenarioLibraryTab::Create,
    ] {
        let session = create_blank_studio_session("modal_action").expect("session");
        let mut transport = StudioSimClockTransport::new();
        transport
            .apply(StudioSimClockTransportCommand::Play)
            .expect("play");
        let mut library = StudioScenarioLibraryModel {
            selected_tab: tab,
            ..Default::default()
        };
        library.open(&mut transport);
        let mut bridge = StudioLiveSessionBridge::new();

        let ran = bridge
            .tick_from_clock(transport.clock_mut(), Some(&session), 2.0)
            .expect("paused tick");
        assert_eq!(ran, 0, "{tab:?} modal ticked");
        assert_eq!(transport.clock().tick_index(), 0);
    }
}
