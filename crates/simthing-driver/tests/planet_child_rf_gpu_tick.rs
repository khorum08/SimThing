//! PLANET-CHILD-RF-GPU-PARTICIPANT-0 — planet/non-grid child RF participant GPU tick proof.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_planet_child_rf_gpu_tick_plan, planet_child_rf_aggregate_slot,
    planet_child_rf_deficit_tick_inputs, planet_child_rf_participant_deficit_total,
    planet_child_rf_participant_surplus_total, planet_child_rf_surplus_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, apply_star_system_local_grid_frame_metadata,
    deserialize_scenario_authority, evaluate_planet_child_locations,
    evaluate_planet_child_rf_admission, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    planet_child_rf_participant_inputs, scenario_metadata_string_value,
    serialize_scenario_authority, structural_property_value_u32,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionErrorKind, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, SpecError, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, PLANET_OWNER_REF_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_planet_child_rf_gpu_readback_test.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(
                        file,
                        "simthing-driver planet-child-rf readback integration lock"
                    );
                    return Self { path };
                }
                Err(_) => thread::sleep(Duration::from_millis(25)),
            }
        }
    }
}

impl Drop for ProcessReadbackTestLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        set_debug_readback_allowed(false);
    }
}

fn with_isolated_readback_gate_test<F: FnOnce()>(f: F) {
    let _lock = ProcessReadbackTestLock::acquire();
    set_debug_readback_allowed(false);
    f();
    set_debug_readback_allowed(false);
}

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn make_gridcell(role: &str, system_id: u32, col: u32, row: u32) -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell, role);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );
    cell.add_child(SimThing::new(SimThingKind::Cohort, 0));
    cell
}

fn build_planet_child_rf_admitted_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "PLANET-CHILD-RF-GPU-PARTICIPANT-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "planet_child_rf".into(),
    };
    apply_scenario_metadata_to_root(
        &mut root,
        "planet_child_rf_participants_admitted",
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );

    let mut owner = make_owner_entity("owner_a", "Owner A", "player");
    apply_owner_silo_metadata(&mut owner, 50, Some(100));

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(owner);

    let mut galaxy_map = make_galaxy_map("galaxy_a", "Galaxy A");

    let inert = make_gridcell(GALAXY_GRIDCELL_ROLE_INERT, 1, 0, 0);
    let inert_raw = inert.id.raw();

    let mut star_system = make_gridcell(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, 2, 1, 0);
    apply_star_system_local_grid_frame_metadata(
        &mut star_system,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
        STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
    );
    let star_raw = star_system.id.raw();

    let mut planet = make_planet_gridcell("terra_prime", 0, 0, Some("Terra Prime"));
    planet.add_property(
        PLANET_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value("owner_a"),
    );
    apply_participant_owner_flow_metadata(&mut planet, "owner_a", 10, 5);

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut cohort, "owner_a", 15, 0);

    let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut fleet, "owner_a", 0, 8);

    let mut infrastructure = SimThing::new(SimThingKind::Custom("Infrastructure".into()), 0);
    apply_participant_owner_flow_metadata(&mut infrastructure, "owner_a", 5, 0);

    planet.add_child(cohort);
    planet.add_child(fleet);
    planet.add_child(infrastructure);
    star_system.add_child(planet);

    galaxy_map.add_child(inert);
    galaxy_map.add_child(star_system);
    let map_raw = galaxy_map.id.raw();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    let mut spec = SimThingScenarioSpec {
        scenario_id: "planet_child_rf_participants_admitted".into(),
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
                    simthing_id_raw: inert_raw,
                },
                SimThingStructuralGridPlacement {
                    location_id: "cell_b".into(),
                    target_id: "cell_b".into(),
                    system_id: 2,
                    row: 0,
                    col: 1,
                    simthing_id_raw: star_raw,
                },
            ],
        },
        links: Vec::new(),
        provenance,
    };
    spec.sync_sidecar_from_root_metadata();
    spec
}
#[test]
fn planet_child_rf_compile_requires_owner_channel_for_active_participant() {
    let mut spec = build_planet_child_rf_admitted_spec();
    let star = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Location)
        .unwrap()
        .children
        .iter_mut()
        .find(|c| {
            simthing_spec::gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .unwrap();
    let fleet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap()
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::Fleet)
        .unwrap();
    fleet
        .properties
        .remove(&simthing_spec::OWNER_FLOW_OWNER_REF_PROPERTY_ID);

    let report = evaluate_planet_child_rf_admission(&spec);
    assert_eq!(
        report.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(report.errors.iter().any(|e| {
        e.kind == PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant
    }));
    let err = compile_planet_child_rf_gpu_tick_plan(&spec).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn planet_child_rf_participant_inputs_include_planet_gridcell_and_non_grid_children() {
    let spec = build_planet_child_rf_admitted_spec();
    let participants = planet_child_rf_participant_inputs(&spec).expect("inputs");
    assert_eq!(participants.len(), 4);
    assert!(participants
        .iter()
        .any(|p| p.participant_kind_label == "planet_gridcell"));
    assert!(participants
        .iter()
        .any(|p| p.participant_kind_label == "cohort"));
    assert!(participants
        .iter()
        .any(|p| p.participant_kind_label == "fleet"));
    assert!(participants
        .iter()
        .any(|p| p.participant_kind_label == "Infrastructure"));
}

#[test]
fn planet_child_rf_participants_preserve_spatial_parentage() {
    let spec = build_planet_child_rf_admitted_spec();
    let location_report = evaluate_planet_child_locations(&spec);
    assert_eq!(location_report.planet_gridcell_count, 1);
    assert_eq!(location_report.planet_non_grid_child_count, 3);
    assert_eq!(spec.structural_grid.placements.len(), 2);

    let participants = planet_child_rf_participant_inputs(&spec).expect("inputs");
    for participant in &participants {
        assert!(participant
            .spatial_parent_path
            .contains("galaxymap/star_system"));
        assert!(participant
            .spatial_parent_path
            .contains("planet/terra_prime"));
        assert_eq!(participant.owner_ref, "owner_a");
    }
}

#[test]
fn planet_child_rf_cpu_oracle_totals_match_expected() {
    let spec = build_planet_child_rf_admitted_spec();
    let plan = compile_planet_child_rf_gpu_tick_plan(&spec).expect("compile");
    assert_eq!(planet_child_rf_participant_surplus_total(&plan), 30);
    assert_eq!(planet_child_rf_participant_deficit_total(&plan), 13);
    assert!(plan.full_state_mutation_deferred);

    let surplus_inputs = planet_child_rf_surplus_tick_inputs(&plan);
    let deficit_inputs = planet_child_rf_deficit_tick_inputs(&plan);
    let aggregate = planet_child_rf_aggregate_slot(&plan);
    let surplus_cpu =
        execute_accumulator_plan_tick_cpu(&plan.surplus_plan, &surplus_inputs).expect("surplus");
    let deficit_cpu =
        execute_accumulator_plan_tick_cpu(&plan.deficit_plan, &deficit_inputs).expect("deficit");
    assert_eq!(surplus_cpu[aggregate], 30.0);
    assert_eq!(deficit_cpu[aggregate], 13.0);
}

#[test]
fn planet_child_rf_gpu_surplus_sum_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_planet_child_rf_gpu_matches_cpu_oracle());
}

