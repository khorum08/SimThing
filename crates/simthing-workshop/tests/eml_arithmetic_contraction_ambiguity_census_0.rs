//! EML-ARITHMETIC-SEMANTICS-0 — uniqueness census (ADD hits + SUB reachability).
//!
//! Enumerates admitted EML programs for ADD nodes whose both immediate
//! producers are MUL results, measures CPU-twin / interpreted-WGSL behaviour,
//! and records SSA-JIT as not-an-arm for OrdinaryAccumulatorEvalEml.
//! DA `5192270934`: two+ MUL → ADD is authored UNFUSED (`U`); no tie-break;
//! non-empty set is no longer a STOP.
//! DA `5193244394` / remand `5193312235`: the same walk also counts MUL→SUB
//! dataflows (reachability of fused-multiply-subtract).

use simthing_core::{
    eml_nodes, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SlotIndex,
    SourceSpec,
};
use simthing_core::eml_nodes::EmlNode;
use simthing_gpu::{
    set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable, GpuContext,
    PackedAccumulatorUpload,
};
use simthing_kernel::{LnConsumerGadgets, SoftStepPolicyConditional, SoftmaxWeightGadget};
use simthing_spec::{compile_eml_gadget, EmlGadgetCompileOptions, EmlGadgetInstanceSpec};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProducerKind {
    Mul,
    Other,
}

#[derive(Clone, Debug)]
struct AmbiguousHit {
    program: &'static str,
    source: &'static str,
    add_index: usize,
    mul_lhs_index: usize,
    mul_rhs_index: usize,
    nodes: Vec<EmlNode>,
    columns: u32,
    /// Probe row layout matching SLOT indices used by `nodes`.
    probe_row: Vec<f32>,
    params: [f32; 4],
}

/// Walk results: two-MUL→ADD hits, and every MUL→SUB immediate dataflow edge.
fn classify_contraction_walk(
    nodes: &[EmlNode],
) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
    let mut stack: Vec<(ProducerKind, usize)> = Vec::new();
    let mut add_hits = Vec::new();
    let mut mul_into_sub = Vec::new();
    for (idx, node) in nodes.iter().enumerate() {
        match node.opcode {
            eml_nodes::opcode::LITERAL_F32
            | eml_nodes::opcode::SLOT_VALUE
            | eml_nodes::opcode::PARAM => {
                stack.push((ProducerKind::Other, idx));
            }
            eml_nodes::opcode::ADD => {
                let rhs = stack.pop().expect("ADD rhs");
                let lhs = stack.pop().expect("ADD lhs");
                if lhs.0 == ProducerKind::Mul && rhs.0 == ProducerKind::Mul {
                    add_hits.push((idx, lhs.1, rhs.1));
                }
                stack.push((ProducerKind::Other, idx));
            }
            eml_nodes::opcode::SUB => {
                let rhs = stack.pop().expect("SUB rhs");
                let lhs = stack.pop().expect("SUB lhs");
                // DA 5193244394: record MUL→SUB reachability in this walk.
                if lhs.0 == ProducerKind::Mul {
                    mul_into_sub.push((idx, lhs.1));
                }
                if rhs.0 == ProducerKind::Mul {
                    mul_into_sub.push((idx, rhs.1));
                }
                stack.push((ProducerKind::Other, idx));
            }
            eml_nodes::opcode::MUL
            | eml_nodes::opcode::DIV
            | eml_nodes::opcode::MIN
            | eml_nodes::opcode::MAX
            | eml_nodes::opcode::CMP_LT
            | eml_nodes::opcode::CMP_LE
            | eml_nodes::opcode::CMP_GT
            | eml_nodes::opcode::CMP_GE
            | eml_nodes::opcode::CMP_EQ => {
                let _rhs = stack.pop().expect("bin rhs");
                let _lhs = stack.pop().expect("bin lhs");
                let kind = if node.opcode == eml_nodes::opcode::MUL {
                    ProducerKind::Mul
                } else {
                    ProducerKind::Other
                };
                stack.push((kind, idx));
            }
            eml_nodes::opcode::SELECT => {
                let _c = stack.pop().expect("select c");
                let _b = stack.pop().expect("select b");
                let _a = stack.pop().expect("select a");
                stack.push((ProducerKind::Other, idx));
            }
            eml_nodes::opcode::NEG
            | eml_nodes::opcode::ABS
            | eml_nodes::opcode::FLOOR
            | eml_nodes::opcode::CLAMP_BOUNDED
            | eml_nodes::opcode::CLAMP_FLOORED
            | eml_nodes::opcode::EXP
            | eml_nodes::opcode::LN => {
                let _v = stack.pop().expect("unary");
                stack.push((ProducerKind::Other, idx));
            }
            eml_nodes::opcode::RETURN_TOP => {}
            _ => {
                // Unknown opcode — refuse to classify as non-ambiguous.
                panic!("census refuses unknown opcode {} at node {idx}", node.opcode);
            }
        }
    }
    (add_hits, mul_into_sub)
}

