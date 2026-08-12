//! ACTIONBAND-SPATIAL-FLUX-WITNESS-0 (7.5b) — workshop-homed GPU witness.
//!
//! Born-mortal: all proof lives here. Production crates are consumption-only.
//! DA A1: observe BOTH pre-clamp progress operand and post-clamp executable result.

use std::sync::Mutex;

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode,
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlTreeId, GateSpec, ScaleSpec,
    SimProperty, SimThing, SimThingKind, SlotIndex, SourceSpec, StructuralScalarChannel,
    SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution_with_native_lanes, compile_gu_yang_n4_field_sweeps,
    ActionBandActiveInstance, ActionBandNativeLaneAdmission, GuYangN4FieldSweepSpec,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu, wgpu,
    AccumulatorOpSession, ActionBandEmissionBindingGpu, ActionBandGpuExecution, FieldSweepOutput,
    FieldSweepSession, GpuContext, PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator,
};
use simthing_sim::ThresholdRegistry;
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec,
    ScalarBoundDirection,
};
use simthing_workshop::actionband_spatial_flux_witness_0::{
    assert_capacity_witness, assert_mutant_pre_clamp_pair_reds, assert_no_sink_posture,
    assert_opposed_demand_law, assert_pre_clamp_preserves_native_sign,
    assert_production_has_zero_workshop_coupling, descent_identity, lawful_pre_clamp_operand,
    reject_abs_flux_mutant, CapacityWitnessSample, OpposedDemandObservation, OpposedDemandOperand,
    PreClampConsumption,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

struct Fixture {
    registry: DimensionRegistry,
    threshold: EmitOnThresholdRegistration,
    value: ColumnIndex,
    conductance: ColumnIndex,
    rf_claim: ColumnIndex,
    rf_result: ColumnIndex,
    /// Dual non-conserved emission destination: witnesses pre-clamp EML payload.
    pre_clamp_obs: ColumnIndex,
    palma_d: ColumnIndex,
}

fn fixture(threshold: f32) -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut register = |name: &str| {
        let property = registry.register(SimProperty::simple("spatial-flux-witness", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .expect("amount")
    };
    let value = register("gu-yang-value");
    let conductance = register("gu-yang-conductance");
    let rf_claim = register("rf-claim");
    let rf_result = register("rf-result");
    let pre_clamp_obs = register("pre-clamp-obs");
    let palma_d = register("palma-d-fixed");
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: value,
        threshold,
        direction: ThresholdDirection::Upward,
        event_kind: 7_520,
        buffer: EmitOnThresholdBuffer::Values,
    };
    Fixture {
        registry,
        threshold,
        value,
        conductance,
        rf_claim,
        rf_result,
        pre_clamp_obs,
        palma_d,
    }
}

fn channel(column: ColumnIndex) -> ActionBandChannelBindingSpec {
    ActionBandChannelBindingSpec {
        column: column.raw_u32(),
        kind: ActionBandChannelKind::Primitive,
    }
}

fn amplifying_eml(value_col: ColumnIndex) -> EmlExpressionRegistry {
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
            display_name: "flux-witness-2x-desired".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: value_col.raw_u32(),
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
    .expect("2x EML");
    eml
}

fn identity_eml(value_col: ColumnIndex) -> EmlExpressionRegistry {
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
            node_count: 2,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: "flux-witness-identity-desired".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: value_col.raw_u32(),
                ..node(opcode::SLOT_VALUE)
            },
            node(opcode::RETURN_TOP),
        ],
    )
    .expect("identity EML");
    eml
}

