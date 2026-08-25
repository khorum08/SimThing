//! ACTIONBAND-SPATIAL-VENDORIZATION-0 focused pure-consumer proof (remand R1–R5).
//!
//! Born-mortal workshop witness: spatial progress is ActionBand target + PALMA D
//! field + sealed Phase-5 crossing + ordinary structural/native consequence.
//! Production crates are READ surfaces only. No peer movement facility.
//!
//! R1: admit consumes typed `StructuralCommitment` only.
//! R2: field/overlay-only redirect through real ActionBand execution.
//! R3: no non-ActionBand mint fallback on ActionBand exit-proofs.
//! R4: sealed slot is opaque mapping key, not row-major structural formula.
//! R5: matrix-shaped mutants are table-driven; named tests hold distinct obligations.

use std::sync::Mutex;

use simthing_core::owner_channel::{
    bind_owner, resolve_owner, OwnerRef, OWNER_CHANNEL_PROPERTY_ID,
};
use simthing_core::{
    cost_band_quantize, CostBandDraw, DimensionRegistry, DissolveCondition, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, ObjectResidencyRelation, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing,
    SimThingId, SimThingKind, SlotIndex, SubFieldRole, ThresholdDirection, TransformOp,
};
use simthing_driver::{
    compile_action_band_gpu_execution, ActionBandActiveInstance, FrozenActionBandStructuralRequests,
};
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, cpu_min_plus_d_from_w,
    emit_on_threshold_registrations_to_gpu, pack_w_and_initial_d, scoped_debug_readback_allowed,
    wgpu, AccumulatorOpSession, ActionBandEmissionBindingGpu, ActionBandGpuExecution, GpuContext,
    MinPlusStencilConfig, PackedThresholdUpload, SlotAllocator, MIN_PLUS_INF,
};
use simthing_kernel::StructuralCommitment;
use simthing_sim::{apply_structural_mutations, SimRuntimeTree};
use simthing_spec::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, FrozenActionBandTemplates,
};
use simthing_workshop::actionband_spatial_vendorization_0::{
    manhattan, reject_non_adjacent, resolve_authoritative_cell, validate_spatial_overlay,
    AdmittedTopologyCell, SpatialStepOverlayEffect, SpatialVendorizationError,
    SpatialVendorizationStep,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

/// Default opaque sealed slot keys — deliberately NOT equal to row-major indices.
const SLOT_A: u32 = 10;
const SLOT_B: u32 = 20;
const SLOT_C: u32 = 30;
/// R4 alternate assignment: same logical A/B/C, different physical sealed keys.
const SLOT_A_ALT: u32 = 77;
const SLOT_B_ALT: u32 = 10;
const SLOT_C_ALT: u32 = 55;
const SPATIAL_EVENT_KIND_B: u32 = 0x5350_4154; // "SPAT"
const SPATIAL_EVENT_KIND_C: u32 = 0x5350_4155;

/// Opaque sealed-slot assignment for the three logical cells.
#[derive(Clone, Copy, Debug)]
struct SlotAssignment {
    a: u32,
    b: u32,
    c: u32,
}

const ASSIGNMENT_PRIMARY: SlotAssignment = SlotAssignment {
    a: SLOT_A,
    b: SLOT_B,
    c: SLOT_C,
};
const ASSIGNMENT_PERMUTED: SlotAssignment = SlotAssignment {
    a: SLOT_A_ALT,
    b: SLOT_B_ALT,
    c: SLOT_C_ALT,
};

struct Arena {
    tree: SimRuntimeTree,
    allocator: SlotAllocator,
    registry: DimensionRegistry,
    actor: SimThingId,
    cargo: SimThingId,
    a: SimThingId,
    b: SimThingId,
    c: SimThingId,
    property: SimPropertyId,
}

/// Logical cells + structural coords with opaque sealed slot keys.
/// `sealed_col` must match the ActionBand commitment value-plane column.
fn topology_cells(
    a: SimThingId,
    b: SimThingId,
    c: SimThingId,
    sealed_col: u32,
    slots: SlotAssignment,
) -> Vec<AdmittedTopologyCell> {
    // Structural N4: A(0,0)—B(0,1)
    //                  |
    //                 C(1,0)
    // Sealed slots are opaque mapping keys, not row-major.
    let cells = vec![
        AdmittedTopologyCell {
            sealed_slot: slots.a,
            sealed_col,
            grid_row: 0,
            grid_col: 0,
            cell: a,
        },
        AdmittedTopologyCell {
            sealed_slot: slots.b,
            sealed_col,
            grid_row: 0,
            grid_col: 1,
            cell: b,
        },
        AdmittedTopologyCell {
            sealed_slot: slots.c,
            sealed_col,
            grid_row: 1,
            grid_col: 0,
            cell: c,
        },
    ];
    for cell in &cells {
        assert_ne!(
            cell.sealed_slot,
            cell.grid_row * 2 + cell.grid_col,
            "R4: sealed slot must not equal row-major structural index"
        );
    }
    cells
}

fn arena() -> Arena {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("spatial-witness", "progress", 0));

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut a = SimThing::new(SimThingKind::Location, 0);
    let mut b = SimThing::new(SimThingKind::Location, 0);
    let mut c = SimThing::new(SimThingKind::Location, 0);
    bind_owner(&mut a, &OwnerRef::new("alpha"));
    bind_owner(&mut b, &OwnerRef::new("beta"));
    bind_owner(&mut c, &OwnerRef::new("gamma"));
    let (a_id, b_id, c_id) = (a.id, b.id, c.id);

    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    actor.add_property(property, registry.property(property).default_value());
    let actor_id = actor.id;
    let cargo = SimThing::new(SimThingKind::Cohort, 0);
    let cargo_id = cargo.id;
    actor.add_child(cargo);
    a.add_child(actor);
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);

    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);

    Arena {
        tree: SimRuntimeTree::admit(root),
        allocator,
        registry,
        actor: actor_id,
        cargo: cargo_id,
        a: a_id,
        b: b_id,
        c: c_id,
        property,
    }
}

