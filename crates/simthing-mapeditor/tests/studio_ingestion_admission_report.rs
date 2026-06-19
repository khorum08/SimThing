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
fn studio_displays_admitted_canonical_scenario_report() {
    let json = load_corpus_json("minimal_scenario_galaxymap.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("minimal_galaxymap", &json);
    assert!(matches!(
        report.classification.as_str(),
        "Admitted" | "PartiallyAdmitted"
    ));
    assert_eq!(report.canonical_tree_status, "canonical_valid");
    assert!(!report.legacy_world_root);
}

#[test]
fn studio_displays_rejected_missing_gamesession_report() {
    let json = load_corpus_json("invalid_missing_gamesession.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("missing_gs", &json);
    assert_eq!(report.classification, "Rejected");
    assert!(report.validation_error_count > 0);
    assert_eq!(report.canonical_tree_status, "canonical_invalid");
}

#[test]
fn studio_displays_rejected_duplicate_owner_report() {
    let json = load_corpus_json("invalid_duplicate_owner_ids.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("dup_owners", &json);
    assert_eq!(report.classification, "Rejected");
    assert!(report
        .errors
        .iter()
        .any(|e| e.message.contains("duplicate owner_id")));
}

#[test]
fn studio_displays_unsupported_planet_child_deferral() {
    let json = load_corpus_json("unsupported_planet_child_valid_schema.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("planet_child", &json);
    assert_ne!(report.classification, "Rejected");
    assert!(report
        .deferrals
        .iter()
        .any(|d| d.kind == "PlanetsNotYetAdmitted"));
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
fn studio_displays_owner_silo_admission_summary() {
    let json = load_corpus_json("owner_silo_balanced_flow.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("owner_silo_balanced", &json);
    let silo = report.owner_silo.expect("owner silo summary");
    assert!(matches!(
        silo.classification.as_str(),
        "Admitted" | "PartiallyAdmitted"
    ));
    assert_eq!(silo.participant_count, 2);
    assert_eq!(silo.reducible_surplus_total, 30.0);
    assert_eq!(silo.resolvable_deficit_total, 20.0);
}

#[test]
fn studio_displays_owner_silo_gpu_participant_ready_and_full_mutation_deferred() {
    let json = load_corpus_json("owner_silo_balanced_flow.simthing-scenario.json");
    let report = studio_ingest_scenario_text_for_report("owner_silo_gpu", &json);
    let silo = report.owner_silo.expect("owner silo");
    assert!(silo.gpu_participant_accumulation_ready);
    assert!(silo.full_state_mutation_deferred);
    assert!(
        report
            .compile_readiness
            .owner_silo_gpu_participant_accumulation_ready
    );
    assert!(
        report
            .compile_readiness
            .owner_silo_full_state_mutation_deferred
    );
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

#[test]
fn studio_ingestion_report_does_not_dispatch_gpu() {
    let mapeditor_lib = include_str!("../src/lib.rs");
    let admission_src = include_str!("../src/studio_admission_report.rs");
    let session_src = include_str!("../src/session.rs");
    for forbidden in [
        "SimGpuAccumulatorTickState",
        "compile_owner_silo_gpu_tick_plan",
        "AccumulatorOpSession",
        "gpu_context_blocking",
        "execute_accumulator_plan_tick",
    ] {
        assert!(
            !mapeditor_lib.contains(forbidden)
                && !admission_src.contains(forbidden)
                && !session_src.contains(forbidden),
            "mapeditor must not dispatch GPU via {forbidden}"
        );
    }
    assert!(admission_src.contains("ingest_scenario_from_str"));
    assert!(admission_src.contains("build_studio_admission_summary_from_ingestion"));
}

#[test]
fn studio_ingestion_report_does_not_mutate_scenario_authority() {
    let json = load_corpus_json("owner_silo_balanced_flow.simthing-scenario.json");
    let spec = deserialize_scenario_authority(&json).expect("parse");
    let before = studio_scenario_authority_snapshot(&spec);
    let _ = studio_ingest_scenario_text_for_report("mutation_guard", &json);
    let after = studio_scenario_authority_snapshot(&spec);
    assert_eq!(before, after);

    let (ingestion, _) = ingest_scenario_from_str(
        "mutation_guard_ingest",
        &json,
        studio_canonical_ingestion_profile(),
    );
    assert_ne!(
        ingestion.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert_eq!(studio_scenario_authority_snapshot(&spec), before);
}

#[test]
fn studio_loaded_session_carries_admission_summary() {
    let path = corpus_path("minimal_scenario_root.simthing-scenario.json");
    let session = load_studio_session_from_scenario_path(&path, None).expect("load session");
    assert!(matches!(
        session.admission_summary.classification.as_str(),
        "Admitted" | "PartiallyAdmitted"
    ));
    assert!(session.scenario_document.admission_summary.is_some());
}

#[test]
fn studio_admission_summary_from_spec_matches_text_ingestion() {
    let json = load_corpus_json("minimal_scenario_root.simthing-scenario.json");
    let spec = deserialize_scenario_authority(&json).expect("parse");
    let from_text = studio_ingest_scenario_text_for_report("from_text", &json);
    let from_spec = build_studio_admission_summary_from_spec("from_spec", &spec);
    assert_eq!(from_text.classification, from_spec.classification);
    assert_eq!(
        from_text.canonical_tree_status,
        from_spec.canonical_tree_status
    );
}
