//! Column-aware parent EML bridge after structured field stencil (V7.6).

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SourceSpec,
    WHITELISTED_FORMULA_CLASSES,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable,
    GpuContext, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const N_DIMS: u32 = 8;
const COL_FIELD: u32 = 0;
const COL_RESOURCE: u32 = 1;
const COL_WEIGHT_A: u32 = 2;
const COL_WEIGHT_B: u32 = 3;
const COL_OUTPUT: u32 = 4;
const GRID_W: u32 = 10;
const GRID_H: u32 = 10;
const N_CELLS: u32 = GRID_W * GRID_H;
const PARENT_SLOT: u32 = 100;
const TREE_URGENCY: u32 = 1;

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn get(v: &[f32], slot: u32, col: u32) -> f32 {
    v[idx(slot, col)]
}

fn urgency_nodes() -> Vec<EmlNodeGpu> {
    vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_FIELD,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_WEIGHT_A,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_RESOURCE,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_WEIGHT_B,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::ADD,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::RETURN_TOP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ]
}

struct Setup {
    registry: EmlExpressionRegistry,
    table: EmlGpuProgramTable,
}

impl Setup {
    fn new(ctx: &GpuContext) -> Self {
        let mut registry = EmlExpressionRegistry::new();
        let mut table = EmlGpuProgramTable::new(ctx, 128, 16);
        let nodes = urgency_nodes();
        registry
            .register_formula(
                EmlTreeId(TREE_URGENCY),
                EmlFormulaMeta {
                    tree_id: EmlTreeId(TREE_URGENCY),
                    execution_class: EmlExecutionClass::ExactDeterministic,
                    allowed_consumers: EmlConsumerMask(
                        EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
                    ),
                    max_abs_error: None,
                    deterministic_gpu: true,
                    requires_guard_for_hard_threshold: false,
                    node_count: nodes.len() as u32,
                    max_stack_depth: 0,
                    has_loops: false,
                    has_recursion: false,
                    display_name: "field_urgency".into(),
                },
                nodes,
            )
            .unwrap();
        let trees: Vec<_> = registry
            .formulas_for_gpu_upload()
            .map(|(t, m, n)| (t, m.clone(), n.to_vec()))
            .collect();
        for (t, ri) in table.upload_trees(ctx, &trees).unwrap() {
            registry
                .mark_tree_uploaded(t, ri, table.generation)
                .unwrap();
        }
        Self { registry, table }
    }
}

fn run_bands(
    ctx: &GpuContext,
    setup: &Setup,
    session: &mut AccumulatorOpSession,
    ops: &[AccumulatorOp],
    values: &[f32],
) -> Vec<f32> {
    set_debug_readback_allowed(true);
    session.upload_values(ctx, values);
    session
        .upload_ops_with_eml(ctx, ops, Some(&setup.registry))
        .unwrap();
    let eml = Some((&setup.table.node_buffer, &setup.table.range_buffer));
    for band in [0u32, 1] {
        session.tick_with_eml(ctx, band, eml).unwrap();
    }
    session.readback_full(ctx).unwrap()
}

#[test]
fn test_e_column_aware_parent_eml() {
    with_gpu(|ctx| {
        let setup = Setup::new(ctx);
        let config = StructuredFieldStencilConfig {
            width: GRID_W,
            height: GRID_H,
            n_dims: N_DIMS,
            source_col: COL_FIELD,
            target_col: COL_FIELD,
            horizon: 8,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            weight_north: 0.0,
            weight_south: 0.0,
            weight_east: 0.0,
            weight_west: 0.0,
            source_cap: Some(500.0),
            operator: StructuredFieldStencilOperator::SourceCappedNormalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let cluster = [
            (0u32, 0u32, 80.0f32),
            (0, 1, 60.0),
            (1, 0, 60.0),
            (1, 1, 40.0),
        ];
        let mut values = vec![0.0f32; op.config().values_len()];
        for &(r, c, v) in &cluster {
            values[idx(r * GRID_W + c, COL_FIELD)] = v;
        }
        op.upload_values(ctx, &values).unwrap();
        op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
        let mut grid = op.readback_after_ping_pong(ctx, 1);
        for &(r, c, _) in &cluster {
            grid[idx(r * GRID_W + c, COL_FIELD)] = 0.0;
        }
        op.upload_values(ctx, &grid).unwrap();
        let (grid, _) = op.run_configured_horizon(ctx).unwrap();

        let sum_slots = PARENT_SLOT + 1;
        let mut session = AccumulatorOpSession::new(ctx, sum_slots, N_DIMS);
        let mut pv = grid;
        for s in 0..N_CELLS {
            pv[idx(s, COL_RESOURCE)] = 1.0;
        }
        pv.resize((sum_slots * N_DIMS) as usize, 0.0);

        let ops = [
            AccumulatorOp {
                source: SourceSpec::SlotRange {
                    start: 0,
                    count: N_CELLS,
                    col: COL_FIELD,
                },
                combine: CombineFn::Sum,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(PARENT_SLOT, COL_FIELD)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotRange {
                    start: 0,
                    count: N_CELLS,
                    col: COL_RESOURCE,
                },
                combine: CombineFn::Sum,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(PARENT_SLOT, COL_RESOURCE)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: PARENT_SLOT,
                    col: COL_FIELD,
                },
                combine: CombineFn::EvalEML {
                    tree_id: TREE_URGENCY,
                },
                gate: GateSpec::OrderBand(1),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(PARENT_SLOT, COL_OUTPUT)],
            },
        ];

        let mut run = |wa: f32, wb: f32| -> f32 {
            let mut v = pv.clone();
            v[idx(PARENT_SLOT, COL_WEIGHT_A)] = wa;
            v[idx(PARENT_SLOT, COL_WEIGHT_B)] = wb;
            v = run_bands(ctx, &setup, &mut session, &ops, &v);
            get(&v, PARENT_SLOT, COL_OUTPUT)
        };

        let urgency_a = run(0.2, 0.1);
        let urgency_b = run(0.9, 0.1);
        assert!(urgency_a > 0.0, "urgency_a={urgency_a}");
        assert!(urgency_b > urgency_a, "a={urgency_a} b={urgency_b}");
    });
}

#[test]
fn test_g_production_defaults_unaffected() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    for class in [
        "field_pressure",
        "field_urgency",
        "field_decay",
        "bounded_field_update",
        "conversion_rate",
    ] {
        assert!(WHITELISTED_FORMULA_CLASSES.contains(&class));
    }
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!gpu_lib.contains("StructuredFieldStencilOp::new(&ctx"));
    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("StructuredFieldStencilOp"));
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("StructuredFieldStencilOp"));
}