fn effect(property: SimPropertyId, consuming: bool) -> SpatialStepOverlayEffect {
    SpatialStepOverlayEffect {
        property_id: property,
        deltas: consuming
            .then(|| vec![(SubFieldRole::Amount, TransformOp::add(0.25))])
            .unwrap_or_default(),
    }
}

fn require_gpu() -> Option<GpuContext> {
    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    use simthing_gpu::accumulator_op::set_debug_readback_allowed;
    set_debug_readback_allowed(true);
    GpuContext::new_blocking().ok()
}

/// Shared ActionBand admission product: one LocusRadius template, structural emission.
/// Threshold registrations always cover **both** adjacent candidate loci B and C.
fn admit_spatial_actionband(
    slots: SlotAssignment,
) -> (
    FrozenActionBandTemplates,
    Vec<EmitOnThresholdRegistration>,
    DimensionRegistry,
) {
    let mut registry = DimensionRegistry::new();
    let pot = registry.register(SimProperty::simple("spatial-witness", "field-potential", 0));
    let d_prop = registry.register(SimProperty::simple("spatial-witness", "palma-d", 0));
    let d_col = registry
        .column_range(d_prop)
        .col_for_role(&SubFieldRole::Amount, &registry.property(d_prop).layout)
        .unwrap()
        .raw_u32();

    let thresholds = vec![
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(slots.b),
            col: registry
                .column_range(pot)
                .col_for_role(&SubFieldRole::Amount, &registry.property(pot).layout)
                .unwrap(),
            threshold: 1.0,
            direction: ThresholdDirection::Upward,
            event_kind: SPATIAL_EVENT_KIND_B,
            buffer: EmitOnThresholdBuffer::Values,
        },
        EmitOnThresholdRegistration {
            slot: SlotIndex::new(slots.c),
            col: registry
                .column_range(pot)
                .col_for_role(&SubFieldRole::Amount, &registry.property(pot).layout)
                .unwrap(),
            threshold: 1.0,
            direction: ThresholdDirection::Upward,
            event_kind: SPATIAL_EVENT_KIND_C,
            buffer: EmitOnThresholdBuffer::Values,
        },
    ];

    let pot_col_u32 = thresholds[0].col.raw_u32();
    let d_col_u32 = d_col;
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(
            &ActionBandSessionSpec {
                budget: ActionBandAdmissionBudgetSpec {
                    axis_channel_count: 2,
                    dependency_binding_count: 0,
                    storage_rows: 2,
                    eml_program_count: 0,
                    emission_binding_count: 1,
                },
                templates: vec![ActionBandTemplateSpec {
                    id: "spatial-locus-radius".into(),
                    label: Some("presentation-only-spatial-shadow".into()),
                    axis_channels: vec![
                        ActionBandChannelBindingSpec {
                            column: pot_col_u32,
                            kind: ActionBandChannelKind::Primitive,
                        },
                        ActionBandChannelBindingSpec {
                            column: d_col_u32,
                            kind: ActionBandChannelKind::CachedDerived,
                        },
                    ],
                    // LocusRadius consumes D as field, never as path object.
                    target: ActionBandTargetSpec::LocusRadius {
                        distance_channel: d_col_u32,
                        radius: 8.0,
                    },
                    velocity: None,
                    // One band per candidate locus registration (B then C).
                    bands: vec![
                        ActionBandBandSpec {
                            threshold_registration_index: 0,
                            eml_program: None,
                            emission_binding_indices: vec![0],
                        },
                        ActionBandBandSpec {
                            threshold_registration_index: 1,
                            eml_program: None,
                            emission_binding_indices: vec![0],
                        },
                    ],
                    subordinate_template_ids: vec![],
                    max_active_subordinates: 0,
                    reserved_instance_rows: 2,
                    requirement_semantics: Default::default(),
                }],
            },
            &registry,
            &simthing_core::EmlExpressionRegistry::new(),
            &thresholds,
        )
        .expect("ActionBand spatial LocusRadius admission")
        .clone();
    (frozen, thresholds, registry)
}

