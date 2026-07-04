//! Phase M EML-GADGET-1 — Tier-1 stateless gadget registry, compiler, and CPU oracle parity.

use simthing_core::{EmlExecutionClass, MAX_EML_TREE_NODES};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, deserialize_eml_gadget_stack_ron, eval_eml_postfix,
    oracle_field_sampler, oracle_soft_step, oracle_weighted_accumulator,
    reject_unknown_gadget_kind, CompiledEmlGadgetStack, EmlGadgetCompileOptions,
    EmlGadgetCompositionPlan, EmlGadgetInstanceSpec, EmlGadgetKind, EmlGadgetRegistry,
    EmlGadgetStackSpec, MappingExecutionProfile, ResourceFlowExecutionProfile, SpecError,
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

fn get_col(values: &[f32], col: u32) -> f32 {
    values[(EVAL_SLOT * N_DIMS + col) as usize]
}

fn assert_f32_eq(got: f32, expected: f32, ctx: &str) {
    assert!(
        (got - expected).abs() <= 1e-6,
        "{ctx}: got {got}, expected {expected}"
    );
}

// ── Test 1 — registry contains Tier-1 gadgets ────────────────────────────────

// ── Test 2 — FieldSampler oracle parity ──────────────────────────────────────

#[test]
fn field_sampler_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::FieldSampler {
            id: "sample".into(),
            input_col: 12,
            output_col: Some(20),
            cap: 120.0,
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("FieldSampler compiles");

    let cases = [
        (-10.0, 0.0),
        (0.0, 0.0),
        (60.0, 0.5),
        (120.0, 1.0),
        (180.0, 1.0),
    ];
    for (input, expected) in cases {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 12, input);
        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_field_sampler(input, 120.0);
        assert_eq!(got, expected, "input={input}");
        assert_eq!(got, oracle, "oracle mismatch input={input}");
    }
}

// ── Test 3 — WeightedAccumulator oracle parity ───────────────────────────────

#[test]
fn weighted_accumulator_oracle_parity() {
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::WeightedAccumulator {
            id: "urgency".into(),
            input_cols: vec![21, 22, 23],
            weight_cols: vec![30, 31, 32],
            output_col: Some(40),
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("WeightedAccumulator compiles");

    let cases: [([f32; 3], [f32; 3]); 4] = [
        ([0.2, 0.7, 1.0], [2.0, 3.0, -1.0]),
        ([1.0, 0.0, 0.5], [1.0, 1.0, 2.0]),
        ([0.5, 0.5, 0.5], [0.0, 0.0, 4.0]),
        ([0.1, 0.2, 0.3], [-1.0, 2.0, 0.5]),
    ];

    for (inputs, weights) in cases {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 21, inputs[0]);
        set_col(&mut values, 22, inputs[1]);
        set_col(&mut values, 23, inputs[2]);
        set_col(&mut values, 30, weights[0]);
        set_col(&mut values, 31, weights[1]);
        set_col(&mut values, 32, weights[2]);

        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_weighted_accumulator(&inputs, &weights);
        assert_f32_eq(got, oracle, "weighted accumulator parity");
    }
}

// ── Test 4 — SoftStep oracle parity ──────────────────────────────────────────

#[test]
fn soft_step_oracle_parity() {
    let center = 0.65;
    let steepness = 6.0;
    let spec = EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::SoftStep {
            id: "soft".into(),
            input_col: 20,
            output_col: Some(21),
            center,
            steepness,
        }],
    };
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("SoftStep compiles");

    let xs = [0.0, 0.65, 0.9, 10.0, -10.0];
    for x in xs {
        let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 20, x);
        let got = eval_gadget(&compiled, 0, &values);
        let oracle = oracle_soft_step(x, center, steepness);
        assert_f32_eq(got, oracle, &format!("x={x}"));
    }

    let at_center = eval_gadget(&compiled, 0, &{
        let mut v = vec![0.0; N_DIMS as usize];
        set_col(&mut v, 20, center);
        v
    });
    assert_f32_eq(at_center, 0.5, "center");

    let below = oracle_soft_step(center - 0.2, center, steepness);
    let above = oracle_soft_step(center + 0.2, center, steepness);
    assert!(below < 0.5);
    assert!(above > 0.5);
    assert!(oracle_soft_step(100.0, center, steepness) <= 1.0);
    assert!(oracle_soft_step(-100.0, center, steepness) >= 0.0);
}

// ── R1 Test 1 — single-gadget flatten preview is executable ─────────────────

// ── R2 Test 1 — multi-gadget total over cap admits as PerGadgetOnly ───────────

// ── R2 Test 2 — single gadget over cap still rejects ────────────────────────

// ── Test 5 — RON gadget stack admits ─────────────────────────────────────────

const TIER1_STACK_RON: &str = r#"
(
    gadgets: [
        (
            kind: "FieldSampler",
            id: "sample_threat",
            input_col: 12,
            output_col: 20,
            cap: 120.0,
        ),
        (
            kind: "SoftStep",
            id: "soft_desperation",
            input_col: 20,
            output_col: 21,
            center: 0.65,
            steepness: 6.0,
        ),
        (
            kind: "WeightedAccumulator",
            id: "urgency",
            input_cols: [21, 22],
            weight_cols: [30, 31],
            output_col: 40,
        ),
    ],
)
"#;

// ── Test 6 — stack composition parity ────────────────────────────────────────

#[test]
fn stack_composition_oracle_parity() {
    let spec = deserialize_eml_gadget_stack_ron(TIER1_STACK_RON).expect("RON parses");
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("stack compiles");

    let mut values = vec![0.0; (N_DIMS * (EVAL_SLOT + 1)) as usize];
    set_col(&mut values, 12, 90.0);
    set_col(&mut values, 22, 0.4);
    set_col(&mut values, 30, 0.8);
    set_col(&mut values, 31, 0.2);

    let sampled = eval_gadget(&compiled, 0, &values);
    assert_eq!(sampled, oracle_field_sampler(90.0, 120.0));
    set_col(&mut values, 20, sampled);

    let softened = eval_gadget(&compiled, 1, &values);
    assert_eq!(softened, oracle_soft_step(sampled, 0.65, 6.0));
    set_col(&mut values, 21, softened);

    let accumulated = eval_gadget(&compiled, 2, &values);
    let manual = oracle_weighted_accumulator(
        &[softened, get_col(&values, 22)],
        &[get_col(&values, 30), get_col(&values, 31)],
    );
    assert_f32_eq(accumulated, manual, "stack composition");
    assert_eq!(
        compiled.report.execution_class,
        EmlExecutionClass::ExactDeterministic
    );

    // R1: multi-gadget chained stack must not expose executable flatten preview.
    assert!(!compiled.composition.flatten_preview_executable());
    assert!(compiled.composition.flatten_preview_nodes().is_none());
    match &compiled.composition {
        EmlGadgetCompositionPlan::PerGadgetOnly { reason } => {
            assert!(reason.contains("OrderBand") || reason.contains("deferred"));
        }
        other => panic!("expected PerGadgetOnly for chained stack, got {other:?}"),
    }
    assert!(
        compiled
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "chained_runtime_deferred"),
        "expected chained_runtime_deferred diagnostic"
    );
}

// ── R1 Test 3 — no runtime consumption of flatten preview ────────────────────

fn walkdir_light(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out
}

// ── Test 7 — invalid unknown gadget rejects ──────────────────────────────────

// ── Test 8 — invalid params reject clearly ───────────────────────────────────

// ── Test 9 — deferred temporal gadgets rejected ──────────────────────────────

// ── Test 10 — posture preservation ───────────────────────────────────────────
