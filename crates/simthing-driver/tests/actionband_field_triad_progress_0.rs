use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode,
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlTreeId, GateSpec, ScaleSpec,
    SimProperty, SimThing, SimThingKind, SlotIndex, SourceSpec, StructuralScalarChannel,
    SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    carry_bound_observables, compile_action_band_gpu_execution,
    compile_action_band_gpu_execution_with_native_lanes, compile_gu_yang_n4_field_sweeps,
    ActionBandActiveInstance, ActionBandExecutionCompileError, ActionBandNativeLaneAdmission,
    BoundObservableIdentity, FieldNeutralityGate, GuYangN4FieldSweepSpec, FIELD_NEUTRALITY_OUTCOME,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu, wgpu,
    AccumulatorOpSession, ActionBandEmissionBindingGpu, ActionBandGpuExecution, FieldSweepOutput,
    FieldSweepSession, GpuContext, PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator,
};
use simthing_sim::ThresholdRegistry;
use simthing_spec::{
    compile_eml_gadget, ActionBandAdmissionBudgetSpec, ActionBandAdmissionError,
    ActionBandBandSpec, ActionBandChannelBindingSpec, ActionBandChannelKind,
    ActionBandConservedProgressBindingSpec, ActionBandConservedProgressBoundSourceSpec,
    ActionBandRequirementSemantics, ActionBandSessionBuildDoor, ActionBandSessionSpec,
    ActionBandTargetSpec, ActionBandTemplateSpec, AdmittedActionBandConservedProgressBoundSource,
    EmlGadgetCompileOptions, EmlGadgetInstanceSpec, FrozenActionBandTemplates,
    ScalarBoundDirection,
};

static GPU_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct Fixture {
    registry: DimensionRegistry,
    threshold: EmitOnThresholdRegistration,
    value: ColumnIndex,
    conductance: ColumnIndex,
    rf_claim: ColumnIndex,
    rf_result: ColumnIndex,
    feedback_previous: ColumnIndex,
    feedback_input: ColumnIndex,
    feedback_output: ColumnIndex,
}

fn fixture(threshold: f32) -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut register = |name: &str| {
        let property = registry.register(SimProperty::simple("actionband-field-triad", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .expect("amount column")
    };
    let value = register("gu-yang-value");
    let conductance = register("gu-yang-conductance");
    let rf_claim = register("rf-claim");
    let rf_result = register("rf-result");
    let feedback_previous = register("feedback-previous");
    let feedback_input = register("feedback-input");
    let feedback_output = register("feedback-output");
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: value,
        threshold,
        direction: ThresholdDirection::Upward,
        event_kind: 7_500,
        buffer: EmitOnThresholdBuffer::Values,
    };
    Fixture {
        registry,
        threshold,
        value,
        conductance,
        rf_claim,
        rf_result,
        feedback_previous,
        feedback_input,
        feedback_output,
    }
}

fn session_spec(fixture: &Fixture, eml_program: Option<u32>) -> ActionBandSessionSpec {
    let emission_binding_indices = if eml_program.is_some() {
        vec![0, 1]
    } else {
        vec![0]
    };
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: if eml_program.is_some() { 3 } else { 1 },
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: u32::from(eml_program.is_some()),
            emission_binding_count: emission_binding_indices.len() as u32,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "field-triad-progress".into(),
            label: Some("presentation-only-label".into()),
            axis_channels: if eml_program.is_some() {
                vec![
                    channel(fixture.value),
                    channel(fixture.feedback_previous),
                    channel(fixture.feedback_input),
                ]
            } else {
                vec![channel(fixture.value)]
            },
            target: ActionBandTargetSpec::ScalarBound {
                channel: fixture.value.raw_u32(),
                bound: fixture.threshold.threshold,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program,
                emission_binding_indices,
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: ActionBandRequirementSemantics::Ordinary,
        }],
    }
}

fn channel(column: ColumnIndex) -> ActionBandChannelBindingSpec {
    ActionBandChannelBindingSpec {
        column: column.raw_u32(),
        kind: ActionBandChannelKind::Primitive,
    }
}

fn conserved(
    source: ActionBandConservedProgressBoundSourceSpec,
) -> ActionBandConservedProgressBindingSpec {
    ActionBandConservedProgressBindingSpec {
        template_id: "field-triad-progress".into(),
        band_index: 0,
        emission_binding_index: 0,
        bound_source: source,
    }
}

