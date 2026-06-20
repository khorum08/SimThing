//! OWNER-SILO-RUNTIME-WRITEBACK-0 — runtime owner-silo writeback driver proof.

mod reduce_up_fixture;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_driver::{
    compile_owner_silo_runtime_writeback_plan, owner_silo_writeback_aggregate_deficit_tick_inputs,
    owner_silo_writeback_aggregate_slot, owner_silo_writeback_aggregate_surplus_tick_inputs,
};
use simthing_gpu::set_debug_readback_allowed;
use simthing_sim::{
    execute_accumulator_plan_tick_cpu, gpu_context_blocking, SimGpuAccumulatorTickState,
    SimGpuReadbackPolicy,
};
use simthing_spec::{
    apply_owner_silo_runtime_writeback_cpu, read_owner_silo_current_from_owner,
    serialize_scenario_authority, RuntimeOwnerSiloState, RuntimeOwnerSiloWritebackInput, SpecError,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, OWNER_SILO_CURRENT_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY, PLANET_ID_PROPERTY_ID,
};

use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

struct ProcessReadbackTestLock {
    path: PathBuf,
}

impl ProcessReadbackTestLock {
    fn acquire() -> Self {
        let path = std::env::temp_dir().join("simthing_owner_silo_runtime_writeback.lock");
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "simthing-driver owner-silo-runtime-writeback lock");
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

fn owner_entity_mut<'a>(
    spec: &'a mut simthing_spec::SimThingScenarioSpec,
    owner_ref: &str,
) -> &'a mut simthing_core::SimThing {
    let gs = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    gs.children
        .iter_mut()
        .find(|c| {
            c.kind == SimThingKind::Owner
                && simthing_spec::owner_entity_id(c).as_deref() == Some(owner_ref)
        })
        .expect("owner")
}

#[test]
fn owner_silo_runtime_writeback_compile_rejects_rejected_reduce_up() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
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
    let planet = star
        .children
        .iter_mut()
        .find(|c| simthing_spec::is_planet_gridcell(c))
        .unwrap();
    planet.properties.remove(&PLANET_ID_PROPERTY_ID);

    let err = compile_owner_silo_runtime_writeback_plan(&spec).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn owner_silo_runtime_writeback_compile_rejects_unknown_owner_ref() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
    let gs = spec
        .root
        .children
        .iter_mut()
        .find(|c| c.kind == SimThingKind::GameSession)
        .unwrap();
    gs.children.retain(|c| {
        !(c.kind == SimThingKind::Owner
            && simthing_spec::owner_entity_id(c).as_deref() == Some("owner_b"))
    });

    let err = compile_owner_silo_runtime_writeback_plan(&spec).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn owner_silo_runtime_writeback_compile_rejects_invalid_owner_silo_metadata() {
    let mut spec = build_planet_child_rf_reduce_up_scoped_spec();
    let owner = owner_entity_mut(&mut spec, "owner_a");
    owner.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        PropertyValue { data: vec![1.5] },
    );

    let err = compile_owner_silo_runtime_writeback_plan(&spec).unwrap_err();
    assert!(matches!(err, SpecError::ValidationFailed));
}

#[test]
fn owner_silo_runtime_writeback_cpu_applies_net_surplus() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");

    let owner_a = plan
        .cpu_results
        .iter()
        .find(|r| r.owner_ref == "owner_a")
        .expect("owner_a");
    assert_eq!(owner_a.previous_current, 50);
    assert_eq!(owner_a.next_current, 62);

    let owner_b = plan
        .cpu_results
        .iter()
        .find(|r| r.owner_ref == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.previous_current, 40);
    assert_eq!(owner_b.next_current, 45);
}

#[test]
fn owner_silo_runtime_writeback_cpu_applies_net_deficit_without_underflow() {
    let initial = vec![RuntimeOwnerSiloState {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        current: 20,
        capacity: Some(100),
    }];
    let writeback = vec![RuntimeOwnerSiloWritebackInput {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        net_surplus: 0,
        net_deficit: 8,
        source_bucket_count: 1,
    }];
    let results = apply_owner_silo_runtime_writeback_cpu(&initial, &writeback).expect("apply");
    assert_eq!(results[0].next_current, 12);
    assert_eq!(results[0].applied_deficit, 8);
    assert_eq!(results[0].unmet_deficit, 0);
}

#[test]
fn owner_silo_runtime_writeback_cpu_records_unmet_deficit() {
    let initial = vec![RuntimeOwnerSiloState {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        current: 5,
        capacity: Some(100),
    }];
    let writeback = vec![RuntimeOwnerSiloWritebackInput {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        net_surplus: 0,
        net_deficit: 10,
        source_bucket_count: 1,
    }];
    let results = apply_owner_silo_runtime_writeback_cpu(&initial, &writeback).expect("apply");
    assert_eq!(results[0].next_current, 0);
    assert_eq!(results[0].unmet_deficit, 5);
    assert_eq!(results[0].applied_deficit, 5);
}

