//! RESOLUTION-SITE-SPLIT-0 referees — one model, two resolution sites.
//!
//! Synthetic inline input only (Invariant Set law: no corpus, fixture, or
//! generator prerequisite). Crossings are minted through the CPU-oracle twin of
//! the shader's crossing predicate (`cpu_oracle_threshold_events`), so both
//! placements consume the same sealed slot-space event vocabulary the GPU
//! emits: `{slot, col, value, event_kind}` — no identity on the wire.
//!
//! Bit-identity convention: products derive `PartialEq` and every f32-bearing
//! field is additionally compared on `to_bits()`; stream-level equality is also
//! checked on `{:?}` formatting, which is bit-faithful for the finite payloads
//! used here (shortest-roundtrip float repr distinguishes distinct bit
//! patterns, including -0.0).

use simthing_core::{
    deliver_routed_overlay, DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing,
    SimThingId, SimThingKind, SlotIndex, SubFieldRole, TransformOp,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    cpu_oracle_threshold_events, SlotAllocator, ThresholdEvent, ThresholdEventGpu,
    ThresholdRegistration, DIR_UPWARD, THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
};
use simthing_sim::{
    collect_aggregate_alerts_vendorized, collect_velocity_alerts_vendorized,
    mint_attach_overlay_at_barrier, reattach_aggregate_alerts_at_barrier,
    reattach_velocity_alerts_at_barrier, BoundaryProtocol, ResolutionSite, SimRuntimeTree,
    SlotIdentityReattachError, SlotSpaceOverlayDraft, ThresholdRegistry, ThresholdSemantic,
};

/// Synthetic arena: root + two property-bearing children, slot map populated
/// from the tree (the admitted authority the closed loop re-attaches through).
struct Arena {
    tree: SimThing,
    registry: DimensionRegistry,
    allocator: SlotAllocator,
    a_id: SimThingId,
    b_id: SimThingId,
    pid: SimPropertyId,
}

fn arena() -> Arena {
    let mut registry = DimensionRegistry::new();
    let pid = registry.register(SimProperty::simple("core", "pressure", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut a = SimThing::new(SimThingKind::Cohort, 0);
    a.add_property(pid, registry.property(pid).default_value());
    let a_id = a.id;
    let mut b = SimThing::new(SimThingKind::Cohort, 0);
    b.add_property(pid, registry.property(pid).default_value());
    let b_id = b.id;
    root.add_child(a);
    root.add_child(b);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    Arena {
        tree: root,
        registry,
        allocator,
        a_id,
        b_id,
        pid,
    }
}

fn locus(arena: &Arena, id: SimThingId, role: &SubFieldRole) -> (u32, u32) {
    let slot = arena.allocator.slot_of(id).unwrap().raw();
    let layout = &arena.registry.property(arena.pid).layout;
    let col = arena
        .registry
        .column_range(arena.pid)
        .col_for_role(role, layout)
        .unwrap()
        .raw_u32();
    (slot, col)
}

/// Mint sealed crossings for the given registrations through the CPU-oracle
/// twin of the shader predicate: value buffer crossing 0.25 -> 0.75 upward.
fn oracle_crossings(arena: &Arena, regs: &[ThresholdRegistration]) -> Vec<ThresholdEvent> {
    let n_dims = arena.registry.total_columns as u32;
    let n_slots = arena.allocator.capacity() as u32;
    let len = (n_slots * n_dims) as usize;
    let mut prev = vec![0.25f32; len];
    let mut curr = vec![0.75f32; len];
    // Distinct value per locus so bit-compares are non-degenerate.
    for (i, reg) in regs.iter().enumerate() {
        let addr = (reg.slot * n_dims + reg.col) as usize;
        prev[addr] = 0.25 + i as f32 * 0.01;
        curr[addr] = 0.75 + i as f32 * 0.01;
    }
    let events =
        cpu_oracle_threshold_events(&prev, &curr, &prev.clone(), &curr.clone(), n_dims, regs, 7);
    assert!(
        !events.is_empty(),
        "referee precondition: oracle crossings must be non-empty"
    );
    events
}

fn reg(slot: u32, col: u32, event_kind: u32, buffer: u32) -> ThresholdRegistration {
    ThresholdRegistration {
        slot,
        col,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind,
        buffer,
    }
}

#[test]
fn velocity_alert_parity_bit_identical_and_mirror_drift_mutant_reds() {
    let arena = arena();
    let role = SubFieldRole::Velocity;
    let (slot_a, col_a) = locus(&arena, arena.a_id, &role);
    let (slot_b, col_b) = locus(&arena, arena.b_id, &role);

    let mut sem = ThresholdRegistry::new();
    let kind_a = sem.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: arena.a_id,
        property_id: arena.pid,
        sub_field: role.clone(),
    });
    let kind_b = sem.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: arena.b_id,
        property_id: arena.pid,
        sub_field: role.clone(),
    });

    let regs = vec![
        reg(slot_a, col_a, kind_a, THRESH_BUF_VALUES),
        reg(slot_b, col_b, kind_b, THRESH_BUF_VALUES),
    ];
    let events = oracle_crossings(&arena, &regs);

    let vendorized = collect_velocity_alerts_vendorized(&events, &sem);
    let closed_loop =
        reattach_velocity_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
            .expect("total over admitted crossings");

    assert_eq!(closed_loop.len(), events.len());
    assert_eq!(closed_loop, vendorized);
    assert_eq!(format!("{closed_loop:?}"), format!("{vendorized:?}"));
    for (cl, ven) in closed_loop.iter().zip(&vendorized) {
        assert_eq!(cl.value.to_bits(), ven.value.to_bits());
    }

    // Planted semantic divergence (TEST-LOCAL: no mutant constructor ships —
    // see the resolution_site module compile_fail proofs): a deliberately
    // drifted mirror registry built through the ordinary push door, same
    // shape and kinds, wrong registration-time identity at kind_a. The parity
    // referee must RED — the closed loop (authority) exposes the stale mirror.
    let mut drifted = ThresholdRegistry::new();
    let drifted_kind_a = drifted.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: SimThingId::new(),
        property_id: arena.pid,
        sub_field: role.clone(),
    });
    let drifted_kind_b = drifted.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: arena.b_id,
        property_id: arena.pid,
        sub_field: role.clone(),
    });
    assert_eq!((drifted_kind_a, drifted_kind_b), (kind_a, kind_b));
    let mutant_vendorized = collect_velocity_alerts_vendorized(&events, &drifted);
    assert_ne!(
        closed_loop, mutant_vendorized,
        "mirror drift must break dual-placement parity"
    );
}

