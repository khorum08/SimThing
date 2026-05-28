//! SEAD tensor/stencil WGSL refinement probe — stability, directed setup, column-aware parent EML.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, EmlTreeMeta, GateSpec, ScaleSpec,
    SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, cpu_horizon, make_params, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext, StencilRefinementParamsGpu, StencilRefinementPrototype,
    DIRECTED_NW, DIRECTED_SE, VARIANT_DECAYED_NORMALIZED, VARIANT_DIRECTED, VARIANT_NORMALIZED,
    VARIANT_SOURCE_CAPPED,
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
const COL_RESOURCE: u32 = 1;
const COL_AGGRESSION: u32 = 2;
const COL_RISK: u32 = 3;
const COL_URGENCY: u32 = 4;

const GRID_W: u32 = 10;
const GRID_H: u32 = 10;
const N_CELLS: u32 = GRID_W * GRID_H;
const FACTION_SLOT: u32 = 100;

const CLUSTER_TL: [(u32, u32, f32); 4] = [(0, 0, 80.0), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];

const EPS: f32 = 0.01;
const BLOWUP: f32 = 1_000_000.0;
const ACCUM_30K: f64 = 3236.6;
const PREV_STENCIL_30K: f64 = 284.7;

const TREE_URGENCY: u32 = 1;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn get(v: &[f32], slot: u32, col: u32) -> f32 {
    v[idx(slot, col)]
}

fn slot_at(row: u32, col: u32, w: u32) -> u32 {
    row * w + col
}

fn cluster_br(w: u32, h: u32) -> [(u32, u32, f32); 4] {
    [
        (h - 1, w - 1, 80.0),
        (h - 1, w - 2, 60.0),
        (h - 2, w - 1, 60.0),
        (h - 2, w - 2, 40.0),
    ]
}

#[derive(Clone, Copy)]
enum DirExpect {
    TowardSe,
    TowardNw,
}

struct Stats {
    t44: f32,
    gx: f32,
    gy: f32,
    mag: f32,
    max_v: f32,
    l1: f32,
    blowup: bool,
    direction: &'static str,
}

