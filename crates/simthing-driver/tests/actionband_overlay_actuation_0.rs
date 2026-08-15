//! ACTIONBAND-OVERLAY-ACTUATION-0: one sealed crossing consequence door.

use std::sync::Mutex;

use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, DissolveCondition, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimThing, SimThingKind,
    SlotIndex, SubFieldRole, ThresholdDirection, TransformOp,
};
use simthing_driver::{
    compile_crossing_consequence_session, compile_gu_yang_n4_field_sweeps,
    ActionBandActiveInstance, ActionBandNativeLaneAdmission, CrossingConsequenceBinding,
    GuYangN4FieldSweepSpec, RoutedOverlayDelivery, StructuralAuthorization,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    FacilityPlaneError, FacilityPlaneGenerationBoundary, FacilityResidentPlane, GpuContext,
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
    frozen: simthing_spec::FrozenActionBandTemplates,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("actuation-proof", "parameter", 1));
    let column = registry
        .column_range(property)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
        .unwrap();
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: column,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 7801,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    let mut eml = EmlExpressionRegistry::new();
    let program = EmlTreeId(7801);
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
        program,
        EmlFormulaMeta {
            tree_id: program,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 2,
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
    allocator.populate_from_tree(&SimThing::new(SimThingKind::GameSession, 0));
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
    assert_eq!(values[fx.column.raw()].to_bits(), 3.0f32.to_bits());

    // RoutedOverlayDelivery: only authored duration + sealed source provenance
    // leave the new arm. Destination generation 7 establishes the activation
    // epoch and therefore deadline 11, not a source-relative absolute 5.
    let mut route_root = SimThing::new(SimThingKind::World, 0);
    let target = route_root.id;
    let property = resident_identity.property_id();
    route_root.add_property(property, fx.registry.property(property).default_value());
    let overlay_id = OverlayId::new();
    let overlay = Overlay {
        id: overlay_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.25))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 4 }],
        },
    };
    let routed = RoutedOverlayDelivery::admit(target, overlay).unwrap();
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
    route_allocator.populate_from_tree(&route_root);
    let mut route_runtime = SimRuntimeTree::admit(route_root);
    let mut route_registry = fx.registry.clone();
    let mut shadow = vec![0.0; route_registry.total_columns];
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
    allocator.populate_from_tree(&structural_root);
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
    );
    assert_eq!(applied.reparented, vec![(child_id, second_id)]);
    assert_eq!(first_id.raw() != second_id.raw(), true);
}

#[test]
fn forbidden_overlay_and_state_plane_shapes_are_rejected_by_the_real_door() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx =
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

    // A direct write/bind against a sibling facility plane is rejected by the
    // real owner capability, before either GPU buffer can be accessed.
    let boundary = FacilityPlaneGenerationBoundary::new();
    let resident_owner = boundary.admit_facility();
    let foreign_owner = boundary.admit_facility();
    let resident = FacilityResidentPlane::from_rows(
        &ctx,
        "actionband_7_8_foreign_write_falsifier",
        &boundary,
        &resident_owner,
        &[0.0f32],
    )
    .unwrap();
    assert_eq!(
        resident.validate_owner(&foreign_owner),
        Err(FacilityPlaneError::ForeignPlaneWrite)
    );

    // Certificate-envelope mutation: chi is an admission certificate bound,
    // not a runtime parameter that an overlay may widen.
    let mut field_registry = fx.registry.clone();
    let conductance_property =
        field_registry.register(SimProperty::simple("actuation-proof", "conductance", 1));
    let conductance_col = field_registry
        .column_range(conductance_property)
        .col_for_role(
            &SubFieldRole::Amount,
            &field_registry.property(conductance_property).layout,
        )
        .unwrap();
    assert!(compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims: field_registry.total_columns as u32,
        value_col: fx.column,
        conductance_col,
        saturation: 1.0,
        chi: 1.25,
        dt: 1.0,
    })
    .is_err());

    // Generation pacing cannot legalize gain >= 1 without a finite clamp.
    let unbounded = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "actionband-7-8-unbounded-field-feedback".into(),
        previous_col: 0,
        input_col: 0,
        output_col: Some(0),
        decay: 1.0,
        gain: 2.0,
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
    assert!(compile_eml_gadget(
        &unbounded,
        EmlGadgetCompileOptions {
            max_col: fx.registry.total_columns as u32,
        },
    )
    .is_err());
}