/// Both candidate loci always active. Only field-plane values differ between runs.
/// R2: caller does **not** choose the winning slot; field values do.
/// R3: missing crossing/commitment is RED (no non-ActionBand mint fallback).
fn sealed_commitment_from_field_state(
    ctx: &GpuContext,
    frozen: &FrozenActionBandTemplates,
    thresholds: &[EmitOnThresholdRegistration],
    registry: &DimensionRegistry,
    slots: SlotAssignment,
    // Potential previous/current at B and C field loci (the sole run-varying authority).
    pot_b_prev: f32,
    pot_b_curr: f32,
    pot_c_prev: f32,
    pot_c_curr: f32,
    distance_value: f32,
) -> StructuralCommitment {
    let pot_col = thresholds[0].col.raw_u32();
    let d_col = registry
        .column_range(
            registry
                .id_of("spatial-witness", "palma-d")
                .expect("palma-d property"),
        )
        .col_for_role(
            &SubFieldRole::Amount,
            &registry
                .property(registry.id_of("spatial-witness", "palma-d").unwrap())
                .layout,
        )
        .unwrap()
        .raw_u32();

    let template = frozen.templates()[0].index();
    // R2: identical active-instance set on every run — both candidates present.
    let active = [
        ActionBandActiveInstance::new(template, SlotIndex::new(slots.b), [0.0; 4]),
        ActionBandActiveInstance::new(template, SlotIndex::new(slots.c), [0.0; 4]),
    ];
    let compiled = compile_action_band_gpu_execution(
        frozen,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .expect("ActionBand GPU lowering");
    let plan = compiled.execution_plan().clone();

    let n_dims = registry.total_columns as u32;
    let n_slots = slots.a.max(slots.b).max(slots.c) + 1;
    let mut previous = vec![0.0f32; (n_slots * n_dims) as usize];
    let mut current = previous.clone();
    let write = |buf: &mut [f32], slot: u32, col: u32, value: f32| {
        buf[(slot * n_dims + col) as usize] = value;
    };
    write(&mut previous, slots.b, pot_col, pot_b_prev);
    write(&mut current, slots.b, pot_col, pot_b_curr);
    write(&mut previous, slots.c, pot_col, pot_c_prev);
    write(&mut current, slots.c, pot_col, pot_c_curr);
    // PALMA D field plane: within LocusRadius for both candidates (field, not path).
    write(&mut previous, slots.b, d_col, distance_value);
    write(&mut current, slots.b, d_col, distance_value);
    write(&mut previous, slots.c, d_col, distance_value);
    write(&mut current, slots.c, d_col, distance_value);

    let regs = emit_on_threshold_registrations_to_gpu(thresholds);
    let mut session = AccumulatorOpSession::new_attached(ctx, n_slots, n_dims, 8);
    session.bind_generation_authority(7);
    session.upload_values(ctx, &current);
    session.upload_previous_values(ctx, &previous);
    session
        .upload_packed_threshold_ops(
            ctx,
            &PackedThresholdUpload::from_registrations(&regs).expect("threshold pack"),
        )
        .expect("threshold upload");
    session.tick(ctx, 0).expect("sealed Phase-5 threshold scan");
    let emissions = session
        .readback_threshold_emissions(ctx)
        .expect("sealed emissions");

    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    for _ in 0..n_slots {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);

    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
        registry,
        &allocator,
    );
    assert!(
        !deltas.is_empty(),
        "R3: missing ActionBand Phase-5 BandCrossingDelta is RED, not a fallback path"
    );
    let crossings = plan
        .crossings_from_sealed(&deltas)
        .expect("ActionBand joins only sealed Phase-5 evidence");

    let world = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("spatial_vendorization_world"),
            contents: bytemuck::cast_slice(&current),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let mut execution = match ActionBandGpuExecution::new(ctx, plan).expect("GPU operator") {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("spatial ActionBand rows must be active"),
    };
    let _scope = scoped_debug_readback_allowed(true);
    let production = execution
        .dispatch(ctx, &world, n_dims, &crossings)
        .expect("ActionBand structural emission");
    assert_eq!(
        production.commitments.len(),
        1,
        "R2/R3: exactly one sealed commitment when exactly one candidate field crosses; got {}",
        production.commitments.len()
    );
    let commitment = production.commitments[0];
    assert_eq!(commitment.col(), pot_col);
    assert!(
        commitment.event_kind() == SPATIAL_EVENT_KIND_B
            || commitment.event_kind() == SPATIAL_EVENT_KIND_C
    );
    commitment
}

