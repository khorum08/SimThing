//! EML-ARITHMETIC-SEMANTICS-0 — uniqueness lowerings, plants, zero-evidence EXP consumer.
//!
//! Binding: DA uniqueness ruling `5192270934` / RESUME `5192307920`.

use simthing_core::{
    eml_nodes, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SlotIndex,
    SourceSpec,
};
use simthing_gpu::{
    set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable, FieldSweepSession,
    GpuContext, PackedAccumulatorUpload,
};
use simthing_kernel::{OpcodeRegistrationGate, EXP_PRIMITIVE_NAME};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv_fold(mut hash: u64, bits: u32) -> u64 {
    for byte in bits.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn certified_context() -> Option<GpuContext> {
    let ctx = GpuContext::new_blocking().ok()?;
    let live =
        simthing_kernel::eml_exp_qualification::EmlExpLiveToolchainIdentity::from_context(&ctx);
    simthing_kernel::eml_exp_qualification::require_certified_toolchain(&live)
        .expect("certified toolchain");
    Some(ctx)
}

fn stead_falloff_registration() -> simthing_kernel::FieldSweepRegistration {
    use simthing_driver::field_sweep_compile::{
        compile_stead_exponential_falloff_field_sweep, SteadExponentialFalloffSpec,
    };
    compile_stead_exponential_falloff_field_sweep(SteadExponentialFalloffSpec {
        width: 8,
        height: 8,
        n_dims: 2,
        value_col: ColumnIndex::try_from_admitted_authored(0, 2).expect("v"),
        output_col: ColumnIndex::try_from_admitted_authored(1, 2).expect("o"),
        lambda: 0.73,
        dt: 1.0,
    })
    .expect("falloff")
}

fn probe_falloff_values() -> Vec<f32> {
    let mut state = 0x243F_6A88_85A3_08D3u64;
    let raw: Vec<f32> = (0..64)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            0.1 + ((state >> 40) as f32) / (1u32 << 24) as f32 * 7.3
        })
        .collect();
    raw.iter().flat_map(|v| [*v, 0.0]).collect()
}

fn digest_outputs(outputs: &[f32]) -> u64 {
    outputs
        .iter()
        .fold(FNV_OFFSET, |d, v| fnv_fold(d, v.to_bits()))
}

/// NEW EXP consumer: soft-tail `EXP(CLAMP(NEG(ABS(x)), domain_min, 0))`.
/// Determinism from admitted arithmetic + pinned EXP — zero ExactBearing /
/// declaration / census.
fn soft_tail_exp_nodes(x_col: u32) -> Vec<eml_nodes::EmlNode> {
    use eml_nodes::{opcode, EmlNode};
    let op = |opcode: u32| EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let nodes = vec![
        EmlNode {
            opcode: opcode::SLOT_VALUE,
            flags: 0,
            a: x_col,
            b: 0,
            c: 0,
            d: 0,
        },
        op(opcode::ABS),
        op(opcode::NEG),
        EmlNode {
            opcode: opcode::CLAMP_BOUNDED,
            flags: 0,
            a: simthing_core::EML_EXP_DOMAIN_MIN_BITS,
            b: simthing_core::EML_EXP_SATURATION_CEILING_BITS,
            c: 0,
            d: 0,
        },
        op(opcode::EXP),
        op(opcode::RETURN_TOP),
    ];
    OpcodeRegistrationGate::admit_tree_nodes(&nodes).expect("closed vocab");
    simthing_kernel::admit_exp_call_sites(&nodes).expect("EXP call sites");
    nodes
}

#[test]
fn eml_arithmetic_semantics_0_new_exp_consumer_needs_zero_exactness_evidence() {
    let nodes = soft_tail_exp_nodes(0);
    let meta = EmlFormulaMeta {
        tree_id: EmlTreeId(42),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: Default::default(),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: nodes.len() as u32,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: "soft-tail-exp".into(),
    };
    let mut reg = EmlExpressionRegistry::new();
    reg.register_formula(EmlTreeId(42), meta, nodes.clone())
        .expect("admits from arithmetic semantics alone");

    let gpu: Vec<EmlNodeGpu> = nodes
        .iter()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
    let v = simthing_kernel::eval_eml_cpu(&gpu, 0, &[1.25], 1, [0.0; 4]);
    let expected = simthing_core::eml_exp_pinned_f32((-1.25_f32.abs()).clamp(
        f32::from_bits(simthing_core::EML_EXP_DOMAIN_MIN_BITS),
        f32::from_bits(simthing_core::EML_EXP_SATURATION_CEILING_BITS),
    ));
    assert_eq!(v.to_bits(), expected.to_bits());
    // Absence of the deleted 5.13 policing symbols is proven by
    // `eml_arithmetic_semantics_0_deleted_plumbing_grep_absent` — this test
    // only proves the new consumer admits with no exactness declaration.
    let _ = EXP_PRIMITIVE_NAME;
}

