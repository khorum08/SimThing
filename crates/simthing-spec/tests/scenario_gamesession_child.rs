//! SCENARIO-GAMESESSION-CHILD-0 — canonical Scenario root requires exactly one GameSession child.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_scenario_metadata_to_root, deserialize_scenario_authority, game_session_child,
    make_owner_entity, scenario_metadata_seed, serialize_scenario_authority,
    validate_legacy_world_root_compatibility, validate_scenario_game_session_child,
    validate_scenario_root_authority, ScenarioRootError, ScenarioRootValidationMode,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SCENARIO_SCHEMA_VERSION,
};

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
const MIXED_PATTERN_SEED: u64 = 0x1234_5678_9ABC_DEF0;
const FIXTURE_SEED: u64 = 0x0001_2345_6789_ABCD;

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
    };
    apply_scenario_metadata_to_root(
        &mut root,
        MINIMAL_SCENARIO_ID,
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity(
        "minimal_owner",
        "Minimal Owner",
        "player",
    ));
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
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}

#[test]
fn scenario_requires_exactly_one_gamesession_child() {
    let spec = minimal_scenario_spec();
    validate_scenario_game_session_child(&spec).expect("one GameSession");
    let child = game_session_child(&spec).expect("resolve");
    assert_eq!(child.kind, SimThingKind::GameSession);
    assert_eq!(spec.root.children.len(), 1);
}

#[test]
fn scenario_missing_gamesession_child_is_rejected() {
    let mut spec = minimal_scenario_spec();
    spec.root.children.clear();
    let err = validate_scenario_game_session_child(&spec).expect_err("missing");
    assert!(matches!(err, ScenarioRootError::MissingGameSessionChild));
    let err = validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect_err("canonical reject");
    assert!(matches!(err, ScenarioRootError::MissingGameSessionChild));
}

#[test]
fn scenario_multiple_gamesession_children_are_rejected() {
    let mut spec = minimal_scenario_spec();
    spec.root
        .add_child(SimThing::new(SimThingKind::GameSession, 0));
    let err = validate_scenario_game_session_child(&spec).expect_err("multiple");
    assert!(matches!(
        err,
        ScenarioRootError::MultipleGameSessionChildren { count: 2 }
    ));
}

#[test]
fn scenario_world_child_does_not_count_as_gamesession() {
    let mut spec = minimal_scenario_spec();
    spec.root.children.clear();
    spec.root.add_child(SimThing::new(SimThingKind::World, 0));
    let err = validate_scenario_game_session_child(&spec).expect_err("world not gamesession");
    assert!(matches!(
        err,
        ScenarioRootError::GameSessionChildWrongKind { .. }
    ));
}

#[test]
fn scenario_gamesession_child_roundtrips() {
    let spec = minimal_scenario_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(round.root.kind, SimThingKind::Scenario);
    assert_eq!(round.root.children.len(), 1);
    assert_eq!(round.root.children[0].kind, SimThingKind::GameSession);
    game_session_child(&round).expect("GameSession after roundtrip");
}

#[test]
fn scenario_gamesession_preserves_lossless_metadata_roundtrip() {
    let spec = minimal_scenario_spec();
    assert_eq!(scenario_metadata_seed(&spec.root), Some(MIXED_PATTERN_SEED));
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(
        scenario_metadata_seed(&round.root),
        Some(MIXED_PATTERN_SEED)
    );
    assert_eq!(round.provenance.generator_seed, MIXED_PATTERN_SEED);
    game_session_child(&round).expect("GameSession preserved");
}

#[test]
fn legacy_world_root_compatibility_does_not_satisfy_canonical_gamesession_validation() {
    let legacy_json = include_str!(
        "../../simthing-mapeditor/tests/fixtures/terran_pirate_skeleton.simthing-scenario.json"
    );
    let loaded = deserialize_scenario_authority(legacy_json).expect("legacy terran pirate");
    validate_legacy_world_root_compatibility(&loaded).expect("legacy admitted");
    let err = validate_scenario_game_session_child(&loaded).expect_err("no GameSession on World");
    assert!(matches!(
        err,
        ScenarioRootError::LegacyWorldRootHasNoGameSessionRequirement
    ));
}

#[test]
fn minimal_gamesession_fixture_deserializes() {
    let fixture = std::fs::read_to_string(minimal_fixture_path()).expect("corpus fixture");
    let loaded = deserialize_scenario_authority(&fixture).expect("fixture load");
    assert_eq!(loaded.root.kind, SimThingKind::Scenario);
    let gs = game_session_child(&loaded).expect("fixture GameSession");
    assert_eq!(gs.kind, SimThingKind::GameSession);
    assert_eq!(scenario_metadata_seed(&loaded.root), Some(FIXTURE_SEED));
}

#[test]
fn arbitrary_non_scenario_root_still_rejected() {
    let mut spec = minimal_scenario_spec();
    spec.root.kind = SimThingKind::Location;
    let err = validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect_err("not scenario");
    assert!(matches!(err, ScenarioRootError::RootIsNotScenario));
    let json = serialize_scenario_authority(&spec).expect("serialize transitional");
    let err = deserialize_scenario_authority(&json).expect_err("reject location root");
    assert!(matches!(
        err,
        simthing_spec::ScenarioSerdeError::RootValidation(
            ScenarioRootError::ArbitraryRootKind { .. }
        )
    ));
}
