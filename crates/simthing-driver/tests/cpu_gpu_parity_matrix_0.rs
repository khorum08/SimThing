//! TP-PURGE-0 Remand 3 — `cpu_gpu_parity_matrix` (DA `5135942768` / Remand `5136003644`,
//! continuation `5136490881`).
//!
//! Five approved cases. Inline input. Each case: live kernel + CPU reference + GPU path
//! + planted defect. With `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`, every case must execute.
//!
//! `rf-need-binding` exercises the live `need_binding` Identity-stage + EvalEML path
//! even with zero antecedent mapped rows among the 218 (DA substrate coverage).

use bytemuck::cast_slice;
use simthing_core::{
    eml_nodes, eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SimThingId,
    SlotIndex, SourceSpec, SubFieldRole,
};
use simthing_driver::need_binding::{
    build_need_binding_ops, ResolvedFullCell, ResolvedNeedBinding,
};
use simthing_gpu::{
    cpu_horizon, cpu_scatter_indexed, cpu_w_impedance_compose_oracle, encode_column, eval_eml_cpu,
    execute_ops_cpu, params_from_config, set_debug_readback_allowed, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext, IndexedScatterOp, PackedAccumulatorUpload, ScatterEntry,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig, StructuredFieldStencilMaskMode,
    StructuredFieldStencilOp, StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
    WImpedanceComposeConfig, WImpedanceComposeOp, WImpedanceComposeProfile,
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

fn case_accumulator(ctx: &GpuContext, plant_defect: bool) -> bool {
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
}
