//! ACTIONBAND-FULL-FIELD-TRIAD-MOVEMENT-VENDOR-0 (7.5c).
//!
//! New born-mortal positive vendor proof. Production crates are consumed only.

use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode,
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlTreeId, GateSpec, ScaleSpec,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, SourceSpec,
    StructuralScalarChannel, SubFieldRole, ThresholdDirection,
};
use simthing_driver::{
    compile_action_band_gpu_execution_with_native_lanes, compile_comparative_bundle,
    compile_crossing_consequence_session, compile_gu_yang_n4_field_sweeps,
    compile_palma_n4_field_sweep, neighbor_slots_from_grid, ActionBandActiveInstance,
    ActionBandExecutionCompileError, ActionBandNativeLaneAdmission, ComparativeBandReadouts,
    ComparativeEmitterClass, ComparativeProjectionOutputs, ComparativeProjectionRequest,
    GuYangN4FieldSweepSpec, GuYangStallOutputs, PalmaN4FieldSweepSpec, StructuralAuthorization,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu,
    readback_buffer_bytes_blocking, scoped_debug_readback_allowed, wgpu, AccumulatorOpSession,
    ActionBandEmissionBindingGpu, ActionBandGpuExecution, FieldAdjacency, FieldSweepSession,
    GpuContext, PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator, GRID_N4_NSEW,
    MIN_PLUS_INF,
};
use simthing_sim::overlay_lifecycle::resolve_overlay_lifecycle;
use simthing_sim::{
    apply_structural_mutations, CostBandSemantic, SimRuntimeTree, ThresholdRegistry,
    ThresholdSemantic,
};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, FrozenActionBandTemplates,
};
use simthing_workshop::actionband_full_field_triad_movement_vendor_0::{
    assert_arrival_collapses, assert_capacity_and_next_generation_law,
    assert_costband_is_downstream, assert_costband_scaling_mutant_reds,
    assert_semantic_mutations_are_authority_blind, local_descent_identity,
    native_bound_source_code, ArrivalLifecycleObservation, CostBandOrderingObservation,
    SealedVendorAuthority, SemanticMutationProjection, VendorGenerationSample,
};
use simthing_workshop::actionband_spatial_flux_witness_0::{
    assert_opposed_demand_law, FluxWitnessError, OpposedDemandObservation, OpposedDemandOperand,
};
use simthing_workshop::actionband_spatial_vendorization_0::{
    AdmittedTopologyCell, SpatialStepOverlayEffect, SpatialVendorizationStep,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const WIDTH: u32 = 4;
const HEIGHT: u32 = 1;
const ACTOR_SLOT: u32 = 0;
const STEP_SLOT: u32 = 1;
const TARGET_SLOT: u32 = 3;
const MOVEMENT_EVENT: u32 = 0x375C_0001;

struct Fixture {
    registry: DimensionRegistry,
    d: ColumnIndex,
    w: ColumnIndex,
    flux: ColumnIndex,
    conductance: ColumnIndex,
    rf_claim: ColumnIndex,
    rf_result: ColumnIndex,
    desired: ColumnIndex,
    cost_progress: ColumnIndex,
    demand: ColumnIndex,
    net_flux: ColumnIndex,
    gross_flux: ColumnIndex,
    stall: ColumnIndex,
    contest: ColumnIndex,
    dominance: ColumnIndex,
    margin: ColumnIndex,
    border: ColumnIndex,
    chokepoint: ColumnIndex,
    emitter_a: ColumnIndex,
    emitter_b: ColumnIndex,
    movement_threshold: EmitOnThresholdRegistration,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut column = |name: &str| {
        let property = registry.register(SimProperty::simple("movement-vendor", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .expect("amount column")
    };
    let d = column("palma-d");
    let w = column("stead-impedance");
    let flux = column("guyang-realized");
    let conductance = column("guyang-conductance");
    let rf_claim = column("rf-claim");
    let rf_result = column("rf-result");
    let desired = column("eml-desired");
    let cost_progress = column("costband-progress");
    let demand = column("opposed-demand");
    let net_flux = column("guyang-net");
    let gross_flux = column("guyang-gross");
    let stall = column("guyang-stall");
    let contest = column("guyang-contest");
    let dominance = column("dominance");
    let margin = column("margin");
    let border = column("border");
    let chokepoint = column("chokepoint");
    let emitter_a = column("emitter-a");
    let emitter_b = column("emitter-b");
    let movement_threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(STEP_SLOT),
        col: flux,
        threshold: 0.11,
        direction: ThresholdDirection::Upward,
        event_kind: MOVEMENT_EVENT,
        buffer: EmitOnThresholdBuffer::Values,
    };
    Fixture {
        registry,
        d,
        w,
        flux,
        conductance,
        rf_claim,
        rf_result,
        desired,
        cost_progress,
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
        movement_threshold,
    }
}

fn channel(column: ColumnIndex, kind: ActionBandChannelKind) -> ActionBandChannelBindingSpec {
    ActionBandChannelBindingSpec {
        column: column.raw_u32(),
        kind,
    }
}

fn eml(column: ColumnIndex, absolute: bool) -> EmlExpressionRegistry {
    use simthing_core::eml_nodes::{opcode, EmlNode};
    let node = |opcode| EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let mut nodes = vec![EmlNode {
        opcode: opcode::SLOT_VALUE,
        a: column.raw_u32(),
        ..node(opcode::SLOT_VALUE)
    }];
    nodes.push(EmlNode {
        opcode: opcode::LITERAL_F32,
        a: 2.0f32.to_bits(),
        ..node(opcode::LITERAL_F32)
    });
    nodes.push(node(opcode::MUL));
    if absolute {
        nodes.push(node(opcode::ABS));
    }
    nodes.push(node(opcode::RETURN_TOP));
    let mut registry = EmlExpressionRegistry::new();
    registry
        .register_formula(
            EmlTreeId(75),
            EmlFormulaMeta {
                tree_id: EmlTreeId(75),
                execution_class: EmlExecutionClass::ExactDeterministic,
                allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
                max_abs_error: None,
                deterministic_gpu: true,
                requires_guard_for_hard_threshold: false,
                node_count: nodes.len() as u32,
                max_stack_depth: 2,
                has_loops: false,
                has_recursion: false,
                display_name: if absolute {
                    "movement-vendor-abs-mutant".into()
                } else {
                    "movement-vendor-2x-desired".into()
                },
            },
            nodes,
        )
        .expect("movement EML");
    registry
}

fn session_spec(
    fx: &Fixture,
    authored_template_id: &str,
    label: Option<&str>,
) -> ActionBandSessionSpec {
    ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 2,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 4,
        },
        templates: vec![ActionBandTemplateSpec {
            id: authored_template_id.into(),
            label: label.map(str::to_owned),
            axis_channels: vec![
                channel(fx.d, ActionBandChannelKind::CachedDerived),
                channel(fx.flux, ActionBandChannelKind::CachedDerived),
            ],
            target: ActionBandTargetSpec::PalmaReachableSet {
                distance_channel: fx.d.raw_u32(),
                maximum_distance: 2.0,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(75),
                emission_binding_indices: vec![0, 1, 2, 3],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: ActionBandRequirementSemantics::Ordinary,
        }],
    }
}

fn admit(
    fx: &Fixture,
    programs: &EmlExpressionRegistry,
    authored_template_id: &str,
    label: Option<&str>,
) -> FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_with_conserved_progress_at_session_build(
        &session_spec(fx, authored_template_id, label),
        &[ActionBandConservedProgressBindingSpec {
            template_id: authored_template_id.into(),
            band_index: 0,
            emission_binding_index: 0,
            bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
        }],
        &fx.registry,
        programs,
        std::slice::from_ref(&fx.movement_threshold),
    )
    .expect("7.5c admission")
    .clone()
}