fn apply_spatial_step(
    step: &SpatialVendorizationStep,
    tree: &mut SimRuntimeTree,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    shadow: &mut [f32],
    n_dims: usize,
) {
    step.validate_overlay().expect("overlay still lawful");
    step.validate_cost_band().expect("cost band still lawful");
    let outcome = apply_structural_mutations(
        vec![
            BoundaryRequest::Reparent {
                child: step.actor(),
                new_parent: step.deciding_cell(),
            },
            BoundaryRequest::AttachOverlay {
                target: step.actor(),
                overlay: step.overlay().clone(),
                source_generation: simthing_core::GenerationStamp::new(0),
            },
        ],
        tree,
        allocator,
        registry,
        shadow,
        n_dims,
        None,
        simthing_core::GenerationStamp::new(0),
        &mut simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState::default(),
        &std::collections::BTreeMap::new(),
    );
    assert!(
        outcome
            .reparented
            .iter()
            .any(|pair| *pair == (step.actor(), step.deciding_cell())),
        "ordinary Reparent must apply"
    );
    assert!(
        outcome
            .overlays_attached
            .iter()
            .any(|pair| *pair == (step.actor(), step.overlay_id())),
        "ordinary AttachOverlay must apply"
    );
}

// ─── R2 + R3 + structural Reparent: primary through-ActionBand exit proof ───

#[test]
fn field_overlay_only_redirects_same_actionband_to_different_adjacent_step() {
    let Some(ctx) = require_gpu() else {
        eprintln!(
            "SKIP field_overlay_only_redirects: no local GPU (established GPU-proof convention)"
        );
        return;
    };
    let arena = arena();
    let slots = ASSIGNMENT_PRIMARY;
    let (frozen, thresholds, ab_registry) = admit_spatial_actionband(slots);
    let sealed_col = thresholds[0].col.raw_u32();
    let cells = topology_cells(arena.a, arena.b, arena.c, sealed_col, slots);
    let template_index = frozen.templates()[0].index();

    // R2: same frozen product, same thresholds, same dual active-instance set.
    // Run A: only B's field potential crosses; C stays below.
    let commit_b = sealed_commitment_from_field_state(
        &ctx,
        &frozen,
        &thresholds,
        &ab_registry,
        slots,
        0.25,
        1.75, // B crosses
        0.25,
        0.40, // C does not
        2.0,
    );
    let mut cells_b = cells.clone();
    cells_b.reverse(); // append order non-semantic
    let step_b = SpatialVendorizationStep::admit(
        commit_b,
        arena.actor,
        arena.a,
        &cells_b,
        effect(arena.property, true),
        true,
        1.0,
        Some(1),
    )
    .expect("field-selected sealed locus B → one N4 edge");
    assert_eq!(step_b.deciding_cell(), arena.b);
    assert_eq!(step_b.commitment().slot(), slots.b);
    assert_eq!(step_b.commitment().event_kind(), SPATIAL_EVENT_KIND_B);

    // Run B: only C's field potential crosses; B stays below. No active-instance,
    // threshold, template, or destination identity change — field values only.
    let commit_c = sealed_commitment_from_field_state(
        &ctx,
        &frozen,
        &thresholds,
        &ab_registry,
        slots,
        0.25,
        0.40, // B does not
        0.25,
        1.75, // C crosses
        2.0,
    );
    let step_c = SpatialVendorizationStep::admit(
        commit_c,
        arena.actor,
        arena.a,
        &cells,
        effect(arena.property, true),
        true,
        1.0,
        Some(1),
    )
    .expect("field-selected sealed locus C → one N4 edge");
    assert_eq!(step_c.deciding_cell(), arena.c);
    assert_eq!(step_c.commitment().slot(), slots.c);
    assert_eq!(step_c.commitment().event_kind(), SPATIAL_EVENT_KIND_C);

    assert_eq!(frozen.templates()[0].index(), template_index);
    assert_ne!(step_b.deciding_cell(), step_c.deciding_cell());
    assert_ne!(step_b.commitment().slot(), step_c.commitment().slot());
}

#[test]
fn sealed_actionband_locus_reparents_one_n4_edge_with_stable_slots() {
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP sealed_actionband_locus_reparents: no local GPU");
        return;
    };
    let mut arena = arena();
    let slots = ASSIGNMENT_PRIMARY;
    let (frozen, thresholds, ab_registry) = admit_spatial_actionband(slots);
    let cells = topology_cells(
        arena.a,
        arena.b,
        arena.c,
        thresholds[0].col.raw_u32(),
        slots,
    );
    let commitment = sealed_commitment_from_field_state(
        &ctx,
        &frozen,
        &thresholds,
        &ab_registry,
        slots,
        0.25,
        1.75,
        0.25,
        0.40,
        2.0,
    );
    let step = SpatialVendorizationStep::admit(
        commitment,
        arena.actor,
        arena.a,
        &cells,
        effect(arena.property, true),
        true,
        1.0,
        Some(1),
    )
    .unwrap();
    assert_eq!(step.deciding_cell(), arena.b);

    let actor_slot = arena.allocator.slot_of(arena.actor).unwrap();
    let cargo_slot = arena.allocator.slot_of(arena.cargo).unwrap();
    let n_dims = arena.registry.total_columns;
    let mut shadow: Vec<f32> = (0..arena.allocator.capacity() * n_dims)
        .map(|i| i as f32 + 0.125)
        .collect();
    let before = shadow.clone();
    apply_spatial_step(
        &step,
        &mut arena.tree,
        &mut arena.allocator,
        &mut arena.registry,
        &mut shadow,
        n_dims,
    );
    assert_eq!(arena.allocator.slot_of(arena.actor), Some(actor_slot));
    assert_eq!(arena.allocator.slot_of(arena.cargo), Some(cargo_slot));
    assert_eq!(shadow, before, "reparent must not relocate or rewrite rows");
    assert_eq!(
        arena.allocator.relation_of(arena.actor),
        Some(ObjectResidencyRelation::ChildOf(arena.b))
    );
    assert!(arena.tree.has_overlay(arena.actor, step.overlay_id()));
    assert_eq!(step.overlay().origin, arena.b);
    assert_eq!(step.cost_band_draw().n, 1);
    assert!(step.cost_band_draw().conserves_exactly());
}