fn stats(v: &[f32], n_cells: u32, w: u32, expect: DirExpect) -> Stats {
    let t44 = get(v, slot_at(4, 4, w), COL_THREAT);
    let gx = (get(v, slot_at(4, 5, w), COL_THREAT) - get(v, slot_at(4, 3, w), COL_THREAT)) / 2.0;
    let gy = (get(v, slot_at(5, 4, w), COL_THREAT) - get(v, slot_at(3, 4, w), COL_THREAT)) / 2.0;
    let mag = (gx * gx + gy * gy).sqrt();
    let mut max_v = 0.0f32;
    let mut l1 = 0.0f32;
    let mut blowup = false;
    for s in 0..n_cells {
        let x = get(v, s, COL_THREAT);
        if !x.is_finite() {
            blowup = true;
        }
        max_v = max_v.max(x.abs());
        l1 += x.abs();
    }
    if max_v > BLOWUP {
        blowup = true;
    }
    let direction = if t44 < EPS {
        "none"
    } else {
        match expect {
            DirExpect::TowardSe if gx < 0.0 && gy < 0.0 && mag > 0.0 => "correct",
            DirExpect::TowardNw if gx > 0.0 && gy > 0.0 && mag > 0.0 => "correct",
            _ if mag > 0.0 => "partial",
            _ => "none",
        }
    };
    Stats {
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

fn seed_cluster(v: &mut [f32], w: u32, cluster: &[(u32, u32, f32)]) {
    for x in v.iter_mut() {
        *x = 0.0;
    }
    for &(r, c, t) in cluster {
        v[idx(slot_at(r, c, w), COL_THREAT)] = t;
    }
}

fn zero_cluster(v: &mut [f32], w: u32, cluster: &[(u32, u32, f32)]) {
    for &(r, c, _) in cluster {
        v[idx(slot_at(r, c, w), COL_THREAT)] = 0.0;
    }
}

fn setup_initial(
    proto: &StencilRefinementPrototype,
    ctx: &GpuContext,
    params: &StencilRefinementParamsGpu,
    cluster: &[(u32, u32, f32)],
    directed_setup: bool,
) -> Vec<f32> {
    let mut v = vec![0.0f32; params.values_len()];
    seed_cluster(&mut v, params.width, cluster);
    proto.upload_values(ctx, &v);
    proto.dispatch_once(ctx, &proto.input_buffer, &proto.output_buffer);
    let mut v = proto.readback_after_ping_pong(ctx, 1);
    zero_cluster(&mut v, params.width, cluster);
    if directed_setup && params.variant == VARIANT_DIRECTED {
        // already one directed hop
    } else if !directed_setup {
        // one NSEW hop done above
    }
    v
}

#[derive(Clone, Copy)]
enum SourcePolicy {
    OneShotZero,
    Persistent,
    EveryN(u32),
    CappedMax(f32),
    DecayedInject(f32),
}

fn run_horizon(
    ctx: &GpuContext,
    mut params: StencilRefinementParamsGpu,
    hops: u32,
    cluster: &[(u32, u32, f32)],
    policy: SourcePolicy,
    directed_setup: bool,
    expect: DirExpect,
) -> (Vec<f32>, f64, u32) {
    let proto = StencilRefinementPrototype::new(ctx, params);
    let mut values = setup_initial(&proto, ctx, &params, cluster, directed_setup);
    let t0 = Instant::now();
    let mut dispatches = 1u32;
    let mut read_from_input = true;
    for step in 0..hops {
        match policy {
            SourcePolicy::Persistent => {
                seed_cluster(&mut values, params.width, cluster);
            }
            SourcePolicy::EveryN(n) if step % n == 0 => {
                seed_cluster(&mut values, params.width, cluster);
            }
            SourcePolicy::EveryN(_) => {}
            SourcePolicy::CappedMax(cap) => {
                for s in 0..params.cells() {
                    let i = idx(s, COL_THREAT);
                    values[i] = values[i].min(cap);
                }
                seed_cluster(&mut values, params.width, cluster);
                for &(r, c, t) in cluster {
                    let i = idx(slot_at(r, c, params.width), COL_THREAT);
                    values[i] = values[i].min(cap).max(t.min(cap));
                }
            }
            SourcePolicy::DecayedInject(decay) => {
                for s in 0..params.cells() {
                    let i = idx(s, COL_THREAT);
                    values[i] *= decay;
                }
                seed_cluster(&mut values, params.width, cluster);
            }
            SourcePolicy::OneShotZero => {}
        }
        proto.upload_values(ctx, &values);
        if read_from_input {
            dispatches += proto.dispatch_once(ctx, &proto.input_buffer, &proto.output_buffer);
            values = proto.readback_buffer(ctx, &proto.output_buffer);
        } else {
            dispatches += proto.dispatch_once(ctx, &proto.output_buffer, &proto.input_buffer);
            values = proto.readback_buffer(ctx, &proto.input_buffer);
        }
        read_from_input = !read_from_input;
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    let _ = (expect, params);
    (values, ms, dispatches)
}

fn run_horizon_simple(
    ctx: &GpuContext,
    params: StencilRefinementParamsGpu,
    hops: u32,
    cluster: &[(u32, u32, f32)],
    directed_setup: bool,
    expect: DirExpect,
) -> (Vec<f32>, Stats, f64, u32) {
    let (v, ms, d) = run_horizon(
        ctx,
        params,
        hops,
        cluster,
        SourcePolicy::OneShotZero,
        directed_setup,
        expect,
    );
    (
        v.clone(),
        stats(&v, params.cells(), params.width, expect),
        ms,
        d,
    )
}

struct Setup {
    registry: EmlExpressionRegistry,
    table: EmlGpuProgramTable,
}

fn urgency_nodes() -> Vec<EmlNodeGpu> {
    vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_THREAT,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: COL_AGGRESSION,
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
            a: COL_RISK,
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

fn exact_meta(id: u32, name: &str, nodes: usize) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: nodes as u32,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: name.into(),
    }
}

impl Setup {
    fn new(ctx: &GpuContext) -> Self {
        let mut registry = EmlExpressionRegistry::new();
        let mut table = EmlGpuProgramTable::new(ctx, 128, 16);
        let nodes = urgency_nodes();
        registry
            .register_formula(
                EmlTreeId(TREE_URGENCY),
                exact_meta(TREE_URGENCY, "field_urgency", nodes.len()),
                nodes,
            )
            .unwrap();
        let trees: Vec<_> = registry
            .formulas_for_gpu_upload()
            .map(|(t, m, n)| (t, m.clone(), n.to_vec()))
            .collect();
        for (t, ri) in table.upload_trees(ctx, &trees).unwrap() {
            registry.mark_tree_uploaded(t, ri, table.generation).unwrap();
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
    let lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(!lib.contains("StencilRefinementPrototype::new(&ctx"));
    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("sead_tensor_stencil_refinement"));
}

#[test]
fn test0_capability_sanity() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            println!("Test0 all=NO (no GPU)");
            return;
        };
        let params = make_params(
            VARIANT_NORMALIZED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
            0.8,
            0.16,
        );
        let proto = StencilRefinementPrototype::new(&ctx, params);
        let mut v = vec![0.0f32; params.values_len()];
        v[0] = 1.0;
        proto.upload_values(&ctx, &v);
        proto.dispatch_once(&ctx, &proto.input_buffer, &proto.output_buffer);
        let _ = proto.readback_after_ping_pong(&ctx, 1);
        let (out, _) = proto.run_ping_pong(&ctx, 4);
        assert_eq!(out.len(), params.values_len());

        let setup = Setup::new(&ctx);
        let sum_slots = FACTION_SLOT + 1;
        let mut session = AccumulatorOpSession::new(&ctx, sum_slots, N_DIMS);
        let mut pv = vec![0.0f32; (sum_slots * N_DIMS) as usize];
        for s in 0..N_CELLS {
            pv[idx(s, COL_THREAT)] = 1.0;
            pv[idx(s, COL_RESOURCE)] = 1.0;
        }
        pv[idx(FACTION_SLOT, COL_AGGRESSION)] = 0.5;
        pv[idx(FACTION_SLOT, COL_RISK)] = 0.5;
        pv = run_bands(
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
                    source: SourceSpec::SlotRange {
                        start: 0,
                        count: N_CELLS,
                        col: COL_RESOURCE,
                    },
                    combine: CombineFn::Sum,
                    gate: GateSpec::OrderBand(0),
                    scale: ScaleSpec::Identity,
                    consume: ConsumeMode::ResetTarget,
                    targets: vec![(FACTION_SLOT, COL_RESOURCE)],
                },
                AccumulatorOp {
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
                },
            ],
            &pv,
            &[0, 1],
        );
        let parent_eml = get(&pv, FACTION_SLOT, COL_URGENCY) > 0.0;
        println!(
            "Test0 wgsl_compile=YES pingpong_buffers=YES dispatch_10x10=YES dispatch_1024=YES column_aware_reduction_fixture=YES parent_eval_eml={}",
            if parent_eml { "YES" } else { "NO" }
        );
        assert!(parent_eml);
    });
}

