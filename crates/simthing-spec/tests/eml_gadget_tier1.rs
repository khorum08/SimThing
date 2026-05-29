//! Phase M EML-GADGET-1 — Tier-1 stateless gadget registry, compiler, and CPU oracle parity.

use simthing_core::{EmlExecutionClass, MAX_EML_TREE_NODES};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, deserialize_eml_gadget_stack_ron, eval_eml_postfix,
    oracle_field_sampler, oracle_soft_step, oracle_weighted_accumulator, reject_unknown_gadget_kind,
    CompiledEmlGadgetStack, DEFERRED_GADGET_KINDS, EmlGadgetCompileOptions, EmlGadgetInstanceSpec,
    EmlGadgetKind, EmlGadgetRegistry, EmlGadgetStackSpec, MappingExecutionProfile,
    ResourceFlowExecutionProfile, SpecError,
};

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

fn eval_gadget(
    stack: &CompiledEmlGadgetStack,
    gadget_index: usize,
    values: &[f32],
) -> f32 {
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

#[test]
fn tier1_registry_contains_all_gadgets() {
    let registry = EmlGadgetRegistry::new();
    for kind in registry.tier1_kinds() {
        assert!(registry.is_registered(*kind));
        assert_eq!(kind.execution_class(), EmlExecutionClass::ExactDeterministic);
        assert!(!kind.requires_temporal_memory());
    }
    assert_eq!(registry.available_names(), vec![
        "FieldSampler",
        "WeightedAccumulator",
        "SoftStep",
    ]);
}

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

    let cases = [(-10.0, 0.0), (0.0, 0.0), (60.0, 0.5), (120.0, 1.0), (180.0, 1.0)];
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

    let at_center = eval_gadget(
        &compiled,
        0,
        &{
            let mut v = vec![0.0; N_DIMS as usize];
            set_col(&mut v, 20, center);
            v
        },
    );
    assert_f32_eq(at_center, 0.5, "center");

    let below = oracle_soft_step(center - 0.2, center, steepness);
    let above = oracle_soft_step(center + 0.2, center, steepness);
    assert!(below < 0.5);
    assert!(above > 0.5);
    assert!(oracle_soft_step(100.0, center, steepness) <= 1.0);
    assert!(oracle_soft_step(-100.0, center, steepness) >= 0.0);
}

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

#[test]
fn ron_gadget_stack_admits() {
    let spec = deserialize_eml_gadget_stack_ron(TIER1_STACK_RON).expect("RON parses");
    let compiled = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("stack compiles");

    assert_eq!(compiled.report.gadget_count, 3);
    assert_eq!(
        compiled.report.gadget_ids,
        vec!["sample_threat", "soft_desperation", "urgency"]
    );
    assert!(compiled.report.total_node_count <= MAX_EML_TREE_NODES as usize);

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

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
    assert_eq!(compiled.report.execution_class, EmlExecutionClass::ExactDeterministic);
    assert!(compiled.report.total_node_count <= MAX_EML_TREE_NODES as usize);
}

// ── Test 7 — invalid unknown gadget rejects ──────────────────────────────────

#[test]
fn unknown_gadget_kind_rejects() {
    let err = reject_unknown_gadget_kind("NotAGadget", "bad_id").unwrap_err();
    match err {
        SpecError::EmlGadgetAdmission { gadget, reason } => {
            assert_eq!(gadget, "bad_id");
            assert!(reason.contains("unknown gadget kind"));
            assert!(reason.contains("NotAGadget"));
            assert!(reason.contains("FieldSampler"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// ── Test 8 — invalid params reject clearly ───────────────────────────────────

#[test]
fn invalid_params_reject() {
    let reject = |spec: EmlGadgetStackSpec, needle: &str| {
        let err = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(needle), "expected `{needle}` in `{msg}`");
    };

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::FieldSampler {
                id: "bad_cap".into(),
                input_col: 1,
                output_col: None,
                cap: 0.0,
            }],
        },
        "cap must be finite and > 0",
    );

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::FieldSampler {
                id: "nan_cap".into(),
                input_col: 1,
                output_col: None,
                cap: f32::NAN,
            }],
        },
        "cap must be finite and > 0",
    );

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::SoftStep {
                id: "nan_center".into(),
                input_col: 1,
                output_col: None,
                center: f32::INFINITY,
                steepness: 1.0,
            }],
        },
        "center must be finite",
    );

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::SoftStep {
                id: "bad_steep".into(),
                input_col: 1,
                output_col: None,
                center: 0.5,
                steepness: 0.0,
            }],
        },
        "steepness must be finite and > 0",
    );

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::WeightedAccumulator {
                id: "empty".into(),
                input_cols: vec![],
                weight_cols: vec![],
                output_col: None,
            }],
        },
        "at least one input",
    );

    reject(
        EmlGadgetStackSpec {
            gadgets: vec![EmlGadgetInstanceSpec::WeightedAccumulator {
                id: "mismatch".into(),
                input_cols: vec![1, 2],
                weight_cols: vec![3],
                output_col: None,
            }],
        },
        "input count",
    );
}

// ── Test 9 — deferred temporal gadgets rejected ──────────────────────────────

#[test]
fn deferred_gadget_kinds_not_registered() {
    for kind in DEFERRED_GADGET_KINDS {
        assert!(EmlGadgetKind::parse(kind).is_none());
        let err = reject_unknown_gadget_kind(kind, "deferred").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("deferred") || msg.contains("unknown"), "{msg}");
    }
}

// ── Test 10 — posture preservation ───────────────────────────────────────────

#[test]
fn posture_preservation() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let gpu_lib = std::fs::read_to_string(repo_root.join("crates/simthing-gpu/src/lib.rs"))
        .expect("simthing-gpu lib.rs");
    assert!(!gpu_lib.contains("EmlGadget"));
    assert!(!gpu_lib.contains("FieldSampler"));

    let sim_lib = std::fs::read_to_string(repo_root.join("crates/simthing-sim/src/lib.rs"))
        .expect("simthing-sim lib.rs");
    assert!(!sim_lib.contains("EmlGadget"));
    assert!(!sim_lib.contains("Personality"));

    let wgsl_dir = repo_root.join("crates/simthing-gpu/src/shaders");
    for entry in std::fs::read_dir(&wgsl_dir).expect("shaders dir") {
        let path = entry.expect("dir entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        assert!(
            !name.contains("gadget") && !name.contains("field_sampler") && !name.contains("soft_step"),
            "unexpected gadget WGSL: {name}"
        );
    }

    let core_nodes = std::fs::read_to_string(repo_root.join("crates/simthing-core/src/eml_nodes.rs"))
        .expect("eml_nodes.rs");
    let opcode_count = core_nodes.matches("pub const ").count();
    assert!(opcode_count > 0, "opcode table present");
    assert!(!core_nodes.contains("EXP"));
    assert!(!core_nodes.contains("LOGISTIC"));
}
