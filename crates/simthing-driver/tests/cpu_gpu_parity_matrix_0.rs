//! TP-PURGE-0 Remand 4 — `cpu_gpu_parity_matrix` (DA `5135942768` / Remand `5136696481`).
//!
//! Five approved cases. Inline input. Each case: live kernel + CPU reference + GPU path
//! + planted defect. With `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`, every case must execute.
//!
//! `accumulator` is a subpath table over absorbed families (transfer, emission, intent,
//! velocity, weighted-mean, owner-silo Sum, bh2 W-compose). `rf-need-binding` remains the
//! zero-antecedent live `need_binding` path.

use bytemuck::cast_slice;
use simthing_core::{
    eml_nodes, eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, InputSpec, ScaleSpec,
    SimThingId, SlotIndex, SourceSpec, StructuralScalarChannel, SubFieldRole,
};
use simthing_driver::need_binding::{
    build_need_binding_ops, ResolvedFullCell, ResolvedNeedBinding,
};
use simthing_gpu::{
    cpu_horizon, cpu_scatter_indexed, cpu_w_impedance_compose_oracle, encode_column,
    encode_emission_plan, encode_transfer_plan, eval_eml_cpu, execute_ops_cpu, params_from_config,
    plan_emission_ops, plan_transfer_ops, plan_velocity_integration, set_debug_readback_allowed,
    AccumulatorOpSession, EmissionFormula, EmissionRegistration, EmlGpuProgramTable, GovernedPair,
    GpuContext, IndexedScatterOp, IntentDelta, PackedAccumulatorUpload, PackedIntentUpload,
    ScatterEntry, StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, TransferInputRef, TransferRegistration, WImpedanceComposeConfig,
    WImpedanceComposeOp, WImpedanceComposeProfile, CLAMP_UNBOUNDED,
};
use wgpu::util::DeviceExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParityCase {
    Mobility,
    EmlEval,
    Accumulator,
    RfNeedBinding,
    FluxChoke,
}

const CASES: [ParityCase; 5] = [
    ParityCase::Mobility,
    ParityCase::EmlEval,
    ParityCase::Accumulator,
    ParityCase::RfNeedBinding,
    ParityCase::FluxChoke,
];

fn require_gpu() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(ctx) => Some(ctx),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU adapter required for cpu_gpu_parity_matrix");
        }
        Err(_) => None,
    }
}

fn bits_eq(cpu: &[f32], gpu: &[f32]) -> bool {
    cpu.len() == gpu.len()
        && cpu
            .iter()
            .zip(gpu.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

fn readback_buffer(ctx: &GpuContext, buf: &wgpu::Buffer, floats: usize) -> Vec<f32> {
    let bytes = (floats * std::mem::size_of::<f32>()) as u64;
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("parity_staging"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("parity_copy"),
        });
    encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, bytes);
    ctx.queue.submit(Some(encoder.finish()));
    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| tx.send(r).unwrap());
    ctx.device.poll(wgpu::Maintain::Wait);
    rx.recv().unwrap().unwrap();
    let data = slice.get_mapped_range();
    let out = cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();
    out
}

fn col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn case_mobility(ctx: &GpuContext, plant_defect: bool) -> bool {
    let src_host = vec![1.0f32, 2.0, 3.0, 4.0];
    let mut dst_cpu = vec![0.0f32; 4];
    let entries = [
        ScatterEntry {
            src_index: 0,
            dst_index: 2,
        },
        ScatterEntry {
            src_index: 1,
            dst_index: 3,
        },
    ];
    cpu_scatter_indexed(&src_host, &mut dst_cpu, &entries);
    let gpu_entries = if plant_defect {
        [
            ScatterEntry {
                src_index: 0,
                dst_index: 3,
            },
            ScatterEntry {
                src_index: 1,
                dst_index: 2,
            },
        ]
    } else {
        entries
    };
    let op = IndexedScatterOp::new(ctx);
    let src = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mob_src"),
        contents: cast_slice(&src_host),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    let dst = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mob_dst"),
        contents: cast_slice(&[0.0f32; 4]),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    op.dispatch(ctx, &src, &dst, &gpu_entries).expect("scatter");
    bits_eq(&dst_cpu, &readback_buffer(ctx, &dst, 4))
}