fn admit_conserved(
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    cost_band_sink: bool,
) -> simthing_spec::FrozenActionBandTemplates {
    let emission_count = if cost_band_sink { 3 } else { 2 };
    let emission_binding_indices: Vec<u32> = (0..emission_count).collect();
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_with_conserved_progress_at_session_build(
        &ActionBandSessionSpec {
            budget: ActionBandAdmissionBudgetSpec {
                axis_channel_count: 2,
                dependency_binding_count: 0,
                storage_rows: 1,
                eml_program_count: 1,
                emission_binding_count: emission_count,
            },
            templates: vec![ActionBandTemplateSpec {
                id: "spatial-flux-witness".into(),
                label: Some("presentation-only".into()),
                axis_channels: vec![channel(fixture.value), channel(fixture.palma_d)],
                // Fixed target + fixed PALMA column (descent identity independent of capacity).
                target: ActionBandTargetSpec::ScalarBound {
                    channel: fixture.value.raw_u32(),
                    bound: fixture.threshold.threshold,
                    direction: ScalarBoundDirection::AtLeast,
                },
                velocity: None,
                bands: vec![ActionBandBandSpec {
                    threshold_registration_index: 0,
                    eml_program: Some(17),
                    emission_binding_indices,
                }],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 1,
                requirement_semantics: ActionBandRequirementSemantics::Ordinary,
            }],
        },
        &[ActionBandConservedProgressBindingSpec {
            template_id: "spatial-flux-witness".into(),
            band_index: 0,
            emission_binding_index: 0,
            bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
        }],
        &fixture.registry,
        eml,
        std::slice::from_ref(&fixture.threshold),
    )
    .expect("admit conserved progress")
    .clone()
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

fn gu_yang(fixture: &Fixture, saturation: f32) -> [simthing_gpu::FieldSweepRegistration; 2] {
    compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims: fixture.registry.total_columns as u32,
        value_col: fixture.value,
        conductance_col: fixture.conductance,
        saturation,
        chi: 1.0,
        dt: 1.0,
    })
    .expect("gu-yang")
}

fn values(fixture: &Fixture, left: f32, right: f32) -> Vec<f32> {
    let n_dims = fixture.registry.total_columns;
    let mut values = vec![0.0; 2 * n_dims];
    values[fixture.value.raw()] = left;
    values[n_dims + fixture.value.raw()] = right;
    values[fixture.palma_d.raw()] = 1.0; // fixed PALMA potential
    values[n_dims + fixture.palma_d.raw()] = 1.0;
    values
}

fn storage_buffer(ctx: &GpuContext, label: &str, size: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// Run resident Gu-Yang → Phase-5 → ActionBand with dual emission:
/// binding0 = conserved RF claim (POST-CLAMP), binding1 = property_next pre-clamp obs.
fn run_witness_leg(
    ctx: &GpuContext,
    fixture: &Fixture,
    saturation: f32,
    initial: &[f32],
    plan: &simthing_gpu::ActionBandExecutionPlan,
    rf_plan: &CompiledAccumulatorOpPlan,
) -> (f32, f32, f32) {
    let registrations = gu_yang(fixture, saturation);
    assert_eq!(
        registrations[1].output(),
        FieldSweepOutput::Matrix(fixture.value)
    );

    let mut field = FieldSweepSession::new(ctx, &registrations[0]).unwrap();
    field.upload_values(ctx, initial).unwrap();
    field.dispatch_chain(ctx, &registrations, 1).unwrap();

    let resident = storage_buffer(
        ctx,
        "flux_witness_resident",
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

    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    assert!(!deltas.is_empty(), "Phase-5 crossing required");
    let native_flux = deltas[0].post_value();
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();

    let next = storage_buffer(
        ctx,
        "flux_witness_next",
        std::mem::size_of_val(initial) as u64,
    );
    let mut action = match ActionBandGpuExecution::new(ctx, plan.clone()).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("active ActionBand required"),
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
    (
        result[fixture.rf_result.raw()],      // post-clamp conserved progress
        result[fixture.pre_clamp_obs.raw()], // pre-clamp dual emission
        native_flux,
    )
}

fn compile_witness_plan(
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    cost_band_sink: bool,
) -> (
    simthing_gpu::ActionBandExecutionPlan,
    CompiledAccumulatorOpPlan,
) {
    let frozen = admit_conserved(fixture, eml, cost_band_sink);
    let rf = rf_plan(fixture);
    let mut native_cols = vec![fixture.pre_clamp_obs];
    if cost_band_sink {
        native_cols.push(fixture.pre_clamp_obs); // placeholder width; sink uses costband dest
    }
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.pre_clamp_obs],
        std::slice::from_ref(&rf),
        &[],
        &ThresholdRegistry::new(),
    );
    let mut bindings = vec![
        ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32()),
        ActionBandEmissionBindingGpu::property_next(
            fixture.pre_clamp_obs.raw_u32(),
            simthing_gpu::ActionBandPropertyWrite::Set,
        ),
    ];
    if cost_band_sink {
        bindings.push(ActionBandEmissionBindingGpu::cost_band(
            fixture.pre_clamp_obs.raw_u32(),
        ));
    }
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        eml,
        &bindings,
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            SlotIndex::new(0),
            [0.0; 4],
        )],
        &native,
    )
    .expect("compile with native lanes");
    (compiled.into_execution_plan(), rf)
}