fn rf_plan(fx: &Fixture) -> CompiledAccumulatorOpPlan {
    CompiledAccumulatorOpPlan {
        slot_count: WIDTH * HEIGHT,
        n_dims: fx.registry.total_columns as u32,
        input_channel: StructuralScalarChannel::new(fx.rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(fx.rf_result.raw_u32()),
        ops: (0..WIDTH * HEIGHT)
            .map(|slot| AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(slot),
                    col: fx.rf_claim,
                },
                combine: CombineFn::Identity,
                gate: GateSpec::Always,
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(SlotIndex::new(slot), fx.rf_result)],
            })
            .collect(),
    }
}

struct CompiledVendor {
    frozen: FrozenActionBandTemplates,
    compiled: simthing_driver::CompiledActionBandGpuExecution,
    programs: EmlExpressionRegistry,
    native: ActionBandNativeLaneAdmission,
    rf: CompiledAccumulatorOpPlan,
    cost_registry: ThresholdRegistry,
    cost_event: u32,
}

fn compile_vendor(
    fx: &Fixture,
    programs: &EmlExpressionRegistry,
    authored_template_id: &str,
    label: Option<&str>,
) -> CompiledVendor {
    let frozen = admit(fx, programs, authored_template_id, label);
    let rf = rf_plan(fx);
    let mut cost_registry = ThresholdRegistry::new();
    let cost_event = cost_registry.push_with_cost_band(
        ThresholdSemantic::ScriptedEventTrigger {
            event_id: "movement-vendor-cost".into(),
        },
        CostBandSemantic::admit_sink(None, None).expect("sink admission"),
    );
    let cost_threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(STEP_SLOT),
        col: fx.cost_progress,
        threshold: 0.25,
        direction: ThresholdDirection::Upward,
        event_kind: cost_event,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fx.registry,
        &[fx.desired],
        std::slice::from_ref(&rf),
        std::slice::from_ref(&cost_threshold),
        &cost_registry,
    );
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        programs,
        &[
            ActionBandEmissionBindingGpu::rf_claim(fx.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                fx.desired.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
            ActionBandEmissionBindingGpu::cost_band(fx.cost_progress.raw_u32()),
            ActionBandEmissionBindingGpu::structural_request(0),
        ],
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            SlotIndex::new(STEP_SLOT),
            [0.0; 4],
        )],
        &native,
    )
    .expect("compile full vendor");
    CompiledVendor {
        frozen,
        compiled,
        programs: programs.clone(),
        native,
        rf,
        cost_registry,
        cost_event,
    }
}

fn initial_values(fx: &Fixture, ordinary_condition: f32) -> Vec<f32> {
    let n = fx.registry.total_columns;
    let mut values = vec![0.0; WIDTH as usize * n];
    for slot in 0..WIDTH as usize {
        let base = slot * n;
        values[base + fx.d.raw()] = if slot as u32 == TARGET_SLOT {
            0.0
        } else {
            MIN_PLUS_INF
        };
        values[base + fx.w.raw()] = 1.0;
        // One ordinary STEAD/RF pressure condition changes only the source
        // field presented to the already-admitted Gu-Yang chain. PALMA
        // impedance and topology remain fixed, so the local descent cannot be
        // silently rerouted by this next-generation condition witness.
        values[base + fx.flux.raw()] = if slot as u32 == STEP_SLOT {
            0.1
        } else if slot as u32 == ACTOR_SLOT {
            0.8 * ordinary_condition
        } else {
            0.8
        };
        values[base + fx.demand.raw()] = if slot % 2 == 0 { 1.0 } else { -1.0 };
        values[base + fx.emitter_a.raw()] = if slot % 2 == 0 { 0.9 } else { 0.85 };
        values[base + fx.emitter_b.raw()] = if slot % 2 == 0 { 0.85 } else { 0.9 };
    }
    values
}