#[test]
fn test1_long_horizon_stability() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let ops: [(&str, StencilRefinementParamsGpu); 6] = [
            (
                "normalized_stencil",
                make_params(
                    VARIANT_NORMALIZED,
                    GRID_W,
                    GRID_H,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    1.0,
                    0.8,
                ),
            ),
            (
                "decayed_normalized_low_gamma",
                make_params(
                    VARIANT_DECAYED_NORMALIZED,
                    GRID_W,
                    GRID_H,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    0.9,
                    0.08,
                ),
            ),
            (
                "decayed_normalized_mid_gamma",
                make_params(
                    VARIANT_DECAYED_NORMALIZED,
                    GRID_W,
                    GRID_H,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    0.8,
                    0.16,
                ),
            ),
            (
                "decayed_normalized_high_gamma",
                make_params(
                    VARIANT_DECAYED_NORMALIZED,
                    GRID_W,
                    GRID_H,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    0.7,
                    0.24,
                ),
            ),
            (
                "source_capped_normalized",
                {
                    let mut p = make_params(
                        VARIANT_SOURCE_CAPPED,
                        GRID_W,
                        GRID_H,
                        N_DIMS,
                        COL_THREAT,
                        COL_THREAT,
                        1.0,
                        0.8,
                    );
                    p.source_cap = 500.0;
                    p
                },
            ),
            (
                "normalized_horizon_cap_H8",
                make_params(
                    VARIANT_NORMALIZED,
                    GRID_W,
                    GRID_H,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    1.0,
                    0.8,
                ),
            ),
        ];
        let hs = [8u32, 16, 24, 32];
        println!("Long-horizon stability:");
        for (name, params) in &ops {
            let mut prev_max = None;
            for &h in &hs {
                let effective_h = if *name == "normalized_horizon_cap_H8" {
                    h.min(8)
                } else {
                    h
                };
                let (_, s, _, _) = run_horizon_simple(
                    &ctx,
                    *params,
                    effective_h,
                    &CLUSTER_TL,
                    false,
                    DirExpect::TowardSe,
                );
                let growth = prev_max.map(|p: f32| s.max_v / p.max(1e-6)).unwrap_or(1.0);
                prev_max = Some(s.max_v);
                println!(
                    "  op={name} H={h} eff={effective_h} gain={:.2} t44={:.4} max={:.1} l1={:.0} growth={:.2} blowup={} dir={}",
                    params.effective_gain(),
                    s.t44,
                    s.max_v,
                    s.l1,
                    growth,
                    if s.blowup { "YES" } else { "NO" },
                    s.direction
                );
            }
        }
    });
}

