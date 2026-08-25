//! ACTIONBAND-OVERLAY-ACTUATION-0: one sealed crossing consequence door.

use std::sync::Mutex;

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    ColumnIndex, DimensionRegistry, DissolveCondition, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlTreeId, GenerationStamp, GenerationStamped, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimThing, SimThingKind,
    SlotIndex, SubFieldRole, ThresholdDirection, TransformOp,
};
use simthing_driver::{
    compile_crossing_consequence_session, compile_gu_yang_overlay_parameterized_n4_field_sweeps,
    compile_palma_overlay_parameterized_n4_field_sweep,
    compile_stead_overlay_parameterized_n4_field_sweep, ActionBandActiveInstance,
    ActionBandNativeLaneAdmission, CrossingConsequenceBinding, GuYangOverlayParameterizedN4Spec,
    PalmaOverlayParameterizedN4Spec, RoutedOverlayDelivery, RoutedOverlayProduct,
    SteadOverlayParameterizedN4Spec, StructuralAuthorization,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, AccumulatorOpSession, ActionBandEmissionBindingGpu, GpuContext,
    PackedThresholdUpload, SlotAllocator,
};
use simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState;
use simthing_sim::{apply_structural_mutations, SimRuntimeTree, ThresholdRegistry};
use simthing_spec::{
    compile_eml_gadget, ActionBandAdmissionBudgetSpec, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandSessionBuildDoor,
    ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec, EmlGadgetCompileOptions,
    EmlGadgetInstanceSpec, ScalarBoundDirection,
};

static GPU: Mutex<()> = Mutex::new(());

struct Fixture {
    registry: DimensionRegistry,
    thresholds: Vec<EmitOnThresholdRegistration>,
    eml: EmlExpressionRegistry,
    column: ColumnIndex,
    stead_falloff: ColumnIndex,
    palma_w: ColumnIndex,
    palma_terminal: ColumnIndex,
    guyang_conductance_input: ColumnIndex,
    guyang_capacity: ColumnIndex,
    stead_output: ColumnIndex,
    palma_d: ColumnIndex,
    guyang_conductance_output: ColumnIndex,
    frozen: simthing_spec::FrozenActionBandTemplates,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("actuation-proof", "parameter", 5));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .unwrap();
    let parameter_col = |registry: &DimensionRegistry, index: usize| {
        registry
            .column_range(property)
            .col_for_role(
                &SubFieldRole::Named(format!("vec_{index}")),
                &registry.property(property).layout,
            )
            .unwrap()
    };
    let stead_falloff = parameter_col(&registry, 0);
    let palma_w = parameter_col(&registry, 1);
    let palma_terminal = parameter_col(&registry, 2);
    let guyang_conductance_input = parameter_col(&registry, 3);
    let guyang_capacity = parameter_col(&registry, 4);
    let mut output_col = |name: &str| {
        let id = registry.register(SimProperty::simple("actuation-proof", name, 0));
        registry
            .column_range(id)
            .col_for_role(&SubFieldRole::Amount, &registry.property(id).layout)
            .unwrap()
    };
    let stead_output = output_col("stead-output");
    let palma_d = output_col("palma-d");
    let guyang_conductance_output = output_col("guyang-conductance-output");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 7801,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    let feedback = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "actionband-7-8-field-seeded-feedback".into(),
        previous_col: column.raw_u32(),
        input_col: stead_output.raw_u32(),
        output_col: Some(column.raw_u32()),
        decay: 0.5,
        gain: 0.5,
        min: 0.0,
        max: 4.0,
    };
    let compiled_feedback = compile_eml_gadget(
        &feedback,
        EmlGadgetCompileOptions {
            max_col: registry.total_columns as u32,
        },
    )
    .expect("bounded feedback is admitted before becoming the ActionBand program");
    let mut eml = EmlExpressionRegistry::new();
    let program = EmlTreeId(7801);
    let nodes = compiled_feedback.nodes;
    eml.register_formula(
        program,
        EmlFormulaMeta {
            tree_id: program,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 3,
            has_loops: false,
            has_recursion: false,
            display_name: "shared-actionband-actuation-program".into(),
        },
        nodes,
    )
    .unwrap();
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "actuation-proof".into(),
            label: Some("presentation only".into()),
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: column.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: column.raw_u32(),
                bound: 2.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(program.0),
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(&spec, &registry, &eml, &thresholds)
        .unwrap()
        .clone();
    Fixture {
        registry,
        thresholds,
        eml,
        column,
        stead_falloff,
        palma_w,
        palma_terminal,
        guyang_conductance_input,
        guyang_capacity,
        stead_output,
        palma_d,
        guyang_conductance_output,
        frozen,
    }
}