fn eml_nodes(scale: f32) -> Vec<EmlNodeGpu> {
    vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::LITERAL_F32,
            flags: 0,
            a: scale.to_bits(),
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
            opcode: eml_opcode::RETURN_TOP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ]
}

fn case_eml_eval(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let values = [2.0f32];
    let cpu = eval_eml_cpu(&eml_nodes(3.0), 0, &values, 1, [0.0; 4]);
    let gpu_scale = if plant_defect { 7.0 } else { 3.0 };
    let meta = EmlFormulaMeta {
        tree_id: EmlTreeId(1),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: Default::default(),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 4,
        max_stack_depth: 2,
        has_loops: false,
        has_recursion: false,
        display_name: "tp_purge_eml".into(),
    };
    let host_nodes: Vec<eml_nodes::EmlNode> = eml_nodes(gpu_scale)
        .into_iter()
        .map(|n| eml_nodes::EmlNode {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
    let mut reg = EmlExpressionRegistry::new();
    reg.register_formula(EmlTreeId(1), meta.clone(), host_nodes)
        .expect("register");
    let mut table = EmlGpuProgramTable::new(ctx, 32, 4);
    let mapping = table
        .upload_trees(ctx, &[(EmlTreeId(1), meta, eml_nodes(gpu_scale))])
        .expect("upload");
    for (id, idx) in mapping {
        reg.mark_tree_uploaded(id, idx, table.generation)
            .expect("mark");
    }
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: col(0),
        },
        combine: CombineFn::EvalEML { tree_id: 1 },
        gate: GateSpec::Always,
        scale: ScaleSpec::Constant(1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(0), col(0))],
    };
    let upload =
        PackedAccumulatorUpload::from_ops_with_eml(std::slice::from_ref(&op), Some(&reg))
            .expect("pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, 1, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("ops");
    session
        .tick_with_eml(ctx, 0, Some(&table))
        .expect("tick eml");
    let gpu_vals = session.readback_full(ctx).expect("readback");
    cpu.to_bits() == gpu_vals[0].to_bits()
}

/// Transfer subpath: live `plan_transfer_ops` + `encode_transfer_plan` + AO tick.
/// Defect: corrupt GPU `scale_a` (max_transfer) after encode.
fn accumulator_transfer(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let n_cols = 2u32;
    let values = vec![10.0f32, 2.0];
    let regs = [TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: col(0),
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: col(1),
        output_scale: 1.0,
        max_transfer: Some(3.0),
        tree_id: None,
        order_band: 0,
    }];
    let plan = plan_transfer_ops(&regs).expect("transfer plan");
    let mut cpu = values.clone();
    execute_ops_cpu(&mut cpu, &plan.ops, 0, n_cols).expect("cpu transfer");
    let mut gpu_ops = encode_transfer_plan(&plan, &[]).expect("encode transfer");
    if plant_defect {
        gpu_ops[0].scale_a = 9.0f32.to_bits();
    }
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_cols, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session
        .upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_gpu_ops(gpu_ops).expect("pack"),
        )
        .expect("upload");
    session.tick(ctx, 0).expect("tick");
    bits_eq(&cpu, &session.readback_full(ctx).expect("rb"))
}

/// Emission subpath: live `plan_emission_ops` + `encode_emission_plan` + emission readback.
/// Defect: rewrite GPU constant source bits after encode.
fn accumulator_emission(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let values = vec![1.0f32];
    let regs = [EmissionRegistration {
        source_slot: 0,
        source_col: col(0),
        tree_id: None,
        formula: EmissionFormula::Constant { value: 3.0 },
        max_emit: None,
        reg_idx: 7,
    }];
    let plan = plan_emission_ops(&regs, None).expect("emission plan");
    // CPU EmitEvent twin: floor(max(write_value,0)) with Constant source = 3.
    let cpu_emit = 3u32;
    let mut gpu_ops = encode_emission_plan(&plan, None).expect("encode emission");
    if plant_defect {
        // Constant source encodes value bits into source_slot; corrupt after encode.
        gpu_ops[0].source_slot = 9.0f32.to_bits();
    }
    let mut session = AccumulatorOpSession::with_emission_capacity(ctx, 1, 1, 4);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session
        .upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_gpu_ops(gpu_ops).expect("pack"),
        )
        .expect("upload");
    session.tick(ctx, 0).expect("tick");
    let records = session.readback_emissions(ctx).expect("emissions");
    records.len() == 1
        && records[0].reg_idx() == 7
        && records[0].emit_count() == cpu_emit
}

