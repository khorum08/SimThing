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
    real_gpu_crossing_at_generation(fx, ctx, 0)
}

/// Mint one sealed Phase-5 crossing at an explicit SOURCE generation through
/// the production generation-authority door (EVENT-GENERATION-STAMP-0), which
/// is what ordinary step boundaries bind. No synthetic key, no test-only
/// ingress: this is the same seal path the session uses.
fn real_gpu_crossing_at_generation(
    fx: &Fixture,
    ctx: &GpuContext,
    source_generation: u32,
) -> simthing_gpu::BandCrossingDelta {
    real_gpu_crossing_with_registrations(fx, ctx, source_generation, &fx.thresholds)
}

/// Mint one sealed Phase-5 crossing from a REBUILT threshold registration set,
/// through the same production seal path. This is how the ordinary public
/// threshold configuration path is modelled: registrations are cleared and
/// re-registered while the installed ActionBand plan stays frozen.
fn real_gpu_crossing_with_registrations(
    fx: &Fixture,
    ctx: &GpuContext,
    source_generation: u32,
    registrations: &[EmitOnThresholdRegistration],
) -> simthing_gpu::BandCrossingDelta {
    real_gpu_crossings_with_registrations(fx, ctx, source_generation, registrations)
        .into_iter()
        .next()
        .expect("the existing Phase-5 GPU crossing is the only ingress")
}

/// Every sealed crossing minted from a rebuilt registration set, so a witness
/// can select the exact definition it means to exercise.
fn real_gpu_crossings_with_registrations(
    fx: &Fixture,
    ctx: &GpuContext,
    source_generation: u32,
    registrations: &[EmitOnThresholdRegistration],
) -> Vec<simthing_gpu::BandCrossingDelta> {
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
                registrations,
            ))
            .unwrap(),
        )
        .unwrap();
    phase5.bind_generation_authority(source_generation);
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
    apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fx.registry,
        &allocator,
    )
}

fn resident_values(fx: &Fixture) -> Vec<f32> {
    let mut values = vec![0.0; fx.registry.total_columns];
    values[fx.column.raw()] = 1.5;
    values[fx.stead_output.raw()] = 2.0;
    values
}

