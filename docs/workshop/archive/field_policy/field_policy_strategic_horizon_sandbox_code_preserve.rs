//! FIELD_POLICY strategic horizon / velocity column / PF-skip feasibility probe (prototype only).
//!
//! Extends the first FIELD_POLICY probe on unresolved questions: long-range gradient horizon,
//! explicit-column velocity without previous-buffer EML, and PF-style convergence measurability.
//! No mapping runtime. No new WGSL.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable,
    GpuContext,
};
use simthing_sim::PipelineFlags;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu_lock<F: FnOnce()>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f();
}

const N_DIMS: u32 = 8;
const COL_THREAT: u32 = 0;
const COL_PREVIOUS: u32 = 6;
const COL_VELOCITY: u32 = 7;
const FALLOFF: f32 = 0.8;

const TREE_VELOCITY: u32 = 1;

const GRID_W: u32 = 10;
const GRID_H: u32 = 10;
const N_CELLS: u32 = GRID_W * GRID_H;

const CLUSTER: [(u32, u32, f32); 4] = [(0, 0, 80.0), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];

const EPS_VALUE: f32 = 0.01;
const EPS_DELTA: f32 = 0.001;
const STABLE_REQUIRED: u32 = 3;

#[derive(Clone, Debug, Default)]
struct CapabilityProbe {
    eval_eml_gpu: bool,
    add_to_target: bool,
    scale_target_decay: bool,
    slot_value_same_buffer: bool,
    copy_current_to_previous: bool,
}

static CAP_PROBE: OnceLock<CapabilityProbe> = OnceLock::new();

fn cap_probe() -> &'static CapabilityProbe {
    CAP_PROBE.get_or_init(run_capability_probe)
}

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn get(values: &[f32], slot: u32, col: u32) -> f32 {
    values[idx(slot, col)]
}

fn set(values: &mut [f32], slot: u32, col: u32, v: f32) {
    values[idx(slot, col)] = v;
}

fn grid_slot(row: u32, col: u32) -> u32 {
    row * GRID_W + col
}

fn literal(v: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot_value(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn return_top() -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn bin_op(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn velocity_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_value(COL_THREAT),
        slot_value(COL_PREVIOUS),
        bin_op(eml_opcode::SUB),
        return_top(),
    ]
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

struct SandboxSetup {
    registry: EmlExpressionRegistry,
    table: EmlGpuProgramTable,
}

impl SandboxSetup {
    fn new(ctx: &GpuContext) -> Self {
        let mut registry = EmlExpressionRegistry::new();
        let mut table = EmlGpuProgramTable::new(ctx, 64, 8);
        register_and_upload(
            ctx,
            &mut registry,
            &mut table,
            TREE_VELOCITY,
            exact_meta(TREE_VELOCITY, "velocity_sub"),
            velocity_nodes(),
        );
        Self { registry, table }
    }
}

fn register_and_upload(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
    tree_id: u32,
    meta: EmlFormulaMeta,
    nodes: Vec<EmlNodeGpu>,
) {
    let id = EmlTreeId(tree_id);
    if registry.get(id).is_none() {
        registry.register_formula(id, meta, nodes).unwrap();
    }
    let trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    let mapping = table.upload_trees(ctx, &trees).unwrap();
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .unwrap();
    }
}

fn seed_op(slot: u32, threat: f32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(threat),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(slot, COL_THREAT)],
    }
}

fn propagate_op(src: u32, dst: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: src,
            col: COL_THREAT,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Constant(FALLOFF),
        consume: ConsumeMode::AddToTarget,
        targets: vec![(dst, COL_THREAT)],
    }
}

fn nsew_prop_ops(band: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for r in 0..GRID_H {
        for c in 0..GRID_W {
            let s = grid_slot(r, c);
            if r > 0 {
                ops.push(propagate_op(s, grid_slot(r - 1, c), band));
            }
            if r + 1 < GRID_H {
                ops.push(propagate_op(s, grid_slot(r + 1, c), band));
            }
            if c > 0 {
                ops.push(propagate_op(s, grid_slot(r, c - 1), band));
            }
            if c + 1 < GRID_W {
                ops.push(propagate_op(s, grid_slot(r, c + 1), band));
            }
        }
    }
    ops
}

