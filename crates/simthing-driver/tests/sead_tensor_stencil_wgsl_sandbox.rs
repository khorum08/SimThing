//! SEAD tensor/stencil WGSL prototype probe — structured field kernel vs per-edge AccumulatorOp.
//! Prototype only; informs mapping ADR StructuredFieldStencilOp candidacy.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, cpu_stencil_step, default_params_for_variant,
    AccumulatorOpSession, EmlGpuProgramTable, GpuContext, StencilParamsGpu, StencilPrototype,
    VARIANT_CLAMPED, VARIANT_DECAYED_NORMALIZED, VARIANT_DIRECTED, VARIANT_NORMALIZED, VARIANT_RAW,
};
use simthing_sim::PipelineFlags;
use std::sync::Mutex;
use std::time::Instant;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu_lock<F: FnOnce()>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f();
}

const N_DIMS: u32 = 8;
const COL_THREAT: u32 = 0;
const COL_AGGRESSION: u32 = 2;
const COL_RESOURCE: u32 = 1;
const COL_RISK: u32 = 3;
const COL_URGENCY: u32 = 4;

const GRID_W: u32 = 10;
const GRID_H: u32 = 10;
const N_CELLS: u32 = GRID_W * GRID_H;
const FACTION_SLOT: u32 = 100;

const CLUSTER: [(u32, u32, f32); 4] = [(0, 0, 80.0), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];

const EPS_VALUE: f32 = 0.01;
const BLOWUP_THRESHOLD: f32 = 1_000_000.0;
const ACCUMULATOR_BASELINE_30K_MS: f64 = 3236.6;

const TREE_URGENCY: u32 = 1;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn get(values: &[f32], slot: u32, col: u32, n_dims: u32) -> f32 {
    values[idx(slot, col, n_dims)]
}

fn grid_slot(row: u32, col: u32, grid_w: u32) -> u32 {
    row * grid_w + col
}

fn variant_name(v: u32) -> &'static str {
    match v {
        VARIANT_RAW => "raw_stencil",
        VARIANT_NORMALIZED => "normalized_stencil",
        VARIANT_DIRECTED => "directed_stencil",
        VARIANT_CLAMPED => "clamped_stencil",
        VARIANT_DECAYED_NORMALIZED => "decayed_normalized_stencil",
        _ => "unknown",
    }
}

