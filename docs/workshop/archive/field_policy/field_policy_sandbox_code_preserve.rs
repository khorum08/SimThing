//! FIELD_POLICY field-intelligence feasibility probe (prototype only).
//!
//! Staged validation of P1 locality, P2 personality-weighted EML, P3 dissipation,
//! gradient quality, scale/cost, and whitelist admissibility on existing
//! AccumulatorOp substrate. No mapping runtime. No new WGSL.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, EmlTreeMeta,
    GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext,
};
use simthing_sim::PipelineFlags;
fn guard_wgsl_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("simthing-gpu")
        .join("src")
        .join("shaders")
        .join("accumulator_op.wgsl")
}
use std::sync::OnceLock;
use std::time::Instant;

const N_DIMS: u32 = 6;
const COL_THREAT: u32 = 0;
const COL_RESOURCE: u32 = 1;
const COL_AGGRESSION: u32 = 2;
const COL_RISK: u32 = 3;
const COL_URGENCY: u32 = 4;
const COL_VELOCITY: u32 = 5;
const FALLOFF: f32 = 0.8;

const TREE_FALLOFF: u32 = 1;
const TREE_URGENCY: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum PropagationStaging {
    #[default]
    Unsupported,
    SameBandCascade,
    LaterBandCascade,
    NextTickPropagation,
}

#[derive(Clone, Debug, Default)]
struct SubstrateProbe {
    eval_eml_register: bool,
    eval_eml_gpu: bool,
    add_to_target_propagation: bool,
    propagation_staging: PropagationStaging,
    decay_without_erasure: bool,
    previous_values_supported: bool,
    whitelist_propagation: bool,
    whitelist_urgency: bool,
    whitelist_conversion_alias: bool,
    whitelist_bypass: bool,
}

static PROBE: OnceLock<SubstrateProbe> = OnceLock::new();

fn probe() -> &'static SubstrateProbe {
    PROBE.get_or_init(run_substrate_probe)
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

fn falloff_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_value(COL_THREAT),
        literal(FALLOFF),
        bin_op(eml_opcode::MUL),
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
            TREE_FALLOFF,
            exact_meta(TREE_FALLOFF, "falloff"),
            falloff_nodes(),
        );
        register_and_upload(
            ctx,
            &mut registry,
            &mut table,
            TREE_URGENCY,
            exact_meta(TREE_URGENCY, "urgency"),
            urgency_nodes(),
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

fn propagate_eml_op(src: u32, dst: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: src,
            col: COL_THREAT,
        },
        combine: CombineFn::EvalEML {
            tree_id: TREE_FALLOFF,
        },
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(dst, COL_THREAT)],
    }
}

fn self_decay_op(slot: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(FALLOFF),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ScaleTarget,
        targets: vec![(slot, COL_THREAT)],
    }
}

fn urgency_eml_op(slot: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot,
            col: COL_THREAT,
        },
        combine: CombineFn::EvalEML {
            tree_id: TREE_URGENCY,
        },
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(slot, COL_URGENCY)],
    }
}