fn storage(ctx: &GpuContext, label: &str, bytes: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[derive(Debug)]
struct VendorRun {
    generation: u32,
    saturation: f32,
    native_flux: f32,
    desired: f32,
    physical_progress: f32,
    cost_progress: f32,
    unit_price: f32,
    cost_draw: simthing_core::CostBandDraw,
    palma_d: Vec<f32>,
    commitment: simthing_kernel::StructuralCommitment,
    plan_fingerprint: u64,
    consequence_parity_digest: u64,
    consequence_door_reparented: bool,
}

fn run_vendor(
    ctx: &GpuContext,
    fx: &Fixture,
    vendor: &CompiledVendor,
    generation: u32,
    saturation: f32,
    ordinary_condition: f32,
    unit_price: f32,
) -> VendorRun {
    let initial = initial_values(fx, ordinary_condition);
    let palma = compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
        width: WIDTH,
        height: HEIGHT,
        n_dims: fx.registry.total_columns as u32,
        d_col: fx.d,
        w_col: fx.w,
        destination_slot: SlotIndex::new(TARGET_SLOT),
        inf_sentinel: MIN_PLUS_INF,
    })
    .expect("PALMA registration");
    let mut palma_session = FieldSweepSession::new(ctx, &palma).expect("PALMA session");
    palma_session
        .upload_values(ctx, &initial)
        .expect("PALMA upload");
    palma_session
        .dispatch(ctx, &palma, WIDTH)
        .expect("PALMA dispatch");
    let palma_values = {
        let _proof = scoped_debug_readback_allowed(true);
        palma_session.readback(ctx).expect("PALMA proof readback")
    };
    let n = fx.registry.total_columns;
    let palma_d: Vec<_> = (0..WIDTH as usize)
        .map(|slot| palma_values[slot * n + fx.d.raw()])
        .collect();
    assert!(
        palma_d[STEP_SLOT as usize] < palma_d[ACTOR_SLOT as usize],
        "authored adjacent step must be lawful local descent: {palma_d:?}"
    );

    let palma_resident = storage(ctx, "movement_vendor_palma", (initial.len() * 4) as u64);
    palma_session.copy_values_to_buffer(ctx, &palma_resident);
    let guyang = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: WIDTH,
        height: HEIGHT,
        n_dims: fx.registry.total_columns as u32,
        value_col: fx.flux,
        conductance_col: fx.conductance,
        saturation,
        chi: 0.5,
        dt: 1.0,
    })
    .expect("Gu-Yang registrations");
    let mut guyang_session = FieldSweepSession::new(ctx, &guyang[0]).expect("Gu-Yang session");
    guyang_session.upload_values_from_buffer(ctx, &palma_resident);
    guyang_session
        .dispatch_chain(ctx, &guyang, 1)
        .expect("Gu-Yang chain");
    let guyang_values = {
        let _proof = scoped_debug_readback_allowed(true);
        guyang_session
            .readback(ctx)
            .expect("Gu-Yang consequence-door input")
    };
    let resident = storage(ctx, "movement_vendor_resident", (initial.len() * 4) as u64);
    guyang_session.copy_values_to_buffer(ctx, &resident);

    let mut phase5 =
        AccumulatorOpSession::new_attached(ctx, WIDTH, fx.registry.total_columns as u32, 4);
    phase5.bind_generation_authority(generation);
    assert_eq!(phase5.generation(), generation);
    phase5.upload_previous_values(ctx, &palma_values);
    phase5
        .copy_values_prefix_from_buffer(ctx, &resident, 0, 0, resident.size())
        .expect("resident into Phase-5");
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&emit_on_threshold_registrations_to_gpu(
                std::slice::from_ref(&fx.movement_threshold),
            ))
            .expect("threshold pack"),
        )
        .expect("threshold upload");
    phase5.tick(ctx, 0).expect("Phase-5 crossing");
    let emissions = phase5
        .readback_threshold_emissions(ctx)
        .expect("sealed emissions");
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    for _ in 0..WIDTH - 1 {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fx.registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 1, "one native crossing required: {deltas:?}");
    let native_flux = deltas[0].post_value();
    let plan = vendor.compiled.execution_plan().clone();
    let crossings = plan
        .crossings_from_sealed(&deltas)
        .expect("ActionBand joins Phase-5 only");
    let next = storage(ctx, "movement_vendor_next", resident.size());
    let mut action = match ActionBandGpuExecution::new(ctx, plan).expect("ActionBand GPU") {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("movement vendor ActionBand inactive"),
    };
    let production = action
        .dispatch_with_native_next(
            ctx,
            &resident,
            &next,
            fx.registry.total_columns as u32,
            &crossings,
        )
        .expect("ActionBand native+structural dispatch");
    assert_eq!(production.commitments.len(), 1);

    // 7.8 oracle-first migration: the graduated 7.5c binding executes through
    // the canonical consequence door. The old low-level dispatch above is
    // retained here only as the bit-exact oracle for the migration proof.
    let (mut door_tree, mut door_allocator, _cells, actor, ids) = topology();
    let consequence_rows = vec![
        vendor
            .native
            .bind_resident_next(ActionBandEmissionBindingGpu::rf_claim(
                fx.rf_claim.raw_u32(),
            ))
            .expect("admitted RF Next consequence"),
        vendor
            .native
            .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
                fx.desired.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ))
            .expect("admitted property Next consequence"),
        vendor
            .native
            .bind_resident_next(ActionBandEmissionBindingGpu::cost_band(
                fx.cost_progress.raw_u32(),
            ))
            .expect("admitted CostBand Next consequence"),
        StructuralAuthorization::admit(BoundaryRequest::Reparent {
            child: actor,
            new_parent: ids[STEP_SLOT as usize],
        })
        .expect("admitted structural consequence"),
    ];
    let door = compile_crossing_consequence_session(
        &vendor.frozen,
        &vendor.programs,
        &consequence_rows,
        &[ActionBandActiveInstance::new(
            vendor.frozen.templates()[0].index(),
            SlotIndex::new(STEP_SLOT),
            [0.0; 4],
        )],
        &vendor.native,
    )
    .expect("compile 7.5c through the 7.8 consequence door");
    assert_eq!(
        door.compiled().plan_fingerprint(),
        vendor.compiled.plan_fingerprint(),
        "the door must preserve the graduated numeric plan"
    );
    let (door_tx, door_rx) = feeder_channel();
    let mut door_dispatch = door
        .bind_dispatch(ctx, &guyang_values)
        .expect("bind facility-local resident plane");
    let door_outcome = door_dispatch
        .dispatch_and_apply(ctx, fx.registry.total_columns as u32, crossings, &door_tx)
        .expect("one canonical crossing consequence dispatch");
    assert_eq!(door_outcome.structural_authorizations, 1);
    let door_values = {
        let _proof = scoped_debug_readback_allowed(true);
        door_dispatch
            .resident_current_for_proof(ctx)
            .expect("door resident proof")
    };
    let old_bytes = readback_buffer_bytes_blocking(
        &ctx.device,
        &ctx.queue,
        &next,
        next.size(),
        "movement_vendor_7_5c_oracle_next",
    )
    .expect("old 7.5c oracle readback");
    let old_values: &[f32] = bytemuck::cast_slice(&old_bytes);
    let base = STEP_SLOT as usize * fx.registry.total_columns;
    let mut consequence_hasher = std::collections::hash_map::DefaultHasher::new();
    for column in [fx.rf_claim, fx.desired, fx.cost_progress] {
        let old_bits = old_values[base + column.raw()].to_bits();
        let door_bits = door_values[base + column.raw()].to_bits();
        assert_eq!(door_bits, old_bits, "7.5c native Next parity at {column:?}");
        door_bits.hash(&mut consequence_hasher);
    }
    let door_requests = door_rx
        .drain_now()
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("consequence door emitted a non-boundary work item"),
        })
        .collect::<Vec<_>>();
    let mut door_registry = fx.registry.clone();
    let mut door_shadow = vec![0.0; door_allocator.capacity() * door_registry.total_columns];
    let door_applied = apply_structural_mutations(
        door_requests,
        &mut door_tree,
        &mut door_allocator,
        &mut door_registry,
        &mut door_shadow,
        fx.registry.total_columns,
        None,
        simthing_core::GenerationStamp::new(1),
        &mut simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState::default(),
        &std::collections::BTreeMap::new(),
    );
    let consequence_door_reparented =
        door_applied.reparented == vec![(actor, ids[STEP_SLOT as usize])];
    assert!(
        consequence_door_reparented,
        "the 7.5c structural consequence must use the ordinary boundary"
    );

    let mut rf = AccumulatorOpSession::new(ctx, vendor.rf.slot_count, vendor.rf.n_dims);
    rf.copy_values_prefix_from_buffer(ctx, &next, 0, 0, next.size())
        .expect("next into RF");
    rf.upload_packed_ops(
        ctx,
        &PackedAccumulatorUpload::from_ops(&vendor.rf.ops).expect("RF pack"),
    )
    .expect("RF upload");
    rf.tick(ctx, 0).expect("RF tick");
    let result = rf.readback_full(ctx).expect("proof readback");
    let base = STEP_SLOT as usize * n;
    let cost_progress = result[base + fx.cost_progress.raw()];
    let mut cost_registry = vendor.cost_registry.clone();
    let cost_draw = cost_registry
        .resolve_cost_band_draw(vendor.cost_event, cost_progress.abs(), unit_price)
        .expect("admitted CostBand sink draw");
    VendorRun {
        generation: phase5.generation(),
        saturation,
        native_flux,
        desired: result[base + fx.desired.raw()],
        physical_progress: result[base + fx.rf_result.raw()],
        cost_progress,
        unit_price,
        cost_draw,
        palma_d,
        commitment: production.commitments[0],
        plan_fingerprint: vendor.compiled.plan_fingerprint(),
        consequence_parity_digest: consequence_hasher.finish(),
        consequence_door_reparented,
    }
}

