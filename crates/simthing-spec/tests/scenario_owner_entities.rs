//! SESSION-OWNER-ENTITIES-0 — Owner entities as direct GameSession children.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_scenario_metadata_to_root, deserialize_scenario_authority, game_session_child,
    game_session_owners, is_galaxy_map_entity, make_galaxy_map, make_owner_entity, owner_entity_id,
    scenario_metadata_seed, serialize_scenario_authority, validate_legacy_world_root_compatibility,
    validate_scenario_root_authority, validate_session_owner_entities, ScenarioRootError,
    ScenarioRootValidationMode, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SCENARIO_SCHEMA_VERSION,
};

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
const MIXED_PATTERN_SEED: u64 = 0x1234_5678_9ABC_DEF0;
const FIXTURE_SEED: u64 = 0x0001_2345_6789_ABCD;
const MINIMAL_OWNER_ID: &str = "minimal_owner";
const MINIMAL_GALAXY_ID: &str = "minimal_galaxy";

fn minimal_fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_root.simthing-scenario.json")
}

fn minimal_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0".into(),
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

#[test]
fn scenario_requires_at_least_one_owner_child() {
    let spec = minimal_scenario_spec();
    validate_session_owner_entities(&spec).expect("owner present");
    let owners = game_session_owners(&spec).expect("owners");
    assert_eq!(owners.len(), 1);
    assert_eq!(
        owner_entity_id(owners[0]).as_deref(),
        Some(MINIMAL_OWNER_ID)
    );
}

#[test]
fn scenario_owner_child_must_be_direct_gamesession_child() {
    let mut spec = minimal_scenario_spec();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession");
    let galaxy_idx = game_session
        .children
        .iter()
        .position(|child| is_galaxy_map_entity(child))
        .expect("galaxymap");
    game_session.children[galaxy_idx].add_child(make_owner_entity(
        "nested_under_galaxy",
        "Nested",
        "player",
    ));
    let err = validate_session_owner_entities(&spec).expect_err("owner not under gamesession");
    assert!(matches!(
        err,
        ScenarioRootError::OwnerNotDirectGameSessionChild
    ));
}

#[test]
fn scenario_owner_id_roundtrips() {
    let spec = minimal_scenario_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    let owners = game_session_owners(&round).expect("owners");
    assert_eq!(
        owner_entity_id(owners[0]).as_deref(),
        Some(MINIMAL_OWNER_ID)
    );
}

#[test]
fn scenario_non_owner_gamesession_child_does_not_count_as_owner() {
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
    validate_session_owner_entities(&spec).expect("owner still satisfies");
    assert_eq!(game_session_owners(&spec).expect("owners").len(), 1);
}

#[test]
fn scenario_owner_validation_preserves_gamesession_requirement() {
    let mut spec = minimal_scenario_spec();
    spec.root.children.clear();
    let err = validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect_err("missing gamesession");
    assert!(matches!(err, ScenarioRootError::MissingGameSessionChild));
}

#[test]
fn minimal_owner_fixture_deserializes() {
    let fixture = std::fs::read_to_string(minimal_fixture_path()).expect("corpus fixture");
    let loaded = deserialize_scenario_authority(&fixture).expect("fixture load");
    let owners = game_session_owners(&loaded).expect("owners");
    assert_eq!(
        owner_entity_id(owners[0]).as_deref(),
        Some(MINIMAL_OWNER_ID)
    );
    assert_eq!(scenario_metadata_seed(&loaded.root), Some(FIXTURE_SEED));
}

#[test]
fn scenario_owner_preserves_lossless_metadata_roundtrip() {
    let spec = minimal_scenario_spec();
    assert_eq!(scenario_metadata_seed(&spec.root), Some(MIXED_PATTERN_SEED));
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(
        scenario_metadata_seed(&round.root),
        Some(MIXED_PATTERN_SEED)
    );
    validate_session_owner_entities(&round).expect("owners preserved");
    game_session_child(&round).expect("gamesession preserved");
}

#[test]
fn direct_owner_under_gamesession_remains_valid() {
    validate_session_owner_entities(&minimal_scenario_spec()).expect("direct owner valid");
}

#[test]
fn non_owner_gamesession_child_remains_allowed() {
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
    validate_session_owner_entities(&spec).expect("non-owner child allowed");
}
