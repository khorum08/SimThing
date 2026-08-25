//! CONTINUOUS-POSTURE-SOAK-0 — dual ResolutionSite forced-lag soak.
//!
//! Identical seeds → bit-identical BoundaryRequest streams in BOTH placements
//! over N generations. A planted mode-divergence mutant REDs. Synthetic only.

use simthing_core::{
    DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SubFieldRole, TransformOp,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    cpu_oracle_threshold_events, SlotAllocator, ThresholdEvent, ThresholdRegistration, DIR_UPWARD,
    THRESH_BUF_VALUES,
};
use simthing_sim::{
    collect_velocity_alerts_vendorized, mint_attach_overlay_at_barrier,
    reattach_velocity_alerts_at_barrier, ResolutionSite, SlotSpaceOverlayDraft, ThresholdRegistry,
    ThresholdSemantic,
};

struct Arena {
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
    let _ = root;
    Arena {
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

fn oracle_crossings(
    arena: &Arena,
    regs: &[ThresholdRegistration],
    salt: u32,
) -> Vec<ThresholdEvent> {
    let n_dims = arena.registry.total_columns as u32;
    let n_slots = arena.allocator.capacity() as u32;
    let len = (n_slots * n_dims) as usize;
    let mut prev = vec![0.25f32; len];
    let mut curr = vec![0.75f32; len];
    for (i, reg) in regs.iter().enumerate() {
        let addr = (reg.slot * n_dims + reg.col) as usize;
        prev[addr] = 0.25 + i as f32 * 0.01 + salt as f32 * 0.001;
        curr[addr] = 0.75 + i as f32 * 0.01 + salt as f32 * 0.001;
    }
    cpu_oracle_threshold_events(&prev, &curr, &prev.clone(), &curr.clone(), n_dims, regs, 7)
}

fn reg(slot: u32, col: u32, event_kind: u32) -> ThresholdRegistration {
    ThresholdRegistration {
        slot,
        col,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind,
        buffer: THRESH_BUF_VALUES,
    }
}

fn attach_draft(arena: &Arena, amount: f32) -> SlotSpaceOverlayDraft {
    SlotSpaceOverlayDraft {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Event,
        origin_slot: arena.allocator.slot_of(arena.a_id).unwrap(),
        target_slot: arena.allocator.slot_of(arena.b_id).unwrap(),
        transform: PropertyTransformDelta {
            property_id: arena.pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(amount))],
        },
        lifecycle: OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
        },
    }
}

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
fn n_generation_forced_lag_dual_site_boundary_streams_bit_identical_and_mode_divergence_reds() {
    const N: u32 = 16;
    let arena = arena();
    assert_eq!(ResolutionSite::default(), ResolutionSite::ClosedLoop);

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
    let regs = vec![reg(slot_a, col_a, kind_a), reg(slot_b, col_b, kind_b)];

    let mut closed_streams = Vec::new();
    let mut cpu_streams = Vec::new();

    for gen in 0..N {
        // Forced lag salt: identical seed for both placements each generation.
        let events = oracle_crossings(&arena, &regs, gen);
        let closed =
            reattach_velocity_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
                .expect("closed-loop total");
        let cpu = collect_velocity_alerts_vendorized(&events, &sem);
        assert_eq!(closed, cpu);
        assert_eq!(format!("{closed:?}"), format!("{cpu:?}"));
        for (a, b) in closed.iter().zip(&cpu) {
            assert_eq!(a.value.to_bits(), b.value.to_bits());
        }
        closed_streams.push(closed);
        cpu_streams.push(cpu);

        // AttachOverlay stream parity under the same generation salt.
        let amount = 0.1 + gen as f32 * 0.01;
        let d = attach_draft(&arena, amount);
        let cl_attach = mint_attach_overlay_at_barrier(
            &d,
            &arena.allocator,
            simthing_core::GenerationStamp::new(0),
        )
        .expect("mint");
        let cpu_attach = vendorized_attach(&arena, &d);
        assert_eq!(
            format!("{cl_attach:?}"),
            format!("{cpu_attach:?}"),
            "dual-site AttachOverlay streams must stay bit-identical across soak gens"
        );
    }

    assert_eq!(closed_streams, cpu_streams);
    assert_eq!(closed_streams.len(), N as usize);

    // Planted mode-divergence mutant: drifted mirror registry breaks parity.
    let mut drifted = ThresholdRegistry::new();
    let _ = drifted.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: SimThingId::new(),
        property_id: arena.pid,
        sub_field: role.clone(),
    });
    let _ = drifted.push(ThresholdSemantic::VelocityAlert {
        sim_thing_id: arena.b_id,
        property_id: arena.pid,
        sub_field: role,
    });
    let events = oracle_crossings(&arena, &regs, 0);
    let closed =
        reattach_velocity_alerts_at_barrier(&events, &sem, &arena.registry, &arena.allocator)
            .unwrap();
    let mutant = collect_velocity_alerts_vendorized(&events, &drifted);
    assert_ne!(
        closed, mutant,
        "mode-divergence / mirror-drift mutant must RED dual-site soak parity"
    );
}