fn topology() -> (
    SimRuntimeTree,
    SlotAllocator,
    Vec<AdmittedTopologyCell>,
    SimThingId,
    [SimThingId; 4],
) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut ids = [SimThingId::new(); 4];
    let mut actor = None;
    for slot in 0..WIDTH {
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        ids[slot as usize] = cell.id;
        if slot == ACTOR_SLOT {
            let unit = SimThing::new(SimThingKind::Cohort, 0);
            actor = Some(unit.id);
            cell.add_child(unit);
        }
        root.add_child(cell);
    }
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let cells = (0..WIDTH)
        .map(|slot| AdmittedTopologyCell {
            sealed_slot: slot,
            sealed_col: 0,
            grid_row: 0,
            grid_col: slot,
            cell: ids[slot as usize],
        })
        .collect();
    (
        SimRuntimeTree::admit(root),
        allocator,
        cells,
        actor.expect("actor"),
        ids,
    )
}

fn consequence_fingerprint(actor: SimThingId, target: SimThingId) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    actor.raw().hash(&mut hasher);
    target.raw().hash(&mut hasher);
    hasher.finish()
}

fn numeric_binding_fingerprint(vendor: &CompiledVendor) -> u64 {
    let binding = vendor.compiled.conserved_progress_bindings()[0];
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    binding.template().raw().hash(&mut hasher);
    binding.band_table_index().hash(&mut hasher);
    binding.emission_binding_index().hash(&mut hasher);
    binding.bound_source().hash(&mut hasher);
    binding.threshold_column().raw_u32().hash(&mut hasher);
    binding.destination().hash(&mut hasher);
    hasher.finish()
}

fn admit_opposed(
    fx: &Fixture,
    programs: &EmlExpressionRegistry,
    thresholds: &[EmitOnThresholdRegistration],
) -> FrozenActionBandTemplates {
    let mut door = ActionBandSessionBuildDoor::new();
    door.admit_once_with_conserved_progress_at_session_build(
        &ActionBandSessionSpec {
            budget: ActionBandAdmissionBudgetSpec {
                axis_channel_count: 3,
                dependency_binding_count: 0,
                storage_rows: 2,
                eml_program_count: 1,
                emission_binding_count: 2,
            },
            templates: vec![ActionBandTemplateSpec {
                id: "opposed-full-vendor".into(),
                label: Some("presentation opposed movement".into()),
                axis_channels: vec![
                    channel(fx.d, ActionBandChannelKind::CachedDerived),
                    channel(fx.demand, ActionBandChannelKind::Primitive),
                    channel(fx.net_flux, ActionBandChannelKind::CachedDerived),
                ],
                target: ActionBandTargetSpec::ScalarBound {
                    channel: fx.net_flux.raw_u32(),
                    bound: 0.0,
                    direction: simthing_spec::ScalarBoundDirection::AtLeast,
                },
                velocity: None,
                bands: vec![
                    ActionBandBandSpec {
                        threshold_registration_index: 0,
                        eml_program: Some(75),
                        emission_binding_indices: vec![0, 1],
                    },
                    ActionBandBandSpec {
                        threshold_registration_index: 1,
                        eml_program: Some(75),
                        emission_binding_indices: vec![0, 1],
                    },
                ],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 2,
                requirement_semantics: ActionBandRequirementSemantics::Ordinary,
            }],
        },
        &[ActionBandConservedProgressBindingSpec {
            template_id: "opposed-full-vendor".into(),
            band_index: 0,
            emission_binding_index: 0,
            bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
        }],
        &fx.registry,
        programs,
        thresholds,
    )
    .expect("opposed vendor admission")
    .clone()
}