fn run_planet_child_rf_gpu_matches_cpu_oracle() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("PLANET-CHILD-RF-GPU-PARTICIPANT-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };
    let spec = build_planet_child_rf_admitted_spec();
    let plan = compile_planet_child_rf_gpu_tick_plan(&spec).expect("compile");
    let aggregate = planet_child_rf_aggregate_slot(&plan);

    let surplus_inputs = planet_child_rf_surplus_tick_inputs(&plan);
    let surplus_cpu = execute_accumulator_plan_tick_cpu(&plan.surplus_plan, &surplus_inputs)
        .expect("surplus cpu");
    let mut surplus_state =
        SimGpuAccumulatorTickState::new(&ctx, plan.surplus_plan.clone()).expect("surplus init");
    let surplus_gpu = surplus_state
        .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("surplus gpu")
        .expect("surplus readback");
    assert_eq!(surplus_cpu[aggregate], surplus_gpu[aggregate]);

    let deficit_inputs = planet_child_rf_deficit_tick_inputs(&plan);
    let deficit_cpu = execute_accumulator_plan_tick_cpu(&plan.deficit_plan, &deficit_inputs)
        .expect("deficit cpu");
    let mut deficit_state =
        SimGpuAccumulatorTickState::new(&ctx, plan.deficit_plan.clone()).expect("deficit init");
    let deficit_gpu = deficit_state
        .tick(&ctx, &deficit_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("deficit gpu")
        .expect("deficit readback");
    assert_eq!(deficit_cpu[aggregate], deficit_gpu[aggregate]);
    eprintln!("PLANET-CHILD-RF-GPU-PARTICIPANT-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn planet_child_rf_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("PLANET-CHILD-RF-GPU-PARTICIPANT-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn planet_child_rf_full_state_mutation_deferred() {
    let spec = build_planet_child_rf_admitted_spec();
    let plan = compile_planet_child_rf_gpu_tick_plan(&spec).expect("compile");
    assert!(plan.full_state_mutation_deferred);
    let report = evaluate_planet_child_rf_admission(&spec);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            simthing_spec::PlanetChildRfDeferralKind::PlanetChildRfSimulationDeferred
        )
    }));
}

#[test]
fn planet_child_rf_corpus_fixture_roundtrips() {
    let spec = build_planet_child_rf_admitted_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let roundtrip = deserialize_scenario_authority(&json).expect("deserialize");
    let plan = compile_planet_child_rf_gpu_tick_plan(&roundtrip).expect("compile");
    assert_eq!(plan.participants.len(), 4);
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn write_planet_child_rf_corpus_fixture() {
    let spec = build_planet_child_rf_admitted_spec();
    let json = serialize_scenario_authority(&spec).expect("serialize");
    fs::write(
        corpus_path("planet_child_rf_participants_admitted.simthing-scenario.json"),
        json,
    )
    .expect("write corpus");
}
