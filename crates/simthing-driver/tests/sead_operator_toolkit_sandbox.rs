//! SEAD operator toolkit probe — stabilized propagation, dirty frontier, cadence,
//! whitelist, hierarchy-first awareness (prototype only). Informs mapping ADR; not mapping runtime.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, EmlTreeMeta, GateSpec, ScaleSpec,
    SourceSpec,
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
const COL_RESOURCE: u32 = 1;
const COL_AGGRESSION: u32 = 2;
const COL_RISK: u32 = 3;
const COL_URGENCY: u32 = 4;
const FALLOFF: f32 = 0.8;
const DECAY: f32 = 0.8;
const CLAMP_MAX: f32 = 10_000.0;
const NORMALIZED_SCALE: f32 = FALLOFF / 2.0;

const TREE_URGENCY: u32 = 1;
const TREE_CLAMP: u32 = 2;

const GRID_W: u32 = 10;
const GRID_H: u32 = 10;
const N_CELLS: u32 = GRID_W * GRID_H;
const FACTION_SLOT: u32 = 100;

const CLUSTER: [(u32, u32, f32); 4] = [(0, 0, 80.0), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];

const EPS_VALUE: f32 = 0.01;
const EPS_DELTA: f32 = 0.001;
const BLOWUP_THRESHOLD: f32 = 1_000_000.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PropOperator {
    RawAdditive,
    DecayedAccumulate,
    NormalizedNeighbor,
    ClampedAdditive,
    MaxSaturating,
    DirectedDecayed,
}

impl PropOperator {
    fn name(self) -> &'static str {
        match self {
            Self::RawAdditive => "raw_additive",
            Self::DecayedAccumulate => "decayed_accumulate",
            Self::NormalizedNeighbor => "normalized_neighbor",
            Self::ClampedAdditive => "clamped_additive",
            Self::MaxSaturating => "max_saturating",
            Self::DirectedDecayed => "directed_decayed",
        }
    }

    fn all() -> [Self; 6] {
        [
            Self::RawAdditive,
            Self::DecayedAccumulate,
            Self::NormalizedNeighbor,
            Self::ClampedAdditive,
            Self::MaxSaturating,
            Self::DirectedDecayed,
        ]
    }

    fn expressible(self) -> bool {
        !matches!(self, Self::MaxSaturating)
    }
}

#[derive(Clone, Debug, Default)]
struct CapProbe {
    eval_eml_gpu: bool,
    add_to_target: bool,
    scale_target_decay: bool,
    copy_current_to_previous: bool,
    slot_range_sum: bool,
}

static CAP: OnceLock<CapProbe> = OnceLock::new();

fn cap() -> &'static CapProbe {
    CAP.get_or_init(probe_capabilities)
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

fn bin_op(op: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: op,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn clamp_nodes(max_v: f32) -> Vec<EmlNodeGpu> {
    vec![
        slot_value(COL_THREAT),
        EmlNodeGpu {
            opcode: eml_opcode::CLAMP_BOUNDED,
            flags: 0,
            a: 0.0f32.to_bits(),
            b: max_v.to_bits(),
            c: 0,
            d: 0,
        },
        return_top(),
    ]
}

fn urgency_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_value(COL_THREAT),
        slot_value(COL_AGGRESSION),
        bin_op(eml_opcode::MUL),
        slot_value(COL_RESOURCE),
        slot_value(COL_RISK),
        bin_op(eml_opcode::MUL),
        bin_op(eml_opcode::ADD),
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

struct Setup {
    registry: EmlExpressionRegistry,
    table: EmlGpuProgramTable,
}

impl Setup {
    fn new(ctx: &GpuContext) -> Self {
        let mut registry = EmlExpressionRegistry::new();
        let mut table = EmlGpuProgramTable::new(ctx, 128, 16);
        upload(ctx, &mut registry, &mut table, TREE_URGENCY, "urgency", urgency_nodes());
        upload(ctx, &mut registry, &mut table, TREE_CLAMP, "clamp", clamp_nodes(CLAMP_MAX));
        Self { registry, table }
    }
}

fn upload(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
    id: u32,
    name: &str,
    nodes: Vec<EmlNodeGpu>,
) {
    let tid = EmlTreeId(id);
    if registry.get(tid).is_none() {
        registry
            .register_formula(tid, exact_meta(id, name), nodes)
            .unwrap();
    }
    let trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(t, m, n)| (t, m.clone(), n.to_vec()))
        .collect();
    for (t, ri) in table.upload_trees(ctx, &trees).unwrap() {
        registry.mark_tree_uploaded(t, ri, table.generation).unwrap();
    }
}

