use simthing_core::{
    eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode,
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SimProperty, SimThing, SimThingKind, SlotIndex, SourceSpec, StructuralScalarChannel,
    SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, compile_action_band_gpu_execution_with_native_lanes,
    ActionBandActiveInstance, ActionBandExecutionCompileError, ActionBandNativeLaneAdmission,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandExecutionError, ActionBandGpuExecution, ActionBandPropertyWrite, ActionBandStateGpu,
    GpuContext, PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator,
    ACTIONBAND_STATE_ACTIVE,
};
use simthing_sim::{CostBandSemantic, ThresholdRegistry, ThresholdSemantic};
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
    child_a: ColumnIndex,
    child_b: ColumnIndex,
    consequence: ColumnIndex,
    rf_available: ColumnIndex,
    cost_available: ColumnIndex,
    rf_claim: ColumnIndex,
    rf_result: ColumnIndex,
    cost_progress: ColumnIndex,
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
    let child_a = register("child-a-state");
    let child_b = register("child-b-state");
    let consequence = register("ordinary-consequence");
    let rf_available = register("rf-available");
    let cost_available = register("cost-available");
    let rf_claim = register("rf-claim");
    let rf_result = register("rf-result");
    let cost_progress = register("cost-progress");
    let thresholds = [state, child_a, child_b]
        .into_iter()
        .enumerate()
        .map(|(event_kind, col)| EmitOnThresholdRegistration {
            slot: SlotIndex::new(0),
            col,
            threshold: 1.0,
            direction: ThresholdDirection::Upward,
            event_kind: event_kind as u32 + 100,
            buffer: EmitOnThresholdBuffer::Values,
        })
        .collect();
    Fixture {
        registry,
        thresholds,
        state,
        child_a,
        child_b,
        consequence,
        rf_available,
        cost_available,
        rf_claim,
        rf_result,
        cost_progress,
    }
}

fn template(
    id: &str,
    column: u32,
    threshold_registration_index: u32,
    emission_binding_index: u32,
    children: &[&str],
) -> ActionBandTemplateSpec {
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
            threshold_registration_index,
            eml_program: None,
            emission_binding_indices: vec![emission_binding_index],
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
            axis_channel_count: 3,
            dependency_binding_count: 2,
            storage_rows: 3,
            eml_program_count: 0,
            emission_binding_count: 3,
        },
        templates: vec![
            template("parent", fixture.state.raw_u32(), 0, 0, &children),
            template("child-a", fixture.child_a.raw_u32(), 1, 1, &[]),
            template("child-b", fixture.child_b.raw_u32(), 2, 2, &[]),
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

fn recursive_bindings(fixture: &Fixture) -> [ActionBandEmissionBindingGpu; 3] {
    [
        binding(),
        ActionBandEmissionBindingGpu::property_next(
            fixture.state.raw_u32(),
            ActionBandPropertyWrite::Add,
        ),
        ActionBandEmissionBindingGpu::property_next(
            fixture.consequence.raw_u32(),
            ActionBandPropertyWrite::Set,
        ),
    ]
}

fn world(
    fixture: &Fixture,
    state: f32,
    child_a: f32,
    child_b: f32,
    rf_available: f32,
    cost_available: f32,
) -> Vec<f32> {
    let mut values = vec![0.0; fixture.registry.total_columns];
    values[fixture.state.raw()] = state;
    values[fixture.child_a.raw()] = child_a;
    values[fixture.child_b.raw()] = child_b;
    values[fixture.rf_available.raw()] = rf_available;
    values[fixture.cost_available.raw()] = cost_available;
    values
}

fn gpu_deltas(
    fixture: &Fixture,
    ctx: &GpuContext,
    previous: &[f32],
    current: &[f32],
    registrations: &[EmitOnThresholdRegistration],
) -> Vec<simthing_gpu::BandCrossingDelta> {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let registrations = emit_on_threshold_registrations_to_gpu(registrations);
    let mut session = AccumulatorOpSession::new_attached(
        ctx,
        1,
        fixture.registry.total_columns as u32,
        registrations.len().max(1) as u32,
    );
    session.upload_values(ctx, current);
    session.upload_previous_values(ctx, previous);
    session
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&registrations).unwrap(),
        )
        .unwrap();
    session.tick(ctx, 0).unwrap();
    let emissions = session.readback_threshold_emissions(ctx).unwrap();
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
        &fixture.registry,
        &allocator,
    )
}

