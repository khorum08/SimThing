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
    compile_action_band_gpu_execution_with_native_lanes, compile_comparative_bundle,
    compile_gu_yang_n4_field_sweeps, neighbor_slots_from_grid, ActionBandActiveInstance,
    ActionBandNativeLaneAdmission, ComparativeBandReadouts, ComparativeEmitterClass,
    ComparativeProjectionOutputs, ComparativeProjectionRequest, GuYangN4FieldSweepSpec,
    GuYangStallOutputs,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    scoped_debug_readback_allowed, wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu,
    ActionBandGpuExecution, FieldAdjacency, FieldSweepOutput, FieldSweepSession, GpuContext,
    PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator, GRID_N4_NSEW,
};
use simthing_sim::ThresholdRegistry;
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, ScalarBoundDirection,
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
    /// Authored equal-opposed demand axis (not Gu-Yang reconstruction).
    demand: ColumnIndex,
    /// Graduated Gu-Yang stall chain outputs (gross − |net|).
    net_flux: ColumnIndex,
    gross_flux: ColumnIndex,
    stall: ColumnIndex,
    /// Comparative contest (consumes stall under both-strong/small-margin).
    contest: ColumnIndex,
    dominance: ColumnIndex,
    margin: ColumnIndex,
    border: ColumnIndex,
    chokepoint: ColumnIndex,
    /// Comparative emitters (scenario-neutral class strengths).
    emitter_a: ColumnIndex,
    emitter_b: ColumnIndex,
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
    let demand = register("opposed-demand");
    let net_flux = register("guyang-net-flux");
    let gross_flux = register("guyang-gross-flux");
    let stall = register("guyang-stall");
    let contest = register("guyang-contest");
    let dominance = register("comp-dominance");
    let margin = register("comp-margin");
    let border = register("comp-border");
    let chokepoint = register("comp-chokepoint");
    let emitter_a = register("emitter-a");
    let emitter_b = register("emitter-b");
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
        demand,
        net_flux,
        gross_flux,
        stall,
        contest,
        dominance,
        margin,
        border,
        chokepoint,
        emitter_a,
        emitter_b,
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
    demand_eml(value_col) // identity read of named column
}

/// Desired progress from an authored demand axis (equal opposed ±demand).
fn demand_eml(demand_col: ColumnIndex) -> EmlExpressionRegistry {
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
            display_name: "flux-witness-opposed-demand".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: demand_col.raw_u32(),
                ..node(opcode::SLOT_VALUE)
            },
            node(opcode::RETURN_TOP),
        ],
    )
    .expect("demand EML");
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

fn gu_yang(
    fixture: &Fixture,
    saturation: f32,
    width: u32,
    height: u32,
) -> [simthing_gpu::FieldSweepRegistration; 2] {
    // chi * max_degree must stay within the admitted conductance bound (1.0).
    // 2×2 N4 cells have degree 2 → chi ≤ 0.5.
    let chi = if width * height >= 4 { 0.5 } else { 1.0 };
    compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width,
        height,
        n_dims: fixture.registry.total_columns as u32,
        value_col: fixture.value,
        conductance_col: fixture.conductance,
        saturation,
        chi,
        dt: 1.0,
    })
    .expect("gu-yang")
}

fn values(fixture: &Fixture, left: f32, right: f32) -> Vec<f32> {
    // 2×1 grid (capacity witness).
    let n_dims = fixture.registry.total_columns;
    let mut values = vec![0.0; 2 * n_dims];
    values[fixture.value.raw()] = left;
    values[n_dims + fixture.value.raw()] = right;
    values[fixture.palma_d.raw()] = 1.0;
    values[n_dims + fixture.palma_d.raw()] = 1.0;
    values
}

/// 2×2 grid world for opposed demand + multi-edge Gu-Yang stall.
fn values_2x2(
    fixture: &Fixture,
    cells: [f32; 4],
    demand: [f32; 4],
    emitters: [(f32, f32); 4],
) -> Vec<f32> {
    let n_dims = fixture.registry.total_columns;
    let mut values = vec![0.0; 4 * n_dims];
    for (s, (&u, (&d, &(ea, eb)))) in cells
        .iter()
        .zip(demand.iter().zip(emitters.iter()))
        .enumerate()
    {
        let base = s * n_dims;
        values[base + fixture.value.raw()] = u;
        values[base + fixture.conductance.raw()] = 0.5;
        values[base + fixture.palma_d.raw()] = 1.0;
        values[base + fixture.demand.raw()] = d;
        values[base + fixture.emitter_a.raw()] = ea;
        values[base + fixture.emitter_b.raw()] = eb;
    }
    values
}

