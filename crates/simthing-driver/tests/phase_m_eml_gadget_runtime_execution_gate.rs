//! Phase M EML-GADGET Runtime Execution Gate — minimal opt-in fixture over existing EvalEML
//! AccumulatorOp runtime substrate. No new opcode, no JIT, no chained scheduling.

use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    eval_eml_cpu, set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable, GpuContext,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_eml_gadget_stack, deserialize_eml_gadget_stack_ron, eval_eml_postfix,
    oracle_ema, oracle_weighted_accumulator, EmlGadgetCompileOptions, EmlGadgetKind,
    MappingExecutionProfile,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const WEIGHTED_ACC_RON: &str =
    include_str!("fixtures/eml_gadget_runtime_gate_weighted_accumulator.ron");
const EMA_RON: &str = include_str!("fixtures/eml_gadget_runtime_gate_ema.ron");

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;
const GADGET_TREE_ID: u32 = 9001;

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn set_col(values: &mut [f32], col: u32, v: f32) {
    values[(EVAL_SLOT * N_DIMS + col) as usize] = v;
}

fn exact_meta(id: u32, name: &str) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 0,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: name.into(),
    }
}

fn register_and_upload(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
    tree_id: u32,
    name: &str,
    nodes: Vec<EmlNodeGpu>,
) {
    let id = EmlTreeId(tree_id);
    if registry.get(id).is_none() {
        registry
            .register_formula(id, exact_meta(tree_id, name), nodes)
            .expect("register compiled gadget executable");
    }
    let mut trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    trees.sort_by_key(|(id, _, _)| id.0);
    let mapping = table.upload_trees(ctx, &trees).expect("upload EML trees");
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .expect("mark uploaded");
    }
}

fn eval_eml_op(tree_id: u32, eval_slot: u32, target_slot: u32, target_col: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: eval_slot,
            col: 0,
        },
        combine: CombineFn::EvalEML { tree_id },
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, target_col)],
    }
}

fn run_gadget_runtime(
    ctx: &GpuContext,
    nodes: &[EmlNodeGpu],
    values: &[f32],
    target_col: u32,
) -> f32 {
    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(ctx, N_DIMS * 2, N_DIMS);
    register_and_upload(
        ctx,
        &mut registry,
        &mut table,
        GADGET_TREE_ID,
        "gadget_runtime_gate",
        nodes.to_vec(),
    );

    let mut session = AccumulatorOpSession::new(ctx, values.len() as u32 / N_DIMS, N_DIMS);
    set_debug_readback_allowed(true);
    session.upload_values(ctx, values);
    let op = eval_eml_op(GADGET_TREE_ID, EVAL_SLOT, EVAL_SLOT, target_col);
    session
        .upload_ops_with_eml(ctx, std::slice::from_ref(&op), Some(&registry))
        .expect("upload EvalEML op");
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session.tick_with_eml(ctx, 0, eml).expect("EvalEML tick");
    let out = session.readback_full(ctx).expect("readback");
    out[(EVAL_SLOT * N_DIMS + target_col) as usize]
}

fn nodes_from_compiled(gadget_index: usize, ron: &str) -> (Vec<EmlNodeGpu>, u32) {
    let stack = deserialize_eml_gadget_stack_ron(ron).expect("parse gadget RON");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("compile gadget stack");
    let gadget = &compiled.gadgets[gadget_index];
    let gpu_nodes: Vec<EmlNodeGpu> = gadget
        .nodes
        .iter()
        .copied()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
    let output_col = gadget.output_col.expect("gadget output_col");
    (gpu_nodes, output_col)
}

#[test]
fn runtime_gate_weighted_accumulator_ron_loads_and_compiles() {
    let stack = deserialize_eml_gadget_stack_ron(WEIGHTED_ACC_RON).expect("RON load");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("compile");
    assert_eq!(compiled.gadgets.len(), 1);
    assert_eq!(compiled.gadgets[0].kind, EmlGadgetKind::WeightedAccumulator);
}