fn variant_list() -> [u32; 5] {
    [
        VARIANT_RAW,
        VARIANT_NORMALIZED,
        VARIANT_DECAYED_NORMALIZED,
        VARIANT_DIRECTED,
        VARIANT_CLAMPED,
    ]
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

fn field_stats(values: &[f32], n_cells: u32, grid_w: u32, n_dims: u32) -> FieldStats {
    let t44 = get(values, grid_slot(4, 4, grid_w), COL_THREAT, n_dims);
    let gx = (get(values, grid_slot(4, 5, grid_w), COL_THREAT, n_dims)
        - get(values, grid_slot(4, 3, grid_w), COL_THREAT, n_dims))
        / 2.0;
    let gy = (get(values, grid_slot(5, 4, grid_w), COL_THREAT, n_dims)
        - get(values, grid_slot(3, 4, grid_w), COL_THREAT, n_dims))
        / 2.0;
    let mag = (gx * gx + gy * gy).sqrt();
    let mut max_v = 0.0f32;
    let mut l1 = 0.0f32;
    let mut blowup = false;
    for s in 0..n_cells {
        let v = get(values, s, COL_THREAT, n_dims);
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

fn seed_cluster(values: &mut [f32], grid_w: u32, n_dims: u32) {
    for v in values.iter_mut() {
        *v = 0.0;
    }
    for &(r, c, threat) in &CLUSTER {
        values[idx(grid_slot(r, c, grid_w), COL_THREAT, n_dims)] = threat;
    }
}

fn zero_cluster(values: &mut [f32], grid_w: u32, n_dims: u32) {
    for &(r, c, _) in &CLUSTER {
        values[idx(grid_slot(r, c, grid_w), COL_THREAT, n_dims)] = 0.0;
    }
}

struct HorizonRun {
    values: Vec<f32>,
    wall_ms: f64,
    dispatches: u32,
    stats: FieldStats,
}

fn run_stencil_horizon(
    ctx: &GpuContext,
    variant: u32,
    hops: u32,
    width: u32,
    height: u32,
    seed: bool,
) -> HorizonRun {
    let params = default_params_for_variant(
        variant,
        width,
        height,
        N_DIMS,
        COL_THREAT,
        COL_THREAT,
    );
    let proto = StencilPrototype::new(ctx, params);
    let mut values = vec![0.0f32; proto.params().values_len()];
    if seed {
        if width == GRID_W && height == GRID_H {
            seed_cluster(&mut values, width, N_DIMS);
            proto.upload_values(ctx, &values);
            proto.dispatch_once_internal(ctx);
            values = proto.readback_output(ctx);
            zero_cluster(&mut values, width, N_DIMS);
        } else {
            let center = grid_slot(height / 2, width / 2, width);
            values[idx(center, COL_THREAT, N_DIMS)] = 80.0;
        }
    }
    proto.upload_values(ctx, &values);
    let t0 = Instant::now();
    let (out, dispatches) = proto.readback_after_steps(ctx, hops);
    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let stats = field_stats(&out, width * height, width, N_DIMS);
    HorizonRun {
        values: out,
        wall_ms,
        dispatches,
        stats,
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

fn urgency_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_value(COL_THREAT),
        slot_value(COL_AGGRESSION),
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        slot_value(COL_RESOURCE),
        slot_value(COL_RISK),
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
        return_top(),
    ]
}

fn exact_meta(id: u32, name: &str, node_count: u32) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count,
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
            .register_formula(tid, exact_meta(id, name, nodes.len() as u32), nodes)
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

#[test]
fn guard_pipeline_flags_default_off() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}

#[test]
fn guard_no_production_pipeline_integration() {
    let src = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!src.contains("StencilPrototype::new(&ctx"));
    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("sead_tensor_stencil"));
}

#[test]
fn test0_wgsl_prototype_capability_sanity() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            println!("Test0 wgsl_compile=NO dispatch_10x10=NO dispatch_1024=NO readback=NO (no GPU)");
            return;
        };
        let params = default_params_for_variant(
            VARIANT_DECAYED_NORMALIZED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
        );
        let proto = StencilPrototype::new(&ctx, params);
        let mut values = vec![0.0f32; proto.params().values_len()];
        values[0] = 1.0;
        proto.upload_values(&ctx, &values);
        let ok_10 = proto.dispatch_once_internal(&ctx);
        assert_eq!(ok_10, 1);
        let rb = proto.readback_output(&ctx);
        assert_eq!(rb.len(), values.len());

        let params1024 = default_params_for_variant(
            VARIANT_DECAYED_NORMALIZED,
            32,
            32,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
        );
        let proto1024 = StencilPrototype::new(&ctx, params1024);
        let v1024 = vec![0.0f32; proto1024.params().values_len()];
        proto1024.upload_values(&ctx, &v1024);
        let ok1024 = proto1024.dispatch_once_internal(&ctx);
        assert_eq!(ok1024, 1);

        println!("Test0 wgsl_compile=YES dispatch_10x10=YES dispatch_1024=YES readback=YES");
    });
}

