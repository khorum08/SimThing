//! SESSION-RESOURCE-FLOW-SILOS-0 — owner silo reduce-up / disburse-down admission tests.

use std::fs;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, evaluate_owner_silo_flow, ingest_scenario,
    ingest_scenario_from_str, make_galaxy_map, make_owner_entity, owner_flow_owner_ref,
    owner_has_silo_metadata, owner_silo_flow_participant_inputs, serialize_scenario_authority,
    structural_property_value_u32, OwnerSiloAdmissionClassification, OwnerSiloAdmissionErrorKind,
    OwnerSiloDeferralKind, ScenarioDeferralKind, ScenarioIngestionClassification,
    ScenarioIngestionProfile, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement, SpecError,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
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

fn base_spec(scenario_id: &str) -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "SESSION-RESOURCE-FLOW-SILOS-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "owner_silo".into(),
        ..SimThingScenarioProvenance::default()
    };
    apply_scenario_metadata_to_root(&mut root, scenario_id, &provenance, SCENARIO_SCHEMA_VERSION);
    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    let mut owner = make_owner_entity("owner_a", "Owner A", "player");
    apply_owner_silo_metadata(&mut owner, 50, Some(100));
    game_session.add_child(owner);
    let galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");
    let map_raw = galaxy_map.id.raw().to_string();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);
    let mut spec = SimThingScenarioSpec {
        scenario_id: scenario_id.to_string(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
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

fn add_gridcell_with_participant(
    spec: &mut SimThingScenarioSpec,
    role: &str,
    system_id: u32,
    col: u32,
    row: u32,
    surplus: u32,
    deficit: u32,
) -> u32 {
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

    let mut gridcell = SimThing::new(SimThingKind::Location, 0);
    if role == GALAXY_GRIDCELL_ROLE_INERT {
        apply_gridcell_role_metadata(&mut gridcell, GALAXY_GRIDCELL_ROLE_INERT);
    } else {
        apply_gridcell_role_metadata(&mut gridcell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    }
    gridcell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );

    let mut participant = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut participant, "owner_a", surplus, deficit);
    let raw = participant.id.raw();
    gridcell.add_child(participant);
    let cell_raw = gridcell.id.raw();
    galaxy_map.add_child(gridcell);

    spec.structural_grid
        .placements
        .push(SimThingStructuralGridPlacement {
            location_id: format!("cell_{system_id}"),
            target_id: format!("cell_{system_id}"),
            system_id,
            row,
            col,
            simthing_id_raw: cell_raw,
        });
    spec.structural_grid.frame.occupied_cells = spec.structural_grid.placements.len() as u64;
    raw
}

fn balanced_flow_spec() -> SimThingScenarioSpec {
    let mut spec = base_spec("owner_silo_balanced_flow");
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 30, 0);
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, 2, 1, 0, 0, 20);
    spec
}

#[test]
fn owner_silo_balanced_flow_admits() {
    let spec = balanced_flow_spec();
    let report = evaluate_owner_silo_flow(&spec);
    assert_eq!(report.participant_count, 2);
    assert_eq!(report.silo_owner_count, 1);
    assert!(matches!(
        report.classification,
        OwnerSiloAdmissionClassification::PartiallyAdmitted
            | OwnerSiloAdmissionClassification::Admitted
    ));
    assert!(report.errors.is_empty());
}

#[test]
fn owner_silo_surplus_reduces_up_to_owner() {
    let mut spec = base_spec("surplus_only");
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 40, 0);
    let report = evaluate_owner_silo_flow(&spec);
    assert_eq!(report.reducible_surplus_total, 40.0);
    assert_eq!(report.resolvable_deficit_total, 0.0);
}

#[test]
fn owner_silo_disburses_down_to_deficit() {
    let mut spec = base_spec("deficit_only");
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 0, 15);
    let report = evaluate_owner_silo_flow(&spec);
    assert_eq!(report.reducible_surplus_total, 0.0);
    assert_eq!(report.resolvable_deficit_total, 15.0);
    assert_eq!(report.unresolved_deficit_total, 0.0);
}

#[test]
fn owner_silo_unresolved_deficit_reported() {
    let json = load_corpus("owner_silo_unresolved_deficit.simthing-scenario.json");
    let spec = simthing_spec::deserialize_scenario_authority(&json).expect("parse");
    let report = evaluate_owner_silo_flow(&spec);
    assert!(report.unresolved_deficit_total > 0.0);
    assert!(report.resolvable_deficit_total > 0.0);
}

#[test]
fn owner_silo_does_not_require_owner_spatial_parenting() {
    let spec = balanced_flow_spec();
    let game_session = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let owners_under_gs = game_session
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Owner)
        .count();
    assert_eq!(owners_under_gs, 1);

    let galaxy_map = game_session
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Location)
        .expect("map");
    for gridcell in &galaxy_map.children {
        for child in &gridcell.children {
            assert_ne!(child.kind, SimThingKind::Owner);
            assert!(owner_flow_owner_ref(child).is_some());
        }
    }
}

#[test]
fn owner_silo_ingestion_no_longer_emits_not_executed_when_admitted() {
    let json = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("balanced", &json, CANONICAL_PROFILE);
    assert_ne!(
        result.classification,
        ScenarioIngestionClassification::Rejected
    );
    assert!(result.owner_silo.is_some());
    assert!(!result
        .deferrals
        .iter()
        .any(|d| { d.kind == ScenarioDeferralKind::OwnerResourceFlowNotYetExecuted }));
}