/// SOURCE-GENERATION AUTHORITY witnesses (DA admission, relay 5720895186).
///
/// Simulation/source generation is authoritative for crossing identity; the
/// ActionBand facility generation is an internal monotone dispatch ordinal
/// that advances only on non-empty dispatch. Quiet boundaries therefore
/// separate the two clocks by construction, and SPARSE crossings must remain
/// lawful without manufacturing anything for the quiet generations.
#[test]
fn sparse_source_generations_execute_once_each_without_clock_pumping() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.8 requires a real GPU adapter; skips are forbidden");
    let fx = fixture();
    let lanes = native_lanes(&fx);

    // gap_length 3 is the relay's G2 -> quiet G3/G4 -> G5 shape; gap_length 1
    // is the contiguous G2 -> G3 control. Authored order is preserved in both.
    for gap_length in [3u32, 1] {
        let first_generation = 2u32;
        let later_generation = first_generation + gap_length;
        let resident = lanes
            .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
                fx.column.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ))
            .unwrap();
        let session = compile_crossing_consequence_session(
            &fx.frozen,
            &fx.eml,
            &[resident],
            &active(&fx),
            &lanes,
        )
        .unwrap();
        let plan_deltas = |generation: u32| real_gpu_crossing_at_generation(&fx, &ctx, generation);
        let first_delta = plan_deltas(first_generation);
        let later_delta = plan_deltas(later_generation);
        let (tx, _rx) = feeder_channel();
        let mut dispatch = session
            .bind_dispatch(&ctx, &resident_values(&fx))
            .unwrap();

        // Facility ordinal starts at 0 while source time is already 2: the
        // two clocks are separated from the very first crossing.
        assert_eq!(dispatch.generation(), 0);
        let first = dispatch
            .dispatch_sealed_and_apply(
                &ctx,
                fx.registry.total_columns as u32,
                std::slice::from_ref(&first_delta),
                &tx,
            )
            .expect("first sealed crossing executes at its own source generation")
            .expect("a non-empty batch dispatches");
        assert_eq!(first.crossing_count, 1);
        assert_eq!(
            dispatch.generation_dedupe_for_proof().unwrap(),
            (Some(first_generation), 1),
            "the window carries the SOURCE generation, not the facility ordinal"
        );
        let facility_after_first = dispatch.generation();
        assert_eq!(facility_after_first, 1, "one non-empty dispatch, one ordinal");

        // Quiet boundaries: no crossings, therefore no dispatch and NO
        // manufactured facility generation (rule 4, no clock pumping).
        for _ in 0..gap_length.saturating_sub(1) {
            assert!(
                dispatch
                    .dispatch_sealed_and_apply(&ctx, fx.registry.total_columns as u32, &[], &tx)
                    .expect("a quiet boundary is lawful")
                    .is_none(),
                "quiet boundaries must not dispatch"
            );
        }
        assert_eq!(
            dispatch.generation(),
            facility_after_first,
            "quiet boundaries must not manufacture facility generations"
        );

        // The later sparse crossing executes ONCE at its actual source
        // generation. Under the previous affine association this refused with
        // CrossingGenerationMismatch { expected: 2, actual: 5 }.
        let later = dispatch
            .dispatch_sealed_and_apply(
                &ctx,
                fx.registry.total_columns as u32,
                std::slice::from_ref(&later_delta),
                &tx,
            )
            .expect("a sparse later crossing is lawful")
            .expect("a non-empty batch dispatches");
        assert_eq!(later.crossing_count, 1);
        assert_eq!(
            dispatch.generation_dedupe_for_proof().unwrap(),
            (Some(later_generation), 1),
            "the window advanced to the later SOURCE generation exactly once"
        );
        assert_eq!(dispatch.generation(), facility_after_first + 1);

        // Duplicate at the accepted source generation stays suppressed.
        assert!(matches!(
            dispatch.dispatch_sealed_and_apply(
                &ctx,
                fx.registry.total_columns as u32,
                std::slice::from_ref(&later_delta),
                &tx,
            ),
            Err(simthing_driver::CrossingConsequenceDispatchError::DuplicateCrossingConsumption)
        ));

        // Source-generation REGRESSION is refused fail-closed and leaves the
        // accepted window untouched.
        match dispatch.dispatch_sealed_and_apply(
            &ctx,
            fx.registry.total_columns as u32,
            std::slice::from_ref(&first_delta),
            &tx,
        ) {
            Err(
                simthing_driver::CrossingConsequenceDispatchError::CrossingSourceGenerationRegressed {
                    accepted,
                    actual,
                },
            ) => {
                assert_eq!(accepted, later_generation);
                assert_eq!(actual, first_generation);
            }
            other => panic!("source-generation regression must refuse fail-closed: {other:?}"),
        }
        assert_eq!(
            dispatch.generation_dedupe_for_proof().unwrap(),
            (Some(later_generation), 1),
            "a refused regression neither rewinds nor clears the accepted window"
        );
    }
}

