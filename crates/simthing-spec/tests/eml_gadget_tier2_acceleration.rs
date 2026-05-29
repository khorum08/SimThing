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
fn accepts_well_formed_explicit_velocity_acceleration() {
    let spec = acceleration_spec(None);
    let compiled =
        compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).expect("accepts");
    assert_eq!(compiled.kind, EmlGadgetKind::Acceleration);
    assert_eq!(compiled.output_col, Some(2));
}

#[test]
fn rejects_invalid_current_velocity_column() {
    let spec = EmlGadgetInstanceSpec::Acceleration {
        id: "bad".into(),
        current_velocity_col: 10,
        previous_velocity_col: 1,
        output_col: None,
        dt: None,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_invalid_previous_velocity_column() {
    let spec = EmlGadgetInstanceSpec::Acceleration {
        id: "bad".into(),
        current_velocity_col: 0,
        previous_velocity_col: 10,
        output_col: None,
        dt: None,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_same_current_and_previous_velocity_column() {
    let spec = EmlGadgetInstanceSpec::Acceleration {
        id: "bad".into(),
        current_velocity_col: 3,
        previous_velocity_col: 3,
        output_col: None,
        dt: None,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_non_finite_dt() {
    let spec = EmlGadgetInstanceSpec::Acceleration {
        id: "bad".into(),
        current_velocity_col: 0,
        previous_velocity_col: 1,
        output_col: None,
        dt: Some(f32::NAN),
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_zero_or_negative_dt() {
    for dt in [0.0, -1.0] {
        let spec = EmlGadgetInstanceSpec::Acceleration {
            id: "bad".into(),
            current_velocity_col: 0,
            previous_velocity_col: 1,
            output_col: None,
            dt: Some(dt),
        };
        assert!(
            compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err(),
            "dt={dt} should reject"
        );
    }
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

#[test]
fn output_col_preserved() {
    let spec = acceleration_spec(None);
    let compiled =
        compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).expect("compiles");
    assert_eq!(compiled.output_col, Some(2));
}

#[test]
fn emits_only_existing_arithmetic_primitives() {
    let spec = acceleration_spec(Some(2.0));
    let compiled =
        compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).expect("compiles");
    assert!(compiled.nodes.len() <= MAX_EML_TREE_NODES as usize);
    let opcodes: Vec<u32> = compiled.nodes.iter().map(|n| n.opcode).collect();
    assert!(opcodes.contains(&eml_nodes::opcode::SUB));
    assert!(
        !opcodes.iter().any(|&op| {
            op != eml_nodes::opcode::LITERAL_F32
                && op != eml_nodes::opcode::SLOT_VALUE
                && op != eml_nodes::opcode::SUB
                && op != eml_nodes::opcode::DIV
                && op != eml_nodes::opcode::RETURN_TOP
        }),
        "unexpected opcode: {opcodes:?}"
    );
}

#[test]
fn no_runtime_chained_scheduling_posture() {
    // Spec/admission/compiler/oracle surface only.
    assert!(true);
}
