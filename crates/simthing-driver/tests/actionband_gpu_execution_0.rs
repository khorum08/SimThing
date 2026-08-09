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
    cpu_oracle_band_crossing_deltas, emit_on_threshold_registrations_to_gpu, eval_eml_cpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandEmissionDestination, ActionBandExecutionError, ActionBandGpuExecution, GpuContext,
    PackedThresholdUpload, SlotAllocator,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandAdmissionError, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandSessionBuildDoor,
    ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec, ScalarBoundDirection,
};
use wgpu::util::DeviceExt;

struct Fixture {
    registry: DimensionRegistry,
    thresholds: Vec<EmitOnThresholdRegistration>,
    eml: EmlExpressionRegistry,
    column: ColumnIndex,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("proof", "axis", 1));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .expect("amount column");
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
    }
}

fn spec(column: u32, label: &str, storage_rows: u32) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "proof-template".into(),
            label: Some(label.into()),
            axis_channels: vec![ActionBandChannelBindingSpec {
                column,
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: column,
                bound: 2.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
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
        &spec(fixture.column.raw_u32(), label, storage_rows),
        &fixture.registry,
        &fixture.eml,
        &fixture.thresholds,
    )
    .expect("7.1 admission")
    .clone()
}

fn binding() -> ActionBandEmissionBindingGpu {
    ActionBandEmissionBindingGpu::new(ActionBandEmissionDestination::StructuralRequest, 42, 7, 9)
}

