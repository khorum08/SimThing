//! TP-STUDIO-CLAUSE-INGEST-0R — workshop-homed candidate: clause → ScenarioSpec save/reload.

use std::path::PathBuf;

use simthing_spec::save_scenario_spec_to_canonical_json;
use simthing_workshop::{
    default_tp_base_disc_json_path, ingest_tp_clause_scenario_path,
    load_scenario_authority_json_from_path, save_scenario_authority_json_to_path,
    TpStudioClauseIngestOptions,
};
use tempfile::TempDir;

const TERRAN_PIRATE_SCENARIO_ID: &str = "terran_pirate_galaxy";

fn approved_clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause",
    )
}

fn default_options() -> TpStudioClauseIngestOptions {
    TpStudioClauseIngestOptions {
        embedded_source_json_path: Some(default_tp_base_disc_json_path()),
    }
}

/// Workshop-homed candidate ingest + production ScenarioSpec authority save/reload.
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

    // Workshop-homed candidate service (not a production mapeditor API).
    let ingest = ingest_tp_clause_scenario_path(&clause_path, &default_options())
        .expect("workshop TP clause ingest");
    assert_eq!(ingest.scenario.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert_eq!(ingest.pack.scenario_id, TERRAN_PIRATE_SCENARIO_ID);
    assert!(
        ingest.pack.authority_root.is_some(),
        "authority_root required for ScenarioSpec projection"
    );
    assert_eq!(ingest.pack.owners.len(), 2, "Terran + Pirate owners expected");

    // Production ScenarioSpec authority serde (same layer mapeditor scenario_io uses).
    let tmp = TempDir::new().expect("tempdir");
    let save_path = tmp
        .path()
        .join("terran_pirate_from_clause.simthing-scenario.json");
    save_scenario_authority_json_to_path(&save_path, &ingest.scenario).expect("authority save");

    let reloaded = load_scenario_authority_json_from_path(&save_path).expect("authority load");
    assert_eq!(reloaded.scenario_id, TERRAN_PIRATE_SCENARIO_ID);

    let save_a = save_scenario_spec_to_canonical_json(&ingest.scenario).expect("canonical a");
    let save_b = save_scenario_spec_to_canonical_json(&reloaded).expect("canonical b");
    assert_eq!(
        save_a.canonical_json, save_b.canonical_json,
        "reloaded canonical bytes must match saved output (no semantic drift)"
    );
    assert_eq!(save_a.authority_digest, save_b.authority_digest);
}

/// Error plumbing for workshop-homed candidate status display.
#[test]
fn tp_studio_clause_ingest_0_malformed_clause_error_context() {
    let tmp = TempDir::new().expect("tempdir");
    let bad_path = tmp.path().join("malformed_scenario.clause");
    std::fs::write(
        &bad_path,
        "scenario = broken_tp {\n    metadata = {\n        display_name = \"broken\"\n",
    )
    .expect("write malformed");

    let err = ingest_tp_clause_scenario_path(&bad_path, &TpStudioClauseIngestOptions::default())
        .expect_err("malformed clause must fail workshop ingest");
    let status = err.status_message();
    assert!(!status.is_empty(), "status message must be non-empty");
    assert!(
        status.contains("ClauseThing")
            || status.contains("parse")
            || status.contains("hydrat")
            || status.contains("error")
            || status.contains("TP clause"),
        "error should carry source context: {status}"
    );
}