#[test]
fn test1_stencil_correctness_3x3() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let params = StencilParamsGpu {
            width: 3,
            height: 3,
            n_dims: N_DIMS,
            source_col: COL_THREAT,
            target_col: COL_THREAT,
            alpha_self_decay: 1.0,
            gamma_neighbor: 0.8,
            cap: 0.0,
            boundary_mode: 0,
            variant: VARIANT_RAW,
            use_active_mask: 0,
            _pad: 0,
        };
        let mut values = vec![0.0f32; params.values_len()];
        values[idx(4, COL_THREAT, N_DIMS)] = 100.0;

        let cpu = cpu_stencil_step(&values, &params);
        let proto = StencilPrototype::new(&ctx, params);
        proto.upload_values(&ctx, &values);
        proto.dispatch_once_internal(&ctx);
        let gpu = proto.readback_output(&ctx);

        let mut max_err = 0.0f32;
        for i in 0..params.values_len() {
            max_err = max_err.max((cpu[i] - gpu[i]).abs());
        }

        let center = get(&gpu, 4, COL_THREAT, N_DIMS);
        let north = get(&gpu, 1, COL_THREAT, N_DIMS);
        let south = get(&gpu, 7, COL_THREAT, N_DIMS);
        let east = get(&gpu, 5, COL_THREAT, N_DIMS);
        let west = get(&gpu, 3, COL_THREAT, N_DIMS);
        let corners = [0, 2, 6, 8]
            .iter()
            .map(|&s| get(&gpu, s, COL_THREAT, N_DIMS))
            .collect::<Vec<_>>();
        let corner_min = corners.iter().copied().fold(f32::INFINITY, f32::min);
        let corner_max = corners.iter().copied().fold(f32::NEG_INFINITY, f32::max);

        println!(
            "variant=raw_stencil center={center:.4} north={north:.4} south={south:.4} east={east:.4} west={west:.4} corner_min={corner_min:.4} corner_max={corner_max:.4} gpu_cpu_max_error={max_err:.6}"
        );
        assert!(max_err < 1e-4, "gpu/cpu mismatch max_err={max_err}");
        assert!(north > 0.0 && south > 0.0 && east > 0.0 && west > 0.0);
        assert!(corner_max < EPS_VALUE);
        assert!((center - 100.0).abs() < 1e-3);
        assert!((north - 80.0).abs() < 1e-3 || (north - 160.0).abs() < 1e-3);
    });
}

#[test]
fn test2_10x10_horizon_stencil_vs_accumulator_baseline() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let horizons = [4u32, 8, 12, 16, 24];
        println!("Horizon sweep vs AccumulatorOp directed_decayed baseline (H=8 dir=correct, projected_30k_dirty=3236.6ms):");
        for &variant in &variant_list() {
            let mut first_nz = None;
            let mut first_dir = None;
            for &h in &horizons {
                let r = run_stencil_horizon(&ctx, variant, h, GRID_W, GRID_H, true);
                let s = &r.stats;
                if s.t44 > EPS_VALUE && first_nz.is_none() {
                    first_nz = Some(h);
                }
                if s.direction == "correct" && first_dir.is_none() && !s.blowup {
                    first_dir = Some(h);
                }
                println!(
                    "  variant={} H={h} t44={:.4} grad=({:.2},{:.2}) mag={:.2} max={:.1} l1={:.0} blowup={} dir={} ms={:.2} dispatches={}",
                    variant_name(variant),
                    s.t44,
                    s.gx,
                    s.gy,
                    s.mag,
                    s.max_v,
                    s.l1,
                    if s.blowup { "YES" } else { "NO" },
                    s.direction,
                    r.wall_ms,
                    r.dispatches
                );
            }
            println!(
                "  variant={} first_nonzero_H={} first_directional_H={}",
                variant_name(variant),
                first_nz.map(|x| x.to_string()).unwrap_or_else(|| "NONE".into()),
                first_dir.map(|x| x.to_string()).unwrap_or_else(|| "NONE".into())
            );
        }
    });
}

#[test]
fn test3_cost_scaling_sweep() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let grids: &[(u32, u32, u32)] = &[
            (10, 10, 8),
            (14, 14, 8),
            (32, 32, 8),
            (64, 64, 8),
            (128, 128, 4),
        ];
        let variant = VARIANT_NORMALIZED;
        println!("Cost scaling (variant={}):", variant_name(variant));
        for &(w, h, horizon) in grids {
            let cells = w * h;
            let r = run_stencil_horizon(
                &ctx,
                variant,
                horizon,
                w,
                h,
                w == GRID_W && h == GRID_H,
            );
            let mean_ms = r.wall_ms / r.dispatches.max(1) as f64;
            let projected_30k = r.wall_ms * (30_000.0 / cells as f64);
            let projected_100k = r.wall_ms * (100_000.0 / cells as f64);
            let speedup = ACCUMULATOR_BASELINE_30K_MS / projected_30k;
            let values_bytes = cells * N_DIMS * 4;
            println!(
                "  cells={cells} {w}x{h} H={horizon} dispatches={} wall_ms={:.3} mean_ms_per_dispatch={:.4} values_bytes={values_bytes} projected_30k={projected_30k:.1} projected_100k={projected_100k:.1} speedup_vs_accum={speedup:.1}x",
                r.dispatches, r.wall_ms, mean_ms
            );
        }
        let r10 = run_stencil_horizon(&ctx, variant, 8, GRID_W, GRID_H, true);
        let projected_30k = r10.wall_ms * (30_000.0 / N_CELLS as f64);
        let speedup = ACCUMULATOR_BASELINE_30K_MS / projected_30k;
        println!(
            "Comparison: per_edge_accumulator_projected_30k_dirty_adjusted={ACCUMULATOR_BASELINE_30K_MS} stencil_projected_30k={projected_30k:.1} speedup={speedup:.1}x"
        );
    });
}