fn admit(
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    source: ActionBandConservedProgressBoundSourceSpec,
) -> Result<FrozenActionBandTemplates, ActionBandAdmissionError> {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_with_conserved_progress_at_session_build(
        &session_spec(fixture, (!eml.is_empty()).then_some(17)),
        &[conserved(source)],
        &fixture.registry,
        eml,
        std::slice::from_ref(&fixture.threshold),
    )
    .cloned()
}

fn empty_eml() -> EmlExpressionRegistry {
    EmlExpressionRegistry::new()
}

fn amplifying_eml(fixture: &Fixture) -> EmlExpressionRegistry {
    use simthing_core::eml_nodes::{opcode, EmlNode};

    let node = |opcode| EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let mut eml = EmlExpressionRegistry::new();
    eml.register_formula(
        EmlTreeId(17),
        EmlFormulaMeta {
            tree_id: EmlTreeId(17),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 4,
            max_stack_depth: 2,
            has_loops: false,
            has_recursion: false,
            display_name: "conserved-progress-desired-amplifier-2x".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: fixture.value.raw_u32(),
                ..node(opcode::SLOT_VALUE)
            },
            EmlNode {
                opcode: opcode::LITERAL_F32,
                a: 2.0f32.to_bits(),
                ..node(opcode::LITERAL_F32)
            },
            node(opcode::MUL),
            node(opcode::RETURN_TOP),
        ],
    )
    .expect("2*x amplifier is an otherwise valid ActionBand EML program");
    eml
}

