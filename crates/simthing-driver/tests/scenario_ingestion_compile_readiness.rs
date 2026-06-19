//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — driver compile-readiness for ingested scenarios.

use std::fs;
use std::path::PathBuf;

use simthing_driver::{
    compile_structural_n4_theater, evaluate_scenario_compile_readiness, AtlasDeferralReason,
    StructuralTheaterAdmission,
};
use simthing_spec::{
    ingest_scenario_from_str, MappingExecutionProfile, ScenarioIngestionClassification,
    ScenarioIngestionProfile,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn ingest_corpus(name: &str) -> simthing_spec::SimThingScenarioSpec {
    let json = fs::read_to_string(corpus_path(name)).expect("corpus");
    let profile = ScenarioIngestionProfile {
        require_canonical_tree: true,
        admit_legacy_world_root: true,
    };
    let (result, spec) = ingest_scenario_from_str(name, &json, profile);
    assert_ne!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    spec.expect("spec")
}

#[test]
fn canonical_ingestion_structural_admission_reaches_driver() {
    let spec = ingest_corpus("minimal_scenario_root.simthing-scenario.json");
    let readiness = evaluate_scenario_compile_readiness(&spec);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("driver compile reached");
    assert!(matches!(admission, StructuralTheaterAdmission::Admit(_)));
    assert!(readiness.structural_n4_ready);
    assert!(readiness.mapping_plan_deferred);
}

#[test]
fn canonical_galaxymap_ingestion_can_compile_structural_n4_theater() {
    let spec = ingest_corpus("minimal_scenario_galaxymap.simthing-scenario.json");
    let readiness = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness.structural_n4_ready);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile");
    assert!(matches!(admission, StructuralTheaterAdmission::Admit(_)));
}

#[test]
fn oversized_canonical_ingestion_reports_atlas_partition_or_deferral() {
    let mut spec = ingest_corpus("minimal_scenario_galaxymap.simthing-scenario.json");
    spec.structural_grid.frame.width = 11;
    spec.structural_grid.frame.height = 11;
    let readiness = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness.structural_n4_deferred);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile");
    match admission {
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            assert!(matches!(
                reason,
                AtlasDeferralReason::FrameExceedsStandardMaxGrid { .. }
            ));
        }
        StructuralTheaterAdmission::Admit(_) => panic!("oversize must defer"),
    }
}

#[test]
fn mapping_plan_compile_readiness_is_reported_or_typed_deferred() {
    let spec = ingest_corpus("minimal_scenario_galaxymap.simthing-scenario.json");
    let readiness = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness.mapping_plan_deferred);
    assert!(!readiness.mapping_plan_ready);
    assert!(readiness.note.is_some());
}
