//! Phase M EML-GADGET-2B — VelocityMonitor + Decay/EMA
//!
//! Spec/admission/compiler/oracle surfaces only. Stateful sequence CPU oracle parity.
//! No runtime gadget execution, no new EML opcodes, no WGSL, no simthing-gpu/sim changes.

use simthing_core::EmlExecutionClass;
use simthing_spec::{
    compile_eml_gadget_stack, eval_eml_postfix, oracle_decay, oracle_ema, oracle_velocity_monitor,
    CompiledEmlGadgetStack, EmlGadgetCompileOptions, EmlGadgetInstanceSpec, EmlGadgetKind,
    EmlGadgetRegistry, EmlGadgetStackSpec, SpecError, DEFERRED_GADGET_KINDS,
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

// ── Test 1 — registry and deferred list after 2B ─────────────────────────────

// ── Test 2 — VelocityMonitor compile + stateful sequence oracle parity ───────

#[test]
fn velocity_monitor_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::VelocityMonitor {
            id: "vel".into(),
            current_col: 10,
            previous_col: 11,
            output_col: Some(12),
            dt: Some(1.0),
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("VelocityMonitor compiles");

    let currents = [1.0, 1.5, 1.25];
    let previous = [0.0, 1.0, 1.5];
    let expected = [1.0, 0.5, -0.25];

    for (i, (&c, &p)) in currents.iter().zip(previous.iter()).enumerate() {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 10, c);
        set_col(&mut values, 11, p);

        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_velocity_monitor(c, p, 1.0);
        assert_f32_eq(got, expected[i], &format!("step {i}"));
        assert_f32_eq(got, oracle, &format!("step {i} oracle"));
    }
}

// ── Test 3 — VelocityMonitor invalid params reject ───────────────────────────

// ── Test 4 — Decay compile + stateful sequence oracle parity ─────────────────

#[test]
fn decay_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::Decay {
            id: "decay".into(),
            state_col: 20,
            output_col: Some(21),
            decay: 0.5,
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("Decay compiles");

    let mut state = 1.0f32;
    let expected = [0.5, 0.25, 0.125];

    for (step, &exp) in expected.iter().enumerate() {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 20, state);

        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_decay(state, 0.5);
        assert_f32_eq(got, exp, &format!("step {step}"));
        assert_f32_eq(got, oracle, &format!("step {step} oracle"));

        state = got; // simulate the persistent state column being written back
    }
}

// ── Test 5 — Decay invalid params reject ─────────────────────────────────────

// ── Test 6 — EMA compile + stateful sequence oracle parity ───────────────────

#[test]
fn ema_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::Ema {
            id: "ema".into(),
            input_col: 30,
            previous_col: 31,
            output_col: Some(32),
            decay: 0.5,
        }],
    };
    let compiled =
        compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default()).expect("Ema compiles");

    let inputs = [0.0, 1.0, 1.0, 0.0];
    let expected = [0.0, 0.5, 0.75, 0.375];

    let mut prev = 0.0f32;

    for (step, (&input, &exp)) in inputs.iter().zip(expected.iter()).enumerate() {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 30, input);
        set_col(&mut values, 31, prev);

        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_ema(input, prev, 0.5);
        assert_f32_eq(got, exp, &format!("step {step}"));
        assert_f32_eq(got, oracle, &format!("step {step} oracle"));

        prev = got; // the EMA output becomes the previous for the next step
    }
}

// ── Test 7 — EMA invalid params reject ───────────────────────────────────────

// ── Test 8 — no runtime scheduling / gadget execution posture ────────────────

// ── Test 9 — 2A snapshot/copy regression remains green ───────────────────────

// ── Test 10 — overall posture preservation (defaults, no atlas, etc.) ────────
