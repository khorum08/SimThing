//! STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 — Studio admission report presentation proofs.

use std::fs;
use std::path::PathBuf;

use simthing_mapeditor::{
    build_studio_admission_summary_from_spec, load_studio_session_from_scenario_path,
    studio_ingest_scenario_text_for_report, studio_scenario_authority_snapshot,
    StudioScenarioAuthorityKind,
};
use simthing_spec::{
    deserialize_scenario_authority, ingest_scenario_from_str, studio_canonical_ingestion_profile,
    ScenarioIngestionClassification,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_corpus_json(name: &str) -> String {
    fs::read_to_string(corpus_path(name)).expect("corpus")
}

fn terran_pirate_json() -> String {
    let reference = load_corpus_json("legacy_world_root_terran_pirate_reference.txt");
    fs::read_to_string(reference.trim()).expect("terran pirate")
}
#[test]
fn studio_displays_unknown_gridcell_role_deferral() {
    let json = load_corpus_json("unsupported_unknown_gridcell_role.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("unknown_role", &json);
    assert!(matches!(
        report.classification.as_str(),
        "PartiallyAdmitted" | "Unsupported"
    ));
    assert!(report
        .deferrals
        .iter()
        .any(|d| d.kind == "UnsupportedGridcellRole"));
}
#[test]
fn studio_legacy_terran_pirate_report_is_legacy_compatibility() {
    let json = terran_pirate_json();
    let report = studio_ingest_scenario_text_for_report("terran_pirate", &json);
    assert_ne!(report.classification, "Rejected");
    assert!(report.legacy_world_root);
    assert_eq!(
        report.canonical_tree_status,
        "legacy_world_root_compatibility"
    );
    assert!(report
        .deferrals
        .iter()
        .any(|d| d.kind == "LegacyWorldRootCompatibility"));

    let spec = deserialize_scenario_authority(&json).expect("parse terran pirate");
    let document = simthing_mapeditor::build_studio_scenario_document(&spec).expect("legacy doc");
    assert_eq!(
        document.authority_kind,
        StudioScenarioAuthorityKind::LegacyWorldRoot
    );
}
