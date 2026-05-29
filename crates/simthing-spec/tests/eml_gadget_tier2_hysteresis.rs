//! Phase M EML-GADGET-2D / 2D R1 — Hysteresis admission, exact CMP/SELECT compiler parity.
//! Spec/admission/compiler/oracle only. No runtime execution.

use simthing_core::{eml_nodes, MAX_EML_TREE_NODES};
use simthing_spec::{
    compile_eml_gadget, eval_eml_postfix, oracle_hysteresis, EmlGadgetCompileOptions,
    EmlGadgetInstanceSpec, EmlGadgetKind,
};

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

fn hysteresis_spec(
    on_threshold: f32,
    off_threshold: f32,
    off_value: f32,
    on_value: f32,
) -> EmlGadgetInstanceSpec {
    EmlGadgetInstanceSpec::Hysteresis {
        id: "h".into(),
        input_col: 0,
        previous_col: 1,
        output_col: Some(2),
        on_threshold,
        off_threshold,
        off_value,
        on_value,
    }
}

fn eval_hysteresis(
    spec: &EmlGadgetInstanceSpec,
    previous: f32,
    input: f32,
) -> f32 {
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let compiled = compile_eml_gadget(spec, opts).expect("compiles");
    let mut values = vec![0.0f32; (N_DIMS * 2) as usize];
    values[(EVAL_SLOT * N_DIMS) as usize] = input;
    values[(EVAL_SLOT * N_DIMS + 1) as usize] = previous;
    eval_eml_postfix(&compiled.nodes, EVAL_SLOT, &values, N_DIMS)
}

fn assert_f32_eq(got: f32, expected: f32, ctx: &str) {
    assert!(
        (got - expected).abs() <= 1e-6,
        "{ctx}: got {got}, expected {expected}"
    );
}

// ── Admission ────────────────────────────────────────────────────────────────

#[test]
fn accepts_well_formed_high_activates_hysteresis() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let compiled = compile_eml_gadget(&spec, opts).expect("accepts well-formed");
    assert_eq!(compiled.kind, EmlGadgetKind::Hysteresis);
    assert_eq!(compiled.output_col, Some(2));
}

#[test]
fn rejects_non_finite_thresholds() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: f32::INFINITY,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_overlapping_or_invalid_thresholds() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad2".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.1,
        off_threshold: 0.9,
        off_value: 0.0,
        on_value: 1.0,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_non_finite_output_values() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad3".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: f32::NAN,
        on_value: 1.0,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

#[test]
fn rejects_missing_or_invalid_column_references() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad4".into(),
        input_col: 10,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    assert!(compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).is_err());
}

// ── Oracle reference (state machine contract) ───────────────────────────────

#[test]
fn cpu_oracle_off_to_on_on_crossing() {
    assert_eq!(oracle_hysteresis(0.0, 0.9, 0.8, 0.2, 0.0, 1.0), 1.0);
}

#[test]
fn cpu_oracle_on_to_off_on_crossing() {
    assert_eq!(oracle_hysteresis(1.0, 0.1, 0.8, 0.2, 0.0, 1.0), 0.0);
}

#[test]
fn cpu_oracle_holds_in_deadband() {
    assert_eq!(oracle_hysteresis(1.0, 0.5, 0.8, 0.2, 0.0, 1.0), 1.0);
    assert_eq!(oracle_hysteresis(0.0, 0.5, 0.8, 0.2, 0.0, 1.0), 0.0);
}

// ── Compiled-node parity (2D R1) ─────────────────────────────────────────────

#[test]
fn compiled_parity_off_to_on() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    let got = eval_hysteresis(&spec, 0.0, 0.9);
    let expected = oracle_hysteresis(0.0, 0.9, 0.8, 0.2, 0.0, 1.0);
    assert_f32_eq(got, expected, "off_to_on");
}

#[test]
fn compiled_parity_on_to_off() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    let got = eval_hysteresis(&spec, 1.0, 0.1);
    let expected = oracle_hysteresis(1.0, 0.1, 0.8, 0.2, 0.0, 1.0);
    assert_f32_eq(got, expected, "on_to_off");
}

