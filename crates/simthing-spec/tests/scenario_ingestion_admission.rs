//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion admission tests.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_scenario_metadata_to_root, ingest_scenario,
    ingest_scenario_from_str, make_galaxy_map, make_owner_entity, scenario_metadata_seed,
    serialize_scenario_authority, structural_property_value_u32, ScenarioDeferralKind,
    ScenarioIngestionClassification, ScenarioIngestionProfile, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

const CANONICAL_PROFILE: ScenarioIngestionProfile = ScenarioIngestionProfile {
    require_canonical_tree: true,
    admit_legacy_world_root: true,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_corpus(name: &str) -> String {
    fs::read_to_string(corpus_path(name)).unwrap_or_else(|_| panic!("missing corpus {name}"))
}

fn base_canonical_spec(scenario_id: &str) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "GENERAL-SCENARIO-INGESTION-ADMISSION-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "ingestion".into(),
    };
    apply_scenario_metadata_to_root(&mut root, scenario_id, &provenance, SCENARIO_SCHEMA_VERSION);
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(make_owner_entity("owner_a", "Owner A", "player"));
    let galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");
    let map_raw = galaxy_map.id.raw().to_string();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);
    let mut spec = SimThingScenarioSpec {
        scenario_id: scenario_id.to_string(),
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

fn galaxymap_with_gridcells() -> SimThingScenarioSpec {
    let mut spec = base_canonical_spec("ingestion_galaxymap_builder");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");

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
    let map_raw = galaxy_map.id.raw();
    spec.structural_grid = SimThingScenarioGrid {
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
    };
    spec
}

#[test]
fn ingests_minimal_scenario_root_as_admitted_or_partially_admitted() {
    let json = load_corpus("minimal_scenario_root.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("minimal_scenario_root", &json, CANONICAL_PROFILE);
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
    assert!(result.validation.canonical_validation_ok);
    assert!(result.canonical_tree.has_game_session);
    assert_eq!(result.owner_admission.owner_count, 1);
}

#[test]
fn ingests_minimal_galaxymap_as_admitted() {
    let json = load_corpus("minimal_scenario_galaxymap.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("minimal_galaxymap", &json, CANONICAL_PROFILE);
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
    assert_eq!(result.canonical_tree.gridcell_count, 2);
    assert_eq!(result.galaxy_map_admission.gridcell_inert_count, 1);
    assert_eq!(result.galaxy_map_admission.gridcell_star_system_count, 1);
    assert!(result.owner_silo.is_some());
}

#[test]
fn rejects_missing_gamesession() {
    let json = load_corpus("invalid_missing_gamesession.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("missing_gs", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.gamesession_ok);
}

#[test]
fn rejects_missing_owner() {
    let json = load_corpus("invalid_missing_owner.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("missing_owner", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.owners_ok);
}

#[test]
fn rejects_duplicate_owner_ids() {
    let json = load_corpus("invalid_duplicate_owner_ids.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("dup_owners", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(result
        .errors
        .iter()
        .any(|e| e.message.contains("duplicate owner_id")));
}

#[test]
fn rejects_missing_galaxymap() {
    let json = load_corpus("invalid_missing_galaxymap.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("missing_map", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.galaxy_map_ok);
}

#[test]
fn rejects_bad_map_container() {
    let json = load_corpus("invalid_bad_map_container.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("bad_map_container", &json, CANONICAL_PROFILE);
    assert_eq!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(!result.validation.stead_mapping_ok);
}

#[test]
fn classifies_planet_child_as_unsupported_not_rejected_if_otherwise_valid() {
    let json = load_corpus("unsupported_planet_child_valid_schema.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("planet_child", &json, CANONICAL_PROFILE);
    assert_ne!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(result
        .deferrals
        .iter()
        .any(|d| { d.kind == ScenarioDeferralKind::UnsupportedChildLocationRole }));
}

#[test]
fn classifies_unknown_gridcell_role_as_unsupported_or_partially_admitted() {
    let json = load_corpus("unsupported_unknown_gridcell_role.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("unknown_role", &json, CANONICAL_PROFILE);
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::PartiallyAdmitted
            | ScenarioIngestionClassification::Unsupported
    ));
    assert!(result
        .deferrals
        .iter()
        .any(|d| { d.kind == ScenarioDeferralKind::UnsupportedGridcellRole }));
}

#[test]
fn classifies_legacy_terran_pirate_as_legacy_compatibility_not_canonical() {
    let reference = load_corpus("legacy_world_root_terran_pirate_reference.txt");
    let path = PathBuf::from(reference.trim());
    let json = fs::read_to_string(&path).expect("terran pirate path");
    let (result, _) = ingest_scenario_from_str("terran_pirate", &json, CANONICAL_PROFILE);
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

#[test]
fn ingestion_result_contains_typed_deferrals() {
    let mut spec = base_canonical_spec("typed_deferrals");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let owner = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Owner)
        .expect("owner");
    owner.add_child(SimThing::new(
        SimThingKind::Custom("CapabilityTree".into()),
        0,
    ));
    let result = ingest_scenario("typed_deferrals", &spec, CANONICAL_PROFILE);
    assert!(result.deferrals.iter().any(|d| {
        d.kind == ScenarioDeferralKind::CapabilityTreeNotYetExecuted
            && d.simthing_id_raw.is_some()
            && !d.reason.is_empty()
    }));
}

#[test]
fn ingestion_result_preserves_lossless_seed_metadata() {
    let json = load_corpus("minimal_scenario_root.simthing-scenario.json");
    let (result, spec) = ingest_scenario_from_str("seed_meta", &json, CANONICAL_PROFILE);
    let spec = spec.expect("spec");
    assert!(result.validation.seed_metadata_ok);
    let seed = scenario_metadata_seed(&spec.root).expect("root seed");
    assert_eq!(seed, spec.provenance.generator_seed);
    assert_eq!(seed, 0x0001_2345_6789_ABCD);
}

#[test]
#[ignore = "run once to refresh corpus invalid/unsupported fixtures"]
fn write_ingestion_corpus_fixtures() {
    write_missing_gamesession();
    write_missing_owner();
    write_duplicate_owners();
    write_missing_galaxymap();
    write_bad_map_container();
    write_planet_child();
    write_unknown_gridcell_role();
    write_terran_pirate_reference();
}

fn write_json(spec: &SimThingScenarioSpec, name: &str) {
    let json = serialize_scenario_authority(spec).expect("serialize");
    fs::write(corpus_path(name), json).expect("write");
}

fn write_missing_gamesession() {
    let mut spec = base_canonical_spec("invalid_missing_gamesession");
    let game_session = spec.root.children.pop().expect("gs");
    let owner = game_session
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Owner)
        .expect("owner")
        .clone();
    let galaxy_map = game_session
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map")
        .clone();
    spec.root.add_child(owner);
    spec.root.add_child(galaxy_map);
    write_json(&spec, "invalid_missing_gamesession.simthing-scenario.json");
}

fn write_missing_owner() {
    let mut spec = base_canonical_spec("invalid_missing_owner");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    game_session
        .children
        .retain(|c| !matches!(c.kind, SimThingKind::Owner));
    write_json(&spec, "invalid_missing_owner.simthing-scenario.json");
}

fn write_duplicate_owners() {
    let mut spec = base_canonical_spec("invalid_duplicate_owner_ids");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    game_session.add_child(make_owner_entity("owner_a", "Owner B", "player"));
    write_json(&spec, "invalid_duplicate_owner_ids.simthing-scenario.json");
}

fn write_missing_galaxymap() {
    let mut spec = base_canonical_spec("invalid_missing_galaxymap");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    game_session
        .children
        .retain(|c| c.kind != SimThingKind::Location);
    spec.structural_grid.map_container_id = "99999".into();
    write_json(&spec, "invalid_missing_galaxymap.simthing-scenario.json");
}

fn write_bad_map_container() {
    let mut spec = galaxymap_with_gridcells();
    spec.scenario_id = "invalid_bad_map_container".into();
    spec.structural_grid.map_container_id = "99999".into();
    write_json(&spec, "invalid_bad_map_container.simthing-scenario.json");
}

fn write_planet_child() {
    let mut spec = galaxymap_with_gridcells();
    spec.scenario_id = "unsupported_planet_child_valid_schema".into();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .expect("star system cell");
    gridcell.add_child(SimThing::new(SimThingKind::Custom("Planet".into()), 0));
    write_json(
        &spec,
        "unsupported_planet_child_valid_schema.simthing-scenario.json",
    );
}

fn write_unknown_gridcell_role() {
    let mut spec = galaxymap_with_gridcells();
    spec.scenario_id = "unsupported_unknown_gridcell_role".into();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let galaxy_map = game_session
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");
    let gridcell = galaxy_map.children.first_mut().expect("cell");
    apply_gridcell_role_metadata(gridcell, "anomaly");
    write_json(
        &spec,
        "unsupported_unknown_gridcell_role.simthing-scenario.json",
    );
}

fn write_terran_pirate_reference() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");
    fs::write(
        corpus_path("legacy_world_root_terran_pirate_reference.txt"),
        path.display().to_string(),
    )
    .expect("write reference");
}