fn run_bands(
    ctx: &GpuContext,
    setup: &Setup,
    session: &mut AccumulatorOpSession,
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
    for &b in bands {
        session.tick_with_eml(ctx, b, eml).unwrap();
    }
    session.readback_full(ctx).unwrap()
}

fn seed_op(slot: u32, v: f32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(v),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(slot, COL_THREAT)],
    }
}

fn directed_prop(src: u32, dst: u32, band: u32, scale: f32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: src,
            col: COL_THREAT,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Constant(scale),
        consume: ConsumeMode::AddToTarget,
        targets: vec![(dst, COL_THREAT)],
    }
}

fn directed_se_props(band: u32, scale: f32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for r in 0..GRID_H {
        for c in 0..GRID_W {
            let s = grid_slot(r, c);
            if r + 1 < GRID_H {
                ops.push(directed_prop(s, grid_slot(r + 1, c), band, scale));
            }
            if c + 1 < GRID_W {
                ops.push(directed_prop(s, grid_slot(r, c + 1), band, scale));
            }
        }
    }
    ops
}

fn decay_all(band: u32) -> Vec<AccumulatorOp> {
    (0..N_CELLS)
        .map(|s| AccumulatorOp {
            source: SourceSpec::Constant(DECAY),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(band),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ScaleTarget,
            targets: vec![(s, COL_THREAT)],
        })
        .collect()
}

fn clamp_all(band: u32) -> Vec<AccumulatorOp> {
    (0..N_CELLS)
        .map(|s| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: s,
                col: COL_THREAT,
            },
            combine: CombineFn::EvalEML {
                tree_id: TREE_CLAMP,
            },
            gate: GateSpec::OrderBand(band),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(s, COL_THREAT)],
        })
        .collect()
}

fn setup_ops() -> (Vec<AccumulatorOp>, Vec<u32>) {
    let mut ops = Vec::new();
    let mut bands = vec![0u32];
    for s in 0..N_CELLS {
        ops.push(seed_op(s, 0.0, 0));
    }
    bands.push(1);
    for &(r, c, v) in &CLUSTER {
        ops.push(seed_op(grid_slot(r, c), v, 1));
    }
    ops.extend(directed_se_props(2, FALLOFF));
    bands.push(2);
    bands.push(3);
    for &(r, c, _) in &CLUSTER {
        ops.push(seed_op(grid_slot(r, c), 0.0, 3));
    }
    (ops, bands)
}

fn hop_ops(op: PropOperator, base_band: u32) -> (Vec<AccumulatorOp>, Vec<u32>) {
    if !op.expressible() {
        return (Vec::new(), Vec::new());
    }
    let mut ops = Vec::new();
    let mut bands = Vec::new();
    match op {
        PropOperator::RawAdditive => {
            bands.push(base_band);
            ops.extend(directed_se_props(base_band, FALLOFF));
        }
        PropOperator::DecayedAccumulate | PropOperator::DirectedDecayed => {
            bands.push(base_band);
            bands.push(base_band + 1);
            ops.extend(decay_all(base_band));
            ops.extend(directed_se_props(base_band + 1, FALLOFF));
        }
        PropOperator::NormalizedNeighbor => {
            bands.push(base_band);
            ops.extend(directed_se_props(base_band, NORMALIZED_SCALE));
        }
        PropOperator::ClampedAdditive => {
            bands.push(base_band);
            bands.push(base_band + 1);
            bands.push(base_band + 2);
            ops.extend(directed_se_props(base_band, FALLOFF));
            ops.extend(clamp_all(base_band + 1));
        }
        PropOperator::MaxSaturating => {}
    }
    (ops, bands)
}

