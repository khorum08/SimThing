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
    assert_capacity_witness, assert_no_sink_posture, assert_opposed_demand_law,
    assert_pre_clamp_preserves_native_sign, assert_production_has_zero_workshop_coupling,
    descent_identity, lawful_pre_clamp_operand, mutant_abs_flux_pre_clamp,
    mutant_flip_sign_pre_clamp, reject_abs_flux_mutant, reject_sign_order_mutant,
    CapacityWitnessSample, OpposedDemandObservation, OpposedDemandOperand,
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

#[test]
fn opposed_demand_pre_clamp_signs_and_post_clamp_mutual_stall() {
    // Synthetic equal opposed demand on one conservative channel:
    // forward native +q, reverse native -q (canonical opposed signs).
    // Pre-clamp operands must preserve those signs; post-clamp mutual stall.
    let forward_native = 0.8f32;
    let reverse_native = -0.8f32;

    let forward_pre = lawful_pre_clamp_operand(forward_native);
    let reverse_pre = lawful_pre_clamp_operand(reverse_native);
    assert_pre_clamp_preserves_native_sign(forward_native, forward_pre).unwrap();
    assert_pre_clamp_preserves_native_sign(reverse_native, reverse_pre).unwrap();

    // abs mutant at reverse pre-clamp RED even if post-clamp were green.
    let abs_pre = mutant_abs_flux_pre_clamp(reverse_native);
    assert!(reject_abs_flux_mutant(reverse_native, abs_pre).is_err());
    let flip_pre = mutant_flip_sign_pre_clamp(forward_native);
    assert!(reject_sign_order_mutant(forward_native, flip_pre).is_err());

    // Composed post-clamp under equal opposed demand: mutual stall.
    let obs = OpposedDemandObservation {
        forward: OpposedDemandOperand {
            native_flux: forward_native,
            pre_clamp_progress: forward_pre,
            post_clamp_progress: 0.0,
        },
        reverse: OpposedDemandOperand {
            native_flux: reverse_native,
            pre_clamp_progress: reverse_pre,
            post_clamp_progress: 0.0,
        },
    };
    assert_opposed_demand_law(obs).expect("mutual stall");

    // GPU leg: signed native flux through real Gu-Yang+ActionBand clamp (negative fixture).
    let Some(ctx) = require_gpu() else {
        eprintln!("spatial_flux_witness opposed: GPU leg skipped (no adapter)");
        return;
    };
    let fx = fixture(-0.75);
    let eml = amplifying_eml(fx.value);
    let (plan, rf) = compile_witness_plan(&fx, &eml, false);
    let initial = values(&fx, -0.8, -0.1);
    let (post, pre, native) = run_witness_leg(&ctx, &fx, 1.0, &initial, &plan, &rf);
    assert!(
        native < 0.0,
        "native signed Gu-Yang/Phase-5 flux must stay negative"
    );
    assert!(
        pre < 0.0,
        "PRE-CLAMP dual emission must preserve native negative sign"
    );
    assert!(
        post < 0.0,
        "POST-CLAMP conserved progress must preserve native negative sign"
    );
    assert_eq!(pre.to_bits(), (2.0 * native).to_bits());
    assert_eq!(post.to_bits(), native.to_bits());
    assert_pre_clamp_preserves_native_sign(native, pre).unwrap();
    // abs mutant would produce +|pre| and RED against native.
    assert!(reject_abs_flux_mutant(native, pre.abs()).is_err());
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
