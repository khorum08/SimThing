//! STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 — Studio admission report presentation proofs.
//!
//! Input is constructed INLINE. The law under test — a legacy **World**-root
//! document is reported by the Studio as legacy-compatibility and never as
//! canonical — holds for ANY legacy World-root document, so the minimal one
//! below is a complete witness.
//!
//! This previously followed a pointer file into a shipped scenario. A scenario
//! is an ASSET; a Studio proof that reads one makes the asset a structural
//! requirement of the editor's test suite, which is how a disposable rehearsal
//! outlives the rehearsal. The Studio is DEV TELEMETRY over whatever is loaded
//! and must name no scenario at all.

use simthing_mapeditor::{studio_ingest_scenario_text_for_report, StudioScenarioAuthorityKind};
use simthing_spec::deserialize_scenario_authority;

/// Minimal legacy **World**-root document: a `World` root, the transitional
/// `scenario_id` sidecar, and one `Location` child named by
/// `structural_grid.map_container_id`. That is the whole admission surface —
/// the classification is a property of the ROOT SHAPE, not of the tree's size.
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
fn studio_legacy_world_root_report_is_legacy_compatibility() {
    let report =
        studio_ingest_scenario_text_for_report("legacy_world_root_minimal", MINIMAL_LEGACY_WORLD_ROOT);
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

    let spec =
        deserialize_scenario_authority(MINIMAL_LEGACY_WORLD_ROOT).expect("parse legacy world root");
    let document = simthing_mapeditor::build_studio_scenario_document(&spec).expect("legacy doc");
    assert_eq!(
        document.authority_kind,
        StudioScenarioAuthorityKind::LegacyWorldRoot
    );
}