#[test]
fn aggregate_alert_parity_bit_identical_and_mirror_drift_mutant_reds() {
    let arena = arena();
    let role = SubFieldRole::Amount;
    let (slot_a, col_a) = locus(&arena, arena.a_id, &role);

    let mut sem = ThresholdRegistry::new();
    let kind = sem.push(ThresholdSemantic::AggregateAlert {
        sim_thing_id: arena.a_id,
        property_id: arena.pid,
        sub_field: role.clone(),
    });

    let regs = vec![reg(slot_a, col_a, kind, THRESH_BUF_OUTPUT)];
    let events = oracle_crossings(&arena, &regs);

    let vendorized = collect_aggregate_alerts_vendorized(&events, &sem);
    let closed_loop =
        reattach_aggregate_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
            .expect("total over admitted crossings");

    assert_eq!(closed_loop.len(), events.len());
    assert_eq!(closed_loop, vendorized);
    assert_eq!(format!("{closed_loop:?}"), format!("{vendorized:?}"));
    for (cl, ven) in closed_loop.iter().zip(&vendorized) {
        assert_eq!(cl.value.to_bits(), ven.value.to_bits());
    }

    // TEST-LOCAL mirror-drift mutant (no mutant constructor ships): same
    // shape, wrong registration-time identity, via the ordinary push door.
    let mut drifted = ThresholdRegistry::new();
    let drifted_kind = drifted.push(ThresholdSemantic::AggregateAlert {
        sim_thing_id: SimThingId::new(),
        property_id: arena.pid,
        sub_field: role.clone(),
    });
    assert_eq!(drifted_kind, kind);
    let mutant_vendorized = collect_aggregate_alerts_vendorized(&events, &drifted);
    assert_ne!(
        closed_loop, mutant_vendorized,
        "mirror drift must break dual-placement parity"
    );
}

fn draft(arena: &Arena) -> SlotSpaceOverlayDraft {
    SlotSpaceOverlayDraft {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin_slot: arena.allocator.slot_of(arena.a_id).unwrap(),
        target_slot: arena.allocator.slot_of(arena.b_id).unwrap(),
        transform: PropertyTransformDelta {
            property_id: arena.pid,
            sub_field_deltas: vec![
                (SubFieldRole::Amount, TransformOp::set(0.625)),
                (SubFieldRole::Velocity, TransformOp::add(-0.0)),
            ],
        },
        lifecycle: OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
        },
    }
}

/// The vendorized origination twin: the pre-split production shape, an
/// `Overlay` constructed directly from `SimThingId`s already in scope.
fn vendorized_attach(arena: &Arena, d: &SlotSpaceOverlayDraft) -> BoundaryRequest {
    BoundaryRequest::AttachOverlay {
        target: arena.b_id,
        source_generation: simthing_core::GenerationStamp::new(0),
        overlay: Overlay {
            id: d.id,
            kind: d.kind.clone(),
            source: d.source.clone(),
            origin: arena.a_id,
            affects: Vec::new(),
            transform: d.transform.clone(),
            lifecycle: d.lifecycle.clone(),
        },
    }
}

