//! RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick boundary proofs.

mod disburse_down_fixture;
mod reduce_up_fixture;

use simthing_core::{PropertyValue, SimThingKind};
use simthing_spec::{
    evaluate_runtime_rf_tick, serialize_scenario_authority, RuntimeRfTickErrorKind,
    OWNER_SILO_CURRENT_PROPERTY_ID, PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

#[test]
fn runtime_rf_tick_handles_empty_demands_deterministically() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let report = evaluate_runtime_rf_tick(&spec).expect("tick");

    assert!(report.owner_silo_disburse_down_ready);
    assert!(report.runtime_local_allocation_ready);
    assert_eq!(report.disburse_down_result_count, 0);
    assert_eq!(report.local_allocation_count, 0);
    assert_eq!(report.local_allocated_total, 0);
}