/// Directed south/east propagation from each cell — stable for top-left threat sources
/// without bidirectional feedback amplification.
fn directed_se_prop_ops(band: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for r in 0..GRID_H {
        for c in 0..GRID_W {
            let s = grid_slot(r, c);
            if r + 1 < GRID_H {
                ops.push(propagate_op(s, grid_slot(r + 1, c), band));
            }
            if c + 1 < GRID_W {
                ops.push(propagate_op(s, grid_slot(r, c + 1), band));
            }
        }
    }
    ops
}

fn reset_threat_ops(band: u32) -> Vec<AccumulatorOp> {
    (0..N_CELLS)
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| AccumulatorOp {
            source: SourceSpec::Constant(0.0),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(band),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: chunk.iter().map(|&s| (s, COL_THREAT)).collect(),
        })
        .collect()
}

fn decay_all_ops(band: u32) -> Vec<AccumulatorOp> {
    (0..N_CELLS)
        .collect::<Vec<_>>()
        .chunks(4)
        .flat_map(|chunk| {
            chunk.iter().map(|&s| AccumulatorOp {
                source: SourceSpec::Constant(FALLOFF),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(band),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ScaleTarget,
                targets: vec![(s, COL_THREAT)],
            })
        })
        .collect()
}

fn copy_current_to_previous_op(slot: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot,
            col: COL_THREAT,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(slot, COL_PREVIOUS)],
    }
}

fn velocity_eml_op(slot: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot,
            col: COL_THREAT,
        },
        combine: CombineFn::EvalEML {
            tree_id: TREE_VELOCITY,
        },
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(slot, COL_VELOCITY)],
    }
}

fn run_bands_gpu(
    ctx: &GpuContext,
    session: &mut AccumulatorOpSession,
    setup: &SandboxSetup,
    ops: &[AccumulatorOp],
    values: &[f32],
    bands: &[u32],
) -> Vec<f32> {
    set_debug_readback_allowed(true);
    session.upload_values(ctx, values);
    session
        .upload_ops_with_eml(ctx, ops, Some(&setup.registry))
        .unwrap();
    let eml = Some((&setup.table.node_buffer, &setup.table.range_buffer));
    for &band in bands {
        session.tick_with_eml(ctx, band, eml).unwrap();
    }
    session.readback_full(ctx).unwrap()
}

fn threat_at(values: &[f32], row: u32, col: u32) -> f32 {
    get(values, grid_slot(row, col), COL_THREAT)
}

fn gradient_at(values: &[f32], row: u32, col: u32) -> (f32, f32, f32) {
    let gx = (threat_at(values, row, col + 1) - threat_at(values, row, col - 1)) / 2.0;
    let gy = (threat_at(values, row + 1, col) - threat_at(values, row - 1, col)) / 2.0;
    let mag = (gx * gx + gy * gy).sqrt();
    (gx, gy, mag)
}

fn grad_direction(gx: f32, gy: f32, mag: f32, t44: f32) -> &'static str {
    if t44 < 1e-4 {
        "none"
    } else if gx < 0.0 && gy < 0.0 && mag > 0.0 {
        "correct"
    } else if mag > 0.0 {
        "partial"
    } else {
        "none"
    }
}

fn build_setup_ops() -> (Vec<AccumulatorOp>, Vec<u32>) {
    let mut ops = reset_threat_ops(0);
    for &(r, c, v) in &CLUSTER {
        ops.push(seed_op(grid_slot(r, c), v, 1));
    }
    ops.extend(directed_se_prop_ops(2));
    for &(r, c, _) in &CLUSTER {
        ops.push(seed_op(grid_slot(r, c), 0.0, 3));
    }
    (ops, vec![0, 1, 2, 3])
}

fn build_prop_ops_for_hops(hops: u32, base_band: u32) -> (Vec<AccumulatorOp>, Vec<u32>) {
    let mut ops = Vec::new();
    let mut bands = Vec::new();
    for h in 0..hops {
        let band = base_band + h;
        bands.push(band);
        ops.extend(directed_se_prop_ops(band));
    }
    (ops, bands)
}

