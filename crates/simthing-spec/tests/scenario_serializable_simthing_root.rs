//! SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 — Scenario SimThing as canonical file root.

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_scenario_metadata_to_root, deserialize_scenario_authority, scenario_metadata_seed,
    scenario_metadata_string, scenario_metadata_u32, serialize_scenario_authority,
    validate_legacy_world_root_compatibility, validate_scenario_root_authority, ScenarioRootError,
    ScenarioRootValidationMode, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SCENARIO_ID_PROPERTY_ID,
    SCENARIO_SCHEMA_VERSION, SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
    SCENARIO_SOURCE_LABEL_PROPERTY_ID,
};

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
const MINIMAL_FIXTURE_PATH: &str = "scenarios/corpus/minimal_scenario_root.simthing-scenario.json";

fn minimal_scenario_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0".into(),
        generator_seed: 0,
        generator_shape: "minimal".into(),
    };
    apply_scenario_metadata_to_root(
        &mut root,
        MINIMAL_SCENARIO_ID,
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );
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
fn scenario_root_roundtrips_metadata_properties() {
    let scenario = minimal_scenario_spec();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(
        scenario_metadata_string(&round.root, SCENARIO_ID_PROPERTY_ID).as_deref(),
        Some(MINIMAL_SCENARIO_ID)
    );
    assert_eq!(
        scenario_metadata_u32(&round.root, SCENARIO_SCHEMA_VERSION_PROPERTY_ID),
        Some(SCENARIO_SCHEMA_VERSION)
    );
    assert_eq!(
        scenario_metadata_string(&round.root, SCENARIO_SOURCE_LABEL_PROPERTY_ID).as_deref(),
        Some("SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0")
    );
    assert_eq!(scenario_metadata_seed(&round.root), Some(0));
    assert_eq!(round.canonical_scenario_id(), MINIMAL_SCENARIO_ID);
}

#[test]
fn scenario_root_is_canonical_authority() {
    let scenario = minimal_scenario_spec();
    assert_eq!(scenario.root.kind, SimThingKind::Scenario);
    validate_scenario_root_authority(&scenario, ScenarioRootValidationMode::Canonical)
        .expect("canonical root");
}

#[test]
fn scenario_root_fixture_deserializes_from_corpus() {
    let fixture = std::fs::read_to_string(MINIMAL_FIXTURE_PATH).expect("corpus fixture");
    let loaded = deserialize_scenario_authority(&fixture).expect("fixture load");
    assert_eq!(loaded.root.kind, SimThingKind::Scenario);
    assert_eq!(loaded.canonical_scenario_id(), MINIMAL_SCENARIO_ID);
}

#[test]
fn legacy_world_root_fixture_uses_explicit_compatibility_path() {
    let legacy_json = include_str!(
        "../../simthing-mapeditor/tests/fixtures/terran_pirate_skeleton.simthing-scenario.json"
    );
    let loaded = deserialize_scenario_authority(legacy_json).expect("legacy terran pirate");
    assert_eq!(loaded.root.kind, SimThingKind::World);
    validate_legacy_world_root_compatibility(&loaded).expect("legacy World-root admitted");
    assert!(
        validate_scenario_root_authority(&loaded, ScenarioRootValidationMode::Canonical).is_err()
    );
}

#[test]
fn scenario_sidecar_metadata_is_transitional_not_authority() {
    let mut scenario = minimal_scenario_spec();
    scenario.scenario_id = "stale_sidecar".to_string();
    let err = validate_scenario_root_authority(&scenario, ScenarioRootValidationMode::Canonical)
        .expect_err("sidecar mismatch");
    assert!(matches!(
        err,
        ScenarioRootError::ScenarioMetadataMismatch { .. }
    ));
}

#[test]
fn scenario_root_rejects_missing_metadata_if_canonical_mode_requires_it() {
    let mut scenario = minimal_scenario_spec();
    scenario.root.properties.remove(&SCENARIO_ID_PROPERTY_ID);
    let err = validate_scenario_root_authority(&scenario, ScenarioRootValidationMode::Canonical)
        .expect_err("missing metadata");
    assert!(matches!(
        err,
        ScenarioRootError::MissingScenarioMetadata("scenario_id")
    ));
}

#[test]
fn scenario_root_does_not_accept_arbitrary_non_scenario_root() {
    let mut scenario = minimal_scenario_spec();
    scenario.root.kind = SimThingKind::Location;
    let err = validate_scenario_root_authority(&scenario, ScenarioRootValidationMode::Canonical)
        .expect_err("not scenario");
    assert!(matches!(err, ScenarioRootError::RootIsNotScenario));
    let json = serialize_scenario_authority(&scenario).expect("serialize transitional");
    let err = deserialize_scenario_authority(&json).expect_err("reject location root");
    assert!(matches!(
        err,
        simthing_spec::ScenarioSerdeError::RootValidation(
            ScenarioRootError::ArbitraryRootKind { .. }
        )
    ));
}

#[test]
fn canonical_serialize_prefers_scenario_kind_in_json() {
    let scenario = minimal_scenario_spec();
    let json = serialize_scenario_authority(&scenario).expect("serialize");
    assert!(json.contains("\"kind\":\"Scenario\""));
}