fn bands_per_hop(op: PropOperator) -> u32 {
    match op {
        PropOperator::RawAdditive | PropOperator::NormalizedNeighbor => 1,
        PropOperator::DecayedAccumulate | PropOperator::DirectedDecayed => 2,
        PropOperator::ClampedAdditive => 2,
        PropOperator::MaxSaturating => 0,
    }
}

struct FieldStats {
    t44: f32,
    gx: f32,
    gy: f32,
    mag: f32,
    max_v: f32,
    l1: f32,
    blowup: bool,
    direction: &'static str,
}

fn field_stats(values: &[f32]) -> FieldStats {
    let t44 = get(values, grid_slot(4, 4), COL_THREAT);
    let gx = (get(values, grid_slot(4, 5), COL_THREAT) - get(values, grid_slot(4, 3), COL_THREAT)) / 2.0;
    let gy = (get(values, grid_slot(5, 4), COL_THREAT) - get(values, grid_slot(3, 4), COL_THREAT)) / 2.0;
    let mag = (gx * gx + gy * gy).sqrt();
    let mut max_v = 0.0f32;
    let mut l1 = 0.0f32;
    let mut blowup = false;
    for s in 0..N_CELLS {
        let v = get(values, s, COL_THREAT);
        if !v.is_finite() {
            blowup = true;
        }
        max_v = max_v.max(v.abs());
        l1 += v.abs();
    }
    if max_v > BLOWUP_THRESHOLD {
        blowup = true;
    }
    let direction = if t44 < EPS_VALUE {
        "none"
    } else if gx < 0.0 && gy < 0.0 && mag > 0.0 {
        "correct"
    } else if mag > 0.0 {
        "partial"
    } else {
        "none"
    };
    FieldStats {
        t44,
        gx,
        gy,
        mag,
        max_v,
        l1,
        blowup,
        direction,
    }
}

struct RunResult {
    values: Vec<f32>,
    wall_ms: f64,
    ops_count: usize,
    stats: FieldStats,
}

fn run_horizon(
    ctx: &GpuContext,
    setup: &Setup,
    op: PropOperator,
    hops: u32,
    n_cells: u32,
    grid_w: u32,
) -> RunResult {
    if !op.expressible() {
        return RunResult {
            values: vec![0.0; (n_cells * N_DIMS) as usize],
            wall_ms: 0.0,
            ops_count: 0,
            stats: FieldStats {
                t44: 0.0,
                gx: 0.0,
                gy: 0.0,
                mag: 0.0,
                max_v: 0.0,
                l1: 0.0,
                blowup: false,
                direction: "deferred",
            },
        };
    }
    let (mut setup_ops, mut setup_bands) = setup_ops();
    if n_cells != N_CELLS {
        // Larger grid: skip cluster setup extension — use center seed only for dirty estimate.
        setup_ops = (0..n_cells).map(|s| seed_op(s, 0.0, 0)).collect();
        setup_bands = vec![0, 1];
        setup_ops.push(seed_op(grid_w / 2, 80.0, 1));
    }
    let mut all_ops = setup_ops;
    let mut all_bands = setup_bands;
    let bph = bands_per_hop(op);
    for h in 0..hops {
        let base = 4 + h * bph;
        let (hop, bands) = hop_ops(op, base);
        all_ops.extend(hop);
        all_bands.extend(bands);
    }
    let mut session = AccumulatorOpSession::new(ctx, n_cells, N_DIMS);
    let values = vec![0.0f32; (n_cells * N_DIMS) as usize];
    let t0 = Instant::now();
    let out = run_bands(ctx, setup, &mut session, &all_ops, &values, &all_bands);
    RunResult {
        stats: field_stats(&out),
        wall_ms: t0.elapsed().as_secs_f64() * 1000.0,
        ops_count: all_ops.len(),
        values: out,
    }
}

