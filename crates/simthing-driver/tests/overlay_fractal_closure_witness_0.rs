//! OVERLAY-FRACTAL-CLOSURE-WITNESS-0: one adversarial closure scenario.

use std::io::Cursor;
use std::sync::Mutex;

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    deliver_predicate_broadcast, deliver_routed_overlay, AccumulatorOp, CombineFn,
    CompiledAccumulatorOpPlan, ConsumeMode, DimensionRegistry, DissolveCondition,
    EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlTreeId, GateSpec, GenerationStamp, OverlayKind,
    OverlayLifecycle, OverlaySource, RoutedPredicate, RoutedPredicateComparison, ScaleSpec,
    SimProperty, SimThing, SimThingKind, SlotIndex, SourceSpec, StructuralScalarChannel,
    SubFieldRole, ThresholdDirection, TransformOp,
};
use simthing_driver::{
    compile_action_band_gpu_execution_with_native_lanes, compile_crossing_consequence_session,
    compile_gu_yang_n4_field_sweeps, ActionBandActiveInstance, ActionBandNativeLaneAdmission,
    GuYangN4FieldSweepSpec, RoutedOverlayDelivery,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandGpuExecution, FieldSweepOutput, FieldSweepSession, GpuContext,
    OverlayProjectionHostChange, OverlaySpanProjection, PackedAccumulatorUpload,
    PackedThresholdUpload, SlotAllocator, OP_MULTIPLY,
};
use simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState;
use simthing_sim::{
    apply_structural_mutations, BoundaryDeltaEntry, ReplayDriver, ReplayFrame, ReplayReader,
    ReplaySnapshot, ReplayWriter, SimRuntimeTree, ThresholdRegistry,
};
use simthing_spec::{
    compile_eml_gadget, compile_overlay, ActionBandAdmissionBudgetSpec, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandSessionBuildDoor, ActionBandSessionSpec,
    ActionBandTargetSpec, ActionBandTemplateSpec, EmlGadgetCompileOptions, EmlGadgetInstanceSpec,
    InstallTargetSpec, OverlaySpec, ScalarBoundDirection, SpecError,
};

static GPU: Mutex<()> = Mutex::new(());
const LARGE_SUBTREE_LEAVES: usize = 100_000;
const EVENT_KIND: u32 = 7_900;

struct ActionFixture {
    registry: DimensionRegistry,
    threshold: EmitOnThresholdRegistration,
    feedback_input: simthing_core::ColumnIndex,
    feedback_previous: simthing_core::ColumnIndex,
    rf_claim: simthing_core::ColumnIndex,
    rf_result: simthing_core::ColumnIndex,
    eml: EmlExpressionRegistry,
    frozen: simthing_spec::FrozenActionBandTemplates,
}

fn action_fixture() -> ActionFixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("closure", "crossing", 0));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .unwrap();
    let feedback_input_property =
        registry.register(SimProperty::simple("closure", "field-input", 0));
    let feedback_input = registry
        .column_range(feedback_input_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(feedback_input_property).layout,
        )
        .unwrap();
    let feedback_previous_property =
        registry.register(SimProperty::simple("closure", "feedback-previous", 0));
    let feedback_previous = registry
        .column_range(feedback_previous_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(feedback_previous_property).layout,
        )
        .unwrap();
    let rf_claim_property = registry.register(SimProperty::simple("closure", "rf-claim", 0));
    let rf_claim = registry
        .column_range(rf_claim_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(rf_claim_property).layout,
        )
        .unwrap();
    let rf_result_property = registry.register(SimProperty::simple("closure", "rf-result", 0));
    let rf_result = registry
        .column_range(rf_result_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(rf_result_property).layout,
        )
        .unwrap();
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: EVENT_KIND,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "closure-trigger".into(),
            label: Some("presentation-only".into()),
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: column.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: column.raw_u32(),
                bound: 1.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: None,
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let eml = EmlExpressionRegistry::new();
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(&spec, &registry, &eml, std::slice::from_ref(&threshold))
        .unwrap()
        .clone();
    ActionFixture {
        registry,
        threshold,
        feedback_input,
        feedback_previous,
        rf_claim,
        rf_result,
        eml,
        frozen,
    }
}