#[test]
fn actionband_structural_door_emits_spatial_reparent_from_sealed_crossing() {
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP actionband_structural_door: no local GPU");
        return;
    };
    let arena = arena();
    let slots = ASSIGNMENT_PRIMARY;
    let (frozen, thresholds, ab_registry) = admit_spatial_actionband(slots);
    let cells = topology_cells(
        arena.a,
        arena.b,
        arena.c,
        thresholds[0].col.raw_u32(),
        slots,
    );
    let template = frozen.templates()[0].index();
    // Same dual active set as the R2 field-only referee.
    let active = [
        ActionBandActiveInstance::new(template, SlotIndex::new(slots.b), [0.0; 4]),
        ActionBandActiveInstance::new(template, SlotIndex::new(slots.c), [0.0; 4]),
    ];
    let compiled = compile_action_band_gpu_execution(
        &frozen,
        &simthing_core::EmlExpressionRegistry::new(),
        &[ActionBandEmissionBindingGpu::structural_request(0)],
        &active,
    )
    .unwrap();

    // Pre-admit the spatial Reparent consequence (not a generic Remove).
    // Two event kinds (B/C) share destination_index 0 → same Reparent shape;
    // field state selects which commitment fires.
    let mut pre_admitted = vec![None; 1];
    pre_admitted[0] = Some(BoundaryRequest::Reparent {
        child: arena.actor,
        new_parent: arena.b,
    });
    let requests =
        FrozenActionBandStructuralRequests::from_compiled_admission(&compiled, pre_admitted)
            .expect("session-frozen structural door");

    let commitment = sealed_commitment_from_field_state(
        &ctx,
        &frozen,
        &thresholds,
        &ab_registry,
        slots,
        0.25,
        1.75,
        0.25,
        0.40,
        2.0,
    );
    // Also prove pure consumer agrees.
    let step = SpatialVendorizationStep::admit(
        commitment,
        arena.actor,
        arena.a,
        &cells,
        effect(arena.property, true),
        true,
        1.0,
        Some(1),
    )
    .unwrap();
    assert_eq!(step.deciding_cell(), arena.b);

    let (sender, receiver) = feeder_channel();
    let submitted = requests
        .submit_committed(&[commitment], &sender)
        .expect("sealed commitment selects fixed spatial Reparent");
    assert_eq!(submitted, 1);
    let drained = receiver.drain_now();
    let boundary_requests: Vec<_> = drained
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("structural door emitted non-boundary work"),
        })
        .collect();
    assert!(matches!(
        &boundary_requests[0],
        BoundaryRequest::Reparent { child, new_parent }
            if *child == arena.actor && *new_parent == arena.b
    ));

    let mut tree = arena.tree;
    let mut allocator = arena.allocator;
    let mut registry = arena.registry;
    let n_dims = registry.total_columns;
    let mut shadow = vec![0.0; allocator.capacity() * n_dims];
    let outcome = apply_structural_mutations(
        boundary_requests,
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
    assert!(outcome
        .reparented
        .iter()
        .any(|pair| *pair == (arena.actor, arena.b)));
}

// ─── R4: slot permutation ───────────────────────────────────────────────────

#[test]
fn physical_slot_assignment_permutation_preserves_spatial_choice() {
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP physical_slot_assignment_permutation: no local GPU");
        return;
    };
    // R4: two genuinely different opaque sealed-slot assignments for the same
    // logical topology. Not a vector reorder of the same keys.
    //   primary:  A→10, B→20, C→30
    //   permuted: A→77, B→10, C→55
    let arena = arena();

    let run = |slots: SlotAssignment| {
        let (frozen, thresholds, ab_registry) = admit_spatial_actionband(slots);
        assert_eq!(frozen.templates().len(), 1);
        let cells = topology_cells(
            arena.a,
            arena.b,
            arena.c,
            thresholds[0].col.raw_u32(),
            slots,
        );
        // Same logical field state: B crosses, C does not.
        let commit = sealed_commitment_from_field_state(
            &ctx,
            &frozen,
            &thresholds,
            &ab_registry,
            slots,
            0.25,
            1.75,
            0.25,
            0.40,
            2.0,
        );
        assert_eq!(
            commit.slot(),
            slots.b,
            "field-selected sealed slot must be B under this assignment"
        );
        let step = SpatialVendorizationStep::admit(
            commit,
            arena.actor,
            arena.a,
            &cells,
            effect(arena.property, true),
            true,
            1.0,
            Some(1),
        )
        .expect("admit under slot assignment");
        assert_eq!(step.deciding_cell(), arena.b);
        assert_eq!(manhattan(cells[0], cells[1]), 1);
        (step.deciding_cell(), step.commitment().slot(), slots)
    };

    let (dest_primary, sealed_primary, assign_primary) = run(ASSIGNMENT_PRIMARY);
    let (dest_permuted, sealed_permuted, assign_permuted) = run(ASSIGNMENT_PERMUTED);

    // Same logical destination and one-edge consequence.
    assert_eq!(dest_primary, arena.b);
    assert_eq!(dest_permuted, arena.b);
    assert_eq!(dest_primary, dest_permuted);
    // Physical sealed keys differ across assignments.
    assert_ne!(assign_primary.b, assign_permuted.b);
    assert_ne!(sealed_primary, sealed_permuted);
    assert_eq!(sealed_primary, assign_primary.b);
    assert_eq!(sealed_permuted, assign_permuted.b);
}