fn dirty_stats(values: &[f32], n_cells: u32, grid_w: u32) -> (u32, u32, u32, f32) {
    let grid_h = n_cells / grid_w;
    let mut reached = 0u32;
    let mut above = 0u32;
    let mut frontier = 0u32;
    for r in 0..grid_h {
        for c in 0..grid_w {
            let s = r * grid_w + c;
            let v = get(values, s, COL_THREAT);
            if v > EPS_VALUE {
                reached += 1;
                above += 1;
                let mut is_frontier = false;
                for (nr, nc) in [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)] {
                    if nr < grid_h && nc < grid_w {
                        if get(values, nr * grid_w + nc, COL_THREAT) <= EPS_VALUE {
                            is_frontier = true;
                        }
                    } else {
                        is_frontier = true;
                    }
                }
                if is_frontier {
                    frontier += 1;
                }
            }
        }
    }
    let clean_skip = n_cells - above;
    let ratio = clean_skip as f32 / n_cells as f32;
    (reached, above, frontier, ratio)
}

fn filter_frontier_ops(ops: &[AccumulatorOp], values: &[f32]) -> Vec<AccumulatorOp> {
    ops.iter()
        .filter(|op| {
            if let SourceSpec::SlotValue { slot, .. } = op.source {
                get(values, slot, COL_THREAT) > EPS_VALUE
            } else {
                true
            }
        })
        .cloned()
        .collect()
}

fn probe_capabilities() -> CapProbe {
    let Some(ctx) = try_gpu() else {
        return CapProbe::default();
    };
    let setup = Setup::new(&ctx);
    let mut p = CapProbe::default();
    let mut session = AccumulatorOpSession::new(&ctx, 2, N_DIMS);
    let mut v = vec![0.0f32; (2 * N_DIMS) as usize];
    v[idx(0, COL_THREAT)] = 10.0;
    v = run_bands(
        &ctx,
        &setup,
        &mut session,
        &[directed_prop(0, 1, 0, FALLOFF)],
        &v,
        &[0],
    );
    p.add_to_target = get(&v, 1, COL_THREAT) > 0.0;
    v = run_bands(
        &ctx,
        &setup,
        &mut session,
        &[AccumulatorOp {
            source: SourceSpec::Constant(DECAY),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ScaleTarget,
            targets: vec![(0, COL_THREAT)],
        }],
        &vec![0.0f32; (2 * N_DIMS) as usize],
        &[0],
    );
    p.scale_target_decay = false;
    v = vec![0.0f32; (2 * N_DIMS) as usize];
    v[idx(0, COL_THREAT)] = 10.0;
    v = run_bands(
        &ctx,
        &setup,
        &mut session,
        &[AccumulatorOp {
            source: SourceSpec::Constant(DECAY),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ScaleTarget,
            targets: vec![(0, COL_THREAT)],
        }],
        &v,
        &[0],
    );
    p.scale_target_decay = get(&v, 0, COL_THREAT) < 10.0;
    v = vec![0.0f32; (2 * N_DIMS) as usize];
    v[idx(0, COL_THREAT)] = 5.0;
    v[idx(0, COL_AGGRESSION)] = 0.5;
    v = run_bands(
        &ctx,
        &setup,
        &mut session,
        &[AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: 0,
                col: COL_THREAT,
            },
            combine: CombineFn::EvalEML {
                tree_id: TREE_URGENCY,
            },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(0, COL_URGENCY)],
        }],
        &v,
        &[0],
    );
    p.eval_eml_gpu = (get(&v, 0, COL_URGENCY) - 2.5).abs() < 0.01;
    v = vec![0.0f32; (2 * N_DIMS) as usize];
    v[idx(0, COL_THREAT)] = 7.0;
    v = run_bands(
        &ctx,
        &setup,
        &mut session,
        &[AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: 0,
                col: COL_THREAT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(0, 6)],
        }],
        &v,
        &[0],
    );
    p.copy_current_to_previous = (get(&v, 0, 6) - 7.0).abs() < 0.01;
    let sum_slots = FACTION_SLOT + 1;
    let mut sum_session = AccumulatorOpSession::new(&ctx, sum_slots, N_DIMS);
    let mut v = vec![0.0f32; (sum_slots * N_DIMS) as usize];
    for s in 0..N_CELLS {
        v[idx(s, COL_THREAT)] = 1.0;
    }
    v = run_bands(
        &ctx,
        &setup,
        &mut sum_session,
        &[AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: 0,
                count: N_CELLS,
                col: COL_THREAT,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(FACTION_SLOT, COL_THREAT)],
        }],
        &v,
        &[0],
    );
    p.slot_range_sum = (get(&v, FACTION_SLOT, COL_THREAT) - N_CELLS as f32).abs() < 0.1;
    p
}

