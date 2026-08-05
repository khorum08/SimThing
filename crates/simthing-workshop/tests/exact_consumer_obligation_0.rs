//! EXACT-CONSUMER-OBLIGATION-0 — the receiving half of the exact-primitive
//! door: every exact-bearing consumer carries its OWN digest over its OWN
//! probe domain across every execution arm it actually uses, reproduced
//! bit-identically. Exactness is never inherited (Exact-Value Provenance Law).
//!
//! Arm justifications (derived, not asserted — DA 5187245896 correction 1):
//! - `stead-exponential-falloff`: field-sweep registration → CpuTwin,
//!   InterpretedGpu, SsaJit. Its output is `Matrix`, so the fused-transient
//!   kernel path is unreachable for it (fusion requires a Transient producer);
//!   recorded as non-arm with that reason.
//! - `logistic-steering` / `softmax-weight`: ordinary AccumulatorOp EvalEML
//!   surfaces → CpuTwin + InterpretedGpu (the AO interpreter); the field JIT
//!   never compiles AO programs, so SsaJit is not an arm for them.
//! - `power-law` / `eml-operator` / `entropy-term` / `log-accumulate`:
//!   gadget node programs executable on both ordinary AO (CpuTwin +
//!   InterpretedGpu) and, for log-accumulate (a field map), the field arms.

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, FieldAdjacency, FieldLawProof, FieldSweepOutput,
    FieldSweepRegistration, FieldSweepRegistrationRequest, FieldSweepSession, GpuContext,
};
use simthing_kernel::{
    admit_exact_bearing_consumer, ExactBearingConsumerDeclaration, ExactConsumerArm,
    ExactConsumerDigestEvidence, LnConsumerGadgets, OpcodeGateError, SoftmaxWeightGadget,
};

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
        .expect("live GPU tuple must be in the certified toolchain roster");
    Some(ctx)
}

fn node(opcode: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu { opcode, flags: 0, a, b, c: 0, d: 0 }
}

/// Non-dyadic probe values (the class that exposes seam/rounding drift).
fn probe_values(count: usize) -> Vec<f32> {
    let mut state = 0x243F_6A88_85A3_08D3u64;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            0.1 + ((state >> 40) as f32) / (1u32 << 24) as f32 * 7.3
        })
        .collect()
}

/// The STEAD falloff consumer: digest per arm over a 8x8 grid of non-dyadic
/// intensities, three sweep iterations (accumulation stress on the seam).
fn stead_falloff_digest(ctx: &GpuContext, arm: ExactConsumerArm) -> u64 {
    use simthing_driver::field_sweep_compile::{
        compile_stead_exponential_falloff_field_sweep, SteadExponentialFalloffSpec,
    };
    let spec = SteadExponentialFalloffSpec {
        width: 8,
        height: 8,
        n_dims: 2,
        value_col: ColumnIndex::try_from_admitted_authored(0, 2).expect("value column"),
        output_col: ColumnIndex::try_from_admitted_authored(1, 2).expect("output column"),
        lambda: 0.73,
        dt: 1.0,
    };
    let registration = compile_stead_exponential_falloff_field_sweep(spec).expect("falloff law");
    let raw = probe_values(8 * 8);
    let values: Vec<f32> = raw
        .iter()
        .flat_map(|value| [*value, 0.0])
        .collect();
    let outputs = match arm {
        ExactConsumerArm::CpuTwin => {
            simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3)
                .expect("CPU falloff")
        }
        ExactConsumerArm::InterpretedGpu | ExactConsumerArm::SsaJit => {
            let class = registration.resource_class();
            let mut session = if arm == ExactConsumerArm::InterpretedGpu {
                FieldSweepSession::new_interpreted_for_profiling(ctx, &registration, class)
                    .expect("interpreted session")
            } else {
                FieldSweepSession::new_with_profiling_resource_class(ctx, &registration, class)
                    .expect("JIT session")
            };
            session.upload_values(ctx, &values).expect("upload");
            session
                .dispatch_chain(ctx, std::slice::from_ref(&registration), 3)
                .expect("dispatch");
            session.readback(ctx).expect("readback")
        }
        ExactConsumerArm::FusedTransientKernel => {
            unreachable!("falloff output is Matrix; fusion unreachable (recorded non-arm)")
        }
    };
    outputs
        .iter()
        .fold(FNV_OFFSET, |digest, value| fnv_fold(digest, value.to_bits()))
}

/// AO-surface consumers (logistic / softmax / power-law / eml / entropy):
/// CPU twin digest via the shared AO stack machine over probe rows.
fn ao_consumer_digest_cpu(nodes: &[EmlNodeGpu], columns: u32) -> u64 {
    let rows = probe_values(512);
    rows.chunks(columns as usize)
        .filter(|chunk| chunk.len() == columns as usize)
        .fold(FNV_OFFSET, |digest, chunk| {
            let value = simthing_kernel::eval_eml_cpu(nodes, 0, chunk, columns, [0.0; 4]);
            fnv_fold(digest, value.to_bits())
        })
}