fn active(fx: &Fixture) -> [ActionBandActiveInstance; 1] {
    [ActionBandActiveInstance::new(
        fx.frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )]
}

fn native_lanes(fx: &Fixture) -> ActionBandNativeLaneAdmission {
    ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fx.registry,
        &[fx.column],
        &[],
        &fx.thresholds,
        &ThresholdRegistry::new(),
    )
}

fn real_gpu_crossing(fx: &Fixture, ctx: &GpuContext) -> simthing_gpu::BandCrossingDelta {
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&SimThing::new(SimThingKind::GameSession, 0));
    let mut previous = vec![0.0; fx.registry.total_columns];
    let mut current = previous.clone();
    previous[fx.column.raw()] = 0.5;
    current[fx.column.raw()] = 1.5;
    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, 1, fx.registry.total_columns as u32, 2);
    phase5.upload_values(ctx, &current);
    phase5.upload_previous_values(ctx, &previous);
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&emit_on_threshold_registrations_to_gpu(
                &fx.thresholds,
            ))
            .unwrap(),
        )
        .unwrap();
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fx.registry,
        &allocator,
    )
    .into_iter()
    .next()
    .expect("the existing Phase-5 GPU crossing is the only ingress")
}

fn resident_values(fx: &Fixture) -> Vec<f32> {
    let mut values = vec![0.0; fx.registry.total_columns];
    values[fx.column.raw()] = 1.5;
    values[fx.stead_output.raw()] = 2.0;
    values
}

