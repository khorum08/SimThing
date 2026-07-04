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