#[test]
fn conserved_progress_source_is_closed_exactly_once_and_existing_threshold_bound() {
    let fixture = fixture(0.15);
    let eml = empty_eml();
    for source in [
        ActionBandConservedProgressBoundSourceSpec::RfGrant,
        ActionBandConservedProgressBoundSourceSpec::GuYangAvailable,
        ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    ] {
        let frozen = admit(&fixture, &eml, source).expect("one explicit native source");
        let binding = frozen.conserved_progress_bindings()[0];
        assert_eq!(binding.band_table_index(), 0);
        assert_eq!(binding.emission_binding().raw(), 0);
        assert_eq!(binding.bound_source().threshold_registration().raw(), 0);
    }

    assert!(matches!(
        admit(
            &fixture,
            &eml,
            ActionBandConservedProgressBoundSourceSpec::None
        ),
        Err(ActionBandAdmissionError::ConservedProgressBoundRequired { .. })
    ));

    let mut door = ActionBandSessionBuildDoor::new();
    let duplicate = [
        conserved(ActionBandConservedProgressBoundSourceSpec::GuYangRealized),
        conserved(ActionBandConservedProgressBoundSourceSpec::RfGrant),
    ];
    assert!(matches!(
        door.admit_once_with_conserved_progress_at_session_build(
            &session_spec(&fixture, None),
            &duplicate,
            &fixture.registry,
            &eml,
            std::slice::from_ref(&fixture.threshold),
        ),
        Err(ActionBandAdmissionError::DuplicateConservedProgressBound { .. })
    ));

    let missing = serde_json::json!({
        "template_id": "field-triad-progress",
        "band_index": 0,
        "emission_binding_index": 0
    });
    assert!(serde_json::from_value::<ActionBandConservedProgressBindingSpec>(missing).is_err());
    let fifth = serde_json::json!({
        "template_id": "field-triad-progress",
        "band_index": 0,
        "emission_binding_index": 0,
        "bound_source": "PrivateThroughput"
    });
    assert!(serde_json::from_value::<ActionBandConservedProgressBindingSpec>(fifth).is_err());

    let frozen = admit(
        &fixture,
        &eml,
        ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    )
    .unwrap();
    let active = [ActionBandActiveInstance::new(
        frozen.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    assert!(matches!(
        compile_action_band_gpu_execution(
            &frozen,
            &eml,
            &[ActionBandEmissionBindingGpu::rf_claim(
                fixture.rf_claim.raw_u32()
            )],
            &active,
        ),
        Err(ActionBandExecutionCompileError::ConservedProgressRequiresNativeLane { .. })
    ));

    let amplifier = amplifying_eml(&fixture);
    let amplified = admit(
        &fixture,
        &amplifier,
        ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    )
    .expect("conserved EML computes desired quantity before the native clamp");
    assert_eq!(amplified.bands()[0].eml_program(), Some(EmlTreeId(17)));

    let rf_plan = rf_plan(&fixture);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.feedback_output],
        std::slice::from_ref(&rf_plan),
        &[],
        &ThresholdRegistry::new(),
    );
    let active = [ActionBandActiveInstance::new(
        amplified.templates()[0].index(),
        SlotIndex::new(0),
        [0.0; 4],
    )];
    let reapplied = [
        ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32())
            .with_conserved_progress_bound_source(
                ActionBandEmissionBindingGpu::CONSERVED_BOUND_RF_GRANT,
            ),
        ActionBandEmissionBindingGpu::property_next(
            fixture.feedback_output.raw_u32(),
            simthing_gpu::ActionBandPropertyWrite::Set,
        ),
    ];
    assert!(matches!(
        compile_action_band_gpu_execution_with_native_lanes(
            &amplified, &amplifier, &reapplied, &active, &native,
        ),
        Err(ActionBandExecutionCompileError::InvalidConservedProgressBinding { .. })
    ));

    let mut ordinary_door = ActionBandSessionBuildDoor::new();
    let ordinary = ordinary_door
        .admit_once_at_session_build(
            &session_spec(&fixture, None),
            &fixture.registry,
            &eml,
            std::slice::from_ref(&fixture.threshold),
        )
        .unwrap();
    let invalid_fifth = [
        ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32())
            .with_conserved_progress_bound_source(4),
    ];
    assert!(matches!(
        compile_action_band_gpu_execution_with_native_lanes(
            ordinary,
            &eml,
            &invalid_fifth,
            &active,
            &native,
        ),
        Err(ActionBandExecutionCompileError::Kernel(
            simthing_gpu::ActionBandExecutionError::InvalidTableSpan
        ))
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldSweepCallGraphProbe {
    registration_dispatches: u64,
    resident_exports: u64,
    host_readbacks: u64,
}

impl FieldSweepCallGraphProbe {
    fn observe(session: &FieldSweepSession) -> Self {
        Self {
            registration_dispatches: session.registration_dispatches(),
            resident_exports: session.resident_exports(),
            host_readbacks: session.host_readbacks(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ResidentPathProbe {
    field: FieldSweepCallGraphProbe,
    sparse_phase5_readbacks: u32,
    proof_result_readbacks: u32,
}

fn gu_yang(fixture: &Fixture, saturation: f32) -> [simthing_gpu::FieldSweepRegistration; 2] {
    compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims: fixture.registry.total_columns as u32,
        value_col: fixture.value,
        conductance_col: fixture.conductance,
        saturation,
        chi: 0.25,
        dt: 1.0,
    })
    .expect("ordinary Gu-Yang registrations")
}

fn rf_plan(fixture: &Fixture) -> CompiledAccumulatorOpPlan {
    CompiledAccumulatorOpPlan {
        slot_count: 2,
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
    }
}

fn values(fixture: &Fixture, left: f32, right: f32) -> Vec<f32> {
    let n_dims = fixture.registry.total_columns;
    let mut values = vec![0.0; 2 * n_dims];
    values[fixture.value.raw()] = left;
    values[n_dims + fixture.value.raw()] = right;
    values
}

fn storage_buffer(ctx: &GpuContext, label: &str, byte_len: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: byte_len,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn run_resident_progress(
    ctx: &GpuContext,
    fixture: &Fixture,
    registrations: &[simthing_gpu::FieldSweepRegistration; 2],
    initial: &[f32],
    plan: &simthing_gpu::ActionBandExecutionPlan,
    rf_plan: &CompiledAccumulatorOpPlan,
) -> (f32, f32, f32, ResidentPathProbe) {
    let mut sparse_phase5_readbacks = 0;
    let mut proof_result_readbacks = 0;
    assert_eq!(
        registrations[0].output(),
        FieldSweepOutput::Matrix(fixture.conductance)
    );
    assert_eq!(
        registrations[1].output(),
        FieldSweepOutput::Matrix(fixture.value)
    );

    let mut field = FieldSweepSession::new(ctx, &registrations[0]).unwrap();
    field.upload_values(ctx, initial).unwrap();
    field.dispatch_chain(ctx, registrations, 1).unwrap();

    let resident = storage_buffer(
        ctx,
        "actionband_field_triad_resident",
        std::mem::size_of_val(initial) as u64,
    );
    field.copy_values_to_buffer(ctx, &resident);

    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, 2, fixture.registry.total_columns as u32, 1);
    phase5.upload_previous_values(ctx, initial);
    phase5
        .copy_values_prefix_from_buffer(ctx, &resident, 0, 0, resident.size())
        .unwrap();
    let gpu_thresholds =
        emit_on_threshold_registrations_to_gpu(std::slice::from_ref(&fixture.threshold));
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&gpu_thresholds).unwrap(),
        )
        .unwrap();
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
    sparse_phase5_readbacks += 1;

    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 1, "real Phase-5 crossing");
    let native_flux = deltas[0].post_value();
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    assert_eq!(crossings.crossing_count(), 1);

    let next = storage_buffer(
        ctx,
        "actionband_field_triad_next",
        std::mem::size_of_val(initial) as u64,
    );
    let mut action = match ActionBandGpuExecution::new(ctx, plan.clone()).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("one conserved progress row"),
    };
    action
        .dispatch_with_native_next(
            ctx,
            &resident,
            &next,
            fixture.registry.total_columns as u32,
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
    let result = rf.readback_full(ctx).unwrap();
    proof_result_readbacks += 1;
    (
        result[fixture.rf_result.raw()],
        result[fixture.feedback_output.raw()],
        native_flux,
        ResidentPathProbe {
            field: FieldSweepCallGraphProbe::observe(&field),
            sparse_phase5_readbacks,
            proof_result_readbacks,
        },
    )
}

fn run_duplicate_solve_and_readback_mutant(
    ctx: &GpuContext,
    registrations: &[simthing_gpu::FieldSweepRegistration; 2],
    initial: &[f32],
) -> FieldSweepCallGraphProbe {
    let mut field = FieldSweepSession::new(ctx, &registrations[0]).unwrap();
    field.upload_values(ctx, initial).unwrap();
    field.dispatch_chain(ctx, registrations, 1).unwrap();
    field.dispatch_chain(ctx, registrations, 1).unwrap();
    field.readback(ctx).unwrap();
    FieldSweepCallGraphProbe::observe(&field)
}

fn compile_resident_plan(
    fixture: &Fixture,
) -> (
    simthing_gpu::ActionBandExecutionPlan,
    CompiledAccumulatorOpPlan,
) {
    let eml = amplifying_eml(fixture);
    let frozen = admit(
        fixture,
        &eml,
        ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
    )
    .unwrap();
    let rf_plan = rf_plan(fixture);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.feedback_output],
        std::slice::from_ref(&rf_plan),
        &[],
        &ThresholdRegistry::new(),
    );
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &[
            ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                fixture.feedback_output.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
        ],
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            SlotIndex::new(0),
            [0.0; 4],
        )],
        &native,
    )
    .unwrap();
    let bound = compiled.conserved_progress_bindings()[0];
    assert_eq!(bound.band_table_index(), 0);
    assert_eq!(bound.emission_binding_index(), 0);
    assert_eq!(bound.threshold_column(), fixture.value);
    assert_eq!(
        bound.destination(),
        simthing_gpu::ActionBandEmissionDestination::RfClaim
    );
    assert!(matches!(
        bound.bound_source(),
        AdmittedActionBandConservedProgressBoundSource::GuYangRealized(index)
            if index.raw() == 0
    ));
    (compiled.into_execution_plan(), rf_plan)
}