#[test]
fn one_real_gpu_door_executes_all_three_consequence_arms() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.8 requires a real GPU adapter; skips are forbidden");
    let fx = fixture();
    let delta = real_gpu_crossing(&fx, &ctx);
    let lanes = native_lanes(&fx);

    // ResidentNextWrite: the binding is admitted from an existing native lane,
    // retains logical property/role identity, and writes only Next at t.
    let resident = lanes
        .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
            fx.column.raw_u32(),
            simthing_gpu::ActionBandPropertyWrite::Set,
        ))
        .unwrap();
    let resident_session = compile_crossing_consequence_session(
        &fx.frozen,
        &fx.eml,
        &[resident.clone()],
        &active(&fx),
        &lanes,
    )
    .unwrap();
    let CrossingConsequenceBinding::ResidentNextWrite(resident_identity) = resident else {
        unreachable!()
    };
    assert_eq!(resident_identity.role(), &SubFieldRole::Amount);
    let resident_crossings = resident_session
        .compiled()
        .execution_plan()
        .crossings_from_sealed(std::slice::from_ref(&delta))
        .unwrap();
    let replay_crossings = resident_session
        .compiled()
        .execution_plan()
        .crossings_from_sealed(std::slice::from_ref(&delta))
        .expect("a cloned sealed delta can mint a rival batch, but becomes stale at the boundary");
    let (tx, _rx) = feeder_channel();
    let initial_resident = resident_values(&fx);
    let mut dispatch = resident_session
        .bind_dispatch(&ctx, &initial_resident)
        .unwrap();
    let resident_outcome = dispatch
        .dispatch_and_apply(
            &ctx,
            fx.registry.total_columns as u32,
            resident_crossings,
            &tx,
        )
        .unwrap();
    assert_eq!(resident_outcome.generation, GenerationStamp::new(1));
    assert_eq!(resident_outcome.routed_deliveries, 0);
    assert_eq!(resident_outcome.structural_authorizations, 0);
    let _proof = scoped_debug_readback_allowed(true);
    let values = dispatch.resident_current_for_proof(&ctx).unwrap();
    assert_eq!(values[fx.column.raw()].to_bits(), 1.75f32.to_bits());
    assert_eq!(dispatch.generation(), 1);
    assert_eq!(
        dispatch.generation_dedupe_for_proof().unwrap(),
        (Some(1), 0),
        "the real resident-plane boundary must drop every prior-generation key immediately"
    );
    assert!(matches!(
        dispatch.dispatch_and_apply(
            &ctx,
            fx.registry.total_columns as u32,
            replay_crossings,
            &tx,
        ),
        Err(
            simthing_driver::CrossingConsequenceDispatchError::CrossingGenerationMismatch {
                expected: 1,
                actual: 0,
            }
        )
    ));
    assert_eq!(
        dispatch.generation_dedupe_for_proof().unwrap(),
        (Some(1), 0)
    );

    // RoutedOverlayDelivery: only authored duration + sealed source provenance
    // leave the new arm. Destination generation 7 establishes the activation
    // epoch and therefore deadline 11, not a source-relative absolute 5.
    let mut route_root = SimThing::new(SimThingKind::World, 0);
    let mut policy_host = SimThing::new(SimThingKind::Location, 0);
    let origin_node = SimThing::new(SimThingKind::Cohort, 0);
    let origin = origin_node.id;
    policy_host.add_child(origin_node);
    let policy_host_id = policy_host.id;
    let property = resident_identity.property_id();
    policy_host.add_overlay(Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin: policy_host_id,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(0.5))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    });
    let mut target_node = SimThing::new(SimThingKind::Cohort, 0);
    let target = target_node.id;
    let mut target_value = fx.registry.property(property).default_value();
    target_value.set_role(
        &SubFieldRole::Amount,
        &fx.registry.property(property).layout,
        0.2,
    );
    target_node.add_property(property, target_value);
    route_root.add_child(policy_host);
    route_root.add_child(target_node);
    let overlay_id = OverlayId::new();
    let overlay = Overlay {
        id: overlay_id,
        kind: OverlayKind::Instruction,
        source: OverlaySource::Ai,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![
                (SubFieldRole::Amount, TransformOp::add(0.4)),
                (SubFieldRole::Named("vec_0".into()), TransformOp::set(0.5)),
                (SubFieldRole::Named("vec_1".into()), TransformOp::set(1.0)),
                (SubFieldRole::Named("vec_2".into()), TransformOp::set(0.0)),
                (SubFieldRole::Named("vec_3".into()), TransformOp::set(0.75)),
                (SubFieldRole::Named("vec_4".into()), TransformOp::set(1.0)),
            ],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 4 }],
        },
    };
    let routed = RoutedOverlayDelivery::admit(target, overlay.clone()).unwrap();
    let routed_session =
        compile_crossing_consequence_session(&fx.frozen, &fx.eml, &[routed], &active(&fx), &lanes)
            .unwrap();
    let routed_crossings = routed_session
        .compiled()
        .execution_plan()
        .crossings_from_sealed(std::slice::from_ref(&delta))
        .unwrap();
    let (tx, rx) = feeder_channel();
    let initial_resident = resident_values(&fx);
    let mut dispatch = routed_session
        .bind_dispatch(&ctx, &initial_resident)
        .unwrap();
    let routed_outcome = dispatch
        .dispatch_and_apply(
            &ctx,
            fx.registry.total_columns as u32,
            routed_crossings,
            &tx,
        )
        .unwrap();
    assert_eq!(routed_outcome.routed_deliveries, 1);
    let requests = rx
        .drain_now()
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("consequence door emitted a non-boundary work item"),
        })
        .collect::<Vec<_>>();
    let BoundaryRequest::AttachOverlay {
        source_generation, ..
    } = &requests[0]
    else {
        panic!("routed arm must use the existing attach boundary")
    };
    assert_eq!(*source_generation, GenerationStamp::new(1));
    let mut route_allocator = SlotAllocator::new();
    route_allocator.install_initial_tree(&route_root);
    let mut route_runtime = SimRuntimeTree::admit(route_root);
    let mut route_registry = fx.registry.clone();
    let mut shadow = vec![0.0; route_allocator.capacity() * route_registry.total_columns];
    let mut lifecycle = OverlayLifecycleAdmissionState::default();
    let applied = apply_structural_mutations(
        requests,
        &mut route_runtime,
        &mut route_allocator,
        &mut route_registry,
        &mut shadow,
        fx.registry.total_columns,
        None,
        GenerationStamp::new(7),
        &mut lifecycle,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(applied.overlays_attached, vec![(target, overlay_id)]);
    assert_eq!(
        lifecycle.routed_provenance(target, overlay_id),
        Some(GenerationStamp::new(1))
    );
    assert_eq!(
        lifecycle.activation_generation(target, overlay_id),
        Some(GenerationStamp::new(7))
    );
    assert_eq!(
        simthing_core::rebase_routed_overlay_duration(
            simthing_core::RoutedGenerationDuration::new(4, GenerationStamp::new(1)),
            lifecycle.activation_generation(target, overlay_id).unwrap(),
        )
        .unwrap(),
        GenerationStamp::new(11)
    );
    let received_tree: SimThing = serde_json::from_value(
        serde_json::to_value(&route_runtime).expect("serialize the authoritative receive tree"),
    )
    .expect("inspect the authoritative receive tree through its public serialized shape");
    let evaluated = Evaluator::new(&route_registry, 0.0).evaluate(&received_tree, 0);
    let received_amount = evaluated
        .get(target)
        .and_then(|entity| entity.properties.get(&property))
        .expect("received target retains the routed property")
        .get_role(
            &SubFieldRole::Amount,
            &route_registry.property(property).layout,
        );
    assert_eq!(
        received_amount.to_bits(),
        0.3f32.to_bits(),
        "origin→policy-host→LCA→target traversal must preserve the live route filter; direct target append yields 0.6"
    );

    // The same routed overlay's ordinary logical sub-fields are valid inputs
    // to all three existing generic field-sweep registrations. Only values
    // vary; adjacency, canonical order, conservation, symmetry, and chi proofs
    // are minted by the existing admission door and remain frozen.
    assert_eq!(overlay.transform.sub_field_deltas.len(), 6);
    let field_seed =
        compile_stead_overlay_parameterized_n4_field_sweep(SteadOverlayParameterizedN4Spec {
            width: 2,
            height: 1,
            n_dims: fx.registry.total_columns as u32,
            source_col: fx.column,
            falloff_col: fx.stead_falloff,
            output_col: fx.stead_output,
            dt: 1.0,
        })
        .expect("one overlay parameterizes STEAD source/falloff inside the generic sweep");
    assert_eq!(
        field_seed.output(),
        simthing_gpu::FieldSweepOutput::Matrix(fx.stead_output),
        "the ActionBand bounded-feedback input is the actual admitted field output"
    );
    compile_palma_overlay_parameterized_n4_field_sweep(PalmaOverlayParameterizedN4Spec {
        width: 2,
        height: 1,
        n_dims: fx.registry.total_columns as u32,
        d_col: fx.palma_d,
        w_col: fx.palma_w,
        terminal_value_col: fx.palma_terminal,
        destination_slot: SlotIndex::new(1),
        inf_sentinel: simthing_gpu::MIN_PLUS_INF,
    })
    .expect("one overlay parameterizes PALMA W/terminal value inside the generic sweep");
    compile_gu_yang_overlay_parameterized_n4_field_sweeps(GuYangOverlayParameterizedN4Spec {
        width: 2,
        height: 1,
        n_dims: fx.registry.total_columns as u32,
        value_col: fx.column,
        conductance_input_col: fx.guyang_conductance_input,
        conductance_output_col: fx.guyang_conductance_output,
        capacity_col: fx.guyang_capacity,
        chi: 0.5,
        dt: 1.0,
    })
    .expect("one overlay parameterizes Gu-Yang conductance/capacity inside its certificate");

    // StructuralAuthorization: the same sealed packet authorizes only the
    // existing Reparent boundary verb; no GPU state-plane target is expressible.
    let mut structural_root = SimThing::new(SimThingKind::GameSession, 0);
    let mut first = SimThing::new(SimThingKind::Location, 0);
    let child = SimThing::new(SimThingKind::Custom("actor".into()), 0);
    let child_id = child.id;
    first.add_child(child);
    let first_id = first.id;
    let second = SimThing::new(SimThingKind::Location, 0);
    let second_id = second.id;
    structural_root.add_child(first);
    structural_root.add_child(second);
    let structural = StructuralAuthorization::admit(BoundaryRequest::Reparent {
        child: child_id,
        new_parent: second_id,
    })
    .unwrap();
    let structural_session = compile_crossing_consequence_session(
        &fx.frozen,
        &fx.eml,
        &[structural],
        &active(&fx),
        &lanes,
    )
    .unwrap();
    let structural_crossings = structural_session
        .compiled()
        .execution_plan()
        .crossings_from_sealed(std::slice::from_ref(&delta))
        .unwrap();
    let (tx, rx) = feeder_channel();
    let initial_resident = resident_values(&fx);
    let mut dispatch = structural_session
        .bind_dispatch(&ctx, &initial_resident)
        .unwrap();
    let structural_outcome = dispatch
        .dispatch_and_apply(
            &ctx,
            fx.registry.total_columns as u32,
            structural_crossings,
            &tx,
        )
        .unwrap();
    assert_eq!(structural_outcome.structural_authorizations, 1);
    let requests = rx
        .drain_now()
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("structural arm emitted a non-boundary work item"),
        })
        .collect::<Vec<_>>();
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&structural_root);
    let mut runtime = SimRuntimeTree::admit(structural_root);
    let mut registry = fx.registry.clone();
    let mut shadow = vec![0.0; allocator.capacity() * registry.total_columns];
    let mut lifecycle = OverlayLifecycleAdmissionState::default();
    let applied = apply_structural_mutations(
        requests,
        &mut runtime,
        &mut allocator,
        &mut registry,
        &mut shadow,
        fx.registry.total_columns,
        None,
        GenerationStamp::new(1),
        &mut lifecycle,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(applied.reparented, vec![(child_id, second_id)]);
    assert_eq!(first_id.raw() != second_id.raw(), true);
}