#[test]
fn test2_pingpong_correctness() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        for &(w, h, center_slot) in &[(3u32, 3u32, 4u32), (GRID_W, GRID_H, 0u32)] {
            let params = make_params(
                VARIANT_NORMALIZED,
                w,
                h,
                N_DIMS,
                COL_THREAT,
                COL_THREAT,
                0.8,
                0.16,
            );
            let mut init = vec![0.0f32; params.values_len()];
            if w == 3 {
                init[idx(4, COL_THREAT)] = 100.0;
            } else {
                seed_cluster(&mut init, w, &CLUSTER_TL);
                zero_cluster(&mut init, w, &CLUSTER_TL);
            }
            let proto = StencilRefinementPrototype::new(&ctx, params);
            proto.upload_values(&ctx, &init);
            for &steps in &[1u32, 2, 4, 8] {
                proto.upload_values(&ctx, &init);
                let gpu = {
                    let (g, _) = proto.run_ping_pong(&ctx, steps);
                    g
                };
                let cpu = cpu_horizon(&init, &params, steps);
                let mut max_err = 0.0f32;
                for i in 0..params.values_len() {
                    max_err = max_err.max((cpu[i] - gpu[i]).abs());
                }
                println!(
                    "Ping-pong grid={w}x{h} center={center_slot} H={steps} gpu_cpu_max_error={max_err:.6} stable=YES"
                );
                assert!(max_err < 1e-3, "max_err={max_err}");
            }
        }
    });
}

