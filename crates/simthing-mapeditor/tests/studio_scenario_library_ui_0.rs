//! STUDIO-SCENARIO-LIBRARY-UI-0 focused modal, authority I/O, and scope proofs.

use std::path::PathBuf;

use simthing_mapeditor::{
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    run_clause_picker_action, runtime_vertical_seed_scenario_spec,
    save_current_session_scenario_to_path, save_scenario_authority_to_path,
    ClausePickerActionResult, ClausePickerSelection, StudioLiveSessionBridge,
    StudioScenarioLibraryModel, StudioScenarioLibraryTab, StudioSession, StudioSimClockTransport,
    StudioSimClockTransportCommand,
};
use simthing_spec::{serialize_scenario_authority, validate_stead_mapping_consistency};
use tempfile::TempDir;

const CLAUSE_PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

fn loaded_session() -> StudioSession {
    StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("runtime_vertical_seed.simthing-scenario.json"),
        None,
    )
    .expect("load runtime vertical seed")
}

fn clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause")
}

fn clause_fixture_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn clause_selection(json_out: PathBuf, with_resolver: bool) -> ClausePickerSelection {
    let mut resolver_entries = std::collections::BTreeMap::new();
    if with_resolver {
        resolver_entries.insert(CLAUSE_PLACEHOLDER.to_string(), clause_fixture_json_path());
    }
    ClausePickerSelection {
        clause_path: clause_path(),
        resolver_entries,
        scenario_json_path: Some(json_out),
    }
}

#[test]
fn scenario_library_open_pauses_clock() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();

    library.open(&mut transport);

    assert!(library.visible);
    assert!(transport.clock().is_paused());
}

#[test]
fn scenario_library_visible_freezes_live_bridge_execution() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);
    let mut bridge = StudioLiveSessionBridge::new();

    let ran = bridge
        .tick_from_clock(transport.clock_mut(), None, 10.0)
        .expect("paused bridge tick");

    assert_eq!(ran, 0);
    assert_eq!(bridge.executed_ticks(), 0);
    assert_eq!(transport.clock().tick_index(), 0);
}

#[test]
fn scenario_library_close_does_not_autoplay() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);

    library.close();

    assert!(!library.visible);
    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock_mut().advance(10.0), 0);
}

#[test]
fn scenario_library_load_uses_existing_scenario_io() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("library-load.simthing-scenario.json");
    let authority = runtime_vertical_seed_scenario_spec();
    save_scenario_authority_to_path(&path, &authority).expect("seed authority");

    let loaded = load_studio_session_from_scenario_path(&path, None).expect("production load");

    assert_eq!(loaded.scenario_authority.scenario_id, authority.scenario_id);
    assert_eq!(
        loaded.scenario_authority.structural_grid.map_container_id,
        authority.structural_grid.map_container_id
    );
    assert_eq!(
        loaded.scenario_authority.structural_grid.placements,
        authority.structural_grid.placements
    );
    assert_eq!(loaded.scenario_authority.links.len(), authority.links.len());
    assert!(loaded.is_loaded_scenario());
    let library_src = include_str!("../src/studio_scenario_library_ui.rs");
    assert!(!library_src.contains("serde_json"));
    assert!(!library_src.contains("read_to_string"));
}

#[test]
fn scenario_library_save_writes_scenario_authority_only() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("library-save.simthing-scenario.json");
    let session = loaded_session();

    let expected_json =
        serialize_scenario_authority(&session.scenario_authority).expect("serialize session");
    save_current_session_scenario_to_path(&session, &path).expect("production save");
    let saved = load_scenario_authority_from_path(&path).expect("reload authority");

    let text = std::fs::read_to_string(path).expect("saved json");
    assert_eq!(text, expected_json);
    assert_eq!(saved.scenario_id, session.scenario_authority.scenario_id);
    assert!(!text.contains("view_model"));
    assert!(!text.contains("live_bridge_readout"));
}

#[test]
fn scenario_library_load_preserves_stead_and_session_identity() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("identity.simthing-scenario.json");
    let authority = runtime_vertical_seed_scenario_spec();
    let expected_id = authority.scenario_id.clone();
    save_scenario_authority_to_path(&path, &authority).expect("seed authority");

    let loaded = load_studio_session_from_scenario_path(&path, None).expect("production load");

    assert_eq!(loaded.scenario_authority.scenario_id, expected_id);
    assert_eq!(loaded.scenario_summary.scenario_id, expected_id);
    assert!(loaded.scenario_summary.stead_valid);
    validate_stead_mapping_consistency(&loaded.scenario_authority).expect("STEAD");
}