// ─── R5 table-driven matrix: fail-closed mutants ────────────────────────────

#[derive(Clone, Copy)]
enum MutantKind {
    AmbiguousLocus,
    UnboundLocus,
    NonAdjacent,
    SelfStep,
    HardcodedOrigin,
    SessionEndLifecycle,
    BareUntilDissolved,
    FreeRepositionConsumes,
}

struct MutantCase {
    name: &'static str,
    kind: MutantKind,
}

const MUTANT_CASES: &[MutantCase] = &[
    MutantCase {
        name: "ambiguous_locus",
        kind: MutantKind::AmbiguousLocus,
    },
    MutantCase {
        name: "unbound_locus",
        kind: MutantKind::UnboundLocus,
    },
    MutantCase {
        name: "non_adjacent",
        kind: MutantKind::NonAdjacent,
    },
    MutantCase {
        name: "self_step",
        kind: MutantKind::SelfStep,
    },
    MutantCase {
        name: "hardcoded_origin",
        kind: MutantKind::HardcodedOrigin,
    },
    MutantCase {
        name: "session_end_lifecycle",
        kind: MutantKind::SessionEndLifecycle,
    },
    MutantCase {
        name: "bare_until_dissolved",
        kind: MutantKind::BareUntilDissolved,
    },
    MutantCase {
        name: "free_reposition_must_not_consume",
        kind: MutantKind::FreeRepositionConsumes,
    },
];