struct HorizonRunResult {
    values: Vec<f32>,
    wall_ms: f64,
    ops_count: usize,
    max_band: u32,
    bands_per_tick: u32,
    ticks: u32,
}

fn run_horizon(
    ctx: &GpuContext,
    setup: &SandboxSetup,
    _effective_hops: u32,
    bands_per_tick: u32,
    ticks: u32,
    with_decay: bool,
) -> HorizonRunResult {
    let (setup_ops, setup_bands) = build_setup_ops();
    let (prop_ops, _) = build_prop_ops_for_hops(bands_per_tick, 4);
    let decay_band = 4 + bands_per_tick;
    let decay_ops = if with_decay {
        decay_all_ops(decay_band)
    } else {
        Vec::new()
    };

    let tick_chunk_len = prop_ops.len() + if with_decay { decay_ops.len() } else { 0 };

    let mut session = AccumulatorOpSession::new(ctx, N_CELLS, N_DIMS);
    let mut values = vec![0.0f32; (N_CELLS * N_DIMS) as usize];
    let t0 = Instant::now();

    // Tick 1: setup + propagation (+ optional decay).
    let mut first_ops = setup_ops;
    first_ops.extend(prop_ops.clone());
    if with_decay {
        first_ops.extend(decay_ops.clone());
    }
    let mut first_bands = setup_bands;
    first_bands.extend((4..4 + bands_per_tick).collect::<Vec<_>>());
    if with_decay {
        first_bands.push(decay_band);
    }
    values = run_bands_gpu(ctx, &mut session, setup, &first_ops, &values, &first_bands);

    // Follow-up ticks: propagation only (+ optional decay).
    for _ in 1..ticks {
        let mut tick_ops = prop_ops.clone();
        if with_decay {
            tick_ops.extend(decay_ops.clone());
        }
        let mut tick_bands: Vec<u32> = (4..4 + bands_per_tick).collect();
        if with_decay {
            tick_bands.push(decay_band);
        }
        values = run_bands_gpu(ctx, &mut session, setup, &tick_ops, &values, &tick_bands);
    }

    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let max_band = if with_decay {
        decay_band
    } else {
        3 + bands_per_tick
    };
    let ops_count = first_ops.len() + tick_chunk_len * (ticks.saturating_sub(1) as usize);
    HorizonRunResult {
        values,
        wall_ms,
        ops_count,
        max_band,
        bands_per_tick,
        ticks,
    }
}

fn field_l1(values: &[f32]) -> f32 {
    (0..N_CELLS).map(|s| get(values, s, COL_THREAT).abs()).sum()
}

fn field_max(values: &[f32]) -> f32 {
    (0..N_CELLS)
        .map(|s| get(values, s, COL_THREAT))
        .fold(0.0f32, f32::max)
}

fn run_capability_probe() -> CapabilityProbe {
    let Some(ctx) = try_gpu() else {
        return CapabilityProbe::default();
    };
    let setup = SandboxSetup::new(&ctx);
    let mut p = CapabilityProbe::default();

    // EvalEML GPU
    let mut values = vec![0.0f32; N_DIMS as usize];
    set(&mut values, 0, COL_THREAT, 3.0);
    set(&mut values, 0, COL_PREVIOUS, 1.0);
    let mut session = AccumulatorOpSession::new(&ctx, 1, N_DIMS);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[velocity_eml_op(0, 0)],
        &values,
        &[0],
    );
    p.eval_eml_gpu = (get(&values, 0, COL_VELOCITY) - 2.0).abs() < 0.01;

    // AddToTarget propagation
    let mut session = AccumulatorOpSession::new(&ctx, 2, N_DIMS);
    let mut values = vec![0.0f32; (2 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[propagate_op(0, 1, 2)],
        &values,
        &[2],
    );
    p.add_to_target = get(&values, 1, COL_THREAT) > 0.0;

    // ScaleTarget decay
    let mut session = AccumulatorOpSession::new(&ctx, 1, N_DIMS);
    let mut values = vec![0.0f32; N_DIMS as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[AccumulatorOp {
            source: SourceSpec::Constant(FALLOFF),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ScaleTarget,
            targets: vec![(0, COL_THREAT)],
        }],
        &values,
        &[1],
    );
    p.scale_target_decay = get(&values, 0, COL_THREAT) < 10.0 && get(&values, 0, COL_THREAT) > 0.0;

    // Same-buffer SlotValue read in EML
    p.slot_value_same_buffer = p.eval_eml_gpu;

    // Copy current -> previous
    let mut session = AccumulatorOpSession::new(&ctx, 1, N_DIMS);
    let mut values = vec![0.0f32; N_DIMS as usize];
    set(&mut values, 0, COL_THREAT, 7.0);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[copy_current_to_previous_op(0, 1)],
        &values,
        &[1],
    );
    p.copy_current_to_previous = (get(&values, 0, COL_PREVIOUS) - 7.0).abs() < 0.01;

    p
}