fn run_opposed_actionband(
    ctx: &GpuContext,
    fx: &Fixture,
    world: &[f32],
    programs: &EmlExpressionRegistry,
    stall: f32,
    contest: f32,
) -> OpposedDemandObservation {
    let n = fx.registry.total_columns;
    let mut seats: Vec<(usize, f32, f32)> = (0..4)
        .map(|slot| {
            let base = slot * n;
            (
                slot,
                world[base + fx.net_flux.raw()],
                world[base + fx.stall.raw()].abs(),
            )
        })
        .collect();
    seats.sort_by(|a, b| {
        b.2.partial_cmp(&a.2)
            .unwrap()
            .then(a.1.abs().partial_cmp(&b.1.abs()).unwrap())
    });
    let forward_slot = seats[0].0 as u32;
    let reverse_slot = seats[1].0 as u32;
    let thresholds = [
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(forward_slot),
            col: fx.net_flux,
            threshold: 0.1,
            direction: ThresholdDirection::Downward,
            event_kind: MOVEMENT_EVENT + 1,
            buffer: EmitOnThresholdBuffer::Values,
        },
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(reverse_slot),
            col: fx.net_flux,
            threshold: 0.1,
            direction: ThresholdDirection::Downward,
            event_kind: MOVEMENT_EVENT + 2,
            buffer: EmitOnThresholdBuffer::Values,
        },
    ];
    let frozen = admit_opposed(fx, programs, &thresholds);
    let rf = rf_plan(fx);
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fx.registry,
        &[fx.desired],
        std::slice::from_ref(&rf),
        &[],
        &ThresholdRegistry::new(),
    );
    let template = frozen.templates()[0].index();
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        programs,
        &[
            ActionBandEmissionBindingGpu::rf_claim(fx.rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                fx.desired.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
        ],
        &[
            ActionBandActiveInstance::new(template, SlotIndex::new(forward_slot), [0.0; 4]),
            ActionBandActiveInstance::new(template, SlotIndex::new(reverse_slot), [0.0; 4]),
        ],
        &native,
    )
    .expect("opposed ActionBand compile");
    let plan = compiled.into_execution_plan();

    let mut previous = world.to_vec();
    previous[forward_slot as usize * n + fx.net_flux.raw()] = 1.0;
    previous[reverse_slot as usize * n + fx.net_flux.raw()] = 1.0;
    let current = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("opposed_vendor_current"),
            contents: bytemuck::cast_slice(world),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        });
    let mut phase5 = AccumulatorOpSession::new_attached(ctx, 4, n as u32, 4);
    phase5.upload_previous_values(ctx, &previous);
    phase5
        .copy_values_prefix_from_buffer(ctx, &current, 0, 0, current.size())
        .unwrap();
    phase5
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&emit_on_threshold_registrations_to_gpu(
                &thresholds,
            ))
            .unwrap(),
        )
        .unwrap();
    phase5.tick(ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(ctx).unwrap();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    for _ in 0..3 {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &fx.registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 2, "both opposed native crossings required");
    let native_for = |slot: u32| {
        deltas
            .iter()
            .find(|delta| delta.slot().raw() == slot)
            .expect("native crossing for opposed slot")
            .post_value()
    };
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    let next = storage(ctx, "opposed_vendor_next", current.size());
    let mut action = match ActionBandGpuExecution::new(ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("opposed rows inactive"),
    };
    let production = action
        .dispatch_with_native_next(ctx, &current, &next, n as u32, &crossings)
        .unwrap();
    assert!(
        production.commitments.is_empty(),
        "stalled opposed legs have no structural movement binding"
    );
    let mut rf_session = AccumulatorOpSession::new(ctx, rf.slot_count, rf.n_dims);
    rf_session
        .copy_values_prefix_from_buffer(ctx, &next, 0, 0, next.size())
        .unwrap();
    rf_session
        .upload_packed_ops(ctx, &PackedAccumulatorUpload::from_ops(&rf.ops).unwrap())
        .unwrap();
    rf_session.tick(ctx, 0).unwrap();
    let result = rf_session.readback_full(ctx).unwrap();
    let leg = |slot: u32| OpposedDemandOperand {
        native_flux: native_for(slot),
        pre_clamp_progress: result[slot as usize * n + fx.desired.raw()],
        post_clamp_progress: result[slot as usize * n + fx.rf_result.raw()],
    };
    OpposedDemandObservation {
        forward: leg(forward_slot),
        reverse: leg(reverse_slot),
        guyang_stall_magnitude: stall,
        guyang_contest_magnitude: contest,
    }
}