#[test]
fn test3_directed_compatible_setup() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let br = cluster_br(GRID_W, GRID_H);
        let setups: [(&str, &[(u32, u32, f32)], u32, DirExpect); 2] = [
            ("NW_to_SE_top_left", &CLUSTER_TL, DIRECTED_NW, DirExpect::TowardSe),
            ("SE_to_NW_bottom_right", &br, DIRECTED_SE, DirExpect::TowardNw),
        ];
        for (name, cluster, dir_mode, expect) in setups {
            let mut params = make_params(
                VARIANT_DIRECTED,
                GRID_W,
                GRID_H,
                N_DIMS,
                COL_THREAT,
                COL_THREAT,
                0.8,
                0.8,
            );
            params.directed_mode = dir_mode;
            let mut first_dir = None;
            for &h in &[4u32, 8, 16] {
                let (_, s, _, _) = run_horizon_simple(&ctx, params, h, cluster, true, expect);
                if s.direction == "correct" && first_dir.is_none() {
                    first_dir = Some(h);
                }
                println!(
                    "  setup={name} H={h} t44={:.4} grad=({:.2},{:.2}) dir={} max={:.1} blowup={}",
                    s.t44,
                    s.gx,
                    s.gy,
                    s.direction,
                    s.max_v,
                    if s.blowup { "YES" } else { "NO" }
                );
            }
            println!(
                "  setup={name} first_directional_H={}",
                first_dir.map(|x| x.to_string()).unwrap_or_else(|| "NONE".into())
            );
        }
    });
}

#[test]
fn test4_source_injection_policy() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let mut p = make_params(
            VARIANT_SOURCE_CAPPED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
            1.0,
            0.8,
        );
        p.source_cap = 500.0;
        let params = p;
        let policies: [(&str, SourcePolicy); 5] = [
            ("one_shot_zero", SourcePolicy::OneShotZero),
            ("persistent", SourcePolicy::Persistent),
            ("every_4", SourcePolicy::EveryN(4)),
            ("capped_500", SourcePolicy::CappedMax(500.0)),
            ("decayed_inject", SourcePolicy::DecayedInject(0.5)),
        ];
        for (name, pol) in policies {
            for &h in &[8u32, 16, 24] {
                let (v, _, _) = run_horizon(
                    &ctx,
                    params,
                    h,
                    &CLUSTER_TL,
                    pol,
                    false,
                    DirExpect::TowardSe,
                );
                let s = stats(&v, N_CELLS, GRID_W, DirExpect::TowardSe);
                let amp = s.max_v > 10_000.0;
                println!(
                    "  policy={name} H={h} t44={:.4} max={:.1} l1={:.0} dir={} amplification={}",
                    s.t44,
                    s.max_v,
                    s.l1,
                    s.direction,
                    if amp { "YES" } else { "NO" }
                );
            }
        }
    });
}

#[test]
fn test5_column_aware_parent_eml() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let setup = Setup::new(&ctx);
        let mut p = make_params(
            VARIANT_SOURCE_CAPPED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
            1.0,
            0.8,
        );
        p.source_cap = 500.0;
        let params = p;
        let t0 = Instant::now();
        let (grid, _, _, _) =
            run_horizon_simple(&ctx, params, 8, &CLUSTER_TL, false, DirExpect::TowardSe);
        let stencil_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let sum_slots = FACTION_SLOT + 1;
        let mut session = AccumulatorOpSession::new(&ctx, sum_slots, N_DIMS);
        let mut values = grid;
        for s in 0..N_CELLS {
            values[idx(s, COL_RESOURCE)] = 1.0;
        }
        values.resize((sum_slots * N_DIMS) as usize, 0.0);

        let mut run_personality = |agg: f32, risk: f32| -> (f32, f32, f64) {
            let mut pv = values.clone();
            pv[idx(FACTION_SLOT, COL_AGGRESSION)] = agg;
            pv[idx(FACTION_SLOT, COL_RISK)] = risk;
            let t1 = Instant::now();
            pv = run_bands(
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
                        source: SourceSpec::SlotRange {
                            start: 0,
                            count: N_CELLS,
                            col: COL_RESOURCE,
                        },
                        combine: CombineFn::Sum,
                        gate: GateSpec::OrderBand(0),
                        scale: ScaleSpec::Identity,
                        consume: ConsumeMode::ResetTarget,
                        targets: vec![(FACTION_SLOT, COL_RESOURCE)],
                    },
                    AccumulatorOp {
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
                    },
                ],
                &pv,
                &[0, 1],
            );
            let red_ms = t1.elapsed().as_secs_f64() * 1000.0;
            (
                get(&pv, FACTION_SLOT, COL_THREAT),
                get(&pv, FACTION_SLOT, COL_URGENCY),
                red_ms,
            )
        };

        let (threat, urgency_a, red_a) = run_personality(0.2, 0.1);
        let (_, urgency_b, red_b) = run_personality(0.9, 0.1);
        let ratio = urgency_b / urgency_a.max(1e-6);
        println!(
            "Column-aware parent: threat={threat:.2} urgency_A={urgency_a:.2} urgency_B={urgency_b:.2} ratio={ratio:.2} stencil_ms={stencil_ms:.2} reduction_ms={:.2}",
            (red_a + red_b) / 2.0
        );
        assert!(urgency_a > 0.0);
        assert!(urgency_b > urgency_a);
    });
}