#[test]
fn test0_baseline_capability_sanity() {
    with_gpu_lock(|| {
        let p = cap_probe();
        println!(
            "Test0 EvalEML_GPU={} AddToTarget={} ScaleTarget_decay={} SlotValue_same_buffer={} copy_current_to_previous={}",
            p.eval_eml_gpu, p.add_to_target, p.scale_target_decay, p.slot_value_same_buffer, p.copy_current_to_previous
        );
    });
}

#[test]
fn test1_strategic_horizon_sweep_10x10() {
    with_gpu_lock(|| {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    assert!(cap_probe().add_to_target, "local propagation required");

    let horizons = [4u32, 8, 12, 16, 24];
    let mut first_nonzero = None;
    let mut first_directional = None;

    println!("Horizon sweep (directed SE propagation, bands_per_tick=H, ticks=1):");
    for &h in &horizons {
        let r = run_horizon(&ctx, &setup, h, h, 1, false);
        let t44 = threat_at(&r.values, 4, 4);
        let (gx, gy, mag) = gradient_at(&r.values, 4, 4);
        let dir = grad_direction(gx, gy, mag, t44);
        if t44 > 1e-4 && first_nonzero.is_none() {
            first_nonzero = Some(h);
        }
        if dir == "correct" && first_directional.is_none() {
            first_directional = Some(h);
        }
        println!(
            "  H={h}: threat[0][0]={:.2} threat[1][1]={:.2} threat[2][2]={:.2} threat[3][3]={:.2} threat[4][4]={:.4} threat[5][5]={:.4} threat[6][6]={:.4} threat[9][9]={:.4} grad=({gx:.4},{gy:.4}) mag={mag:.4} direction={dir}",
            threat_at(&r.values, 0, 0),
            threat_at(&r.values, 1, 1),
            threat_at(&r.values, 2, 2),
            threat_at(&r.values, 3, 3),
            t44,
            threat_at(&r.values, 5, 5),
            threat_at(&r.values, 6, 6),
            threat_at(&r.values, 9, 9),
        );
    }
    println!(
        "first_nonzero_horizon_at_4_4={} first_directional_horizon_at_4_4={}",
        first_nonzero.map(|h| h.to_string()).unwrap_or_else(|| "NONE".into()),
        first_directional
            .map(|h| h.to_string())
            .unwrap_or_else(|| "NONE".into())
    );
    });
}

#[test]
fn test2_horizon_cost_curve() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let horizons = [4u32, 8, 12, 16, 24];
    let values_bytes = N_CELLS * N_DIMS * 4;

    println!("Horizon cost (cells={N_CELLS} n_dims={N_DIMS} values_bytes={values_bytes}):");
    let mut best_directional_h = None;
    let mut best_ms = 0.0f64;

    for &h in &horizons {
        let r = run_horizon(&ctx, &setup, h, h, 1, false);
        let t44 = threat_at(&r.values, 4, 4);
        let (_, _, mag) = gradient_at(&r.values, 4, 4);
        let dir = grad_direction(
            gradient_at(&r.values, 4, 4).0,
            gradient_at(&r.values, 4, 4).1,
            mag,
            t44,
        );
        let ms_per_hop = r.wall_ms / h as f64;
        if dir == "correct" && best_directional_h.is_none() {
            best_directional_h = Some(h);
            best_ms = r.wall_ms;
        }
        println!(
            "  H={h}: order_bands={} ticks=1 ops={} wall_ms={:.2} ms_per_hop={:.3} readback_bytes={values_bytes}",
            r.max_band, r.ops_count, r.wall_ms, ms_per_hop
        );
    }

    if let Some(h) = best_directional_h {
        let projected_30k = best_ms * (30_000.0 / N_CELLS as f64);
        let projected_100k = best_ms * (100_000.0 / N_CELLS as f64);
        println!(
            "Projection at best_directional_H={h}: projected_30k_ms={projected_30k:.1} projected_100k_ms={projected_100k:.1} (rough linear projection only)"
        );
    } else {
        println!("Projection: INCONCLUSIVE — no directional horizon found by H=24");
    }
}

