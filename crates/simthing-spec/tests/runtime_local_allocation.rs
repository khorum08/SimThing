//! RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 — runtime allocation application proofs.

mod disburse_down_fixture;

use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, apply_owner_silo_runtime_writeback_cpu,
    apply_runtime_local_allocations_from_disburse_down, evaluate_planet_child_rf_reduce_up,
    owner_silo_demand_buckets_from_planet_child_rf,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, serialize_scenario_authority, OwnerRef, ResourceKey,
    RuntimeLocalAllocationApplicationErrorKind, RuntimeOwnerSiloDisburseDownAllocation,
    RuntimeOwnerSiloDisburseDownResult, ScopeId, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

fn disburse_results_for_spec(
    spec: &simthing_spec::SimThingScenarioSpec,
) -> Vec<RuntimeOwnerSiloDisburseDownResult> {
    let reduce_up = evaluate_planet_child_rf_reduce_up(spec);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("inputs");
    let initial = runtime_owner_silo_states_from_scenario(spec).expect("initial");
    let writeback = apply_owner_silo_runtime_writeback_cpu(&initial, &inputs).expect("writeback");
    let demands = owner_silo_demand_buckets_from_planet_child_rf(spec).expect("demands");
    apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("disburse")
}

#[test]
fn runtime_local_allocation_applies_disburse_down_allocations() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let disburse = disburse_results_for_spec(&spec);
    let report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");
    assert_eq!(report.allocation_count, 3);
    assert_eq!(report.allocated_total, 72);
    assert_eq!(report.unmet_total, 8);
}

#[test]
fn runtime_local_allocation_records_allocated_and_unmet_per_source() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let disburse = disburse_results_for_spec(&spec);
    let report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");

    let owner_a: Vec<_> = report
        .states
        .iter()
        .filter(|s| s.owner_ref.as_str() == "owner_a")
        .collect();
    assert_eq!(owner_a.len(), 2);
    let cohort = owner_a.iter().find(|s| s.requested == 20).expect("cohort");
    assert_eq!(cohort.allocated, 20);
    assert_eq!(cohort.unmet, 0);
    let fleet = owner_a.iter().find(|s| s.requested == 50).expect("fleet");
    assert_eq!(fleet.allocated, 42);
    assert_eq!(fleet.unmet, 8);

    let owner_b = report
        .states
        .iter()
        .find(|s| s.owner_ref.as_str() == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.requested, 10);
    assert_eq!(owner_b.allocated, 10);
    assert_eq!(owner_b.unmet, 0);
}

#[test]
fn runtime_local_allocation_preserves_owner_resource_scope() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let disburse = disburse_results_for_spec(&spec);
    let report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");

    for state in &report.states {
        assert_eq!(
            state.resource_key.as_str(),
            PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY
        );
        assert!(!state.scope_id.as_str().is_empty());
        assert_eq!(state.planet_id.as_deref(), Some(state.scope_id.as_str()));
        assert!(state.star_system_gridcell_id_raw.is_some());
    }
}

#[test]
fn runtime_local_allocation_rejects_missing_source_simthing_id() {
    let disburse = vec![RuntimeOwnerSiloDisburseDownResult {
        owner_ref: OwnerRef::new("owner_x"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        available_before: 10,
        allocated_total: 5,
        remaining_after: 5,
        unmet_total: 0,
        allocations: vec![RuntimeOwnerSiloDisburseDownAllocation {
            owner_ref: OwnerRef::new("owner_x"),
            resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
            scope_id: ScopeId::new("scope_a"),
            planet_id: Some("scope_a".into()),
            star_system_gridcell_id_raw: Some(8),
            requested: 5,
            allocated: 5,
            unmet: 0,
            priority: 10,
            source_simthing_id_raw: None,
        }],
    }];
    let err = apply_runtime_local_allocations_from_disburse_down(&disburse).unwrap_err();
    assert_eq!(
        err.kind,
        RuntimeLocalAllocationApplicationErrorKind::MissingSourceSimThingId
    );
}

#[test]
fn runtime_local_allocation_rejects_duplicate_source_allocation() {
    let allocation = RuntimeOwnerSiloDisburseDownAllocation {
        owner_ref: OwnerRef::new("owner_x"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        scope_id: ScopeId::new("scope_a"),
        planet_id: Some("scope_a".into()),
        star_system_gridcell_id_raw: Some(8),
        requested: 5,
        allocated: 5,
        unmet: 0,
        priority: 10,
        source_simthing_id_raw: Some(101),
    };
    let disburse = vec![RuntimeOwnerSiloDisburseDownResult {
        owner_ref: OwnerRef::new("owner_x"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        available_before: 10,
        allocated_total: 10,
        remaining_after: 0,
        unmet_total: 0,
        allocations: vec![allocation.clone(), allocation],
    }];
    let err = apply_runtime_local_allocations_from_disburse_down(&disburse).unwrap_err();
    assert_eq!(
        err.kind,
        RuntimeLocalAllocationApplicationErrorKind::DuplicateSourceAllocation
    );
}

#[test]
fn runtime_local_allocation_uses_checked_totals() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let disburse = disburse_results_for_spec(&spec);
    let report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");

    let sum_allocated: u32 = report.states.iter().map(|s| s.allocated).sum();
    let sum_unmet: u32 = report.states.iter().map(|s| s.unmet).sum();
    assert_eq!(report.allocated_total, sum_allocated);
    assert_eq!(report.unmet_total, sum_unmet);
}

#[test]
fn runtime_local_allocation_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let disburse = disburse_results_for_spec(&spec);
    let _report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn runtime_local_allocation_empty_disburse_down_zero_report_or_typed_deferral() {
    let report = apply_runtime_local_allocations_from_disburse_down(&[]).expect("empty");
    assert_eq!(report.allocation_count, 0);
    assert!(report.states.is_empty());
    assert!(report.economy_execution_deferred);
}

#[test]
fn runtime_local_allocation_economy_execution_deferred() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let disburse = disburse_results_for_spec(&spec);
    let report = apply_runtime_local_allocations_from_disburse_down(&disburse).expect("apply");
    assert!(report.economy_execution_deferred);
    assert!(report.scenario_authority_mutation_deferred);
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn normal_tests_do_not_write_runtime_allocation_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let json = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../scenarios/corpus/runtime_local_allocation_application_scoped.simthing-scenario.json",
    );
    std::fs::write(path, json).expect("write");
}