fn reset_all_ops(slots: &[u32], band: u32) -> Vec<AccumulatorOp> {
    slots
        .chunks(4)
        .map(|chunk| AccumulatorOp {
            source: SourceSpec::Constant(0.0),
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(band),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: chunk.iter().map(|s| (*s, COL_THREAT)).collect(),
        })
        .collect()
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

fn urgency_cpu_oracle(threat: f32, resource: f32, aggression: f32, risk: f32) -> f32 {
    threat * aggression + resource * risk
}

fn run_urgency_gpu(
    ctx: &GpuContext,
    setup: &SandboxSetup,
    slot: u32,
    threat: f32,
    resource: f32,
    aggression: f32,
    risk: f32,
) -> Option<f32> {
    let n_slots = slot + 1;
    let mut values = vec![0.0f32; (n_slots * N_DIMS) as usize];
    set(&mut values, slot, COL_THREAT, threat);
    set(&mut values, slot, COL_RESOURCE, resource);
    set(&mut values, slot, COL_AGGRESSION, aggression);
    set(&mut values, slot, COL_RISK, risk);

    let mut session = AccumulatorOpSession::new(ctx, n_slots, N_DIMS);
    let ops = [urgency_eml_op(slot, 0)];
    let out = run_bands_gpu(ctx, &mut session, setup, &ops, &values, &[0]);
    Some(get(&out, slot, COL_URGENCY))
}

fn grid_slot(row: u32, col: u32, width: u32) -> u32 {
    row * width + col
}

fn nsew_prop_ops(width: u32, height: u32, band: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for r in 0..height {
        for c in 0..width {
            let s = grid_slot(r, c, width);
            if r > 0 {
                ops.push(propagate_op(s, grid_slot(r - 1, c, width), band));
            }
            if r + 1 < height {
                ops.push(propagate_op(s, grid_slot(r + 1, c, width), band));
            }
            if c > 0 {
                ops.push(propagate_op(s, grid_slot(r, c - 1, width), band));
            }
            if c + 1 < width {
                ops.push(propagate_op(s, grid_slot(r, c + 1, width), band));
            }
        }
    }
    ops
}

fn run_substrate_probe() -> SubstrateProbe {
    let Some(ctx) = try_gpu() else {
        eprintln!("substrate probe: no GPU — defaults remain false/unsupported");
        return SubstrateProbe::default();
    };
    let setup = SandboxSetup::new(&ctx);
    let mut probe = SubstrateProbe::default();

    probe.eval_eml_register = setup.registry.get(EmlTreeId(TREE_FALLOFF)).is_some();
    probe.eval_eml_gpu = run_urgency_gpu(&ctx, &setup, 0, 1.0, 0.0, 1.0, 0.0).is_some();

    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    let hop_ops = [propagate_op(0, 1, 2)];
    values = run_bands_gpu(&ctx, &mut session, &setup, &hop_ops, &values, &[2]);
    probe.add_to_target_propagation = get(&values, 1, COL_THREAT) > 0.0;

    // Same-band two-hop attempt.
    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    let same_band = [propagate_op(0, 1, 2), propagate_op(1, 2, 2)];
    values = run_bands_gpu(&ctx, &mut session, &setup, &same_band, &values, &[2]);
    let same_band_c2 = get(&values, 2, COL_THREAT);

    // Later-band two-hop attempt.
    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    let later_band = [propagate_op(0, 1, 2), propagate_op(1, 2, 3)];
    values = run_bands_gpu(&ctx, &mut session, &setup, &later_band, &values, &[2, 3]);
    let later_band_c2 = get(&values, 2, COL_THREAT);

    // Next-tick two-hop attempt.
    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[propagate_op(0, 1, 2)],
        &values,
        &[2],
    );
    let tick1_c1 = get(&values, 1, COL_THREAT);
    values = run_bands_gpu(
        &ctx,
        &mut session,
        &setup,
        &[propagate_op(1, 2, 2)],
        &values,
        &[2],
    );
    let next_tick_c2 = get(&values, 2, COL_THREAT);

    probe.propagation_staging = if same_band_c2 > 0.01 {
        PropagationStaging::SameBandCascade
    } else if later_band_c2 > 0.01 {
        PropagationStaging::LaterBandCascade
    } else if next_tick_c2 > 0.01 && tick1_c1 > 0.01 {
        PropagationStaging::NextTickPropagation
    } else {
        PropagationStaging::Unsupported
    };

    // Decay without erasure via ScaleTarget.
    let mut session = AccumulatorOpSession::new(&ctx, 1, N_DIMS);
    let mut values = vec![0.0f32; N_DIMS as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    for _ in 0..5 {
        values = run_bands_gpu(
            &ctx,
            &mut session,
            &setup,
            &[self_decay_op(0, 1)],
            &values,
            &[1],
        );
    }
    probe.decay_without_erasure =
        get(&values, 0, COL_THREAT) > 0.0 && get(&values, 0, COL_THREAT) < 10.0;

    // Previous-values: PARAM is tick dt; no PREVIOUS opcode in EML.
    probe.previous_values_supported = false;

    // Whitelist probes.
    let mut reg = EmlExpressionRegistry::new();
    probe.whitelist_propagation = reg
        .register(
            EmlTreeId(90),
            EmlTreeMeta {
                node_count: 1,
                has_transcendental: false,
                formula_class: "propagation_formula".into(),
            },
        )
        .is_err();
    probe.whitelist_urgency = reg
        .register(
            EmlTreeId(91),
            EmlTreeMeta {
                node_count: 1,
                has_transcendental: false,
                formula_class: "urgency_formula".into(),
            },
        )
        .is_err();
    probe.whitelist_conversion_alias = reg
        .register(
            EmlTreeId(92),
            EmlTreeMeta {
                node_count: 1,
                has_transcendental: false,
                formula_class: "conversion_rate".into(),
            },
        )
        .is_ok();
    probe.whitelist_bypass = setup.registry.get(EmlTreeId(TREE_URGENCY)).is_some();

    probe
}

fn staging_label(staging: PropagationStaging) -> &'static str {
    match staging {
        PropagationStaging::SameBandCascade => "same-band-cascade",
        PropagationStaging::LaterBandCascade => "later-band-cascade",
        PropagationStaging::NextTickPropagation => "next-tick-propagation",
        PropagationStaging::Unsupported => "unsupported",
    }
}

