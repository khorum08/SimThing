use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu,
    EmlTreeId, SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution, ActionBandActiveInstance, FrozenActionBandStructuralRequests,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, cpu_oracle_band_crossing_deltas,
    emit_on_threshold_registrations_to_gpu, eval_eml_cpu, readback_buffer_bytes_blocking,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandExecutionError, ActionBandGpuExecution, ActionBandStateGpu, FacilityPlaneError,
    FacilityPlaneGenerationBoundary, FacilityResidentPlane, GpuContext, PackedThresholdUpload,
    SlotAllocator, ThresholdRegistration, DIR_UPWARD, THRESH_BUF_OWNING_GENERATION,
};
use simthing_sim::{apply_structural_mutations, SimRuntimeTree};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandAdmissionError, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandSessionBuildDoor,
    ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec, ActionBandVelocitySpec,
    ScalarBoundDirection,
};
use wgpu::util::DeviceExt;

struct Fixture {
    registry: DimensionRegistry,
    thresholds: Vec<EmitOnThresholdRegistration>,
    eml: EmlExpressionRegistry,
    column: ColumnIndex,
    previous_column: ColumnIndex,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("proof", "axis", 1));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .expect("amount column");
    let mut register_column = |name: &str| {
        let property = registry.register(SimProperty::simple("proof", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .expect("amount column")
    };
    let previous_column = register_column("previous");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 701,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    let mut eml = EmlExpressionRegistry::new();
    let tree_id = EmlTreeId(0);
    let nodes = vec![
        EmlNodeGpu {
            opcode: eml_opcode::PARAM,
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
    eml.register_formula(
        tree_id,
        EmlFormulaMeta {
            tree_id,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 2,
            has_loops: false,
            has_recursion: false,
            display_name: "actionband-payload-proof".into(),
        },
        nodes,
    )
    .expect("admitted bounded EML");
    for (tree_id, display_name, node) in [
        (
            EmlTreeId(1),
            "actionband-membership-proof",
            EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
        ),
        (
            EmlTreeId(2),
            "actionband-projection-proof",
            EmlNodeGpu {
                opcode: eml_opcode::PARAM,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ),
    ] {
        eml.register_formula(
            tree_id,
            EmlFormulaMeta {
                tree_id,
                execution_class: EmlExecutionClass::ExactDeterministic,
                allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
                max_abs_error: None,
                deterministic_gpu: true,
                requires_guard_for_hard_threshold: false,
                node_count: 1,
                max_stack_depth: 1,
                has_loops: false,
                has_recursion: false,
                display_name: display_name.into(),
            },
            vec![node],
        )
        .expect("admitted target EML");
    }
    Fixture {
        registry,
        thresholds,
        eml,
        column,
        previous_column,
    }
}

fn spec(column: u32, previous: u32, label: &str, storage_rows: u32) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 2,
            dependency_binding_count: 0,
            storage_rows,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "proof-template".into(),
            label: Some(label.into()),
            axis_channels: vec![
                ActionBandChannelBindingSpec {
                    column,
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: previous,
                    kind: ActionBandChannelKind::Primitive,
                },
            ],
            target: ActionBandTargetSpec::ScalarBound {
                channel: column,
                bound: 2.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: Some(ActionBandVelocitySpec {
                current_channel: column,
                previous_generation_channel: Some(previous),
            }),
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(0),
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: storage_rows,
            requirement_semantics: Default::default(),
        }],
    }
}

fn frozen(
    fixture: &Fixture,
    label: &str,
    storage_rows: u32,
) -> simthing_spec::FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(
        &spec(
            fixture.column.raw_u32(),
            fixture.previous_column.raw_u32(),
            label,
            storage_rows,
        ),
        &fixture.registry,
        &fixture.eml,
        &fixture.thresholds,
    )
    .expect("7.1 admission")
    .clone()
}