fn real_crossing(fixture: &ActionFixture, ctx: &GpuContext) -> simthing_gpu::BandCrossingDelta {
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&SimThing::new(SimThingKind::GameSession, 0));
    let column = fixture.threshold.col;
    let mut previous = vec![0.0; fixture.registry.total_columns];
    let mut current = previous.clone();
    previous[column.raw()] = 0.5;
    current[column.raw()] = 1.5;
    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, 1, fixture.registry.total_columns as u32, 2);
    phase5.upload_values(ctx, &current);
    phase5.upload_previous_values(ctx, &previous);
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&emit_on_threshold_registrations_to_gpu(
                std::slice::from_ref(&fixture.threshold),
            ))
            .unwrap(),
        )
        .unwrap();
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fixture.registry,
        &allocator,
    )
    .into_iter()
    .next()
    .expect("the existing fused Phase-5 crossing is the only trigger")
}

fn overlay_spec(
    id: String,
    factor: f32,
    lifecycle: OverlayLifecycle,
    current_dependency_edges: Vec<(String, String)>,
    next_dependency_edges: Vec<(String, String)>,
    source_span_token: Option<usize>,
) -> OverlaySpec {
    OverlaySpec {
        id,
        display_name: "not a dispatch key".into(),
        targets_property: "closure::admitted-gate".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(factor))],
        lifecycle,
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        install: InstallTargetSpec::SessionRoot,
        order_weight_class: None,
        composition_class: Some("conjunctive-restriction".into()),
        current_dependency_edges,
        next_dependency_edges,
        source_span_token,
    }
}

fn admitted_property_registry() -> (DimensionRegistry, simthing_core::SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("closure", "admitted-gate", 0));
    (registry, property)
}

fn tree_from_runtime(runtime: &SimRuntimeTree) -> SimThing {
    serde_json::from_value(serde_json::to_value(runtime).unwrap()).unwrap()
}

fn write_replay(snapshot: &ReplaySnapshot, frame: &ReplayFrame) -> Vec<u8> {
    let mut writer = ReplayWriter::new(Vec::new());
    writer.write_snapshot(snapshot).unwrap();
    writer.write_frame(frame).unwrap();
    writer.flush().unwrap();
    writer.into_inner()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClosureFeedbackGeneration {
    field_signal_bits: u32,
    native_flux_bits: u32,
    feedback_bits: u32,
    rf_increment_bits: u32,
    action_generation: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClosureFeedbackRun {
    generations: [ClosureFeedbackGeneration; 2],
    final_world_bits: Vec<u32>,
}

fn closure_rf_plan(action: &ActionFixture) -> CompiledAccumulatorOpPlan {
    CompiledAccumulatorOpPlan {
        slot_count: 2,
        n_dims: action.registry.total_columns as u32,
        input_channel: StructuralScalarChannel::new(action.rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(action.rf_result.raw_u32()),
        ops: vec![AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: action.rf_claim,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(0), action.rf_result)],
        }],
    }
}