#[test]
fn forbidden_overlay_and_state_plane_shapes_are_rejected_by_the_real_door() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let _ctx =
        GpuContext::new_blocking().expect("7.8 falsifiers require a real GPU adapter; no skips");
    let fx = fixture();
    let lanes = native_lanes(&fx);

    let foreign_column = fx.registry.total_columns as u32 + 1;
    let foreign = lanes.bind_resident_next(ActionBandEmissionBindingGpu::property_next(
        foreign_column,
        simthing_gpu::ActionBandPropertyWrite::Set,
    ));
    assert!(matches!(
        foreign,
        Err(simthing_driver::CrossingConsequenceAdmissionError::UnadmittedResidentLane { .. })
    ));

    let target = SimThing::new(SimThingKind::World, 0).id;
    let overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin: target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: fx.registry.column_owners[fx.column.raw()].0,
            sub_field_deltas: vec![],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    };
    assert!(matches!(
        StructuralAuthorization::admit(BoundaryRequest::AttachOverlay {
            target,
            overlay,
            source_generation: GenerationStamp::new(99),
        }),
        Err(simthing_driver::CrossingConsequenceAdmissionError::NonStructuralBoundaryVerb)
    ));

    assert!(matches!(
        StructuralAuthorization::admit(BoundaryRequest::AddDimension {
            property: fx.registry.column_owners[fx.column.raw()].0,
        }),
        Err(simthing_driver::CrossingConsequenceAdmissionError::NonStructuralBoundaryVerb)
    ));

    // A resident binding admitted by an otherwise identical sibling native
    // facility must RED at the actual consequence compile door.
    let foreign_lanes = native_lanes(&fx);
    let foreign_resident = foreign_lanes
        .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
            fx.column.raw_u32(),
            simthing_gpu::ActionBandPropertyWrite::Set,
        ))
        .unwrap();
    assert!(matches!(
        compile_crossing_consequence_session(
            &fx.frozen,
            &fx.eml,
            &[foreign_resident],
            &active(&fx),
            &lanes,
        ),
        Err(simthing_driver::CrossingConsequenceAdmissionError::ForeignResidentLaneAdmission)
    ));

    // DA A1: the real generation-stamped routed carrier rejects a planted
    // source-relative absolute deadline before it reaches boundary ingress.
    let deadline_overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin: target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: fx.registry.column_owners[fx.column.raw()].0,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 4 }],
        },
    };
    let deadline_binding = RoutedOverlayDelivery::admit(target, deadline_overlay).unwrap();
    let CrossingConsequenceBinding::RoutedOverlayDelivery(deadline_route) = deadline_binding else {
        unreachable!()
    };
    let mut planted =
        serde_json::to_value(deadline_route.stamped_product(GenerationStamp::new(1))).unwrap();
    planted["product"]
        .as_object_mut()
        .unwrap()
        .insert("foreign_absolute_deadline".into(), serde_json::json!(5));
    assert!(
        serde_json::from_value::<GenerationStamped<RoutedOverlayProduct>>(planted).is_err(),
        "the production routed carrier must RED a foreign source-relative deadline"
    );

    // Certificate-envelope mutation: chi is an admission certificate bound,
    // not a runtime parameter that an overlay may widen.
    assert!(compile_gu_yang_overlay_parameterized_n4_field_sweeps(
        GuYangOverlayParameterizedN4Spec {
            width: 2,
            height: 1,
            n_dims: fx.registry.total_columns as u32,
            value_col: fx.column,
            conductance_input_col: fx.guyang_conductance_input,
            conductance_output_col: fx.guyang_conductance_output,
            capacity_col: fx.guyang_capacity,
            chi: 1.25,
            dt: 1.0,
        }
    )
    .is_err());

    // Mutate the same field-seeded feedback admission used by the positive
    // consequence chain into an unbounded form.
    let unbounded = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "actionband-7-8-unbounded-field-feedback".into(),
        previous_col: fx.column.raw_u32(),
        input_col: fx.stead_output.raw_u32(),
        output_col: Some(fx.column.raw_u32()),
        decay: 1.0,
        gain: 0.5,
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
    let error = compile_eml_gadget(
        &unbounded,
        EmlGadgetCompileOptions {
            max_col: fx.registry.total_columns as u32,
        },
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("0 <= decay < 1"),
        "the same admission must RED specifically when feedback becomes unbounded: {error}"
    );
}