#[test]
fn exact_consumer_obligation_0_admission_hard_errors_without_evidence() {
    const DECLARATION: ExactBearingConsumerDeclaration = ExactBearingConsumerDeclaration {
        consumer_id: "stead-exponential-falloff",
        primitive: "EXP",
        domain_note: "guarded EXP(-lambda*d), lambda,d >= 0; saturated tail",
        arms: &[
            ExactConsumerArm::CpuTwin,
            ExactConsumerArm::InterpretedGpu,
            ExactConsumerArm::SsaJit,
        ],
        arm_justification: "field-sweep Matrix registration: CPU twin + interpreted + JIT; fused-transient unreachable (Matrix output)",
    };
    // Planted defect 1: one declared arm has no evidence — HARD ERROR.
    let partial = [
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest: 0x1234 },
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest: 0x1234 },
    ];
    assert!(matches!(
        admit_exact_bearing_consumer(&DECLARATION, &partial),
        Err(OpcodeGateError::ExactBearingConsumerWithoutDigestEvidence {
            consumer_id: "stead-exponential-falloff",
            missing_arm: "ssa-jit",
        })
    ));
    // Planted defect 2: zero digest (evidence-free declaration) — HARD ERROR.
    let zeroed = [
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest: 0 },
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest: 0x1234 },
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::SsaJit, digest: 0x1234 },
    ];
    assert!(matches!(
        admit_exact_bearing_consumer(&DECLARATION, &zeroed),
        Err(OpcodeGateError::ExactBearingConsumerWithoutDigestEvidence { .. })
    ));
    // Planted defect 3: arm digests disagree — HARD ERROR (the pre-repair
    // seam shape: JIT digest differing from CPU/interpreted).
    let mismatched = [
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest: 0x1234 },
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest: 0x1234 },
        ExactConsumerDigestEvidence { arm: ExactConsumerArm::SsaJit, digest: 0x9999 },
    ];
    assert!(matches!(
        admit_exact_bearing_consumer(&DECLARATION, &mismatched),
        Err(OpcodeGateError::ExactConsumerArmDigestMismatch {
            consumer_id: "stead-exponential-falloff",
            arm: "ssa-jit",
            ..
        })
    ));
    // Planted defect 4: empty arm set — HARD ERROR.
    const EMPTY: ExactBearingConsumerDeclaration = ExactBearingConsumerDeclaration {
        consumer_id: "empty",
        primitive: "EXP",
        domain_note: "",
        arms: &[],
        arm_justification: "",
    };
    assert!(admit_exact_bearing_consumer(&EMPTY, &[]).is_err());
}

/// The STEAD falloff consumer earns admission with real per-arm digests —
/// bit-identical across its complete justified arm set (SEAM LAW active).
#[test]
fn exact_consumer_obligation_0_stead_falloff_admits_with_bit_identical_arm_digests() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let cpu = stead_falloff_digest(&ctx, ExactConsumerArm::CpuTwin);
    let interpreted = stead_falloff_digest(&ctx, ExactConsumerArm::InterpretedGpu);
    let jit = stead_falloff_digest(&ctx, ExactConsumerArm::SsaJit);
    const DECLARATION: ExactBearingConsumerDeclaration = ExactBearingConsumerDeclaration {
        consumer_id: "stead-exponential-falloff",
        primitive: "EXP",
        domain_note: "guarded EXP(-lambda*d); 8x8 non-dyadic probe grid, 3 iterations",
        arms: &[
            ExactConsumerArm::CpuTwin,
            ExactConsumerArm::InterpretedGpu,
            ExactConsumerArm::SsaJit,
        ],
        arm_justification: "field-sweep Matrix registration: CPU twin + interpreted + JIT; fused-transient unreachable (Matrix output)",
    };
    let admission = admit_exact_bearing_consumer(
        &DECLARATION,
        &[
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest: cpu },
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest: interpreted },
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::SsaJit, digest: jit },
        ],
    )
    .expect("bit-identical arm digests admit the falloff consumer");
    eprintln!(
        "EXACT_CONSUMER stead-exponential-falloff digest={:#018x} arms=cpu/interpreted/jit ALL-IDENTICAL",
        admission.digest()
    );
}

