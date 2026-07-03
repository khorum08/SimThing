//! PLANET-CHILD-RF-REDUCE-UP-0 — scoped reduce-up driver proof.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{
    compile_planet_child_rf_reduce_up_gpu_proof_plan,
    planet_child_rf_reduce_up_bucket_aggregate_slot,
    planet_child_rf_reduce_up_bucket_cpu_deficit_total,
    planet_child_rf_reduce_up_bucket_cpu_surplus_total,
    planet_child_rf_reduce_up_bucket_deficit_tick_inputs,
    planet_child_rf_reduce_up_bucket_surplus_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_owner_silo_metadata, apply_participant_owner_flow_metadata,
    apply_scenario_metadata_to_root, apply_star_system_local_grid_frame_metadata,
    evaluate_planet_child_rf_reduce_up, make_galaxy_map, make_owner_entity, make_planet_gridcell,
    structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec, SimThingStructuralGridFrame, SimThingStructuralGridPlacement, SpecError,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY, PLANET_ID_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS, STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS,
};

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

fn build_planet_child_rf_reduce_up_scoped_spec() -> SimThingScenarioSpec {
    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    let provenance = SimThingScenarioProvenance {
        source: "PLANET-CHILD-RF-REDUCE-UP-0".into(),
        generator_seed: 0x0001_2345_6789_ABCD,
        generator_shape: "planet_child_rf_reduce_up".into(),
    };
    apply_scenario_metadata_to_root(
        &mut root,
        "planet_child_rf_reduce_up_scoped",
        &provenance,
        SCENARIO_SCHEMA_VERSION,
    );

    let mut owner_a = make_owner_entity("owner_a", "Owner A", "player");
    apply_owner_silo_metadata(&mut owner_a, 50, Some(100));
    let mut owner_b = make_owner_entity("owner_b", "Owner B", "player");
    apply_owner_silo_metadata(&mut owner_b, 40, Some(80));

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    game_session.add_child(owner_a);
    game_session.add_child(owner_b);

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

    let mut terra_prime = make_planet_gridcell("terra_prime", 0, 0, Some("Terra Prime"));
    let mut terra_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut terra_cohort, "owner_a", 15, 0);
    let mut terra_fleet = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut terra_fleet, "owner_a", 0, 8);
    let mut terra_infra = SimThing::new(SimThingKind::Custom("Infrastructure".into()), 0);
    apply_participant_owner_flow_metadata(&mut terra_infra, "owner_a", 5, 0);
    terra_prime.add_child(terra_cohort);
    terra_prime.add_child(terra_fleet);
    terra_prime.add_child(terra_infra);

    let mut border_moon = make_planet_gridcell("border_moon", 1, 0, Some("Border Moon"));
    let mut moon_cohort = SimThing::new(SimThingKind::Cohort, 0);
    apply_participant_owner_flow_metadata(&mut moon_cohort, "owner_b", 7, 2);
    border_moon.add_child(moon_cohort);

    star_system.add_child(terra_prime);
    star_system.add_child(border_moon);
    galaxy_map.add_child(inert);
    galaxy_map.add_child(star_system);
    let map_raw = galaxy_map.id.raw();
    game_session.add_child(galaxy_map);
    root.add_child(game_session);

    let mut spec = SimThingScenarioSpec {
        scenario_id: "planet_child_rf_reduce_up_scoped".into(),
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

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_planet_child_rf_reduce_up_readback.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(
                        file,
                        "simthing-driver planet-child-rf-reduce-up readback lock"
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
#[test]
fn planet_child_rf_reduce_up_compile_preserves_bucket_scopes() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");
    assert_eq!(plan.bucket_plans.len(), 2);
    for bucket_plan in &plan.bucket_plans {
        assert_eq!(
            bucket_plan.scope.resource_key.as_str(),
            PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY
        );
        assert!(bucket_plan.scope.planet_id.is_some());
        assert!(bucket_plan.scope.star_system_gridcell_id_raw.is_some());
        assert!(!bucket_plan.participant_indices.is_empty());
    }
}

#[test]
fn planet_child_rf_reduce_up_cpu_oracle_matches_expected_buckets() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");
    let report = evaluate_planet_child_rf_reduce_up(&spec);

    for bucket_plan in &plan.bucket_plans {
        let expected = report
            .buckets
            .iter()
            .find(|b| b.scope == bucket_plan.scope)
            .expect("report bucket");
        assert_eq!(
            planet_child_rf_reduce_up_bucket_cpu_surplus_total(&plan, bucket_plan),
            expected.surplus_total
        );
        assert_eq!(
            planet_child_rf_reduce_up_bucket_cpu_deficit_total(&plan, bucket_plan),
            expected.deficit_total
        );

        let surplus_inputs =
            planet_child_rf_reduce_up_bucket_surplus_tick_inputs(&plan, bucket_plan);
        let deficit_inputs =
            planet_child_rf_reduce_up_bucket_deficit_tick_inputs(&plan, bucket_plan);
        let aggregate = planet_child_rf_reduce_up_bucket_aggregate_slot(bucket_plan);
        let surplus_cpu =
            execute_accumulator_plan_tick_cpu(&bucket_plan.surplus_plan, &surplus_inputs)
                .expect("surplus cpu");
        let deficit_cpu =
            execute_accumulator_plan_tick_cpu(&bucket_plan.deficit_plan, &deficit_inputs)
                .expect("deficit cpu");
        assert_eq!(surplus_cpu[aggregate], expected.surplus_total as f32);
        assert_eq!(deficit_cpu[aggregate], expected.deficit_total as f32);
    }
}

