//! TP-STUDIO-CLAUSE-API-1 — production limited ClauseScript composition proofs.
//!
//! Terran-Pirate fixtures are **caller-supplied test data only** — not production defaults.

use std::path::PathBuf;

use simthing_mapeditor::{
    ingest_clause_scenario_bytes, ingest_clause_scenario_path, load_clause_studio_session_from_path,
    load_scenario_authority_from_path, load_studio_session_from_clause_ingest_result,
    save_clause_scenario_authority_to_path, ClauseScenarioIngestOptions,
    ClauseScenarioProjectionMode, ClauseScenarioSourceResolver,
};
use simthing_spec::{validate_scenario_links, validate_stead_mapping_consistency};
use tempfile::TempDir;

const PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

fn clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause",
    )
}

fn embedded_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

/// Caller-supplied options (explicit TP fixtures only for this test harness).
fn caller_options() -> ClauseScenarioIngestOptions {
    ClauseScenarioIngestOptions {
        projection_mode: ClauseScenarioProjectionMode::StructuralRebindReady,
        source_resolver: ClauseScenarioSourceResolver::new()
            .with_placeholder(PLACEHOLDER, embedded_json_path()),
    }
}

#[test]
fn api_1_clause_path_to_structural_rebind_ready_spec() {
    let result = ingest_clause_scenario_path(&clause_path(), &caller_options()).expect("ingest");
    assert_eq!(
        result.report.projection_mode,
        ClauseScenarioProjectionMode::StructuralRebindReady.as_str()
    );
    assert!(!result.scenario.structural_grid.map_container_id.is_empty());
    assert!(!result.scenario.structural_grid.placements.is_empty());
    assert_eq!(result.report.stead_validation, "PASS");
    validate_stead_mapping_consistency(&result.scenario).expect("stead");
    if !result.scenario.links.is_empty() {
        validate_scenario_links(&result.scenario).expect("links");
    }
}

#[test]
fn api_1_no_production_tp_defaults() {
    // Empty resolver / default options must NOT invent TP fixture paths.
    let options = ClauseScenarioIngestOptions::default();
    assert!(options.source_resolver.placeholder_paths.is_empty());
    let err = ingest_clause_scenario_path(&clause_path(), &options)
        .expect_err("must fail without caller-supplied placeholder resolver");
    let msg = err.status_message();
    assert!(
        msg.contains("unresolved") || msg.contains("placeholder") || msg.contains("resolver"),
        "expected structured resolution error, got: {msg}"
    );
    // Production default options must not hardcode TP fixture paths.
    let default_src = include_str!("../src/clause_scenario_ingest.rs");
    assert!(
        !default_src.contains("tp_base_disc_1500"),
        "production API source must not hardcode tp_base_disc_1500"
    );
    assert!(
        !default_src.contains("terran_pirate_galaxy"),
        "production API source must not hardcode terran_pirate_galaxy"
    );
    assert!(
        !default_src.contains("TP-FULL-TRANSPILE"),
        "production API source must not reference TP-FULL-TRANSPILE"
    );
}

#[test]
fn api_1_authority_serde_roundtrip_preserves_structural_rebind_ready() {
    let result = ingest_clause_scenario_path(&clause_path(), &caller_options()).expect("ingest");
    let tmp = TempDir::new().expect("tmp");
    let path = tmp.path().join("from_clause.simthing-scenario.json");
    save_clause_scenario_authority_to_path(&path, &result.scenario).expect("save");
    let loaded = load_scenario_authority_from_path(&path).expect("load");
    assert_eq!(
        loaded.structural_grid.map_container_id,
        result.scenario.structural_grid.map_container_id
    );
    assert_eq!(
        loaded.structural_grid.placements.len(),
        result.scenario.structural_grid.placements.len()
    );
    validate_stead_mapping_consistency(&loaded).expect("stead after serde");
}

#[test]
fn api_1_studio_session_hydrates_from_clause_api_output() {
    let tmp = TempDir::new().expect("tmp");
    let json_path = tmp.path().join("session.simthing-scenario.json");
    let (ingest, session) =
        load_clause_studio_session_from_path(&clause_path(), &caller_options(), &json_path, None)
            .expect("clause → session");
    assert_eq!(
        session.scenario_authority.scenario_id,
        ingest.scenario.scenario_id
    );
    assert!(
        !session.scenario_authority.structural_grid.placements.is_empty()
    );

    // Direct from_loaded_scenario path on API output.
    let session2 = load_studio_session_from_clause_ingest_result(
        &ingest,
        PathBuf::from("clause_api_label.simthing-scenario.json"),
        None,
    )
    .expect("from_loaded_scenario");
    assert_eq!(
        session2.scenario_authority.structural_grid.map_container_id,
        ingest.scenario.structural_grid.map_container_id
    );
}

#[test]
fn api_1_missing_resolver_or_source_returns_structured_error() {
    let options = ClauseScenarioIngestOptions {
        projection_mode: ClauseScenarioProjectionMode::StructuralRebindReady,
        source_resolver: ClauseScenarioSourceResolver::new(),
    };
    let bytes = std::fs::read(clause_path()).expect("read clause");
    let err = ingest_clause_scenario_bytes(&bytes, &options).expect_err("unresolved");
    let msg = err.status_message();
    assert!(!msg.is_empty());
    assert!(
        msg.contains("placeholder") || msg.contains("unresolved") || msg.contains("resolver"),
        "{msg}"
    );
}

#[test]
fn api_1_no_ui_picker_surface() {
    // API-1 production ingest module remains free of UI dialog hooks (picker is a separate module).
    let api_src = include_str!("../src/clause_scenario_ingest.rs");
    assert!(!api_src.contains("rfd::"));
    assert!(!api_src.contains("FileDialog"));
    assert!(!api_src.contains("open_native"));
    assert!(!api_src.contains("file_picker"));
    assert!(!api_src.contains("Open ClauseScript"));
}