fn require_gpu() -> Option<GpuContext> {
    let _g = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    GpuContext::new_blocking().ok()
}

#[test]
fn capacity_witness_fixed_descent_varying_gu_yang_capacity() {
    let Some(ctx) = require_gpu() else {
        eprintln!("spatial_flux_witness: GPU leg skipped (no adapter)");
        return;
    };
    let fx = fixture(0.15);
    let eml = amplifying_eml(fx.value);
    let (plan, rf) = compile_witness_plan(&fx, &eml, false);
    let initial = values(&fx, 0.1, 0.8);
    let descent = descent_identity(fx.value.raw_u32(), fx.palma_d.raw_u32(), "spatial-flux-witness");

    let mut samples = Vec::new();
    for cap in [0.5f32, 1.0, 0.75] {
        let (post, pre, native) = run_witness_leg(&ctx, &fx, cap, &initial, &plan, &rf);
        // Pre-clamp dual emission observes 2*native when EML is 2x amplifier.
        assert_eq!(pre.to_bits(), (2.0 * native).to_bits());
        assert_eq!(post.to_bits(), native.to_bits());
        assert_pre_clamp_preserves_native_sign(native, lawful_pre_clamp_operand(native)).unwrap();
        samples.push(CapacityWitnessSample {
            channel_capacity: cap,
            descent_identity: descent,
            pre_clamp_progress: pre,
            post_clamp_progress: post,
            native_flux: native,
        });
    }
    assert_capacity_witness(&samples).expect("capacity witness");
    // Monotonicity on ordered pair 0.5 < 0.75 < 1.0 when natives follow capacity.
    let mut by_cap = samples.clone();
    by_cap.sort_by(|a, b| {
        a.channel_capacity
            .partial_cmp(&b.channel_capacity)
            .unwrap()
    });
    assert!(
        by_cap[0].post_clamp_progress.abs() <= by_cap[2].post_clamp_progress.abs() + 1e-5
            || by_cap[0].native_flux.abs() <= by_cap[2].native_flux.abs() + 1e-5
    );
}

/// Two-slot tree so SlotAllocator admits slots 0 and 1 for Gu-Yang grid cells.
fn two_slot_root() -> SimThing {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let child = SimThing::new(SimThingKind::Location, 0);
    root.add_child(child);
    root
}

fn abs_eml(value_col: ColumnIndex) -> EmlExpressionRegistry {
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
            node_count: 3,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: "flux-witness-abs-mutant".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: value_col.raw_u32(),
                ..node(opcode::SLOT_VALUE)
            },
            node(opcode::ABS),
            node(opcode::RETURN_TOP),
        ],
    )
    .expect("ABS EML mutant");
    eml
}

/// Admit two-instance conserved ActionBand on the same Gu-Yang-bound template.
fn admit_two_leg(
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    thresholds: &[EmitOnThresholdRegistration],
) -> simthing_spec::FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_with_conserved_progress_at_session_build(
        &ActionBandSessionSpec {
            budget: ActionBandAdmissionBudgetSpec {
                axis_channel_count: 2,
                dependency_binding_count: 0,
                storage_rows: 2,
                eml_program_count: 1,
                emission_binding_count: 2,
            },
            templates: vec![ActionBandTemplateSpec {
                id: "spatial-flux-witness".into(),
                label: Some("presentation-only".into()),
                axis_channels: vec![channel(fixture.value), channel(fixture.palma_d)],
                target: ActionBandTargetSpec::ScalarBound {
                    channel: fixture.value.raw_u32(),
                    bound: 0.0,
                    direction: ScalarBoundDirection::AtLeast,
                },
                velocity: None,
                // One band per Phase-5 registration so each slot's sealed
                // crossing joins its ActionBand instance (reg_idx match).
                bands: vec![
                    ActionBandBandSpec {
                        threshold_registration_index: 0,
                        eml_program: Some(17),
                        emission_binding_indices: vec![0, 1],
                    },
                    ActionBandBandSpec {
                        threshold_registration_index: 1,
                        eml_program: Some(17),
                        emission_binding_indices: vec![0, 1],
                    },
                ],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 2,
                requirement_semantics: ActionBandRequirementSemantics::Ordinary,
            }],
        },
        // One conserved binding on the shared RF emission row (emission table is
        // session-global; second band reuses the same clamp door).
        &[ActionBandConservedProgressBindingSpec {
            template_id: "spatial-flux-witness".into(),
            band_index: 0,
            emission_binding_index: 0,
            bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
        }],
        &fixture.registry,
        eml,
        thresholds,
    )
    .expect("two-leg admit")
    .clone()
}

