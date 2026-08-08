//! MOVEMENT-DECISION-INGRESS-0 focused production-path proof.

use simthing_core::owner_channel::{bind_owner, OwnerRef, OWNER_CHANNEL_PROPERTY_ID};
use simthing_core::{
    cost_band_quantize, CostBandDraw, DimensionRegistry, DissolveCondition, Overlay, OverlayId,
    OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty,
    SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, GpuContext,
    PackedThresholdUpload, SlotAllocator, ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES,
};
use simthing_kernel::{
    BoundaryEmissionToken, EmissionToken, StructuralCommitment, ThresholdCrossingToken,
};
use simthing_sim::{
    apply_movement_commitments, validate_movement_cost_band, validate_movement_overlay,
    CostBandSemantic, MovementCommitment, MovementFieldLocus, MovementIngressError,
    MovementOverlayEffect, SimRuntimeTree,
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
    loci: Vec<MovementFieldLocus>,
}

fn arena() -> Arena {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("core", "movement_pressure", 0));

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
    allocator.populate_from_tree(&root);
    let loci = vec![
        MovementFieldLocus {
            slot: 0,
            value_col: 0,
            grid_row: 0,
            grid_col: 0,
            cell: a_id,
        },
        MovementFieldLocus {
            slot: 1,
            value_col: 0,
            grid_row: 0,
            grid_col: 1,
            cell: b_id,
        },
        MovementFieldLocus {
            slot: 2,
            value_col: 0,
            grid_row: 1,
            grid_col: 0,
            cell: c_id,
        },
    ];
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
        loci,
    }
}

fn sealed_commitment(decision_slot: u32, value: f32) -> StructuralCommitment {
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("GPU context");
    let n_slots = 4;
    let n_dims = 1;
    let mut session = AccumulatorOpSession::new_attached(&ctx, n_slots, n_dims, 8);
    session.bind_generation_authority(7);
    let previous = vec![0.0; (n_slots * n_dims) as usize];
    let mut current = previous.clone();
    current[(decision_slot * n_dims) as usize] = value;
    session.upload_values(&ctx, &current);
    session.upload_previous_values(&ctx, &previous);
    let regs = [ThresholdRegistration {
        slot: decision_slot,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 0x4d4f_5645,
        buffer: THRESH_BUF_VALUES,
    }];
    session
        .upload_packed_threshold_ops(
            &ctx,
            &PackedThresholdUpload::from_registrations(&regs).expect("threshold pack"),
        )
        .expect("threshold upload");
    session.tick(&ctx, 0).expect("sealed threshold scan");
    let events = session
        .readback_threshold_events(&ctx)
        .expect("sealed events");
    let emissions = session
        .readback_threshold_emissions(&ctx)
        .expect("sealed emissions");
    assert_eq!(events.len(), 1);
    assert_eq!(emissions.len(), 1);
    assert!(events[0].is_production_sealed());
    assert!(emissions[0].is_production_sealed());
    assert_eq!(events[0].generation(), 7);
    assert_eq!(emissions[0].generation(), 7);

    let threshold = ThresholdCrossingToken::from_sealed_threshold_event(&events[0]);
    let emission = EmissionToken::from_sealed_threshold_emission(&emissions[0]);
    let boundary = BoundaryEmissionToken::bind(threshold, emission).expect("same sealed locus");
    StructuralCommitment::mint_from_sealed_path(threshold, emission, boundary)
        .expect("sealed structural commitment")
}

fn effect(property: SimPropertyId, consuming: bool) -> MovementOverlayEffect {
    MovementOverlayEffect {
        property_id: property,
        deltas: consuming
            .then(|| vec![(SubFieldRole::Amount, TransformOp::add(0.25))])
            .unwrap_or_default(),
    }
}