fn four_slot_root() -> SimThing {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    for _ in 0..3 {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    root
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
    let registrations = gu_yang(fixture, saturation, 2, 1);
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
    allocator.install_initial_tree(&root);
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
        result[fixture.rf_result.raw()],     // post-clamp conserved progress
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
    let descent = descent_identity(
        fx.value.raw_u32(),
        fx.palma_d.raw_u32(),
        "spatial-flux-witness",
    );

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
    by_cap.sort_by(|a, b| a.channel_capacity.partial_cmp(&b.channel_capacity).unwrap());
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
                axis_channel_count: 4,
                dependency_binding_count: 0,
                storage_rows: 4,
                eml_program_count: 1,
                emission_binding_count: 2,
            },
            templates: vec![ActionBandTemplateSpec {
                id: "spatial-flux-witness".into(),
                label: Some("presentation-only".into()),
                axis_channels: vec![
                    channel(fixture.value),
                    channel(fixture.palma_d),
                    channel(fixture.demand),
                    channel(fixture.net_flux),
                ],
                target: ActionBandTargetSpec::ScalarBound {
                    channel: fixture.net_flux.raw_u32(),
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
                reserved_instance_rows: 4,
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
    .unwrap_or_else(|e| panic!("two-leg admit failed: {e:?}"))
    .clone()
}

fn rf_plan_two_slot(fixture: &Fixture) -> CompiledAccumulatorOpPlan {
    let ops = (0..4u32)
        .map(|s| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(s),
                col: fixture.rf_claim,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(s), fixture.rf_result)],
        })
        .collect();
    CompiledAccumulatorOpPlan {
        slot_count: 4,
        n_dims: fixture.registry.total_columns as u32,
        input_channel: StructuralScalarChannel::new(fixture.rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(fixture.rf_result.raw_u32()),
        ops,
    }
}

/// Run graduated comparative stall chain into `world` (mutates net/gross/stall/contest).
/// Returns (stall, contest) max over slots. Formula is production `gross − |net|`.
fn run_guyang_stall_contest_into(
    ctx: &GpuContext,
    fixture: &Fixture,
    world: &mut [f32],
) -> (f32, f32) {
    let n_dims = fixture.registry.total_columns as u32;
    let adjacency =
        FieldAdjacency::grid_n4(2, 2, GRID_N4_NSEW, fixture.value).expect("2x2 adjacency");
    let neighbor_slots = neighbor_slots_from_grid(&adjacency).expect("neighbor slots");
    let bundle = compile_comparative_bundle(ComparativeProjectionRequest {
        adjacency,
        neighbor_slots,
        n_dims,
        emitters: vec![
            ComparativeEmitterClass {
                authored_order: 0,
                class_id: 10.0,
                value_col: fixture.emitter_a,
            },
            ComparativeEmitterClass {
                authored_order: 1,
                class_id: 20.0,
                value_col: fixture.emitter_b,
            },
        ],
        outputs: ComparativeProjectionOutputs {
            dominance_col: fixture.dominance,
            margin_col: fixture.margin,
            contest_col: fixture.contest,
        },
        band_readouts: ComparativeBandReadouts {
            border_col: fixture.border,
            chokepoint_col: fixture.chokepoint,
        },
        palma_d_col: fixture.palma_d,
        guyang_value_col: fixture.value,
        guyang_conductance_col: fixture.conductance,
        stall_outputs: GuYangStallOutputs {
            net_flux_col: fixture.net_flux,
            gross_flux_col: fixture.gross_flux,
            stall_col: fixture.stall,
        },
        bands: Default::default(),
        authored_opt_out_reason: None,
    })
    .expect("comparative+stall bundle");
    assert!(
        !bundle.registrations.is_empty(),
        "stall chain must be born (need ≥2 emitters)"
    );

    let _scope = scoped_debug_readback_allowed(true);
    let mut field = FieldSweepSession::new(ctx, &bundle.registrations[0]).unwrap();
    field.upload_values(ctx, world).unwrap();
    field.dispatch_chain(ctx, &bundle.registrations, 1).unwrap();
    let out = field.readback(ctx).unwrap();
    world.copy_from_slice(&out);
    let n = fixture.registry.total_columns;
    let mut stall = 0.0f32;
    let mut contest = 0.0f32;
    for s in 0..4 {
        let base = s * n;
        stall = stall.max(out[base + fixture.stall.raw()].abs());
        contest = contest.max(out[base + fixture.contest.raw()].abs());
    }
    (stall, contest)
}

/// Real two-leg path: Gu-Yang → stall/contest observe → Phase-5 → two ActionBands.
///
/// PRE-CLAMP demand comes from authored demand axis (EML SLOT_VALUE).
/// POST-CLAMP is RF after GuYangRealized clamp to Phase-5 post_value.
/// Under equal opposed demand the fixture settles so executable progress is
/// mutually stalled while Gu-Yang stall/contest is positive.
fn run_two_leg_opposed(
    ctx: &GpuContext,
    fixture: &Fixture,
    eml: &EmlExpressionRegistry,
    world_in: &[f32],
    _saturation: f32,
) -> OpposedDemandObservation {
    // Phase-5 on **net_flux** after the graduated stall chain so the
    // GuYangRealized clamp bound is the sealed net (small under multi-edge
    // cancelation) while demand EML still reports opposed ±demand.
    // 1) Native Gu-Yang stall/contest first.
    let mut world = world_in.to_vec();
    let (stall_mag, contest_mag) = run_guyang_stall_contest_into(ctx, fixture, &mut world);
    let n_dims = fixture.registry.total_columns;
    // Seat legs on high-stall / low-|net| cells so GuYangRealized clamp bound
    // (sealed net post_value) is near zero → mutual stall of executable progress,
    // while demand EML still reports opposed non-zero desired progress.
    let mut seats: Vec<(usize, f32, f32)> = (0..4)
        .map(|s| {
            let base = s * n_dims;
            (
                s,
                world[base + fixture.net_flux.raw()],
                world[base + fixture.stall.raw()].abs(),
            )
        })
        .collect();
    seats.sort_by(|a, b| {
        // Prefer high stall, then low |net|.
        b.2.partial_cmp(&a.2)
            .unwrap()
            .then(a.1.abs().partial_cmp(&b.1.abs()).unwrap())
    });
    let slot_fwd = seats[0].0 as u32;
    let slot_rev = seats[1].0 as u32;
    assert!(
        seats[0].2 > 1e-4 || stall_mag > 1e-4,
        "need positive stall on seated legs; seats={seats:?} stall_mag={stall_mag}"
    );

    // Downward fire: previous net high positive → current sealed net (near 0 under cancelation).
    let thresholds = [
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot_fwd),
            col: fixture.net_flux,
            threshold: 0.1,
            direction: ThresholdDirection::Downward,
            event_kind: 7_521,
            buffer: EmitOnThresholdBuffer::Values,
        },
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot_rev),
            col: fixture.net_flux,
            threshold: 0.1,
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
            ActionBandActiveInstance::new(template, SlotIndex::new(slot_fwd), [0.0; 4]),
            ActionBandActiveInstance::new(template, SlotIndex::new(slot_rev), [0.0; 4]),
        ],
        &native,
    )
    .expect("two-leg compile");
    let plan = compiled.into_execution_plan();

    // Previous net high so Downward to sealed near-zero net fires with post≈0.
    let mut previous = world_in.to_vec();
    for s in 0..4 {
        previous[s * n_dims + fixture.net_flux.raw()] = 1.0;
    }

    let resident = storage_buffer(ctx, "two_leg_resident", (world.len() * 4) as u64);
    // Upload current world (with stall chain results) into a storage buffer.
    {
        use wgpu::util::DeviceExt;
        let tmp = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("two_leg_world_upload"),
                contents: bytemuck::cast_slice(&world),
                usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            });
        let mut enc = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("two_leg_copy_world"),
            });
        enc.copy_buffer_to_buffer(&tmp, 0, &resident, 0, (world.len() * 4) as u64);
        ctx.queue.submit(Some(enc.finish()));
    }

    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, 4, fixture.registry.total_columns as u32, 4);
    phase5.upload_previous_values(ctx, &previous);
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

    let root = four_slot_root();
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fixture.registry,
        &allocator,
    );
    if deltas.len() < 2 {
        let n0 = world[fixture.net_flux.raw()];
        let n1 = world[n_dims + fixture.net_flux.raw()];
        let s0 = world[fixture.stall.raw()];
        let s1 = world[n_dims + fixture.stall.raw()];
        let g0 = world[fixture.gross_flux.raw()];
        let ecount = emissions.len();
        panic!(
            "equal-opposed needs ≥2 Phase-5 crossings, got {} (emissions={ecount}); \
             net=[{n0},{n1}] stall=[{s0},{s1}] gross0={g0} stall_mag={stall_mag} contest_mag={contest_mag}",
            deltas.len()
        );
    }

    let mut native_fwd = None;
    let mut native_rev = None;
    for d in &deltas {
        let s = d.slot().raw();
        if s == slot_fwd {
            native_fwd = Some(d.post_value());
        }
        if s == slot_rev {
            native_rev = Some(d.post_value());
        }
    }
    let native_fwd = native_fwd.expect("forward leg native");
    let native_rev = native_rev.expect("reverse leg native");

    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    assert!(
        crossings.crossing_count() >= 2,
        "ActionBand must join both sealed crossings"
    );

    let next = storage_buffer(ctx, "two_leg_next", (world.len() * 4) as u64);
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

    let pre0 = result[slot_fwd as usize * n_dims + fixture.pre_clamp_obs.raw()];
    let pre1 = result[slot_rev as usize * n_dims + fixture.pre_clamp_obs.raw()];
    let post0 = result[slot_fwd as usize * n_dims + fixture.rf_result.raw()];
    let post1 = result[slot_rev as usize * n_dims + fixture.rf_result.raw()];

    OpposedDemandObservation {
        forward: OpposedDemandOperand {
            native_flux: native_fwd,
            pre_clamp_progress: pre0,
            post_clamp_progress: post0,
        },
        reverse: OpposedDemandOperand {
            native_flux: native_rev,
            pre_clamp_progress: pre1,
            post_clamp_progress: post1,
        },
        guyang_stall_magnitude: stall_mag,
        guyang_contest_magnitude: contest_mag,
    }
}

