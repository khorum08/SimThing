//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion admission tests.

use std::fs;
use std::path::PathBuf;

use simthing_spec::{
    ingest_scenario_from_str, ScenarioDeferralKind, ScenarioIngestionClassification,
    ScenarioIngestionProfile,
};

const CANONICAL_PROFILE: ScenarioIngestionProfile = ScenarioIngestionProfile {
    require_canonical_tree: true,
    admit_legacy_world_root: true,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn repo_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(name)
}

fn resolve_corpus_path_reference(reference: &str) -> PathBuf {
    let trimmed = reference.trim();
    let literal = PathBuf::from(trimmed);
    if literal.exists() {
        return literal;
    }

    let normalized = trimmed.replace('\\', "/");
    if let Some(index) = normalized.find("scenarios/") {
        let candidate = repo_path(&normalized[index..]);
        if candidate.exists() {
            return candidate;
        }
    }

    literal
}

fn load_corpus(name: &str) -> String {
    fs::read_to_string(corpus_path(name)).unwrap_or_else(|_| panic!("missing corpus {name}"))
}

#[test]
fn rejects_missing_owner() {
    let json = load_corpus("invalid_missing_owner.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("missing_owner", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.owners_ok);
}

#[test]
fn rejects_duplicate_owner_ids() {
    let json = load_corpus("invalid_duplicate_owner_ids.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("dup_owners", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(result
        .errors
        .iter()
        .any(|e| e.message.contains("duplicate owner_id")));
}

#[test]
fn classifies_legacy_terran_pirate_as_legacy_compatibility_not_canonical() {
    let reference = load_corpus("legacy_world_root_terran_pirate_reference.txt");
    let path = resolve_corpus_path_reference(&reference);
    let json = fs::read_to_string(&path).expect("terran pirate path");
    let (result, _) = ingest_scenario_from_str("terran_pirate", &json, CANONICAL_PROFILE);
    assert_ne!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.canonical_validation_ok);
    assert!(result.validation.legacy_compat_ok);
    assert!(result
        .deferrals
        .iter()
        .any(|d| { d.kind == ScenarioDeferralKind::LegacyWorldRootCompatibility }));
}