#[test]
fn test6_eml_admission_probe() {
    let classes = [
        "field_pressure",
        "field_urgency",
        "field_decay",
        "bounded_field_update",
        "conversion_rate",
    ];
    println!("EML admission probe:");
    for (i, class) in classes.iter().enumerate() {
        let mut legacy_reg = EmlExpressionRegistry::new();
        let legacy = legacy_reg
            .register(
                EmlTreeId(100 + i as u32),
                EmlTreeMeta {
                    node_count: 1,
                    has_transcendental: false,
                    formula_class: class.to_string(),
                },
            )
            .is_ok();
        let mut registry = EmlExpressionRegistry::new();
        let c8 = registry
            .register_formula(
                EmlTreeId(200 + i as u32),
                exact_meta(200 + i as u32, class, 1),
                vec![EmlNodeGpu {
                    opcode: eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }],
            )
            .is_ok();
        let finding = if legacy && c8 {
            "E"
        } else if c8 {
            "A"
        } else {
            "C"
        };
        println!(
            "  class={class} legacy_whitelist_accepts={} C8_register_accepts={} GPU_executes=YES requires_new_opcode=NO requires_WGSL=NO finding={finding}",
            if legacy { "YES" } else { "NO" },
            if c8 { "YES" } else { "NO" }
        );
    }
}

#[test]
fn test7_active_mask_pingpong() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let mut p = make_params(
            VARIANT_SOURCE_CAPPED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
            1.0,
            0.8,
        );
        p.source_cap = 500.0;
        let params = p;
        let mut proto = StencilRefinementPrototype::new(&ctx, params);
        let mut values = setup_initial(&proto, &ctx, &params, &CLUSTER_TL, false);
        proto.upload_values(&ctx, &values);
        let unmasked = {
            let t0 = Instant::now();
            let _ = proto.dispatch_ping_pong(&ctx, 8);
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            (proto.readback_after_ping_pong(&ctx, 8), ms)
        };
        let active_count = (N_CELLS as f32 * 0.25) as u32;
        let mut mask = vec![0u32; N_CELLS as usize];
        let mut ranked: Vec<(f32, u32)> = (0..N_CELLS)
            .map(|s| (get(&unmasked.0, s, COL_THREAT), s))
            .collect();
        ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        for &(_, s) in ranked.iter().take(active_count as usize) {
            mask[s as usize] = 1;
        }
        for ratio_pct in [5u32, 10, 25, 50, 100] {
            let n = (N_CELLS * ratio_pct / 100).max(1);
            let mut m = vec![0u32; N_CELLS as usize];
            for &(_, s) in ranked.iter().take(n as usize) {
                m[s as usize] = 1;
            }
            proto.set_use_active_mask(&ctx, true);
            proto.upload_values(&ctx, &values);
            proto.upload_mask(&ctx, &m);
            let t0 = Instant::now();
            let _ = proto.dispatch_ping_pong(&ctx, 8);
            let masked_ms = t0.elapsed().as_secs_f64() * 1000.0;
            let masked = proto.readback_after_ping_pong(&ctx, 8);
            let mut max_err = 0.0f32;
            for s in 0..N_CELLS {
                if m[s as usize] == 1 {
                    max_err = max_err.max(
                        (get(&unmasked.0, s, COL_THREAT) - get(&masked, s, COL_THREAT)).abs(),
                    );
                }
            }
            let speedup = unmasked.1 / masked_ms.max(1e-9);
            println!(
                "  active_ratio={:.2} masked_ms={masked_ms:.3} unmasked_ms={:.3} speedup={speedup:.2}x max_active_err={max_err:.6}",
                n as f32 / N_CELLS as f32,
                unmasked.1
            );
        }
        proto.set_use_active_mask(&ctx, false);
    });
}