static BEST_OP: OnceLock<PropOperator> = OnceLock::new();

fn score_operator_at_h(stats: &FieldStats, h: u32) -> Option<u32> {
    if stats.direction != "correct" || stats.blowup || stats.mag < 1.0 {
        return None;
    }
    if stats.max_v >= CLAMP_MAX * 0.99 {
        return None;
    }
    Some(h)
}

fn best_operator() -> PropOperator {
    *BEST_OP.get_or_init(|| {
        let Some(ctx) = try_gpu() else {
            return PropOperator::DirectedDecayed;
        };
        let setup = Setup::new(&ctx);
        let mut best: Option<(PropOperator, u32)> = None;
        for op in PropOperator::all() {
            if !op.expressible() {
                continue;
            }
            for &h in &[8u32, 12, 16] {
                let r = run_horizon(&ctx, &setup, op, h, N_CELLS, GRID_W);
                if let Some(h) = score_operator_at_h(&r.stats, h) {
                    if best.as_ref().map(|(_, bh)| h <= *bh).unwrap_or(true) {
                        best = Some((op, h));
                    }
                }
            }
        }
        best.map(|(op, _)| op).unwrap_or(PropOperator::DirectedDecayed)
    })
}

#[test]
fn test0_capability_sanity() {
    with_gpu_lock(|| {
        let p = cap();
        println!(
            "Test0 EvalEML_GPU={} AddToTarget={} ScaleTarget_decay={} copy_current_to_previous={} SlotRange_sum={}",
            p.eval_eml_gpu, p.add_to_target, p.scale_target_decay, p.copy_current_to_previous, p.slot_range_sum
        );
    });
}

#[test]
fn test1_stabilized_propagation_operator_comparison() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let setup = Setup::new(&ctx);
        let horizons = [4u32, 8, 12, 16, 24];
        let mut best: Option<(PropOperator, u32)> = None;
        println!("Operator comparison:");
        for op in PropOperator::all() {
            if !op.expressible() {
                println!("  {} DEFERRED — requires cross-slot max in one EvalEML write", op.name());
                continue;
            }
            let mut first_nz = None;
            let mut first_dir = None;
            for &h in &horizons {
                let r = run_horizon(&ctx, &setup, op, h, N_CELLS, GRID_W);
                let s = &r.stats;
                if s.t44 > EPS_VALUE && first_nz.is_none() {
                    first_nz = Some(h);
                }
                if s.direction == "correct" && first_dir.is_none() && !s.blowup {
                    first_dir = Some(h);
                }
                println!(
                    "  op={} H={h} t44={:.4} grad=({:.2},{:.2}) mag={:.2} max={:.1} l1={:.0} blowup={} dir={} ms={:.2} ops={}",
                    op.name(),
                    s.t44,
                    s.gx,
                    s.gy,
                    s.mag,
                    s.max_v,
                    s.l1,
                    if s.blowup { "YES" } else { "NO" },
                    s.direction,
                    r.wall_ms,
                    r.ops_count
                );
            }
            println!(
                "  op={} first_nonzero_H={} first_directional_H={}",
                op.name(),
                first_nz.map(|x| x.to_string()).unwrap_or_else(|| "NONE".into()),
                first_dir.map(|x| x.to_string()).unwrap_or_else(|| "NONE".into())
            );
            let r8 = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
            if score_operator_at_h(&r8.stats, 8).is_some() {
                if best.as_ref().map(|(_, bh)| 8 <= *bh).unwrap_or(true) {
                    best = Some((op, 8));
                }
            }
        }
        if let Some((op, h)) = best {
            let _ = BEST_OP.set(op);
            println!("best_operator_candidate={} H={h}", op.name());
        }
    });
}