fn to_gpu(nodes: &[EmlNode]) -> Vec<EmlNodeGpu> {
    nodes
        .iter()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect()
}

/// Reference fused multiply-add via f64 evaluation. Used only as a measurement
/// discriminator when the host `f32::mul_add` is not hardware-fused (common on
/// default Windows MSVC targets without `+fma`). Not a language meaning.
fn fma_ref(a: f32, b: f32, c: f32) -> f32 {
    ((f64::from(a) * f64::from(b)) + f64::from(c)) as f32
}

fn oracle_triple(a: f32, b: f32, c: f32, d: f32) -> (u32, u32, u32) {
    let u = (a * b) + (c * d);
    let left = fma_ref(a, b, c * d); // left MUL contracts
    let right = fma_ref(c, d, a * b); // right MUL contracts
    (u.to_bits(), left.to_bits(), right.to_bits())
}

fn sample_f32(state: &mut u64) -> f32 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    // Mix magnitude decades and signs so sticky-bit cancellation can appear.
    let unit = ((*state >> 40) as f32) / (1u32 << 24) as f32; // [0,1)
    let mag = match (*state >> 3) % 5 {
        0 => 1.0e-3,
        1 => 1.0,
        2 => 1.0e2,
        3 => 1.0e-6,
        _ => 1.0e3,
    };
    let signed = if (*state & 1) == 0 { 1.0 } else { -1.0 };
    signed * (0.5 + unit) * mag
}

/// Search operands until U / L-fma / R-fma are pairwise distinct.
fn discriminating_abcd() -> (f32, f32, f32, f32) {
    // Proven discriminator (bits verified): U/L/R pairwise distinct under f64-ref FMA.
    let proven = (
        f32::from_bits(0x3f800001),
        f32::from_bits(0x3f800001),
        1.0,
        f32::from_bits(0xbf7ffffe), // -0.9999999
    );
    {
        let (u, l, r) = oracle_triple(proven.0, proven.1, proven.2, proven.3);
        if u != l && u != r && l != r {
            return proven;
        }
    }
    // Python-found magnitude-skewed discriminator as fallback seed.
    let skewed = (
        f32::from_bits(0xba10_d4a0), // ~-0.00055257
        f32::from_bits(0x42b4_e5d0), // ~90.449
        f32::from_bits(0x4443_8c4c), // ~781.54
        f32::from_bits(0x3890_0000), // ~6.86e-5 approx
    );
    {
        let (u, l, r) = oracle_triple(skewed.0, skewed.1, skewed.2, skewed.3);
        if u != l && u != r && l != r {
            return skewed;
        }
    }
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..2_000_000 {
        let a = sample_f32(&mut state);
        let b = sample_f32(&mut state);
        let c = sample_f32(&mut state);
        let d = sample_f32(&mut state);
        let (u, l, r) = oracle_triple(a, b, c, d);
        let uf = f32::from_bits(u);
        let lf = f32::from_bits(l);
        let rf = f32::from_bits(r);
        if u != l && u != r && l != r && uf.is_finite() && lf.is_finite() && rf.is_finite() {
            return (a, b, c, d);
        }
    }
    panic!("failed to find discriminating (a,b,c,d) for U/L/R");
}

