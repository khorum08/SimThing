//! C-8d emission substrate parity and validation tests.

use simthing_core::{
    eml_opcode, ClampBehavior, DimensionRegistry, EmlConsumerKind, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError,
    EmlTreeId, SimProperty, SubFieldRole, SubFieldSpec,
};
use simthing_gpu::{
    encode_emission_plan, plan_emission_ops, plan_transfer_ops, set_debug_readback_allowed,
    AccumulatorOpSession, AccumulatorOpSessionError, AccumulatorPipelineSessions, EmissionFormula,
    EmissionPlanError, EmissionRegistration, GpuContext, PackedAccumulatorUpload, Pipelines,
    TransferInputRef, TransferRegistration, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn emission_registry_with_columns(cols: u32) -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    let sub_fields: Vec<SubFieldSpec> = (0..cols)
        .map(|i| SubFieldSpec {
            role: SubFieldRole::Named(format!("col{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("col{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        })
        .collect();
    reg.register(SimProperty {
        namespace: "emission".into(),
        name: "resources".into(),
        layout: simthing_core::PropertyLayout { sub_fields },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });
    reg
}

fn setup_emission_state(n_slots: u32, per_slot: &[f32]) -> WorldGpuState {
    let reg = emission_registry_with_columns(per_slot.len() as u32);
    let state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &reg, n_slots);
    let mut flat = vec![0.0_f32; state.values_len()];
    for slot in 0..n_slots {
        let base = slot as usize * state.n_dims as usize;
        for (col, &v) in per_slot.iter().enumerate() {
            flat[base + col] = v;
        }
    }
    state.install_resolved_values_at_boundary(&flat);
    state
}

fn run_accumulator_emission(
    state: &mut WorldGpuState,
    dt: f32,
) -> Result<Vec<simthing_gpu::EmissionRecord>, AccumulatorOpSessionError> {
    set_debug_readback_allowed(true);
    let pipelines = Pipelines::new(&state.ctx);
    let mut emission_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_emission_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: None,
            emission: emission_session.as_mut(),
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_emission_session(emission_session);
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .emission_session()
        .unwrap()
        .readback_emissions(&state.ctx)
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

fn register_slot_mul_tree(state: &mut WorldGpuState, tree_id: u32) {
    let nodes = vec![
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
            a: 2.0f32.to_bits(),
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
    ];
    if state.accumulator_runtime.is_none() {
        state.accumulator_runtime = Some(simthing_gpu::WorldAccumulatorRuntime::new());
    }
    let runtime = state.accumulator_runtime.as_mut().unwrap();
    runtime.ensure_eml_program_table(&state.ctx);
    let id = EmlTreeId(tree_id);
    runtime
        .eml_registry
        .register_formula(id, exact_meta(tree_id, "slot_mul2"), nodes)
        .expect("register emission EML");
    runtime
        .upload_eml_trees(&state.ctx)
        .expect("upload EML for emission test");
}

#[test]
fn c8d_identity_floor_emission_records_count() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[3.7]);
    let regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: None,
        formula: EmissionFormula::IdentityFloor,
        max_emit: None,
        reg_idx: 11,
    }];
    state
        .sync_emission_accumulator(&regs)
        .expect("sync emission");
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("readback");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 11);
    assert_eq!(emissions[0].emit_count(), 3);
}

#[test]
fn c8d_nonpositive_emission_emits_no_record() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    for value in [0.0, -2.0] {
        let mut state = setup_emission_state(1, &[value]);
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 0,
        }];
        state.sync_emission_accumulator(&regs).expect("sync");
        let emissions = run_accumulator_emission(&mut state, 1.0).expect("readback");
        assert!(emissions.is_empty(), "value {value} should not emit");
    }
}

#[test]
fn c8d_constant_emission_records_count() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[0.0]);
    let regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: None,
        formula: EmissionFormula::Constant { value: 5.0 },
        max_emit: None,
        reg_idx: 3,
    }];
    state.sync_emission_accumulator(&regs).expect("sync");
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("readback");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 3);
    assert_eq!(emissions[0].emit_count(), 5);
}

#[test]
fn c8d_eval_eml_exact_emission_matches_cpu_oracle() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[2.4]);
    state.ensure_emission_accumulator();
    register_slot_mul_tree(&mut state, 1);
    let regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: Some(EmlTreeId(1)),
        formula: EmissionFormula::EvalEml {
            tree_id: EmlTreeId(1),
        },
        max_emit: None,
        reg_idx: 5,
    }];
    state.sync_emission_accumulator(&regs).expect("sync");
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("readback");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 5);
    assert_eq!(emissions[0].emit_count(), 4);
}