fn frozen_depth1_fast(fixture: &Fixture, label: &str) -> simthing_spec::FrozenActionBandTemplates {
    let mut fast_spec = spec(
        fixture.column.raw_u32(),
        fixture.previous_column.raw_u32(),
        label,
        1,
    );
    fast_spec.templates[0].velocity = None;
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(
        &fast_spec,
        &fixture.registry,
        &fixture.eml,
        &fixture.thresholds,
    )
    .expect("depth-1 fast-path admission")
    .clone()
}

fn binding() -> ActionBandEmissionBindingGpu {
    ActionBandEmissionBindingGpu::structural_request(42)
}

fn world_values(fixture: &Fixture, current: f32, previous: f32) -> Vec<f32> {
    let mut values = vec![0.0; fixture.registry.total_columns];
    values[fixture.column.raw()] = current;
    values[fixture.previous_column.raw()] = previous;
    values
}

fn sealed_delta(fixture: &Fixture) -> simthing_gpu::BandCrossingDelta {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let previous = world_values(fixture, 0.5, 1.0);
    let current = world_values(fixture, 1.5, 1.0);
    cpu_oracle_band_crossing_deltas(
        &previous,
        &current,
        &[],
        &[],
        fixture.registry.total_columns as u32,
        &regs,
        &fixture.registry,
        &allocator,
    )
    .into_iter()
    .next()
    .expect("existing Phase-5 crossing")
}

fn sealed_delta_from_gpu(fixture: &Fixture, ctx: &GpuContext) -> simthing_gpu::BandCrossingDelta {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    let previous = world_values(fixture, 0.5, 1.0);
    let current = world_values(fixture, 1.5, 1.0);
    let mut session =
        AccumulatorOpSession::new_attached(ctx, 1, fixture.registry.total_columns as u32, 4);
    session.upload_values(ctx, &current);
    session.upload_previous_values(ctx, &previous);
    session
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&regs).expect("threshold upload"),
        )
        .expect("upload existing Phase-5 registrations");
    session.tick(ctx, 0).expect("existing fused threshold pass");
    let emissions = session
        .readback_threshold_emissions(ctx)
        .expect("sealed GPU threshold emissions");
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
        &fixture.registry,
        &allocator,
    )
    .into_iter()
    .next()
    .expect("real sealed Phase-5 GPU crossing")
}