fn two_hop_bands(staging: PropagationStaging) -> (u32, u32) {
    match staging {
        PropagationStaging::SameBandCascade => (2, 2),
        PropagationStaging::LaterBandCascade => (2, 3),
        PropagationStaging::NextTickPropagation | PropagationStaging::Unsupported => (2, 2),
    }
}

#[test]
fn test0_substrate_staging_and_admissibility_probe() {
    let p = probe();
    println!(
        "Test0 eval_eml_register={} eval_eml_gpu={} add_to_target={} propagation_staging={} decay_without_erasure={} previous_values={} whitelist_prop={} whitelist_urgency={} whitelist_conversion={} whitelist_bypass={}",
        p.eval_eml_register,
        p.eval_eml_gpu,
        p.add_to_target_propagation,
        staging_label(p.propagation_staging),
        p.decay_without_erasure,
        if p.previous_values_supported {
            "YES"
        } else {
            "NO"
        },
        if p.whitelist_propagation {
            "rejected"
        } else {
            "accepted"
        },
        if p.whitelist_urgency {
            "rejected"
        } else {
            "accepted"
        },
        if p.whitelist_conversion_alias {
            "accepted"
        } else {
            "rejected"
        },
        p.whitelist_bypass,
    );
}

#[test]
fn test1_p2_formula_logic_personality_weight_discrimination() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let p = probe();

    let oracle_a = urgency_cpu_oracle(5.0, 3.0, 0.2, 0.1);
    let oracle_b = urgency_cpu_oracle(5.0, 3.0, 0.9, 0.1);
    assert!((oracle_a - 1.3).abs() < 0.001);
    assert!((oracle_b - 4.8).abs() < 0.001);
    assert!(oracle_b > oracle_a);

    let gpu_a = if p.eval_eml_gpu {
        run_urgency_gpu(&ctx, &setup, 0, 5.0, 3.0, 0.2, 0.1)
    } else {
        None
    };
    let gpu_b = if p.eval_eml_gpu {
        run_urgency_gpu(&ctx, &setup, 0, 5.0, 3.0, 0.9, 0.1)
    } else {
        None
    };

    match (gpu_a, gpu_b) {
        (Some(a), Some(b)) => {
            assert!((a - oracle_a).abs() < 0.01, "gpu_a={a} oracle={oracle_a}");
            assert!((b - oracle_b).abs() < 0.01, "gpu_b={b} oracle={oracle_b}");
            assert!(b > a);
            println!(
                "Test1 urgency_cpu_oracle[0.2]={oracle_a} urgency_cpu_oracle[0.9]={oracle_b} urgency_gpu_eml[0.2]={a} urgency_gpu_eml[0.9]={b}"
            );
        }
        _ => {
            println!(
                "Test1 PARTIAL urgency_cpu_oracle[0.2]={oracle_a} urgency_cpu_oracle[0.9]={oracle_b} urgency_gpu_eml=DEFERRED"
            );
        }
    }
}

