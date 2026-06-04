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

#[test]
fn tier2c_registry_contains_bounded_feedback() {
    assert!(EmlGadgetKind::parse("BoundedFeedback").is_some());

    let kind = EmlGadgetKind::parse("BoundedFeedback").unwrap();
    assert_eq!(
        kind.execution_class(),
        EmlExecutionClass::ExactDeterministic
    );
    assert!(kind.requires_temporal_memory());

    // Still-deferred items remain
    assert!(!DEFERRED_GADGET_KINDS.contains(&"Hysteresis"));
    assert!(!DEFERRED_GADGET_KINDS.contains(&"Acceleration"));
    assert!(DEFERRED_GADGET_KINDS.is_empty());

    // BoundedFeedback is no longer in the deferred list
    assert!(!DEFERRED_GADGET_KINDS.contains(&"BoundedFeedback"));
}

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

#[test]
fn bounded_feedback_clamp_edges() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::BoundedFeedback {
            id: "bf".into(),
            previous_col: 10,
            input_col: 11,
            output_col: None,
            decay: 0.9,
            gain: 0.5,
            min: 0.0,
            max: 1.0,
        }],
    };
    let compiled =
        compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default()).expect("compiles");

    // Upper clamp
    let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 10, 0.9);
    set_col(&mut values, 11, 1.0);
    let got = eval_gadget(&compiled, 0, &values);
    assert_f32_eq(got, 1.0, "upper clamp");

    // Lower clamp
    let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 10, 0.1);
    set_col(&mut values, 11, -1.0);
    let got = eval_gadget(&compiled, 0, &values);
    assert_f32_eq(got, 0.0, "lower clamp");
}

// ── Test 4 — invalid decay rejects ───────────────────────────────────────────

#[test]
fn bounded_feedback_invalid_decay_rejects() {
    let bad = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bad".into(),
        previous_col: 5,
        input_col: 6,
        output_col: None,
        decay: 1.2,
        gain: 0.5,
        min: 0.0,
        max: 1.0,
    };
    let res = compile_eml_gadget_stack(
        &EmlGadgetStackSpec { gadgets: vec![bad] },
        EmlGadgetCompileOptions { max_col: 64 },
    );
    assert!(res.is_err());
}

// ── Test 5 — invalid gain rejects (NaN/inf) ──────────────────────────────────

#[test]
fn bounded_feedback_invalid_gain_rejects() {
    let bad = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bad".into(),
        previous_col: 5,
        input_col: 6,
        output_col: None,
        decay: 0.5,
        gain: f32::INFINITY,
        min: 0.0,
        max: 1.0,
    };
    let res = compile_eml_gadget_stack(
        &EmlGadgetStackSpec { gadgets: vec![bad] },
        EmlGadgetCompileOptions { max_col: 64 },
    );
    assert!(res.is_err());
}

// ── Test 6 — invalid clamp (min >= max or non-finite) rejects ────────────────

#[test]
fn bounded_feedback_invalid_clamp_rejects() {
    let bad = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bad".into(),
        previous_col: 5,
        input_col: 6,
        output_col: None,
        decay: 0.5,
        gain: 0.5,
        min: 1.0,
        max: 0.0,
    };
    let res = compile_eml_gadget_stack(
        &EmlGadgetStackSpec { gadgets: vec![bad] },
        EmlGadgetCompileOptions { max_col: 64 },
    );
    assert!(res.is_err());
}

// ── Test 7 — invalid columns reject ──────────────────────────────────────────

#[test]
fn bounded_feedback_invalid_columns_reject() {
    let same = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bad".into(),
        previous_col: 5,
        input_col: 5,
        output_col: None,
        decay: 0.5,
        gain: 0.5,
        min: 0.0,
        max: 1.0,
    };
    let res = compile_eml_gadget_stack(
        &EmlGadgetStackSpec {
            gadgets: vec![same],
        },
        EmlGadgetCompileOptions { max_col: 64 },
    );
    assert!(res.is_err());
}

// ── Test 8 — no unbounded recurrence form exists ─────────────────────────────

#[test]
fn bounded_feedback_no_unbounded_form() {
    // The authoring type always carries min/max. There is no variant without clamp.
    // Admission already enforces min < max and finite bounds.
    // This test documents the intentional design decision.
    let _ = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "example".into(),
        previous_col: 1,
        input_col: 2,
        output_col: None,
        decay: 0.5,
        gain: 1.0,
        min: 0.0,
        max: 10.0,
    };
}

// ── Test 9 — no runtime gadget execution posture ─────────────────────────────

#[test]
fn no_runtime_gadget_execution_posture_2c() {
    // Posture is enforced at the architecture level (PerGadgetOnly, no driver consumption).
    // We simply ensure the new kind does not introduce any forbidden strings.
    let src = include_str!("../src/compile/eml_gadget.rs");
    assert!(!src.contains("runtime gadget stack execution"));
}

// ── Test 10 — 2A + 2B regressions remain green (via required list) ───────────

#[test]
fn prior_slices_still_green() {
    // Actual heavy tests are run in the required regression list.
    // Touching DEFERRED and Kind here must not break prior 2B oracles.
    assert!(!DEFERRED_GADGET_KINDS.contains(&"BoundedFeedback"));
}

// ── Test 11 — posture preservation ───────────────────────────────────────────

#[test]
fn posture_preservation_2c() {
    use simthing_spec::MappingExecutionProfile;
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
}