#[test]
fn owner_silo_runtime_writeback_cpu_clamps_to_capacity() {
    let initial = vec![RuntimeOwnerSiloState {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        current: 95,
        capacity: Some(100),
    }];
    let writeback = vec![RuntimeOwnerSiloWritebackInput {
        owner_ref: "owner_x".into(),
        resource_key: PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY.into(),
        net_surplus: 12,
        net_deficit: 0,
        source_bucket_count: 1,
    }];
    let results = apply_owner_silo_runtime_writeback_cpu(&initial, &writeback).expect("apply");
    assert_eq!(results[0].next_current, 100);
    assert_eq!(results[0].clamped_surplus, 7);
    assert_eq!(results[0].applied_surplus, 5);
}

#[test]
fn owner_silo_runtime_writeback_keeps_two_owners_separate() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");
    assert_eq!(plan.cpu_results.len(), 2);
    assert_eq!(plan.writeback_inputs.len(), 2);
}

#[test]
fn owner_silo_runtime_writeback_preserves_scenario_authority() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");
    assert_eq!(plan.cpu_results.len(), 2);
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);

    let gs = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs");
    let owner_a = gs
        .children
        .iter()
        .find(|c| simthing_spec::owner_entity_id(c).as_deref() == Some("owner_a"))
        .expect("owner_a");
    assert_eq!(read_owner_silo_current_from_owner(owner_a), Some(50));
}

#[test]
fn owner_silo_runtime_writeback_disburse_down_deferred() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");
    assert!(plan.disburse_down_deferred);
    assert!(plan.scenario_authority_mutation_deferred);
}

#[test]
fn owner_silo_runtime_writeback_gpu_aggregate_matches_cpu_when_adapter_available() {
    with_isolated_readback_gate_test(|| run_owner_silo_writeback_gpu_aggregate_proof());
}

fn run_owner_silo_writeback_gpu_aggregate_proof() {
    let Some(ctx) = gpu_context_blocking().ok() else {
        eprintln!("OWNER-SILO-RUNTIME-WRITEBACK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
        return;
    };

    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");

    for proof_plan in &plan.gpu_aggregate_proof_plans {
        let writeback_input = plan
            .writeback_inputs
            .iter()
            .find(|i| {
                i.owner_ref == proof_plan.owner_ref && i.resource_key == proof_plan.resource_key
            })
            .expect("input");
        let aggregate = owner_silo_writeback_aggregate_slot(proof_plan);

        let surplus_inputs = owner_silo_writeback_aggregate_surplus_tick_inputs(&plan, proof_plan);
        let surplus_cpu =
            execute_accumulator_plan_tick_cpu(&proof_plan.surplus_plan, &surplus_inputs)
                .expect("surplus cpu");
        let mut surplus_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.surplus_plan.clone()).expect("init");
        let surplus_gpu = surplus_state
            .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu")
            .expect("readback");
        assert_eq!(surplus_cpu[aggregate], surplus_gpu[aggregate]);
        assert_eq!(surplus_cpu[aggregate], writeback_input.net_surplus as f32);

        let deficit_inputs = owner_silo_writeback_aggregate_deficit_tick_inputs(&plan, proof_plan);
        let deficit_cpu =
            execute_accumulator_plan_tick_cpu(&proof_plan.deficit_plan, &deficit_inputs)
                .expect("deficit cpu");
        let mut deficit_state =
            SimGpuAccumulatorTickState::new(&ctx, proof_plan.deficit_plan.clone()).expect("init");
        let deficit_gpu = deficit_state
            .tick(&ctx, &deficit_inputs, SimGpuReadbackPolicy::ProofReadback)
            .expect("gpu")
            .expect("readback");
        assert_eq!(deficit_cpu[aggregate], deficit_gpu[aggregate]);
        assert_eq!(deficit_cpu[aggregate], writeback_input.net_deficit as f32);
    }

    eprintln!("OWNER-SILO-RUNTIME-WRITEBACK-0: REAL_ADAPTER_OBSERVED");
}

#[test]
fn owner_silo_runtime_writeback_gpu_skips_honestly_without_adapter() {
    if gpu_context_blocking().is_ok() {
        return;
    }
    eprintln!("OWNER-SILO-RUNTIME-WRITEBACK-0: GPU_TESTS_SKIPPED_NO_ADAPTER");
}

#[test]
fn owner_silo_runtime_writeback_uses_existing_accumulator_plan() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let plan = compile_owner_silo_runtime_writeback_plan(&spec).expect("compile");
    for proof in &plan.gpu_aggregate_proof_plans {
        assert_eq!(proof.surplus_plan.ops.len(), 1);
        assert_eq!(proof.deficit_plan.ops.len(), 1);
    }
}