#[test]
fn full_vendor_capacity_overlay_costband_and_arrival_chain_is_native_bounded() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.5c requires a real GPU adapter; skips forbidden");
    let fx = fixture();
    let programs = eml(fx.flux, false);
    let vendor = compile_vendor(
        &fx,
        &programs,
        "full-field-triad-vendor",
        Some("presentation movement"),
    );
    let target_identity = ((fx.d.raw_u32() as u64) << 32) | 2.0f32.to_bits() as u64;
    let descent = local_descent_identity(fx.d.raw_u32(), fx.d.raw_u32(), ACTOR_SLOT, STEP_SLOT);
    let mut runs = Vec::new();
    for (generation, capacity) in [(75, 1.0f32), (76, 0.25), (77, 1.0)] {
        let run = run_vendor(&ctx, &fx, &vendor, generation, capacity, 1.0, 1.0);
        assert_eq!(run.generation, generation);
        assert_eq!(run.desired.to_bits(), (2.0 * run.native_flux).to_bits());
        assert_eq!(run.cost_progress.to_bits(), run.desired.to_bits());
        assert_eq!(run.physical_progress.to_bits(), run.native_flux.to_bits());
        assert!(run.consequence_door_reparented);
        println!(
            "ACTIONBAND_7_8_MOVEMENT_PARITY generation={} digest={:016x}",
            generation, run.consequence_parity_digest
        );
        runs.push((generation, capacity, run));
    }
    let samples: Vec<_> = runs
        .iter()
        .map(|(_generation, capacity, run)| {
            VendorGenerationSample::new(
                run.generation,
                *capacity,
                1.0,
                target_identity,
                descent,
                run.plan_fingerprint,
                run.native_flux,
                run.physical_progress,
            )
        })
        .collect();
    assert_capacity_and_next_generation_law(&samples).expect("capacity/restore law");

    // At fixed Gu-Yang capacity, change one ordinary condition only in N+1.
    // The same admitted plan and local PALMA descent dispatch at each bound
    // generation; the generic native field/throughput result changes, then
    // restores bit-exactly at N+2. No route, search, convergence or history
    // object participates in this next-generation witness.
    let condition_runs =
        [(90, 1.0f32), (91, 0.5f32), (92, 1.0f32)].map(|(generation, condition)| {
            (
                condition,
                run_vendor(&ctx, &fx, &vendor, generation, 1.0, condition, 1.0),
            )
        });
    for ((expected_generation, _), (_, run)) in [(90, 1.0f32), (91, 0.5), (92, 1.0)]
        .iter()
        .zip(condition_runs.iter())
    {
        assert_eq!(run.generation, *expected_generation);
        assert_eq!(run.plan_fingerprint, runs[0].2.plan_fingerprint);
        assert_eq!(run.palma_d, runs[0].2.palma_d);
        assert_eq!(run.physical_progress.to_bits(), run.native_flux.to_bits());
    }
    assert_ne!(
        condition_runs[1].1.physical_progress.to_bits(),
        condition_runs[0].1.physical_progress.to_bits(),
        "N+1 ordinary condition must reach the Gu-Yang field/throughput result"
    );
    assert_eq!(
        condition_runs[2].1.physical_progress.to_bits(),
        condition_runs[0].1.physical_progress.to_bits(),
        "restoring the ordinary condition at N+2 must restore throughput exactly"
    );

    let native = runs[0].2.native_flux;
    let cost_runs = [native.abs() / 2.0, native.abs()]
        .map(|unit_price| run_vendor(&ctx, &fx, &vendor, 78, 1.0, 1.0, unit_price));
    for run in &cost_runs {
        assert_eq!(run.saturation.to_bits(), 1.0f32.to_bits());
        assert_eq!(run.native_flux.to_bits(), native.to_bits());
        assert_eq!(run.physical_progress.to_bits(), run.native_flux.to_bits());
        assert_eq!(run.cost_progress.to_bits(), run.desired.to_bits());
        assert_eq!(run.plan_fingerprint, runs[0].2.plan_fingerprint);
        assert_eq!(run.palma_d, runs[0].2.palma_d);
    }
    let cost_rows = [&cost_runs[0], &cost_runs[1]].map(|run| {
        CostBandOrderingObservation::from_run(
            run.native_flux,
            run.physical_progress,
            run.unit_price,
            run.cost_draw,
        )
    });
    println!(
        "R2_GPU_TRACE price0={} draw0=({}, {}) native0={} physical0={} price1={} draw1=({}, {}) native1={} physical1={} plan={} capacity=1",
        cost_runs[0].unit_price,
        cost_runs[0].cost_draw.n,
        cost_runs[0].cost_draw.r,
        cost_runs[0].native_flux,
        cost_runs[0].physical_progress,
        cost_runs[1].unit_price,
        cost_runs[1].cost_draw.n,
        cost_runs[1].cost_draw.r,
        cost_runs[1].native_flux,
        cost_runs[1].physical_progress,
        cost_runs[0].plan_fingerprint,
    );
    assert_costband_is_downstream(&cost_rows).expect("sink price is downstream");
    assert_costband_scaling_mutant_reds(cost_rows[0])
        .expect("constructible CostBand-scaled movement mutant must RED");

    let (mut tree, mut allocator, mut cells, actor, ids) = topology();
    for cell in &mut cells {
        cell.sealed_col = fx.flux.raw_u32();
    }
    let step = SpatialVendorizationStep::admit(
        runs[0].2.commitment,
        actor,
        ids[ACTOR_SLOT as usize],
        &cells,
        SpatialStepOverlayEffect {
            property_id: fx.registry.id_of("movement-vendor", "rf-result").unwrap(),
            deltas: vec![],
        },
        false,
        1.0,
        None,
    )
    .expect("ordinary adjacent structural consequence");
    assert_eq!(step.deciding_cell(), ids[STEP_SLOT as usize]);
    let n_dims = fx.registry.total_columns;
    let mut registry = fx.registry;
    let mut shadow = vec![0.0; allocator.capacity() * n_dims];
    let outcome = apply_structural_mutations(
        vec![
            simthing_feeder::BoundaryRequest::Reparent {
                child: actor,
                new_parent: step.deciding_cell(),
            },
            simthing_feeder::BoundaryRequest::AttachOverlay {
                target: actor,
                overlay: step.overlay().clone(),
                source_generation: simthing_core::GenerationStamp::new(0),
            },
        ],
        &mut tree,
        &mut allocator,
        &mut registry,
        &mut shadow,
        n_dims,
        None,
        simthing_core::GenerationStamp::new(0),
        &mut simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState::default(),
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(outcome.reparented, vec![(actor, ids[STEP_SLOT as usize])]);
    assert!(tree.has_overlay(actor, step.overlay_id()));
    // Exercise the ordinary lifecycle operator directly on an admitted raw
    // tree. Structural attachment above and lifecycle dissolution here are
    // separate existing boundary stages; neither is movement-owned.
    let mut lifecycle_actor = SimThing::new(SimThingKind::Cohort, 0);
    lifecycle_actor.add_overlay(step.overlay().clone());
    let lifecycle_actor_id = lifecycle_actor.id;
    let mut lifecycle_root = SimThing::new(SimThingKind::World, 0);
    lifecycle_root.add_child(lifecycle_actor);
    let mut lifecycle_allocator = SlotAllocator::new();
    lifecycle_allocator.install_initial_tree(&lifecycle_root);
    let mut lifecycle_shadow = vec![0.0; lifecycle_allocator.capacity() * n_dims];
    let first = resolve_overlay_lifecycle(
        &mut lifecycle_root,
        &registry,
        &lifecycle_allocator,
        &mut lifecycle_shadow,
        n_dims,
        76,
        None,
    );
    assert_eq!(first.after_ticks_decremented, 1);
    let second = resolve_overlay_lifecycle(
        &mut lifecycle_root,
        &registry,
        &lifecycle_allocator,
        &mut lifecycle_shadow,
        n_dims,
        77,
        None,
    );
    assert_eq!(second.dissolved, 1);
    assert_arrival_collapses(ArrivalLifecycleObservation {
        target_satisfied: runs[0].2.palma_d[STEP_SLOT as usize] <= 2.0,
        actor_at_target: allocator.relation_of(actor)
            == Some(simthing_core::ObjectResidencyRelation::ChildOf(
                ids[STEP_SLOT as usize],
            )),
        actuation_overlay_present: second.dissolved_overlays.iter().all(|(target, overlay)| {
            *target != lifecycle_actor_id || *overlay != step.overlay_id()
        }),
        transient_overlay_present: false,
        retained_executor_records: 0,
    })
    .expect("ordinary arrival collapses actuation");
    assert!(assert_arrival_collapses(ArrivalLifecycleObservation {
        target_satisfied: true,
        actor_at_target: true,
        actuation_overlay_present: false,
        transient_overlay_present: false,
        retained_executor_records: 1,
    })
    .is_err());
}

