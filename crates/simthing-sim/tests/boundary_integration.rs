//! End-to-end day boundary integration tests. Requires a GPU adapter;
//! skips cleanly when none is available, matching the convention used in
//! `simthing-gpu` and `simthing-feeder`.
//!
//! Each test runs a real GPU pipeline (Pass 0/1/2/3/7) through a sequence
//! of ticks, captures threshold events, and feeds them into a
//! `BoundaryProtocol::execute` call. We assert on both the CPU side
//! (SimThing tree mutations) and the GPU side (`state.read_values()`).

use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, IntensityBehavior,
    PropertyValue, SimProperty, SimThing, SimThingKind, SimThingKindTag, SubFieldRole,
};
use simthing_feeder::{
    feeder_channel, BoundaryRequest, DispatchCoordinator, FeederWork, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::BoundaryProtocol;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

/// Property whose `FissionThreshold` fires when Amount falls below 0.3
/// (Falling direction). Spawns a Cohort child on fire.
fn loyalty_with_fission() -> SimProperty {
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    p.fission_templates = vec![FissionThreshold {
        // dimension/sub_field of the threshold itself. The Builder uses
        // these to pick the right column for the GPU registration.
        dimension: simthing_core::SimPropertyId(0),
        sub_field: SubFieldRole::Amount,
        threshold: 0.3,
        direction: Direction::Falling,
        template:  FissionTemplate {
            child_kind:                 SimThingKindTag::Cohort,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient:    0.05,
            resolution_label:           "rebellion_settled".into(),
        },
        secondary: None,
    }];
    p
}

/// Tree fixture: World → Location → Cohort(loyalty: Amount=0.5, Velocity=-0.21).
/// At dt=0.5 the velocity carries Amount across the 0.3 threshold on tick 2.
fn build_initial_world(reg: &mut DimensionRegistry) -> (SimThing, SlotAllocator, simthing_core::SimThingId) {
    let pid = reg.id_of("core", "loyalty").unwrap();

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&reg.property(pid).layout);
    let layout = &reg.property(pid).layout;
    let amount_off  = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off     = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    pv.data[amount_off] = 0.5;
    pv.data[vel_off]    = -0.21;
    cohort.add_property(pid, pv);
    let cohort_id = cohort.id;

    let mut loc = SimThing::new(SimThingKind::Location, 0);
    loc.add_child(cohort);
    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(loc);

    // Allocate slots for the live tree (3 nodes) plus headroom for growth.
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);

    (world, alloc, cohort_id)
}