#[test]
fn planet_child_rf_reduce_up_keeps_owner_a_and_owner_b_separate() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");
    let owners: Vec<_> = plan
        .bucket_plans
        .iter()
        .map(|b| b.scope.owner_ref.as_str())
        .collect();
    assert!(owners.contains(&"owner_a"));
    assert!(owners.contains(&"owner_b"));
}

#[test]
fn planet_child_rf_reduce_up_gpu_each_bucket_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_planet_child_rf_reduce_up_gpu_matches_cpu());
}

fn run_planet_child_rf_reduce_up_gpu_matches_cpu() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("PLANET-CHILD-RF-REDUCE-UP-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");

    for bucket_plan in &plan.bucket_plans {
        let aggregate = planet_child_rf_reduce_up_bucket_aggregate_slot(bucket_plan);
        let surplus_inputs =
            planet_child_rf_reduce_up_bucket_surplus_tick_inputs(&plan, bucket_plan);
        let surplus_cpu =
            execute_accumulator_plan_tick_cpu(&bucket_plan.surplus_plan, &surplus_inputs)
                .expect("surplus cpu");
        let mut surplus_state =
            SimGpuAccumulatorTickState::new(&ctx, bucket_plan.surplus_plan.clone())
                .expect("surplus init");
        let surplus_gpu = surplus_state
            .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("surplus gpu")
            .expect("surplus readback");
        assert_eq!(surplus_cpu[aggregate], surplus_gpu[aggregate]);

        let deficit_inputs =
            planet_child_rf_reduce_up_bucket_deficit_tick_inputs(&plan, bucket_plan);
        let deficit_cpu =
            execute_accumulator_plan_tick_cpu(&bucket_plan.deficit_plan, &deficit_inputs)
                .expect("deficit cpu");
        let mut deficit_state =
            SimGpuAccumulatorTickState::new(&ctx, bucket_plan.deficit_plan.clone())
                .expect("deficit init");
        let deficit_gpu = deficit_state
            .tick(&ctx, &deficit_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("deficit gpu")
            .expect("deficit readback");
        assert_eq!(deficit_cpu[aggregate], deficit_gpu[aggregate]);
    }

    eprintln!("PLANET-CHILD-RF-REDUCE-UP-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn planet_child_rf_reduce_up_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("PLANET-CHILD-RF-REDUCE-UP-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn planet_child_rf_reduce_up_full_state_mutation_deferred() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");
    assert!(plan.full_state_mutation_deferred);
    let report = evaluate_planet_child_rf_reduce_up(&spec);
    assert!(report.deferrals.iter().any(|d| {
        matches!(
            d.kind,
            simthing_spec::PlanetChildRfDeferralKind::PlanetChildRfSimulationDeferred
        )
    }));
}

#[test]
fn planet_child_rf_reduce_up_does_not_require_new_gpu_primitive() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile");
    for bucket_plan in &plan.bucket_plans {
        assert_eq!(bucket_plan.surplus_plan.ops.len(), 1);
        assert_eq!(bucket_plan.deficit_plan.ops.len(), 1);
    }
}
