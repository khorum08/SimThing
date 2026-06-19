//! SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 — Scenario SimThing as canonical file root.
//! SCENARIO-METADATA-LOSSLESS-0 — lossless Scenario-root generator seed metadata.

use simthing_core::{PropertyValue, SimThing, SimThingKind};
use simthing_spec::{
    apply_scenario_metadata_to_root, deserialize_scenario_authority, scenario_metadata_seed,
    scenario_metadata_seed_value, scenario_metadata_string, scenario_metadata_u32,
    serialize_scenario_authority, sync_root_metadata_from_sidecar, sync_sidecar_from_root_metadata,
    validate_legacy_world_root_compatibility, validate_scenario_root_authority, ScenarioRootError,
    ScenarioRootValidationMode, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SCENARIO_GENERATOR_SEED_PROPERTY_ID,
    SCENARIO_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION, SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
    SCENARIO_SOURCE_LABEL_PROPERTY_ID,
};

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
/// JSON f64-safe non-zero seed for the on-disk corpus fixture (full `MIXED_PATTERN_SEED` tested in code).
const FIXTURE_SEED: u64 = 0x0001_2345_6789_ABCD;
const MIXED_PATTERN_SEED: u64 = 0x1234_5678_9ABC_DEF0;
const HIGH_BIT_PATTERN_SEED: u64 = 0x8000_0000_0000_0001;

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
    root.add_child(SimThing::new(SimThingKind::GameSession, 0));
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

fn scenario_with_seed(seed: u64) -> SimThingScenarioSpec {
    let mut spec = minimal_scenario_spec();
    spec.provenance.generator_seed = seed;
    spec.sync_root_metadata_from_sidecar();
    spec.sync_sidecar_from_root_metadata();
    spec
}

fn assert_seed_roundtrips(seed: u64) {
    let spec = scenario_with_seed(seed);
    assert_eq!(scenario_metadata_seed(&spec.root), Some(seed));
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let round = deserialize_scenario_authority(&json).expect("deserialize");
    assert_eq!(scenario_metadata_seed(&round.root), Some(seed));
    assert_eq!(round.provenance.generator_seed, seed);
}

#[test]
fn scenario_seed_roundtrips_zero() {
    assert_seed_roundtrips(0);
}

#[test]
fn scenario_seed_roundtrips_u64_max() {
    assert_seed_roundtrips(u64::MAX);
}

#[test]
fn scenario_seed_roundtrips_high_bit_pattern() {
    assert_seed_roundtrips(HIGH_BIT_PATTERN_SEED);
}

#[test]
fn scenario_seed_roundtrips_low_high_mixed_pattern() {
    assert_seed_roundtrips(MIXED_PATTERN_SEED);
}

#[test]
fn scenario_seed_rejects_malformed_length() {
    let mut spec = minimal_scenario_spec();
    spec.root.add_property(
        SCENARIO_GENERATOR_SEED_PROPERTY_ID,
        PropertyValue {
            data: vec![0.0, 0.0],
        },
    );
    assert_eq!(scenario_metadata_seed(&spec.root), None);
    let err = validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect_err("malformed length");
    assert!(matches!(
        err,
        ScenarioRootError::MissingScenarioMetadata("generator_seed")
    ));
}

#[test]
fn scenario_seed_rejects_fractional_chunks_if_chunk_encoding_is_used() {
    let mut spec = minimal_scenario_spec();
    spec.root.add_property(
        SCENARIO_GENERATOR_SEED_PROPERTY_ID,
        PropertyValue {
            data: vec![1.5, 0.0, 0.0, 0.0],
        },
    );
    assert_eq!(scenario_metadata_seed(&spec.root), None);
}

#[test]
fn scenario_seed_rejects_out_of_range_chunks_if_chunk_encoding_is_used() {
    let mut spec = minimal_scenario_spec();
    spec.root.add_property(
        SCENARIO_GENERATOR_SEED_PROPERTY_ID,
        PropertyValue {
            data: vec![65536.0, 0.0, 0.0, 0.0],
        },
    );
    assert_eq!(scenario_metadata_seed(&spec.root), None);
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
    assert_eq!(
        scenario_metadata_seed(&round.root),
        Some(MIXED_PATTERN_SEED)
    );
    assert_eq!(round.canonical_scenario_id(), MINIMAL_SCENARIO_ID);
    assert_eq!(round.provenance.generator_seed, MIXED_PATTERN_SEED);
}

#[test]
fn scenario_sidecar_sync_from_root_metadata_is_exact() {
    let spec = scenario_with_seed(u64::MAX);
    let mut synced = spec.clone();
    synced.provenance.generator_seed = 0;
    synced.sync_sidecar_from_root_metadata();
    assert_eq!(synced.provenance.generator_seed, u64::MAX);
}

#[test]
fn scenario_root_metadata_from_sidecar_is_exact() {
    let mut spec = minimal_scenario_spec();
    spec.provenance.generator_seed = HIGH_BIT_PATTERN_SEED;
    spec.sync_root_metadata_from_sidecar();
    assert_eq!(
        scenario_metadata_seed(&spec.root),
        Some(HIGH_BIT_PATTERN_SEED)
    );
    spec.sync_sidecar_from_root_metadata();
    assert_eq!(spec.provenance.generator_seed, HIGH_BIT_PATTERN_SEED);
}

#[test]
fn scenario_metadata_seed_value_uses_four_u16_chunks() {
    let value = scenario_metadata_seed_value(MIXED_PATTERN_SEED);
    assert_eq!(value.data.len(), 4);
    assert_eq!(value.data, vec![57072.0, 39612.0, 22136.0, 4660.0]);
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
    let fixture = std::fs::read_to_string(minimal_fixture_path()).expect("corpus fixture");
    let loaded = deserialize_scenario_authority(&fixture).expect("fixture load");
    assert_eq!(loaded.root.kind, SimThingKind::Scenario);
    assert_eq!(loaded.canonical_scenario_id(), MINIMAL_SCENARIO_ID);
    assert_eq!(scenario_metadata_seed(&loaded.root), Some(FIXTURE_SEED));
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