fn discriminating_ema_probe(decay: f32) -> (f32 /*prev*/, f32 /*input*/) {
    let one_minus = 1.0 - decay;
    let mut state = 0xdead_beef_cafe_u64;
    for _ in 0..2_000_000 {
        let prev = sample_f32(&mut state);
        let input = sample_f32(&mut state);
        let (u, l, r) = oracle_triple(prev, decay, input, one_minus);
        let uf = f32::from_bits(u);
        let lf = f32::from_bits(l);
        let rf = f32::from_bits(r);
        if u != l && u != r && l != r && uf.is_finite() && lf.is_finite() && rf.is_finite() {
            return (prev, input);
        }
    }
    panic!("failed to find discriminating EMA probe");
}

fn discriminating_bf_probe(decay: f32, gain: f32) -> (f32 /*prev*/, f32 /*input*/) {
    let mut state = 0x1234_5678_9abc_def0u64;
    for _ in 0..2_000_000 {
        let prev = sample_f32(&mut state);
        let input = sample_f32(&mut state);
        let (u, l, r) = oracle_triple(prev, decay, input, gain);
        let uf = f32::from_bits(u);
        let lf = f32::from_bits(l);
        let rf = f32::from_bits(r);
        if u != l && u != r && l != r && uf.is_finite() && lf.is_finite() && rf.is_finite() {
            // Keep the unclamped result inside BoundedFeedback clamp window.
            if uf.abs() < 90.0 && lf.abs() < 90.0 && rf.abs() < 90.0 {
                return (prev, input);
            }
        }
    }
    panic!("failed to find discriminating BoundedFeedback probe");
}

fn classify_bits(observed: u32, u: u32, left: u32, right: u32) -> &'static str {
    if observed == u {
        "matches-U-separate-rounding"
    } else if observed == left {
        "matches-L-fma-left-MUL"
    } else if observed == right {
        "matches-R-fma-right-MUL"
    } else {
        "matches-other"
    }
}

fn eval_cpu(nodes: &[EmlNode], row: &[f32], columns: u32, params: [f32; 4]) -> f32 {
    simthing_kernel::eval_eml_cpu(&to_gpu(nodes), 0, row, columns, params)
}

fn eval_interpreted_gpu(
    ctx: &GpuContext,
    nodes: &[EmlNode],
    row: &[f32],
    columns: u32,
) -> f32 {
    set_debug_readback_allowed(true);
    let gpu_nodes = to_gpu(nodes);
    let n_cols = columns + 1;
    let out_col = ColumnIndex::try_from_admitted_authored(columns, n_cols).expect("out col");
    let mut values = row.to_vec();
    values.push(0.0);

    let host_nodes = nodes.to_vec();
    let meta = EmlFormulaMeta {
        tree_id: EmlTreeId(1),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: Default::default(),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: nodes.len() as u32,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: "contraction_ambiguity_probe".into(),
    };
    let mut reg = EmlExpressionRegistry::new();
    reg.register_formula(EmlTreeId(1), meta, host_nodes)
        .expect("register");
    let meta = reg.get(EmlTreeId(1)).expect("meta").clone();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 4);
    let mapping = table
        .upload_trees(ctx, &[(EmlTreeId(1), meta, gpu_nodes)])
        .expect("upload");
    for (id, idx) in mapping {
        reg.mark_tree_uploaded(id, idx, table.generation)
            .expect("mark");
    }
    let ops = vec![AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::try_from_admitted_authored(0, n_cols).expect("in"),
        },
        combine: CombineFn::EvalEML { tree_id: 1 },
        gate: GateSpec::Always,
        scale: ScaleSpec::Constant(1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(0), out_col)],
    }];
    let upload = PackedAccumulatorUpload::from_ops_with_eml(&ops, Some(&reg)).expect("pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_cols, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("ops");
    session
        .tick_with_eml(ctx, 0, Some(&table))
        .expect("tick");
    let gpu_values = session.readback_full(ctx).expect("readback");
    gpu_values[columns as usize]
}