#[test]
fn opposed_demand_pre_clamp_signs_and_post_clamp_mutual_stall() {
    // DA A1 + mutual stall: real two-leg Gu-Yang → stall/contest → Phase-5 →
    // two ActionBands. PRE-CLAMP = demand dual-emission; POST-CLAMP = RF;
    // Gu-Yang stall/contest from graduated comparative stall chain.
    let Some(ctx) = require_gpu() else {
        eprintln!("spatial_flux_witness opposed: GPU leg skipped (no adapter)");
        return;
    };
    let fx = fixture(0.0);
    // 2×2 multi-edge grid: opposed demand on slots 0/1, both-strong emitters
    // so contest consumes stall. Gu-Yang values near zero so Phase-5 post_value
    // clamp bounds are near zero → mutual stall of executable progress while
    // demand EML still reports ±1.0 desired progress.
    // Mixed-sign neighbors so gross > |net| → stall = gross−|net| > 0.
    // Slot0 (u=0) has neighbors +1 and −1 → net≈0, stall>0.
    // Both-strong emitters so contest can consume stall.
    let world = values_2x2(
        &fx,
        [0.0, 1.0, -1.0, 0.0],  // gu-yang u
        [1.0, -1.0, 1.0, -1.0], // equal opposed demand on seats
        [(0.9, 0.85), (0.85, 0.9), (0.9, 0.85), (0.85, 0.9)],
    );

    let eml = demand_eml(fx.demand);
    let obs = run_two_leg_opposed(&ctx, &fx, &eml, &world, 1.0);

    assert!(
        obs.forward.pre_clamp_progress.abs() > 0.5 && obs.reverse.pre_clamp_progress.abs() > 0.5,
        "PRE-CLAMP demand must be non-trivial; got {obs:?}"
    );
    assert_ne!(
        obs.forward.pre_clamp_progress.signum(),
        obs.reverse.pre_clamp_progress.signum(),
        "PRE-CLAMP must preserve opposed demand signs; got {obs:?}"
    );
    assert!(
        obs.guyang_stall_magnitude > 1e-4 || obs.guyang_contest_magnitude > 1e-4,
        "native Gu-Yang stall/contest must be positive; got stall={} contest={}",
        obs.guyang_stall_magnitude,
        obs.guyang_contest_magnitude
    );

    // POST-CLAMP mutual stall from real RF — not free-run cancellation.
    assert_opposed_demand_law(obs)
        .unwrap_or_else(|e| panic!("opposed demand law failed ({e:?}) with obs={obs:?}"));
    assert!(
        obs.forward.post_clamp_progress.abs() <= 1e-2
            && obs.reverse.post_clamp_progress.abs() <= 1e-2,
        "both POST-CLAMP legs must stall (near-zero progress); got {obs:?}"
    );

    // Mutants on the real pre-clamp demand pair RED (abs loses opposition).
    let demand_pair_native_for_mutant = (
        obs.forward.pre_clamp_progress,
        obs.reverse.pre_clamp_progress,
    );
    assert_mutant_pre_clamp_pair_reds(
        demand_pair_native_for_mutant.0,
        demand_pair_native_for_mutant.1,
        PreClampConsumption::MutantAbsFlux,
    )
    .expect("abs(flux) mutant must RED on real demand pair");
    assert_mutant_pre_clamp_pair_reds(
        demand_pair_native_for_mutant.0,
        demand_pair_native_for_mutant.1,
        PreClampConsumption::MutantFlipSign,
    )
    .expect("sign-flip mutant must RED on real demand pair");

    // ABS EML path: magnitude-only demand loses opposition at PRE-CLAMP.
    let abs_obs = run_two_leg_opposed(&ctx, &fx, &abs_eml(fx.demand), &world, 1.0);
    assert!(
        assert_opposed_demand_law(abs_obs).is_err()
            || abs_obs.forward.pre_clamp_progress.signum()
                == abs_obs.reverse.pre_clamp_progress.signum(),
        "ABS EML must not pass opposed-demand as lawful: {abs_obs:?}"
    );
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