/// FROZEN THRESHOLD-DEFINITION PROVENANCE witnesses (DA admission, relay
/// 5721882717).
///
/// Tick-zero ActionBand commitments freeze a consequence against an admitted
/// threshold DEFINITION. The ordinary public threshold configuration path may
/// later clear and re-register thresholds while the installed plan stays
/// frozen. Registration index is an ephemeral registry position and is NOT the
/// threshold's meaning: an identical rebuild stays lawful at any index, while a
/// redefinition fails closed BEFORE consequence authority.
#[test]
fn frozen_threshold_definition_is_authority_not_registration_index() {
    let _guard = GPU.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.8 requires a real GPU adapter; skips are forbidden");
    let fx = fixture();
    let lanes = native_lanes(&fx);
    let admitted = fx.thresholds[0].clone();

    let session = |fx: &Fixture, lanes: &ActionBandNativeLaneAdmission| {
        let resident = lanes
            .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
                fx.column.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ))
            .unwrap();
        compile_crossing_consequence_session(&fx.frozen, &fx.eml, &[resident], &active(fx), lanes)
            .unwrap()
    };

    // 1. Identical rebuild: the bound definition is exactly unchanged, so the
    //    frozen consequence remains lawful with correct product timing.
    let identical = vec![admitted.clone()];
    let delta = real_gpu_crossing_with_registrations(&fx, &ctx, 1, &identical);
    let plan_session = session(&fx, &lanes);
    assert!(
        plan_session
            .compiled()
            .execution_plan()
            .crossings_from_sealed(std::slice::from_ref(&delta))
            .expect("an identical rebuild keeps the frozen binding lawful")
            .crossing_count()
            > 0,
        "identical rebuild must still join its frozen consequence"
    );

    // 2. Redefinition BEFORE the first crossing (the relay's 1.5 -> 0.75
    //    shape): the sealed crossing carries the rebuilt threshold, so the
    //    frozen consequence must NOT be routed through it.
    for rebuilt_threshold in [0.75f32, 1.25] {
        let mut redefined = admitted.clone();
        redefined.threshold = rebuilt_threshold;
        let stale_delta =
            real_gpu_crossing_with_registrations(&fx, &ctx, 2, std::slice::from_ref(&redefined));
        let plan_session = session(&fx, &lanes);
        match plan_session
            .compiled()
            .execution_plan()
            .crossings_from_sealed(std::slice::from_ref(&stale_delta))
        {
            Err(simthing_gpu::ActionBandExecutionError::FrozenThresholdDefinitionStale {
                admitted_threshold_bits,
                observed_threshold_bits,
                ..
            }) => {
                assert_eq!(admitted_threshold_bits, admitted.threshold.to_bits());
                assert_eq!(observed_threshold_bits, rebuilt_threshold.to_bits());
            }
            other => panic!(
                "a redefined bound threshold ({rebuilt_threshold}) must fail closed before \
                 consequence authority, got {other:?}"
            ),
        }
    }

    // 3. Reorder: the SAME bound definition re-registered at a different
    //    index stays lawful — index-order independent. An unrelated UNBOUND
    //    registration (its own event_kind) is registered ahead of it.
    let unrelated = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: fx.column,
        threshold: 0.6,
        direction: ThresholdDirection::Upward,
        event_kind: 9999,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let reordered = vec![unrelated.clone(), admitted.clone()];
    let reordered_deltas = real_gpu_crossings_with_registrations(&fx, &ctx, 3, &reordered);
    let bound_delta = reordered_deltas
        .iter()
        .find(|delta| delta.event_kind() == admitted.event_kind)
        .expect("the bound definition still mints its own sealed crossing")
        .clone();
    assert_ne!(
        bound_delta.reg_idx(),
        0,
        "the bound definition moved to a new registry index"
    );
    let plan_session = session(&fx, &lanes);
    assert!(
        plan_session
            .compiled()
            .execution_plan()
            .crossings_from_sealed(std::slice::from_ref(&bound_delta))
            .expect("reordering must not stale a frozen binding")
            .crossing_count()
            > 0,
        "the frozen binding follows its DEFINITION, not its old index"
    );

    // 4. Unbound/additive churn at a NON-bound index is simply not this
    //    plan's business: no consequence, and no session-wide poisoning.
    // ADDITIVE set: the bound definition keeps index 0, the unrelated one is
    // appended after it, so the churn genuinely sits at a NON-bound index.
    let additive = vec![admitted.clone(), unrelated.clone()];
    let additive_deltas = real_gpu_crossings_with_registrations(&fx, &ctx, 4, &additive);
    let unbound_delta = additive_deltas
        .iter()
        .find(|delta| delta.event_kind() == unrelated.event_kind)
        .expect("the unrelated definition mints its own sealed crossing")
        .clone();
    assert_ne!(
        unbound_delta.reg_idx(),
        0,
        "additive churn must not occupy the bound registration index"
    );
    let plan_session = session(&fx, &lanes);
    assert_eq!(
        plan_session
            .compiled()
            .execution_plan()
            .crossings_from_sealed(std::slice::from_ref(&unbound_delta))
            .expect("unbound threshold churn is lawful")
            .crossing_count(),
        0,
        "an unbound definition neither routes a consequence nor poisons the session"
    );

    // 5. REMAP: a different definition occupying the frozen band's own
    //    registration index fails closed — the bound index no longer means
    //    what was admitted.
    let remapped = vec![unrelated.clone()];
    let remapped_delta = real_gpu_crossing_with_registrations(&fx, &ctx, 5, &remapped);
    assert_eq!(remapped_delta.reg_idx(), 0, "remap occupies the bound index");
    let plan_session = session(&fx, &lanes);
    assert!(
        matches!(
            plan_session
                .compiled()
                .execution_plan()
                .crossings_from_sealed(std::slice::from_ref(&remapped_delta)),
            Err(simthing_gpu::ActionBandExecutionError::FrozenThresholdDefinitionStale { .. })
        ),
        "a remapped bound index must fail closed before consequence authority"
    );
}