fn compile_gadget(instance: EmlGadgetInstanceSpec) -> Vec<EmlNode> {
    compile_eml_gadget(&instance, EmlGadgetCompileOptions::default())
        .expect("admit gadget")
        .nodes
}

fn admitted_programs() -> Vec<(&'static str, &'static str, Vec<EmlNode>, u32, Vec<f32>, [f32; 4])> {
    let mut out = Vec::new();

    // Tier-1 / Tier-2 gadgets (canonical admitted compilers).
    let wa2 = compile_gadget(EmlGadgetInstanceSpec::WeightedAccumulator {
        id: "wa2".into(),
        input_cols: vec![0, 1],
        weight_cols: vec![2, 3],
        output_col: None,
    });
    let (a, b, c, d) = discriminating_abcd();
    out.push((
        "WeightedAccumulator-n2",
        "crates/simthing-spec/src/compile/eml_gadget.rs::compile_weighted_accumulator_nodes",
        wa2,
        4,
        vec![a, c, b, d], // slots: in0, in1, w0, w1 → (a*b)+(c*d)
        [0.0; 4],
    ));

    let wa3 = compile_gadget(EmlGadgetInstanceSpec::WeightedAccumulator {
        id: "wa3".into(),
        input_cols: vec![0, 1, 2],
        weight_cols: vec![3, 4, 5],
        output_col: None,
    });
    // First ADD consumes MUL(in1,w1)+MUL(in2,w2); in0/w0 only feed the later ADD.
    let (c2, d2, e2, f2) = discriminating_abcd();
    out.push((
        "WeightedAccumulator-n3",
        "crates/simthing-spec/src/compile/eml_gadget.rs::compile_weighted_accumulator_nodes",
        wa3,
        6,
        vec![a, c2, e2, b, d2, f2],
        [0.0; 4],
    ));

    // EMA / BoundedFeedback bake one factor as a literal — search (prev, input)
    // pairs that still discriminate under those fixed factors.
    let (ema_prev, ema_input) = discriminating_ema_probe(0.73);
    let ema = compile_gadget(EmlGadgetInstanceSpec::Ema {
        id: "ema".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        decay: 0.73,
    });
    out.push((
        "Ema",
        "crates/simthing-spec/src/compile/eml_gadget.rs::compile_ema_nodes",
        ema,
        2,
        vec![ema_input, ema_prev],
        [0.0; 4],
    ));

    let (bf_prev, bf_input) = discriminating_bf_probe(0.73, 1.41);
    let bf = compile_gadget(EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bf".into(),
        previous_col: 0,
        input_col: 1,
        output_col: None,
        decay: 0.73,
        gain: 1.41,
        min: -100.0,
        max: 100.0,
    });
    out.push((
        "BoundedFeedback",
        "crates/simthing-spec/src/compile/eml_gadget.rs::compile_bounded_feedback_nodes",
        bf,
        2,
        vec![bf_prev, bf_input],
        [0.0; 4],
    ));

    // Non-ambiguous catalogue members still walked for negative proof.
    for (name, instance) in [
        (
            "FieldSampler",
            EmlGadgetInstanceSpec::FieldSampler {
                id: "fs".into(),
                input_col: 0,
                output_col: None,
                cap: 2.0,
            },
        ),
        (
            "SoftStep",
            EmlGadgetInstanceSpec::SoftStep {
                id: "ss".into(),
                input_col: 0,
                output_col: None,
                center: 0.5,
                steepness: 2.0,
            },
        ),
        (
            "VelocityMonitor",
            EmlGadgetInstanceSpec::VelocityMonitor {
                id: "vm".into(),
                current_col: 0,
                previous_col: 1,
                output_col: None,
                dt: Some(0.5),
            },
        ),
        (
            "Decay",
            EmlGadgetInstanceSpec::Decay {
                id: "dec".into(),
                state_col: 0,
                output_col: None,
                decay: 0.9,
            },
        ),
        (
            "Acceleration",
            EmlGadgetInstanceSpec::Acceleration {
                id: "acc".into(),
                current_velocity_col: 0,
                previous_velocity_col: 1,
                output_col: None,
                dt: Some(0.5),
            },
        ),
        (
            "Hysteresis",
            EmlGadgetInstanceSpec::Hysteresis {
                id: "hyst".into(),
                input_col: 0,
                previous_col: 1,
                output_col: None,
                on_threshold: 0.7,
                off_threshold: 0.3,
                off_value: 0.0,
                on_value: 1.0,
            },
        ),
    ] {
        let nodes = compile_gadget(instance);
        out.push((
            name,
            "crates/simthing-spec/src/compile/eml_gadget.rs",
            nodes,
            2,
            vec![a, c],
            [0.0; 4],
        ));
    }

    // Kernel / core closed-vocab builders.
    let soft_policy = SoftStepPolicyConditional {
        input_col: 0,
        center: 0.5,
        steepness: 2.0,
        branch_a_col: 1,
        branch_b_col: 2,
        output_col: 3,
    }
    .compile_nodes()
    .expect("soft policy");
    out.push((
        "SoftStepPolicyConditional",
        "crates/simthing-kernel/src/eml_opcode_gate.rs::SoftStepPolicyConditional",
        soft_policy,
        3,
        vec![a, b, c],
        [0.0; 4],
    ));

    let softmax = SoftmaxWeightGadget {
        z_col: 0,
        max_col: 1,
        beta: 1.25,
    }
    .compile_nodes()
    .expect("softmax");
    out.push((
        "softmax-weight",
        "crates/simthing-kernel/src/eml_opcode_gate.rs::SoftmaxWeightGadget",
        softmax,
        2,
        vec![1.0, 1.5],
        [0.0; 4],
    ));

    let power = LnConsumerGadgets::power_law_nodes(0, 1.7).expect("power");
    out.push((
        "power-law",
        "crates/simthing-kernel/src/eml_opcode_gate.rs::LnConsumerGadgets::power_law_nodes",
        power,
        1,
        vec![2.5],
        [0.0; 4],
    ));

    let entropy = LnConsumerGadgets::entropy_term_nodes(0).expect("entropy");
    out.push((
        "entropy-term",
        "crates/simthing-kernel/src/eml_opcode_gate.rs::LnConsumerGadgets::entropy_term_nodes",
        entropy,
        1,
        vec![0.37],
        [0.0; 4],
    ));

    let logistic = simthing_core::logistic_steering_eml_nodes(0.25, 4.0, 0.9, 3.0);
    out.push((
        "logistic-steering",
        "crates/simthing-core/src/property.rs::logistic_steering_eml_nodes",
        logistic,
        1,
        vec![0.0],
        [0.0, 2.0, 0.0, 0.0], // PARAM N = 2.0
    ));

    let need2 = need_binding_nodes(2);
    out.push((
        "need-binding-weighted-n2",
        "crates/simthing-driver/src/need_binding.rs::build_weighted_need_nodes",
        need2,
        4,
        vec![a, c, b, d],
        [0.0; 4],
    ));

    // Intensity: one-MUL→ADD (licensed) — walked as negative control.
    let intensity = simthing_core::compile_intensity_behavior_to_eml(
        &simthing_core::IntensityBehavior {
            velocity_threshold: 0.1,
            build_coefficient: 0.2,
            decay_coefficient: 0.05,
        },
        EmlTreeId(0x8000),
        0,
        1,
    )
    .1;
    out.push((
        "intensity-behavior",
        "crates/simthing-core/src/intensity_eml.rs::compile_intensity_behavior_to_eml",
        intensity,
        2,
        vec![1.5, 0.25],
        [0.016, 0.0, 0.0, 0.0],
    ));

    out
}