#[test]
fn semantic_rename_delete_is_blind_at_shadow_to_positional_template_boundary() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.5c requires a real GPU adapter; skips forbidden");
    let fx = fixture();
    let programs = eml(fx.flux, false);
    let canonical = compile_vendor(
        &fx,
        &programs,
        "full-field-triad-vendor",
        Some("advance to beacon"),
    );
    let renamed = compile_vendor(
        &fx,
        &programs,
        "renamed-full-field-triad-vendor",
        Some("renamed presentation only"),
    );
    let deleted = compile_vendor(&fx, &programs, "replacement-opaque-template-id", None);

    let canonical_order: Vec<_> = canonical
        .frozen
        .semantic_shadow()
        .iter()
        .map(|row| row.template().raw())
        .collect();
    assert_eq!(canonical_order, vec![0]);
    assert_ne!(
        renamed.frozen.semantic_shadow()[0].authored_id(),
        canonical.frozen.semantic_shadow()[0].authored_id(),
        "the A1 mutation must actually rename authored identity"
    );
    assert_ne!(
        deleted.frozen.semantic_shadow()[0].authored_id(),
        canonical.frozen.semantic_shadow()[0].authored_id(),
        "replacement identity must reach the semantic shadow"
    );
    let renamed_order: Vec<_> = renamed
        .frozen
        .semantic_shadow()
        .iter()
        .map(|row| row.template().raw())
        .collect();
    let deleted_order: Vec<_> = deleted
        .frozen
        .semantic_shadow()
        .iter()
        .map(|row| row.template().raw())
        .collect();
    assert_eq!(
        renamed_order, canonical_order,
        "authored order must be stable"
    );
    assert_eq!(
        deleted_order, canonical_order,
        "authored order must be stable"
    );
    assert_ne!(
        canonical.frozen.semantic_shadow()[0].label(),
        renamed.frozen.semantic_shadow()[0].label()
    );
    assert_eq!(deleted.frozen.semantic_shadow()[0].label(), None);

    let run = run_vendor(&ctx, &fx, &canonical, 75, 1.0, 1.0, 1.0);
    let renamed_run = run_vendor(&ctx, &fx, &renamed, 75, 1.0, 1.0, 1.0);
    let deleted_run = run_vendor(&ctx, &fx, &deleted, 75, 1.0, 1.0, 1.0);
    let binding = canonical.compiled.conserved_progress_bindings()[0];
    let source_code = native_bound_source_code(binding.bound_source());
    let target_identity = ((fx.d.raw_u32() as u64) << 32) | 2.0f32.to_bits() as u64;
    let descent = local_descent_identity(fx.d.raw_u32(), fx.d.raw_u32(), ACTOR_SLOT, STEP_SLOT);
    let (_tree, _allocator, _cells, actor, ids) = topology();
    let consequence_for = |variant_run: &VendorRun| {
        consequence_fingerprint(actor, ids[variant_run.commitment.slot() as usize])
    };
    let consequence = consequence_for(&run);
    let sealed_invariant = |variant_run: &VendorRun| {
        (
            variant_run.generation,
            variant_run.native_flux.to_bits(),
            variant_run.commitment.slot(),
            variant_run.commitment.col(),
            variant_run.commitment.event_kind(),
            variant_run.commitment.value().to_bits(),
        )
    };
    assert_eq!(sealed_invariant(&renamed_run), sealed_invariant(&run));
    assert_eq!(sealed_invariant(&deleted_run), sealed_invariant(&run));
    let authority = SealedVendorAuthority {
        template_index: canonical.frozen.templates()[0].index().raw(),
        plan_fingerprint: run.plan_fingerprint,
        frozen_binding: numeric_binding_fingerprint(&canonical),
        generation: run.generation,
        sealed_slot: run.commitment.slot(),
        sealed_column: run.commitment.col(),
        sealed_event_kind: run.commitment.event_kind(),
        sealed_value_bits: run.commitment.value().to_bits(),
        target_identity,
        descent_identity: descent,
        bound_source_code: source_code,
        native_flux_bits: run.native_flux.to_bits(),
        stall_bits: 0.0f32.to_bits(),
        structural_consequence_fingerprint: consequence,
    };
    let projection =
        |vendor: &CompiledVendor, variant_run: &VendorRun| SemanticMutationProjection {
            authored_order: vendor
                .frozen
                .semantic_shadow()
                .iter()
                .map(|row| row.template().raw())
                .collect(),
            plan_fingerprint: vendor.compiled.plan_fingerprint(),
            frozen_binding: numeric_binding_fingerprint(vendor),
            template_index: vendor.frozen.templates()[0].index().raw(),
            target_identity,
            descent_identity: descent,
            bound_source_code: native_bound_source_code(
                vendor.compiled.conserved_progress_bindings()[0].bound_source(),
            ),
            structural_consequence_fingerprint: consequence_for(variant_run),
        };
    assert_semantic_mutations_are_authority_blind(
        &authority,
        &canonical_order,
        &[
            projection(&renamed, &renamed_run),
            projection(&deleted, &deleted_run),
        ],
    )
    .expect("one sealed result remains authoritative under in-place label mutation");
    assert_eq!(
        canonical.compiled.plan_fingerprint(),
        renamed.compiled.plan_fingerprint()
    );
    assert_eq!(
        canonical.compiled.plan_fingerprint(),
        deleted.compiled.plan_fingerprint()
    );
}

#[test]
fn costband_selector_is_unrepresentable_and_workshop_is_structurally_reapable() {
    let fx = fixture();
    let programs = eml(fx.flux, false);
    let vendor = compile_vendor(
        &fx,
        &programs,
        "full-field-triad-vendor",
        Some("semantic only"),
    );
    let source = vendor.compiled.conserved_progress_bindings()[0].bound_source();
    assert_eq!(native_bound_source_code(source), 3);
    assert_eq!(ActionBandEmissionBindingGpu::CONSERVED_BOUND_NONE, 0);
    assert_eq!(ActionBandEmissionBindingGpu::CONSERVED_BOUND_RF_GRANT, 1);
    assert_eq!(
        ActionBandEmissionBindingGpu::CONSERVED_BOUND_GU_YANG_AVAILABLE,
        2
    );
    assert_eq!(
        ActionBandEmissionBindingGpu::CONSERVED_BOUND_GU_YANG_REALIZED,
        3
    );

    // CostBand is separately constructible only as an emission destination.
    let cargos = [
        include_str!("../../simthing-core/Cargo.toml"),
        include_str!("../../simthing-kernel/Cargo.toml"),
        include_str!("../../simthing-gpu/Cargo.toml"),
        include_str!("../../simthing-sim/Cargo.toml"),
        include_str!("../../simthing-driver/Cargo.toml"),
        include_str!("../../simthing-spec/Cargo.toml"),
    ];
    for body in cargos {
        assert!(!body.lines().any(|line| {
            let line = line.trim();
            !line.starts_with('#') && line.contains("simthing-workshop")
        }));
    }
    let mut ordinary_door = ActionBandSessionBuildDoor::new();
    let ordinary = ordinary_door
        .admit_once_at_session_build(
            &session_spec(&fx, "closed-bound-source-vocabulary", None),
            &fx.registry,
            &programs,
            std::slice::from_ref(&fx.movement_threshold),
        )
        .expect("ordinary admission for closed wire-code validation");
    let rf = rf_plan(&fx);
    let mut cost_registry = ThresholdRegistry::new();
    let cost_event = cost_registry.push_with_cost_band(
        ThresholdSemantic::ScriptedEventTrigger {
            event_id: "closed-bound-source-cost".into(),
        },
        CostBandSemantic::admit_sink(None, None).expect("sink admission"),
    );
    let cost_threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(STEP_SLOT),
        col: fx.cost_progress,
        threshold: 0.25,
        direction: ThresholdDirection::Upward,
        event_kind: cost_event,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &fx.registry,
        &[fx.desired],
        std::slice::from_ref(&rf),
        std::slice::from_ref(&cost_threshold),
        &cost_registry,
    );
    let invalid_fifth = [
        ActionBandEmissionBindingGpu::rf_claim(fx.rf_claim.raw_u32())
            .with_conserved_progress_bound_source(4),
        ActionBandEmissionBindingGpu::property_next(
            fx.desired.raw_u32(),
            simthing_gpu::ActionBandPropertyWrite::Set,
        ),
        ActionBandEmissionBindingGpu::cost_band(fx.cost_progress.raw_u32()),
        ActionBandEmissionBindingGpu::structural_request(0),
    ];
    let rejected = compile_action_band_gpu_execution_with_native_lanes(
        ordinary,
        &programs,
        &invalid_fifth,
        &[ActionBandActiveInstance::new(
            ordinary.templates()[0].index(),
            SlotIndex::new(STEP_SLOT),
            [0.0; 4],
        )],
        &native,
    );
    assert!(matches!(
        rejected,
        Err(ActionBandExecutionCompileError::Kernel(
            simthing_gpu::ActionBandExecutionError::InvalidTableSpan
        ))
    ));
    println!("R3_CLOSED_SET_TRACE constants=[0,1,2,3] rejected_code=4 error=InvalidTableSpan");
}