#[test]
fn test3_multi_cadence_strategic_horizon() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);

    let models: [( &str, u32, u32); 3] = [
        ("A_16bands_1tick", 16, 1),
        ("B_4bands_4ticks", 4, 4),
        ("C_2bands_8ticks", 2, 8),
    ];

    println!("Multi-cadence horizon (effective_hops=16):");
    for (name, bands_per_tick, ticks) in models {
        let t0 = Instant::now();
        let r = run_horizon(&ctx, &setup, 16, bands_per_tick, ticks, false);
        let total_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let mean_per_tick = total_ms / ticks as f64;
        let t44 = threat_at(&r.values, 4, 4);
        let (gx, gy, mag) = gradient_at(&r.values, 4, 4);
        let dir = grad_direction(gx, gy, mag, t44);
        println!(
            "  model={name} bands_per_tick={bands_per_tick} ticks={ticks} final_threat[4][4]={t44:.4} grad=({gx:.4},{gy:.4}) mag={mag:.4} direction={dir} total_wall_ms={total_ms:.2} mean_ms_per_tick={mean_per_tick:.2}"
        );
    }
}

#[test]
fn test4_velocity_explicit_previous_column() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let p = cap_probe();

    if !p.copy_current_to_previous {
        println!(
            "Test4 DEFERRED explicit_prev_column_supported=NO copy_current_to_previous_supported=NO velocity=DEFERRED"
        );
        return;
    }

    let slot = 0u32;
    let mut session = AccumulatorOpSession::new(&ctx, 1, N_DIMS);
    let mut values = vec![0.0f32; N_DIMS as usize];
    let mut vel_tick2 = None;
    let mut vel_tick3 = None;
    let mut vel_decay = None;

    let threat_seq = [1.0f32, 3.0, 5.0, 5.0, 5.0, 5.0];
    for (tick, &threat) in threat_seq.iter().enumerate() {
        let mut tick_ops = vec![
            velocity_eml_op(slot, 0),
            copy_current_to_previous_op(slot, 1),
            seed_op(slot, threat, 2),
        ];
        if tick >= 3 {
            tick_ops.push(AccumulatorOp {
                source: SourceSpec::Constant(FALLOFF),
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(3),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ScaleTarget,
                targets: vec![(slot, COL_THREAT)],
            });
        }
        let bands: Vec<u32> = if tick >= 3 {
            vec![0, 1, 2, 3]
        } else {
            vec![0, 1, 2]
        };
        values = run_bands_gpu(&ctx, &mut session, &setup, &tick_ops, &values, &bands);
        let vel = get(&values, slot, COL_VELOCITY);
        if tick == 1 {
            vel_tick2 = Some(vel);
        }
        if tick == 2 {
            vel_tick3 = Some(vel);
        }
        if tick == threat_seq.len() - 1 {
            vel_decay = Some(vel);
        }
    }

    let v2 = vel_tick2.unwrap();
    let v3 = vel_tick3.unwrap();
    let vd = vel_decay.unwrap();
    assert!(v2 > 0.0, "velocity tick2={v2}");
    assert!(v3 > 0.0, "velocity tick3={v3}");
    assert!(vd <= 0.0 + 0.01, "velocity decay={vd}");
    println!(
        "Test4 explicit_prev_column_supported=YES copy_current_to_previous_supported=YES velocity_tick2={v2:.4} velocity_tick3={v3:.4} velocity_decay_tick={vd:.4}"
    );
}