#[test]
fn matrix_shaped_mutants_fail_closed() {
    let Some(ctx) = require_gpu() else {
        eprintln!("SKIP matrix_shaped_mutants: no local GPU");
        return;
    };
    let arena = arena();
    let slots = ASSIGNMENT_PRIMARY;
    let (frozen, thresholds, ab_registry) = admit_spatial_actionband(slots);
    let cells = topology_cells(
        arena.a,
        arena.b,
        arena.c,
        thresholds[0].col.raw_u32(),
        slots,
    );
    let sealed_col = thresholds[0].col.raw_u32();
    let good_commit = sealed_commitment_from_field_state(
        &ctx,
        &frozen,
        &thresholds,
        &ab_registry,
        slots,
        0.25,
        1.75,
        0.25,
        0.40,
        2.0,
    );

    for case in MUTANT_CASES {
        match case.kind {
            MutantKind::AmbiguousLocus => {
                let mut amb = cells.clone();
                amb.push(cells[1]);
                let err = SpatialVendorizationStep::admit(
                    good_commit,
                    arena.actor,
                    arena.a,
                    &amb,
                    effect(arena.property, true),
                    true,
                    1.0,
                    Some(1),
                )
                .expect_err(case.name);
                assert!(
                    matches!(
                        err,
                        SpatialVendorizationError::AmbiguousDecisionLocus { .. }
                    ),
                    "{} => {err:?}",
                    case.name
                );
            }
            MutantKind::UnboundLocus => {
                // Commitment for B with mapping that only knows A and C.
                let unbound: Vec<_> = cells
                    .iter()
                    .copied()
                    .filter(|c| c.cell != arena.b)
                    .collect();
                let err = SpatialVendorizationStep::admit(
                    good_commit,
                    arena.actor,
                    arena.a,
                    &unbound,
                    effect(arena.property, true),
                    true,
                    1.0,
                    Some(1),
                )
                .expect_err(case.name);
                assert!(
                    matches!(err, SpatialVendorizationError::UnboundDecisionLocus { .. }),
                    "{} => {err:?}",
                    case.name
                );
            }
            MutantKind::NonAdjacent => {
                let a = cells[0];
                let far = AdmittedTopologyCell {
                    sealed_slot: 999,
                    sealed_col,
                    grid_row: 1,
                    grid_col: 1,
                    cell: SimThingId::new(),
                };
                assert!(
                    matches!(
                        reject_non_adjacent(a, far),
                        Err(SpatialVendorizationError::NotOneN4Edge { .. })
                    ),
                    "{}",
                    case.name
                );
            }
            MutantKind::SelfStep => {
                assert!(
                    matches!(
                        reject_non_adjacent(cells[0], cells[0]),
                        Err(SpatialVendorizationError::NotOneN4Edge { .. })
                    ),
                    "{}",
                    case.name
                );
            }
            MutantKind::HardcodedOrigin => {
                let overlay = Overlay {
                    id: OverlayId::new(),
                    kind: OverlayKind::Instruction,
                    source: OverlaySource::System,
                    origin: arena.a, // wrong: not deciding cell B
                    affects: vec![arena.actor],
                    transform: PropertyTransformDelta {
                        property_id: arena.property,
                        sub_field_deltas: vec![],
                    },
                    lifecycle: OverlayLifecycle::UntilDissolvedWith {
                        dissolution_conditions: vec![DissolveCondition::AfterTicks {
                            remaining: 1,
                        }],
                    },
                };
                assert_eq!(
                    validate_spatial_overlay(arena.actor, arena.b, &overlay),
                    Err(SpatialVendorizationError::OverlayOriginDrift),
                    "{}",
                    case.name
                );
            }
            MutantKind::SessionEndLifecycle => {
                let overlay = Overlay {
                    id: OverlayId::new(),
                    kind: OverlayKind::Instruction,
                    source: OverlaySource::System,
                    origin: arena.b,
                    affects: vec![arena.actor],
                    transform: PropertyTransformDelta {
                        property_id: arena.property,
                        sub_field_deltas: vec![],
                    },
                    lifecycle: OverlayLifecycle::UntilDissolvedWith {
                        dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
                    },
                };
                assert_eq!(
                    validate_spatial_overlay(arena.actor, arena.b, &overlay),
                    Err(SpatialVendorizationError::LawfulLifecycleRequired),
                    "{}",
                    case.name
                );
            }
            MutantKind::BareUntilDissolved => {
                let overlay = Overlay {
                    id: OverlayId::new(),
                    kind: OverlayKind::Instruction,
                    source: OverlaySource::System,
                    origin: arena.b,
                    affects: vec![arena.actor],
                    transform: PropertyTransformDelta {
                        property_id: arena.property,
                        sub_field_deltas: vec![],
                    },
                    lifecycle: OverlayLifecycle::UntilDissolved,
                };
                assert_eq!(
                    validate_spatial_overlay(arena.actor, arena.b, &overlay),
                    Err(SpatialVendorizationError::LawfulLifecycleRequired),
                    "{}",
                    case.name
                );
            }
            MutantKind::FreeRepositionConsumes => {
                // Observation path must complete N=0 with R=V.
                let step = SpatialVendorizationStep::admit(
                    good_commit,
                    arena.actor,
                    arena.a,
                    &cells,
                    effect(arena.property, false),
                    false,
                    1.0,
                    None,
                )
                .expect(case.name);
                let draw = step.cost_band_draw();
                assert_eq!(draw.n, 0, "{}", case.name);
                assert_eq!(draw.r.to_bits(), draw.v.to_bits(), "{}", case.name);
            }
        }
    }
}

// ─── Distinct non-matrix obligations ────────────────────────────────────────

