//! OVERLAY-FRACTAL-CLOSURE-WITNESS-0: one adversarial closure scenario.

use std::io::Cursor;
use std::sync::Mutex;

use simthing_core::{
    deliver_routed_overlay, DimensionRegistry, DissolveCondition, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlExpressionRegistry, GenerationStamp, OverlayKind,
    OverlayLifecycle, OverlaySource, SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole,
    ThresholdDirection, TransformOp,
};
use simthing_driver::{
    compile_crossing_consequence_session, compile_gu_yang_n4_field_sweeps,
    ActionBandActiveInstance, ActionBandNativeLaneAdmission, GuYangN4FieldSweepSpec,
    RoutedOverlayDelivery,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    AccumulatorOpSession, GpuContext, OverlayProjectionHostChange, OverlaySpanProjection,
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
    feedback_output: simthing_core::ColumnIndex,
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
    let feedback_output_property =
        registry.register(SimProperty::simple("closure", "feedback-output", 0));
    let feedback_output = registry
        .column_range(feedback_output_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(feedback_output_property).layout,
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
        feedback_output,
        eml,
        frozen,
    }
}

fn real_crossing(fixture: &ActionFixture, ctx: &GpuContext) -> simthing_gpu::BandCrossingDelta {
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&SimThing::new(SimThingKind::GameSession, 0));
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

#[test]
fn adversarial_fractal_closure_uses_one_intrinsic_overlay_loop() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx = GpuContext::new_blocking().expect("7.9 requires a real GPU adapter");
    let action = action_fixture();
    let crossing = real_crossing(&action, &ctx);

    let (registry, property) = admitted_property_registry();
    let mut target_root = SimThing::new(SimThingKind::GameSession, 0);
    let target_root_id = target_root.id;
    target_root.add_property(property, registry.property(property).default_value());

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
        leaf.add_property(property, registry.property(property).default_value());
        if index == LARGE_SUBTREE_LEAVES / 2 {
            special_leaf = Some(leaf.id);
        }
        target_root.add_child(leaf);
    }
    let special_leaf = special_leaf.unwrap();

    let source_id = target_root_id;
    let local_id = format!("local::{generated_key}");
    let (mut local, _) = compile_overlay(
        &overlay_spec(
            local_id,
            0.8,
            OverlayLifecycle::Suspended {
                when_activated: Box::new(OverlayLifecycle::Transient {
                    dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
                }),
            },
            Vec::new(),
            Vec::new(),
            Some(7_902),
        ),
        &registry,
        source_id,
    )
    .unwrap();
    local.source = OverlaySource::Ai;
    let local_overlay_id = local.id;

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
    allocator.populate_from_tree(&target_root);
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
    let mut projection = OverlaySpanProjection::compile(&attached_tree);
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
    let (rebuilt, dirty, candidates, member_rows) = projection.refresh_with_metrics(
        &live_tree,
        &[OverlayProjectionHostChange::OverlayState(special_leaf)],
        GenerationStamp::new(8),
    );
    let (logical_rows, profiles, spans) = projection.projection_counts();
    assert_eq!(logical_rows, LARGE_SUBTREE_LEAVES as u64 + 1);
    assert_eq!((profiles, spans), (2, 3));
    assert_eq!((rebuilt, dirty, candidates, member_rows), (1, 1, 1, 0));
    assert_eq!(
        projection.refresh_with_metrics(&live_tree, &[], GenerationStamp::new(9)),
        (0, 0, 0, 0),
        "unchanged generations perform no semantic rewalk"
    );

    let (deltas, ranges) = projection.materialize_dense(&registry, &allocator);
    let local_slot = allocator.slot_of(special_leaf).unwrap();
    let local_range = ranges[local_slot.as_usize()];
    assert_eq!(local_range.length, 2);
    let mut effective = 1.0f32;
    for delta in
        &deltas[local_range.offset as usize..(local_range.offset + local_range.length) as usize]
    {
        assert_eq!(delta.op_kind, OP_MULTIPLY);
        effective *= delta.value;
    }
    assert_eq!(effective.to_bits(), 0.4f32.to_bits());
    assert!(
        effective < 0.45,
        "(6) active-state numeric projection feeds the admitted predicate"
    );
    assert_eq!(runtime.overlay_count(target_root_id), Some(1));
    assert_eq!(runtime.overlay_count(special_leaf), Some(1));

    // (8): RF and Gu-Yang remain the native bound/input authorities, while
    // recurrence is the existing bounded-feedback EML shape.
    let field_columns = action.registry.total_columns as u32;
    compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims: field_columns,
        value_col: action.threshold.col,
        conductance_col: action.feedback_input,
        saturation: 1.0,
        chi: 0.25,
        dt: 1.0,
    })
    .expect("Gu-Yang remains the admitted conservative field law");
    compile_eml_gadget(
        &EmlGadgetInstanceSpec::BoundedFeedback {
            id: "closure-field-rf-feedback".into(),
            previous_col: action.threshold.col.raw_u32(),
            input_col: action.feedback_input.raw_u32(),
            output_col: Some(action.feedback_output.raw_u32()),
            decay: 0.5,
            gain: 0.25,
            min: 0.0,
            max: 1.0,
        },
        EmlGadgetCompileOptions {
            max_col: field_columns,
        },
    )
    .expect("bounded Current -> Next field/RF recurrence");
    let mut conserved_door = ActionBandSessionBuildDoor::new();
    let conserved = [ActionBandConservedProgressBindingSpec {
        template_id: "closure-trigger".into(),
        band_index: 0,
        emission_binding_index: 0,
        bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    }];
    conserved_door
        .admit_once_with_conserved_progress_at_session_build(
            &ActionBandSessionSpec {
                budget: ActionBandAdmissionBudgetSpec {
                    axis_channel_count: 1,
                    dependency_binding_count: 0,
                    storage_rows: 1,
                    eml_program_count: 0,
                    emission_binding_count: 1,
                },
                templates: vec![ActionBandTemplateSpec {
                    id: "closure-trigger".into(),
                    label: None,
                    axis_channels: vec![ActionBandChannelBindingSpec {
                        column: action.threshold.col.raw_u32(),
                        kind: ActionBandChannelKind::Primitive,
                    }],
                    target: ActionBandTargetSpec::ScalarBound {
                        channel: action.threshold.col.raw_u32(),
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
            },
            &conserved,
            &action.registry,
            &action.eml,
            std::slice::from_ref(&action.threshold),
        )
        .expect("one existing Gu-Yang/RF bound source is frozen at admission");

    // Existing stamped crossing + canonical schedule/delta history is the
    // entire replay surface. No OverlayHistory or second log exists.
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
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    };
    let bytes = write_replay(&replay_snapshot, &frame);
    let mut reader = ReplayReader::new(Cursor::new(bytes));
    let decoded_snapshot = reader.read_snapshot().unwrap();
    let decoded_frame = reader.next_frame().unwrap().unwrap();
    let mut replay = ReplayDriver::from_snapshot(decoded_snapshot);
    replay.apply_frame(decoded_frame);
    assert_eq!(replay.last_band_crossing_deltas, vec![crossing]);
    assert_eq!(
        serde_json::to_value(&replay.root).unwrap(),
        serde_json::to_value(&runtime).unwrap()
    );
    let replay_tree = tree_from_runtime(&replay.root);
    let replay_projection = OverlaySpanProjection::compile(&replay_tree);
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