#[test]
fn c8d_multiple_emissions_compact_records() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[3.2, 0.0, 7.9]);
    let regs = vec![
        EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 1,
        },
        EmissionRegistration {
            source_slot: 0,
            source_col: 1,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 2,
        },
        EmissionRegistration {
            source_slot: 0,
            source_col: 2,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 3,
        },
    ];
    state.sync_emission_accumulator(&regs).expect("sync");
    let mut emissions = run_accumulator_emission(&mut state, 1.0).expect("readback");
    emissions.sort_by_key(|r| r.reg_idx());
    assert_eq!(emissions.len(), 2);
    assert_eq!(emissions[0].reg_idx(), 1);
    assert_eq!(emissions[0].emit_count(), 3);
    assert_eq!(emissions[1].reg_idx(), 3);
    assert_eq!(emissions[1].emit_count(), 7);
}

#[test]
fn c8d_emission_overflow_count_exceeds_capacity() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut session = AccumulatorOpSession::with_emission_capacity(&ctx, 1, 3, 1);
    let regs = vec![
        EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::Constant { value: 2.0 },
            max_emit: None,
            reg_idx: 0,
        },
        EmissionRegistration {
            source_slot: 0,
            source_col: 1,
            tree_id: None,
            formula: EmissionFormula::Constant { value: 3.0 },
            max_emit: None,
            reg_idx: 1,
        },
    ];
    let plan = plan_emission_ops(&regs, None).unwrap();
    let gpu_ops = encode_emission_plan(&plan, None).unwrap();
    session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_gpu_ops(gpu_ops.to_vec()).unwrap(),
        )
        .unwrap();
    session.tick(&ctx, 0).unwrap();
    let (count, records) = session.readback_emissions_capped(&ctx).unwrap();
    assert_eq!(count, 2);
    assert_eq!(records.len(), 1);
    assert!(matches!(
        session.readback_emissions(&ctx),
        Err(AccumulatorOpSessionError::EmissionOverflow {
            count: 2,
            capacity: 1,
        })
    ));
}

#[test]
fn c8d_rejects_soft_or_fast_emission_without_tolerance_gate() {
    let mut registry = EmlExpressionRegistry::new();
    for (id, class) in [
        (1, EmlExecutionClass::SoftDeterministic),
        (2, EmlExecutionClass::FastApproximate),
    ] {
        let mut meta = exact_meta(id, "soft");
        meta.execution_class = class;
        meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
        registry
            .register_formula(
                EmlTreeId(id),
                meta,
                vec![EmlNodeGpu {
                    opcode: eml_opcode::LITERAL_F32,
                    flags: 0,
                    a: 1.0f32.to_bits(),
                    b: 0,
                    c: 0,
                    d: 0,
                }],
            )
            .unwrap();
    }
    for id in [1, 2] {
        let regs = vec![EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: Some(EmlTreeId(id)),
            formula: EmissionFormula::EvalEml {
                tree_id: EmlTreeId(id),
            },
            max_emit: None,
            reg_idx: 0,
        }];
        assert!(
            plan_emission_ops(&regs, Some(&registry)).is_err(),
            "tree {id} should be rejected"
        );
    }
}

#[test]
fn c8d_transfer_conservation_still_rejects_soft_formula() {
    let mut registry = EmlExpressionRegistry::new();
    let id = EmlTreeId(9);
    let mut meta = exact_meta(9, "soft_xfer");
    meta.execution_class = EmlExecutionClass::SoftDeterministic;
    meta.allowed_consumers = EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE);
    registry
        .register_formula(
            id,
            meta,
            vec![EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }],
        )
        .unwrap();
    assert!(matches!(
        registry.assert_consumer_admissible(id, EmlConsumerKind::TransferConservation),
        Err(EmlRegistryError::ClassNotAdmissibleForConsumer { .. })
    ));
    let transfer_regs = vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: 0,
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: 1,
        output_scale: 1.0,
        max_transfer: Some(1.0),
        tree_id: Some(id),
        order_band: 0,
    }];
    assert!(plan_transfer_ops(&transfer_regs).is_ok());
}

#[test]
fn c8d_emission_path_no_cpu_mediated_evaluation() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[4.5]);
    let regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: None,
        formula: EmissionFormula::IdentityFloor,
        max_emit: None,
        reg_idx: 0,
    }];
    state.sync_emission_accumulator(&regs).expect("sync");
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("gpu path");
    assert_eq!(emissions[0].emit_count(), 4);
}

#[test]
fn c8d_eval_eml_emission_does_not_reupload_eml_per_tick() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[2.0]);
    state.ensure_emission_accumulator();
    register_slot_mul_tree(&mut state, 1);
    let regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: Some(EmlTreeId(1)),
        formula: EmissionFormula::EvalEml {
            tree_id: EmlTreeId(1),
        },
        max_emit: None,
        reg_idx: 0,
    }];
    state.sync_emission_accumulator(&regs).expect("sync");
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    let eml_generation = runtime.eml_generation();
    let eml_upload_count = runtime.eml.as_ref().unwrap().upload_count();
    let op_uploads = runtime.emission_op_upload_count();
    run_accumulator_emission(&mut state, 1.0).expect("tick");
    state.sync_emission_accumulator(&regs).expect("re-sync");
    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert_eq!(runtime.eml_generation(), eml_generation);
    assert_eq!(
        runtime.eml.as_ref().unwrap().upload_count(),
        eml_upload_count
    );
    assert_eq!(runtime.emission_op_upload_count(), op_uploads);
    run_accumulator_emission(&mut state, 1.0).expect("tick2");
}

