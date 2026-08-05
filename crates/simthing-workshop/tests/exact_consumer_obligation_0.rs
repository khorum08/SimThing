//! EXACT-CONSUMER-OBLIGATION-0 — the receiving half of the exact-primitive
//! door: every exact-bearing consumer carries its OWN digest over its OWN
//! probe domain across every execution arm it actually uses, reproduced
//! bit-identically. Exactness is never inherited (Exact-Value Provenance Law).
//!
//! There is exactly ONE consumer-evidence channel:
//! `ExactPrimitiveConsumerEvidence` verified by
//! `ExactPrimitiveAdmissionDoor::verify_consumer` (remand 5190634963 §1).
//! The execution-arm obligation is DERIVED from the consumer's concrete
//! execution shape by `derive_consumer_arms` — never a caller-authored list,
//! never a count (remand §2; DA #1642):
//! - `FieldSweepMatrix` → CpuTwin + InterpretedGpu + SsaJit; the
//!   fused-transient kernel is unreachable (fusion requires a Transient
//!   producer), so no fused arm exists to omit.
//! - `OrdinaryAccumulatorEvalEml` → CpuTwin + InterpretedGpu (the AO
//!   interpreter); the field JIT never compiles AO programs.
//!
//! The shape itself is BOUND to the production consumer surface (remand
//! 5190934274): an AO consumer cannot present a field shape and vice versa
//! (`ExactConsumerShapeNotBoundToConsumer`), and within the field family the
//! Matrix-vs-TransientFusable distinction rides a sealed proof mintable only
//! from an ADMITTED `FieldSweepRegistration`
//! (`exact_consumer_shape_proof()` reads the registration's typed `output` /
//! `transient_read_proof` — the same fields the fused-pair predicate uses).
//!
//! Every arm digest in this battery is measured by EXECUTING that arm here —
//! the AO interpreted digests run the real AO EvalEML GPU path per consumer
//! (remand §3); no digest is copied between arms and no inherited parity
//! battery is cited as substitute evidence.

use simthing_core::{
    eml_nodes, eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SlotIndex,
    SourceSpec,
};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, set_debug_readback_allowed, AccumulatorOpSession,
    EmlGpuProgramTable, FieldAdjacency, FieldLawProof, FieldSweepOutput,
    FieldSweepRegistrationRequest, FieldSweepSession, GpuContext, PackedAccumulatorUpload,
};
use simthing_kernel::{
    derive_consumer_arms, ExactBearingEvidence, ExactConsumerArm, ExactConsumerDigestEvidence,
    ExactConsumerExecutionShape, ExactConsumerShapeBinding, ExactPrimitiveAdmissionDoor,
    ExactPrimitiveConsumer, ExactPrimitiveConsumerEvidence, FieldSweepRegistration,
    LnConsumerGadgets, OpcodeGateError, SoftmaxWeightGadget, EXP_PRIMITIVE_NAME,
    LN_PRIMITIVE_NAME,
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

/// The ADMITTED STEAD falloff registration — the production surface the
/// sealed shape proof is minted from (`exact_consumer_shape_proof()`).
fn stead_falloff_registration() -> FieldSweepRegistration {
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
    compile_stead_exponential_falloff_field_sweep(spec).expect("falloff law")
}

/// The STEAD falloff consumer: digest per arm over a 8x8 grid of non-dyadic
/// intensities, three sweep iterations (accumulation stress on the seam).
fn stead_falloff_digest(ctx: &GpuContext, arm: ExactConsumerArm) -> u64 {
    let registration = stead_falloff_registration();
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
            unreachable!("falloff shape is FieldSweepMatrix; the fused arm does not derive")
        }
    };
    outputs
        .iter()
        .fold(FNV_OFFSET, |digest, value| fnv_fold(digest, value.to_bits()))
}

/// AO-surface consumers: CPU twin digest via the shared AO stack machine over
/// probe rows. Identical inputs and fold order to the interpreted arm below.
fn ao_consumer_digest_cpu(nodes: &[EmlNodeGpu], columns: u32) -> u64 {
    let rows = probe_values(512);
    rows.chunks(columns as usize)
        .filter(|chunk| chunk.len() == columns as usize)
        .fold(FNV_OFFSET, |digest, chunk| {
            let value = simthing_kernel::eval_eml_cpu(nodes, 0, chunk, columns, [0.0; 4]);
            fnv_fold(digest, value.to_bits())
        })
}