#[test]
fn real_gu_yang_resident_output_bounds_rf_progress_without_duplicate_solve_or_cpu_mirror() {
    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("actionband_field_triad_progress_0: GPU leg skipped (no adapter)");
        return;
    };
    let fx = fixture(0.15);
    let initial = values(&fx, 0.1, 0.8);
    let high = gu_yang(&fx, 1.0);
    let low = gu_yang(&fx, 0.5);
    let restored = gu_yang(&fx, 1.0);
    let (plan, rf_plan) = compile_resident_plan(&fx);

    let (high_progress, high_desired, high_flux, high_probe) =
        run_resident_progress(&ctx, &fx, &high, &initial, &plan, &rf_plan);
    let (low_progress, low_desired, low_flux, low_probe) =
        run_resident_progress(&ctx, &fx, &low, &initial, &plan, &rf_plan);
    let (restored_progress, restored_desired, restored_flux, restored_probe) =
        run_resident_progress(&ctx, &fx, &restored, &initial, &plan, &rf_plan);

    for (progress, desired, flux) in [
        (high_progress, high_desired, high_flux),
        (low_progress, low_desired, low_flux),
        (restored_progress, restored_desired, restored_flux),
    ] {
        assert_eq!(desired.to_bits(), (2.0 * flux).to_bits());
        assert_eq!(
            progress.to_bits(),
            flux.to_bits(),
            "native q_flux clamps the amplified q_desired"
        );
    }

    assert!(
        low_progress < high_progress,
        "capacity down => progress down"
    );
    assert_eq!(
        restored_progress.to_bits(),
        high_progress.to_bits(),
        "capacity restored => exact progress restored"
    );
    for probe in [high_probe, low_probe, restored_probe] {
        assert_eq!(probe.field.registration_dispatches, 2);
        assert_eq!(probe.field.resident_exports, 1);
        assert_eq!(probe.field.host_readbacks, 0);
        assert_eq!(probe.sparse_phase5_readbacks, 1);
        assert_eq!(probe.proof_result_readbacks, 1);
    }
    let rival = run_duplicate_solve_and_readback_mutant(&ctx, &high, &initial);
    assert_eq!(rival.registration_dispatches, 4);
    assert_eq!(rival.resident_exports, 0);
    assert_eq!(rival.host_readbacks, 1);
    assert_ne!(
        rival, high_probe.field,
        "real duplicate/readback seam must RED"
    );

    let signed_fixture = fixture(-0.75);
    let signed_initial = values(&signed_fixture, -0.8, -0.1);
    let signed_registrations = gu_yang(&signed_fixture, 1.0);
    let (signed_plan, signed_rf) = compile_resident_plan(&signed_fixture);
    let (signed_progress, signed_desired, signed_flux, signed_probe) = run_resident_progress(
        &ctx,
        &signed_fixture,
        &signed_registrations,
        &signed_initial,
        &signed_plan,
        &signed_rf,
    );
    assert!(
        signed_progress < 0.0,
        "native signed order is preserved; no abs(flux)"
    );
    assert_eq!(signed_desired.to_bits(), (2.0 * signed_flux).to_bits());
    assert_eq!(signed_progress.to_bits(), signed_flux.to_bits());
    assert_eq!(signed_probe.field.host_readbacks, 0);
}