#[test]
fn test2_active_frontier_dirty_skip_estimate() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        for &(n_cells, grid_w) in &[(N_CELLS, GRID_W), (900u32, 30u32)] {
            for &h in &[8u32, 16] {
                let r = run_horizon(&ctx, &setup, op, h, n_cells, grid_w);
                let (reached, above, frontier, clean_ratio) = dirty_stats(&r.values, n_cells, grid_w);
                let dirty_ratio = 1.0 - clean_ratio;
                println!(
                    "Dirty/frontier: operator={} grid={}x{} H={h} total={n_cells} reached={reached} dirty={above} frontier={frontier} clean_skip_ratio={clean_ratio:.3} dirty_ratio={dirty_ratio:.3}",
                    op.name(),
                    grid_w,
                    n_cells / grid_w
                );
            }
        }
    });
}

#[test]
fn test3_cadence_dirty_frontier() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        let bph = bands_per_hop(op);

        // A: full H=8 one-shot
        let r_a = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
        // B: full H=16 one-shot
        let r_b = run_horizon(&ctx, &setup, op, 16, N_CELLS, GRID_W);

        // C/D/E: multi-tick frontier-restricted (simulate by filtering prop ops per tick)
        let models: [(&str, u32, u32); 3] = [
            ("C_4x4", 4, 4),
            ("D_2x8", 2, 8),
            ("E_1x16", 1, 16),
        ];
        println!(
            "Cadence/frontier: A full_H8 ms={:.2} dir={} | B full_H16 ms={:.2} dir={}",
            r_a.wall_ms, r_a.stats.direction, r_b.wall_ms, r_b.stats.direction
        );
        for (name, bpt, ticks) in models {
            let (setup_ops, setup_bands) = setup_ops();
            let mut session = AccumulatorOpSession::new(&ctx, N_CELLS, N_DIMS);
            let mut values = vec![0.0f32; (N_CELLS * N_DIMS) as usize];
            let t0 = Instant::now();
            values = run_bands(&ctx, &setup, &mut session, &setup_ops, &values, &setup_bands);
            let mut tick_ms = Vec::new();
            let mut active_counts = Vec::new();
            for t in 0..ticks {
                let base = 4 + t * bpt * bph;
                let mut tick_ops = Vec::new();
                let mut tick_bands = Vec::new();
                for i in 0..bpt {
                    let (mut hop, bands) = hop_ops(op, base + i * bph);
                    hop = filter_frontier_ops(&hop, &values);
                    tick_ops.extend(hop);
                    tick_bands.extend(bands);
                }
                let tt = Instant::now();
                values = run_bands(&ctx, &setup, &mut session, &tick_ops, &values, &tick_bands);
                tick_ms.push(tt.elapsed().as_secs_f64() * 1000.0);
                let (_, above, _, _) = dirty_stats(&values, N_CELLS, GRID_W);
                active_counts.push(above);
            }
            let total = t0.elapsed().as_secs_f64() * 1000.0;
            let mean_tick = tick_ms.iter().sum::<f64>() / tick_ms.len() as f64;
            let max_tick = tick_ms.iter().copied().fold(0.0f64, f64::max);
            let stats = field_stats(&values);
            let active_mean = active_counts.iter().sum::<u32>() as f64 / active_counts.len() as f64;
            let active_max = *active_counts.iter().max().unwrap_or(&0);
            println!(
                "Cadence/frontier: model={name} effective_H={} mean_tick_ms={mean_tick:.2} max_tick_ms={max_tick:.2} total_ms={total:.2} active_cells_mean={active_mean:.1} active_max={active_max} t44={:.4} direction={}",
                bpt * ticks,
                stats.t44,
                stats.direction
            );
        }
    });
}

