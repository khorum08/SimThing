//! OWNER-SILO-RUNTIME-WRITEBACK-0 — writeback input derivation and CPU oracle proofs.

mod reduce_up_fixture;

use simthing_spec::{
    apply_owner_silo_runtime_writeback_cpu, evaluate_planet_child_rf_reduce_up,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, serialize_scenario_authority, OwnerRef, ResourceKey,
    RuntimeOwnerSiloState, RuntimeOwnerSiloWritebackErrorKind, RuntimeOwnerSiloWritebackInput,
    ScopeId, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

use reduce_up_fixture::build_planet_child_rf_reduce_up_scoped_spec;

#[test]
fn owner_silo_writeback_inputs_group_by_owner_resource() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("inputs");
    assert_eq!(inputs.len(), 2);

    let owner_a = inputs
        .iter()
        .find(|i| i.owner_ref.as_str() == "owner_a")
        .expect("owner_a");
    assert_eq!(
        owner_a.resource_key.as_str(),
        PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY
    );
    assert_eq!(owner_a.net_surplus, 12);
    assert_eq!(owner_a.net_deficit, 0);
    assert_eq!(owner_a.source_bucket_count, 1);

    let owner_b = inputs
        .iter()
        .find(|i| i.owner_ref.as_str() == "owner_b")
        .expect("owner_b");
    assert_eq!(owner_b.net_surplus, 5);
    assert_eq!(owner_b.net_deficit, 0);
}

#[test]
fn owner_silo_writeback_inputs_keep_planet_scopes_as_sources_not_channels() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    assert_eq!(reduce_up.bucket_count, 2);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("inputs");
    assert_eq!(inputs.len(), 2);
    assert!(inputs.iter().all(|i| i.source_bucket_count == 1));
}

#[test]
fn owner_silo_writeback_inputs_reject_unknown_owner_ref() {
    let initial = vec![RuntimeOwnerSiloState {
        owner_ref: OwnerRef::new("owner_a"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        current: 50,
        capacity: Some(100),
    }];
    let inputs = vec![RuntimeOwnerSiloWritebackInput {
        owner_ref: OwnerRef::new("owner_missing"),
        resource_key: ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY),
        net_surplus: 1,
        net_deficit: 0,
        source_bucket_count: 1,
    }];
    let err = apply_owner_silo_runtime_writeback_cpu(&initial, &inputs).unwrap_err();
    assert_eq!(
        err.kind,
        RuntimeOwnerSiloWritebackErrorKind::UnknownOwnerReference
    );
}

#[test]
fn owner_silo_writeback_inputs_do_not_mutate_scenario_authority() {
    let spec = build_planet_child_rf_reduce_up_scoped_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize");
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("inputs");
    let initial = runtime_owner_silo_states_from_scenario(&spec).expect("initial");
    let _results = apply_owner_silo_runtime_writeback_cpu(&initial, &inputs).expect("apply");
    let after = serialize_scenario_authority(&spec).expect("serialize");
    assert_eq!(before, after);
}