/// Intent subpath: live `PackedIntentUpload` + `upload_packed_intent_ops` + tick.
/// Defect: encode a corrupted add term on the GPU intent packet only.
fn accumulator_intent(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let n_dims = 1u32;
    let values = vec![4.0f32];
    let deltas = [IntentDelta {
        slot: 0,
        col: 0,
        mul: 2.0,
        add: 1.0,
    }];
    let mut cpu = values.clone();
    // Live CPU intent oracle: v = v*mul + add.
    cpu[0] = cpu[0] * deltas[0].mul + deltas[0].add;
    let gpu_deltas = if plant_defect {
        [IntentDelta {
            slot: 0,
            col: 0,
            mul: 2.0,
            add: 99.0,
        }]
    } else {
        deltas
    };
    let upload = PackedIntentUpload::from_deltas(&gpu_deltas).expect("intent pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_dims, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session
        .upload_packed_intent_ops(ctx, &upload)
        .expect("intent upload");
    session.tick(ctx, 0).expect("intent tick");
    bits_eq(&cpu, &session.readback_full(ctx).expect("rb"))
}

/// Velocity subpath: live `plan_velocity_integration` + `encode_velocity_into`.
/// Defect: dispatch with wrong dt while CPU uses sealed dt.
fn accumulator_velocity(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let n_dims = 2u32;
    let values = vec![0.0f32, 2.0]; // amount, velocity
    let dt = 1.0f32;
    let pair = GovernedPair {
        governed_col: encode_column(col(0)),
        governing_col: encode_column(col(1)),
        clamp_min: f32::NEG_INFINITY,
        clamp_max: f32::INFINITY,
        vel_max: f32::INFINITY,
        clamp_kind: CLAMP_UNBOUNDED,
    };
    let plan = plan_velocity_integration(std::slice::from_ref(&pair), 1);
    let mut cpu = values.clone();
    let effective_vel = cpu[1].clamp(-pair.vel_max, pair.vel_max);
    cpu[0] = cpu[0] + effective_vel * dt;
    let gpu_dt = if plant_defect { 3.0f32 } else { dt };
    let mut session = AccumulatorOpSession::new_attached(ctx, 1, n_dims, plan.ops.len() as u32);
    session
        .upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_gpu_ops(plan.ops.clone()).expect("pack"),
        )
        .expect("vel ops");
    let values_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("vel_values"),
        contents: cast_slice(&values),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let prev_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("vel_prev"),
        contents: cast_slice(&values),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("vel_enc"),
        });
    session.encode_velocity_into(ctx, &mut encoder, &values_buf, &prev_buf, gpu_dt);
    ctx.queue.submit(Some(encoder.finish()));
    bits_eq(&cpu, &readback_buffer(ctx, &values_buf, values.len()))
}

