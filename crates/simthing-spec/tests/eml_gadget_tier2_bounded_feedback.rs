//! Phase M EML-GADGET-2C — BoundedFeedback
//!
//! Strict clamp-bounded recurrent accumulator.
//! Spec/admission/compiler/oracle only. Stateful sequence CPU oracle parity.

use simthing_core::EmlExecutionClass;
use simthing_spec::{
    compile_eml_gadget_stack, eval_eml_postfix, oracle_bounded_feedback, CompiledEmlGadgetStack,
    EmlGadgetCompileOptions, EmlGadgetInstanceSpec, EmlGadgetKind, EmlGadgetStackSpec, SpecError,
    DEFERRED_GADGET_KINDS,
};

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

fn eval_gadget(stack: &CompiledEmlGadgetStack, gadget_index: usize, values: &[f32]) -> f32 {
    let gadget = &stack.gadgets[gadget_index];
    eval_eml_postfix(&gadget.nodes, EVAL_SLOT, values, N_DIMS)
}

fn set_col(values: &mut [f32], col: u32, v: f32) {
    values[(EVAL_SLOT * N_DIMS + col) as usize] = v;
}

fn assert_f32_eq(got: f32, expected: f32, ctx: &str) {
    assert!(
        (got - expected).abs() <= 1e-6,
        "{ctx}: got {got}, expected {expected}"
    );
}

// ── Test 1 — registry after 2C ───────────────────────────────────────────────

// ── Test 2 — BoundedFeedback compile + stateful sequence oracle parity ───────

#[test]
fn bounded_feedback_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::BoundedFeedback {
            id: "bf".into(),
            previous_col: 40,
            input_col: 41,
            output_col: Some(42),
            decay: 0.8,
            gain: 0.5,
            min: 0.0,
            max: 1.0,
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("BoundedFeedback compiles");

    let inputs = [1.0, 1.0, 1.0, -2.0];
    let expected = [0.5, 0.9, 1.0, 0.0];

    let mut prev = 0.0f32;

    for (step, (&input, &exp)) in inputs.iter().zip(expected.iter()).enumerate() {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 40, prev);
        set_col(&mut values, 41, input);

        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_bounded_feedback(prev, input, 0.8, 0.5, 0.0, 1.0);

        assert_f32_eq(got, exp, &format!("step {step}"));
        assert_f32_eq(got, oracle, &format!("step {step} oracle"));

        prev = got; // stateful: output becomes previous for next step
    }
}

// ── Test 3 — explicit upper and lower clamp behavior ─────────────────────────

// ── Test 4 — invalid decay rejects ───────────────────────────────────────────

// ── Test 5 — invalid gain rejects (NaN/inf) ──────────────────────────────────

// ── Test 6 — invalid clamp (min >= max or non-finite) rejects ────────────────

// ── Test 7 — invalid columns reject ──────────────────────────────────────────

// ── Test 8 — no unbounded recurrence form exists ─────────────────────────────

// ── Test 9 — no runtime gadget execution posture ─────────────────────────────

// ── Test 10 — 2A + 2B regressions remain green (via required list) ───────────

// ── Test 11 — posture preservation ───────────────────────────────────────────