#[test]
fn sealed_cell_crossing_moves_one_edge_with_stable_slot_root_owner_bind_and_arrival_overlay() {
    let mut arena = arena();
    let commitment = sealed_commitment(1, 1.75);
    let mut reversed = arena.loci.clone();
    reversed.reverse();
    let movement = MovementCommitment::admit(
        commitment,
        arena.actor,
        arena.a,
        2,
        &reversed,
        effect(arena.property, true),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .expect("sealed cell B is one admitted edge from cell A");
    assert_eq!(movement.deciding_cell(), arena.b);

    let actor_slot = arena.allocator.slot_of(arena.actor).unwrap();
    let cargo_slot = arena.allocator.slot_of(arena.cargo).unwrap();
    let n_dims = arena.registry.total_columns;
    let mut shadow: Vec<f32> = (0..arena.allocator.capacity() * n_dims)
        .map(|index| index as f32 + 0.125)
        .collect();
    let before = shadow.clone();
    let (maintainer, outcome) = apply_movement_commitments(
        vec![movement.clone()],
        &mut arena.tree,
        &mut arena.allocator,
        &mut arena.registry,
        &mut shadow,
        n_dims,
    );

    assert_eq!(outcome.applied, 1);
    assert_eq!(outcome.rejected, 0);
    assert_eq!(outcome.owner_root_binds, 1);
    assert_eq!(maintainer.reparented, vec![(arena.actor, arena.b)]);
    assert_eq!(arena.tree.parent_id_of(arena.actor), Some(arena.b));
    assert_eq!(arena.allocator.slot_of(arena.actor), Some(actor_slot));
    assert_eq!(arena.allocator.slot_of(arena.cargo), Some(cargo_slot));
    assert_eq!(shadow, before, "reparent must not relocate or rewrite rows");
    assert_eq!(
        arena.tree.resolved_owner(arena.actor).unwrap().as_str(),
        "beta"
    );
    assert_eq!(
        arena.tree.resolved_owner(arena.cargo).unwrap().as_str(),
        "beta"
    );
    assert!(arena
        .tree
        .snapshot_node(arena.actor)
        .unwrap()
        .property_ids
        .contains(&OWNER_CHANNEL_PROPERTY_ID));
    assert!(!arena
        .tree
        .snapshot_node(arena.cargo)
        .unwrap()
        .property_ids
        .contains(&OWNER_CHANNEL_PROPERTY_ID));

    let overlay = arena
        .tree
        .overlay_snapshot(arena.actor, movement.overlay_id())
        .expect("in-transit overlay attached to moved root");
    assert_eq!(overlay.origin, arena.b);
    assert_eq!(overlay.affects, vec![arena.actor]);
    assert_eq!(
        overlay.lifecycle,
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::ArrivedAt {
                destination: arena.b,
            }],
        }
    );
    assert_eq!(movement.cost_band_draw().n, 1);
    assert!(movement.cost_band_draw().conserves_exactly());
}

#[test]
fn changing_sealed_field_locus_changes_choice_without_editing_identity_or_using_order() {
    let arena = arena();
    let to_b = MovementCommitment::admit(
        sealed_commitment(1, 1.5),
        arena.actor,
        arena.a,
        2,
        &arena.loci,
        effect(arena.property, true),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .unwrap();
    let mut reordered = arena.loci.clone();
    reordered.rotate_left(1);
    let to_c = MovementCommitment::admit(
        sealed_commitment(2, 1.5),
        arena.actor,
        arena.a,
        2,
        &reordered,
        effect(arena.property, true),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .unwrap();
    assert_eq!(to_b.deciding_cell(), arena.b);
    assert_eq!(to_c.deciding_cell(), arena.c);

    let mut ambiguous = arena.loci.clone();
    ambiguous.push(arena.loci[1]);
    assert!(matches!(
        MovementCommitment::admit(
            sealed_commitment(1, 1.5),
            arena.actor,
            arena.a,
            2,
            &ambiguous,
            effect(arena.property, true),
            CostBandSemantic::admit_sink(Some(1), None).unwrap(),
            1.0,
        ),
        Err(MovementIngressError::AmbiguousDecisionLocus { .. })
    ));
}

#[test]
fn free_repositioning_uses_same_costband_path_and_consumes_zero() {
    let arena = arena();
    let movement = MovementCommitment::admit(
        sealed_commitment(1, 1.75),
        arena.actor,
        arena.a,
        2,
        &arena.loci,
        effect(arena.property, false),
        CostBandSemantic::observation(),
        1.0,
    )
    .unwrap();
    let draw = movement.cost_band_draw();
    assert_eq!(draw.n, 0);
    assert_eq!(draw.r.to_bits(), draw.v.to_bits());
    assert_eq!(draw.c.to_bits(), 0.0f32.to_bits());
}

fn overlay_with(
    actor: SimThingId,
    origin: SimThingId,
    property: SimPropertyId,
    lifecycle: OverlayLifecycle,
) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin,
        affects: vec![actor],
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: Vec::new(),
        },
        lifecycle,
    }
}