fn gpu_deltas_from_buffer(
    fixture: &Fixture,
    ctx: &GpuContext,
    previous: &[f32],
    current: &wgpu::Buffer,
    registrations: &[EmitOnThresholdRegistration],
) -> Vec<simthing_gpu::BandCrossingDelta> {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let registrations = emit_on_threshold_registrations_to_gpu(registrations);
    let mut session = AccumulatorOpSession::new_attached(
        ctx,
        1,
        fixture.registry.total_columns as u32,
        registrations.len().max(1) as u32,
    );
    session.upload_previous_values(ctx, previous);
    session
        .copy_values_prefix_from_buffer(ctx, current, 0, 0, current.size())
        .unwrap();
    session
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&registrations).unwrap(),
        )
        .unwrap();
    session.tick(ctx, 0).unwrap();
    let emissions = session.readback_threshold_emissions(ctx).unwrap();
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
        &fixture.registry,
        &allocator,
    )
}

fn gpu_world_buffer(ctx: &GpuContext, label: &str, values: &[f32]) -> wgpu::Buffer {
    ctx.device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(values),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        })
}

fn native_next_buffer(ctx: &GpuContext, label: &str, bytes: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[test]
fn parent_activates_children_next_then_resolves_after_later_collapse() {
    let fixture = fixture();
    let eml = EmlExpressionRegistry::new();
    let frozen = admit(&fixture, &recursive_spec(&fixture, false), &eml);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.state, fixture.consequence],
        &[],
        &[],
        &ThresholdRegistry::new(),
    );
    let plan = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &recursive_bindings(&fixture),
        &recursive_instances(&frozen),
        &native,
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
    let before_parent = world(&fixture, 0.5, 0.0, 0.0, 2.0, 3.0);
    let parent_generation = world(&fixture, 1.5, 0.0, 0.0, 2.0, 3.0);
    let child_generation = world(&fixture, 0.5, 1.5, 1.5, 2.0, 3.0);
    let parent_crossings = plan
        .crossings_from_sealed(&gpu_deltas(
            &fixture,
            &ctx,
            &before_parent,
            &parent_generation,
            &fixture.thresholds,
        ))
        .expect("generation t has only the real parent crossing");
    let child_crossings = plan
        .crossings_from_sealed(&gpu_deltas(
            &fixture,
            &ctx,
            &parent_generation,
            &child_generation,
            &fixture.thresholds,
        ))
        .expect("generation t+1 has two real child crossings");
    let parent_values = gpu_world_buffer(&ctx, "actionband_parent_generation", &parent_generation);
    let child_values = gpu_world_buffer(&ctx, "actionband_child_generation", &child_generation);
    let next_a = native_next_buffer(&ctx, "actionband_recursive_next_a", parent_values.size());
    let next_b = native_next_buffer(&ctx, "actionband_recursive_next_b", parent_values.size());
    let mut session = match ActionBandGpuExecution::new(&ctx, plan.clone()).expect("GPU session") {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("three materialized sparse rows"),
    };
    let _proof = scoped_debug_readback_allowed(true);

    let activate = session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &parent_values,
            &next_a,
            fixture.registry.total_columns as u32,
            &parent_crossings,
        )
        .expect("generation t activates child-next");
    assert!(activate.commitments.is_empty());
    assert_eq!(activate.states[0].generation, 1);
    assert_eq!(activate.states[1].generation, 0);
    assert_eq!(activate.states[2].generation, 0);
    assert_eq!(activate.states[1].reserved[0], ACTIONBAND_STATE_ACTIVE);
    assert_eq!(activate.states[2].reserved[0], ACTIONBAND_STATE_ACTIVE);

    let children = session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &child_values,
            &next_b,
            fixture.registry.total_columns as u32,
            &child_crossings,
        )
        .expect("generation t+1 executes concurrent children");
    assert!(children.commitments.is_empty());
    assert_eq!(children.states[0].generation, 1);
    for state in &children.states[1..] {
        assert_eq!(state.generation, 1);
        assert_eq!(state.satisfied, 1);
        assert_eq!(state.reserved[0], 0);
    }

    let later_parent_crossings = plan
        .crossings_from_sealed(&gpu_deltas_from_buffer(
            &fixture,
            &ctx,
            &child_generation,
            &next_b,
            &fixture.thresholds,
        ))
        .expect("ordinary child consequence creates a fresh parent crossing at t+2");
    let resolved = session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &next_b,
            &next_a,
            fixture.registry.total_columns as u32,
            &later_parent_crossings,
        )
        .expect("generation t+2 observes collapsed children");
    assert_eq!(resolved.commitments.len(), 1);
    assert_eq!(resolved.states[0].generation, 2);
    assert_eq!(resolved.states[0].satisfied, 1);
    assert_eq!(resolved.states[0].reserved[0], ACTIONBAND_STATE_ACTIVE);
    assert_eq!(resolved.states[1].reserved[0], 0);
    assert_eq!(resolved.states[2].reserved[0], 0);
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
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.state, fixture.consequence],
        &[],
        &[],
        &ThresholdRegistry::new(),
    );
    let bindings = recursive_bindings(&fixture);
    let first_plan = compile_action_band_gpu_execution_with_native_lanes(
        &first,
        &eml,
        &bindings,
        &first_instances,
        &native,
    )
    .unwrap()
    .into_execution_plan();
    let reversed_plan = compile_action_band_gpu_execution_with_native_lanes(
        &reversed,
        &eml,
        &bindings,
        &reversed_instances,
        &native,
    )
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
    let before_parent = world(&fixture, 0.5, 0.0, 0.0, 2.0, 3.0);
    let parent_generation = world(&fixture, 1.5, 0.0, 0.0, 2.0, 3.0);
    let child_generation = world(&fixture, 0.5, 1.5, 1.5, 2.0, 3.0);
    let parent_deltas = gpu_deltas(
        &fixture,
        &ctx,
        &before_parent,
        &parent_generation,
        &fixture.thresholds,
    );
    let child_deltas = gpu_deltas(
        &fixture,
        &ctx,
        &parent_generation,
        &child_generation,
        &fixture.thresholds,
    );
    let first_parent_crossings = first_plan.crossings_from_sealed(&parent_deltas).unwrap();
    let reversed_parent_crossings = reversed_plan.crossings_from_sealed(&parent_deltas).unwrap();
    let first_child_crossings = first_plan.crossings_from_sealed(&child_deltas).unwrap();
    let reversed_child_crossings = reversed_plan.crossings_from_sealed(&child_deltas).unwrap();
    let parent_values = gpu_world_buffer(&ctx, "actionband_append_parent", &parent_generation);
    let child_values = gpu_world_buffer(&ctx, "actionband_append_children", &child_generation);
    let first_next_a = native_next_buffer(&ctx, "actionband_first_next_a", parent_values.size());
    let first_next_b = native_next_buffer(&ctx, "actionband_first_next_b", parent_values.size());
    let reversed_next_a =
        native_next_buffer(&ctx, "actionband_reversed_next_a", parent_values.size());
    let reversed_next_b =
        native_next_buffer(&ctx, "actionband_reversed_next_b", parent_values.size());
    let mut first_session = match ActionBandGpuExecution::new(&ctx, first_plan.clone()).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("first recursive plan is active"),
    };
    let mut reversed_session =
        match ActionBandGpuExecution::new(&ctx, reversed_plan.clone()).unwrap() {
            ActionBandGpuExecution::Active(session) => session,
            ActionBandGpuExecution::Inactive => panic!("reversed recursive plan is active"),
        };
    let _proof = scoped_debug_readback_allowed(true);
    for (values, first_next, reversed_next, first_crossings, reversed_crossings) in [
        (
            &parent_values,
            &first_next_a,
            &reversed_next_a,
            &first_parent_crossings,
            &reversed_parent_crossings,
        ),
        (
            &child_values,
            &first_next_b,
            &reversed_next_b,
            &first_child_crossings,
            &reversed_child_crossings,
        ),
    ] {
        let first = first_session
            .dispatch_with_native_next_and_readback(
                &ctx,
                values,
                first_next,
                fixture.registry.total_columns as u32,
                first_crossings,
            )
            .unwrap();
        let reversed = reversed_session
            .dispatch_with_native_next_and_readback(
                &ctx,
                values,
                reversed_next,
                fixture.registry.total_columns as u32,
                reversed_crossings,
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
    let first_later = first_plan
        .crossings_from_sealed(&gpu_deltas_from_buffer(
            &fixture,
            &ctx,
            &child_generation,
            &first_next_b,
            &fixture.thresholds,
        ))
        .unwrap();
    let reversed_later = reversed_plan
        .crossings_from_sealed(&gpu_deltas_from_buffer(
            &fixture,
            &ctx,
            &child_generation,
            &reversed_next_b,
            &fixture.thresholds,
        ))
        .unwrap();
    let first = first_session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &first_next_b,
            &first_next_a,
            fixture.registry.total_columns as u32,
            &first_later,
        )
        .unwrap();
    let reversed = reversed_session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &reversed_next_b,
            &reversed_next_a,
            fixture.registry.total_columns as u32,
            &reversed_later,
        )
        .unwrap();
    assert_eq!(
        bytemuck::cast_slice::<ActionBandStateGpu, u8>(&first.states),
        bytemuck::cast_slice::<ActionBandStateGpu, u8>(&reversed.states)
    );
    assert_eq!(first.commitments.len(), 1);
    assert_eq!(reversed.commitments.len(), 1);
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
        compile_action_band_gpu_execution(
            &frozen,
            &eml,
            &recursive_bindings(&fixture),
            &parent_only,
        ),
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
    nonfast.budget.axis_channel_count = 4;
    let nonfast = admit(&fixture, &nonfast, &eml);
    assert!(matches!(
        compile_action_band_gpu_execution(
            &nonfast,
            &eml,
            &recursive_bindings(&fixture),
            &recursive_instances(&nonfast),
        ),
        Err(ActionBandExecutionCompileError::Kernel(
            ActionBandExecutionError::RecursiveShapeDeferred
        ))
    ));

    let mut shared_spec = recursive_spec(&fixture, false);
    shared_spec.templates.insert(
        1,
        template("second-parent", fixture.state.raw_u32(), 0, 0, &["child-a"]),
    );
    shared_spec.budget.axis_channel_count = 4;
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
        compile_action_band_gpu_execution(&shared, &eml, &recursive_bindings(&fixture), &rows),
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
    // Ordinary state and the existing RF/CostBand input lanes remain EML inputs.
    let nodes = vec![
        slot_value(fixture.state.raw_u32()),
        slot_value(fixture.rf_available.raw_u32()),
        multiply,
        slot_value(fixture.cost_available.raw_u32()),
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
            emission_binding_count: 2,
        },
        templates: vec![ActionBandTemplateSpec {
            axis_channels: vec![
                ActionBandChannelBindingSpec {
                    column: fixture.state.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: fixture.rf_available.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: fixture.cost_available.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
            ],
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(0),
                emission_binding_indices: vec![0, 1],
            }],
            ..template("inline-gate", fixture.state.raw_u32(), 0, 0, &[])
        }],
    };
    let frozen = admit(&fixture, &spec, &eml);
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let rf_plan = CompiledAccumulatorOpPlan {
        slot_count: 1,
        n_dims: fixture.registry.total_columns as u32,
        input_channel: StructuralScalarChannel::new(fixture.rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(fixture.rf_result.raw_u32()),
        ops: vec![AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: fixture.rf_claim,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(0), fixture.rf_result)],
        }],
    };
    let mut cost_registry = ThresholdRegistry::new();
    let cost_event = cost_registry.push_with_cost_band(
        ThresholdSemantic::ScriptedEventTrigger {
            event_id: "actionband-cost-proof".into(),
        },
        CostBandSemantic::admit_sink(None, None).unwrap(),
    );
    let cost_threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: fixture.cost_progress,
        threshold: 3.0,
        direction: ThresholdDirection::Upward,
        event_kind: cost_event,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let native_bindings = [
        ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32()),
        ActionBandEmissionBindingGpu::cost_band(fixture.cost_progress.raw_u32()),
    ];
    assert!(matches!(
        compile_action_band_gpu_execution(&frozen, &eml, &native_bindings, &active),
        Err(ActionBandExecutionCompileError::Kernel(
            ActionBandExecutionError::StructuralBindingCount { count: 2 }
        ))
    ));
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[],
        std::slice::from_ref(&rf_plan),
        std::slice::from_ref(&cost_threshold),
        &cost_registry,
    );
    let plan = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &native_bindings,
        &active,
        &native,
    )
    .expect("source-bound inline native lane gate")
    .into_execution_plan();
    assert_eq!(plan.dependency_row_count(), 0);
    assert!(!plan.uses_depth2_common_fast_shape());

    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let previous = world(&fixture, 0.5, 0.0, 0.0, 2.0, 3.0);
    let current = world(&fixture, 1.5, 0.0, 0.0, 2.0, 3.0);
    let crossings = plan
        .crossings_from_sealed(&gpu_deltas(
            &fixture,
            &ctx,
            &previous,
            &current,
            &fixture.thresholds[..1],
        ))
        .unwrap();
    let values = gpu_world_buffer(&ctx, "actionband_multisource_world", &current);
    let values_next = native_next_buffer(&ctx, "actionband_multisource_next", values.size());
    let mut session = match ActionBandGpuExecution::new(&ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one inline row"),
    };
    let _proof = scoped_debug_readback_allowed(true);
    let readback = session
        .dispatch_with_native_next_and_readback(
            &ctx,
            &values,
            &values_next,
            fixture.registry.total_columns as u32,
            &crossings,
        )
        .expect("same depth-1 entry executes native column gate");
    assert!(readback.emission_payloads.is_empty());
    assert!(readback.commitments.is_empty());

    let mut rf_session = AccumulatorOpSession::new(&ctx, rf_plan.slot_count, rf_plan.n_dims);
    rf_session
        .copy_values_prefix_from_buffer(&ctx, &values_next, 0, 0, values_next.size())
        .unwrap();
    rf_session
        .upload_packed_ops(
            &ctx,
            &PackedAccumulatorUpload::from_ops(&rf_plan.ops).unwrap(),
        )
        .unwrap();
    rf_session.tick(&ctx, 0).unwrap();
    let rf_values = rf_session.readback_full(&ctx).unwrap();
    assert_eq!(rf_values[fixture.rf_result.raw()], 9.0);

    let cost_deltas = gpu_deltas_from_buffer(
        &fixture,
        &ctx,
        &current,
        &values_next,
        std::slice::from_ref(&cost_threshold),
    );
    assert_eq!(cost_deltas.len(), 1);
    let draw = cost_registry
        .resolve_cost_band_draw_from_delta(&cost_deltas[0])
        .unwrap();
    assert_eq!((draw.v, draw.c, draw.n, draw.r), (9.0, 3.0, 3, 0.0));
    assert_eq!(cost_registry.cost_band_resolve_invocations, 1);
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
    let values_cpu = world(&fixture, 1.5, 0.0, 0.0, 2.0, 3.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_carry_curve_world"),
            contents: bytemuck::cast_slice(&values_cpu),
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
                ..template("carry-curve", fixture.state.raw_u32(), 0, 0, &[])
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
        let before = world(&fixture, 0.5, 0.0, 0.0, 2.0, 3.0);
        let crossings = plan
            .crossings_from_sealed(&gpu_deltas(
                &fixture,
                &ctx,
                &before,
                &values_cpu,
                &fixture.thresholds[..1],
            ))
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
