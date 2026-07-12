//! STUDIO-CLAUSE-LOADER-SIMPLIFY-0 — ClauseScript-only loader + source_base + telemetry.

use std::env;
use std::path::{Path, PathBuf};

use simthing_mapeditor::{
    build_studio_scenario_telemetry_readout, ingest_clause_scenario_path,
    default_clause_picker_start_directory, request_live_bridge_reset_after_session_replacement,
    run_clause_picker_action, runtime_vertical_seed_scenario_spec, ClausePickerActionResult,
    ClausePickerSelection, ClauseScenarioIngestOptions, StudioScenarioLibraryModel,
    StudioScenarioLibraryTab, StudioSession, StudioSimClockTransport,
    StudioSimClockTransportCommand,
};
use simthing_spec::{serialize_scenario_authority, validate_stead_mapping_consistency};
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn loaded_session() -> StudioSession {
    StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("runtime_vertical_seed.simthing-scenario.json"),
        None,
    )
    .expect("load runtime vertical seed")
}

/// catches: JSON loader still visible in Scenario Library.
#[test]
fn clause_loader_ui_hides_json_load_controls() {
    let ui_src = include_str!("../src/app/ui.rs");
    assert!(
        !ui_src.contains("draw_scenario_library_json_controls"),
        "JSON library controls function must be removed"
    );
    // Tab strip must not offer JSON selectable (label "JSON" on library modal).
    assert!(
        !ui_src.contains("StudioScenarioLibraryTab::Json,\n                \"JSON\""),
        "JSON tab must not be offered in Scenario Library UI"
    );
    assert!(
        !ui_src.contains("\"JSON\""),
        "Scenario Library UI must not show JSON tab label"
    );
    // Create + Clause remain.
    assert!(ui_src.contains("ClauseScript"));
    assert!(ui_src.contains("Create blank scenario") || ui_src.contains("Create"));
}

/// catches: visible TOKEN=path resolver still present.
#[test]
fn clause_loader_ui_hides_resolver_textbox() {
    let ui_src = include_str!("../src/app/ui.rs");
    assert!(
        !ui_src.contains("Resolver entries (TOKEN=path"),
        "visible resolver textbox label must be removed"
    );
    assert!(
        !ui_src.contains("clause_resolver_text"),
        "clause_resolver_text must not be edited in UI after 11.4"
    );
}

/// catches: process-cwd relative regression from 11.1 residual.
#[test]
fn clause_loader_uses_source_base_for_relative_sibling() {
    let ingest_src = include_str!("../src/clause_scenario_ingest.rs");
    assert!(
        ingest_src.contains("hydrate_scenario_with_source_base"),
        "production ingest must call hydrate_scenario_with_source_base"
    );
    assert!(
        ingest_src.contains("path.parent()") || ingest_src.contains("source_base"),
        "ingest must pass clause parent as source_base"
    );

    let clause = canonical_clause_path();
    assert!(clause.is_file());
    let options = ClauseScenarioIngestOptions::default();
    let result = ingest_clause_scenario_path(&clause, &options).expect("ingest with source_base");
    assert_eq!(result.scenario.scenario_id, "terran_pirate_galaxy");
    assert!(!result.scenario.structural_grid.placements.is_empty());
}

/// catches: canonical operator path failure.
#[test]
fn clause_loader_canonical_clause_loads_from_alien_cwd() {
    let original = env::current_dir().expect("cwd");
    let alien = repo_root().join("crates").join("simthing-mapeditor");
    env::set_current_dir(&alien).expect("chdir alien");
    let result = std::panic::catch_unwind(|| {
        assert_eq!(
            default_clause_picker_start_directory(""),
            repo_root().join("scenarios"),
            "empty ClauseScript picker must open on portable operator scenarios"
        );
        let clause = canonical_clause_path();
        let options = ClauseScenarioIngestOptions::default();
        let result =
            ingest_clause_scenario_path(&clause, &options).expect("alien cwd canonical load");
        assert_eq!(result.scenario.scenario_id, "terran_pirate_galaxy");
        validate_stead_mapping_consistency(&result.scenario).expect("STEAD");
    });
    env::set_current_dir(&original).expect("restore");
    result.expect("canonical load from alien cwd");
}