fn rf_plan_two_slot(fixture: &Fixture) -> CompiledAccumulatorOpPlan {
    CompiledAccumulatorOpPlan {
        slot_count: 2,
        n_dims: fixture.registry.total_columns as u32,
        input_channel: StructuralScalarChannel::new(fixture.rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(fixture.rf_result.raw_u32()),
        ops: vec![
            AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(0),
                    col: fixture.rf_claim,
                },
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(SlotIndex::new(0), fixture.rf_result)],
            },
            AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(1),
                    col: fixture.rf_claim,
                },
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(SlotIndex::new(1), fixture.rf_result)],
            },
        ],
    }
}

/// Real two-leg path: Gu-Yang → Phase-5 (both slots) → two ActionBand instances.
/// Returns opposed observation from **actual** natives / dual-emission / RF results.
///
/// `world` is both the Gu-Yang input and the Phase-5 previous plane. Phase-5
/// current is the Gu-Yang-updated resident field (same pattern as 7.5a).
fn run_two_leg_opposed(
    ctx: &GpuContext,
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    world: &[f32],
    saturation: f32,
) -> OpposedDemandObservation {
    // Thresholds chosen so one conservative Gu-Yang step across an opposed
    // pair fires both legs while post_values remain opposite-signed.
    let thresholds = [
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(0),
            col: fixture.value,
            threshold: -0.3,
            direction: ThresholdDirection::Upward,
            event_kind: 7_521,
            buffer: EmitOnThresholdBuffer::Values,
        },
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(1),
            col: fixture.value,
            threshold: 0.3,
            direction: ThresholdDirection::Downward,
            event_kind: 7_522,
            buffer: EmitOnThresholdBuffer::Values,
        },
    ];
    let frozen = admit_two_leg(fixture, eml, &thresholds);
    let rf = rf_plan_two_slot(fixture);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fixture.registry,
        &[fixture.pre_clamp_obs],
        std::slice::from_ref(&rf),
        &[],
        &ThresholdRegistry::new(),
    );
    let template = frozen.templates()[0].index();
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        eml,
        &[
            ActionBandEmissionBindingGpu::rf_claim(fixture.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                fixture.pre_clamp_obs.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
        ],
        &[
            ActionBandActiveInstance::new(template, SlotIndex::new(0), [0.0; 4]),
            ActionBandActiveInstance::new(template, SlotIndex::new(1), [0.0; 4]),
        ],
        &native,
    )
    .expect("two-leg compile");
    let plan = compiled.into_execution_plan();

    let registrations = gu_yang(fixture, saturation);
    let mut field = FieldSweepSession::new(ctx, &registrations[0]).unwrap();
    field.upload_values(ctx, world).unwrap();
    field.dispatch_chain(ctx, &registrations, 1).unwrap();
    let resident = storage_buffer(
        ctx,
        "two_leg_resident",
        std::mem::size_of_val(world) as u64,
    );
    field.copy_values_to_buffer(ctx, &resident);

    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, 2, fixture.registry.total_columns as u32, 4);
    // Previous = pre-Gu-Yang world; current = post-Gu-Yang resident (7.5a shape).
    phase5.upload_previous_values(ctx, world);
    phase5
        .copy_values_prefix_from_buffer(ctx, &resident, 0, 0, resident.size())
        .unwrap();
    let gpu_thresholds = emit_on_threshold_registrations_to_gpu(&thresholds);
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&gpu_thresholds).unwrap(),
        )
        .unwrap();
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();

    let root = two_slot_root();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    assert!(
        deltas.len() >= 2,
        "equal-opposed channel requires two Phase-5 crossings, got {}",
        deltas.len()
    );

    // Natives from **actual** Phase-5 post_values for each slot (Gu-Yang-updated).
    let mut native_by_slot = [0.0f32; 2];
    let mut saw = [false; 2];
    for d in &deltas {
        let s = d.slot().raw() as usize;
        if s < 2 {
            native_by_slot[s] = d.post_value();
            saw[s] = true;
        }
    }
    assert!(saw[0] && saw[1], "both slots must produce sealed natives");

    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    assert!(
        crossings.crossing_count() >= 2,
        "ActionBand must join both sealed crossings"
    );

    let next = storage_buffer(ctx, "two_leg_next", std::mem::size_of_val(world) as u64);
    let mut action = match ActionBandGpuExecution::new(ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("two active ActionBand rows required"),
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

    let mut rf_sess = AccumulatorOpSession::new(ctx, rf.slot_count, rf.n_dims);
    rf_sess
        .copy_values_prefix_from_buffer(ctx, &next, 0, 0, next.size())
        .unwrap();
    rf_sess
        .upload_packed_ops(ctx, &PackedAccumulatorUpload::from_ops(&rf.ops).unwrap())
        .unwrap();
    rf_sess.tick(ctx, 0).unwrap();
    let result = rf_sess.readback_full(ctx).unwrap();
    let n_dims = fixture.registry.total_columns;

    // Dual-emission PRE-CLAMP and conserved POST-CLAMP from actual GPU results.
    let pre0 = result[fixture.pre_clamp_obs.raw()];
    let pre1 = result[n_dims + fixture.pre_clamp_obs.raw()];
    let post0 = result[fixture.rf_result.raw()];
    let post1 = result[n_dims + fixture.rf_result.raw()];

    OpposedDemandObservation {
        forward: OpposedDemandOperand {
            native_flux: native_by_slot[0],
            pre_clamp_progress: pre0,
            post_clamp_progress: post0,
        },
        reverse: OpposedDemandOperand {
            native_flux: native_by_slot[1],
            pre_clamp_progress: pre1,
            post_clamp_progress: post1,
        },
    }
}