#[test]
fn test2_p1_one_hop_locality_propagation() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let p = probe();
    assert!(p.add_to_target_propagation, "AddToTarget propagation required");

    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    let ops = [propagate_op(0, 1, 2)];
    values = run_bands_gpu(&ctx, &mut session, &setup, &ops, &values, &[2]);

    let c0 = get(&values, 0, COL_THREAT);
    let c1 = get(&values, 1, COL_THREAT);
    let c2 = get(&values, 2, COL_THREAT);
    let locality = c1 > 0.0 && c2.abs() < 0.01;
    assert!(c1 > 0.0, "cell_1 should receive local contribution");
    println!(
        "Test2 propagation_staging={} cell_0_threat_after={c0} cell_1_threat_after={c1} cell_2_threat_after={c2} locality_direct_skip_confirmed={}",
        staging_label(p.propagation_staging),
        if locality { "YES" } else { "NO" }
    );
}

#[test]
fn test3_p1_two_hop_propagation_using_discovered_staging() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let staging = probe().propagation_staging;
    if staging == PropagationStaging::Unsupported {
        println!("Test3 DEFERRED propagation_staging=unsupported");
        return;
    }

    let (b1, b2) = two_hop_bands(staging);
    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);

    if staging == PropagationStaging::NextTickPropagation {
        values = run_bands_gpu(
            &ctx,
            &mut session,
            &setup,
            &[propagate_op(0, 1, b1)],
            &values,
            &[b1],
        );
        values = run_bands_gpu(
            &ctx,
            &mut session,
            &setup,
            &[propagate_op(1, 2, b1)],
            &values,
            &[b1],
        );
    } else {
        let ops = [propagate_op(0, 1, b1), propagate_op(1, 2, b2)];
        let bands = if b1 == b2 { vec![b1] } else { vec![b1, b2] };
        values = run_bands_gpu(&ctx, &mut session, &setup, &ops, &values, &bands);
    }

    let c1 = get(&values, 1, COL_THREAT);
    let c2 = get(&values, 2, COL_THREAT);
    assert!(c1 > 0.0 && c2 > 0.0, "c1={c1} c2={c2}");
    assert!(c2 <= c1 + 0.01, "c2 should not exceed c1: c1={c1} c2={c2}");
    println!(
        "Test3 propagation_staging={} cell_1={c1:.4} cell_2={c2:.4}",
        staging_label(staging)
    );
}

#[test]
fn test4_p3_dissipation_attractor_convergence() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let p = probe();
    if !p.decay_without_erasure {
        println!("Test4 DEFERRED decay_model=unsupported_without_erasure");
        return;
    }

    let slots = [0u32, 1, 2];
    let mut session = AccumulatorOpSession::new(&ctx, 3, N_DIMS);
    let mut values = vec![0.0f32; (3 * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 10.0);
    set(&mut values, 1, COL_THREAT, 6.0);
    set(&mut values, 2, COL_THREAT, 4.0);
    let initial_max = slots
        .iter()
        .map(|&s| get(&values, s, COL_THREAT))
        .fold(0.0f32, f32::max);

    let decay_ops: Vec<_> = slots.iter().map(|&s| self_decay_op(s, 1)).collect();
    let mut tick5_max = initial_max;
    let mut tick10_max = initial_max;
    let mut tick20_max = initial_max;
    let mut monotone = true;

    for tick in 1..=20 {
        let prev = values.clone();
        values = run_bands_gpu(&ctx, &mut session, &setup, &decay_ops, &values, &[1]);
        let max_now = slots
            .iter()
            .map(|&s| get(&values, s, COL_THREAT))
            .fold(0.0f32, f32::max);
        for &s in &slots {
            if get(&values, s, COL_THREAT) > get(&prev, s, COL_THREAT) + 0.001 {
                monotone = false;
            }
            assert!(get(&values, s, COL_THREAT).is_finite());
            assert!(get(&values, s, COL_THREAT) >= 0.0);
        }
        if tick == 5 {
            tick5_max = max_now;
        }
        if tick == 10 {
            tick10_max = max_now;
        }
        if tick == 20 {
            tick20_max = max_now;
        }
    }

    assert!(tick10_max < initial_max);
    assert!(tick20_max < tick10_max);
    println!(
        "Test4 decay_model=ScaleTarget*0.8 initial_max={initial_max:.4} tick5_max={tick5_max:.4} tick10_max={tick10_max:.4} tick20_max={tick20_max:.4} monotone={}",
        if monotone { "YES" } else { "NO" }
    );
}

