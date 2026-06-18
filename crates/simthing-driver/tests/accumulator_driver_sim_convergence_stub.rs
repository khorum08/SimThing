//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 — compile-plan ownership stub (test/doc only).
//!
//! Future vertical-seed structural neighbor accumulation must be assembled here in `simthing-driver`
//! into canonical `AccumulatorOp` plans from `SimThingScenarioSpec` structural links, then dispatched
//! under `simthing-sim` tick/boundary ownership. Studio proof helpers must not become the runtime path.

use simthing_gpu::{
    DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE, SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE,
    STRUCTURAL_NEIGHBOR_SUM_INVARIANT,
};

#[test]
fn driver_compile_plan_stub_names_driver_as_structural_accumulator_owner() {
    assert_eq!(
        DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE,
        "simthing-driver"
    );
    assert_eq!(SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE, "simthing-sim");
    assert!(STRUCTURAL_NEIGHBOR_SUM_INVARIANT.contains("output[a]"));
    assert!(STRUCTURAL_NEIGHBOR_SUM_INVARIANT.contains("input[b]"));
}
