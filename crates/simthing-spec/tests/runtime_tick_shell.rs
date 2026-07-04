//! RUNTIME-TICK-EXECUTION-SHELL-0 — runtime tick shell spec proofs.

mod disburse_down_fixture;

use simthing_core::SimThingKind;
use simthing_spec::{
    evaluate_runtime_tick_shell, runtime_tick_shell_stage_order, serialize_scenario_authority,
    RuntimeTickId, RuntimeTickShellErrorKind, RuntimeTickStage, OWNER_SILO_CURRENT_PROPERTY_ID,
    PLANET_ID_PROPERTY_ID,
};

use disburse_down_fixture::build_owner_silo_disburse_down_scoped_spec;

const TICK_ONE: RuntimeTickId = RuntimeTickId(1);

#[test]
fn runtime_tick_shell_records_deterministic_stage_order() {
    let spec = build_owner_silo_disburse_down_scoped_spec();
    let report = evaluate_runtime_tick_shell(&spec, TICK_ONE).expect("shell");
    let expected = runtime_tick_shell_stage_order();

    assert_eq!(report.stage_order, expected);
    assert_eq!(report.stage_count, 6);
    assert_eq!(
        report.stage_order[0],
        RuntimeTickStage::RuntimeRfTickComposition
    );
    assert_eq!(
        report.stage_order[5],
        RuntimeTickStage::RuntimeLocalAllocation
    );
}
