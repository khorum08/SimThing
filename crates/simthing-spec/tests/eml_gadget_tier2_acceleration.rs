//! Phase M EML-GADGET-2E — explicit velocity-column Acceleration
//!
//! Spec/admission/compiler/oracle only. No position-history acceleration.

use simthing_core::{eml_nodes, MAX_EML_TREE_NODES};
use simthing_spec::{
    compile_eml_gadget, eval_eml_postfix, oracle_acceleration, EmlGadgetCompileOptions,
    EmlGadgetInstanceSpec, EmlGadgetKind,
};

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

fn acceleration_spec(dt: Option<f32>) -> EmlGadgetInstanceSpec {
    EmlGadgetInstanceSpec::Acceleration {
        id: "accel".into(),
        current_velocity_col: 0,
        previous_velocity_col: 1,
        output_col: Some(2),
        dt,
    }
}

fn eval_acceleration(spec: &EmlGadgetInstanceSpec, current_v: f32, previous_v: f32) -> f32 {
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let compiled = compile_eml_gadget(spec, opts).expect("compiles");
    let mut values = vec![0.0f32; (N_DIMS * 2) as usize];
    values[(EVAL_SLOT * N_DIMS) as usize] = current_v;
    values[(EVAL_SLOT * N_DIMS + 1) as usize] = previous_v;
    eval_eml_postfix(&compiled.nodes, EVAL_SLOT, &values, N_DIMS)
}

fn assert_f32_eq(got: f32, expected: f32, ctx: &str) {
    assert!(
        (got - expected).abs() <= 1e-6,
        "{ctx}: got {got}, expected {expected}"
    );
}

#[test]
fn compiled_parity_dt_omitted() {
    let spec = acceleration_spec(None);
    let got = eval_acceleration(&spec, 3.0, 1.0);
    let expected = oracle_acceleration(3.0, 1.0, 1.0);
    assert_f32_eq(got, expected, "dt omitted");
}

#[test]
fn compiled_parity_dt_provided() {
    let spec = acceleration_spec(Some(2.0));
    let got = eval_acceleration(&spec, 5.0, 1.0);
    let expected = oracle_acceleration(5.0, 1.0, 2.0);
    assert_f32_eq(got, expected, "dt=2");
}