fn sealed_delta(fixture: &Fixture) -> simthing_gpu::BandCrossingDelta {
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
    cpu_oracle_band_crossing_deltas(
        &[0.5],
        &[1.5],
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

#[test]
fn sparse_gpu_state_ping_pongs_and_matches_exact_eml_oracle() {
    let _proof_readback = scoped_debug_readback_allowed(true);
    let fixture = fixture();
    let frozen = frozen(&fixture, "first label", 1);
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let plan = compile_action_band_gpu_execution(&frozen, &fixture.eml, &[binding()], &active)
        .expect("numeric lowering");
    let crossings = plan
        .crossings_from_sealed(&[sealed_delta(&fixture)])
        .expect("sealed join");
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let values = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("actionband_test_world_values"),
            contents: bytemuck::cast_slice(&[1.5f32]),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let mut execution = match ActionBandGpuExecution::new(&ctx, plan).expect("GPU operator") {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one sparse row must be active"),
    };
    let first = execution
        .dispatch_and_readback(&ctx, &values, 1, &crossings)
        .expect("first dispatch");
    let second = execution
        .dispatch_and_readback(&ctx, &values, 1, &crossings)
        .expect("second dispatch");
    assert_eq!(first.states[0].generation, 1);
    assert_eq!(second.states[0].generation, 2);
    assert_eq!(first.states[0].satisfied, 0);
    assert_eq!(first.projection, [0.5]);
    let oracle = eval_eml_cpu(
        fixture.eml.get_nodes(EmlTreeId(0)).expect("nodes"),
        0,
        &[1.5],
        1,
        [1.5, 1.0, 0.5, 0.0],
    );
    assert_eq!(first.emissions[0].value.to_bits(), oracle.to_bits());
    assert_eq!(
        first.emissions[0].destination(),
        ActionBandEmissionDestination::StructuralRequest
    );
    assert_eq!(first.emissions[0].destination_index, 42);

    let target = SimThing::new(SimThingKind::Location, 0).id;
    let mut pre_admitted = vec![None; 43];
    pre_admitted[42] = Some(BoundaryRequest::Remove { target });
    let requests = FrozenActionBandStructuralRequests::from_pre_admitted_rows(pre_admitted);
    let (sender, receiver) = feeder_channel();
    assert_eq!(
        requests
            .submit_gpu_authorized(&first.emissions, &sender)
            .unwrap(),
        1
    );
    let drained = receiver.drain_now();
    assert!(matches!(
        drained.as_slice(),
        [FeederWork::Boundary(BoundaryRequest::Remove { target: actual })] if *actual == target
    ));

    let closed_targets = vec![
        ActionBandTargetSpec::Point {
            current_channels: vec![fixture.column.raw_u32()],
            target: vec![2.0],
        },
        ActionBandTargetSpec::ScalarBound {
            channel: fixture.column.raw_u32(),
            bound: 2.0,
            direction: ScalarBoundDirection::AtLeast,
        },
        ActionBandTargetSpec::Interval {
            channel: fixture.column.raw_u32(),
            lo: 1.0,
            hi: 2.0,
        },
        ActionBandTargetSpec::AxisAlignedBox {
            channels: vec![fixture.column.raw_u32()],
            lo: vec![1.0],
            hi: vec![2.0],
        },
        ActionBandTargetSpec::LocusRadius {
            distance_channel: fixture.column.raw_u32(),
            radius: 2.0,
        },
        ActionBandTargetSpec::PalmaReachableSet {
            distance_channel: fixture.column.raw_u32(),
            maximum_distance: 1.0,
        },
        ActionBandTargetSpec::EmlProjectedSet {
            input_channels: vec![fixture.column.raw_u32()],
            membership_program: 1,
            projection_program: Some(2),
            projection_width: 1,
        },
    ];
    let all_forms_spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 7,
            eml_program_count: 2,
            emission_binding_count: 0,
        },
        templates: closed_targets
            .into_iter()
            .enumerate()
            .map(|(index, target)| ActionBandTemplateSpec {
                id: format!("closed-gpu-form-{index}"),
                label: Some(format!("ignored form label {index}")),
                axis_channels: vec![ActionBandChannelBindingSpec {
                    column: fixture.column.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                }],
                target,
                velocity: None,
                bands: vec![],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 1,
                requirement_semantics: Default::default(),
            })
            .collect(),
    };
    let mut all_forms_door = ActionBandSessionBuildDoor::new();
    let all_forms = all_forms_door
        .admit_once_at_session_build(
            &all_forms_spec,
            &fixture.registry,
            &fixture.eml,
            &fixture.thresholds,
        )
        .expect("all seven closed forms admit")
        .clone();
    let all_active: Vec<_> = all_forms
        .templates()
        .iter()
        .map(|template| {
            ActionBandActiveInstance::new(template.index(), SlotIndex::new(0), [0.0; 4])
        })
        .collect();
    let all_plan =
        compile_action_band_gpu_execution(&all_forms, &fixture.eml, &[], &all_active).unwrap();
    let all_crossings = all_plan.crossings_from_sealed(&[]).unwrap();
    let mut all_execution = match ActionBandGpuExecution::new(&ctx, all_plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("seven active target rows"),
    };
    let all_readback = all_execution
        .dispatch_and_readback(&ctx, &values, 1, &all_crossings)
        .expect("all seven target forms execute on GPU");
    assert_eq!(
        all_readback
            .states
            .iter()
            .map(|state| state.satisfied)
            .collect::<Vec<_>>(),
        [0, 0, 1, 1, 1, 0, 1]
    );
    assert_eq!(all_readback.projection, [0.5, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0]);

    if ctx.timestamp_supported() {
        let mut action_samples_ns = Vec::new();
        for _ in 0..15 {
            action_samples_ns.push(
                execution
                    .dispatch_and_readback(&ctx, &values, 1, &crossings)
                    .expect("timed ActionBand dispatch")
                    .gpu_time_ns
                    .expect("timestamp-supported ActionBand sample"),
            );
        }
        action_samples_ns.sort_by(f64::total_cmp);

        let regs = emit_on_threshold_registrations_to_gpu(&fixture.thresholds);
        let upload = PackedThresholdUpload::from_registrations(&regs).expect("threshold packet");
        let mut crossing_session = AccumulatorOpSession::new(&ctx, 1, 1);
        crossing_session
            .upload_packed_threshold_ops(&ctx, &upload)
            .expect("crossing op upload");
        let previous = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("actionband_bench_previous"),
                contents: bytemuck::cast_slice(&[0.5f32]),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let current = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("actionband_bench_current"),
                contents: bytemuck::cast_slice(&[1.5f32]),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let mut crossing_samples_ns = Vec::new();
        for _ in 0..15 {
            crossing_session
                .dispatch_threshold_scan(&ctx, &current, &previous)
                .expect("timed existing crossing dispatch");
            crossing_samples_ns.push(
                crossing_session
                    .last_pass_time_us()
                    .expect("timestamp-supported crossing sample") as f64
                    * 1_000.0,
            );
        }
        crossing_samples_ns.sort_by(f64::total_cmp);
        let action_median = action_samples_ns[action_samples_ns.len() / 2];
        let crossing_median = crossing_samples_ns[crossing_samples_ns.len() / 2];
        let ratio = action_median / crossing_median.max(1.0);
        eprintln!(
            "ACTIONBAND-DEPTH1-MEASUREMENT samples=15 action_gpu_median_ns={action_median:.0} existing_crossing_gpu_median_ns={crossing_median:.0} ratio={ratio:.3}"
        );
        assert!(
            ratio < 20.0,
            "material unexplained depth-1 regression: {ratio:.3}x"
        );
    }
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
    let plan =
        compile_action_band_gpu_execution(&frozen, &fixture.eml, &[binding()], &active).unwrap();
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
}