/// Weighted-mean subpath: live WeightedMean SlotRange AO path.
/// Defect: corrupt GPU weight column after encode.
fn accumulator_weighted_mean(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let n_slots = 3u32;
    let n_cols = 2u32;
    // slots 0..1 contribute; slot 2 is aggregate target.
    let mut values = vec![0.0f32; (n_slots * n_cols) as usize];
    values[0] = 2.0; // s0 value
    values[1] = 1.0; // s0 weight
    values[2] = 4.0; // s1 value
    values[3] = 1.0; // s1 weight
    let mut sum_vw = 0.0f32;
    let mut sum_w = 0.0f32;
    for s in 0..2u32 {
        let base = (s * n_cols) as usize;
        sum_vw += values[base] * values[base + 1];
        sum_w += values[base + 1];
    }
    let mut cpu = values.clone();
    cpu[(2 * n_cols) as usize] = sum_vw / sum_w;
    let op = AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: SlotIndex::new(0),
            count: 2,
            col: col(0),
        },
        combine: CombineFn::WeightedMean {
            weight_col: col(1),
        },
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(2), col(0))],
    };
    let mut upload = PackedAccumulatorUpload::from_ops(std::slice::from_ref(&op)).expect("wm pack");
    if plant_defect {
        let mut ops = upload.ops().to_vec();
        ops[0].combine_a = encode_column(col(0)); // wrong weight col
        upload = PackedAccumulatorUpload::from_gpu_ops(ops).expect("wm defect");
    }
    let mut session = AccumulatorOpSession::new_attached(ctx, n_slots, n_cols, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("upload");
    session.tick(ctx, 0).expect("tick");
    bits_eq(&cpu, &session.readback_full(ctx).expect("rb"))
}

/// Owner-silo GPU subpath: live ConjunctiveCrossing Sum shape used by
/// `compile_owner_silo_gpu_tick_plan` (participant → aggregate).
/// Defect: drop a participant input from the GPU conjunctive sum.
fn accumulator_owner_silo(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let input_col = StructuralScalarChannel::INPUT.into_plan_column();
    let output_col = StructuralScalarChannel::OUTPUT.into_plan_column();
    let n_dims = input_col.raw_u32().max(output_col.raw_u32()) + 1;
    let participant_count = 2u32;
    let aggregate_slot = participant_count;
    let inputs: Vec<InputSpec> = (0..participant_count)
        .map(|slot| InputSpec {
            slot: SlotIndex::new(slot),
            col: input_col,
            unit_cost: 1.0,
        })
        .collect();
    let cpu_op = AccumulatorOp {
        source: SourceSpec::ConjunctiveCrossing {
            inputs: inputs.clone(),
        },
        combine: CombineFn::Sum,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(SlotIndex::new(aggregate_slot), output_col)],
    };
    let mut gpu_op = cpu_op.clone();
    if plant_defect {
        gpu_op.source = SourceSpec::ConjunctiveCrossing {
            inputs: vec![inputs[0].clone()],
        };
    }
    let slot_count = participant_count + 1;
    let mut values = vec![0.0f32; (slot_count * n_dims) as usize];
    values[input_col.raw_u32() as usize] = 3.0;
    values[(n_dims + input_col.raw_u32()) as usize] = 5.0;
    let mut cpu = values.clone();
    execute_ops_cpu(&mut cpu, std::slice::from_ref(&cpu_op), 0, n_dims).expect("cpu silo");
    let upload =
        PackedAccumulatorUpload::from_ops_resolving_input_lists(std::slice::from_ref(&gpu_op))
            .expect("silo pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, slot_count, n_dims, 1);
    session.upload_values(ctx, &values);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("upload");
    session.tick(ctx, 0).expect("tick");
    bits_eq(&cpu, &session.readback_full(ctx).expect("rb"))
}

/// BH2 / W-impedance composition subpath.
/// Defect: corrupt GPU profile weight_a.
fn accumulator_bh2_w(ctx: &GpuContext, plant_defect: bool) -> bool {
    let config = WImpedanceComposeConfig {
        width: 2,
        height: 2,
        n_dims: 4,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![WImpedanceComposeProfile {
            weight_a: 1.0,
            weight_b: 0.5,
            output_w_col: 3,
        }],
    };
    let mut values = vec![0.0f32; config.values_len()];
    for cell in 0..4 {
        let b = cell * 4;
        values[b] = 1.0;
        values[b + 1] = 0.2;
        values[b + 2] = 0.4;
    }
    let cpu = cpu_w_impedance_compose_oracle(&values, &config);
    let mut gpu_config = config.clone();
    if plant_defect {
        gpu_config.profiles[0].weight_a = 9.0;
    }
    let op = WImpedanceComposeOp::new(ctx);
    let buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("w_vals"),
        contents: cast_slice(&values),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    op.compose_resident_field(ctx, &buf, &gpu_config)
        .expect("compose");
    bits_eq(&cpu, &readback_buffer(ctx, &buf, values.len()))
}

