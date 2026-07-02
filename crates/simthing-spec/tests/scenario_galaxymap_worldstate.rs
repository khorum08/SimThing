//! SESSION-GALAXYMAP-WORLDSTATE-0 — GalaxyMap / WorldStateMap under GameSession.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_scenario_metadata_to_root, deserialize_scenario_authority,
    game_session_child, game_session_galaxy_map, is_galaxy_map_entity, make_galaxy_map,
    make_owner_entity, scenario_metadata_seed, serialize_scenario_authority,
    structural_property_value_u32, validate_legacy_world_root_compatibility,
    validate_scenario_root_authority, validate_session_galaxy_map,
    validate_stead_mapping_consistency, ScenarioRootError, ScenarioRootValidationMode,
    SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_SCHEMA_VERSION, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
const GALAXYMAP_FIXTURE_ID: &str = "minimal_scenario_galaxymap";
const MIXED_PATTERN_SEED: u64 = 0x1234_5678_9ABC_DEF0;
const FIXTURE_SEED: u64 = 0x0001_2345_6789_ABCD;
const MINIMAL_OWNER_ID: &str = "minimal_owner";
const MINIMAL_GALAXY_ID: &str = "minimal_galaxy";

fn minimal_fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_root.simthing-scenario.json")
}

fn galaxymap_fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json")
}

fn minimal_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "SESSION-GALAXYMAP-WORLDSTATE-0".into(),
        generator_seed: MIXED_PATTERN_SEED,
        generator_shape: "minimal".into(),
        ..SimThingScenarioProvenance::default()
    };
    apply_scenario_metadata_to_root(
        &mut root,
        MINIMAL_SCENARIO_ID,
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    let mut owner = make_owner_entity(MINIMAL_OWNER_ID, "Minimal Owner", "player");
    owner.add_child(SimThing::new(
        SimThingKind::Custom("CapabilityTree".into()),
        0,
    ));
    game_session.add_child(owner);
    let galaxy_map = make_galaxy_map(MINIMAL_GALAXY_ID, "Minimal Galaxy");
    let map_raw = galaxy_map.id.raw().to_string();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);
    let mut spec = SimThingScenarioSpec {
        scenario_id: MINIMAL_SCENARIO_ID.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: map_raw,
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}

fn galaxymap_spatial_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "SESSION-GALAXYMAP-WORLDSTATE-0".into(),
        generator_seed: FIXTURE_SEED,
        generator_shape: "galaxymap".into(),
        ..SimThingScenarioProvenance::default()
    };
    apply_scenario_metadata_to_root(
        &mut root,
        GALAXYMAP_FIXTURE_ID,
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity(
        MINIMAL_OWNER_ID,
        "Minimal Owner",
        "player",
    ));
    let mut galaxy_map = make_galaxy_map("spatial_galaxy", "Spatial Galaxy");
    let map_raw = galaxy_map.id.raw();

    let mut cell_a = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell_a, GALAXY_GRIDCELL_ROLE_INERT);
    cell_a.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell_a.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell_a.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell_a.add_child(SimThing::new(SimThingKind::Cohort, 0));
    let cell_a_raw = cell_a.id.raw();

    let mut cell_b = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell_b, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    cell_b.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    cell_b.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    cell_b.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell_b.add_child(SimThing::new(SimThingKind::Cohort, 0));
    let cell_b_raw = cell_b.id.raw();

    galaxy_map.add_child(cell_a);
    galaxy_map.add_child(cell_b);
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    SimThingScenarioSpec {
        scenario_id: GALAXYMAP_FIXTURE_ID.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![
                SimThingStructuralGridPlacement {
                    location_id: "cell_a".into(),
                    target_id: "cell_a".into(),
                    system_id: 1,
                    row: 0,
                    col: 0,
                    simthing_id_raw: cell_a_raw,
                },
                SimThingStructuralGridPlacement {
                    location_id: "cell_b".into(),
                    target_id: "cell_b".into(),
                    system_id: 2,
                    row: 0,
                    col: 1,
                    simthing_id_raw: cell_b_raw,
                },
            ],
        },
        links: vec![SimThingScenarioLink {
            from_system_id: "1".into(),
            to_system_id: "2".into(),
        }],
        provenance,
    }
}