#[test]
fn inactive_rows_allocate_zero_hot_storage_and_dense_mutant_is_red() {
    let fixture = fixture();
    let frozen = frozen(&fixture, "zero", 1);
    let plan = compile_action_band_gpu_execution(&frozen, &fixture.eml, &[binding()], &[]).unwrap();
    assert_eq!(plan.active_instance_rows(), 0);
    assert_eq!(plan.hot_state_bytes(), 0);

    let duplicate = [
        ActionBandActiveInstance::new(frozen.templates()[0].index(), SlotIndex::new(0), [0.0; 4]),
        ActionBandActiveInstance::new(frozen.templates()[0].index(), SlotIndex::new(1), [0.0; 4]),
    ];
    assert!(matches!(
        compile_action_band_gpu_execution(&frozen, &fixture.eml, &[binding()], &duplicate),
        Err(simthing_driver::ActionBandExecutionCompileError::Kernel(
            ActionBandExecutionError::SparseRowBudgetExceeded {
                active: 2,
                reserved: 1
            }
        ))
    ));
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
    let plan_a =
        compile_action_band_gpu_execution(&a, &fixture.eml, &[binding()], &active_a).unwrap();
    let plan_b =
        compile_action_band_gpu_execution(&b, &fixture.eml, &[binding()], &active_b).unwrap();
    assert_eq!(plan_a.numeric_fingerprint(), plan_b.numeric_fingerprint());
    assert_eq!(plan_a.buckets(), plan_b.buckets());
    assert_ne!(
        a.semantic_shadow()[0].label(),
        b.semantic_shadow()[0].label()
    );

    let mut bucket_spec = spec(fixture.column.raw_u32(), "bucket authoring", 1);
    bucket_spec.budget.emission_binding_count = 2;
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
            eml_program: Some(0),
            emission_binding_indices: vec![1],
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
        &[
            ActionBandEmissionBindingGpu::new(ActionBandEmissionDestination::Telemetry, 9, 0, 0),
            ActionBandEmissionBindingGpu::new(ActionBandEmissionDestination::CostBand, 4, 0, 0),
        ],
        &bucket_active,
    )
    .expect("bucket lowering");
    assert_eq!(bucket_plan.buckets().len(), 2);
    assert!(bucket_plan
        .buckets()
        .iter()
        .any(|bucket| bucket.band_indices == [0, 1]));
    assert!(bucket_plan
        .buckets()
        .iter()
        .any(|bucket| bucket.band_indices == [2]));
}

#[test]
fn inherited_admission_and_cpu_authority_fences_remain_closed() {
    let fixture = fixture();
    let authored = spec(fixture.column.raw_u32(), "shadow", 1);
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
    let shader = include_str!("../../simthing-kernel/src/shaders/action_band_execution.wgsl");
    for forbidden in [
        "ActionBandScheduler",
        "ActionBandPlanner",
        "ActionBandCallback",
        "came_from",
    ] {
        assert!(!kernel.contains(forbidden));
        assert!(!driver.contains(forbidden));
    }
    assert!(shader.contains("var<storage, read> action_state_current"));
    assert!(shader.contains("var<storage, read_write> action_state_next"));
    assert!(!shader.contains("action_state_current[row] ="));
    assert!(
        !include_str!("../../simthing-kernel/src/decision_ingress.rs").contains("HORIZON-ENTRY")
    );
    assert!(!include_str!("../../simthing-sim/src/overlay_lifecycle.rs").contains("HORIZON-ENTRY"));
}