fn case_accumulator(ctx: &GpuContext, plant_defect: bool) -> bool {
    accumulator_transfer(ctx, plant_defect)
        && accumulator_emission(ctx, plant_defect)
        && accumulator_intent(ctx, plant_defect)
        && accumulator_velocity(ctx, plant_defect)
        && accumulator_weighted_mean(ctx, plant_defect)
        && accumulator_owner_silo(ctx, plant_defect)
        && accumulator_bh2_w(ctx, plant_defect)
}

fn inline_need_binding(plant_defect: bool) -> (ResolvedNeedBinding, Vec<f32>, usize, usize) {
    // Layout: 2 slots × 5 cols. Source row 1 holds input/weight; participant row 0
    // receives staged cells then EvalEML need write via live need_binding ops.
    let n_slots = 2usize;
    let n_cols = 5usize;
    let staged_in = col(2);
    let staged_w = col(3);
    let need = col(4);
    let mut nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: encode_column(staged_in),
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: encode_column(staged_w),
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
            opcode: eml_opcode::RETURN_TOP,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    if plant_defect {
        // Corrupt GPU-side formula literal path by rewriting the MUL into ADD.
        nodes[2].opcode = eml_opcode::ADD;
    }
    let binding = ResolvedNeedBinding {
        id: "inline_nb".into(),
        profile: "weighted".into(),
        participant_slot: 0,
        participant_id: SimThingId::from_session_raw(1),
        eml_source_slot: 0,
        need_col: need,
        inputs: vec![ResolvedFullCell {
            entity: "src".into(),
            simthing_id: SimThingId::from_session_raw(2),
            slot: 1,
            col: col(0),
            role: SubFieldRole::Named("input".into()),
        }],
        weights: vec![ResolvedFullCell {
            entity: "src".into(),
            simthing_id: SimThingId::from_session_raw(2),
            slot: 1,
            col: col(1),
            role: SubFieldRole::Named("weight".into()),
        }],
        staged_input_cols: vec![staged_in],
        staged_weight_cols: vec![staged_w],
        nodes,
        threshold: 0.0,
        event_kind: 1,
    };
    // Slot-major values: participant then source (input=3, weight=2 → need=6).
    let mut values = vec![0.0f32; n_slots * n_cols];
    values[n_cols + 0] = 3.0;
    values[n_cols + 1] = 2.0;
    (binding, values, n_slots, n_cols)
}

fn case_rf_need_binding(ctx: &GpuContext, plant_defect: bool) -> bool {
    set_debug_readback_allowed(true);
    let (binding, initial, n_slots, n_cols) = inline_need_binding(false);
    let (gpu_binding, _, _, _) = inline_need_binding(plant_defect);

    // CPU reference: live need_binding Identity stage ops + eval_eml_cpu for EvalEML
    // (CPU AccumulatorOp oracle does not execute CombineFn::EvalEML).
    let mut eml_reg = EmlExpressionRegistry::new();
    let cpu_ops = build_need_binding_ops(std::slice::from_ref(&binding), &mut eml_reg);
    let stage_ops: Vec<AccumulatorOp> = cpu_ops
        .iter()
        .filter(|op| matches!(op.combine, CombineFn::Identity))
        .cloned()
        .collect();
    let mut cpu_vals = initial.clone();
    execute_ops_cpu(&mut cpu_vals, &stage_ops, 0, n_cols as u32).expect("cpu stage");
    let need = eval_eml_cpu(&binding.nodes, 0, &cpu_vals, n_cols as u32, [0.0; 4]);
    cpu_vals[binding.need_col.raw()] = need;

    let mut gpu_reg = EmlExpressionRegistry::new();
    let gpu_ops = build_need_binding_ops(std::slice::from_ref(&gpu_binding), &mut gpu_reg);
    let upload_rows: Vec<(EmlTreeId, EmlFormulaMeta, Vec<EmlNodeGpu>)> = gpu_reg
        .formulas_for_gpu_upload()
        .map(|(id, meta, nodes)| {
            (
                id,
                meta.clone(),
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
                    .collect(),
            )
        })
        .collect();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 4);
    let mapping = table
        .upload_trees(ctx, &upload_rows)
        .expect("upload need trees");
    for (id, idx) in mapping {
        gpu_reg
            .mark_tree_uploaded(id, idx, table.generation)
            .expect("mark");
    }
    let upload = PackedAccumulatorUpload::from_ops_with_eml(&gpu_ops, Some(&gpu_reg)).expect("pack");
    let mut session = AccumulatorOpSession::new_attached(ctx, n_slots as u32, n_cols as u32, 8);
    session.upload_values(ctx, &initial);
    session.copy_values_to_previous(ctx);
    session.upload_packed_ops(ctx, &upload).expect("ops");
    session
        .tick_with_eml(ctx, 0, Some(&table))
        .expect("gpu stage");
    session.copy_values_to_previous(ctx);
    session
        .tick_with_eml(ctx, 1, Some(&table))
        .expect("gpu eval");
    let gpu_vals = session.readback_full(ctx).expect("readback");
    bits_eq(&cpu_vals, &gpu_vals)
}