#[test]
fn eml_arithmetic_semantics_0_cpu_seam_plant_reds_falloff() {
    let registration = stead_falloff_registration();
    let values = probe_falloff_values();
    let clean =
        simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3).expect("clean");
    simthing_kernel::field_sweep::plant_seam_cpu_separate_rounding(true);
    let planted =
        simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3).expect("plant");
    simthing_kernel::field_sweep::plant_seam_cpu_separate_rounding(false);
    assert_ne!(
        digest_outputs(&clean),
        digest_outputs(&planted),
        "production CPU seam plant (fused→separate) must RED falloff"
    );
}

#[test]
fn eml_arithmetic_semantics_0_interpreted_seam_plant_reds_falloff() {
    let Some(ctx) = certified_context() else {
        return;
    };
    set_debug_readback_allowed(true);
    let registration = stead_falloff_registration();
    let values = probe_falloff_values();
    let class = registration.resource_class();

    let mut clean_session =
        FieldSweepSession::new_interpreted_for_profiling(&ctx, &registration, class)
            .expect("clean interpreted");
    clean_session.upload_values(&ctx, &values).expect("upload");
    clean_session
        .dispatch_chain(&ctx, std::slice::from_ref(&registration), 3)
        .expect("dispatch");
    let clean = clean_session.readback(&ctx).expect("readback");

    simthing_kernel::field_sweep::plant_seam_interpreted_disable_fuse(true);
    let mut planted_session =
        FieldSweepSession::new_interpreted_for_profiling(&ctx, &registration, class)
            .expect("planted interpreted");
    planted_session
        .upload_values(&ctx, &values)
        .expect("upload");
    planted_session
        .dispatch_chain(&ctx, std::slice::from_ref(&registration), 3)
        .expect("dispatch");
    let planted = planted_session.readback(&ctx).expect("readback");
    simthing_kernel::field_sweep::plant_seam_interpreted_disable_fuse(false);

    assert_ne!(
        digest_outputs(&clean),
        digest_outputs(&planted),
        "production interpreted seam plant (pad1 fuse off) must RED falloff"
    );
}

/// BOUNDED STOP witness (remand `5192641222` blocker 3): on the certified
/// toolchain, a truthful JIT plant that replaces the fused `fma` fold body
/// with the ordinary Sum fold (MUL in `eval_map`, ADD in `eval_fold`) is
/// re-contracted to the same bits as the fused form. A post-`fma` bit-flip
/// plant was rejected as not falsifying fusion. No lawful separate-rounding
/// JIT mutant remains without an optimizer fence.
#[test]
fn eml_arithmetic_semantics_0_jit_seam_separate_rounding_plant_is_recontracted() {
    let Some(ctx) = certified_context() else {
        return;
    };
    set_debug_readback_allowed(true);
    let registration = stead_falloff_registration();
    let values = probe_falloff_values();
    let class = registration.resource_class();

    let mut clean_session =
        FieldSweepSession::new_with_profiling_resource_class(&ctx, &registration, class)
            .expect("clean jit");
    clean_session.upload_values(&ctx, &values).expect("upload");
    clean_session
        .dispatch_chain(&ctx, std::slice::from_ref(&registration), 3)
        .expect("dispatch");
    let clean = clean_session.readback(&ctx).expect("readback");

    simthing_kernel::field_sweep::plant_seam_jit_separate_rounding(true);
    let mut planted_session =
        FieldSweepSession::new_with_profiling_resource_class(&ctx, &registration, class)
            .expect("planted jit");
    planted_session
        .upload_values(&ctx, &values)
        .expect("upload");
    planted_session
        .dispatch_chain(&ctx, std::slice::from_ref(&registration), 3)
        .expect("dispatch");
    let planted = planted_session.readback(&ctx).expect("readback");
    simthing_kernel::field_sweep::plant_seam_jit_separate_rounding(false);

    assert_eq!(
        digest_outputs(&clean),
        digest_outputs(&planted),
        "substrate fact: truthful JIT unfused-Sum plant must re-contract to fused bits on certified toolchain (BOUNDED STOP for JIT fusion falsifier)"
    );
    // Contrast: CPU and interpreted plants DO RED on the same falloff probe
    // (see sibling plant tests) — only the SSA-JIT generated fold is opaque.
}