#[test]
fn slot_space_origination_attach_stream_bit_identical_and_divergence_mutant_reds() {
    let arena = arena();
    let d = draft(&arena);

    // Closed-loop placement: origin carried in slot space, SimThingId
    // re-attached only at the barrier through the admitted slot map.
    let closed_loop = mint_attach_overlay_at_barrier(
        &d,
        &arena.allocator,
        simthing_core::GenerationStamp::new(0),
    )
    .expect("admitted draft re-attaches totally");
    let vendorized = vendorized_attach(&arena, &d);

    // BIT-IDENTICAL BoundaryRequest streams (finite payloads; {:?} is
    // bit-faithful and covers the -0.0 delta planted in the draft).
    assert_eq!(format!("{closed_loop:?}"), format!("{vendorized:?}"));

    let BoundaryRequest::AttachOverlay {
        target, overlay, ..
    } = &closed_loop
    else {
        panic!("closed-loop mint must produce AttachOverlay");
    };
    assert_eq!(*target, arena.b_id);
    assert_eq!(overlay.origin, arena.a_id);
    assert!(
        overlay.affects.is_empty(),
        "affects is set by routed delivery, never by the mint door (no direct-affects bypass)"
    );

    // Planted semantic divergence in the origination path (TEST-LOCAL: the
    // divergent draft is mutated here, then run through the REAL mint door —
    // no divergence seam exists in production): parity REDs.
    let mut diverged = d.clone();
    diverged.transform.sub_field_deltas.pop();
    let mutant = mint_attach_overlay_at_barrier(
        &diverged,
        &arena.allocator,
        simthing_core::GenerationStamp::new(0),
    )
    .expect("mutant still re-attaches; it diverges semantically");
    assert_ne!(
        format!("{closed_loop:?}"),
        format!("{mutant:?}"),
        "transform divergence must break stream parity"
    );
}

#[test]
fn unadmitted_slot_reattachment_fails_closed_never_default_origin() {
    let arena = arena();

    // Unadmitted origin slot: the real door fails closed...
    let mut orphan = draft(&arena);
    orphan.origin_slot = SlotIndex::new(97);
    let err = mint_attach_overlay_at_barrier(
        &orphan,
        &arena.allocator,
        simthing_core::GenerationStamp::new(0),
    )
    .expect_err("a slot with no admitted SimThing is an admission-integrity failure");
    assert_eq!(
        err,
        SlotIdentityReattachError::UnadmittedOriginSlot {
            slot: 97,
            overlay: orphan.id,
        }
    );

    // ...while the forbidden comparator is constructed TEST-LOCALLY (no
    // synthesized-origin seam exists in production — see the module
    // compile_fail proofs): the request a defaulting door WOULD have minted,
    // an attributable overlay whose origin no admitted slot vouches for. The
    // real door's `Err` above is what makes this state unreachable in
    // production.
    let fallback = arena.tree.id;
    let forged = BoundaryRequest::AttachOverlay {
        target: arena.b_id,
        source_generation: simthing_core::GenerationStamp::new(0),
        overlay: Overlay {
            id: orphan.id,
            kind: orphan.kind.clone(),
            source: orphan.source.clone(),
            origin: fallback,
            affects: Vec::new(),
            transform: orphan.transform.clone(),
            lifecycle: orphan.lifecycle.clone(),
        },
    };
    let BoundaryRequest::AttachOverlay { overlay, .. } = &forged else {
        panic!("forbidden comparator is AttachOverlay");
    };
    assert_eq!(
        overlay.origin, fallback,
        "the comparator synthesizes a default origin — the door's Err above proves the \
         production path cannot reach this state"
    );
    assert_ne!(
        arena.allocator.owner_of(orphan.origin_slot),
        Some(overlay.origin),
        "no admitted slot vouches for the synthesized origin"
    );

    // Unadmitted target slot also fails closed.
    let mut untargeted = draft(&arena);
    untargeted.target_slot = SlotIndex::new(98);
    assert!(matches!(
        mint_attach_overlay_at_barrier(
            &untargeted,
            &arena.allocator,
            simthing_core::GenerationStamp::new(0),
        ),
        Err(SlotIdentityReattachError::UnadmittedTargetSlot { slot: 98, .. })
    ));

    // Alert re-attachment fails closed on an unadmitted crossing slot too:
    // TOTAL means every converted crossing resolves or the barrier errors —
    // never a skipped row, never a substituted identity.
    let role = SubFieldRole::Amount;
    let (_, col_a) = locus(&arena, arena.a_id, &role);
    let mut sem = ThresholdRegistry::new();
    let kind = sem.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: arena.a_id,
        property_id: arena.pid,
        sub_field: role,
    });
    let n_dims = arena.registry.total_columns as u32;
    let orphan_slot = 41u32;
    let len = ((orphan_slot + 1) * n_dims) as usize;
    let prev = vec![0.25f32; len];
    let curr = vec![0.75f32; len];
    let regs = vec![reg(orphan_slot, col_a, kind, THRESH_BUF_VALUES)];
    let events = cpu_oracle_threshold_events(&prev, &curr, &prev, &curr, n_dims, &regs, 7);
    assert!(!events.is_empty());
    assert_eq!(
        reattach_velocity_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator),
        Err(SlotIdentityReattachError::UnadmittedSlot {
            slot: orphan_slot,
            event_kind: kind,
        })
    );
}