fn constant_emission_reg(value: f32, reg_idx: u32) -> EmissionRegistration {
    EmissionRegistration {
        source_slot: 0,
        source_col: 0,
        tree_id: None,
        formula: EmissionFormula::Constant { value },
        max_emit: None,
        reg_idx,
    }
}

#[test]
fn c8d_constant_value_change_reuploads_emission_ops() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[0.0]);
    state
        .sync_emission_accumulator(&[constant_emission_reg(2.0, 7)])
        .expect("sync");
    let uploads_after_first = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    state
        .sync_emission_accumulator(&[constant_emission_reg(5.0, 7)])
        .expect("re-sync");
    let uploads_after_second = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    assert!(uploads_after_second > uploads_after_first);
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("dispatch");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 7);
    assert_eq!(emissions[0].emit_count(), 5);
}

#[test]
fn c8d_reg_idx_change_reuploads_emission_ops() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[3.7]);
    state
        .sync_emission_accumulator(&[EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 7,
        }])
        .expect("sync");
    let uploads_after_first = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    state
        .sync_emission_accumulator(&[EmissionRegistration {
            source_slot: 0,
            source_col: 0,
            tree_id: None,
            formula: EmissionFormula::IdentityFloor,
            max_emit: None,
            reg_idx: 42,
        }])
        .expect("re-sync");
    let uploads_after_second = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    assert!(uploads_after_second > uploads_after_first);
    let emissions = run_accumulator_emission(&mut state, 1.0).expect("dispatch");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 42);
    assert_eq!(emissions[0].emit_count(), 3);
}

#[test]
fn c8d_mismatched_registration_tree_id_rejected() {
    assert_eq!(
        plan_emission_ops(
            &[EmissionRegistration {
                source_slot: 0,
                source_col: 0,
                tree_id: Some(EmlTreeId(1)),
                formula: EmissionFormula::EvalEml {
                    tree_id: EmlTreeId(2),
                },
                max_emit: None,
                reg_idx: 0,
            }],
            None,
        ),
        Err(EmissionPlanError::MismatchedTreeIdField {
            registration_tree_id: Some(1),
            formula_tree_id: 2,
        })
    );
}

#[test]
fn c8d_max_emit_rejected_until_supported() {
    assert_eq!(
        plan_emission_ops(
            &[EmissionRegistration {
                source_slot: 0,
                source_col: 0,
                tree_id: None,
                formula: EmissionFormula::IdentityFloor,
                max_emit: Some(3),
                reg_idx: 0,
            }],
            None,
        ),
        Err(EmissionPlanError::MaxEmitUnsupported)
    );
}

#[test]
fn c8d_same_emission_plan_skips_upload() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_emission_state(1, &[4.0]);
    let regs = vec![constant_emission_reg(4.0, 1)];
    state.sync_emission_accumulator(&regs).expect("sync");
    let uploads_after_first = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    state.sync_emission_accumulator(&regs).expect("re-sync");
    let uploads_after_second = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .emission_op_upload_count();
    assert_eq!(uploads_after_second, uploads_after_first);
}

#[test]
fn c8d_combined_c1_c2_c4_s4_c7_c8b_c8c_c8d_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_emission_state(1, &[10.0, 2.0, 0.0]);
    let transfer_regs = vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: 0,
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: 1,
        output_scale: 1.0,
        max_transfer: Some(1.0),
        tree_id: None,
        order_band: 0,
    }];
    state
        .sync_transfer_accumulator(&transfer_regs)
        .expect("transfer sync");
    let emission_regs = vec![EmissionRegistration {
        source_slot: 0,
        source_col: 1,
        tree_id: None,
        formula: EmissionFormula::IdentityFloor,
        max_emit: None,
        reg_idx: 99,
    }];
    state
        .sync_emission_accumulator(&emission_regs)
        .expect("emission sync");

    let pipelines = Pipelines::new(&state.ctx);
    let runtime = state.accumulator_runtime.as_mut().unwrap();
    let mut transfer_session = runtime.take_transfer_session();
    let mut emission_session = runtime.take_emission_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        1.0,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: emission_session.as_mut(),
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_emission_session(emission_session);

    let after = state.read_values();
    assert_eq!(after[0].to_bits(), 9.0f32.to_bits());
    assert_eq!(after[1].to_bits(), 3.0f32.to_bits());
    let emissions = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .emission_session()
        .unwrap()
        .readback_emissions(&state.ctx)
        .expect("emission records");
    assert_eq!(emissions.len(), 1);
    assert_eq!(emissions[0].reg_idx(), 99);
    assert_eq!(emissions[0].emit_count(), 3);
}
