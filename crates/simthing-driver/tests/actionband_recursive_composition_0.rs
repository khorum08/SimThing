use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId, SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, ActionBandActiveInstance, ActionBandExecutionCompileError,
};
use simthing_gpu::{
    cpu_oracle_band_crossing_deltas, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, ActionBandEmissionBindingGpu, ActionBandExecutionError,
    ActionBandGpuExecution, ActionBandStateGpu, GpuContext, SlotAllocator, ACTIONBAND_STATE_ACTIVE,
    ACTIONBAND_STATE_TERMINAL,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandRequirementSemantics, ActionBandSessionBuildDoor,
    ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec, ActionBandVelocitySpec,
    FrozenActionBandTemplates, ScalarBoundDirection,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct Fixture {
    registry: DimensionRegistry,
    thresholds: Vec<EmitOnThresholdRegistration>,
    state: ColumnIndex,
    rf_claim: ColumnIndex,
    cost_band: ColumnIndex,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut register = |name: &str| {
        let property = registry.register(SimProperty::simple("actionband-proof", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .expect("amount column")
    };
    let state = register("ordinary-state");
    let rf_claim = register("native-rf-claim");
    let cost_band = register("native-scalar-costband");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: state,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 703,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    Fixture {
        registry,
        thresholds,
        state,
        rf_claim,
        cost_band,
    }
}

fn template(id: &str, column: u32, children: &[&str]) -> ActionBandTemplateSpec {
    ActionBandTemplateSpec {
        id: id.into(),
        label: Some(format!("presentation-only-{id}")),
        axis_channels: vec![ActionBandChannelBindingSpec {
            column,
            kind: ActionBandChannelKind::Primitive,
        }],
        target: ActionBandTargetSpec::ScalarBound {
            channel: column,
            bound: 1.0,
            direction: ScalarBoundDirection::AtLeast,
        },
        velocity: None,
        bands: vec![ActionBandBandSpec {
            threshold_registration_index: 0,
            eml_program: None,
            emission_binding_indices: vec![0],
        }],
        subordinate_template_ids: children.iter().map(|child| (*child).into()).collect(),
        max_active_subordinates: children.len() as u32,
        reserved_instance_rows: 1,
        requirement_semantics: ActionBandRequirementSemantics::Ordinary,
    }
}

fn recursive_spec(fixture: &Fixture, reverse_children: bool) -> ActionBandSessionSpec {
    let children = if reverse_children {
        vec!["child-b", "child-a"]
    } else {
        vec!["child-a", "child-b"]
    };
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 2,
            storage_rows: 3,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![
            template("parent", fixture.state.raw_u32(), &children),
            template("child-a", fixture.state.raw_u32(), &[]),
            template("child-b", fixture.state.raw_u32(), &[]),
        ],
    }
}

fn admit(
    fixture: &Fixture,
    spec: &ActionBandSessionSpec,
    eml: &EmlExpressionRegistry,
) -> FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(spec, &fixture.registry, eml, &fixture.thresholds)
        .expect("frozen 7.1 admission")
        .clone()
}

fn recursive_instances(frozen: &FrozenActionBandTemplates) -> Vec<ActionBandActiveInstance> {
    vec![
        ActionBandActiveInstance::new(frozen.templates()[0].index(), SlotIndex::new(0), [0.0; 4]),
        ActionBandActiveInstance::pre_admitted_subordinate(
            frozen.templates()[1].index(),
            SlotIndex::new(0),
            [0.0; 4],
        ),
        ActionBandActiveInstance::pre_admitted_subordinate(
            frozen.templates()[2].index(),
            SlotIndex::new(0),
            [0.0; 4],
        ),
    ]
}

fn binding() -> ActionBandEmissionBindingGpu {
    ActionBandEmissionBindingGpu::structural_request(0)
}

fn world(fixture: &Fixture, state: f32, rf_claim: f32, cost_band: f32) -> Vec<f32> {
    let mut values = vec![0.0; fixture.registry.total_columns];
    values[fixture.state.raw()] = state;
    values[fixture.rf_claim.raw()] = rf_claim;
    values[fixture.cost_band.raw()] = cost_band;
    values
}

fn sealed_delta(fixture: &Fixture) -> simthing_gpu::BandCrossingDelta {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    cpu_oracle_band_crossing_deltas(
        &world(fixture, 0.5, 2.0, 3.0),
        &world(fixture, 1.5, 2.0, 3.0),
        &[],
        &[],
        fixture.registry.total_columns as u32,
        &emit_on_threshold_registrations_to_gpu(&fixture.thresholds),
        &fixture.registry,
        &allocator,
    )
    .into_iter()
    .next()
    .expect("existing sealed crossing")
}

#[test]
fn parent_activates_children_next_then_resolves_after_later_collapse() {
    let fixture = fixture();
    let eml = EmlExpressionRegistry::new();
    let frozen = admit(&fixture, &recursive_spec(&fixture, false), &eml);
    let plan = compile_action_band_gpu_execution(
        &frozen,
        &eml,
        &[binding()],
        &recursive_instances(&frozen),
    )
    .expect("bounded recursive lowering")
    .into_execution_plan();
    assert!(plan.uses_depth1_crossing_fast_path());
    assert!(plan.uses_depth2_common_fast_shape());
    assert_eq!(plan.dependency_row_count(), 2);

    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let crossings = plan
        .crossings_from_sealed(&[sealed_delta(&fixture)])
        .expect("one sealed crossing joins the pre-admitted rows");
    let values = world(&fixture, 1.5, 2.0, 3.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_recursive_world"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut session = match ActionBandGpuExecution::new(&ctx, plan).expect("GPU session") {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("three materialized sparse rows"),
    };
    let _proof = scoped_debug_readback_allowed(true);

    let activate = session
        .dispatch_and_readback(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &crossings,
        )
        .expect("generation t activates child-next");
    assert!(activate.commitments.is_empty());
    assert_eq!(activate.states[0].generation, 1);
    assert_eq!(activate.states[1].generation, 0);
    assert_eq!(activate.states[2].generation, 0);
    assert_eq!(activate.states[1].reserved[0], ACTIONBAND_STATE_ACTIVE);
    assert_eq!(activate.states[2].reserved[0], ACTIONBAND_STATE_ACTIVE);

    let children = session
        .dispatch_and_readback(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &crossings,
        )
        .expect("generation t+1 executes concurrent children");
    assert_eq!(children.commitments.len(), 2);
    assert_eq!(children.states[0].generation, 2);
    for state in &children.states[1..] {
        assert_eq!(state.generation, 1);
        assert_eq!(state.satisfied, 1);
        assert_eq!(state.reserved[0], ACTIONBAND_STATE_TERMINAL);
    }

    let resolved = session
        .dispatch_and_readback(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &crossings,
        )
        .expect("generation t+2 observes collapsed children");
    assert_eq!(resolved.commitments.len(), 1);
    assert_eq!(resolved.states[0].generation, 3);
    assert_eq!(resolved.states[0].satisfied, 1);
    assert_eq!(resolved.states[0].reserved[0], ACTIONBAND_STATE_ACTIVE);
    assert_eq!(resolved.states[1], children.states[1]);
    assert_eq!(resolved.states[2], children.states[2]);
}

#[test]
fn sibling_and_instance_append_perturbations_compile_bit_identically() {
    let fixture = fixture();
    let eml = EmlExpressionRegistry::new();
    let first = admit(&fixture, &recursive_spec(&fixture, false), &eml);
    let reversed = admit(&fixture, &recursive_spec(&fixture, true), &eml);
    let first_instances = recursive_instances(&first);
    let mut reversed_instances = recursive_instances(&reversed);
    reversed_instances.reverse();
    let first_plan =
        compile_action_band_gpu_execution(&first, &eml, &[binding()], &first_instances)
            .unwrap()
            .into_execution_plan();
    let reversed_plan =
        compile_action_band_gpu_execution(&reversed, &eml, &[binding()], &reversed_instances)
            .unwrap()
            .into_execution_plan();
    assert_eq!(
        first_plan.numeric_fingerprint(),
        reversed_plan.numeric_fingerprint()
    );
    assert_eq!(first_plan.dependency_row_count(), 2);
    assert_eq!(reversed_plan.dependency_row_count(), 2);

    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let delta = sealed_delta(&fixture);
    let first_crossings = first_plan.crossings_from_sealed(&[delta.clone()]).unwrap();
    let reversed_crossings = reversed_plan.crossings_from_sealed(&[delta]).unwrap();
    let values = world(&fixture, 1.5, 2.0, 3.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_append_perturbation_world"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut first_session = match ActionBandGpuExecution::new(&ctx, first_plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("first recursive plan is active"),
    };
    let mut reversed_session = match ActionBandGpuExecution::new(&ctx, reversed_plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("reversed recursive plan is active"),
    };
    let _proof = scoped_debug_readback_allowed(true);
    for _ in 0..3 {
        let first = first_session
            .dispatch_and_readback(
                &ctx,
                &values,
                fixture.registry.total_columns as u32,
                &first_crossings,
            )
            .unwrap();
        let reversed = reversed_session
            .dispatch_and_readback(
                &ctx,
                &values,
                fixture.registry.total_columns as u32,
                &reversed_crossings,
            )
            .unwrap();
        assert_eq!(
            bytemuck::cast_slice::<ActionBandStateGpu, u8>(&first.states),
            bytemuck::cast_slice::<ActionBandStateGpu, u8>(&reversed.states)
        );
        assert_eq!(
            bytemuck::cast_slice::<f32, u8>(&first.projection),
            bytemuck::cast_slice::<f32, u8>(&reversed.projection)
        );
        assert_eq!(first.emission_payloads, reversed.emission_payloads);
        assert_eq!(first.commitments.len(), reversed.commitments.len());
    }
}

#[test]
fn runtime_child_construction_shared_lifecycle_and_nonfast_recursion_are_red() {
    let fixture = fixture();
    let eml = EmlExpressionRegistry::new();
    let frozen = admit(&fixture, &recursive_spec(&fixture, false), &eml);
    let parent_only = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    assert!(matches!(
        compile_action_band_gpu_execution(&frozen, &eml, &[binding()], &parent_only),
        Err(ActionBandExecutionCompileError::MissingPreAdmittedChild { .. })
    ));

    let mut nonfast = recursive_spec(&fixture, false);
    nonfast.templates[0].velocity = Some(ActionBandVelocitySpec {
        current_channel: fixture.state.raw_u32(),
        previous_generation_channel: Some(fixture.rf_claim.raw_u32()),
    });
    nonfast.templates[0]
        .axis_channels
        .push(ActionBandChannelBindingSpec {
            column: fixture.rf_claim.raw_u32(),
            kind: ActionBandChannelKind::Primitive,
        });
    nonfast.budget.axis_channel_count = 2;
    let nonfast = admit(&fixture, &nonfast, &eml);
    assert!(matches!(
        compile_action_band_gpu_execution(
            &nonfast,
            &eml,
            &[binding()],
            &recursive_instances(&nonfast),
        ),
        Err(ActionBandExecutionCompileError::Kernel(
            ActionBandExecutionError::RecursiveShapeDeferred
        ))
    ));

    let mut shared_spec = recursive_spec(&fixture, false);
    shared_spec.templates.insert(
        1,
        template("second-parent", fixture.state.raw_u32(), &["child-a"]),
    );
    shared_spec.budget.dependency_binding_count = 3;
    shared_spec.budget.storage_rows = 4;
    let shared = admit(&fixture, &shared_spec, &eml);
    let rows = vec![
        ActionBandActiveInstance::new(shared.templates()[0].index(), SlotIndex::new(0), [0.0; 4]),
        ActionBandActiveInstance::new(shared.templates()[1].index(), SlotIndex::new(0), [0.0; 4]),
        ActionBandActiveInstance::pre_admitted_subordinate(
            shared.templates()[2].index(),
            SlotIndex::new(0),
            [0.0; 4],
        ),
        ActionBandActiveInstance::pre_admitted_subordinate(
            shared.templates()[3].index(),
            SlotIndex::new(0),
            [0.0; 4],
        ),
    ];
    assert!(matches!(
        compile_action_band_gpu_execution(&shared, &eml, &[binding()], &rows),
        Err(ActionBandExecutionCompileError::SharedChildLifecycle { .. })
    ));
}

fn multisource_eml(fixture: &Fixture) -> EmlExpressionRegistry {
    let mut eml = EmlExpressionRegistry::new();
    let slot_value = |column| EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: column,
        b: 0,
        c: 0,
        d: 0,
    };
    let multiply = EmlNodeGpu {
        opcode: eml_opcode::MUL,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    // state, RF, multiply, CostBand, multiply
    let nodes = vec![
        slot_value(fixture.state.raw_u32()),
        slot_value(fixture.rf_claim.raw_u32()),
        multiply,
        slot_value(fixture.cost_band.raw_u32()),
        multiply,
    ];
    eml.register_formula(
        EmlTreeId(0),
        EmlFormulaMeta {
            tree_id: EmlTreeId(0),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 2,
            has_loops: false,
            has_recursion: false,
            display_name: "state-rf-scalar-costband-gate".into(),
        },
        nodes,
    )
    .expect("bounded existing EML program");
    eml
}

#[test]
fn trivial_state_rf_and_scalar_costband_gate_stays_inline() {
    let fixture = fixture();
    let eml = multisource_eml(&fixture);
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 3,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            axis_channels: vec![
                ActionBandChannelBindingSpec {
                    column: fixture.state.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: fixture.rf_claim.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: fixture.cost_band.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
            ],
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(0),
                emission_binding_indices: vec![0],
            }],
            ..template("inline-gate", fixture.state.raw_u32(), &[])
        }],
    };
    let frozen = admit(&fixture, &spec, &eml);
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let plan = compile_action_band_gpu_execution(&frozen, &eml, &[binding()], &active)
        .expect("inline multisource gate")
        .into_execution_plan();
    assert_eq!(plan.dependency_row_count(), 0);
    assert!(!plan.uses_depth2_common_fast_shape());

    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let crossings = plan
        .crossings_from_sealed(&[sealed_delta(&fixture)])
        .unwrap();
    let values = world(&fixture, 1.5, 2.0, 3.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_multisource_world"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut session = match ActionBandGpuExecution::new(&ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one inline row"),
    };
    let _proof = scoped_debug_readback_allowed(true);
    let readback = session
        .dispatch_and_readback(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &crossings,
        )
        .expect("same depth-1 entry executes native column gate");
    assert_eq!(readback.emission_payloads, [9.0]);
    assert_eq!(readback.commitments.len(), 1);
}