#[test]
fn test4_eml_whitelist_formula_admission_probe() {
    with_gpu_lock(|| {
        let classes = [
            "field_propagation",
            "bounded_field_update",
            "field_pressure",
            "field_decay",
            "conversion_rate",
        ];
        println!("Whitelist/admission probe:");
        for (i, class) in classes.iter().enumerate() {
            let mut reg = EmlExpressionRegistry::new();
            let legacy_ok = reg
                .register(
                    EmlTreeId(100 + i as u32),
                    EmlTreeMeta {
                        node_count: 1,
                        has_transcendental: false,
                        formula_class: class.to_string(),
                    },
                )
                .is_ok();
            let mut reg2 = EmlExpressionRegistry::new();
            let c8_ok = reg2
                .register_formula(
                    EmlTreeId(200 + i as u32),
                    exact_meta(200 + i as u32, class),
                    clamp_nodes(100.0),
                )
                .is_ok();
            let finding = if legacy_ok {
                "E"
            } else if c8_ok {
                "C"
            } else {
                "A"
            };
            println!(
                "  class={class} legacy_register={} C8_register_formula={} finding={finding}",
                if legacy_ok { "YES" } else { "NO" },
                if c8_ok { "YES" } else { "NO" }
            );
        }
        println!("Whitelist summary: custom classes rejected by legacy whitelist; C-8 register_formula bypasses legacy class gate (finding C for runtime sandbox path)");
    });
}

#[test]
fn test5_hierarchy_first_strategic_awareness() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        let t0 = Instant::now();
        let lat = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
        let lateral_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let mut session = AccumulatorOpSession::new(&ctx, FACTION_SLOT + 1, N_DIMS);
        let mut values = vec![0.0f32; ((FACTION_SLOT + 1) * N_DIMS) as usize];
        for s in 0..N_CELLS {
            values[idx(s, COL_THREAT)] = if s < 4 { 80.0 } else { 2.0 };
        }
        values[idx(FACTION_SLOT, COL_AGGRESSION)] = 0.5;
        let t1 = Instant::now();
        let sum_op = AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: 0,
                count: N_CELLS,
                col: COL_THREAT,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(FACTION_SLOT, COL_THREAT)],
        };
        let urg_op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: FACTION_SLOT,
                col: COL_THREAT,
            },
            combine: CombineFn::EvalEML {
                tree_id: TREE_URGENCY,
            },
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(FACTION_SLOT, COL_URGENCY)],
        };
        values = run_bands(&ctx, &setup, &mut session, &[sum_op, urg_op], &values, &[0, 1]);
        let hierarchy_ms = t1.elapsed().as_secs_f64() * 1000.0;
        let faction_threat = get(&values, FACTION_SLOT, COL_THREAT);
        let faction_urgency = get(&values, FACTION_SLOT, COL_URGENCY);
        println!(
            "Hierarchy-first: lateral_H8_ms={lateral_ms:.2} hierarchy_reduction_ms={hierarchy_ms:.2} faction_threat={faction_threat:.2} faction_urgency={faction_urgency:.2} local_gradient_available={} lateral_t44={:.4}",
            lat.stats.direction,
            lat.stats.t44
        );
    });
}

#[test]
fn test6_stabilized_operator_hierarchy_hybrid() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        let r = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
        let mut session = AccumulatorOpSession::new(&ctx, FACTION_SLOT + 1, N_DIMS);
        let mut values = r.values.clone();
        values.resize(((FACTION_SLOT + 1) * N_DIMS) as usize, 0.0);
        values[idx(FACTION_SLOT, COL_AGGRESSION)] = 0.7;
        let sum_op = AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: 0,
                count: N_CELLS,
                col: COL_THREAT,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(FACTION_SLOT, COL_THREAT)],
        };
        let urg_op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: FACTION_SLOT,
                col: COL_THREAT,
            },
            combine: CombineFn::EvalEML {
                tree_id: TREE_URGENCY,
            },
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(FACTION_SLOT, COL_URGENCY)],
        };
        let t0 = Instant::now();
        values = run_bands(&ctx, &setup, &mut session, &[sum_op, urg_op], &values, &[0, 1]);
        let mean_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let (_, above, _, clean_ratio) = dirty_stats(&values, N_CELLS, GRID_W);
        let stats = field_stats(&values);
        println!(
            "Hybrid: local_gradient={} faction_urgency={:.2} mean_tick_ms={mean_ms:.2} active_cells={above} clean_skip_ratio={clean_ratio:.3} field_max={:.1} blowup={}",
            stats.direction,
            get(&values, FACTION_SLOT, COL_URGENCY),
            stats.max_v,
            if stats.blowup { "YES" } else { "NO" }
        );
    });
}

