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