#[test]
fn sealed_crossings_are_the_only_emission_ingress_and_destinations_stay_frozen() {
    let fixture = fixture();
    let frozen = frozen(&fixture, "presentation only", 1);
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let plan = compile_action_band_gpu_execution(&frozen, &fixture.eml, &[binding()], &active)
        .unwrap()
        .into_execution_plan();
    let empty = plan.crossings_from_sealed(&[]).expect("empty sealed input");
    assert_eq!(empty.emission_count(), 0);
    let admitted = plan
        .crossings_from_sealed(&[sealed_delta(&fixture)])
        .expect("sealed join");
    assert_eq!(admitted.crossing_count(), 1);
    assert_eq!(admitted.emission_count(), 1);

    let source = include_str!("../../simthing-kernel/src/accumulator_op/action_band_execution.rs");
    assert!(
        !source.contains("crossed ="),
        "no ActionBand comparator pass"
    );
    assert!(
        !source.contains("pub struct ActionBandCrossingInputGpu"),
        "raw crossing row stays sealed"
    );
    assert!(source.contains("crossings_from_sealed"));

    let fast_frozen = frozen_depth1_fast(&fixture, "empty fast path");
    let fast_active = [ActionBandActiveInstance::new(
        fast_frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let fast_plan =
        compile_action_band_gpu_execution(&fast_frozen, &fixture.eml, &[binding()], &fast_active)
            .unwrap()
            .into_execution_plan();
    assert!(fast_plan.uses_depth1_crossing_fast_path());
    let empty_fast = fast_plan.crossings_from_sealed(&[]).unwrap();
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let world = world_values(&fixture, 1.5, 1.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_empty_fast_world_values"),
            contents: bytemuck::cast_slice(&world),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut execution = match ActionBandGpuExecution::new(&ctx, fast_plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one fast row is active"),
    };
    let result = execution
        .dispatch(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &empty_fast,
        )
        .expect("no crossing performs no ActionBand compute work");
    assert_eq!(result.bucket_dispatches, 0);
    assert!(result.commitments.is_empty());
    assert!(result.gpu_time_ns.is_none());
    assert!(result.evaluation_gpu_time_ns.is_none());
    assert!(result.emission_gpu_time_ns.is_none());
}

fn prove_owning_generation_and_facility_plane_authority_use_the_graduated_gpu_path() {
    let ctx =
        GpuContext::new_blocking().expect("7.6a requires a real GPU adapter; skips forbidden");

    // Owning generation occupies its admitted AccumulatorTickParams word and
    // runs through the sole Phase-5 comparator and emission buffers.
    let registration = ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 3.5,
        direction: DIR_UPWARD,
        event_kind: 7601,
        buffer: THRESH_BUF_OWNING_GENERATION,
    };
    let upload = PackedThresholdUpload::from_registrations(&[registration])
        .expect("owning generation is an admitted Phase-5 observation source");
    let mut threshold_session = AccumulatorOpSession::new(&ctx, 1, 1);
    threshold_session
        .upload_packed_threshold_ops(&ctx, &upload)
        .expect("generation threshold upload retains the ordinary op shape");
    threshold_session.bind_generation_authority(4);
    let dummy_current = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("facility_substrate_dummy_current"),
            contents: bytemuck::cast_slice(&[0.0f32]),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let dummy_previous = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("facility_substrate_dummy_previous"),
            contents: bytemuck::cast_slice(&[0.0f32]),
            usage: wgpu::BufferUsages::STORAGE,
        });
    threshold_session
        .dispatch_threshold_scan(&ctx, &dummy_current, &dummy_previous)
        .expect("ordinary Phase-5 generation crossing");
    let events = threshold_session
        .readback_threshold_events(&ctx)
        .expect("sealed generation crossing readback");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_kind(), 7601);
    assert_eq!(events[0].value().to_bits(), 4.0f32.to_bits());

    let mut invalid = registration;
    invalid.buffer = 99;
    assert!(matches!(
        PackedThresholdUpload::from_registrations(&[invalid]),
        Err(simthing_gpu::EncodeError::InvalidThresholdObservationSource(99))
    ));
    let shader = include_str!("../../simthing-kernel/src/shaders/accumulator_op.wgsl");
    assert_eq!(shader.matches("fn threshold_crossed(").count(), 1);
    assert!(!shader.contains("generation_crossed"));

    // N facilities share the boundary discipline, never a resident plane.
    let mut boundary = FacilityPlaneGenerationBoundary::new();
    let owner_a = boundary.admit_facility();
    let owner_b = boundary.admit_facility();
    let mut plane_a =
        FacilityResidentPlane::from_rows(&ctx, "facility_a", &boundary, &owner_a, &[1u32, 2, 3, 4])
            .unwrap();
    let mut plane_b =
        FacilityResidentPlane::from_rows(&ctx, "facility_b", &boundary, &owner_b, &[9u32, 8])
            .unwrap();
    assert_eq!(plane_a.rows(), 4);
    assert_eq!(plane_a.bytes_per_plane(), 16);
    assert_eq!(plane_b.rows(), 2);
    assert_eq!(plane_b.bytes_per_plane(), 8);
    assert_eq!(
        plane_a.validate_owner(&owner_b),
        Err(FacilityPlaneError::ForeignPlaneWrite)
    );

    let mut second_swap_authority = FacilityPlaneGenerationBoundary::new();
    assert_eq!(
        second_swap_authority.advance(&mut [(&owner_a, &mut plane_a)]),
        Err(FacilityPlaneError::ForeignSwapAuthority)
    );
    assert_eq!(plane_a.generation(), 0);
    assert_eq!(
        boundary.advance(&mut [(&owner_a, &mut plane_a), (&owner_b, &mut plane_b)]),
        Ok(1)
    );
    assert_eq!(plane_a.generation(), 1);
    assert_eq!(plane_b.generation(), 1);
}