fn case_flux_choke(ctx: &GpuContext, plant_defect: bool) -> bool {
    let (wn, ws, we, ww) = StructuredFieldStencilConfig::zero_directional_weights();
    let config = StructuredFieldStencilConfig {
        width: 4,
        height: 4,
        n_dims: 2,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: wn,
        weight_south: ws,
        weight_east: we,
        weight_west: ww,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let mut values = vec![0.0f32; config.values_len()];
    values[10] = 0.8;
    let params = params_from_config(&config);
    let cpu = cpu_horizon(&values, &params, 1);
    let mut gpu_config = config.clone();
    if plant_defect {
        // Stay inside CFL (chi <= 0.25); corrupt u_sat so GPU diverges from CPU.
        gpu_config.operator = StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 0.25,
            chi: 0.25,
            choke_output_col: Some(1),
        };
    }
    let op = StructuredFieldStencilOp::new(ctx, gpu_config).expect("stencil");
    op.upload_values(ctx, &values).expect("upload");
    let (gpu, _) = op.run_configured_horizon(ctx).expect("run");
    bits_eq(&cpu, &gpu)
}

fn case_passes(ctx: &GpuContext, case: ParityCase, plant_defect: bool) -> bool {
    match case {
        ParityCase::Mobility => case_mobility(ctx, plant_defect),
        ParityCase::EmlEval => case_eml_eval(ctx, plant_defect),
        ParityCase::Accumulator => case_accumulator(ctx, plant_defect),
        ParityCase::RfNeedBinding => case_rf_need_binding(ctx, plant_defect),
        ParityCase::FluxChoke => case_flux_choke(ctx, plant_defect),
    }
}

#[test]
fn cpu_gpu_parity_matrix_cases_match() {
    let Some(ctx) = require_gpu() else {
        eprintln!("skipping cpu_gpu_parity_matrix_cases_match: no GPU");
        return;
    };
    for case in CASES {
        assert!(
            case_passes(&ctx, case, false),
            "parity case {case:?} must match"
        );
    }
}

#[test]
fn cpu_gpu_parity_matrix_planted_defects_fail() {
    let Some(ctx) = require_gpu() else {
        eprintln!("skipping cpu_gpu_parity_matrix_planted_defects_fail: no GPU");
        return;
    };
    for case in CASES {
        assert!(
            !case_passes(&ctx, case, true),
            "parity case {case:?} must FAIL under planted defect"
        );
    }
    // Named accumulator subpath defects (Remand 4): each absorbed family must bite.
    assert!(!accumulator_transfer(&ctx, true), "transfer defect");
    assert!(!accumulator_emission(&ctx, true), "emission defect");
    assert!(!accumulator_intent(&ctx, true), "intent defect");
    assert!(!accumulator_velocity(&ctx, true), "velocity defect");
    assert!(!accumulator_weighted_mean(&ctx, true), "weighted-mean defect");
    assert!(!accumulator_owner_silo(&ctx, true), "owner-silo defect");
    assert!(!accumulator_bh2_w(&ctx, true), "bh2-w defect");
}
