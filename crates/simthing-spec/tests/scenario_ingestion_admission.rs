//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion admission tests.
//!
//! Input is constructed INLINE. The admission law under test — a legacy
//! **World**-root document is admitted as legacy-compatibility and never as
//! canonical — holds for ANY legacy World-root document, so the minimal one
//! below is a complete witness. Reading a shipped scenario here would state
//! engine law in terms of one corpus's contents (Corpus Boundary Law) and
//! would couple `simthing-spec` proofs to an asset outside the crate.

use simthing_spec::{
    ingest_scenario_from_str, ScenarioDeferralKind, ScenarioIngestionClassification,
    ScenarioIngestionProfile,
};

const CANONICAL_PROFILE: ScenarioIngestionProfile = ScenarioIngestionProfile {
    require_canonical_tree: true,
    admit_legacy_world_root: true,
};

/// Minimal legacy **World**-root document: a `World` root, the transitional
/// `scenario_id` sidecar (`validate_legacy_world_root_compatibility`), and a
/// single `Location` child named by `structural_grid.map_container_id`
/// (`resolve_map_container`). That is the entire admission surface — one cell
/// is as much a witness as ten thousand, because the classification is a
/// property of the ROOT SHAPE, not of the tree's size or contents.
const MINIMAL_LEGACY_WORLD_ROOT: &str = r#"{
  "scenario_id": "legacy_world_root_minimal",
  "root": {
    "id": 1,
    "kind": "World",
    "properties": [],
    "overlays": [],
    "children": [
      {
        "id": 2,
        "kind": "Location",
        "properties": [],
        "overlays": [],
        "children": [],
        "spawned_day": 0
      }
    ],
    "spawned_day": 0
  },
  "structural_grid": {
    "frame": { "width": 1, "height": 1, "occupied_cells": 0 },
    "map_container_id": "2",
    "placements": []
  }
}"#;

#[test]
fn classifies_legacy_world_root_as_legacy_compatibility_not_canonical() {
    let (result, _) = ingest_scenario_from_str(
        "legacy_world_root_minimal",
        MINIMAL_LEGACY_WORLD_ROOT,
        CANONICAL_PROFILE,
    );
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