fn lit(bits: u32) -> eml_nodes::EmlNode {
    eml_nodes::EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: bits,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn op(opcode: u32) -> eml_nodes::EmlNode {
    op_flags(opcode, 0)
}

fn op_flags(opcode: u32, flags: u32) -> eml_nodes::EmlNode {
    eml_nodes::EmlNode {
        opcode,
        flags,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn binary_program(opcode: u32, a: f32, b: f32) -> Vec<eml_nodes::EmlNode> {
    // DIV requires flags bit 0 (safe division) at admission.
    let bin_flags = u32::from(opcode == eml_nodes::opcode::DIV);
    let nodes = vec![
        lit(a.to_bits()),
        lit(b.to_bits()),
        op_flags(opcode, bin_flags),
        op(eml_nodes::opcode::RETURN_TOP),
    ];
    OpcodeRegistrationGate::admit_tree_nodes(&nodes).expect("closed vocab");
    nodes
}

fn ieee_bits(opcode: u32, a: f32, b: f32) -> u32 {
    match opcode {
        eml_nodes::opcode::ADD => (a + b).to_bits(),
        eml_nodes::opcode::SUB => (a - b).to_bits(),
        eml_nodes::opcode::MUL => (a * b).to_bits(),
        eml_nodes::opcode::DIV => (a / b).to_bits(),
        _ => panic!("not a rounding binary opcode"),
    }
}

fn ao_eval_cpu(nodes: &[eml_nodes::EmlNode]) -> f32 {
    let gpu: Vec<EmlNodeGpu> = nodes
        .iter()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
    simthing_kernel::eval_eml_cpu(&gpu, 0, &[], 0, [0.0; 4])
}

fn ao_eval_interpreted(ctx: &GpuContext, nodes: &[eml_nodes::EmlNode]) -> f32 {
    set_debug_readback_allowed(true);
    let gpu: Vec<EmlNodeGpu> = nodes
        .iter()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
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
        display_name: "standalone-opcode".into(),
    };
    let mut reg = EmlExpressionRegistry::new();
    reg.register_formula(EmlTreeId(1), meta, nodes.to_vec())
        .expect("register");
    let meta = reg.get(EmlTreeId(1)).expect("meta").clone();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 4);
    let mapping = table
        .upload_trees(ctx, &[(EmlTreeId(1), meta, gpu)])
        .expect("upload");
    for (id, idx) in mapping {
        reg.mark_tree_uploaded(id, idx, table.generation)
            .expect("mark");
    }
    let out_col = ColumnIndex::try_from_admitted_authored(0, 1).expect("col");
    let ops = vec![AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: out_col,
        },
        combine: CombineFn::EvalEML { tree_id: 1 },
        gate: GateSpec::Always,
        scale: ScaleSpec::Constant(1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(0), out_col)],
    }];
    let upload = PackedAccumulatorUpload::from_ops_with_eml(&ops, Some(&reg)).expect("pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, 1, 1);
    session.upload_values(ctx, &[0.0]);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("ops");
    session.tick_with_eml(ctx, 0, Some(&table)).expect("tick");
    session.readback_full(ctx).expect("rb")[0]
}