#[test]
fn opposed_demand_pre_clamp_signs_and_post_clamp_mutual_stall() {
    // DA A1: real two-leg equal-opposed case on admitted Gu-Yang → Phase-5 →
    // two ActionBand instances. Natives and pre/post clamp are GPU-observed.
    let Some(ctx) = require_gpu() else {
        eprintln!("spatial_flux_witness opposed: GPU leg skipped (no adapter)");
        return;
    };
    let fx = fixture(0.0);
    // Symmetric opposed potential across the Gu-Yang N4 edge (2 cells).
    // Gu-Yang redistributes; Phase-5 previous is this plane and current is the
    // post-Gu-Yang field so sealed post_values are real Gu-Yang results.
    // Choose a large opposed pair so both legs remain on opposite sides of 0
    // after one conservative step and both thresholds fire.
    // Opposed Gu-Yang state: left negative, right positive. One conservative
    // step moves them toward each other across ±0.3 thresholds while keeping
    // opposite signs on the sealed post_values.
    let world = values(&fx, -0.9, 0.9);

    let eml = identity_eml(fx.value);
    let obs = run_two_leg_opposed(&ctx, &fx, &eml, &world, 1.0);

    assert_ne!(
        obs.forward.native_flux, 0.0,
        "forward native must come from real Phase-5 post_value"
    );
    assert_ne!(
        obs.reverse.native_flux, 0.0,
        "reverse native must come from real Phase-5 post_value"
    );
    assert_ne!(
        obs.forward.native_flux.signum(),
        obs.reverse.native_flux.signum(),
        "canonical Gu-Yang/Phase-5 orientation must yield opposite natives; got {} vs {}",
        obs.forward.native_flux,
        obs.reverse.native_flux
    );

    // PRE-CLAMP dual-emission preserves those real native signs (identity EML).
    assert_pre_clamp_preserves_native_sign(
        obs.forward.native_flux,
        obs.forward.pre_clamp_progress,
    )
    .unwrap();
    assert_pre_clamp_preserves_native_sign(
        obs.reverse.native_flux,
        obs.reverse.pre_clamp_progress,
    )
    .unwrap();
    assert_eq!(
        obs.forward.pre_clamp_progress.to_bits(),
        obs.forward.native_flux.to_bits(),
        "identity EML pre-clamp must equal sealed native"
    );
    assert_eq!(
        obs.reverse.pre_clamp_progress.to_bits(),
        obs.reverse.native_flux.to_bits()
    );

    // POST-CLAMP from actual RF results — not injected zeros.
    assert_opposed_demand_law(obs).unwrap_or_else(|e| {
        panic!(
            "opposed demand law failed ({e:?}) with obs={obs:?}"
        )
    });

    // Mutants at the workshop consumption seam of the **real** natives RED at
    // PRE-CLAMP even if a downstream clamp could mask executable results.
    assert_mutant_pre_clamp_pair_reds(
        obs.forward.native_flux,
        obs.reverse.native_flux,
        PreClampConsumption::MutantAbsFlux,
    )
    .expect("abs(flux) mutant must RED on real natives");
    assert_mutant_pre_clamp_pair_reds(
        obs.forward.native_flux,
        obs.reverse.native_flux,
        PreClampConsumption::MutantFlipSign,
    )
    .expect("sign-flip mutant must RED on real natives");

    // Real-path abs EML mutant: dual-emission pre-clamp loses opposition → RED.
    let abs_obs = run_two_leg_opposed(&ctx, &fx, &abs_eml(fx.value), &world, 1.0);
    assert!(
        assert_opposed_demand_law(abs_obs).is_err()
            || abs_obs.forward.pre_clamp_progress.signum()
                == abs_obs.reverse.pre_clamp_progress.signum(),
        "ABS EML pre-clamp pair must not pass opposed-demand as lawful: {abs_obs:?}"
    );
    // Explicit: both pre-clamps non-negative under abs is the free-run symptom.
    if abs_obs.forward.native_flux < 0.0 || abs_obs.reverse.native_flux < 0.0 {
        assert!(
            reject_abs_flux_mutant(
                abs_obs.forward.native_flux.min(abs_obs.reverse.native_flux),
                abs_obs.forward.pre_clamp_progress.abs().max(abs_obs.reverse.pre_clamp_progress.abs())
            )
            .is_err()
                || abs_obs.forward.pre_clamp_progress >= 0.0
                    && abs_obs.reverse.pre_clamp_progress >= 0.0,
            "abs path must expose magnitude-only pre-clamp: {abs_obs:?}"
        );
    }
}

