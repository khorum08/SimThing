//! EML-ARITHMETIC-SEMANTICS-0 language-level witnesses (re-homed from 5.13).
//!
//! 5.14 deleted per-consumer exactness evidence plumbing
//! (`ExactBearingEvidence`, arm digests as admission, `derive_consumer_arms`).
//! Cross-arm bit identity survives here as language-level witnesses — not
//! admission digests, not census rows, not declarations.

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
    FieldSweepRegistration, LnConsumerGadgets, SoftmaxWeightGadget, EXP_PRIMITIVE_NAME,
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
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b,
        c: 0,
        d: 0,
    }
}

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

#[derive(Clone, Copy)]
enum FieldArm {
    CpuTwin,
    InterpretedGpu,
    SsaJit,
}

fn stead_falloff_digest(ctx: &GpuContext, arm: FieldArm) -> u64 {
    let registration = stead_falloff_registration();
    let raw = probe_values(8 * 8);
    let values: Vec<f32> = raw.iter().flat_map(|value| [*value, 0.0]).collect();
    let outputs = match arm {
        FieldArm::CpuTwin => {
            simthing_gpu::execute_field_sweep_cpu_iterations(&values, &registration, 3)
                .expect("CPU falloff")
        }
        FieldArm::InterpretedGpu | FieldArm::SsaJit => {
            let class = registration.resource_class();
            let mut session = if matches!(arm, FieldArm::InterpretedGpu) {
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
    };
    outputs.iter().fold(FNV_OFFSET, |digest, value| {
        fnv_fold(digest, value.to_bits())
    })
}

fn ao_consumer_digest_cpu(nodes: &[EmlNodeGpu], columns: u32) -> u64 {
    let rows = probe_values(512);
    rows.chunks(columns as usize)
        .filter(|chunk| chunk.len() == columns as usize)
        .fold(FNV_OFFSET, |digest, chunk| {
            let value = simthing_kernel::eval_eml_cpu(nodes, 0, chunk, columns, [0.0; 4]);
            fnv_fold(digest, value.to_bits())
        })
}

fn ao_consumer_digest_interpreted_gpu(ctx: &GpuContext, nodes: &[EmlNodeGpu], columns: u32) -> u64 {
    set_debug_readback_allowed(true);
    let rows: Vec<Vec<f32>> = probe_values(512)
        .chunks(columns as usize)
        .filter(|chunk| chunk.len() == columns as usize)
        .map(<[f32]>::to_vec)
        .collect();
    let n_slots = rows.len() as u32;
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
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: "language_witness_probe".into(),
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
        fnv_fold(
            digest,
            gpu_values[slot * n_cols as usize + columns as usize].to_bits(),
        )
    })
}

fn log_accumulate_registration() -> FieldSweepRegistration {
    let col = ColumnIndex::try_from_admitted_authored(0, 1).expect("column");
    let map: Vec<EmlNodeGpu> = LnConsumerGadgets::log_accumulate_map_nodes(0)
        .expect("log-accumulate map")
        .iter()
        .map(|n| EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
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
fn language_witness_stead_falloff_bit_identical_across_arms() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let cpu = stead_falloff_digest(&ctx, FieldArm::CpuTwin);
    let interpreted = stead_falloff_digest(&ctx, FieldArm::InterpretedGpu);
    let jit = stead_falloff_digest(&ctx, FieldArm::SsaJit);
    assert_eq!(cpu, interpreted, "falloff CPU↔interpreted");
    assert_eq!(cpu, jit, "falloff CPU↔ssa-jit");
    eprintln!("LANGUAGE_WITNESS stead-exponential-falloff digest={cpu:#018x}");
}

#[test]
fn language_witness_ao_consumers_cpu_matches_interpreted() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let softmax = SoftmaxWeightGadget {
        z_col: 0,
        max_col: 1,
        beta: 1.7,
    };
    let cases: [(&'static str, &'static str, Vec<EmlNodeGpu>, u32); 5] = [
        (
            "logistic-steering",
            EXP_PRIMITIVE_NAME,
            simthing_core::logistic_steering_eml_nodes(0.25, 4.0, 0.9, 3.0)
                .iter()
                .map(|n| EmlNodeGpu {
                    opcode: n.opcode,
                    flags: n.flags,
                    a: n.a,
                    b: n.b,
                    c: n.c,
                    d: n.d,
                })
                .collect(),
            1,
        ),
        (
            "softmax-weight",
            EXP_PRIMITIVE_NAME,
            softmax.compile_nodes().expect("softmax admits"),
            2,
        ),
        (
            "power-law",
            "EXP+LN",
            LnConsumerGadgets::power_law_nodes(0, 1.7).expect("power law"),
            1,
        ),
        (
            "eml-operator",
            "EXP+LN",
            LnConsumerGadgets::eml_operator_nodes(0, 1).expect("eml"),
            2,
        ),
        (
            "entropy-term",
            LN_PRIMITIVE_NAME,
            LnConsumerGadgets::entropy_term_nodes(0).expect("entropy"),
            1,
        ),
    ];
    for (consumer_id, primitive, nodes, columns) in cases {
        let cpu = ao_consumer_digest_cpu(&nodes, columns);
        let interpreted = ao_consumer_digest_interpreted_gpu(&ctx, &nodes, columns);
        assert_eq!(
            interpreted, cpu,
            "{consumer_id}: AO interpreted must match CPU twin"
        );
        eprintln!("LANGUAGE_WITNESS {consumer_id} primitive={primitive} digest={cpu:#018x}");
    }
}

#[test]
fn language_witness_log_accumulate_bit_identical_across_arms() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let registration = log_accumulate_registration();
    let values = probe_values(16);
    let digest_for = |outputs: &[f32]| {
        outputs.iter().fold(FNV_OFFSET, |digest, value| {
            fnv_fold(digest, value.to_bits())
        })
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
    assert_eq!(cpu, interpreted, "log-accumulate CPU↔interpreted");
    assert_eq!(cpu, jit, "log-accumulate CPU↔ssa-jit");
    eprintln!("LANGUAGE_WITNESS log-accumulate digest={cpu:#018x}");
}