#[test]
fn test5_p1_p2_grid_propagation_3x3() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let staging = probe().propagation_staging;
    let width = 3u32;
    let n_cells = 9u32;
    let center = 4u32;

    let mut session = AccumulatorOpSession::new(&ctx, n_cells, N_DIMS);
    let mut values = vec![0.0f32; (n_cells * N_DIMS) as usize];
    let mut ops = reset_all_ops(&(0..n_cells).collect::<Vec<_>>(), 0);
    ops.push(seed_op(center, 100.0, 1));
    for cardinal in [1u32, 3, 5, 7] {
        ops.push(propagate_op(center, cardinal, 2));
    }
    values = run_bands_gpu(&ctx, &mut session, &setup, &ops, &values, &[0, 1, 2]);

    let north = get(&values, 1, COL_THREAT);
    let south = get(&values, 7, COL_THREAT);
    let east = get(&values, 5, COL_THREAT);
    let west = get(&values, 3, COL_THREAT);
    let corner_min = [0u32, 2, 6, 8]
        .iter()
        .map(|&s| get(&values, s, COL_THREAT))
        .fold(f32::INFINITY, f32::min);
    let corner_max = [0u32, 2, 6, 8]
        .iter()
        .map(|&s| get(&values, s, COL_THREAT))
        .fold(0.0f32, f32::max);

    assert!(north > 0.0 && south > 0.0 && east > 0.0 && west > 0.0);
    assert!(corner_max < 1.0, "corners should not receive direct one-hop from center");

    // Second hop using discovered staging.
    if staging != PropagationStaging::Unsupported {
        let (b1, b2) = two_hop_bands(staging);
        let prop_ops = nsew_prop_ops(width, width, b2);
        if staging == PropagationStaging::NextTickPropagation {
            values = run_bands_gpu(&ctx, &mut session, &setup, &prop_ops, &values, &[b2]);
        } else {
            values = run_bands_gpu(&ctx, &mut session, &setup, &prop_ops, &values, &[b2]);
        }
        let _ = b1;
    }

    let grad_x = (get(&values, 5, COL_THREAT) - get(&values, 3, COL_THREAT)) / 2.0;
    let grad_y = (get(&values, 7, COL_THREAT) - get(&values, 1, COL_THREAT)) / 2.0;
    let magnitude = (grad_x * grad_x + grad_y * grad_y).sqrt();
    let structured = magnitude > 0.0;
    for slot in 0..n_cells {
        assert!(get(&values, slot, COL_THREAT) >= 0.0);
    }
    println!(
        "Test5 center={} north={north} south={south} east={east} west={west} corner_min={corner_min} corner_max={corner_max} gradient_magnitude={magnitude:.4} gradient_structured={}",
        get(&values, center, COL_THREAT),
        if structured { "YES" } else { "NO" }
    );
}

