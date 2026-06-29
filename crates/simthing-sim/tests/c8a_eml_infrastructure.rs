//! C-8a EML infrastructure — registry, GPU program table, EvalEML interpreter parity.

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerKind, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    eval_eml_cpu, set_debug_readback_allowed, AccumulatorOpGpu, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext, PackedAccumulatorUpload,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
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
    let mut trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    trees.sort_by_key(|(id, _, _)| id.0);
    let mapping = table.upload_trees(ctx, &trees).unwrap();
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .unwrap();
    }
}

fn eval_eml_op(tree_id: u32, eval_slot: u32, target_slot: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: eval_slot,
            col: 0,
        },
        combine: CombineFn::EvalEML { tree_id },
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, 0)],
    }
}

fn run_eval_eml_gpu(
    ctx: &GpuContext,
    session: &mut AccumulatorOpSession,
    registry: &EmlExpressionRegistry,
    table: &EmlGpuProgramTable,
    op: &AccumulatorOp,
    values: &[f32],
) -> Vec<f32> {
    set_debug_readback_allowed(true);
    session.upload_values(ctx, values);
    session
        .upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_ops_with_eml(std::slice::from_ref(op), Some(registry))
                .unwrap(),
        )
        .unwrap();
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session.tick_with_eml(ctx, 0, eml).unwrap();
    session.readback_full(ctx).unwrap()
}

#[test]
fn c8a_eml_execution_class_validation_allows_exact_in_transfer() {
    let mut registry = EmlExpressionRegistry::new();
    let id = EmlTreeId(1);
    registry
        .register_formula(
            id,
            exact_meta(1, "xfer"),
            vec![
                literal(1.0),
                literal(2.0),
                EmlNodeGpu {
                    opcode: eml_opcode::ADD,
                    flags: 0,
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
            ],
        )
        .unwrap();
    assert!(registry
        .assert_consumer_admissible(id, EmlConsumerKind::TransferConservation)
        .is_ok());
}

#[test]
fn c8a_fast_approx_formula_rejected_from_hard_threshold() {
    let mut registry = EmlExpressionRegistry::new();
    let id = EmlTreeId(2);
    let mut meta = exact_meta(2, "fast");
    meta.execution_class = EmlExecutionClass::FastApproximate;
    meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
    registry
        .register_formula(id, meta, vec![literal(1.0)])
        .unwrap();
    assert!(registry
        .assert_consumer_admissible(id, EmlConsumerKind::HardThreshold)
        .is_err());
}

#[test]
fn c8a_eml_tree_table_upload_roundtrip() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    let mut registry = EmlExpressionRegistry::new();
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "lit"),
        vec![
            literal(42.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    assert_eq!(registry.tree_range_index(EmlTreeId(1)), Some(0));
    assert_eq!(table.range_used, 1);
}

#[test]
fn c8a_eval_eml_exact_gpu_matches_cpu_oracle_bit_exact() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let nodes = vec![
        literal(2.0),
        literal(3.0),
        EmlNodeGpu {
            opcode: eml_opcode::ADD,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let cpu = eval_eml_cpu(&nodes, 0, &[0.0; 4], 1, [0.0; 4]);
    assert_eq!(cpu.to_bits(), 5.0f32.to_bits());

    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "add2_3"),
        nodes,
    );

    let mut session = AccumulatorOpSession::new(&ctx, 4, 1);
    let op = eval_eml_op(1, 0, 1);
    let gpu_values = run_eval_eml_gpu(&ctx, &mut session, &registry, &table, &op, &[0.0; 4]);
    assert_eq!(gpu_values[1].to_bits(), cpu.to_bits());
}

#[test]
fn c8a_eval_eml_slot_mul_literal_bit_exact() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        literal(2.0),
        EmlNodeGpu {
            opcode: eml_opcode::MUL,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let values = [3.0, 0.0, 0.0, 0.0];
    let cpu = eval_eml_cpu(&nodes, 0, &values, 1, [0.0; 4]);

    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "slot_mul"),
        nodes,
    );

    let mut session = AccumulatorOpSession::new(&ctx, 4, 1);
    let gpu_values = run_eval_eml_gpu(
        &ctx,
        &mut session,
        &registry,
        &table,
        &eval_eml_op(1, 0, 1),
        &values,
    );
    assert_eq!(gpu_values[1].to_bits(), cpu.to_bits());
    assert_eq!(gpu_values[1].to_bits(), 6.0f32.to_bits());
}

