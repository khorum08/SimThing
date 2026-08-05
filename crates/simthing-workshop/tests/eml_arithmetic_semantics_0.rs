//! EML-ARITHMETIC-SEMANTICS-0 — uniqueness lowerings, plants, zero-evidence EXP consumer.
//!
//! Binding: DA uniqueness ruling `5192270934` / RESUME `5192307920`.

use simthing_core::{
    eml_nodes, ColumnIndex, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId,
};
use simthing_gpu::{set_debug_readback_allowed, FieldSweepSession, GpuContext};
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
    let expected = simthing_core::eml_exp_pinned_f32(
        (-1.25_f32.abs()).clamp(
            f32::from_bits(simthing_core::EML_EXP_DOMAIN_MIN_BITS),
            f32::from_bits(simthing_core::EML_EXP_SATURATION_CEILING_BITS),
        ),
    );
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
    let clean = simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3)
        .expect("clean");
    simthing_kernel::field_sweep::plant_seam_cpu_separate_rounding(true);
    let planted = simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3)
        .expect("plant");
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
    clean_session
        .upload_values(&ctx, &values)
        .expect("upload");
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

#[test]
fn eml_arithmetic_semantics_0_jit_seam_plant_reds_falloff() {
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
    clean_session
        .upload_values(&ctx, &values)
        .expect("upload");
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

    assert_ne!(
        digest_outputs(&clean),
        digest_outputs(&planted),
        "production JIT seam plant (fma→separate) must RED falloff"
    );
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