/// catches: live bridge stale session after load.
#[test]
fn clause_loader_requests_bridge_reset_on_replace() {
    let ui_src = include_str!("../src/app/ui.rs");
    assert!(
        ui_src.contains("request_live_bridge_reset_after_session_replacement"),
        "clause load path must request live bridge reset"
    );
    let mut flag = false;
    request_live_bridge_reset_after_session_replacement(&mut flag);
    assert!(flag, "reset flag must latch");
}

/// catches: modal pause regression.
#[test]
fn clause_loader_modal_pause_no_autoplay() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut library = StudioScenarioLibraryModel::default();
    assert_eq!(library.selected_tab, StudioScenarioLibraryTab::Clause);
    library.open(&mut transport);
    assert!(library.visible);
    assert!(transport.clock().is_paused());
    library.close();
    assert!(!library.visible);
    assert!(transport.clock().is_paused());
    assert_eq!(transport.clock_mut().advance(5.0), 0);
}

/// catches: missing scenario id/path/status telemetry.
#[test]
fn clause_loader_telemetry_scenario_section_reports_identity() {
    let ui_src = include_str!("../src/app/ui.rs");
    assert!(
        ui_src.contains("draw_telemetry_scenario_section")
            || ui_src.contains("CollapsingHeader::new(\"Scenario\")"),
        "Telemetry must include Scenario section"
    );
    let session = loaded_session();
    let transport = StudioSimClockTransport::new();
    let clock = transport.readout();
    let tel = build_studio_scenario_telemetry_readout(
        Some(&session),
        "scenarios/terran_pirate_galaxy.clause",
        &clock,
    );
    assert!(!tel.scenario_id.is_empty());
    assert_eq!(tel.clause_path, "scenarios/terran_pirate_galaxy.clause");
    assert!(tel.source_path.contains("runtime_vertical_seed") || !tel.source_path.is_empty());
    assert!(tel.resolver_state.contains("empty"));
}

/// catches: telemetry shell without useful content.
#[test]
fn clause_loader_telemetry_reports_counts_and_stead() {
    let session = loaded_session();
    let mut transport = StudioSimClockTransport::new();
    let _ = transport.apply(StudioSimClockTransportCommand::Play);
    let _ = transport.clock_mut().advance(0.5);
    let clock = transport.readout();
    let tel = build_studio_scenario_telemetry_readout(Some(&session), "", &clock);
    assert!(tel.system_count > 0);
    assert!(tel.stead_label.contains("STEAD"));
    assert_eq!(tel.tick_index, clock.tick_index);
    assert!(!tel.paused);
}

/// catches: read-only telemetry mutating authority.
#[test]
fn clause_loader_no_spec_mutation_from_telemetry() {
    let session = loaded_session();
    let before = serialize_scenario_authority(&session.scenario_authority).expect("ser");
    let transport = StudioSimClockTransport::new();
    for _ in 0..20 {
        let _ = build_studio_scenario_telemetry_readout(
            Some(&session),
            "path.clause",
            &transport.readout(),
        );
    }
    let after = serialize_scenario_authority(&session.scenario_authority).expect("ser after");
    assert_eq!(before, after, "telemetry must not mutate ScenarioSpec");
}

/// catches: production path still fails loud when {{TOKEN}} present without resolver.
#[test]
fn clause_loader_legacy_token_fails_loud_without_visible_resolver() {
    let dir = TempDir::new().expect("tmp");
    let fixture_clause = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause");
    // Fixture uses {{FIXTURE_JSON}} — empty operator resolver must fail loud.
    let selection = ClausePickerSelection {
        clause_path: fixture_clause,
        resolver_entries: Default::default(),
        scenario_json_path: Some(dir.path().join("out.simthing-scenario.json")),
    };
    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Failed { message } => {
            assert!(
                message.contains("unresolved")
                    || message.contains("resolver")
                    || message.contains("placeholder")
                    || message.contains("FIXTURE"),
                "expected fail-loud token message, got {message}"
            );
        }
        other => panic!("expected token failure, got {}", other.message()),
    }
}