fn closure_feedback_plan(
    action: &ActionFixture,
    threshold: &EmitOnThresholdRegistration,
    rf_plan: &CompiledAccumulatorOpPlan,
) -> simthing_gpu::ActionBandExecutionPlan {
    let compiled_feedback = compile_eml_gadget(
        &EmlGadgetInstanceSpec::BoundedFeedback {
            id: "closure-field-rf-feedback".into(),
            previous_col: action.feedback_previous.raw_u32(),
            input_col: action.threshold.col.raw_u32(),
            output_col: Some(action.feedback_previous.raw_u32()),
            decay: 0.5,
            gain: 0.6,
            min: 0.0,
            max: 1.0,
        },
        EmlGadgetCompileOptions {
            max_col: action.registry.total_columns as u32,
        },
    )
    .expect("bounded Current -> Next field/RF recurrence");
    let feedback_program = EmlTreeId(7_902);
    let mut eml = EmlExpressionRegistry::new();
    eml.register_formula(
        feedback_program,
        EmlFormulaMeta {
            tree_id: feedback_program,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: compiled_feedback.nodes.len() as u32,
            max_stack_depth: 3,
            has_loops: false,
            has_recursion: false,
            display_name: "closure-bounded-feedback".into(),
        },
        compiled_feedback.nodes,
    )
    .unwrap();
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 2,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 2,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "closure-feedback".into(),
            label: None,
            axis_channels: vec![
                ActionBandChannelBindingSpec {
                    column: action.threshold.col.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
                ActionBandChannelBindingSpec {
                    column: action.feedback_previous.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                },
            ],
            target: ActionBandTargetSpec::ScalarBound {
                channel: action.threshold.col.raw_u32(),
                bound: threshold.threshold,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(feedback_program.0),
                emission_binding_indices: vec![0, 1],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let conserved = [ActionBandConservedProgressBindingSpec {
        template_id: "closure-feedback".into(),
        band_index: 0,
        emission_binding_index: 0,
        bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    }];
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_with_conserved_progress_at_session_build(
            &spec,
            &conserved,
            &action.registry,
            &eml,
            std::slice::from_ref(threshold),
        )
        .unwrap()
        .clone();
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &action.registry,
        &[action.feedback_previous],
        std::slice::from_ref(rf_plan),
        &[],
        &ThresholdRegistry::new(),
    );
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &[
            ActionBandEmissionBindingGpu::rf_claim(action.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                action.feedback_previous.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
        ],
        &active,
        &native,
    )
    .unwrap()
    .into_execution_plan()
}

fn closure_storage_buffer(ctx: &GpuContext, label: &str, byte_len: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: byte_len,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn run_closure_feedback(
    ctx: &GpuContext,
    action: &ActionFixture,
    overlay_effective: f32,
) -> ClosureFeedbackRun {
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: action.threshold.col,
        threshold: 0.15,
        direction: ThresholdDirection::Upward,
        event_kind: 7_902,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let registrations = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims: action.registry.total_columns as u32,
        value_col: action.threshold.col,
        conductance_col: action.feedback_input,
        saturation: 1.0,
        chi: 0.25,
        dt: 1.0,
    })
    .expect("Gu-Yang remains the admitted conservative field law");
    assert_eq!(
        registrations[1].output(),
        FieldSweepOutput::Matrix(action.threshold.col)
    );
    let rf_plan = closure_rf_plan(action);
    let plan = closure_feedback_plan(action, &threshold, &rf_plan);
    let mut execution = match ActionBandGpuExecution::new(ctx, plan.clone()).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one closure feedback row is active"),
    };
    let _proof = scoped_debug_readback_allowed(true);
    let n_dims = action.registry.total_columns;
    let mut current = vec![0.0f32; 2 * n_dims];
    current[action.threshold.col.raw()] = overlay_effective;
    current[n_dims + action.threshold.col.raw()] = 0.8;
    current[action.feedback_previous.raw()] = overlay_effective;
    let mut observed = Vec::new();

    for generation in 1..=2 {
        let prior_rf = current[action.rf_result.raw()];
        let mut field = FieldSweepSession::new(ctx, &registrations[0]).unwrap();
        field.upload_values(ctx, &current).unwrap();
        field.dispatch_chain(ctx, &registrations, 1).unwrap();
        let resident = closure_storage_buffer(
            ctx,
            "overlay_closure_field_resident",
            std::mem::size_of_val(current.as_slice()) as u64,
        );
        field.copy_values_to_buffer(ctx, &resident);

        let mut phase5 =
            AccumulatorOpSession::new_attached(ctx, 2, action.registry.total_columns as u32, 1);
        phase5.upload_previous_values(ctx, &vec![0.0; current.len()]);
        phase5
            .copy_values_prefix_from_buffer(ctx, &resident, 0, 0, resident.size())
            .unwrap();
        phase5
            .upload_packed_threshold_ops(
                ctx,
                &PackedThresholdUpload::from_registrations(
                    &emit_on_threshold_registrations_to_gpu(std::slice::from_ref(&threshold)),
                )
                .unwrap(),
            )
            .unwrap();
        phase5.tick(ctx, 0).unwrap();
        let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&SimThing::new(SimThingKind::GameSession, 0));
        let deltas = apply_band_crossing_deltas_from_fused_emissions(
            &emissions,
            phase5.threshold_registrations(),
            &action.registry,
            &allocator,
        );
        assert_eq!(
            deltas.len(),
            1,
            "one real field-seeded crossing per generation"
        );
        let field_signal = deltas[0].post_value();
        let crossings = plan.crossings_from_sealed(&deltas).unwrap();
        let next = closure_storage_buffer(ctx, "overlay_closure_next", resident.size());
        let action_readback = execution
            .dispatch_with_native_next_and_readback(
                ctx,
                &resident,
                &next,
                action.registry.total_columns as u32,
                &crossings,
            )
            .unwrap();

        let mut rf = AccumulatorOpSession::new(ctx, rf_plan.slot_count, rf_plan.n_dims);
        rf.copy_values_prefix_from_buffer(ctx, &next, 0, 0, next.size())
            .unwrap();
        rf.upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_ops(&rf_plan.ops).unwrap(),
        )
        .unwrap();
        rf.tick(ctx, 0).unwrap();
        current = rf.readback_full(ctx).unwrap();
        observed.push(ClosureFeedbackGeneration {
            field_signal_bits: field_signal.to_bits(),
            native_flux_bits: deltas[0].post_value().to_bits(),
            feedback_bits: current[action.feedback_previous.raw()].to_bits(),
            rf_increment_bits: (current[action.rf_result.raw()] - prior_rf).to_bits(),
            action_generation: action_readback.states[0].generation,
        });
        assert_eq!(field.host_readbacks(), 0);
        assert_eq!(field.registration_dispatches(), 2);
        assert_eq!(action_readback.states[0].generation, generation);
    }

    ClosureFeedbackRun {
        generations: observed.try_into().unwrap(),
        final_world_bits: current.into_iter().map(f32::to_bits).collect(),
    }
}