#[test]
fn scenario_requires_exactly_one_galaxymap_child() {
    let spec = minimal_scenario_spec();
    validate_session_galaxy_map(&spec).expect("one galaxymap");
    let galaxy = game_session_galaxy_map(&spec).expect("galaxy");
    assert!(is_galaxy_map_entity(galaxy));
}

#[test]
fn scenario_world_child_does_not_count_as_canonical_galaxymap() {
    let mut spec = minimal_scenario_spec();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession");
    game_session
        .children
        .retain(|child| !is_galaxy_map_entity(child));
    game_session.add_child(SimThing::new(SimThingKind::World, 0));
    let err = validate_session_galaxy_map(&spec).expect_err("world is not galaxymap");
    assert!(matches!(err, ScenarioRootError::MissingGalaxyMap));
}

#[test]
fn scenario_galaxymap_must_be_direct_gamesession_child() {
    let mut spec = minimal_scenario_spec();
    let galaxy = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession")
        .children
        .iter_mut()
        .find(|child| is_galaxy_map_entity(child))
        .expect("galaxymap")
        .clone();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession");
    game_session
        .children
        .retain(|child| !is_galaxy_map_entity(child));
    game_session.children[0].add_child(galaxy);
    let err = validate_session_galaxy_map(&spec).expect_err("nested galaxymap");
    assert!(matches!(
        err,
        ScenarioRootError::GalaxyMapNotDirectGameSessionChild | ScenarioRootError::MissingGalaxyMap
    ));
}

#[test]
fn scenario_galaxymap_id_roundtrips() {
    let spec = minimal_scenario_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    let galaxy = game_session_galaxy_map(&round).expect("galaxy");
    assert!(is_galaxy_map_entity(galaxy));
    validate_session_galaxy_map(&round).expect("valid");
}

#[test]
fn scenario_galaxymap_fixture_deserializes() {
    let fixture = std::fs::read_to_string(minimal_fixture_path()).expect("minimal corpus");
    let loaded = deserialize_scenario_authority(&fixture).expect("minimal load");
    validate_session_galaxy_map(&loaded).expect("minimal validates");

    let spatial_fixture =
        std::fs::read_to_string(galaxymap_fixture_path()).expect("galaxymap corpus");
    let spatial = deserialize_scenario_authority(&spatial_fixture).expect("spatial load");
    validate_session_galaxy_map(&spatial).expect("spatial validates");
    validate_stead_mapping_consistency(&spatial).expect("stead accepts galaxymap root");
}

#[test]
fn scenario_galaxymap_preserves_owner_and_lossless_metadata() {
    let spec = minimal_scenario_spec();
    assert_eq!(scenario_metadata_seed(&spec.root), Some(MIXED_PATTERN_SEED));
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(
        scenario_metadata_seed(&round.root),
        Some(MIXED_PATTERN_SEED)
    );
    validate_session_galaxy_map(&round).expect("galaxy preserved");
    game_session_child(&round).expect("gamesession preserved");
}

#[test]
fn scenario_stead_mapping_accepts_galaxymap_as_spatial_root() {
    let spec = galaxymap_spatial_scenario_spec();
    validate_stead_mapping_consistency(&spec).expect("galaxymap spatial root");
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("canonical");
}

#[test]
fn future_non_owner_children_under_gamesession_remain_allowed() {
    let mut spec = minimal_scenario_spec();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession");
    game_session.add_child(SimThing::new(
        SimThingKind::Custom("FutureSessionChild".into()),
        0,
    ));
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("future child allowed");
}

fn minimal_corpus_scenario_spec() -> SimThingScenarioSpec {
    let mut spec = minimal_scenario_spec();
    spec.provenance.generator_seed = FIXTURE_SEED;
    spec.sync_root_metadata_from_sidecar();
    spec
}

#[test]
#[ignore = "run once to refresh corpus fixtures"]
fn generate_corpus_fixtures() {
    let minimal = serialize_scenario_authority(&minimal_corpus_scenario_spec()).expect("minimal");
    std::fs::write(minimal_fixture_path(), minimal).expect("write minimal");
    let spatial =
        serialize_scenario_authority(&galaxymap_spatial_scenario_spec()).expect("spatial");
    std::fs::write(galaxymap_fixture_path(), spatial).expect("write galaxymap");
}