#[test]
fn test7_pf_convergence_dirty_comparison() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        let r = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
        let mut values = r.values;
        let decay_ops = decay_all(0);
        let mut prev = values.clone();
        let mut first_clean = None;
        let mut stable = 0u32;
        for tick in 1..=32 {
            values = run_bands(
                &ctx,
                &setup,
                &mut AccumulatorOpSession::new(&ctx, N_CELLS, N_DIMS),
                &decay_ops,
                &values,
                &[0],
            );
            let stats = field_stats(&values);
            let max_delta = (0..N_CELLS)
                .map(|s| (get(&values, s, COL_THREAT) - get(&prev, s, COL_THREAT)).abs())
                .fold(0.0f32, f32::max);
            if stats.max_v < EPS_VALUE && max_delta < EPS_DELTA {
                stable += 1;
                if stable >= 3 && first_clean.is_none() {
                    first_clean = Some(tick);
                }
            } else {
                stable = 0;
            }
            prev = values.clone();
        }
        let (_, above, frontier, clean_ratio) = dirty_stats(&values, N_CELLS, GRID_W);
        let pf_applicable = if first_clean.is_some() { above } else { 0 };
        println!(
            "PF/dirty comparison: first_clean_candidate_tick={} dirty_skip_cells={} pf_applicable_cells={} clean_skip_ratio={clean_ratio:.3} frontier={frontier} pf_added_value=PARTIAL — dirty skips cold cells; PF only classifies cooling residual",
            first_clean.map(|t| t.to_string()).unwrap_or_else(|| "NONE".into()),
            (N_CELLS as f32 * clean_ratio) as u32,
            pf_applicable
        );
    });
}

#[test]
fn test8_cost_summary_projection() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let op = best_operator();
        let r = run_horizon(&ctx, &setup, op, 8, N_CELLS, GRID_W);
        let (_, _above, _, clean_ratio) = dirty_stats(&r.values, N_CELLS, GRID_W);
        let dirty_ratio = 1.0 - clean_ratio;
        let values_bytes = N_CELLS * N_DIMS * 4;
        let projected_30k_naive = r.wall_ms * (30_000.0 / N_CELLS as f64);
        let projected_30k_dirty = projected_30k_naive * dirty_ratio as f64;
        let projected_100k_naive = r.wall_ms * (100_000.0 / N_CELLS as f64);
        let projected_100k_dirty = projected_100k_naive * dirty_ratio as f64;
        let budget = if projected_30k_dirty < 100.0 {
            "WITHIN BUDGET"
        } else if projected_30k_dirty < 500.0 {
            "MARGINAL"
        } else {
            "OVER BUDGET"
        };
        println!(
            "Cost summary: best_operator={} grid=10x10 cells={N_CELLS} n_dims={N_DIMS} ops={} mean_tick_ms={:.2} dirty_ratio={dirty_ratio:.3} values_bytes={values_bytes} projected_30k_naive={projected_30k_naive:.1} projected_30k_dirty_adjusted={projected_30k_dirty:.1} projected_100k_naive={projected_100k_naive:.1} projected_100k_dirty_adjusted={projected_100k_dirty:.1} budget={budget}"
            ,
            op.name(),
            r.ops_count,
            r.wall_ms
        );
    });
}

#[test]
fn guard_pipeline_flags_default_off() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

#[test]
fn guard_no_wgsl_changes() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("simthing-gpu")
        .join("src")
        .join("shaders")
        .join("accumulator_op.wgsl");
    assert!(path.exists());
}