#[test]
fn reception_identical_at_both_sites() {
    let arena = arena();
    let d = draft(&arena);

    let closed_loop = mint_attach_overlay_at_barrier(
        &d,
        &arena.allocator,
        simthing_core::GenerationStamp::new(0),
    )
    .unwrap();
    let vendorized = vendorized_attach(&arena, &d);

    // 6.0b reception: both placements' requests arrive through the SAME routed
    // delivery primitive with identical receipts and identical tree state.
    let mut tree_cl = arena.tree.clone();
    let mut tree_ven = arena.tree.clone();
    let receipt_cl = {
        let BoundaryRequest::AttachOverlay {
            target, overlay, ..
        } = closed_loop
        else {
            panic!("AttachOverlay expected");
        };
        deliver_routed_overlay(&mut tree_cl, target, overlay).expect("routed delivery")
    };
    let receipt_ven = {
        let BoundaryRequest::AttachOverlay {
            target, overlay, ..
        } = vendorized
        else {
            panic!("AttachOverlay expected");
        };
        deliver_routed_overlay(&mut tree_ven, target, overlay).expect("routed delivery")
    };
    assert_eq!(receipt_cl, receipt_ven);
    assert_eq!(format!("{tree_cl:?}"), format!("{tree_ven:?}"));
}

#[test]
fn unconverted_semantics_and_default_placement_no_flag_day() {
    let arena = arena();
    let role = SubFieldRole::Amount;
    let (slot_a, col_a) = locus(&arena, arena.a_id, &role);

    // Unconverted arms: the converted doors must skip them IDENTICALLY at both
    // placements (crossing selection is one vocabulary; conversion is
    // per-semantic with no flag day).
    let mut sem = ThresholdRegistry::new();
    let fission_kind = sem.push(ThresholdSemantic::FissionTrigger {
        sim_thing_id: arena.a_id,
        property_id: arena.pid,
        template_idx: 0,
    });
    let expiry_kind = sem.push(ThresholdSemantic::PropertyExpiry {
        sim_thing_id: arena.a_id,
        property_id: arena.pid,
    });
    let scripted_kind = sem.push(ThresholdSemantic::ScriptedEventTrigger {
        event_id: "synthetic".to_string(),
    });

    let regs = vec![
        reg(slot_a, col_a, fission_kind, THRESH_BUF_VALUES),
        reg(slot_a, col_a, expiry_kind, THRESH_BUF_VALUES),
        reg(slot_a, col_a, scripted_kind, THRESH_BUF_VALUES),
    ];
    let events = oracle_crossings(&arena, &regs);

    let vend_v = collect_velocity_alerts_vendorized(&events, &sem);
    let cl_v =
        reattach_velocity_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
            .unwrap();
    let vend_a = collect_aggregate_alerts_vendorized(&events, &sem);
    let cl_a =
        reattach_aggregate_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
            .unwrap();
    assert!(vend_v.is_empty() && cl_v.is_empty() && vend_a.is_empty() && cl_a.is_empty());

    // The closed loop is the DEFAULT placement — on the type and on the
    // boundary protocol — and the vendorized site remains selectable: the
    // split is placement data, not a fork.
    assert_eq!(ResolutionSite::default(), ResolutionSite::ClosedLoop);
    let arena2 = arena;
    let mut proto = BoundaryProtocol::new(
        SimRuntimeTree::admit(arena2.tree),
        arena2.registry,
        arena2.allocator,
    );
    assert_eq!(proto.resolution_site(), ResolutionSite::ClosedLoop);
    proto.set_resolution_site(ResolutionSite::CpuAuthoritative);
    assert_eq!(proto.resolution_site(), ResolutionSite::CpuAuthoritative);
}

#[test]
fn gpu_wire_event_vocabulary_is_slot_space_only() {
    // The GPU wire struct is exactly {slot, col, value, event_kind} — four
    // 4-byte lanes. Identity is not on the wire; it attaches only at the CPU
    // barrier doors. (The WGSL-side grep evidence is recorded in
    // docs/tests/resolution_site_split_0_results.md.)
    assert_eq!(std::mem::size_of::<ThresholdEventGpu>(), 16);
}