#[test]
fn bucketing_is_numeric_deterministic_and_labels_are_semantic_shadow_only() {
    let fixture = fixture();
    let a = frozen(&fixture, "human label A", 1);
    let b = frozen(&fixture, "different domain words", 1);
    let active_a = [ActionBandActiveInstance::new(
        a.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let active_b = [ActionBandActiveInstance::new(
        b.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let plan_a = compile_action_band_gpu_execution(&a, &fixture.eml, &[binding()], &active_a)
        .unwrap()
        .into_execution_plan();
    let plan_b = compile_action_band_gpu_execution(&b, &fixture.eml, &[binding()], &active_b)
        .unwrap()
        .into_execution_plan();
    assert_eq!(plan_a.numeric_fingerprint(), plan_b.numeric_fingerprint());
    assert_eq!(plan_a.buckets(), plan_b.buckets());
    assert_ne!(
        a.semantic_shadow()[0].label(),
        b.semantic_shadow()[0].label()
    );

    let mut bucket_spec = spec(
        fixture.column.raw_u32(),
        fixture.previous_column.raw_u32(),
        "bucket authoring",
        1,
    );
    bucket_spec.budget.emission_binding_count = 1;
    bucket_spec.templates[0].bands = vec![
        ActionBandBandSpec {
            threshold_registration_index: 0,
            eml_program: Some(0),
            emission_binding_indices: vec![0],
        },
        ActionBandBandSpec {
            threshold_registration_index: 0,
            eml_program: Some(0),
            emission_binding_indices: vec![0],
        },
        ActionBandBandSpec {
            threshold_registration_index: 0,
            eml_program: None,
            emission_binding_indices: vec![0],
        },
    ];
    let mut door = ActionBandSessionBuildDoor::new();
    let bucket_product = door
        .admit_once_at_session_build(
            &bucket_spec,
            &fixture.registry,
            &fixture.eml,
            &fixture.thresholds,
        )
        .expect("bucket fixture admission")
        .clone();
    let bucket_active = [ActionBandActiveInstance::new(
        bucket_product.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let bucket_plan = compile_action_band_gpu_execution(
        &bucket_product,
        &fixture.eml,
        &[binding()],
        &bucket_active,
    )
    .expect("bucket lowering")
    .into_execution_plan();
    assert_eq!(bucket_plan.buckets().len(), 2);
    assert!(bucket_plan
        .buckets()
        .iter()
        .any(|bucket| bucket.band_indices == [0, 1]));
    assert!(bucket_plan
        .buckets()
        .iter()
        .any(|bucket| bucket.band_indices == [2]));
    let bucket_crossings = bucket_plan
        .crossings_from_sealed(&[sealed_delta(&fixture)])
        .expect("three crossings grouped by two shapes");
    assert_eq!(bucket_crossings.bucket_dispatch_count(), 2);
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let bucket_world = world_values(&fixture, 1.5, 1.0);
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_bucket_world_values"),
            contents: bytemuck::cast_slice(&bucket_world),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut execution = match ActionBandGpuExecution::new(&ctx, bucket_plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("bucket instance is active"),
    };
    let result = execution
        .dispatch(
            &ctx,
            &values,
            fixture.registry.total_columns as u32,
            &bucket_crossings,
        )
        .expect("bucket partition drives production dispatches");
    assert_eq!(result.bucket_dispatches, 2);
    assert_eq!(result.commitments.len(), 3);
}

#[test]
fn inherited_admission_and_cpu_authority_fences_remain_closed() {
    let fixture = fixture();
    let authored = spec(
        fixture.column.raw_u32(),
        fixture.previous_column.raw_u32(),
        "shadow",
        1,
    );
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_at_session_build(
        &authored,
        &fixture.registry,
        &fixture.eml,
        &fixture.thresholds,
    )
    .expect("first admission");
    assert!(matches!(
        door.admit_once_at_session_build(
            &authored,
            &fixture.registry,
            &fixture.eml,
            &fixture.thresholds
        ),
        Err(ActionBandAdmissionError::MidSessionTemplateMintRefused { .. })
    ));
    let kernel = include_str!("../../simthing-kernel/src/accumulator_op/action_band_execution.rs");
    let driver = include_str!("../src/action_band_execution_compile.rs");
    let structural_door = include_str!("../../simthing-sim/src/tree_mutation.rs");
    let shader = include_str!("../../simthing-kernel/src/shaders/action_band_execution.wgsl");
    let facility_plane =
        include_str!("../../simthing-kernel/src/accumulator_op/facility_resident_plane.rs");
    for forbidden in [
        "ActionBandScheduler",
        "ActionBandPlanner",
        "ActionBandCallback",
        "ActionBandBoundaryQueue",
        "CrossingListener",
        "came_from",
    ] {
        assert!(!kernel.contains(forbidden));
        assert!(!driver.contains(forbidden));
    }
    assert!(shader.contains("var<storage, read> action_state_current"));
    assert!(!shader.contains("var<storage, read_write> action_state_current"));
    assert!(shader.contains("var<storage, read_write> action_state_next"));
    assert!(shader.contains("@binding(7) var<storage, read> values: array<atomic<i32>>"));
    assert!(!shader.contains("@binding(7) var<storage, read_write> values"));
    assert!(kernel.contains("read_only: !matches!(binding, 5 | 6 | 13)"));
    assert!(kernel.contains(".encode_carry(&self.state_owner, &mut encoder)"));
    assert!(kernel.contains(".advance(&mut [(&self.state_owner, &mut self.state_plane)])"));
    assert!(facility_plane.contains("encoder.copy_buffer_to_buffer(current, 0, next, 0"));
    assert!(facility_plane.contains("std::mem::swap(&mut plane.current, &mut plane.next)"));
    assert!(!kernel.contains("state_current: wgpu::Buffer"));
    assert!(!kernel.contains("state_next: wgpu::Buffer"));
    let fast_shader = shader
        .split("fn actionband_emit_depth1")
        .nth(1)
        .expect("depth-1 fast entry")
        .split("fn actionband_emit(")
        .next()
        .expect("bounded fast entry source");
    assert!(fast_shader.contains("crossing.post_value"));
    assert!(!fast_shader.contains("action_value("));
    assert!(kernel.contains("pub fn dispatch("));
    assert!(kernel.contains("fn dispatch_internal("));
    assert!(!kernel.contains("pub struct ActionBandEmissionGpu"));
    assert!(shader.contains("struct ThresholdEmissionGpu"));
    assert!(!shader.contains("atomic_store_f32_at"));
    assert!(!shader.contains("atomic_add_f32_at"));
    assert!(shader.contains("band.threshold_registration"));
    assert!(shader.contains("crossing.crossing_col"));
    assert!(!driver.contains("submit_gpu_authorized"));
    assert!(!driver.contains("from_pre_admitted_rows"));
    assert!(!driver.contains("event_bindings: Vec<(u32"));
    assert!(driver.contains("crossing_binding_for_band"));
    assert!(!driver.contains("emission.reg_idx()"));
    assert!(!structural_door.contains("commitment.value()"));
    assert!(!structural_door.contains("commitment.slot()"));
    assert!(!structural_door.contains("commitment.col()"));
    prove_owning_generation_and_facility_plane_authority_use_the_graduated_gpu_path();
}
