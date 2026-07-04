//! PLANET-CHILD-RECURSIVE-RF-RECONCILIATION-0 — reconciliation spec proofs.

mod disburse_down_fixture;
mod sibling_redistribution_fixture;

use simthing_spec::{
    project_planet_child_rf_ladder_rows, project_recursive_local_rf_rows,
    prove_recursive_rf_reconciliation_preserves_authority,
    reconcile_planet_child_rf_with_recursive_local_rf, RecursiveRfReconciliationMismatchKind,
    OWNER_FLOW_DEFAULT_RESOURCE_KEY, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;
use sibling_redistribution_fixture::{build_sibling_redistribution_spec, star_system_id_raw};
#[test]
fn recursive_rf_reconciliation_mismatch_report_is_deterministic() {
    let spec = build_sibling_redistribution_spec();
    let first = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("first");
    let second = reconcile_planet_child_rf_with_recursive_local_rf(&spec).expect("second");

    assert_eq!(first.mismatches, second.mismatches);
    assert_eq!(first.buckets, second.buckets);
}