#[test]
fn placement_ownership_uses_existing_root_bind_not_participant_stamps() {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("spatial-witness", "progress", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut a = SimThing::new(SimThingKind::Location, 0);
    let mut b = SimThing::new(SimThingKind::Location, 0);
    bind_owner(&mut a, &OwnerRef::new("alpha"));
    bind_owner(&mut b, &OwnerRef::new("beta"));
    let (a_id, b_id) = (a.id, b.id);
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    actor.add_property(property, registry.property(property).default_value());
    let actor_id = actor.id;
    let cargo = SimThing::new(SimThingKind::Cohort, 0);
    let cargo_id = cargo.id;
    actor.add_child(cargo);
    a.add_child(actor);
    root.add_child(a);
    root.add_child(b);

    assert_eq!(resolve_owner(&root, actor_id).unwrap().as_str(), "alpha");
    assert_eq!(resolve_owner(&root, cargo_id).unwrap().as_str(), "alpha");

    let actor_node = {
        let a_node = root.children.iter_mut().find(|n| n.id == a_id).unwrap();
        let idx = a_node
            .children
            .iter()
            .position(|n| n.id == actor_id)
            .unwrap();
        a_node.children.remove(idx)
    };
    root.children
        .iter_mut()
        .find(|n| n.id == b_id)
        .unwrap()
        .children
        .push(actor_node);
    let actor_mut = root
        .children
        .iter_mut()
        .find(|n| n.id == b_id)
        .unwrap()
        .children
        .iter_mut()
        .find(|n| n.id == actor_id)
        .unwrap();
    bind_owner(actor_mut, &OwnerRef::new("beta"));

    assert_eq!(resolve_owner(&root, actor_id).unwrap().as_str(), "beta");
    assert_eq!(resolve_owner(&root, cargo_id).unwrap().as_str(), "beta");
    assert!(root
        .children
        .iter()
        .find(|n| n.id == b_id)
        .unwrap()
        .children
        .iter()
        .find(|n| n.id == actor_id)
        .unwrap()
        .properties
        .contains_key(&OWNER_CHANNEL_PROPERTY_ID));
    assert!(!root
        .children
        .iter()
        .find(|n| n.id == b_id)
        .unwrap()
        .children
        .iter()
        .find(|n| n.id == actor_id)
        .unwrap()
        .children[0]
        .properties
        .contains_key(&OWNER_CHANNEL_PROPERTY_ID));
}

#[test]
fn palma_d_is_a_field_not_a_path_and_feeds_locus_radius_target() {
    let config = MinPlusStencilConfig {
        width: 2,
        height: 2,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 1,
        dest_y: 1,
        inf_sentinel: MIN_PLUS_INF,
    };
    let mut w = vec![1.0f32; 4];
    w[3] = 0.0;
    let packed = pack_w_and_initial_d(&w, &config).expect("pack PALMA W/D");
    let d = cpu_min_plus_d_from_w(&w, &config, 4).expect("PALMA D field");
    assert!(d[0].is_finite());
    assert_eq!(d[3].to_bits(), 0.0f32.to_bits());
    assert_eq!(packed.len(), config.values_len());

    let mut registry = DimensionRegistry::new();
    let d_prop = registry.register(SimProperty::simple("spatial-witness", "palma-d", 0));
    let d_col = registry
        .column_range(d_prop)
        .col_for_role(&SubFieldRole::Amount, &registry.property(d_prop).layout)
        .unwrap();
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: d_col,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: SPATIAL_EVENT_KIND_B,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(
            &ActionBandSessionSpec {
                budget: ActionBandAdmissionBudgetSpec {
                    axis_channel_count: 1,
                    dependency_binding_count: 0,
                    storage_rows: 1,
                    eml_program_count: 0,
                    emission_binding_count: 0,
                },
                templates: vec![ActionBandTemplateSpec {
                    id: "palma-reachable-spatial".into(),
                    label: Some("shadow-only".into()),
                    axis_channels: vec![ActionBandChannelBindingSpec {
                        column: d_col.raw_u32(),
                        kind: ActionBandChannelKind::CachedDerived,
                    }],
                    target: ActionBandTargetSpec::PalmaReachableSet {
                        distance_channel: d_col.raw_u32(),
                        maximum_distance: d[0].max(1.0),
                    },
                    velocity: None,
                    bands: vec![ActionBandBandSpec {
                        threshold_registration_index: 0,
                        eml_program: None,
                        emission_binding_indices: vec![],
                    }],
                    subordinate_template_ids: vec![],
                    max_active_subordinates: 0,
                    reserved_instance_rows: 1,
                    requirement_semantics: Default::default(),
                }],
            },
            &registry,
            &simthing_core::EmlExpressionRegistry::new(),
            &thresholds,
        )
        .expect("PalmaReachableSet admits as ActionBand target form");
    assert_eq!(frozen.templates().len(), 1);
}

#[test]
fn production_core_has_no_peer_movement_facility() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .parent()
        .expect("workspace root");
    let production_roots = [
        "crates/simthing-spec/src",
        "crates/simthing-kernel/src",
        "crates/simthing-gpu/src",
        "crates/simthing-driver/src",
        "crates/simthing-sim/src",
        "crates/simthing-core/src",
    ];
    let forbidden = [
        "MovementCommitment",
        "MovementFieldLocus",
        "MovementIngress",
        "apply_movement_commitments",
        "DestinationRegistry",
        "MovementPlanner",
        "struct MovementPath",
        "struct MovementDestination",
    ];
    for root in production_roots {
        for path in walkdir_rs_files(&workspace.join(root)) {
            let text = std::fs::read_to_string(&path).expect("read");
            for needle in forbidden {
                assert!(
                    !text.contains(needle),
                    "production peer movement facility '{needle}' found in {}",
                    path.display()
                );
            }
        }
    }
    assert!(workspace
        .join("crates/simthing-workshop/src/actionband_spatial_vendorization_0.rs")
        .exists());
    assert!(workspace
        .join("crates/simthing-spec/src/spec/action_band.rs")
        .exists());
    assert!(workspace
        .join("crates/simthing-kernel/src/accumulator_op/action_band_execution.rs")
        .exists());
}

#[test]
fn costband_bypass_shape_is_detectable_against_oracle() {
    let good = cost_band_quantize(1.75, 1.0, true, Some(1)).unwrap();
    let direct_decrement = CostBandDraw {
        r: good.r + 0.25,
        ..good
    };
    assert_ne!(direct_decrement, good);
    assert_ne!(direct_decrement.r.to_bits(), good.r.to_bits());
}

fn walkdir_rs_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
    out
}

// Silence unused import when GPU path not exercised in pure tests.
#[allow(dead_code)]
fn _resolve_export() {
    let _ = resolve_authoritative_cell as fn(&[AdmittedTopologyCell], u32, u32) -> _;
    let _ = manhattan;
}
