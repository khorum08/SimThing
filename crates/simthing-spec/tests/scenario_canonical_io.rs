//! SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 — canonical ScenarioSpec load/save proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    load_scenario_spec_from_json_str, prove_scenario_canonical_load_save_roundtrip,
    save_scenario_spec_to_canonical_json, SpecError,
};

const OWNER_SILO_FIXTURE: &str = "owner_silo_disburse_down_scoped.simthing-scenario.json";

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_owner_silo_fixture_json() -> String {
    fs::read_to_string(corpus_path(OWNER_SILO_FIXTURE))
        .unwrap_or_else(|_| panic!("missing corpus {OWNER_SILO_FIXTURE}"))
}

#[test]
fn scenario_canonical_io_saves_deterministic_json() {
    let json = load_owner_silo_fixture_json();
    let (scenario, _) = load_scenario_spec_from_json_str("owner_silo_corpus", &json).expect("load");
    let save_a = save_scenario_spec_to_canonical_json(&scenario).expect("save a");
    let save_b = save_scenario_spec_to_canonical_json(&scenario).expect("save b");

    assert!(save_a.deterministic);
    assert_eq!(save_a.canonical_json, save_b.canonical_json);
    assert_eq!(save_a.authority_digest, save_b.authority_digest);
    assert!(save_a.byte_len > 0);
}

#[test]
fn scenario_canonical_io_roundtrip_preserves_authority_digest() {
    let json = load_owner_silo_fixture_json();
    let report = prove_scenario_canonical_load_save_roundtrip("owner_silo_corpus", &json)
        .expect("roundtrip");

    assert!(report.digest_stable);
    assert_eq!(report.initial_digest, report.roundtrip_digest);
    assert!(report.scenario_authority_preserved);
    assert!(report.canonical_bytes_stable);
}

#[test]
fn scenario_canonical_io_roundtrip_preserves_ingestion_readiness() {
    let json = load_owner_silo_fixture_json();
    let report = prove_scenario_canonical_load_save_roundtrip("owner_silo_corpus", &json)
        .expect("roundtrip");

    assert!(report.initial_load.ingestion_ready);
    assert!(report.roundtrip_load.ingestion_ready);
}