#[test]
fn scenario_library_clause_open_reuses_production_ingest_path() {
    let dir = TempDir::new().expect("tempdir");
    let selection = clause_selection(
        dir.path().join("library-clause.simthing-scenario.json"),
        true,
    );

    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Loaded {
            session, ingest, ..
        } => {
            assert_eq!(ingest.report.projection_mode, "StructuralRebindReady");
            assert_eq!(
                session.scenario_authority.scenario_id,
                ingest.scenario.scenario_id
            );
            assert_eq!(
                session.scenario_authority.structural_grid.map_container_id,
                ingest.scenario.structural_grid.map_container_id
            );
            assert_eq!(
                session.scenario_authority.structural_grid.placements.len(),
                ingest.scenario.structural_grid.placements.len()
            );
            validate_stead_mapping_consistency(&session.scenario_authority).expect("STEAD");
        }
        other => panic!("production ClauseScript open failed: {}", other.message()),
    }

    let ui_src = include_str!("../src/app/ui.rs");
    assert!(ui_src.contains("open_native_clause_scenario_picker"));
    assert!(ui_src.contains("clause_picker_menu_label"));
}

#[test]
fn scenario_library_clause_open_requires_explicit_resolver_when_needed() {
    let dir = TempDir::new().expect("tempdir");
    let selection = clause_selection(dir.path().join("unresolved.simthing-scenario.json"), false);

    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Failed { message } => {
            assert!(
                message.contains("placeholder")
                    || message.contains("unresolved")
                    || message.contains("resolver"),
                "{message}"
            );
        }
        other => panic!(
            "expected explicit resolver failure, got {}",
            other.message()
        ),
    }
}

#[test]
fn scenario_library_create_affordance_is_available() {
    let mut library = StudioScenarioLibraryModel {
        selected_tab: StudioScenarioLibraryTab::Create,
        ..Default::default()
    };
    let mut transport = StudioSimClockTransport::new();
    library.open(&mut transport);

    assert!(library.create_is_available());
    assert!(!library.create_scenario_id.is_empty());
}

#[test]
fn scenario_library_does_not_mutate_spec_without_load_or_save() {
    let session = loaded_session();
    let before = serialize_scenario_authority(&session.scenario_authority).expect("serialize");
    let mut transport = StudioSimClockTransport::new();
    let mut library = StudioScenarioLibraryModel::default();

    library.open(&mut transport);
    library.selected_tab = StudioScenarioLibraryTab::Clause;
    library.enforce_pause(&mut transport);
    library.close();

    let after = serialize_scenario_authority(&session.scenario_authority).expect("serialize after");
    assert_eq!(before, after);
}

#[test]
fn scenario_library_has_no_workshop_or_gameplay_dependency() {
    let library_src = include_str!("../src/studio_scenario_library_ui.rs");
    let cargo = include_str!("../Cargo.toml");
    for banned in [
        "simthing_workshop",
        "BoundaryRequest",
        "GameModeSpec",
        "attach_rf",
        "combat_arena",
    ] {
        assert!(!library_src.contains(banned), "library contains {banned}");
    }
    assert!(!cargo.contains("simthing-workshop"));
}

#[cfg(windows)]
#[test]
fn scenario_library_status_reports_io_errors_without_silent_fallback() {
    use simthing_mapeditor::app::{scenario_io, StudioAppState};

    let mut state = StudioAppState::default();
    let missing = PathBuf::from("missing-library-file.simthing-scenario.json");
    let result = scenario_io::load_scenario_action(&mut state, &missing);

    assert!(!result.is_success());
    assert_eq!(state.last_scenario_io_status, result.message());
    assert!(state.last_scenario_io_status.contains("failed"));
}

#[cfg(not(windows))]
#[test]
fn scenario_library_status_reports_io_errors_without_silent_fallback() {
    let missing = PathBuf::from("missing-library-file.simthing-scenario.json");
    let err = load_studio_session_from_scenario_path(&missing, None)
        .expect_err("missing authority must report an error");
    assert!(!err.to_string().is_empty());
    let action_src = include_str!("../src/app/scenario_io.rs");
    assert!(action_src.contains("record_scenario_io_status"));
}