fn need_binding_nodes(n: usize) -> Vec<EmlNode> {
    // Mirror of driver build_weighted_need_nodes (same shape; driver helper is private).
    let mut nodes = Vec::new();
    for i in 0..n {
        let in_col = i as u32;
        let w_col = (n + i) as u32;
        nodes.push(EmlNode {
            opcode: eml_nodes::opcode::SLOT_VALUE,
            flags: 0,
            a: in_col,
            b: 0,
            c: 0,
            d: 0,
        });
        nodes.push(EmlNode {
            opcode: eml_nodes::opcode::SLOT_VALUE,
            flags: 0,
            a: w_col,
            b: 0,
            c: 0,
            d: 0,
        });
        nodes.push(EmlNode {
            opcode: eml_nodes::opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        });
    }
    for _ in 1..n {
        nodes.push(EmlNode {
            opcode: eml_nodes::opcode::ADD,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        });
    }
    nodes.push(EmlNode {
        opcode: eml_nodes::opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    nodes
}

/// Recover the two MUL operand pairs feeding an ambiguous ADD via stack replay.
fn mul_operands(
    nodes: &[EmlNode],
    mul_idx: usize,
    row: &[f32],
    columns: u32,
    params: [f32; 4],
) -> (f32, f32) {
    let mut stack: Vec<f32> = Vec::new();
    for (idx, node) in nodes.iter().enumerate() {
        match node.opcode {
            eml_nodes::opcode::LITERAL_F32 => stack.push(f32::from_bits(node.a)),
            eml_nodes::opcode::SLOT_VALUE => {
                stack.push(row.get(node.a as usize).copied().unwrap_or(0.0))
            }
            eml_nodes::opcode::PARAM => stack.push(params[node.a as usize]),
            eml_nodes::opcode::ADD
            | eml_nodes::opcode::SUB
            | eml_nodes::opcode::MUL
            | eml_nodes::opcode::DIV
            | eml_nodes::opcode::MIN
            | eml_nodes::opcode::MAX
            | eml_nodes::opcode::CMP_LT
            | eml_nodes::opcode::CMP_LE
            | eml_nodes::opcode::CMP_GT
            | eml_nodes::opcode::CMP_GE
            | eml_nodes::opcode::CMP_EQ => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                if idx == mul_idx && node.opcode == eml_nodes::opcode::MUL {
                    let _ = columns;
                    return (lhs, rhs);
                }
                let v = match node.opcode {
                    eml_nodes::opcode::ADD => lhs + rhs,
                    eml_nodes::opcode::SUB => lhs - rhs,
                    eml_nodes::opcode::MUL => lhs * rhs,
                    eml_nodes::opcode::DIV => lhs / rhs,
                    eml_nodes::opcode::MIN => lhs.min(rhs),
                    eml_nodes::opcode::MAX => lhs.max(rhs),
                    eml_nodes::opcode::CMP_LT => {
                        if lhs < rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    eml_nodes::opcode::CMP_LE => {
                        if lhs <= rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    eml_nodes::opcode::CMP_GT => {
                        if lhs > rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    eml_nodes::opcode::CMP_GE => {
                        if lhs >= rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    eml_nodes::opcode::CMP_EQ => {
                        if lhs == rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => unreachable!(),
                };
                stack.push(v);
            }
            eml_nodes::opcode::SELECT => {
                let c = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(if a != 0.0 { b } else { c });
            }
            eml_nodes::opcode::NEG => {
                let v = stack.pop().unwrap();
                stack.push(-v);
            }
            eml_nodes::opcode::ABS => {
                let v = stack.pop().unwrap();
                stack.push(v.abs());
            }
            eml_nodes::opcode::FLOOR => {
                let v = stack.pop().unwrap();
                stack.push(v.floor());
            }
            eml_nodes::opcode::CLAMP_BOUNDED => {
                let v = stack.pop().unwrap();
                stack.push(v.clamp(f32::from_bits(node.a), f32::from_bits(node.b)));
            }
            eml_nodes::opcode::CLAMP_FLOORED => {
                let v = stack.pop().unwrap();
                stack.push(v.max(f32::from_bits(node.a)));
            }
            eml_nodes::opcode::EXP => {
                let v = stack.pop().unwrap();
                stack.push(simthing_core::eml_exp_pinned_f32(v));
            }
            eml_nodes::opcode::LN => {
                let v = stack.pop().unwrap();
                stack.push(simthing_core::eml_ln::eml_ln_pinned_f32(v));
            }
            eml_nodes::opcode::RETURN_TOP => {}
            _ => panic!("unknown opcode in mul_operands"),
        }
    }
    panic!("mul index {mul_idx} not reached");
}

#[test]
fn eml_arithmetic_semantics_0_contraction_ambiguity_census_stop_packet() {
    // OrdinaryAccumulatorEvalEml: CpuTwin + InterpretedGpu only (field JIT
    // never compiles AO programs). Recorded for the census packet.
    let mut hits: Vec<AmbiguousHit> = Vec::new();
    let mut mul_into_sub_edges: Vec<(&'static str, usize, usize)> = Vec::new();
    let mut walked = 0usize;
    for (program, source, nodes, columns, probe_row, params) in admitted_programs() {
        walked += 1;
        let (add_hits, sub_edges) = classify_contraction_walk(&nodes);
        for (add_index, mul_lhs_index, mul_rhs_index) in add_hits {
            hits.push(AmbiguousHit {
                program,
                source,
                add_index,
                mul_lhs_index,
                mul_rhs_index,
                nodes: nodes.clone(),
                columns,
                probe_row: probe_row.clone(),
                params,
            });
        }
        for (sub_index, mul_index) in sub_edges {
            mul_into_sub_edges.push((program, sub_index, mul_index));
        }
    }

    println!("=== EML-ARITHMETIC-SEMANTICS-0 UNIQUENESS CENSUS (ADD + SUB reachability) ===");
    println!("programs_walked: {walked}");
    println!("two_mul_into_add_hits: {}", hits.len());
    println!("mul_into_sub_dataflows: {}", mul_into_sub_edges.len());
    // DA 5192270934: two+ MUL → ADD is authored UNFUSED (`U`). Proceed; no tie-break.
    // DA 5193244394: MUL→SUB count decides whether SUB joins uniqueness.

    let ctx = GpuContext::new_blocking().expect("GPU required for interpreted-arm measurement");
    let live =
        simthing_kernel::eml_exp_qualification::EmlExpLiveToolchainIdentity::from_context(&ctx);
    simthing_kernel::eml_exp_qualification::require_certified_toolchain(&live)
        .expect("certified toolchain");

    println!("ORIENT-RECEIPT: e2fd94a4fb2a");
    println!("HD-RECEIPT: b9070974440b");
    println!("base_sha: 98180a4a4e7334fa9476c74170d995b5028202dc");
    println!("ssa_jit_note: OrdinaryAccumulatorEvalEml derives CpuTwin+InterpretedGpu only; SSA-JIT is not-an-execution-arm for these hits");
    println!(
        "uniqueness_rule: one MUL→ADD = FUSED; two+ MUL→ADD = UNFUSED(U); MUL→SUB dataflows={}; no tie-break",
        mul_into_sub_edges.len()
    );
    for (i, (program, sub_index, mul_index)) in mul_into_sub_edges.iter().enumerate() {
        println!("--- mul_into_sub[{i}] ---");
        println!("program: {program}");
        println!("nodes: SUB@{sub_index} fed by MUL@{mul_index}");
    }

    for (i, hit) in hits.iter().enumerate() {
        let (a, b) = mul_operands(
            &hit.nodes,
            hit.mul_lhs_index,
            &hit.probe_row,
            hit.columns,
            hit.params,
        );
        let (c, d) = mul_operands(
            &hit.nodes,
            hit.mul_rhs_index,
            &hit.probe_row,
            hit.columns,
            hit.params,
        );
        let (u_bits, l_bits, r_bits) = oracle_triple(a, b, c, d);
        assert_ne!(u_bits, l_bits, "probe must discriminate U vs L");
        assert_ne!(u_bits, r_bits, "probe must discriminate U vs R");
        assert_ne!(l_bits, r_bits, "probe must discriminate L vs R");

        // Measure the ambiguous composition itself (two MULs → one ADD), not
        // later program nodes that would mask the local rounding choice.
        let local = vec![
            EmlNode {
                opcode: eml_nodes::opcode::LITERAL_F32,
                flags: 0,
                a: a.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::LITERAL_F32,
                flags: 0,
                a: b.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::MUL,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::LITERAL_F32,
                flags: 0,
                a: c.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::LITERAL_F32,
                flags: 0,
                a: d.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::MUL,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::ADD,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNode {
                opcode: eml_nodes::opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ];
        let cpu = eval_cpu(&local, &[], 0, [0.0; 4]);
        let gpu = eval_interpreted_gpu(&ctx, &local, &[], 0);
        let cpu_class = classify_bits(cpu.to_bits(), u_bits, l_bits, r_bits);
        let gpu_class = classify_bits(gpu.to_bits(), u_bits, l_bits, r_bits);

        println!("--- hit[{i}] ---");
        println!("program: {}", hit.program);
        println!("source: {}", hit.source);
        println!(
            "nodes: ADD@{} fed by MUL@{} and MUL@{}",
            hit.add_index, hit.mul_lhs_index, hit.mul_rhs_index
        );
        println!(
            "probe_operands: a={a}({:#010x}) b={b}({:#010x}) c={c}({:#010x}) d={d}({:#010x})",
            a.to_bits(),
            b.to_bits(),
            c.to_bits(),
            d.to_bits()
        );
        println!(
            "oracles: U={:#010x} L-fma(left MUL)={:#010x} R-fma(right MUL)={:#010x}",
            u_bits, l_bits, r_bits
        );
        println!(
            "cpu_twin: bits={:#010x} class={cpu_class} (contracts: {})",
            cpu.to_bits(),
            if cpu_class == "matches-L-fma-left-MUL" {
                format!("MUL@{}", hit.mul_lhs_index)
            } else if cpu_class == "matches-R-fma-right-MUL" {
                format!("MUL@{}", hit.mul_rhs_index)
            } else if cpu_class == "matches-U-separate-rounding" {
                "none".into()
            } else {
                "unknown".into()
            }
        );
        println!(
            "interpreted_wgsl: bits={:#010x} class={gpu_class} (contracts: {})",
            gpu.to_bits(),
            if gpu_class == "matches-L-fma-left-MUL" {
                format!("MUL@{}", hit.mul_lhs_index)
            } else if gpu_class == "matches-R-fma-right-MUL" {
                format!("MUL@{}", hit.mul_rhs_index)
            } else if gpu_class == "matches-U-separate-rounding" {
                "none".into()
            } else {
                "unknown".into()
            }
        );
        println!("ssa_jit: not-an-execution-arm (OrdinaryAccumulatorEvalEml)");
    }

    println!(
        "=== END UNIQUENESS CENSUS — ADD hits={} UNFUSED; MUL→SUB dataflows={} ===",
        hits.len(),
        mul_into_sub_edges.len()
    );
}