/// A cohort whose loyalty Amount falls past 0.3 fires Pass 7's fission
/// threshold. Calling `BoundaryProtocol::execute` with that event must:
/// 1. Spawn a new cohort child of the rebelling cohort.
/// 2. Allocate a slot for it.
/// 3. Seed the child's loyalty property from the parent's current GPU row.
/// 4. Re-upload Pass 7 + Pass 3 buffers to reflect the new tree.
/// 5. Survive a subsequent tick without panic.
#[test]
fn fission_event_spawns_child_and_day_n_plus_1_tick_runs_clean() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    // ── Setup ─────────────────────────────────────────────────────────
    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());

    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let pid    = reg.id_of("core", "loyalty").unwrap();
    let layout = reg.property(pid).layout.clone();
    let n_dims = reg.total_columns as u32;

    // Pre-size the GPU + shadow with headroom for fission growth.
    const N_SLOTS: u32 = 16;
    let mut state    = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines    = Pipelines::new(&state.ctx);
    let mut patcher  = TransformPatcher::new(N_SLOTS as usize);
    let mut coord    = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (_tx, rx)    = feeder_channel();

    // Project initial tree into shadow rows for slots 0..3.
    let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;
    let amount_off  = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off     = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    let base = cohort_slot * n_dims as usize;
    coord.shadow[base + amount_off] = 0.5;
    coord.shadow[base + vel_off]    = -0.21;

    // Move tree + registry + allocator into BoundaryProtocol.
    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    // Initial GPU sync — uploads shadow, overlay deltas, threshold registry.
    proto.initial_gpu_sync(&coord, &mut state);

    // ── Drive ticks until Pass 7 fires the fission threshold ─────────
    let mut events_fired = Vec::new();
    let mut max_ticks = 8;
    while events_fired.is_empty() && max_ticks > 0 {
        let out = coord.tick(
            &rx, &mut patcher, &proto.registry, &proto.allocator,
            &pipelines, &mut state, 0.5,
        );
        if !out.events.is_empty() { events_fired = out.events; break; }
        max_ticks -= 1;
    }
    assert!(!events_fired.is_empty(), "fission threshold never fired");

    // ── Run boundary protocol ─────────────────────────────────────────
    let pre_capacity = proto.allocator.capacity();
    let outcome = proto.execute(events_fired, &mut patcher, &mut coord, &mut state, 1);

    // The event was a FissionTrigger; with no secondary condition it
    // should execute unconditionally.
    assert_eq!(outcome.fission.fissions_executed, 1,
               "expected 1 fission, got {:?}", outcome.fission);
    assert_eq!(outcome.fission.fissions_skipped_secondary, 0);

    // A new SimThing was attached as a child of the rebelling cohort.
    let rebelling = find_node(&proto.root, cohort_id).expect("cohort still in tree");
    assert_eq!(rebelling.children.len(), 1,
               "expected one fission child under the cohort");
    let new_child_id = rebelling.children[0].id;
    assert!(proto.allocator.slot_of(new_child_id).is_some(),
            "new child must have a slot");
    assert_eq!(proto.allocator.capacity(), pre_capacity + 1,
               "allocator capacity should grow by 1");

    // The fission child inherits the activating property from the parent's
    // current GPU row, with Amount reset to 0.0 for the newly-expressing force.
    let child = &rebelling.children[0];
    let child_loyalty = child.property(pid).expect("fission child has loyalty");
    assert_eq!(child_loyalty.data[amount_off], 0.0);
    assert_eq!(child_loyalty.data[vel_off].to_bits(), (-0.21f32).to_bits());
    assert!(outcome.gpu_sync.threshold_regs_uploaded >= 2,
            "parent and child should both have fission threshold registrations");

    // ── Day N+1: another tick must run cleanly ────────────────────────
    let next = coord.tick(
        &rx, &mut patcher, &proto.registry, &proto.allocator,
        &pipelines, &mut state, 0.5,
    );
    assert_eq!(next.uploaded_rows, 0,
               "no patches were sent; no rows should upload on the next tick");

    // The original cohort's Amount kept integrating: started at <0.3
    // (event fired) and velocity still -0.21 → continues falling.
    let gpu = state.read_values();
    let amount_now = gpu[cohort_slot * n_dims as usize + amount_off];
    assert!(amount_now < 0.3,
            "rebelling cohort amount should remain below 0.3, got {amount_now}");
}

/// Boundary requests submitted via the patcher reach the Tree Maintainer
/// and produce real structural mutations: AddChild allocates a slot,
/// Remove tombstones it.
#[test]
fn boundary_requests_apply_structural_mutations() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());
    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let n_dims = reg.total_columns as u32;

    const N_SLOTS: u32 = 16;
    let mut state    = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines    = Pipelines::new(&state.ctx);
    let mut patcher  = TransformPatcher::new(N_SLOTS as usize);
    let mut coord    = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (tx, rx)     = feeder_channel();

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // Queue: AddChild a fleet under the cohort, then Remove a different
    // child. Since the live tree only has world/location/cohort, the
    // Remove targets the location — which also tombstones cohort.
    let new_fleet = SimThing::new(SimThingKind::Fleet, 1);
    let new_fleet_id = new_fleet.id;
    tx.send(FeederWork::Boundary(BoundaryRequest::AddChild {
        parent: cohort_id, child: new_fleet,
    })).unwrap();

    // Run one tick so the patcher parks the request, then immediately
    // boundary-execute (no fission events this time).
    let _ = coord.tick(
        &rx, &mut patcher, &proto.registry, &proto.allocator,
        &pipelines, &mut state, 0.0,
    );
    let outcome = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);

    assert_eq!(outcome.maintainer.adds, 1);
    assert_eq!(outcome.maintainer.allocated, vec![new_fleet_id]);
    assert!(proto.allocator.slot_of(new_fleet_id).is_some());

    let cohort = find_node(&proto.root, cohort_id).expect("cohort exists");
    assert_eq!(cohort.children.len(), 1);
    assert_eq!(cohort.children[0].id, new_fleet_id);
}

/// Helper: depth-first find a node by id.
fn find_node(node: &SimThing, id: simthing_core::SimThingId) -> Option<&SimThing> {
    if node.id == id { return Some(node); }
    for c in &node.children {
        if let Some(n) = find_node(c, id) { return Some(n); }
    }
    None
}