/// Unique MUL→SUB is FUSED (one-rounding fms) on AO-derived arms.
/// SSA-JIT is not-an-execution-arm for OrdinaryAccumulatorEvalEml.
#[test]
fn eml_arithmetic_semantics_0_unique_mul_into_sub_matches_fms_on_derived_arms() {
    let Some(ctx) = certified_context() else {
        return;
    };
    // Shape c - (a*b): unique rhs-MUL → FUSED as (-a).mul_add(b, c).
    // Search until host mul_add differs from separate rounding.
    let (a, b, c, fused) = {
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut found = None;
        for _ in 0..2_000_000 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let unit = ((state >> 40) as f32) / (1u32 << 24) as f32;
            let mag = match (state >> 3) % 5 {
                0 => 1.0e-3,
                1 => 1.0,
                2 => 1.0e2,
                3 => 1.0e-6,
                _ => 1.0e3,
            };
            let signed = if (state & 1) == 0 { 1.0 } else { -1.0 };
            let a = signed * (0.5 + unit) * mag;
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let b = {
                let unit = ((state >> 40) as f32) / (1u32 << 24) as f32;
                let mag = match (state >> 3) % 5 {
                    0 => 1.0e-3,
                    1 => 1.0,
                    2 => 1.0e2,
                    3 => 1.0e-6,
                    _ => 1.0e3,
                };
                let signed = if (state & 1) == 0 { 1.0 } else { -1.0 };
                signed * (0.5 + unit) * mag
            };
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let c = {
                let unit = ((state >> 40) as f32) / (1u32 << 24) as f32;
                let mag = match (state >> 3) % 5 {
                    0 => 1.0e-3,
                    1 => 1.0,
                    2 => 1.0e2,
                    3 => 1.0e-6,
                    _ => 1.0e3,
                };
                let signed = if (state & 1) == 0 { 1.0 } else { -1.0 };
                signed * (0.5 + unit) * mag
            };
            let separate = c - (a * b);
            let fused = (-a).mul_add(b, c);
            if separate.to_bits() != fused.to_bits() && fused.is_finite() && separate.is_finite() {
                found = Some((a, b, c, fused));
                break;
            }
        }
        found.expect("host mul_add must discriminate fms for some finite triple")
    };

    // Postfix: c, a, b, MUL, SUB → lhs=c, rhs=MUL → unique MUL→SUB = FUSED.
    let nodes = vec![
        EmlNodeGpu {
            opcode: eml_nodes::opcode::LITERAL_F32,
            flags: 0,
            a: c.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_nodes::opcode::LITERAL_F32,
            flags: 0,
            a: a.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_nodes::opcode::LITERAL_F32,
            flags: 0,
            a: b.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_nodes::opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_nodes::opcode::SUB,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_nodes::opcode::RETURN_TOP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let expected = fused.to_bits();
    let cpu = ao_eval_cpu(&nodes).to_bits();
    let gpu = ao_eval_interpreted(&ctx, &nodes).to_bits();
    assert_eq!(cpu, expected, "unique MUL→SUB CPU twin must match fms");
    assert_eq!(gpu, expected, "unique MUL→SUB interpreted must match fms");
    assert_eq!(cpu, gpu, "unique MUL→SUB arms must agree");
}

/// Standalone ADD/SUB/MUL/DIV: IEEE single-rounding, no reassociation.
/// OrdinaryAccumulatorEvalEml derives CpuTwin + InterpretedGpu only (SSA-JIT
/// is not an execution arm for these AO programs). Field fused ADD/MUL is
/// covered by the uniqueness-seam witnesses above.
#[test]
fn eml_arithmetic_semantics_0_standalone_opcodes_match_ieee_on_derived_arms() {
    let Some(ctx) = certified_context() else {
        return;
    };
    // Discriminators: finite values where each op is well-defined and not a
    // trivial identity that would hide a wrong lowering.
    let cases: &[(u32, f32, f32, &str)] = &[
        (eml_nodes::opcode::ADD, 1.0e20, 1.0, "ADD"),
        (eml_nodes::opcode::SUB, 1.0e20, 1.0, "SUB"),
        (eml_nodes::opcode::MUL, 1.0000001, 1.0000001, "MUL"),
        (eml_nodes::opcode::DIV, 1.0, 3.0, "DIV"),
    ];
    for &(opcode, a, b, name) in cases {
        let nodes = binary_program(opcode, a, b);
        let expected = ieee_bits(opcode, a, b);
        let cpu = ao_eval_cpu(&nodes).to_bits();
        let gpu = ao_eval_interpreted(&ctx, &nodes).to_bits();
        assert_eq!(cpu, expected, "{name}: CPU twin must match IEEE f32 bits");
        assert_eq!(
            gpu, expected,
            "{name}: interpreted WGSL must match IEEE f32 bits"
        );
        assert_eq!(
            cpu, gpu,
            "{name}: CPU twin and interpreted WGSL must agree bit-exactly"
        );
    }
}

#[test]
fn eml_arithmetic_semantics_0_deleted_plumbing_grep_absent() {
    let gate = include_str!("../../simthing-kernel/src/eml_opcode_gate.rs");
    for needle in [
        "ExactBearingEvidence",
        "derive_consumer_arms",
        "ExactConsumerArm",
        "ExactConsumerDigestEvidence",
        "ExactConsumerExecutionShape",
        "ExactConsumerShapeBinding",
        "FieldConsumerShapeProof",
    ] {
        assert!(
            !gate.contains(needle),
            "eml_opcode_gate.rs must not contain deleted symbol {needle}"
        );
    }
    let sweep = include_str!("../../simthing-kernel/src/field_sweep.rs");
    assert!(
        !sweep.contains("exact_consumer_shape_proof"),
        "field_sweep.rs must not mint exact_consumer_shape_proof"
    );
}