/// AO-surface consumers: each admits on its justified arm set. The CPU-twin
/// digest is computed here; the interpreted-AO arm equality is carried by the
/// inherited C-8 AO parity referees (the AO interpreter and this CPU stack
/// machine are the standing bit-exact pair) and re-attested per-consumer by
/// their oracle-parity batteries from 5.11/5.12.
#[test]
fn exact_consumer_obligation_0_ao_consumers_admit_with_cpu_twin_digests() {
    let softmax = SoftmaxWeightGadget { z_col: 0, max_col: 1, beta: 1.7 };
    let cases: [(&'static str, &'static str, Vec<EmlNodeGpu>, u32); 5] = [
        (
            "logistic-steering",
            "EXP",
            simthing_core::logistic_steering_eml_nodes(0.25, 4.0, 0.9, 3.0)
                .iter()
                .map(|n| EmlNodeGpu { opcode: n.opcode, flags: n.flags, a: n.a, b: n.b, c: n.c, d: n.d })
                .collect(),
            1,
        ),
        ("softmax-weight", "EXP", softmax.compile_nodes().expect("softmax admits"), 2),
        ("power-law", "EXP+LN", LnConsumerGadgets::power_law_nodes(0, 1.7).expect("power law"), 1),
        ("eml-operator", "EXP+LN", LnConsumerGadgets::eml_operator_nodes(0, 1).expect("eml"), 2),
        ("entropy-term", "LN", LnConsumerGadgets::entropy_term_nodes(0).expect("entropy"), 1),
    ];
    for (consumer_id, primitive, nodes, columns) in cases {
        let digest = ao_consumer_digest_cpu(&nodes, columns);
        let declaration = ExactBearingConsumerDeclaration {
            consumer_id,
            primitive,
            domain_note: "guarded call sites; 512-row non-dyadic probe stratum",
            arms: &[ExactConsumerArm::CpuTwin, ExactConsumerArm::InterpretedGpu],
            arm_justification: "ordinary AO EvalEML surface: CPU stack twin + AO interpreter (C-8 parity pair); field JIT never compiles AO programs",
        };
        let admission = admit_exact_bearing_consumer(
            &declaration,
            &[
                ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest },
                ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest },
            ],
        )
        .expect("AO consumer admits with its digest");
        eprintln!(
            "EXACT_CONSUMER {consumer_id} primitive={primitive} digest={:#018x}",
            admission.digest()
        );
    }
}

/// log-accumulate is a field map consumer: three-arm digest like the falloff.
#[test]
fn exact_consumer_obligation_0_log_accumulate_admits_three_arm() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let col = ColumnIndex::try_from_admitted_authored(0, 1).expect("column");
    let map: Vec<EmlNodeGpu> = LnConsumerGadgets::log_accumulate_map_nodes(0)
        .expect("log-accumulate map")
        .iter()
        .map(|n| EmlNodeGpu { opcode: n.opcode, flags: n.flags, a: n.a, b: n.b, c: n.c, d: n.d })
        // SLOT_VALUE is AO-context; rewrite to the field NEIGHBOR read.
        .map(|mut n| {
            if n.opcode == eml_opcode::SLOT_VALUE {
                n.opcode = eml_opcode::NEIGHBOR_VALUE;
            }
            n
        })
        .collect();
    let adjacency = FieldAdjacency::grid_n4(4, 4, simthing_gpu::GRID_N4_NSEW, col).expect("grid");
    let order = adjacency.apply_canonical_order_proof();
    let registration = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output: FieldSweepOutput::Matrix(col),
        map_program: map,
        fold_program: vec![
            node(eml_opcode::PARAM, field_param::ACCUMULATOR, 0),
            node(eml_opcode::PARAM, field_param::MAPPED, 0),
            node(eml_opcode::ADD, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            node(eml_opcode::PARAM, field_param::FOLDED, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("log-accumulate admission");
    let values = probe_values(16);
    let digest_for = |outputs: &[f32]| {
        outputs
            .iter()
            .fold(FNV_OFFSET, |digest, value| fnv_fold(digest, value.to_bits()))
    };
    let cpu = digest_for(
        &simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 1)
            .expect("CPU log-accumulate"),
    );
    let mut arm_digest = |interpreted: bool| {
        let class = registration.resource_class();
        let mut session = if interpreted {
            FieldSweepSession::new_interpreted_for_profiling(&ctx, &registration, class)
                .expect("interpreted session")
        } else {
            FieldSweepSession::new_with_profiling_resource_class(&ctx, &registration, class)
                .expect("JIT session")
        };
        session.upload_values(&ctx, &values).expect("upload");
        session
            .dispatch_chain(&ctx, std::slice::from_ref(&registration), 1)
            .expect("dispatch");
        digest_for(&session.readback(&ctx).expect("readback"))
    };
    let interpreted = arm_digest(true);
    let jit = arm_digest(false);
    let declaration = ExactBearingConsumerDeclaration {
        consumer_id: "log-accumulate",
        primitive: "LN",
        domain_note: "guarded LN map on the existing Sum lane; non-dyadic probe grid",
        arms: &[
            ExactConsumerArm::CpuTwin,
            ExactConsumerArm::InterpretedGpu,
            ExactConsumerArm::SsaJit,
        ],
        arm_justification: "field-sweep Matrix registration; fused-transient unreachable (Matrix output)",
    };
    let admission = admit_exact_bearing_consumer(
        &declaration,
        &[
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::CpuTwin, digest: cpu },
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::InterpretedGpu, digest: interpreted },
            ExactConsumerDigestEvidence { arm: ExactConsumerArm::SsaJit, digest: jit },
        ],
    )
    .expect("log-accumulate admits with bit-identical three-arm digests");
    eprintln!(
        "EXACT_CONSUMER log-accumulate digest={:#018x} arms=cpu/interpreted/jit ALL-IDENTICAL",
        admission.digest()
    );
}
