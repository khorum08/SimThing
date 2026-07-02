//! SCENARIO-STEAD-MAP-ROUNDTRIP-0 — STEAD map roundtrip proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simthing_spec::{
    evaluate_planet_child_locations, evaluate_scenario_stead_map_roundtrip_from_json_str,
    load_scenario_spec_from_json_str, SpecError,
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

fn evaluate_fixture() -> simthing_spec::ScenarioSteadMapRoundtripReport {
    let json = load_owner_silo_fixture_json();
    evaluate_scenario_stead_map_roundtrip_from_json_str("owner_silo_corpus", &json)
        .expect("evaluate")
}

#[test]
fn scenario_stead_map_roundtrip_preserves_authority_digest() {
    let report = evaluate_fixture();
    assert!(report.digest_stable);
    assert_eq!(
        report.initial_authority_digest,
        report.canonical_roundtrip_digest
    );
}

#[test]
fn scenario_stead_map_roundtrip_preserves_stable_ids() {
    let report = evaluate_fixture();
    assert!(report.stead_ids_stable);
    assert!(!report.stead_id_rows_before.is_empty());
    assert_eq!(report.stead_id_rows_before, report.stead_id_rows_after);
}

#[test]
fn scenario_stead_map_roundtrip_preserves_links() {
    let report = evaluate_fixture();
    assert!(report.links_stable);
    assert_eq!(report.link_rows_before, report.link_rows_after);
}

#[test]
fn scenario_stead_map_roundtrip_preserves_spatial_tree_shape() {
    let report = evaluate_fixture();
    assert!(report.spatial_tree_stable);
    assert!(!report.spatial_tree_rows_before.is_empty());
    assert_eq!(
        report.spatial_tree_rows_before,
        report.spatial_tree_rows_after
    );
}

#[test]
fn scenario_stead_map_roundtrip_preserves_owner_metadata_without_spatial_parentage() {
    let report = evaluate_fixture();
    assert!(report.owner_metadata_not_spatial_parentage);
    assert!(report
        .stead_id_rows_before
        .iter()
        .any(|row| row.kind == "Owner"));
}

#[test]
fn scenario_stead_map_roundtrip_preserves_rf_metadata() {
    let report = evaluate_fixture();
    assert!(report.rf_metadata_stable);
    assert!(!report.rf_metadata_rows_before.is_empty());
    assert_eq!(
        report.rf_metadata_rows_before,
        report.rf_metadata_rows_after
    );
}

#[test]
fn scenario_stead_map_roundtrip_exposes_recursive_rf_parent_location_prerequisites() {
    let report = evaluate_fixture();
    assert!(report.local_rf_parent_node_resolution_prerequisites_present);
    assert!(report
        .spatial_tree_rows_before
        .iter()
        .any(|row| row.has_interior_grid));
}

#[test]
fn scenario_stead_map_roundtrip_rebuilds_studio_projection_inputs() {
    let report = evaluate_fixture();
    assert!(report.studio_projection_rebuild_ready);
}

#[test]
fn normal_tests_do_not_write_scenario_stead_map_fixtures() {
    let path = corpus_path(OWNER_SILO_FIXTURE);
    if !path.exists() {
        return;
    }
    let _ = evaluate_fixture();
    let mtime = fs::metadata(&path)
        .and_then(|m| m.modified())
        .expect("mtime");
    let age = SystemTime::now()
        .duration_since(mtime)
        .unwrap_or(Duration::from_secs(0));
    assert!(
        age.as_secs() > 5,
        "corpus fixture must not be rewritten during normal tests"
    );
}

#[test]
fn scenario_stead_map_roundtrip_preserves_surface_gridcell_tier() {
    let json = load_owner_silo_fixture_json();
    let (spec, _) = load_scenario_spec_from_json_str("owner_silo", &json).expect("load");
    let planet = evaluate_planet_child_locations(&spec);
    assert!(planet.surface_gridcell_tier_present);
    assert!(planet.gameplay_child_under_surface_count >= 1);
}