#[test]
fn test6_faction_root_reduction_and_urgency_eml() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let p = probe();
    let faction = 3u32;
    let n_slots = 4u32;

    let mut values = vec![0.0f32; (n_slots * N_DIMS) as usize];
    set(&mut values, 0, COL_THREAT, 2.0);
    set(&mut values, 1, COL_THREAT, 5.0);
    set(&mut values, 2, COL_THREAT, 3.0);

    let sum_op = AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: 0,
            count: 3,
            col: COL_THREAT,
        },
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(1),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(faction, COL_THREAT)],
    };

    let mut session = AccumulatorOpSession::new(&ctx, n_slots, N_DIMS);
    values = run_bands_gpu(&ctx, &mut session, &setup, &[sum_op], &values, &[1]);
    let sum_reduced = get(&values, faction, COL_THREAT);
    assert!((sum_reduced - 10.0).abs() < 0.01, "sum={sum_reduced}");

    let oracle_hi = urgency_cpu_oracle(sum_reduced, 0.0, 0.5, 0.0);
    let oracle_lo = urgency_cpu_oracle(sum_reduced, 0.0, 0.1, 0.0);

    let gpu_hi = if p.eval_eml_gpu {
        run_urgency_gpu(&ctx, &setup, faction, sum_reduced, 0.0, 0.5, 0.0)
    } else {
        None
    };
    let gpu_lo = if p.eval_eml_gpu {
        run_urgency_gpu(&ctx, &setup, faction, sum_reduced, 0.0, 0.1, 0.0)
    } else {
        None
    };

    match (gpu_hi, gpu_lo) {
        (Some(hi), Some(lo)) => {
            assert!(hi > lo);
            println!(
                "Test6 sum_reduction={sum_reduced:.2} urgency_gpu_eml[0.5]={hi:.4} urgency_gpu_eml[0.1]={lo:.4} urgency_cpu_oracle[0.5]={oracle_hi:.4} urgency_cpu_oracle[0.1]={oracle_lo:.4}"
            );
        }
        _ => {
            assert!(oracle_hi > oracle_lo);
            println!(
                "Test6 PARTIAL sum_reduction={sum_reduced:.2} urgency_gpu_eml=DEFERRED urgency_cpu_oracle[0.5]={oracle_hi:.4} urgency_cpu_oracle[0.1]={oracle_lo:.4}"
            );
        }
    }
}

#[test]
fn test7_velocity_temporal_derivative_probe() {
    let p = probe();
    if !p.previous_values_supported {
        println!(
            "Test7 DEFERRED previous_values_supported=NO velocity=DEFERRED — previous/current read not available through existing EvalEML path."
        );
        return;
    }
    unreachable!("previous_values path not implemented");
}

#[test]
fn test8_scale_and_cost_probe() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);

    for &target_cells in &[200u32, 1000u32] {
        let side = (target_cells as f32).sqrt().round() as u32;
        let n_cells = side * side;
        let n_slots = n_cells;
        let width = side;

        let mut prop_ops = Vec::new();
        for band in 2..=3 {
            prop_ops.extend(nsew_prop_ops(width, side, band));
        }
        let center = grid_slot(side / 2, side / 2, width);
        let mut setup_ops = vec![seed_op(center, 50.0, 1)];
        setup_ops.extend(prop_ops.clone());

        let n_ops = setup_ops.len() as u32;
        let eml_trees = 2u32;
        let eml_nodes = (falloff_nodes().len() + urgency_nodes().len()) as u32;
        let order_bands = 4u32;
        let values_bytes = n_slots * N_DIMS * 4;
        let readback_bytes = values_bytes;

        let mut session = AccumulatorOpSession::new(&ctx, n_slots, N_DIMS);
        let mut values = vec![0.0f32; (n_slots * N_DIMS) as usize];
        let setup_t0 = Instant::now();
        let _ = run_bands_gpu(&ctx, &mut session, &setup, &setup_ops, &values, &[0, 1, 2, 3]);
        let setup_ms = setup_t0.elapsed().as_secs_f64() * 1000.0;

        let prop_only = prop_ops.clone();
        let mut tick_ms = Vec::new();
        for _ in 0..5 {
            let t0 = Instant::now();
            values = run_bands_gpu(&ctx, &mut session, &setup, &prop_only, &values, &[2, 3]);
            tick_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
        }
        let mean = tick_ms.iter().sum::<f64>() / tick_ms.len() as f64;
        let min = tick_ms.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = tick_ms.iter().cloned().fold(0.0f64, f64::max);
        let var = tick_ms.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / tick_ms.len() as f64;
        let stddev = var.sqrt();
        let projected_30k = mean * (30_000.0 / n_cells as f64);
        let projected_100k = mean * (100_000.0 / n_cells as f64);
        let ops_per_cell = n_ops as f64 / n_cells as f64;

        println!(
            "Scale probe: cells={n_cells} slots={n_slots} n_dims={N_DIMS} durable_columns=5 derived_outputs=2 accumulator_ops={n_ops} eml_trees={eml_trees} eml_nodes_total={eml_nodes} order_bands={order_bands} values_bytes={values_bytes} readback_bytes={readback_bytes} mean_tick_ms={mean:.3} min_tick_ms={min:.3} max_tick_ms={max:.3} stddev_tick_ms={stddev:.3} setup_ms={setup_ms:.3} ops_per_cell={ops_per_cell:.2} projected_30k_ms={projected_30k:.1} projected_100k_ms={projected_100k:.1}"
        );
    }
}