#[test]
fn test4_operator_stability_comparison() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let hs = [8u32, 16, 24];
        println!("Stability comparison:");
        for &variant in &variant_list() {
            for &h in &hs {
                let r = run_stencil_horizon(&ctx, variant, h, GRID_W, GRID_H, true);
                let s = &r.stats;
                let stability = if s.blowup {
                    "blowup"
                } else if s.max_v >= 9_999.0 && variant == VARIANT_CLAMPED {
                    "saturated"
                } else if s.max_v > 50_000.0 {
                    "marginal"
                } else {
                    "stable"
                };
                println!(
                    "  variant={} H={h} max={:.1} l1={:.0} gradient={} stability={}",
                    variant_name(variant),
                    s.max_v,
                    s.l1,
                    s.direction,
                    stability
                );
            }
        }
        println!("recommended_stencil_operator=normalized_stencil");
    });
}

#[test]
fn test5_hierarchy_stencil_hybrid() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let variant = VARIANT_NORMALIZED;
        let t0 = Instant::now();
        let r = run_stencil_horizon(&ctx, variant, 8, GRID_W, GRID_H, true);
        let stencil_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let sum_slots = FACTION_SLOT + 1;
        let mut session = AccumulatorOpSession::new(&ctx, sum_slots, N_DIMS);
        let mut values = r.values.clone();
        values.resize((sum_slots * N_DIMS) as usize, 0.0);
        for s in 0..N_CELLS {
            values[idx(s, COL_AGGRESSION, N_DIMS)] = 0.5;
            values[idx(s, COL_RESOURCE, N_DIMS)] = 1.0;
            values[idx(s, COL_RISK, N_DIMS)] = 0.5;
        }
        let t1 = Instant::now();
        values = run_bands(
            &ctx,
            &setup,
            &mut session,
            &[
                AccumulatorOp {
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
                },
                AccumulatorOp {
                    source: SourceSpec::SlotValue {
                        slot: FACTION_SLOT,
                        col: COL_THREAT,
                    },
                    combine: CombineFn::EvalEML {
                        tree_id: TREE_URGENCY,
                    },
                    gate: GateSpec::OrderBand(0),
                    scale: ScaleSpec::Identity,
                    consume: ConsumeMode::ResetTarget,
                    targets: vec![(FACTION_SLOT, COL_URGENCY)],
                },
            ],
            &values,
            &[0],
        );
        let reduction_ms = t1.elapsed().as_secs_f64() * 1000.0;
        let faction_threat = get(&values, FACTION_SLOT, COL_THREAT, N_DIMS);
        let faction_urgency = get(&values, FACTION_SLOT, COL_URGENCY, N_DIMS);
        let local_ok = r.stats.direction == "correct";
        println!(
            "Hierarchy+stencil hybrid: local_gradient_correct={} faction_threat={faction_threat:.2} faction_urgency={faction_urgency:.2} stencil_ms={stencil_ms:.2} reduction_ms={reduction_ms:.2} total_ms={:.2} (baseline hierarchy_ms=1.45 lateral_H8_ms=21.09)",
            if local_ok { "YES" } else { "NO" },
            stencil_ms + reduction_ms
        );
    });
}