/// AO-surface consumers, INTERPRETED ARM: the same per-consumer probe domain
/// executed through the real AO EvalEML GPU interpreter (register → upload
/// tree → one EvalEML op per probe row → `tick_with_eml` → readback), hashed
/// independently in the same row order. Nothing is copied from the CPU twin.
fn ao_consumer_digest_interpreted_gpu(ctx: &GpuContext, nodes: &[EmlNodeGpu], columns: u32) -> u64 {
    set_debug_readback_allowed(true);
    let rows: Vec<Vec<f32>> = probe_values(512)
        .chunks(columns as usize)
        .filter(|chunk| chunk.len() == columns as usize)
        .map(<[f32]>::to_vec)
        .collect();
    let n_slots = rows.len() as u32;
    // One extra column receives the evaluated output so probe inputs stay
    // undisturbed for the SLOT_VALUE reads within the same tick.
    let n_cols = columns + 1;
    let out_col = ColumnIndex::try_from_admitted_authored(columns, n_cols).expect("output column");
    let values: Vec<f32> = rows
        .iter()
        .flat_map(|row| row.iter().copied().chain(std::iter::once(0.0)))
        .collect();

    let host_nodes: Vec<eml_nodes::EmlNode> = nodes
        .iter()
        .map(|n| eml_nodes::EmlNode {
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
        max_stack_depth: 0, // recomputed by register_formula's validator
        has_loops: false,
        has_recursion: false,
        display_name: "exact_consumer_obligation_probe".into(),
    };
    let mut reg = EmlExpressionRegistry::new();
    reg.register_formula(EmlTreeId(1), meta, host_nodes)
        .expect("register consumer program");
    let meta = reg.get(EmlTreeId(1)).expect("registered meta").clone();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 4);
    let mapping = table
        .upload_trees(ctx, &[(EmlTreeId(1), meta, nodes.to_vec())])
        .expect("upload consumer program");
    for (id, idx) in mapping {
        reg.mark_tree_uploaded(id, idx, table.generation)
            .expect("mark uploaded");
    }

    let ops: Vec<AccumulatorOp> = (0..n_slots)
        .map(|slot| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(slot),
                col: ColumnIndex::try_from_admitted_authored(0, n_cols).expect("input column"),
            },
            combine: CombineFn::EvalEML { tree_id: 1 },
            gate: GateSpec::Always,
            scale: ScaleSpec::Constant(1.0),
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(slot), out_col)],
        })
        .collect();
    let upload =
        PackedAccumulatorUpload::from_ops_with_eml(&ops, Some(&reg)).expect("pack EvalEML ops");
    let mut session = AccumulatorOpSession::new_attached(ctx, n_slots, n_cols, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("upload ops");
    session
        .tick_with_eml(ctx, 0, Some(&table))
        .expect("AO EvalEML tick");
    let gpu_values = session.readback_full(ctx).expect("readback");
    (0..n_slots as usize).fold(FNV_OFFSET, |digest, slot| {
        fnv_fold(digest, gpu_values[slot * n_cols as usize + columns as usize].to_bits())
    })
}

/// Route an admission through the ONE production consumer-evidence channel.
fn verify(
    consumer: ExactPrimitiveConsumer,
    measured_threshold_excess_bps: u32,
    exact_bearing: ExactBearingEvidence,
) -> Result<simthing_kernel::ExactPrimitiveConsumerKey, OpcodeGateError> {
    ExactPrimitiveAdmissionDoor::verify_consumer(ExactPrimitiveConsumerEvidence {
        consumer,
        measured_threshold_excess_bps,
        exact_bearing,
    })
}

fn row(arm: ExactConsumerArm, digest: u64) -> ExactConsumerDigestEvidence {
    ExactConsumerDigestEvidence { arm, digest }
}

