//! OWNER-SILO-DISBURSE-DOWN-0 — demand derivation and CPU allocation oracle proofs.

mod disburse_down_fixture;
mod reduce_up_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, apply_owner_silo_runtime_writeback_cpu,
    evaluate_planet_child_rf_reduce_up, owner_silo_demand_buckets_from_planet_child_rf,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, serialize_scenario_authority, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloDisburseDownErrorKind,
    RuntimeOwnerSiloWritebackResult, ScopeId, OWNER_FLOW_DEFAULT_PRIORITY,
    OWNER_FLOW_DEMAND_PROPERTY_ID, OWNER_FLOW_PRIORITY_PROPERTY_ID,
    PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

fn writeback_results_for_spec(
    spec: &simthing_spec::SimThingScenarioSpec,
) -> Vec<RuntimeOwnerSiloWritebackResult> {
    let reduce_up = evaluate_planet_child_rf_reduce_up(spec);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("inputs");
    let initial = runtime_owner_silo_states_from_scenario(spec).expect("initial");
    apply_owner_silo_runtime_writeback_cpu(&initial, &inputs).expect("writeback")
}

#[test]
fn owner_silo_disburse_down_derives_demand_buckets_from_planet_children() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("buckets");
    assert_eq!(buckets.len(), 3);

    let owner_a: Vec<_> = buckets
        .iter()
        .filter(|b| b.owner_ref.as_str() == "owner_a")
        .collect();
    assert_eq!(owner_a.len(), 2);
    assert!(owner_a.iter().all(|b| b.scope_id.as_str() == "terra_prime"));
    assert!(owner_a
        .iter()
        .any(|b| b.requested == 20 && b.priority == 10));
    assert!(owner_a
        .iter()
        .any(|b| b.requested == 50 && b.priority == 20));

    let owner_b: Vec<_> = buckets
        .iter()
        .filter(|b| b.owner_ref.as_str() == "owner_b")
        .collect();
    assert_eq!(owner_b.len(), 1);
    assert_eq!(owner_b[0].scope_id.as_str(), "border_moon");
    assert_eq!(owner_b[0].requested, 10);
}

#[test]
fn owner_silo_disburse_down_ignores_children_without_demand_metadata() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("buckets");
    assert!(buckets.is_empty());
}

#[test]
fn owner_silo_disburse_down_allocates_by_priority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let writeback = writeback_results_for_spec(&spec);
    let demands = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("demands");
    let results =
        apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("allocate");

    let owner_a = results
        .iter()
        .find(|r| r.owner_ref.as_str() == "owner_a")
        .expect("owner_a");
    assert_eq!(owner_a.available_before, 62);
    let cohort = owner_a
        .allocations
        .iter()
        .find(|a| a.requested == 20)
        .expect("cohort");
    let fleet = owner_a
        .allocations
        .iter()
        .find(|a| a.requested == 50)
        .expect("fleet");
    assert_eq!(cohort.allocated, 20);
    assert_eq!(fleet.allocated, 42);
    assert!(cohort.priority < fleet.priority);
}

#[test]
fn owner_silo_disburse_down_tie_breaks_deterministically() {
    let writeback = vec![RuntimeOwnerSiloWritebackResult {
        owner_ref: OwnerRef::new("owner_x"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        previous_current: 30,
        next_current: 30,
        capacity: Some(100),
        applied_surplus: 0,
        applied_deficit: 0,
        clamped_surplus: 0,
        unmet_deficit: 0,
    }];
    let demands = vec![
        RuntimeOwnerSiloDemandBucket {
            owner_ref: OwnerRef::new("owner_x"),
            resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
            scope_id: ScopeId::new("scope_b"),
            planet_id: Some("scope_b".into()),
            star_system_gridcell_id_raw: Some(8),
            requested: 20,
            priority: 10,
            source_simthing_id_raw: Some(102),
        },
        RuntimeOwnerSiloDemandBucket {
            owner_ref: OwnerRef::new("owner_x"),
            resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
            scope_id: ScopeId::new("scope_a"),
            planet_id: Some("scope_a".into()),
            star_system_gridcell_id_raw: Some(8),
            requested: 20,
            priority: 10,
            source_simthing_id_raw: Some(101),
        },
    ];
    let results =
        apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("allocate");
    assert_eq!(results[0].allocations[0].scope_id.as_str(), "scope_a");
    assert_eq!(results[0].allocations[1].scope_id.as_str(), "scope_b");
}

#[test]
fn owner_silo_disburse_down_records_unmet_demand() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let writeback = writeback_results_for_spec(&spec);
    let demands = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("demands");
    let results =
        apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("allocate");

    let owner_a = results
        .iter()
        .find(|r| r.owner_ref.as_str() == "owner_a")
        .expect("owner_a");
    assert_eq!(owner_a.unmet_total, 8);
    let fleet = owner_a
        .allocations
        .iter()
        .find(|a| a.requested == 50)
        .expect("fleet");
    assert_eq!(fleet.unmet, 8);
}

#[test]
fn owner_silo_disburse_down_never_exceeds_available() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let writeback = writeback_results_for_spec(&spec);
    let demands = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("demands");
    let results =
        apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("allocate");

    for result in &results {
        assert!(result.allocated_total <= result.available_before);
        assert_eq!(
            result.remaining_after,
            result
                .available_before
                .saturating_sub(result.allocated_total)
        );
        for alloc in &result.allocations {
            assert!(alloc.allocated <= alloc.requested);
            assert!(alloc.allocated <= result.available_before);
        }
    }
}

#[test]
fn owner_silo_disburse_down_preserves_scenario_authority() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let writeback = writeback_results_for_spec(&spec);
    let demands = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("demands");
    let _results =
        apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("allocate");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}

#[test]
fn owner_silo_disburse_down_empty_demands_defer_or_zero_plan() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let buckets = owner_silo_demand_buckets_from_planet_child_rf(&spec).expect("buckets");
    assert!(buckets.is_empty());
    let writeback = writeback_results_for_spec(&spec);
    let results = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &buckets).expect("empty");
    assert!(results.is_empty());
}

#[test]
#[ignore = "manual corpus regeneration only"]
fn normal_tests_do_not_write_disburse_down_fixture() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let json = simthing_spec::serialize_scenario_authority(&spec).expect("serialize");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/owner_silo_disburse_down_scoped.simthing-scenario.json");
    std::fs::write(path, json).expect("write");
}