#[test]
fn state_carry_curve_reports_increasing_active_cardinality() {
    let fixture = fixture();
    let eml = EmlExpressionRegistry::new();
    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    if !ctx.encoder_timestamp_supported() {
        return;
    }
    const WARMUP: usize = 5;
    const SAMPLES: usize = 31;
    let values = world(&fixture, 1.5, 2.0, 3.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_carry_curve_world"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE,
        });

    for rows in [1u32, 64, 1024, 4096] {
        let spec = ActionBandSessionSpec {
            budget: ActionBandAdmissionBudgetSpec {
                axis_channel_count: 1,
                dependency_binding_count: 0,
                storage_rows: rows,
                eml_program_count: 0,
                emission_binding_count: 1,
            },
            templates: vec![ActionBandTemplateSpec {
                reserved_instance_rows: rows,
                ..template("carry-curve", fixture.state.raw_u32(), &[])
            }],
        };
        let frozen = admit(&fixture, &spec, &eml);
        let active = (0..rows)
            .map(|slot| {
                ActionBandActiveInstance::new(
                    frozen.templates()[0].index(),
                    SlotIndex::new(slot),
                    [0.0; 4],
                )
            })
            .collect::<Vec<_>>();
        let plan = compile_action_band_gpu_execution(&frozen, &eml, &[binding()], &active)
            .unwrap()
            .into_execution_plan();
        let crossings = plan
            .crossings_from_sealed(&[sealed_delta(&fixture)])
            .unwrap();
        let mut session = match ActionBandGpuExecution::new(&ctx, plan).unwrap() {
            ActionBandGpuExecution::Active(session) => session,
            ActionBandGpuExecution::Inactive => panic!("carry cardinality is nonzero"),
        };
        for _ in 0..WARMUP {
            session
                .dispatch(
                    &ctx,
                    &values,
                    fixture.registry.total_columns as u32,
                    &crossings,
                )
                .unwrap();
        }
        let mut samples = Vec::with_capacity(SAMPLES);
        for _ in 0..SAMPLES {
            let sample = session
                .dispatch(
                    &ctx,
                    &values,
                    fixture.registry.total_columns as u32,
                    &crossings,
                )
                .unwrap()
                .carry_gpu_time_ns
                .expect("timestamp-supported carry sample");
            samples.push(sample);
        }
        samples.sort_by(f64::total_cmp);
        let median = samples[SAMPLES / 2];
        let state_width = std::mem::size_of::<ActionBandStateGpu>();
        eprintln!(
            "ACTIONBAND-CARRY-CARDINALITY rows={rows} state_width_bytes={state_width} carry_bytes={} warmup={WARMUP} samples={SAMPLES} statistic=median method=same_depth1_entry_one_sealed_crossing_full_state_current_to_state_next_copy gpu_median_ns={median:.0}",
            rows as usize * state_width,
        );
        assert!(median.is_finite() && median > 0.0);
    }
}