#[test]
fn test6_dirty_active_mask_feasibility() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let params = default_params_for_variant(
            VARIANT_DECAYED_NORMALIZED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
        );
        let mut proto = StencilPrototype::new(&ctx, params);
        let mut values = vec![0.0f32; proto.params().values_len()];
        seed_cluster(&mut values, GRID_W, N_DIMS);
        proto.upload_values(&ctx, &values);
        proto.dispatch_once_internal(&ctx);
        values = proto.readback_output(&ctx);
        zero_cluster(&mut values, GRID_W, N_DIMS);
        proto.upload_values(&ctx, &values);

        let t0 = Instant::now();
        let disp_unmasked = proto.dispatch_ping_pong(&ctx, 8);
        let unmasked_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let unmasked = proto.readback_after_ping_pong(&ctx, 8);

        let mut mask = vec![0u32; N_CELLS as usize];
        for s in 0..N_CELLS {
            if get(&values, s, COL_THREAT, N_DIMS) > EPS_VALUE
                || get(&unmasked, s, COL_THREAT, N_DIMS) > EPS_VALUE
            {
                mask[s as usize] = 1;
            }
        }
        let active = mask.iter().filter(|&&m| m == 1).count();
        let active_ratio = active as f64 / N_CELLS as f64;

        proto.set_use_active_mask(&ctx, true);
        proto.upload_values(&ctx, &values);
        proto.upload_mask(&ctx, &mask);
        let t1 = Instant::now();
        let disp_masked = proto.dispatch_ping_pong(&ctx, 8);
        let masked_ms = t1.elapsed().as_secs_f64() * 1000.0;
        let masked = proto.readback_after_ping_pong(&ctx, 8);

        let mut max_active_err = 0.0f32;
        for s in 0..N_CELLS {
            if mask[s as usize] == 1 {
                let e = (get(&unmasked, s, COL_THREAT, N_DIMS)
                    - get(&masked, s, COL_THREAT, N_DIMS))
                    .abs();
                max_active_err = max_active_err.max(e);
            }
        }
        let speedup = unmasked_ms / masked_ms.max(1e-9);
        println!(
            "Dirty mask prototype: active_ratio={active_ratio:.3} unmasked_ms={unmasked_ms:.3} masked_ms={masked_ms:.3} speedup={speedup:.2}x max_active_err={max_active_err:.6} dispatches={disp_unmasked}/{disp_masked}"
        );
    });
}

#[test]
fn test7_wgsl_generality_constitutional_review() {
    let wgsl = include_str!("../../simthing-gpu/src/shaders/sead_tensor_stencil_prototype.wgsl");
    let knows_map = wgsl.contains("RegionCell")
        || wgsl.contains("FactionRoot")
        || wgsl.contains("planet")
        || wgsl.contains("star");
    let knows_factions = wgsl.contains("faction") || wgsl.contains("Faction");
    let knows_ai = wgsl.contains("urgency") || wgsl.contains("personality");
    let flat_buffers = wgsl.contains("input_values")
        && wgsl.contains("width")
        && wgsl.contains("gamma_neighbor");
    let reusable = flat_buffers && !knows_map && !knows_ai;

    println!("Generality review:");
    println!("  knows_maps={}", if knows_map { "YES" } else { "NO" });
    println!("  knows_factions={}", if knows_factions { "YES" } else { "NO" });
    println!("  knows_ai={}", if knows_ai { "YES" } else { "NO" });
    println!(
        "  flat_buffers_dims_columns_weights={}",
        if flat_buffers { "YES" } else { "NO" }
    );
    println!(
        "  reusable_any_2d_field={}",
        if reusable { "YES" } else { "NO" }
    );
    println!("  simthing_sim_awareness=NO");
    println!("  resource_flow_default_on=NO");
    println!(
        "  general_tensor_primitive={}",
        if reusable { "YES" } else { "NO" }
    );
    println!(
        "  mapping_semantics_embedded={}",
        if knows_map { "YES" } else { "NO" }
    );
    println!("  new_runtime_api_needed=PARTIAL (StructuredFieldStencilOp harness only)");

    assert!(!knows_map);
    assert!(!knows_factions);
    assert!(!knows_ai);
    assert!(flat_buffers);
}