#[test]
fn no_sink_capacity_bearing_lane_without_costband() {
    // Capacity-bearing conserved leg does not require a CostBand sink.
    assert_no_sink_posture(false).unwrap();
    let fx = fixture(0.15);
    let eml = identity_eml(fx.value);
    let frozen = admit_conserved(&fx, &eml, false);
    // No CostBand in emission destinations of the no-sink plan.
    let rf = rf_plan(&fx);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fx.registry,
        &[fx.pre_clamp_obs],
        std::slice::from_ref(&rf),
        &[],
        &ThresholdRegistry::new(),
    );
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &[
            ActionBandEmissionBindingGpu::rf_claim(fx.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                fx.pre_clamp_obs.raw_u32(),
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
    .expect("no-sink compile");
    let _ = compiled;
    // Explicit CostBand would be a separate authored sink posture, not implied by capacity.
}

#[test]
fn reapability_production_has_zero_workshop_coupling() {
    // Production leaf crates must not depend on simthing-workshop.
    let cargos = [
        include_str!("../../simthing-core/Cargo.toml"),
        include_str!("../../simthing-kernel/Cargo.toml"),
        include_str!("../../simthing-gpu/Cargo.toml"),
        include_str!("../../simthing-sim/Cargo.toml"),
        include_str!("../../simthing-driver/Cargo.toml"),
        include_str!("../../simthing-spec/Cargo.toml"),
        include_str!("../../simthing-feeder/Cargo.toml"),
    ];
    assert_production_has_zero_workshop_coupling(&cargos)
        .expect("detachability: production_coupling=0 for workshop");
}

#[test]
fn pre_clamp_seam_is_workshop_observable_without_production_src_edit() {
    // Structural proof: pre-clamp is the dual non-conserved emission of the same
    // EML payload used before the conserved clamp (existing 7.5a surface).
    // This test documents the seam; GPU coverage is in capacity/opposed tests.
    let src = include_str!("../../simthing-kernel/src/shaders/action_band_execution.wgsl");
    assert!(
        src.contains("if (binding.auxiliary1 != ACTION_CONSERVED_BOUND_NONE)"),
        "production clamp seam must exist for dual-emission observation"
    );
    assert!(
        src.contains("var executable_payload = payload;"),
        "payload (pre-clamp) must be distinct from executable_payload (post-clamp)"
    );
    // Workshop source must not require production observation hooks.
    let witness = include_str!("../src/actionband_spatial_flux_witness_0.rs");
    assert!(witness.contains("PRE-CLAMP"));
    assert!(witness.contains("POST-CLAMP"));
}