#[test]
fn exact_consumer_obligation_0_admission_hard_errors_without_evidence() {
    // The sealed shape proof is minted from the REAL admitted falloff
    // registration (Matrix output, no transient read → FieldSweepMatrix).
    let falloff_proof = stead_falloff_registration().exact_consumer_shape_proof();
    assert_eq!(falloff_proof.shape(), ExactConsumerExecutionShape::FieldSweepMatrix);
    let bearing = |digests: Vec<ExactConsumerDigestEvidence>| ExactBearingEvidence::ExactBearing {
        consumer_id: "stead-exponential-falloff",
        primitive: EXP_PRIMITIVE_NAME,
        domain_note: "guarded EXP(-lambda*d), lambda,d >= 0; saturated tail",
        shape_binding: ExactConsumerShapeBinding::FieldSweep(falloff_proof),
        digests,
    };
    // Planted defect 1 (remand §2): the authored evidence OMITS a real
    // execution arm entirely — matching rows for the reduced pair only. The
    // production derivation still demands SsaJit for a FieldSweepMatrix
    // shape, so admission REDs; a caller cannot shrink its own obligation.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::FieldSweepEvalEml,
            2_967,
            bearing(vec![
                row(ExactConsumerArm::CpuTwin, 0x1234),
                row(ExactConsumerArm::InterpretedGpu, 0x1234),
            ]),
        ),
        Err(OpcodeGateError::ExactBearingConsumerWithoutDigestEvidence {
            consumer_id: "stead-exponential-falloff",
            missing_arm: "ssa-jit",
        })
    ));
    // Planted defect 2: exact-bearing declared with NO digest evidence at all
    // (the evidence-free declaration) — the very first derived arm REDs.
    assert!(matches!(
        verify(ExactPrimitiveConsumer::FieldSweepEvalEml, 2_967, bearing(vec![])),
        Err(OpcodeGateError::ExactBearingConsumerWithoutDigestEvidence {
            consumer_id: "stead-exponential-falloff",
            missing_arm: "cpu-twin",
        })
    ));
    // Planted defect 3: a zero digest is no evidence.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::FieldSweepEvalEml,
            2_967,
            bearing(vec![
                row(ExactConsumerArm::CpuTwin, 0),
                row(ExactConsumerArm::InterpretedGpu, 0x1234),
                row(ExactConsumerArm::SsaJit, 0x1234),
            ]),
        ),
        Err(OpcodeGateError::ExactBearingConsumerWithoutDigestEvidence { .. })
    ));
    // Planted defect 4: arm digests disagree — the pre-repair seam shape
    // (JIT digest differing from CPU/interpreted) hard-errors.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::FieldSweepEvalEml,
            2_967,
            bearing(vec![
                row(ExactConsumerArm::CpuTwin, 0x1234),
                row(ExactConsumerArm::InterpretedGpu, 0x1234),
                row(ExactConsumerArm::SsaJit, 0x9999),
            ]),
        ),
        Err(OpcodeGateError::ExactConsumerArmDigestMismatch {
            consumer_id: "stead-exponential-falloff",
            arm: "ssa-jit",
            ..
        })
    ));
    // Planted defect 5: evidence for an arm the derived shape does NOT
    // contain (an AO consumer presenting a field-JIT digest) is rejected —
    // the derivation is authoritative in both directions.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::OrdinaryAccumulatorEvalEml,
            2_967,
            ExactBearingEvidence::ExactBearing {
                consumer_id: "logistic-steering",
                primitive: EXP_PRIMITIVE_NAME,
                domain_note: "planted",
                shape_binding: ExactConsumerShapeBinding::OrdinaryAccumulatorEvalEml,
                digests: vec![
                    row(ExactConsumerArm::CpuTwin, 0x1234),
                    row(ExactConsumerArm::InterpretedGpu, 0x1234),
                    row(ExactConsumerArm::SsaJit, 0x1234),
                ],
            },
        ),
        Err(OpcodeGateError::ExactConsumerArmNotDerived {
            consumer_id: "logistic-steering",
            arm: "ssa-jit",
        })
    ));
    // Planted defect 6 (remand 5190934274): the production bypass — a
    // FIELD-SWEEP consumer presenting the AO shape with two internally
    // consistent matching digests, silently shedding its SSA-JIT obligation.
    // The shape is not bound to the consumer surface, so admission REJECTS
    // before any digest row is read.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::FieldSweepEvalEml,
            2_967,
            ExactBearingEvidence::ExactBearing {
                consumer_id: "stead-exponential-falloff",
                primitive: EXP_PRIMITIVE_NAME,
                domain_note: "planted bypass",
                shape_binding: ExactConsumerShapeBinding::OrdinaryAccumulatorEvalEml,
                digests: vec![
                    row(ExactConsumerArm::CpuTwin, 0x1234),
                    row(ExactConsumerArm::InterpretedGpu, 0x1234),
                ],
            },
        ),
        Err(OpcodeGateError::ExactConsumerShapeNotBoundToConsumer {
            consumer_id: "stead-exponential-falloff",
            consumer: ExactPrimitiveConsumer::FieldSweepEvalEml,
        })
    ));
    // Planted defect 7: the inverse direction — an AO consumer presenting a
    // field-sweep shape proof is equally unbound and rejects.
    assert!(matches!(
        verify(
            ExactPrimitiveConsumer::OrdinaryAccumulatorEvalEml,
            2_967,
            ExactBearingEvidence::ExactBearing {
                consumer_id: "logistic-steering",
                primitive: EXP_PRIMITIVE_NAME,
                domain_note: "planted bypass",
                shape_binding: ExactConsumerShapeBinding::FieldSweep(falloff_proof),
                digests: vec![
                    row(ExactConsumerArm::CpuTwin, 0x1234),
                    row(ExactConsumerArm::InterpretedGpu, 0x1234),
                    row(ExactConsumerArm::SsaJit, 0x1234),
                ],
            },
        ),
        Err(OpcodeGateError::ExactConsumerShapeNotBoundToConsumer {
            consumer_id: "logistic-steering",
            consumer: ExactPrimitiveConsumer::OrdinaryAccumulatorEvalEml,
        })
    ));
    // The derivation itself is total over the production shapes and never
    // empty — there is no shape whose obligation collapses to nothing.
    for shape in [
        ExactConsumerExecutionShape::FieldSweepMatrix,
        ExactConsumerExecutionShape::FieldSweepTransientFusable,
        ExactConsumerExecutionShape::OrdinaryAccumulatorEvalEml,
    ] {
        assert!(!derive_consumer_arms(shape).is_empty());
    }
}