#[test]
fn test9_realistic_signal_gradient_quality() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let setup = SandboxSetup::new(&ctx);
    let width = 10u32;
    let height = 10u32;
    let n_cells = width * height;

    let cluster_a = [(0u32, 0u32, 80.0f32), (0, 1, 60.0), (1, 0, 60.0), (1, 1, 40.0)];
    let mut setup_ops = reset_all_ops(&(0..n_cells).collect::<Vec<_>>(), 0);
    for &(r, c, v) in &cluster_a {
        setup_ops.push(seed_op(grid_slot(r, c, width), v, 1));
    }
    // One-hop spread from cluster, then zero sources so later bands diffuse rather than re-inject.
    setup_ops.extend(nsew_prop_ops(width, height, 2));
    for &(r, c, _) in &cluster_a {
        setup_ops.push(seed_op(grid_slot(r, c, width), 0.0, 3));
    }

    let mut session = AccumulatorOpSession::new(&ctx, n_cells, N_DIMS);
    let mut values = vec![0.0f32; (n_cells * N_DIMS) as usize];

    let mut all_ops = setup_ops.clone();
    for b in 4..=9 {
        all_ops.extend(nsew_prop_ops(width, height, b));
    }
    let bands: Vec<u32> = (0..=9).collect();
    values = run_bands_gpu(&ctx, &mut session, &setup, &all_ops, &values, &bands);

    let t00 = get(&values, grid_slot(0, 0, width), COL_THREAT);
    let t22 = get(&values, grid_slot(2, 2, width), COL_THREAT);
    let t44 = get(&values, grid_slot(4, 4, width), COL_THREAT);
    let t66 = get(&values, grid_slot(6, 6, width), COL_THREAT);
    let t99 = get(&values, grid_slot(9, 9, width), COL_THREAT);

    let grad_x = (get(&values, grid_slot(4, 5, width), COL_THREAT)
        - get(&values, grid_slot(4, 3, width), COL_THREAT))
        / 2.0;
    let grad_y = (get(&values, grid_slot(5, 4, width), COL_THREAT)
        - get(&values, grid_slot(3, 4, width), COL_THREAT))
        / 2.0;
    let magnitude = (grad_x * grad_x + grad_y * grad_y).sqrt();

    let direction = if t44 < 1.0 {
        "PARTIAL"
    } else if grad_x < 0.0 && grad_y < 0.0 && magnitude > 0.0 {
        "YES"
    } else if magnitude > 0.0 {
        "NO"
    } else {
        "PARTIAL"
    };

    println!(
        "Test9 threat[0][0]={t00:.2} threat[2][2]={t22:.2} threat[4][4]={t44:.2} threat[6][6]={t66:.2} threat[9][9]={t99:.2} gradient_at_4_4=(x={grad_x:.4}, y={grad_y:.4}, mag={magnitude:.4}) gradient_direction_correct={direction}"
    );
}

#[test]
fn test10_eml_whitelist_production_admissibility_probe() {
    let p = probe();
    let finding = if p.whitelist_propagation && p.whitelist_urgency {
        if p.whitelist_conversion_alias {
            "B"
        } else {
            "A"
        }
    } else if p.whitelist_bypass {
        "C"
    } else {
        "D"
    };
    println!(
        "Test10 propagation_formula accepted: {} urgency_formula accepted: {} conversion_rate alias accepted: {} registration path bypasses whitelist: {} finding={}",
        if p.whitelist_propagation { "NO" } else { "YES" },
        if p.whitelist_urgency { "NO" } else { "YES" },
        if p.whitelist_conversion_alias { "YES" } else { "NO" },
        if p.whitelist_bypass { "YES" } else { "NO" },
        finding
    );
}

#[test]
fn guard_no_wgsl_changes_required() {
    assert!(guard_wgsl_path().exists());
}

#[test]
fn guard_pipeline_flags_default_off() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}