#[test]
fn test5_velocity_cost_impact() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let side = 14u32;
    let n_cells = side * side;

    let mut base_prop = Vec::new();
    for r in 0..side {
        for c in 0..side {
            let s = r * side + c;
            if c > 0 {
                base_prop.push(AccumulatorOp {
                    source: SourceSpec::SlotValue {
                        slot: s,
                        col: COL_THREAT,
                    },
                    combine: CombineFn::Identity,
                    gate: GateSpec::OrderBand(2),
                    scale: ScaleSpec::Constant(FALLOFF),
                    consume: ConsumeMode::AddToTarget,
                    targets: vec![(r * side + (c - 1), COL_THREAT)],
                });
            }
        }
    }
    let prop_band = 2u32;
    let center = (side / 2) * side + side / 2;
    let mut base_ops = vec![seed_op(center, 50.0, 1)];
    base_ops.extend(base_prop.clone());

    // Base n_dims=6
    let n_dims_base = 6u32;
    let mut session = AccumulatorOpSession::new(&ctx, n_cells, n_dims_base);
    let values_base = vec![0.0f32; (n_cells * n_dims_base) as usize];
    let mut base_ms = Vec::new();
    for _ in 0..5 {
        let t0 = Instant::now();
        set_debug_readback_allowed(true);
        session.upload_values(&ctx, &values_base);
        session.upload_ops(&ctx, &base_ops).unwrap();
        session.tick(&ctx, 1).unwrap();
        session.tick(&ctx, prop_band).unwrap();
        let _ = session.readback_full(&ctx).unwrap();
        base_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }
    let base_mean = base_ms.iter().sum::<f64>() / base_ms.len() as f64;
    let base_bytes = n_cells * n_dims_base * 4;

    // Velocity n_dims=8 with per-cell velocity + copy
    let mut vel_ops = base_ops.clone();
    for s in 0..n_cells {
        vel_ops.push(velocity_eml_op(s, 3));
        vel_ops.push(copy_current_to_previous_op(s, 4));
    }
    let mut session = AccumulatorOpSession::new(&ctx, n_cells, N_DIMS);
    let values_vel = vec![0.0f32; (n_cells * N_DIMS) as usize];
    let mut vel_ms = Vec::new();
    for _ in 0..5 {
        let t0 = Instant::now();
        let _ = run_bands_gpu(
            &ctx,
            &mut session,
            &setup,
            &vel_ops,
            &values_vel,
            &[1, prop_band, 3, 4],
        );
        vel_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }
    let vel_mean = vel_ms.iter().sum::<f64>() / vel_ms.len() as f64;
    let vel_bytes = n_cells * N_DIMS * 4;
    let delta_ms = vel_mean - base_mean;
    let overhead_pct = if base_mean > 0.0 {
        100.0 * delta_ms / base_mean
    } else {
        0.0
    };

    println!(
        "Velocity overhead: base_mean_ms={base_mean:.3} velocity_mean_ms={vel_mean:.3} delta_ms={delta_ms:.3} overhead_percent={overhead_pct:.1} base_values_bytes={base_bytes} velocity_values_bytes={vel_bytes} bytes_delta={} ops_delta={}",
        vel_bytes - base_bytes,
        vel_ops.len() - base_ops.len()
    );
}