#[test]
fn field_or_rf_recurrence_reuses_existing_bounded_feedback_admission() {
    let fixture = fixture(0.15);
    let valid = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "bounded-field-seed".into(),
        previous_col: fixture.feedback_previous.raw_u32(),
        input_col: fixture.feedback_input.raw_u32(),
        output_col: Some(fixture.feedback_output.raw_u32()),
        decay: 0.5,
        gain: 0.25,
        min: -1.0,
        max: 1.0,
    };
    let compiled = compile_eml_gadget(
        &valid,
        EmlGadgetCompileOptions {
            max_col: fixture.registry.total_columns as u32,
        },
    )
    .expect("finite decay plus explicit clamp admits");

    let unbounded_gain_by_pacing = EmlGadgetInstanceSpec::BoundedFeedback {
        id: "generation-pacing-is-not-a-gain-bound".into(),
        previous_col: fixture.feedback_previous.raw_u32(),
        input_col: fixture.feedback_input.raw_u32(),
        output_col: Some(fixture.feedback_output.raw_u32()),
        decay: 1.0,
        gain: 2.0,
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
    assert!(compile_eml_gadget(
        &unbounded_gain_by_pacing,
        EmlGadgetCompileOptions {
            max_col: fixture.registry.total_columns as u32,
        },
    )
    .is_err());

    let mut eml = EmlExpressionRegistry::new();
    eml.register_formula(
        EmlTreeId(17),
        EmlFormulaMeta {
            tree_id: EmlTreeId(17),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: compiled.nodes.len() as u32,
            max_stack_depth: 0,
            has_loops: false,
            has_recursion: false,
            display_name: "existing-bounded-feedback-field-seed".into(),
        },
        compiled.nodes,
    )
    .unwrap();
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(
            &session_spec(&fixture, Some(17)),
            &fixture.registry,
            &eml,
            std::slice::from_ref(&fixture.threshold),
        )
        .expect("ordinary ActionBand references the already-admitted bounded program");
    assert_eq!(frozen.bands()[0].eml_program(), Some(EmlTreeId(17)));
}

#[test]
fn field_triad_identity_uses_the_graduated_field_neutral_semantic_schema() {
    assert_eq!(FIELD_NEUTRALITY_OUTCOME, FieldNeutralityGate::FieldNeutral);
    let observable = BoundObservableIdentity::new(
        "synthetic-conserved-progress",
        Some("existing-threshold-registration:0/gu-yang-realized"),
    );
    let carried = carry_bound_observables(std::slice::from_ref(&observable));
    assert_eq!(carried, vec![observable]);
    assert_eq!(carried[0].key(), "synthetic-conserved-progress");
    assert_eq!(
        carried[0].provenance(),
        Some("existing-threshold-registration:0/gu-yang-realized")
    );
}