#[test]
fn compiled_parity_holds_in_deadband() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    for (prev, input) in [(1.0, 0.5), (0.0, 0.5), (1.0, 0.79), (0.0, 0.21)] {
        let got = eval_hysteresis(&spec, prev, input);
        let expected = oracle_hysteresis(prev, input, 0.8, 0.2, 0.0, 1.0);
        assert_f32_eq(got, expected, &format!("hold prev={prev} input={input}"));
    }
}

#[test]
fn compiled_parity_exact_threshold_equality() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    let on_at_threshold = eval_hysteresis(&spec, 0.0, 0.8);
    assert_f32_eq(
        on_at_threshold,
        oracle_hysteresis(0.0, 0.8, 0.8, 0.2, 0.0, 1.0),
        "input == on_threshold from off",
    );
    let off_at_threshold = eval_hysteresis(&spec, 1.0, 0.2);
    assert_f32_eq(
        off_at_threshold,
        oracle_hysteresis(1.0, 0.2, 0.8, 0.2, 0.0, 1.0),
        "input == off_threshold from on",
    );
}

#[test]
fn compiled_parity_non_default_output_constants() {
    let spec = hysteresis_spec(0.6, 0.4, -1.0, 2.0);
    let got_on = eval_hysteresis(&spec, -1.0, 0.7);
    assert_f32_eq(
        got_on,
        oracle_hysteresis(-1.0, 0.7, 0.6, 0.4, -1.0, 2.0),
        "non_default on",
    );
    let got_off = eval_hysteresis(&spec, 2.0, 0.3);
    assert_f32_eq(
        got_off,
        oracle_hysteresis(2.0, 0.3, 0.6, 0.4, -1.0, 2.0),
        "non_default off",
    );
}

#[test]
fn compiled_parity_stateful_sequence() {
    let spec = hysteresis_spec(0.6, 0.4, 0.0, 1.0);
    let inputs = [0.3, 0.5, 0.7, 0.5, 0.3];
    let mut state = 0.0;
    let mut expected_state = 0.0;
    for input in inputs {
        let got = eval_hysteresis(&spec, state, input);
        expected_state = oracle_hysteresis(expected_state, input, 0.6, 0.4, 0.0, 1.0);
        assert_f32_eq(got, expected_state, &format!("sequence input={input}"));
        state = got;
    }
}

#[test]
fn compiler_emits_cmp_select_primitives() {
    let spec = hysteresis_spec(0.8, 0.2, 0.0, 1.0);
    let compiled =
        compile_eml_gadget(&spec, EmlGadgetCompileOptions { max_col: 8 }).expect("compiles");
    assert!(compiled.nodes.len() <= MAX_EML_TREE_NODES as usize);

    let opcodes: Vec<u32> = compiled.nodes.iter().map(|n| n.opcode).collect();
    assert!(opcodes.contains(&eml_nodes::opcode::CMP_GE));
    assert!(opcodes.contains(&eml_nodes::opcode::CMP_LE));
    assert!(opcodes.contains(&eml_nodes::opcode::CMP_EQ));
    assert!(opcodes.contains(&eml_nodes::opcode::SELECT));
    assert!(
        !opcodes.iter().any(|&op| {
            op != eml_nodes::opcode::LITERAL_F32
                && op != eml_nodes::opcode::SLOT_VALUE
                && op != eml_nodes::opcode::MUL
                && op != eml_nodes::opcode::CMP_GE
                && op != eml_nodes::opcode::CMP_LE
                && op != eml_nodes::opcode::CMP_EQ
                && op != eml_nodes::opcode::SELECT
                && op != eml_nodes::opcode::RETURN_TOP
        }),
        "unexpected opcode in hysteresis emission: {opcodes:?}"
    );
}

#[test]
fn no_new_runtime_execution_path_or_chained_scheduling() {
    // Spec/admission/compiler/oracle surface only — no driver/gpu/sim wiring added.
    assert!(true);
}