#[test]
fn runtime_gate_ema_ron_loads_and_compiles() {
    let stack = deserialize_eml_gadget_stack_ron(EMA_RON).expect("RON load");
    let compiled = compile_eml_gadget_stack(&stack, EmlGadgetCompileOptions { max_col: 64 })
        .expect("compile");
    assert_eq!(compiled.gadgets.len(), 1);
    assert_eq!(compiled.gadgets[0].kind, EmlGadgetKind::Ema);
}

#[test]
fn runtime_gate_weighted_accumulator_gpu_matches_spec_oracle() {
    with_gpu(|ctx| {
        let (nodes, output_col) = nodes_from_compiled(0, WEIGHTED_ACC_RON);
        let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 3, 12.0);
        set_col(&mut values, 4, 8.0);
        set_col(&mut values, 20, 0.6);
        set_col(&mut values, 21, 0.4);

        let spec_oracle = eval_eml_postfix(
            &compile_eml_gadget_stack(
                &deserialize_eml_gadget_stack_ron(WEIGHTED_ACC_RON).unwrap(),
                EmlGadgetCompileOptions { max_col: 64 },
            )
            .unwrap()
            .gadgets[0]
            .nodes,
            EVAL_SLOT,
            &values,
            N_DIMS,
        );
        let named_oracle = oracle_weighted_accumulator(&[12.0, 8.0], &[0.6, 0.4]);
        assert!((spec_oracle - named_oracle).abs() < 1e-6);

        let gpu_cpu = eval_eml_cpu(&nodes, EVAL_SLOT, &values, N_DIMS, [0.0; 4]);
        assert_eq!(gpu_cpu.to_bits(), spec_oracle.to_bits());

        let runtime = run_gadget_runtime(ctx, &nodes, &values, output_col);
        assert_eq!(
            runtime.to_bits(),
            spec_oracle.to_bits(),
            "GPU runtime must match spec-layer oracle"
        );
    });
}

#[test]
fn runtime_gate_ema_gpu_matches_spec_oracle() {
    with_gpu(|ctx| {
        let (nodes, output_col) = nodes_from_compiled(0, EMA_RON);
        let mut values = vec![0.0f32; (N_DIMS * (EVAL_SLOT + 1)) as usize];
        set_col(&mut values, 3, 40.0);
        set_col(&mut values, 13, 10.0);

        let spec_oracle = eval_eml_postfix(
            &compile_eml_gadget_stack(
                &deserialize_eml_gadget_stack_ron(EMA_RON).unwrap(),
                EmlGadgetCompileOptions { max_col: 64 },
            )
            .unwrap()
            .gadgets[0]
            .nodes,
            EVAL_SLOT,
            &values,
            N_DIMS,
        );
        let named_oracle = oracle_ema(40.0, 10.0, 0.85);
        assert!((spec_oracle - named_oracle).abs() < 1e-6);

        let gpu_cpu = eval_eml_cpu(&nodes, EVAL_SLOT, &values, N_DIMS, [0.0; 4]);
        assert_eq!(gpu_cpu.to_bits(), spec_oracle.to_bits());

        let runtime = run_gadget_runtime(ctx, &nodes, &values, output_col);
        assert_eq!(
            runtime.to_bits(),
            spec_oracle.to_bits(),
            "GPU runtime must match spec-layer oracle"
        );
    });
}

#[test]
fn runtime_gate_fixture_only_no_default_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let session_src = include_str!("../src/session.rs");
    assert!(!session_src.contains("compile_eml_gadget_stack"));
    assert!(!session_src.contains("eml_gadget_runtime_gate"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("EmlGadget"));
}

#[test]
fn runtime_gate_posture_no_chained_scheduling_or_jit() {
    let sources = [
        WEIGHTED_ACC_RON,
        EMA_RON,
        include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl"),
    ];
    for src in sources {
        assert!(!src.contains("JIT"));
        assert!(!src.contains("source_mask"));
    }
}