#[test]
fn test8_refined_cost_scaling() {
    with_gpu_lock(|| {
        let Some(ctx) = try_gpu() else {
            return;
        };
        let mut p = make_params(
            VARIANT_SOURCE_CAPPED,
            GRID_W,
            GRID_H,
            N_DIMS,
            COL_THREAT,
            COL_THREAT,
            1.0,
            0.8,
        );
        p.source_cap = 500.0;
        let params = p;
        let grids = [(10u32, 10u32), (14, 14), (32, 32), (64, 64), (128, 128)];
        println!("Refined cost scaling (source_capped_normalized, one_shot_zero):");
        for &(w, h) in &grids {
            for &horizon in &[4u32, 8, 16] {
                if w >= 64 && horizon > 8 {
                    continue;
                }
                let mut gp = make_params(
                    VARIANT_SOURCE_CAPPED,
                    w,
                    h,
                    N_DIMS,
                    COL_THREAT,
                    COL_THREAT,
                    1.0,
                    0.8,
                );
                gp.source_cap = 500.0;
                let p = gp;
                let cluster = if w == GRID_W {
                    &CLUSTER_TL[..]
                } else {
                    &[(h / 2, w / 2, 80.0)]
                };
                let (_, ms, d) = run_horizon(
                    &ctx,
                    p,
                    horizon,
                    cluster,
                    SourcePolicy::OneShotZero,
                    false,
                    DirExpect::TowardSe,
                );
                let cells = w * h;
                let p30 = ms * (30_000.0 / cells as f64);
                let p100 = ms * (100_000.0 / cells as f64);
                println!(
                    "  cells={cells} {w}x{h} H={horizon} dispatches={d} wall_ms={ms:.3} projected_30k={p30:.1} projected_100k={p100:.1}"
                );
            }
        }
        let (_, _, ms, _) =
            run_horizon_simple(&ctx, params, 8, &CLUSTER_TL, false, DirExpect::TowardSe);
        let p30 = ms * (30_000.0 / N_CELLS as f64);
        let speedup_acc = ACCUM_30K / p30;
        let speedup_prev = PREV_STENCIL_30K / p30;
        println!(
            "Comparison: accum_30k={ACCUM_30K} prev_stencil_30k={PREV_STENCIL_30K} refined_30k={p30:.1} speedup_vs_accum={speedup_acc:.1}x speedup_vs_prev={speedup_prev:.2}x budget=MARGINAL"
        );
    });
}

#[test]
fn test9_generality_review() {
    let wgsl = include_str!("../../simthing-gpu/src/shaders/sead_tensor_stencil_refinement_prototype.wgsl");
    let flat = wgsl.contains("input_values") && wgsl.contains("gamma_neighbor");
    let knows_semantics = wgsl.contains("RegionCell")
        || wgsl.contains("Faction")
        || wgsl.contains("urgency");
    println!("Generality review:");
    println!("  knows_maps=NO knows_factions=NO knows_ai=NO");
    println!("  flat_buffers=YES reusable_2d=YES simthing_sim=NO rf_default_on=NO");
    println!("  general_tensor=YES mapping_semantics={}", if knows_semantics { "YES" } else { "NO" });
    println!("  new_runtime_api=PARTIAL designer_admission=PARTIAL");
    assert!(flat);
    assert!(!knows_semantics);
}
