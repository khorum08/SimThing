//! TP-STUDIO-CLAUSE-INGEST-0 — Studio-facing ClauseScript → ScenarioSpec load/save.

use std::path::PathBuf;

use simthing_mapeditor::{
    default_tp_base_disc_json_path, ingest_clause_scenario_path, load_scenario_authority_from_path,
    save_scenario_authority_to_path, StudioClauseIngestOptions,
};
use simthing_spec::save_scenario_spec_to_canonical_json;
use tempfile::TempDir;

const TERRAN_PIRATE_SCENARIO_ID: &str = "terran_pirate_galaxy";

fn approved_clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause",
    )
}

fn default_options() -> StudioClauseIngestOptions {
    StudioClauseIngestOptions {
        embedded_source_json_path: Some(default_tp_base_disc_json_path()),
    }
}

/// Studio-facing ingest: open approved .clause, project to ScenarioSpec, Studio save/reload.
#[test]
fn tp_studio_clause_ingest_0_load_save_roundtrip() {
    let clause_path = approved_clause_path();
    assert!(
        clause_path.is_file(),
        "approved clause fixture missing: {}",
        clause_path.display()
    );
    assert!(
        default_tp_base_disc_json_path().is_file(),
        "embedded base-disc JSON missing"
    );

    // 1–4: Studio-facing ingest service (not test-only clausething helpers).
    let ingest =
        ingest_clause_scenario_path(&clause_path, &default_options()).expect("studio clause ingest");
    assert_eq!(ingest.scenario.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert_eq!(ingest.pack.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert!(
        ingest.pack.authority_root.is_some(),
        "authority_root required for Studio ScenarioSpec projection"
    );
    assert_eq!(
        ingest.pack.owners.len(),
        2,
        "Terran + Pirate owners expected"
    );
    // FULL-TRANSPILE authority projection: empty placements / map_container until install rebind.
    // Studio session STEAD rebuild is out of scope for this wiring rung (see report Boundary).

    // 5–7: existing Studio ScenarioSpec save/load authority path + digest/byte parity.
    let tmp = TempDir::new().expect("tempdir");
    let save_path = tmp.path().join("terran_pirate_from_clause.simthing-scenario.json");
    save_scenario_authority_to_path(&save_path, &ingest.scenario).expect("studio save");

    let reloaded = load_scenario_authority_from_path(&save_path).expect("studio load path");
    assert_eq!(reloaded.scenario_id, TERRAN_PIRATE_SCENARIO_ID);

    let save_a = save_scenario_spec_to_canonical_json(&ingest.scenario).expect("canonical a");
    let save_b = save_scenario_spec_to_canonical_json(&reloaded).expect("canonical b");
    assert_eq!(
        save_a.canonical_json, save_b.canonical_json,
        "reloaded canonical bytes must match saved output (no semantic drift)"
    );
    assert_eq!(save_a.authority_digest, save_b.authority_digest);
}

/// Spanned/source-context error plumbing for Studio status display.
#[test]
fn tp_studio_clause_ingest_0_malformed_clause_error_context() {
    let tmp = TempDir::new().expect("tempdir");
    let bad_path = tmp.path().join("malformed_scenario.clause");
    // Intentionally invalid ClauseScript (unbalanced / incomplete scenario block).
    std::fs::write(
        &bad_path,
        "scenario = broken_tp {\n    metadata = {\n        display_name = \"broken\"\n",
    )
    .expect("write malformed");

    let err = ingest_clause_scenario_path(&bad_path, &StudioClauseIngestOptions::default())
        .expect_err("malformed clause must fail Studio ingest");
    let status = err.status_message();
    assert!(
        !status.is_empty(),
        "status message must be non-empty for UI/status display"
    );
    // Parse or hydrate path should surface ClauseThing context in the Display string.
    assert!(
        status.contains("ClauseThing")
            || status.contains("parse")
            || status.contains("hydrat")
            || status.contains("error"),
        "error should carry source context suitable for UI: {status}"
    );
}