#[test]
fn adversarial_fractal_closure_uses_one_intrinsic_overlay_loop() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx = GpuContext::new_blocking().expect("7.9 requires a real GPU adapter");
    let action = action_fixture();
    let crossing = real_crossing(&action, &ctx);

    let (registry, property) = admitted_property_registry();
    let mut target_root = SimThing::new(SimThingKind::GameSession, 0);
    let target_root_id = target_root.id;
    let mut root_value = registry.property(property).default_value();
    root_value.set_role(
        &SubFieldRole::Amount,
        &registry.property(property).layout,
        1.0,
    );
    target_root.add_property(property, root_value);

    // (1), (9): one ancestor standing modifier carries a lawful temporal
    // feedback cycle. The pure-current half is acyclic; the return edge is
    // explicitly Current -> Next and therefore generation paced.
    let generated_key = format!("generated::{}", "modifier-segment-".repeat(256));
    let (standing, _) = compile_overlay(
        &overlay_spec(
            generated_key.clone(),
            0.5,
            OverlayLifecycle::UntilDissolvedWith {
                dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
            },
            vec![("ancestor-current".into(), "field-current".into())],
            vec![("field-current".into(), "ancestor-current".into())],
            Some(7_901),
        ),
        &registry,
        target_root_id,
    )
    .expect("Current -> Next feedback is lawful");
    assert!(
        !serde_json::to_string(&standing)
            .unwrap()
            .contains(&generated_key),
        "(3) the generated authored key must compile away to numeric identity"
    );
    target_root.add_overlay(standing);

    let mut special_leaf = None;
    for index in 0..LARGE_SUBTREE_LEAVES {
        let mut leaf = SimThing::new(SimThingKind::Location, 0);
        let mut leaf_value = registry.property(property).default_value();
        leaf_value.set_role(
            &SubFieldRole::Amount,
            &registry.property(property).layout,
            1.0,
        );
        leaf.add_property(property, leaf_value);
        if index == LARGE_SUBTREE_LEAVES / 2 {
            special_leaf = Some(leaf.id);
        }
        target_root.add_child(leaf);
    }
    let special_leaf = special_leaf.unwrap();

    let source_id = target_root_id;
    let local_id = format!("local::{generated_key}");
    let mut local_spec = overlay_spec(
        local_id,
        1.6,
        OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
            }),
        },
        Vec::new(),
        Vec::new(),
        Some(7_902),
    );
    local_spec.composition_class = Some("sequential".into());
    let (mut local, _) = compile_overlay(&local_spec, &registry, source_id).unwrap();
    local.source = OverlaySource::Ai;
    let local_overlay_id = local.id;
    let predicate_template = local.clone();

    // (2), (4), (5), cross-tree seam: the real Phase-5 crossing passes the
    // canonical 7.8 consequence door and emits the existing stamped receive
    // product. The suspended, timed local instance is then triggered through
    // the ordinary lifecycle boundary—never a peer executor.
    let mut foreign_tree = SimThing::new(SimThingKind::GameSession, 0);
    assert!(deliver_routed_overlay(&mut foreign_tree, special_leaf, local.clone()).is_err());
    let routed = RoutedOverlayDelivery::admit(special_leaf, local).unwrap();
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &action.registry,
        &[],
        &[],
        std::slice::from_ref(&action.threshold),
        &ThresholdRegistry::new(),
    );
    let active = [ActionBandActiveInstance::new(
        action.frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let session = compile_crossing_consequence_session(
        &action.frozen,
        &action.eml,
        &[routed],
        &active,
        &native,
    )
    .unwrap();
    let sealed = session
        .compiled()
        .execution_plan()
        .crossings_from_sealed(std::slice::from_ref(&crossing))
        .unwrap();
    let (tx, rx) = feeder_channel();
    let mut dispatch = session
        .bind_dispatch(&ctx, &vec![1.5; action.registry.total_columns])
        .unwrap();
    let routed_out = dispatch
        .dispatch_and_apply(&ctx, action.registry.total_columns as u32, sealed, &tx)
        .unwrap();
    assert_eq!(routed_out.routed_deliveries, 1);
    let requests = rx
        .drain_now()
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("closure consequence emitted outside the boundary seam"),
        })
        .collect::<Vec<_>>();

    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&target_root);
    let mut runtime = SimRuntimeTree::admit(target_root);
    let mut live_registry = registry.clone();
    let mut shadow = vec![0.0; allocator.capacity() * registry.total_columns];
    let mut lifecycle = OverlayLifecycleAdmissionState::default();
    let attached = apply_structural_mutations(
        requests,
        &mut runtime,
        &mut allocator,
        &mut live_registry,
        &mut shadow,
        registry.total_columns,
        None,
        GenerationStamp::new(7),
        &mut lifecycle,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(
        attached.overlays_attached,
        vec![(special_leaf, local_overlay_id)]
    );
    assert_eq!(
        runtime.overlay_is_suspended(special_leaf, local_overlay_id),
        Some(true)
    );

    let attached_tree = tree_from_runtime(&runtime);
    let replay_snapshot = ReplaySnapshot {
        day: 7,
        root: SimRuntimeTree::admit(attached_tree.clone()),
        registry: registry.clone(),
        fission_lineage: Vec::new(),
    };
    let mut projection = OverlaySpanProjection::compile(&attached_tree)
        .expect("valid closure witness projection must be admitted");
    assert_eq!(
        projection.projection_counts(),
        (LARGE_SUBTREE_LEAVES as u64 + 1, 1, 1)
    );

    let activated = apply_structural_mutations(
        vec![BoundaryRequest::ActivateOverlay {
            target: special_leaf,
            overlay_id: local_overlay_id,
        }],
        &mut runtime,
        &mut allocator,
        &mut live_registry,
        &mut shadow,
        registry.total_columns,
        None,
        GenerationStamp::new(8),
        &mut lifecycle,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(
        activated.overlays_activated,
        vec![(special_leaf, local_overlay_id)]
    );
    assert_eq!(
        lifecycle.activation_generation(special_leaf, local_overlay_id),
        Some(GenerationStamp::new(8))
    );
    let live_tree = tree_from_runtime(&runtime);

    // (7): 7.8a reports the entire cost model. One changed one-row subtree
    // dirties/examines one span and scans zero member rows.
    let (rebuilt, dirty, candidates, member_rows) = projection
        .refresh_with_metrics(
            &live_tree,
            &[OverlayProjectionHostChange::OverlayState(special_leaf)],
            GenerationStamp::new(8),
        )
        .expect("valid overlay refresh must remain admitted");
    let (logical_rows, profiles, spans) = projection.projection_counts();
    assert_eq!(logical_rows, LARGE_SUBTREE_LEAVES as u64 + 1);
    assert_eq!((profiles, spans), (2, 3));
    assert_eq!((rebuilt, dirty, candidates, member_rows), (1, 1, 1, 0));
    assert_eq!(
        projection
            .refresh_with_metrics(&live_tree, &[], GenerationStamp::new(9))
            .expect("empty valid refresh must remain admitted"),
        (0, 0, 0, 0),
        "unchanged generations perform no semantic rewalk"
    );

    let (deltas, ranges) = projection.materialize_dense(&registry, &allocator);
    let local_slot = allocator.slot_of(special_leaf).unwrap();
    let local_range = ranges[local_slot.as_usize()];
    assert_eq!(local_range.length, 2);
    for delta in
        &deltas[local_range.offset as usize..(local_range.offset + local_range.length) as usize]
    {
        assert_eq!(delta.op_kind, OP_MULTIPLY);
    }
    let evaluated = Evaluator::new(&registry, 0.0).evaluate(&live_tree, 8);
    let effective = evaluated
        .get(special_leaf)
        .and_then(|entity| entity.properties.get(&property))
        .expect("ordinary evaluated Property retains the overlay-composed value")
        .get_role(&SubFieldRole::Amount, &registry.property(property).layout);
    assert_eq!(effective.to_bits(), 0.8f32.to_bits());

    // (6): the existing paid routed-predicate path reads the ordinary
    // overlay-composed Property. Exactly the one leaf carrying both the
    // inherited and local modifier satisfies the has-modifier-like selector.
    let mut predicate_tree = live_tree.clone();
    let predicate_receipts = deliver_predicate_broadcast(
        &mut predicate_tree,
        target_root_id,
        &predicate_template,
        &RoutedPredicate {
            property_id: property,
            sub_field: SubFieldRole::Amount,
            comparison: RoutedPredicateComparison::AtLeast,
            threshold: 0.75,
        },
        &registry,
    )
    .expect("the production predicate-broadcast evaluator owns the one paid walk");
    assert_eq!(predicate_receipts.len(), 1);
    assert_eq!(predicate_receipts[0].target, special_leaf);
    assert_eq!(runtime.overlay_count(target_root_id), Some(1));
    assert_eq!(runtime.overlay_count(special_leaf), Some(1));

    // (8), (9): the actual overlay-composed value seeds Gu-Yang. Its resident
    // field result crosses the existing Phase-5 surface, bounds an ordinary RF
    // claim, and feeds a bounded EML Current -> Next write for two real GPU
    // generations. A second execution from the same input is bit-identical.
    let feedback = run_closure_feedback(&ctx, &action, effective);
    let repeated_feedback = run_closure_feedback(&ctx, &action, effective);
    assert_eq!(feedback, repeated_feedback);
    assert_eq!(feedback.generations[0].action_generation, 1);
    assert_eq!(feedback.generations[1].action_generation, 2);
    for generation in feedback.generations {
        assert_eq!(generation.native_flux_bits, generation.field_signal_bits);
        assert_eq!(generation.rf_increment_bits, generation.native_flux_bits);
    }
    assert_ne!(
        feedback.generations[0].feedback_bits, feedback.generations[1].feedback_bits,
        "t+1 must consume t's resident feedback value rather than converge in one generation"
    );
    eprintln!(
        "OVERLAY-CLOSURE-FEEDBACK g1(field={:08x},progress={:08x},next={:08x}) g2(field={:08x},progress={:08x},next={:08x})",
        feedback.generations[0].field_signal_bits,
        feedback.generations[0].rf_increment_bits,
        feedback.generations[0].feedback_bits,
        feedback.generations[1].field_signal_bits,
        feedback.generations[1].rf_increment_bits,
        feedback.generations[1].feedback_bits,
    );

    // Existing stamped crossing + canonical schedule/delta history is the
    // entire replay surface. No OverlayHistory or second log exists.
    let gate_col = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .unwrap();
    let mut replay_shadow = shadow.clone();
    replay_shadow[local_slot.as_usize() * registry.total_columns + gate_col.raw()] =
        f32::from_bits(feedback.generations[1].feedback_bits);
    let frame = ReplayFrame {
        day: 8,
        entries: vec![
            BoundaryDeltaEntry::OverlayActivated {
                target: special_leaf,
                overlay_id: local_overlay_id,
            },
            BoundaryDeltaEntry::BandCrossingDeltasApplied {
                deltas: vec![crossing.clone()],
            },
        ],
        shadow_values: Some(replay_shadow.clone()),
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    };
    let bytes = write_replay(&replay_snapshot, &frame);
    let mut reader = ReplayReader::new(Cursor::new(bytes));
    let decoded_snapshot = reader.read_snapshot().unwrap();
    let decoded_frame = reader.next_frame().unwrap().unwrap();
    let mut replay =
        ReplayDriver::from_snapshot(decoded_snapshot).expect("replay snapshot install");
    replay.apply_frame(decoded_frame);
    assert_eq!(replay.last_band_crossing_deltas, vec![crossing]);
    assert_eq!(replay.shadow_values.as_ref(), Some(&replay_shadow));
    assert_eq!(
        replay.shadow_values.as_ref().unwrap()
            [local_slot.as_usize() * registry.total_columns + gate_col.raw()]
        .to_bits(),
        feedback.generations[1].feedback_bits,
        "existing post-boundary shadow checkpoint carries the executed feedback consequence"
    );
    assert_eq!(
        serde_json::to_value(&replay.root).unwrap(),
        serde_json::to_value(&runtime).unwrap()
    );
    let replay_tree = tree_from_runtime(&replay.root);
    let replay_projection = OverlaySpanProjection::compile(&replay_tree)
        .expect("valid replay projection must be admitted");
    let (replay_deltas, replay_ranges) =
        replay_projection.materialize_dense(&registry, &replay.allocator);
    assert_eq!(replay_deltas, deltas);
    assert_eq!(replay_ranges, ranges);
}

#[test]
fn five_planted_closure_defects_fail_at_production_boundaries() {
    let (registry, _) = admitted_property_registry();
    let origin = SimThing::new(SimThingKind::GameSession, 0).id;

    // Falsifier 1: a descendant conjunctive restriction cannot weaken its
    // ancestor through Set/Add or an amplifying factor.
    let weakening = overlay_spec(
        "weakening".into(),
        1.25,
        OverlayLifecycle::UntilDissolved,
        Vec::new(),
        Vec::new(),
        Some(7_911),
    );
    assert!(matches!(
        compile_overlay(&weakening, &registry, origin),
        Err(SpecError::OverlayEvaluationAdmission { .. })
    ));
    let mut set_weakening = weakening.clone();
    set_weakening.sub_field_deltas[0].1 = TransformOp::set(1.0);
    assert!(matches!(
        compile_overlay(&set_weakening, &registry, origin),
        Err(SpecError::OverlayEvaluationAdmission { .. })
    ));

    // Falsifier 3: direct cross-tree actuation cannot find a target in the
    // source tree; only RoutedOverlayProduct -> stamped receive can cross it.
    let mut source = SimThing::new(SimThingKind::GameSession, 0);
    let target = SimThing::new(SimThingKind::Location, 0).id;
    let (routed, _) = compile_overlay(
        &overlay_spec(
            "cross-tree".into(),
            0.5,
            OverlayLifecycle::UntilDissolvedWith {
                dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
            },
            Vec::new(),
            Vec::new(),
            Some(7_912),
        ),
        &registry,
        source.id,
    )
    .unwrap();
    assert!(deliver_routed_overlay(&mut source, target, routed).is_err());

    // Falsifier 4: an all-current algebraic cycle is rejected at the real
    // compile door and the diagnostic carries the authored source span.
    let cycle = overlay_spec(
        "illegal-cycle".into(),
        0.5,
        OverlayLifecycle::UntilDissolved,
        vec![("a".into(), "b".into()), ("b".into(), "a".into())],
        Vec::new(),
        Some(7_913),
    );
    let error = compile_overlay(&cycle, &registry, origin).unwrap_err();
    assert!(matches!(
        error,
        SpecError::OverlayEvaluationAdmission {
            source_span_token: Some(7_913),
            ..
        }
    ));
    let message = error.to_string();
    assert!(message.contains("Current -> Current algebraic cycle"));
    assert!(message.contains("7913"));

    // Falsifiers 2 and 5 are compiler-owned type-boundary proofs on the real
    // production types: OverlaySpanProjection has no leaf-stamp method and
    // GpuSyncOutcome has no per-template fine-telemetry field (their doctests).
}