/// The STEAD falloff consumer earns admission with real per-arm digests —
/// bit-identical across its complete DERIVED arm set (SEAM LAW active).
#[test]
fn exact_consumer_obligation_0_stead_falloff_admits_with_bit_identical_arm_digests() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let cpu = stead_falloff_digest(&ctx, ExactConsumerArm::CpuTwin);
    let interpreted = stead_falloff_digest(&ctx, ExactConsumerArm::InterpretedGpu);
    let jit = stead_falloff_digest(&ctx, ExactConsumerArm::SsaJit);
    // Necessity provenance: worst staircase-vs-smooth deviation over the EXP
    // steering domain = 2,967 bps of span (5.11 staircase-deviation referee,
    // simthing-core); the falloff previously rode the same CostBand staircase.
    verify(
        ExactPrimitiveConsumer::FieldSweepEvalEml,
        2_967,
        ExactBearingEvidence::ExactBearing {
            consumer_id: "stead-exponential-falloff",
            primitive: EXP_PRIMITIVE_NAME,
            domain_note: "guarded EXP(-lambda*d); 8x8 non-dyadic probe grid, 3 iterations",
            shape_binding: ExactConsumerShapeBinding::FieldSweep(
                stead_falloff_registration().exact_consumer_shape_proof(),
            ),
            digests: vec![
                row(ExactConsumerArm::CpuTwin, cpu),
                row(ExactConsumerArm::InterpretedGpu, interpreted),
                row(ExactConsumerArm::SsaJit, jit),
            ],
        },
    )
    .expect("bit-identical arm digests admit the falloff consumer");
    eprintln!(
        "EXACT_CONSUMER stead-exponential-falloff digest={cpu:#018x} arms=cpu/interpreted/jit ALL-IDENTICAL"
    );
}