#[test]
fn c8a_eval_eml_clamp_min_max_select_bit_exact() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    // clamp(slot, 0, 1) on amount=0.75
    let clamp_nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        EmlNodeGpu {
            opcode: eml_opcode::CLAMP_BOUNDED,
            flags: 0,
            a: 0.0f32.to_bits(),
            b: 1.0f32.to_bits(),
            c: 0,
            d: 0,
        },
    ];
    let clamp_values = [0.75, 0.0, 0.0, 0.0];
    let clamp_cpu = eval_eml_cpu(&clamp_nodes, 0, &clamp_values, 1, [0.0; 4]);

    // select(amount > 0.5, 1, 0)
    let select_nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::SLOT_VALUE,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        literal(0.5),
        EmlNodeGpu {
            opcode: eml_opcode::CMP_GT,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        literal(1.0),
        literal(0.0),
        EmlNodeGpu {
            opcode: eml_opcode::SELECT,
            flags: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
    ];
    let select_cpu = eval_eml_cpu(&select_nodes, 0, &clamp_values, 1, [0.0; 4]);

    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "clamp"),
        clamp_nodes,
    );
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        2,
        exact_meta(2, "select"),
        select_nodes,
    );

    let mut session = AccumulatorOpSession::new(&ctx, 4, 1);
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session.upload_values(&ctx, &clamp_values);

    let ops = vec![eval_eml_op(1, 0, 1), eval_eml_op(2, 0, 2)];
    let gpu_ops = AccumulatorOpGpu::encode_bootstrap_set_with_eml(&ops, Some(&registry)).unwrap();
    session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_gpu_ops(gpu_ops.to_vec()).unwrap(),
        )
        .unwrap();
    session.tick_with_eml(&ctx, 0, eml).unwrap();
    let gpu = session.readback_full(&ctx).unwrap();

    assert_eq!(gpu[1].to_bits(), clamp_cpu.to_bits());
    assert_eq!(gpu[2].to_bits(), select_cpu.to_bits());
}

#[test]
fn c8a_multiple_eml_trees_one_dispatch() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "five"),
        vec![
            literal(5.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        2,
        exact_meta(2, "seven"),
        vec![
            literal(7.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    assert_eq!(registry.tree_range_index(EmlTreeId(1)), Some(0));
    assert_eq!(registry.tree_range_index(EmlTreeId(2)), Some(1));

    let mut session = AccumulatorOpSession::new(&ctx, 4, 1);
    let eml = Some((&table.node_buffer, &table.range_buffer));
    let ops = vec![eval_eml_op(1, 0, 1), eval_eml_op(2, 0, 2)];
    let gpu_ops = AccumulatorOpGpu::encode_bootstrap_set_with_eml(&ops, Some(&registry)).unwrap();
    assert_eq!(gpu_ops[0].combine_a, 0);
    assert_eq!(gpu_ops[1].combine_a, 1);
    session.upload_values(&ctx, &[0.0; 4]);
    session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_gpu_ops(gpu_ops.to_vec()).unwrap(),
        )
        .unwrap();
    session.tick_with_eml(&ctx, 0, eml).unwrap();
    let gpu = session.readback_full(&ctx).unwrap();
    assert_eq!(gpu[1].to_bits(), 5.0f32.to_bits());
    assert_eq!(gpu[2].to_bits(), 7.0f32.to_bits());
}

#[test]
fn c8a_tree_generation_reupload_invalidates_ops() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "v1"),
        vec![
            literal(1.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    let gen1 = table.generation;
    let trees = vec![(
        EmlTreeId(1),
        registry.get(EmlTreeId(1)).unwrap().clone(),
        vec![
            literal(9.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    )];
    table.upload_trees(&ctx, &trees).unwrap();
    registry
        .mark_tree_uploaded(EmlTreeId(1), 0, table.generation)
        .unwrap();
    assert!(table.generation > gen1);
    assert_eq!(table.node_upload_count, 2);
}

#[test]
fn c8a_node_buffer_capacity_growth_preserves_existing_trees() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut table = EmlGpuProgramTable::new(&ctx, 4, 2);
    let mut registry = EmlExpressionRegistry::new();
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "a"),
        vec![
            literal(1.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        2,
        exact_meta(2, "b"),
        vec![
            literal(2.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    assert!(table.node_capacity >= 4);
    assert_eq!(table.range_used, 2);
}

#[test]
fn c8a_persistent_node_buffer_no_per_dispatch_upload() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 8);
    register_and_upload(
        &ctx,
        &mut registry,
        &mut table,
        1,
        exact_meta(1, "five"),
        vec![
            literal(5.0),
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ],
    );
    let uploads_after_register = table.node_upload_count;

    let mut session = AccumulatorOpSession::new(&ctx, 4, 1);
    let eml = Some((&table.node_buffer, &table.range_buffer));
    let op = eval_eml_op(1, 0, 1);
    session.upload_values(&ctx, &[0.0; 4]);
    session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_ops_with_eml(std::slice::from_ref(&op), Some(&registry))
                .unwrap(),
        )
        .unwrap();

    for _ in 0..3 {
        session.tick_with_eml(&ctx, 0, eml).unwrap();
    }
    assert_eq!(table.node_upload_count, uploads_after_register);
    assert_eq!(table.range_upload_count, uploads_after_register);
}
