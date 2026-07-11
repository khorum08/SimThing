//! STUDIO-SCENARIO-LIBRARY-CREATE-0 focused authority-creation proofs.

use simthing_mapeditor::{
    create_blank_studio_session, load_studio_session_from_scenario_path,
    runtime_vertical_seed_scenario_spec, save_current_session_scenario_to_path,
    StudioLiveSessionBridge, StudioScenarioLibraryModel, StudioScenarioLibraryTab, StudioSession,
    StudioSimClockTransport, StudioSimClockTransportCommand,
    STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE,
};
use simthing_spec::{
    serialize_scenario_authority, validate_scenario_links, validate_stead_mapping_consistency,
};
use tempfile::TempDir;

fn prior_session() -> StudioSession {
    StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        "prior.simthing-scenario.json".into(),
        None,
    )
    .expect("prior session")
}

#[test]
fn scenario_library_create_blank_produces_loadable_session() {
    let session = create_blank_studio_session("blank_loadable").expect("create");
    assert_eq!(session.scenario_authority.scenario_id, "blank_loadable");
    assert_eq!(session.scenario_summary.system_count, 1);
    assert!(session.is_loaded_scenario());
    assert_eq!(session.scenario_document.scenario_id, "blank_loadable");
}

#[test]
fn scenario_library_create_session_has_valid_stead_and_links() {
    let session = create_blank_studio_session("blank_valid").expect("create");
    validate_stead_mapping_consistency(&session.scenario_authority).expect("STEAD");
    validate_scenario_links(&session.scenario_authority).expect("links");
    assert!(session.scenario_summary.stead_valid);
    assert!(session.scenario_summary.links_valid);
}

#[test]
fn scenario_library_create_preserves_scenario_authority_boundary() {
    let session = create_blank_studio_session("blank_authority").expect("create");
    let json = serialize_scenario_authority(&session.scenario_authority).expect("serialize");
    assert_eq!(
        session.scenario_authority.provenance.source,
        STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE
    );
    assert!(!json.contains("view_model"));
    assert!(!json.contains("live_bridge_readout"));
    assert!(!json.contains("bevy"));
}

#[test]
fn scenario_library_create_save_load_roundtrip_preserves_identity() {
    let session = create_blank_studio_session("blank_roundtrip").expect("create");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("blank-roundtrip.simthing-scenario.json");
    save_current_session_scenario_to_path(&session, &path).expect("save");

    let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
    assert_eq!(loaded.scenario_authority.scenario_id, "blank_roundtrip");
    assert_eq!(loaded.scenario_summary.system_count, 1);
    assert!(loaded.scenario_summary.stead_valid);
}

#[test]
fn scenario_library_create_failure_leaves_prior_session_intact() {
    let prior = prior_session();
    let before = serialize_scenario_authority(&prior.scenario_authority).expect("before");

    let result = create_blank_studio_session("invalid scenario id");

    assert!(result.is_err());
    let after = serialize_scenario_authority(&prior.scenario_authority).expect("after");
    assert_eq!(before, after);
}

#[test]
fn scenario_library_create_keeps_modal_pause() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let library = StudioScenarioLibraryModel {
        visible: true,
        selected_tab: StudioScenarioLibraryTab::Create,
        ..Default::default()
    };
    library.enforce_pause(&mut transport);

    let _ = create_blank_studio_session(&library.create_scenario_id).expect("create");
    library.enforce_pause(&mut transport);
    assert!(transport.clock().is_paused());
}

#[test]
fn scenario_library_create_close_does_not_autoplay() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);
    let _ = create_blank_studio_session("blank_close").expect("create");

    library.close();

    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock_mut().advance(10.0), 0);
}

#[test]
fn scenario_library_create_does_not_tick_live_bridge() {
    let mut transport = StudioSimClockTransport::new();
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);
    let mut bridge = StudioLiveSessionBridge::new();

    let session = create_blank_studio_session("blank_no_tick").expect("create");
    let ran = bridge
        .tick_from_clock(transport.clock_mut(), Some(&session), 10.0)
        .expect("paused tick");

    assert_eq!(ran, 0);
    assert_eq!(bridge.executed_ticks(), 0);
}

#[test]
fn scenario_library_create_replaces_deferred_create_affordance() {
    let library = StudioScenarioLibraryModel {
        selected_tab: StudioScenarioLibraryTab::Create,
        ..Default::default()
    };
    assert!(library.create_is_available());
    let ui_src = include_str!("../src/app/ui.rs");
    assert!(ui_src.contains("do_create_blank_scenario"));
    assert!(ui_src.contains("Create blank scenario"));
    assert!(!ui_src.contains("STUDIO_SCENARIO_LIBRARY_CREATE_DEFERRED_MESSAGE"));
}

#[test]
fn scenario_library_create_has_no_tp_hardcodes() {
    let create_src = include_str!("../src/studio_scenario_library_ui.rs");
    for banned in [
        "tp_base_disc_1500",
        "terran_pirate",
        "TP-FULL-TRANSPILE",
        "runtime_vertical_seed_scenario_spec",
    ] {
        assert!(
            !create_src.contains(banned),
            "create path contains {banned}"
        );
    }
}

#[test]
fn scenario_library_create_has_no_workshop_or_gameplay_dependency() {
    let create_src = include_str!("../src/studio_scenario_library_ui.rs");
    let cargo = include_str!("../Cargo.toml");
    for banned in [
        "simthing_workshop",
        "BoundaryRequest",
        "GameModeSpec",
        "attach_rf",
        "combat_arena",
        "run_generation",
    ] {
        assert!(
            !create_src.contains(banned),
            "create path contains {banned}"
        );
    }
    assert!(!cargo.contains("simthing-workshop"));
}

#[test]
fn scenario_library_create_reports_errors_without_silent_fallback() {
    let error = create_blank_studio_session("").expect_err("empty id must fail");
    let message = error.to_string();
    assert!(message.contains("invalid scenario id"));
    assert!(message.contains("empty"));
}
