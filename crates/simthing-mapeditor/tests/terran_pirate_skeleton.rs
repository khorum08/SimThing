//! TERRAN-PIRATE-SCENARIO-SKELETON-0 — horizon skeleton authority and Studio projection proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    terran_pirate_skeleton_scenario_spec, TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE,
    TERRAN_PIRATE_SKELETON_SCENARIO_ID,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, validate_scenario_links,
    validate_stead_mapping_consistency, SimThingScenarioSpec, SCENARIO_RENDER_WORLD_X_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_Y_PROPERTY_ID, SCENARIO_RENDER_WORLD_Z_PROPERTY_ID,
};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/terran_pirate_skeleton.simthing-scenario.json")
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

fn semantic_skeleton_equivalent(left: &SimThingScenarioSpec, right: &SimThingScenarioSpec) -> bool {
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

fn builder_seed() -> SimThingScenarioSpec {
    terran_pirate_skeleton_scenario_spec()
}

#[test]
fn terran_pirate_skeleton_scenario_is_valid_simthing_scenario_spec() {
    let scenario = builder_seed();
    validate_stead_mapping_consistency(&scenario).expect("STEAD valid");
    validate_scenario_links(&scenario).expect("links valid");
    assert_eq!(scenario.scenario_id, TERRAN_PIRATE_SKELETON_SCENARIO_ID);
    assert_eq!(
        scenario.provenance.source,
        TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE
    );
}

#[test]
fn terran_pirate_skeleton_preserves_structural_grid() {
    let scenario = builder_seed();
    assert_eq!(scenario.structural_grid.frame.occupied_cells, 4);
    assert_eq!(scenario.structural_grid.placements.len(), 4);
}

#[test]
fn terran_pirate_skeleton_links_are_canonical() {
    let scenario = builder_seed();
    assert_eq!(scenario.links.len(), 3);
    validate_scenario_links(&scenario).expect("canonical links");
}

#[test]
fn terran_pirate_skeleton_has_gridcell_children() {
    let scenario = builder_seed();
    assert_eq!(scenario.gridcell_locations().count(), 4);
    assert!(scenario
        .gridcell_locations()
        .all(|cell| !cell.children.is_empty()));
}

#[test]
fn terran_pirate_skeleton_has_no_render_authority() {
    assert_no_render_authority(&builder_seed());
}

#[test]
fn terran_pirate_skeleton_serializes_and_deserializes() {
    let scenario = builder_seed();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.scenario_id, scenario.scenario_id);
    assert_eq!(round.links, scenario.links);
}

#[test]
fn terran_pirate_skeleton_fixture_matches_builder_semantics() {
    if !fixture_path().is_file() {
        eprintln!("fixture missing; builder-only proof");
        return;
    }
    let built = builder_seed();
    let loaded = load_scenario_authority_from_path(&fixture_path()).expect("load fixture");
    assert!(semantic_skeleton_equivalent(&built, &loaded));
}

#[test]
fn terran_pirate_skeleton_loads_through_studio_scenario_io() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir
        .path()
        .join("terran_pirate_skeleton.simthing-scenario.json");
    let scenario = builder_seed();
    save_scenario_to_path(&scenario, &path);
    let loaded = load_scenario_authority_from_path(&path).expect("load");
    assert_eq!(loaded.scenario_id, TERRAN_PIRATE_SKELETON_SCENARIO_ID);
}

#[test]
fn terran_pirate_skeleton_rebuilds_hydration_boundary() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir
        .path()
        .join("terran_pirate_skeleton.simthing-scenario.json");
    let scenario = builder_seed();
    save_scenario_to_path(&scenario, &path);
    let session = load_studio_session_from_scenario_path(&path, None).expect("session");
    assert_eq!(session.hydration.grid.occupied_cells, 4);
}

#[test]
fn terran_pirate_skeleton_rebuilds_view_model() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let path = dir
        .path()
        .join("terran_pirate_skeleton.simthing-scenario.json");
    let scenario = builder_seed();
    save_scenario_to_path(&scenario, &path);
    let session = load_studio_session_from_scenario_path(&path, None).expect("session");
    assert_eq!(session.view_model.stars.len(), 4);
    assert_eq!(session.view_model.hyperlanes.len(), 3);
}

fn save_scenario_to_path(scenario: &SimThingScenarioSpec, path: &std::path::Path) {
    let json = serialize_scenario_authority(scenario).expect("serialize");
    fs::write(path, json).expect("write");
}

#[test]
#[ignore = "run once to refresh fixture JSON from builder"]
fn write_terran_pirate_skeleton_fixture() {
    save_scenario_to_path(&builder_seed(), &fixture_path());
}