#[test]
fn opposed_demand_requires_native_contest_and_abs_gross_mutant_reds() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let ctx =
        GpuContext::new_blocking().expect("7.5c requires a real GPU adapter; skips forbidden");
    let fx = fixture();
    let n_dims = fx.registry.total_columns as u32;
    let adjacency = FieldAdjacency::grid_n4(2, 2, GRID_N4_NSEW, fx.flux).unwrap();
    let neighbor_slots = neighbor_slots_from_grid(&adjacency).unwrap();
    let bundle = compile_comparative_bundle(ComparativeProjectionRequest {
        adjacency,
        neighbor_slots,
        n_dims,
        emitters: vec![
            ComparativeEmitterClass {
                authored_order: 0,
                class_id: 1.0,
                value_col: fx.emitter_a,
            },
            ComparativeEmitterClass {
                authored_order: 1,
                class_id: 2.0,
                value_col: fx.emitter_b,
            },
        ],
        outputs: ComparativeProjectionOutputs {
            dominance_col: fx.dominance,
            margin_col: fx.margin,
            contest_col: fx.contest,
        },
        band_readouts: ComparativeBandReadouts {
            border_col: fx.border,
            chokepoint_col: fx.chokepoint,
        },
        palma_d_col: fx.d,
        guyang_value_col: fx.flux,
        guyang_conductance_col: fx.conductance,
        stall_outputs: GuYangStallOutputs {
            net_flux_col: fx.net_flux,
            gross_flux_col: fx.gross_flux,
            stall_col: fx.stall,
        },
        bands: Default::default(),
        authored_opt_out_reason: None,
    })
    .expect("native stall/contest bundle");
    let mut world = initial_values(&fx, 1.0);
    let n = fx.registry.total_columns;
    for (slot, value) in [0.0f32, 1.0, -1.0, 0.0].into_iter().enumerate() {
        let base = slot * n;
        world[base + fx.flux.raw()] = value;
        world[base + fx.d.raw()] = 1.0;
        world[base + fx.conductance.raw()] = 0.5;
        world[base + fx.demand.raw()] = if slot % 2 == 0 { 1.0 } else { -1.0 };
    }
    let mut field = FieldSweepSession::new(&ctx, &bundle.registrations[0]).unwrap();
    field.upload_values(&ctx, &world).unwrap();
    field
        .dispatch_chain(&ctx, &bundle.registrations, 1)
        .unwrap();
    let out = {
        let _proof = scoped_debug_readback_allowed(true);
        field.readback(&ctx).unwrap()
    };
    let stall = (0..4)
        .map(|slot| out[slot * n + fx.stall.raw()].abs())
        .fold(0.0, f32::max);
    let contest = (0..4)
        .map(|slot| out[slot * n + fx.contest.raw()].abs())
        .fold(0.0, f32::max);
    assert!(
        stall > 1e-4 || contest > 1e-4,
        "contest versus absence must be positive"
    );
    let lawful_program = eml(fx.demand, false);
    let lawful = run_opposed_actionband(&ctx, &fx, &out, &lawful_program, stall, contest);
    assert_opposed_demand_law(lawful).expect("native contest gates real ActionBand mutual stall");
    assert!(lawful.forward.pre_clamp_progress.abs() > 1e-4);
    assert!(lawful.reverse.pre_clamp_progress.abs() > 1e-4);
    assert_ne!(
        lawful.forward.pre_clamp_progress.signum(),
        lawful.reverse.pre_clamp_progress.signum(),
        "lawful GPU operands must preserve authored demand opposition"
    );

    let abs_program = eml(fx.demand, true);
    let lawful_nodes = lawful_program
        .get_nodes(EmlTreeId(75))
        .expect("lawful demand program");
    let abs_nodes = abs_program
        .get_nodes(EmlTreeId(75))
        .expect("ABS demand mutant program");
    assert_eq!(
        lawful_nodes[0].opcode,
        simthing_core::eml_nodes::opcode::SLOT_VALUE
    );
    assert_eq!(
        abs_nodes[0].opcode,
        simthing_core::eml_nodes::opcode::SLOT_VALUE
    );
    assert_eq!(lawful_nodes[0].a, fx.demand.raw_u32());
    assert_eq!(abs_nodes[0].a, lawful_nodes[0].a);
    assert!(
        abs_nodes
            .iter()
            .any(|node| node.opcode == simthing_core::eml_nodes::opcode::ABS),
        "the exact same-source program passed into the mutant GPU dispatch must contain ABS"
    );
    let abs_gross_mutant = run_opposed_actionband(&ctx, &fx, &out, &abs_program, stall, contest);
    assert!(abs_gross_mutant.forward.pre_clamp_progress.abs() > 1e-4);
    assert!(abs_gross_mutant.reverse.pre_clamp_progress.abs() > 1e-4);
    assert_eq!(
        abs_gross_mutant.forward.pre_clamp_progress.signum(),
        abs_gross_mutant.reverse.pre_clamp_progress.signum(),
        "ABS mutant must express sign loss through non-zero same-signed GPU operands"
    );
    assert_eq!(
        abs_gross_mutant.forward.pre_clamp_progress.to_bits(),
        2.0f32.to_bits()
    );
    assert_eq!(
        abs_gross_mutant.reverse.pre_clamp_progress.to_bits(),
        2.0f32.to_bits()
    );
    let mutant_error = assert_opposed_demand_law(abs_gross_mutant)
        .expect_err("GPU-produced ABS/gross mutant must RED the opposed-demand law");
    assert_eq!(mutant_error, FluxWitnessError::PreClampSignLost);
    println!(
        "R1_GPU_TRACE source_col={} lawful={lawful:?} abs_program_dispatched=true mutant={abs_gross_mutant:?} rejection={mutant_error:?}",
        fx.demand.raw_u32()
    );
}