/// AO-surface consumers: each admits on its DERIVED arm pair with an
/// independently-executed digest per arm — the interpreted digest comes from
/// the real AO EvalEML GPU path over the same probe rows (remand §3).
#[test]
fn exact_consumer_obligation_0_ao_consumers_admit_with_independent_arm_digests() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let softmax = SoftmaxWeightGadget { z_col: 0, max_col: 1, beta: 1.7 };
    // (id, primitive, nodes, columns, measured necessity bps + referee)
    // bps provenance: 2_967 = 5.11 staircase-deviation referee (EXP steering
    // domain); 10_000 = 5.12 product-vs-logsum representability gap referee
    // (multiplicative dynamics cannot ride the Sum lane at all without LN).
    let cases: [(&'static str, &'static str, Vec<EmlNodeGpu>, u32, u32); 5] = [
        (
            "logistic-steering",
            EXP_PRIMITIVE_NAME,
            simthing_core::logistic_steering_eml_nodes(0.25, 4.0, 0.9, 3.0)
                .iter()
                .map(|n| EmlNodeGpu { opcode: n.opcode, flags: n.flags, a: n.a, b: n.b, c: n.c, d: n.d })
                .collect(),
            1,
            2_967,
        ),
        ("softmax-weight", EXP_PRIMITIVE_NAME, softmax.compile_nodes().expect("softmax admits"), 2, 2_967),
        ("power-law", "EXP+LN", LnConsumerGadgets::power_law_nodes(0, 1.7).expect("power law"), 1, 10_000),
        ("eml-operator", "EXP+LN", LnConsumerGadgets::eml_operator_nodes(0, 1).expect("eml"), 2, 10_000),
        ("entropy-term", LN_PRIMITIVE_NAME, LnConsumerGadgets::entropy_term_nodes(0).expect("entropy"), 1, 10_000),
    ];
    for (consumer_id, primitive, nodes, columns, bps) in cases {
        let cpu = ao_consumer_digest_cpu(&nodes, columns);
        let interpreted = ao_consumer_digest_interpreted_gpu(&ctx, &nodes, columns);
        assert_eq!(
            interpreted, cpu,
            "{consumer_id}: independently-executed AO interpreted digest must \
             reproduce the CPU twin bit-for-bit"
        );
        verify(
            ExactPrimitiveConsumer::OrdinaryAccumulatorEvalEml,
            bps,
            ExactBearingEvidence::ExactBearing {
                consumer_id,
                primitive,
                domain_note: "guarded call sites; 512-value non-dyadic probe stratum",
                shape_binding: ExactConsumerShapeBinding::OrdinaryAccumulatorEvalEml,
                digests: vec![
                    row(ExactConsumerArm::CpuTwin, cpu),
                    row(ExactConsumerArm::InterpretedGpu, interpreted),
                ],
            },
        )
        .expect("AO consumer admits with independently-executed arm digests");
        eprintln!(
            "EXACT_CONSUMER {consumer_id} primitive={primitive} cpu={cpu:#018x} interpreted-gpu={interpreted:#018x} INDEPENDENT+IDENTICAL"
        );
    }
}

/// log-accumulate is a field map consumer: three-arm digest like the falloff.
/// The ADMITTED log-accumulate registration (Matrix output, no transient
/// read) — the production surface its sealed shape proof is minted from.
fn log_accumulate_registration() -> FieldSweepRegistration {
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
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
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
    .expect("log-accumulate admission")
}

#[test]
fn exact_consumer_obligation_0_log_accumulate_admits_three_arm() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let registration = log_accumulate_registration();
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
    verify(
        ExactPrimitiveConsumer::FieldSweepEvalEml,
        10_000,
        ExactBearingEvidence::ExactBearing {
            consumer_id: "log-accumulate",
            primitive: LN_PRIMITIVE_NAME,
            domain_note: "guarded LN map on the existing Sum lane; non-dyadic probe grid",
            shape_binding: ExactConsumerShapeBinding::FieldSweep(
                registration.exact_consumer_shape_proof(),
            ),
            digests: vec![
                row(ExactConsumerArm::CpuTwin, cpu),
                row(ExactConsumerArm::InterpretedGpu, interpreted),
                row(ExactConsumerArm::SsaJit, jit),
            ],
        },
    )
    .expect("log-accumulate admits with bit-identical three-arm digests");
    eprintln!(
        "EXACT_CONSUMER log-accumulate digest={cpu:#018x} arms=cpu/interpreted/jit ALL-IDENTICAL"
    );
}