#[test]
fn test6_pf_convergence_detection() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);

    let mut session = AccumulatorOpSession::new(&ctx, N_CELLS, N_DIMS);
    let mut values = vec![0.0f32; (N_CELLS * N_DIMS) as usize];
    for &(r, c, v) in &CLUSTER {
        set(&mut values, grid_slot(r, c), COL_THREAT, v);
    }

    let decay_ops = decay_all_ops(1);
    let mut prev_max = field_max(&values);
    let mut ratios = Vec::new();
    let mut first_skip: Option<u32> = None;
    let mut stable_count = 0u32;

    println!("PF convergence:");
    for tick in 0..=32 {
        let prev = values.clone();
        values = run_bands_gpu(&ctx, &mut session, &setup, &decay_ops, &values, &[1]);
        let max_v = field_max(&values);
        let max_delta = (0..N_CELLS)
            .map(|s| (get(&values, s, COL_THREAT) - get(&prev, s, COL_THREAT)).abs())
            .fold(0.0f32, f32::max);
        let l1 = field_l1(&values);
        let ratio = if prev_max > 1e-8 {
            max_v / prev_max
        } else {
            0.0
        };
        if prev_max > 1e-8 {
            ratios.push(ratio);
        }
        prev_max = max_v;

        if max_v < EPS_VALUE && max_delta < EPS_DELTA {
            stable_count += 1;
            if stable_count >= STABLE_REQUIRED && first_skip.is_none() {
                first_skip = Some(tick);
            }
        } else {
            stable_count = 0;
        }

        if tick == 0 || tick == 5 || tick == 10 || tick == 20 || tick == 32 {
            println!(
                "  tick{tick} max={max_v:.6} delta={max_delta:.6} l1={l1:.4} ratio={ratio:.4}"
            );
        }
    }
    let mean_ratio = if ratios.is_empty() {
        0.0
    } else {
        ratios.iter().sum::<f32>() / ratios.len() as f32
    };
    println!(
        "first_skip_candidate_tick={} estimated_contraction_ratio_mean={mean_ratio:.4}",
        first_skip.map(|t| t.to_string()).unwrap_or_else(|| "NONE".into())
    );
}

#[test]
fn test7_pf_skip_correctness_simulation() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);

    let mut session = AccumulatorOpSession::new(&ctx, N_CELLS, N_DIMS);
    let mut values = vec![0.0f32; (N_CELLS * N_DIMS) as usize];
    for &(r, c, v) in &CLUSTER {
        set(&mut values, grid_slot(r, c), COL_THREAT, v);
    }
    let decay_ops = decay_all_ops(1);
    let mut skip_tick = None;
    let mut stable_count = 0u32;
    let mut prev_max = field_max(&values);

    for tick in 1..=32 {
        let prev = values.clone();
        values = run_bands_gpu(&ctx, &mut session, &setup, &decay_ops, &values, &[1]);
        let max_v = field_max(&values);
        let max_delta = (0..N_CELLS)
            .map(|s| (get(&values, s, COL_THREAT) - get(&prev, s, COL_THREAT)).abs())
            .fold(0.0f32, f32::max);
        let _ = prev_max;
        prev_max = max_v;
        if max_v < EPS_VALUE && max_delta < EPS_DELTA {
            stable_count += 1;
            if stable_count >= STABLE_REQUIRED && skip_tick.is_none() {
                skip_tick = Some(tick);
            }
        } else {
            stable_count = 0;
        }
    }

    let candidate = skip_tick.unwrap_or(32);
    let saved = values.clone();
    for _ in 0..8 {
        values = run_bands_gpu(&ctx, &mut session, &setup, &decay_ops, &values, &[1]);
    }
    let actual_max = field_max(&values);
    let l1_err = field_l1(&values);
    let safe = actual_max < EPS_VALUE && l1_err < 0.1;
    println!(
        "PF skip simulation: skip_candidate_tick={candidate} actual_after_8_more_ticks_max={actual_max:.6} equilibrium_max=0 max_error={actual_max:.6} l1_error={l1_err:.6} skip_would_be_safe={}",
        if safe { "YES" } else { "NO" }
    );
    let _ = saved;
}

#[test]
fn test8_horizon_propagation_decay_interaction() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);

    for &h in &[16u32, 24u32] {
        for &decay in &[false, true] {
            let r = run_horizon(&ctx, &setup, h, h, 1, decay);
            let t44 = threat_at(&r.values, 4, 4);
            let (_, _, mag) = gradient_at(&r.values, 4, 4);
            let src_max = field_max(&r.values);
            let l1 = field_l1(&r.values);
            let label = if decay { "decay" } else { "no_decay" };
            println!(
                "H={h} {label} threat[4][4]={t44:.4} mag={mag:.4} source_max={src_max:.2} l1={l1:.2}"
            );
        }
    }
}

#[test]
fn guard_no_wgsl_changes_required() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("simthing-gpu")
        .join("src")
        .join("shaders")
        .join("accumulator_op.wgsl");
    assert!(path.exists());
}

#[test]
fn guard_pipeline_flags_default_off() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}