#[test]
fn owner_silo_ingestion_preserves_canonical_scenario_validation() {
    let spec = balanced_flow_spec();
    let result = ingest_scenario("balanced_inline", &spec, CANONICAL_PROFILE);
    assert!(result.validation.canonical_validation_ok);
    assert!(result.validation.owners_ok);
    assert!(result.validation.galaxy_map_ok);
}

fn owner_mut(spec: &mut SimThingScenarioSpec) -> &mut SimThing {
    spec.root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs")
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Owner)
        .expect("owner")
}

#[test]
fn owner_silo_marker_without_current_behavior_is_explicitly_tested() {
    let mut spec = base_spec("marker_placeholder_only");
    let owner = owner_mut(&mut spec);
    owner
        .properties
        .remove(&simthing_spec::OWNER_SILO_CURRENT_PROPERTY_ID);
    owner
        .properties
        .remove(&simthing_spec::OWNER_SILO_CAPACITY_PROPERTY_ID);
    owner.properties.insert(
        simthing_spec::OWNER_SILO_MARKER_PROPERTY_ID,
        structural_property_value_u32(0),
    );

    let placeholder_report = evaluate_owner_silo_flow(&spec);
    assert!(matches!(
        placeholder_report.classification,
        OwnerSiloAdmissionClassification::PartiallyAdmitted
            | OwnerSiloAdmissionClassification::Admitted
    ));
    assert!(placeholder_report.errors.is_empty());
    assert!(placeholder_report
        .deferrals
        .iter()
        .any(|d| d.kind == OwnerSiloDeferralKind::ResourceFlowExecutionDeferred));

    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 10, 0);
    let active_report = evaluate_owner_silo_flow(&spec);
    assert_eq!(
        active_report.classification,
        OwnerSiloAdmissionClassification::Rejected
    );
    assert!(active_report.errors.iter().any(|e| {
        e.kind == OwnerSiloAdmissionErrorKind::InvalidSiloAmount
            && e.message.contains("owner_silo_current is required")
    }));
}

#[test]
fn owner_silo_ingestion_reports_gpu_participant_accumulation_readiness() {
    let json = load_corpus("owner_silo_balanced_flow.simthing-scenario.json");
    let (result, _) = ingest_scenario_from_str("balanced", &json, CANONICAL_PROFILE);
    assert!(
        result
            .compile_readiness
            .owner_silo_gpu_participant_accumulation_ready
    );
    assert!(
        result
            .compile_readiness
            .owner_silo_full_state_mutation_deferred
    );
}

#[test]
#[ignore = "run once to refresh owner_silo corpus fixtures"]
fn write_owner_silo_corpus_fixtures() {
    write_balanced_flow();
    write_unresolved_deficit();
    write_unknown_owner_ref();
    write_missing_silo();
    write_invalid_silo_amount();
}

fn write_json(spec: &SimThingScenarioSpec, name: &str) {
    let json = serialize_scenario_authority(spec).expect("serialize");
    fs::write(corpus_path(name), json).expect("write");
}

fn write_balanced_flow() {
    write_json(
        &balanced_flow_spec(),
        "owner_silo_balanced_flow.simthing-scenario.json",
    );
}

fn write_unresolved_deficit() {
    let mut spec = base_spec("owner_silo_unresolved_deficit");
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
    apply_owner_silo_metadata(owner, 10, Some(100));
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 5, 0);
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, 2, 1, 0, 0, 50);
    write_json(
        &spec,
        "owner_silo_unresolved_deficit.simthing-scenario.json",
    );
}

fn write_unknown_owner_ref() {
    let mut spec = base_spec("owner_silo_unknown_owner_ref");
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

    let mut gridcell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut gridcell, GALAXY_GRIDCELL_ROLE_INERT);
    gridcell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    let mut participant = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut participant, "owner_missing", 10, 0);
    gridcell.add_child(participant);
    let cell_raw = gridcell.id.raw();
    galaxy_map.add_child(gridcell);
    spec.structural_grid
        .placements
        .push(SimThingStructuralGridPlacement {
            location_id: "cell_1".into(),
            target_id: "cell_1".into(),
            system_id: 1,
            row: 0,
            col: 0,
            simthing_id_raw: cell_raw,
        });
    spec.structural_grid.frame.occupied_cells = 1;
    write_json(&spec, "owner_silo_unknown_owner_ref.simthing-scenario.json");
}

fn write_missing_silo() {
    let mut spec = base_spec("owner_silo_missing_silo");
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    game_session
        .children
        .retain(|c| c.kind != SimThingKind::Owner);
    let mut owner = make_owner_entity("owner_a", "Owner A", "player");
    owner
        .properties
        .remove(&simthing_spec::OWNER_SILO_MARKER_PROPERTY_ID);
    assert!(!owner_has_silo_metadata(&owner));
    game_session.add_child(owner);
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 10, 0);
    write_json(&spec, "owner_silo_missing_silo.simthing-scenario.json");
}

fn write_invalid_silo_amount() {
    let mut spec = base_spec("owner_silo_invalid_silo_amount");
    add_gridcell_with_participant(&mut spec, GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0, 10, 0);
    owner_mut(&mut spec).add_property(
        simthing_spec::OWNER_SILO_CURRENT_PROPERTY_ID,
        simthing_core::PropertyValue::from_raw_lanes(vec![-7.0]),
    );
    write_json(
        &spec,
        "owner_silo_invalid_silo_amount.simthing-scenario.json",
    );
}