#[test]
fn arrived_at_dissolves_from_authoritative_residency_relation() {
    let registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut destination = SimThing::new(SimThingKind::Location, 0);
    let destination_id = destination.id;
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    let actor_id = actor.id;
    actor.add_overlay(overlay_with(
        actor_id,
        destination_id,
        SimPropertyId(0),
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::ArrivedAt {
                destination: destination_id,
            }],
        },
    ));
    destination.add_child(actor);
    root.add_child(destination);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let out = simthing_sim::overlay_lifecycle::resolve_overlay_lifecycle(
        &mut root,
        &registry,
        &allocator,
        &mut [],
        0,
        1,
        None,
    );
    assert_eq!(out.dissolved, 1);
    assert!(root.children[0].children[0].overlays.is_empty());
}

#[test]
fn origin_lifecycle_cost_and_missing_endpoint_mutants_red_the_production_validators() {
    let mut arena = arena();
    let synthetic = SimThingId::new();
    let arrival = OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions: vec![DissolveCondition::ArrivedAt {
            destination: arena.b,
        }],
    };
    let hardcoded_origin = overlay_with(arena.actor, arena.a, arena.property, arrival.clone());
    assert_eq!(
        validate_movement_overlay(arena.actor, arena.b, &hardcoded_origin),
        Err(MovementIngressError::OverlayOriginDrift)
    );
    let synthesized_origin = overlay_with(arena.actor, synthetic, arena.property, arrival);
    assert_eq!(
        validate_movement_overlay(arena.actor, arena.b, &synthesized_origin),
        Err(MovementIngressError::OverlayOriginDrift)
    );
    let session_end = overlay_with(
        arena.actor,
        arena.b,
        arena.property,
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
        },
    );
    assert_eq!(
        validate_movement_overlay(arena.actor, arena.b, &session_end),
        Err(MovementIngressError::ArrivalLifecycleRequired)
    );
    let bare = overlay_with(
        arena.actor,
        arena.b,
        arena.property,
        OverlayLifecycle::UntilDissolved,
    );
    assert_eq!(
        validate_movement_overlay(arena.actor, arena.b, &bare),
        Err(MovementIngressError::ArrivalLifecycleRequired)
    );

    let commitment = sealed_commitment(1, 1.75);
    let semantic = CostBandSemantic::admit_sink(Some(1), None).unwrap();
    let good = cost_band_quantize(1.75, 1.0, true, Some(1)).unwrap();
    let direct_decrement = CostBandDraw {
        r: good.r + 0.25,
        ..good
    };
    assert_eq!(
        validate_movement_cost_band(commitment, semantic, 1.0, direct_decrement),
        Err(MovementIngressError::CostBandBypass)
    );

    let mut foreign_loci = arena.loci.clone();
    foreign_loci[1].cell = synthetic;
    let foreign = MovementCommitment::admit(
        commitment,
        arena.actor,
        arena.a,
        2,
        &foreign_loci,
        effect(arena.property, true),
        semantic,
        1.0,
    )
    .unwrap();
    let n_dims = arena.registry.total_columns;
    let mut shadow = vec![0.0; arena.allocator.capacity() * n_dims];
    let (_, rejected) = apply_movement_commitments(
        vec![foreign],
        &mut arena.tree,
        &mut arena.allocator,
        &mut arena.registry,
        &mut shadow,
        n_dims,
    );
    assert_eq!(rejected.applied, 0);
    assert_eq!(rejected.rejected, 1);
    assert_eq!(arena.tree.parent_id_of(arena.actor), Some(arena.a));
}
