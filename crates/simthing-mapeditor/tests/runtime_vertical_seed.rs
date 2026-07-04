//! VERTICAL-TEST-SCENARIO-SEED-0 — load/projection/GPU proof for runtime vertical seed fixture.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    build_gpu_structural_upload_packet_from_scenario, build_structural_projection,
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    prove_gpu_buffer_residency_blocking, prove_gpu_structural_validation_blocking,
    runtime_vertical_seed_scenario_spec, StudioSessionSource,
    RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE, RUNTIME_VERTICAL_SEED_SCENARIO_ID,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, validate_scenario_links,
    validate_stead_mapping_consistency, SimThingScenarioSpec, SCENARIO_RENDER_WORLD_X_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_Y_PROPERTY_ID, SCENARIO_RENDER_WORLD_Z_PROPERTY_ID,
};
use tempfile::TempDir;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/runtime_vertical_seed.simthing-scenario.json")
}

fn assert_no_render_authority(scenario: &SimThingScenarioSpec) {
    fn walk(node: &SimThing) {
        for property_id in node.properties.keys() {
            assert!(
                !matches!(
                    property_id.0,
                    id if id == SCENARIO_RENDER_WORLD_X_PROPERTY_ID.0
                        || id == SCENARIO_RENDER_WORLD_Y_PROPERTY_ID.0
                        || id == SCENARIO_RENDER_WORLD_Z_PROPERTY_ID.0
                ),
                "render authority property present on SimThing {}",
                node.id.raw()
            );
        }
        for child in &node.children {
            walk(child);
        }
    }
    walk(&scenario.root);
}

fn semantic_seed_equivalent(left: &SimThingScenarioSpec, right: &SimThingScenarioSpec) -> bool {
    if left.scenario_id != right.scenario_id
        || left.structural_grid.frame != right.structural_grid.frame
        || left.links != right.links
        || left.provenance != right.provenance
    {
        return false;
    }
    if left.structural_grid.placements.len() != right.structural_grid.placements.len() {
        return false;
    }
    for (a, b) in left
        .structural_grid
        .placements
        .iter()
        .zip(right.structural_grid.placements.iter())
    {
        if a.location_id != b.location_id
            || a.target_id != b.target_id
            || a.system_id != b.system_id
            || a.row != b.row
            || a.col != b.col
        {
            return false;
        }
    }
    left.root.subtree_size() == right.root.subtree_size()
        && left.gridcell_locations().count() == right.gridcell_locations().count()
        && left
            .gridcell_locations()
            .zip(right.gridcell_locations())
            .all(|(a, b)| a.kind == b.kind && !a.children.is_empty() && !b.children.is_empty())
}

fn loaded_fixture() -> SimThingScenarioSpec {
    load_scenario_authority_from_path(&fixture_path()).expect("fixture loads")
}

fn builder_seed() -> SimThingScenarioSpec {
    runtime_vertical_seed_scenario_spec()
}

// --- Part G: scenario/spec tests ---

// --- Part G: Studio load/projection tests ---

// --- Part G: GPU structural tests ---

// --- Part G: lifecycle/doc tests ---

#[test]
fn runtime_vertical_seed_save_load_roundtrip_preserves_projection() {
    let scenario = builder_seed();
    let projection = build_structural_projection(&scenario).expect("projection");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("seed.simthing-scenario.json");
    simthing_mapeditor::save_scenario_authority_to_path(&path, &scenario).expect("save");
    let session = load_studio_session_from_scenario_path(&path, None).expect("load");
    assert_eq!(session.structural_projection, projection);
    let known: BTreeSet<u32> = scenario
        .gridcell_locations()
        .map(|cell| cell.id.raw())
        .collect();
    let spawned = SimThing::new(SimThingKind::Cohort, 0);
    assert!(!known.contains(&spawned.id.raw()));
}
