//! End-to-end day boundary integration tests. Requires a GPU adapter;
//! skips cleanly when none is available, matching the convention used in
//! `simthing-gpu` and `simthing-feeder`.
//!
//! Each test runs a real GPU pipeline (Pass 0/1/2/3/7) through a sequence
//! of ticks, captures threshold events, and feeds them into a
//! `BoundaryProtocol::execute` call. We assert on both the CPU side
//! (SimThing tree mutations) and the GPU side (`state.read_values()`).

use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, IntensityBehavior, Overlay,
    OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, PropertyValue,
    SimProperty, SimThing, SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
};
use simthing_feeder::{
    ai_channel, feeder_channel, BoundaryRequest, DispatchCoordinator, FeederWork, TransformPatcher,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};
use simthing_sim::{BoundaryProtocol, VelocityAlertRegistration};

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
        template: FissionTemplate {
            child_kind: SimThingKindTag::Cohort,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient: 0.05,
            resolution_label: "rebellion_settled".into(),
        },
        secondary: None,
    }];
    p
}

/// Tree fixture: World → Location → Cohort(loyalty: Amount=0.5, Velocity=-0.21).
/// At dt=0.5 the velocity carries Amount across the 0.3 threshold on tick 2.
fn build_initial_world(
    reg: &mut DimensionRegistry,
) -> (SimThing, SlotAllocator, simthing_core::SimThingId) {
    let pid = reg.id_of("core", "loyalty").unwrap();

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv = PropertyValue::from_layout(&reg.property(pid).layout);
    let layout = &reg.property(pid).layout;
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    pv.data[amount_off] = 0.5;
    pv.data[vel_off] = -0.21;
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
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    // ── Setup ─────────────────────────────────────────────────────────
    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());

    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let pid = reg.id_of("core", "loyalty").unwrap();
    let layout = reg.property(pid).layout.clone();
    let n_dims = reg.total_columns as u32;

    // Pre-size the GPU + shadow with headroom for fission growth.
    const N_SLOTS: u32 = 16;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    // Project initial tree into shadow rows for slots 0..3.
    let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    let base = cohort_slot * n_dims as usize;
    coord.shadow[base + amount_off] = 0.5;
    coord.shadow[base + vel_off] = -0.21;

    // Move tree + registry + allocator into BoundaryProtocol.
    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    // Initial GPU sync — uploads shadow, overlay deltas, threshold registry.
    proto.initial_gpu_sync(&coord, &mut state);

    // ── Drive ticks until Pass 7 fires the fission threshold ─────────
    let mut events_fired = Vec::new();
    let mut max_ticks = 8;
    while events_fired.is_empty() && max_ticks > 0 {
        let out = coord.tick(
            &rx,
            &mut patcher,
            &proto.registry,
            &proto.allocator,
            &pipelines,
            &mut state,
            0.5,
        );
        if !out.events.is_empty() {
            events_fired = out.events;
            break;
        }
        max_ticks -= 1;
    }
    assert!(!events_fired.is_empty(), "fission threshold never fired");

    // ── Run boundary protocol ─────────────────────────────────────────
    let pre_capacity = proto.allocator.capacity();
    let outcome = proto.execute(events_fired, &mut patcher, &mut coord, &mut state, 1);

    // The event was a FissionTrigger; with no secondary condition it
    // should execute unconditionally.
    assert_eq!(
        outcome.fission.fissions_executed, 1,
        "expected 1 fission, got {:?}",
        outcome.fission
    );
    assert_eq!(outcome.fission.fissions_skipped_secondary, 0);

    // A new SimThing was attached as a child of the rebelling cohort.
    let rebelling = find_node(&proto.root, cohort_id).expect("cohort still in tree");
    assert_eq!(
        rebelling.children.len(),
        1,
        "expected one fission child under the cohort"
    );
    let new_child_id = rebelling.children[0].id;
    assert!(
        proto.allocator.slot_of(new_child_id).is_some(),
        "new child must have a slot"
    );
    assert_eq!(
        proto.allocator.capacity(),
        pre_capacity + 1,
        "allocator capacity should grow by 1"
    );

    // The fission child inherits the activating property from the parent's
    // current GPU row, with Amount reset to 0.0 for the newly-expressing force.
    let child = &rebelling.children[0];
    let child_loyalty = child.property(pid).expect("fission child has loyalty");
    assert_eq!(child_loyalty.data[amount_off], 0.0);
    assert_eq!(child_loyalty.data[vel_off].to_bits(), (-0.21f32).to_bits());
    assert!(
        outcome.gpu_sync.threshold_regs_uploaded >= 2,
        "parent and child should both have fission threshold registrations"
    );

    // ── Day N+1: another tick must run cleanly ────────────────────────
    let next = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.5,
    );
    assert_eq!(
        next.uploaded_rows, 0,
        "no patches were sent; no rows should upload on the next tick"
    );

    // The original cohort's Amount kept integrating: started at <0.3
    // (event fired) and velocity still -0.21 → continues falling.
    let gpu = state.read_values();
    let amount_now = gpu[cohort_slot * n_dims as usize + amount_off];
    assert!(
        amount_now < 0.3,
        "rebelling cohort amount should remain below 0.3, got {amount_now}"
    );
}

/// Boundary requests submitted via the patcher reach the Tree Maintainer
/// and produce real structural mutations: AddChild allocates a slot,
/// Remove tombstones it.
#[test]
fn boundary_requests_apply_structural_mutations() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());
    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let n_dims = reg.total_columns as u32;

    const N_SLOTS: u32 = 16;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (tx, rx) = feeder_channel();

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // Queue: AddChild a fleet under the cohort, then Remove a different
    // child. Since the live tree only has world/location/cohort, the
    // Remove targets the location — which also tombstones cohort.
    let new_fleet = SimThing::new(SimThingKind::Fleet, 1);
    let new_fleet_id = new_fleet.id;
    tx.send(FeederWork::Boundary(BoundaryRequest::AddChild {
        parent: cohort_id,
        child: new_fleet,
    }))
    .unwrap();

    // Run one tick so the patcher parks the request, then immediately
    // boundary-execute (no fission events this time).
    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    let outcome = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);

    assert_eq!(outcome.maintainer.adds, 1);
    assert_eq!(outcome.maintainer.allocated, vec![new_fleet_id]);
    assert!(proto.allocator.slot_of(new_fleet_id).is_some());

    let cohort = find_node(&proto.root, cohort_id).expect("cohort exists");
    assert_eq!(cohort.children.len(), 1);
    assert_eq!(cohort.children[0].id, new_fleet_id);
}

/// AddDimension admits a property registered after GPU state creation,
/// widens the CPU shadow and GPU buffers, and projects any sparse values
/// already present on live SimThings into the new columns.
#[test]
fn add_dimension_request_rebuilds_gpu_layout() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());
    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let old_n_dims = reg.total_columns as u32;

    const N_SLOTS: u32 = 16;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    let mut coord = DispatchCoordinator::new(N_SLOTS, old_n_dims, 1);
    let (tx, rx) = feeder_channel();

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    let food_id = proto
        .registry
        .register(SimProperty::simple("core", "food_security", 0));
    let food_layout = proto.registry.property(food_id).layout.clone();
    let food_amount_off = food_layout.offset_of(&SubFieldRole::Amount).unwrap();
    let mut food_value = PropertyValue::from_layout(&food_layout);
    food_value.data[food_amount_off] = 0.72;
    find_node_mut(&mut proto.root, cohort_id)
        .expect("cohort exists")
        .add_property(food_id, food_value);

    tx.send(FeederWork::Boundary(BoundaryRequest::AddDimension {
        property: food_id,
    }))
    .unwrap();

    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    let outcome = proto.execute(Vec::new(), &mut patcher, &mut coord, &mut state, 1);

    let new_n_dims = proto.registry.total_columns as u32;
    assert!(new_n_dims > old_n_dims);
    assert_eq!(outcome.maintainer.dimensions, 1);
    assert_eq!(outcome.maintainer.deferred, 0);
    assert_eq!(coord.n_dims(), new_n_dims);
    assert_eq!(state.n_dims, new_n_dims);

    let cohort_slot = proto.allocator.slot_of(cohort_id).unwrap() as usize;
    let food_range = proto.registry.column_range(food_id);
    let food_col = food_range.start + food_amount_off;
    let shadow_value = coord.shadow[cohort_slot * new_n_dims as usize + food_col];
    assert_eq!(shadow_value.to_bits(), (0.72f32).to_bits());

    let gpu = state.read_values();
    let gpu_value = gpu[cohort_slot * new_n_dims as usize + food_col];
    assert_eq!(gpu_value.to_bits(), (0.72f32).to_bits());
}

#[test]
fn velocity_alert_registration_surfaces_at_boundary() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    reg.register(loyalty_with_fission());
    let (world, alloc, cohort_id) = build_initial_world(&mut reg);
    let pid = reg.id_of("core", "loyalty").unwrap();
    let layout = reg.property(pid).layout.clone();
    let n_dims = reg.total_columns as u32;

    const N_SLOTS: u32 = 16;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
    let base = cohort_slot * n_dims as usize;
    coord.shadow[base + amount_off] = 0.5;
    coord.shadow[base + vel_off] = 0.0;

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    find_node_mut(&mut proto.root, cohort_id)
        .expect("cohort exists")
        .add_overlay(Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Ai,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Velocity, TransformOp::Add(-0.21))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        });
    proto.register_velocity_alert(VelocityAlertRegistration {
        sim_thing_id: cohort_id,
        property_id: pid,
        sub_field: SubFieldRole::Velocity,
        threshold: -0.10,
        direction: Direction::Falling,
    });
    proto.initial_gpu_sync(&coord, &mut state);

    let out = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.0,
    );
    assert!(
        out.events
            .iter()
            .any(|event| event.value.to_bits() == (-0.21f32).to_bits()),
        "velocity alert threshold never fired"
    );

    let boundary = proto.execute(out.events, &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(boundary.velocity_alerts.len(), 1);
    let alert = &boundary.velocity_alerts[0];
    assert_eq!(alert.sim_thing_id, cohort_id);
    assert_eq!(alert.property_id, pid);
    assert_eq!(alert.sub_field, SubFieldRole::Velocity);
    assert_eq!(alert.value.to_bits(), (-0.21f32).to_bits());
}

/// A player submits a `PlayerIntentOverlay` via `FeederSender::submit_player_intent`.
/// The patcher parks it; `BoundaryProtocol::execute` attaches it to the target
/// node at boundary time.
#[test]
fn player_intent_overlay_arrives_attached_at_boundary() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    // ── Setup ─────────────────────────────────────────────────────────
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "loyalty", 0));
    let pid = reg.id_of("core", "loyalty").unwrap();
    let n_dims = reg.total_columns as u32;

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let cohort_id = cohort.id;
    cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(cohort);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);

    const N_SLOTS: u32 = 8;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    // ticks_per_day=1 so the very first tick signals boundary_reached.
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (tx, rx) = feeder_channel();

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // ── Player submits an overlay before the boundary tick ────────────
    let intent_overlay = Overlay {
        id:        OverlayId::new(),
        kind:      OverlayKind::Policy,
        source:    OverlaySource::Player,
        affects:   vec![cohort_id],
        transform: PropertyTransformDelta {
            property_id:      pid,
            sub_field_deltas: vec![(SubFieldRole::Velocity, TransformOp::Add(-0.1))],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    let overlay_id = intent_overlay.id;
    tx.submit_player_intent(cohort_id, intent_overlay).unwrap();

    // ── Single tick (boundary_reached = true at tick 1 of 1) ─────────
    let tick_out = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        1.0,
    );
    assert!(tick_out.boundary_reached, "expected boundary on tick 1 of 1");

    // ── Boundary: player intent should be attached to cohort ──────────
    let outcome = proto.execute(tick_out.events, &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(outcome.player_intents_attached, 1);

    let cohort_node = find_node(&proto.root, cohort_id).expect("cohort in tree");
    assert!(
        cohort_node.overlays.iter().any(|o| o.id == overlay_id),
        "player intent overlay must be attached to the cohort"
    );
}

/// Player intent transform effect is visible on the GPU within the same tick
/// it is submitted (mid-day fast path), and the overlay is structurally
/// attached to the tree after the subsequent boundary.
#[test]
fn player_intent_mid_day_effect_lands_on_gpu_before_boundary() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    // ── Setup: cohort with loyalty, Amount starts at 0.0 ─────────────
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "loyalty", 0));
    let pid = reg.id_of("core", "loyalty").unwrap();
    let layout = reg.property(pid).layout.clone();
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let n_dims = reg.total_columns as u32;

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let cohort_id = cohort.id;
    cohort.add_property(pid, PropertyValue::from_layout(&layout));
    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(cohort);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;

    const N_SLOTS: u32 = 8;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    // ticks_per_day=2 so tick 1 is mid-day and tick 2 is the boundary.
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 2);
    let (tx, rx) = feeder_channel();

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // ── Submit player intent: Set Amount = 0.6 ────────────────────────
    let intent_overlay = Overlay {
        id:        OverlayId::new(),
        kind:      OverlayKind::Policy,
        source:    OverlaySource::Player,
        affects:   vec![cohort_id],
        transform: PropertyTransformDelta {
            property_id:      pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.6))],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    let overlay_id = intent_overlay.id;
    tx.submit_player_intent(cohort_id, intent_overlay).unwrap();

    // ── Tick 1: mid-day, boundary NOT yet reached ─────────────────────
    let tick1 = coord.tick(
        &rx, &mut patcher, &proto.registry, &proto.allocator, &pipelines, &mut state, 1.0,
    );
    assert!(!tick1.boundary_reached, "tick 1 of 2 must not signal boundary");

    // The transform was applied to the shadow during drain and uploaded to
    // the GPU as a dirty row. Read back and confirm Amount = 0.6.
    let gpu_values = state.read_values();
    let base = cohort_slot * n_dims as usize;
    assert_eq!(
        gpu_values[base + amount_off].to_bits(),
        0.6f32.to_bits(),
        "player intent Set(0.6) must be visible on GPU after tick 1"
    );
    // Overlay not yet structurally attached — boundary hasn't run.
    let cohort_node = find_node(&proto.root, cohort_id).unwrap();
    assert!(
        cohort_node.overlays.iter().all(|o| o.id != overlay_id),
        "overlay must not be in tree before boundary"
    );

    // ── Tick 2: boundary reached ──────────────────────────────────────
    let tick2 = coord.tick(
        &rx, &mut patcher, &proto.registry, &proto.allocator, &pipelines, &mut state, 1.0,
    );
    assert!(tick2.boundary_reached, "tick 2 of 2 must signal boundary");

    let outcome = proto.execute(tick2.events, &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(outcome.player_intents_attached, 1);

    // Now structurally attached.
    let cohort_node = find_node(&proto.root, cohort_id).unwrap();
    assert!(
        cohort_node.overlays.iter().any(|o| o.id == overlay_id),
        "overlay must be attached after boundary"
    );
}

/// AI submits an intent overlay through the dedicated AI channel. The
/// transform delta is visible on the GPU within the same tick; the overlay is
/// structurally attached to the tree after the boundary. The urgency value
/// survives the round-trip.
#[test]
fn ai_intent_mid_day_effect_and_boundary_attach() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    // ── Setup: cohort with loyalty, Amount starts at 0.0 ─────────────
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "loyalty", 0));
    let pid = reg.id_of("core", "loyalty").unwrap();
    let layout = reg.property(pid).layout.clone();
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let n_dims = reg.total_columns as u32;

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    let cohort_id = cohort.id;
    cohort.add_property(pid, PropertyValue::from_layout(&layout));
    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(cohort);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let cohort_slot = alloc.slot_of(cohort_id).unwrap() as usize;

    const N_SLOTS: u32 = 8;
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    // ticks_per_day=2: tick 1 is mid-day, tick 2 triggers boundary.
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 2);
    let (_tx, rx) = feeder_channel();

    // Connect the AI channel to the patcher.
    let (ai_tx, ai_rx) = ai_channel();
    patcher.set_ai_receiver(ai_rx);

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // ── AI submits intent: Set Amount = 0.8, urgency = 0.95 ──────────
    let ai_overlay = Overlay {
        id:        OverlayId::new(),
        kind:      OverlayKind::Policy,
        source:    OverlaySource::Ai,
        affects:   vec![cohort_id],
        transform: PropertyTransformDelta {
            property_id:      pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.8))],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    let overlay_id = ai_overlay.id;
    ai_tx.submit_ai_intent(cohort_id, ai_overlay, 0.95).unwrap();

    // ── Tick 1: mid-day ───────────────────────────────────────────────
    let tick1 = coord.tick(&rx, &mut patcher, &proto.registry, &proto.allocator, &pipelines, &mut state, 1.0);
    assert!(!tick1.boundary_reached);

    // Transform visible on GPU already.
    let gpu_values = state.read_values();
    let base = cohort_slot * n_dims as usize;
    assert_eq!(
        gpu_values[base + amount_off].to_bits(),
        0.8f32.to_bits(),
        "AI intent Set(0.8) must reach GPU within the same tick"
    );
    // Not yet in tree.
    assert!(find_node(&proto.root, cohort_id).unwrap().overlays.iter().all(|o| o.id != overlay_id));

    // ── Tick 2: boundary ─────────────────────────────────────────────
    let tick2 = coord.tick(&rx, &mut patcher, &proto.registry, &proto.allocator, &pipelines, &mut state, 1.0);
    assert!(tick2.boundary_reached);

    let outcome = proto.execute(tick2.events, &mut patcher, &mut coord, &mut state, 1);
    assert_eq!(outcome.ai_intents_attached, 1);
    assert_eq!(outcome.player_intents_attached, 0);

    // Overlay structurally attached.
    assert!(
        find_node(&proto.root, cohort_id).unwrap().overlays.iter().any(|o| o.id == overlay_id),
        "AI intent overlay must be in tree after boundary"
    );
}

/// After `initial_gpu_sync` + one tick, the GPU `output_vectors` buffer must
/// reflect the bottom-up reduction over the tree: leaves carry their post-Pass-3
/// values, inner nodes carry per-column reductions of their children.
///
/// Tree: World → Location → 2 Cohorts. Amount uses Mean, Intensity uses Max
/// (the role defaults).
#[test]
fn reduction_pipeline_produces_aggregated_output_vectors() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let mut reg = DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let layout = reg.property(pid).layout.clone();
    let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

    // Build tree: World → Location → 2 Cohorts (no velocity to avoid drift).
    let mut c1 = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv1 = PropertyValue::from_layout(&layout);
    pv1.data[a_off] = 0.40;
    pv1.data[i_off] = 0.20;
    c1.add_property(pid, pv1);
    let c1_id = c1.id;

    let mut c2 = SimThing::new(SimThingKind::Cohort, 0);
    let mut pv2 = PropertyValue::from_layout(&layout);
    pv2.data[a_off] = 0.60;
    pv2.data[i_off] = 0.80;
    c2.add_property(pid, pv2);
    let c2_id = c2.id;

    let mut loc = SimThing::new(SimThingKind::Location, 0);
    let loc_id = loc.id;
    loc.add_child(c1);
    loc.add_child(c2);
    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(loc);

    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);

    const N_SLOTS: u32 = 8;
    let n_dims = reg.total_columns as u32;

    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(N_SLOTS as usize);
    let mut coord = DispatchCoordinator::new(N_SLOTS, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    // Seed shadow with cohort values; inner-node rows stay zero (reduction fills them).
    let n_dims_us = n_dims as usize;
    let c1_slot = alloc.slot_of(c1_id).unwrap() as usize;
    let c2_slot = alloc.slot_of(c2_id).unwrap() as usize;
    coord.shadow[c1_slot * n_dims_us + a_off] = 0.40;
    coord.shadow[c1_slot * n_dims_us + i_off] = 0.20;
    coord.shadow[c2_slot * n_dims_us + a_off] = 0.60;
    coord.shadow[c2_slot * n_dims_us + i_off] = 0.80;

    let mut proto = BoundaryProtocol::new(world, reg, alloc);
    proto.initial_gpu_sync(&coord, &mut state);

    // One tick — exercises the full pipeline including Passes 4–6.
    let _ = coord.tick(
        &rx,
        &mut patcher,
        &proto.registry,
        &proto.allocator,
        &pipelines,
        &mut state,
        0.5,
    );

    let out = state.read_output_vectors();
    let loc_slot = proto.allocator.slot_of(loc_id).unwrap() as usize;

    // Mean of (0.40, 0.60) = 0.50 — Amount uses Mean by role default.
    assert_eq!(
        out[loc_slot * n_dims_us + a_off].to_bits(),
        0.50_f32.to_bits(),
        "location amount must be mean of cohorts",
    );
    // Max of (0.20, 0.80) = 0.80 — Intensity uses Max by role default.
    assert_eq!(
        out[loc_slot * n_dims_us + i_off].to_bits(),
        0.80_f32.to_bits(),
        "location intensity must be max of cohorts",
    );

    // BoundaryProtocol::read_reduced_field returns the same data wrapped in
    // a presentation-friendly accessor.
    let field = proto.read_reduced_field(&state);
    let loc_loyalty = field
        .property_value(loc_slot as u32, &proto.registry, pid)
        .expect("reduced loyalty for location");
    assert_eq!(loc_loyalty.data[a_off].to_bits(), 0.50_f32.to_bits());
    assert_eq!(loc_loyalty.data[i_off].to_bits(), 0.80_f32.to_bits());

    // Leaves: output mirrors values bit-exactly.
    let vals = state.read_values();
    for slot in [c1_slot, c2_slot] {
        for col in 0..n_dims_us {
            assert_eq!(
                out[slot * n_dims_us + col].to_bits(),
                vals[slot * n_dims_us + col].to_bits(),
                "leaf slot {slot} col {col}",
            );
        }
    }
}

/// Helper: depth-first find a node by id.
fn find_node(node: &SimThing, id: simthing_core::SimThingId) -> Option<&SimThing> {
    if node.id == id {
        return Some(node);
    }
    for c in &node.children {
        if let Some(n) = find_node(c, id) {
            return Some(n);
        }
    }
    None
}

fn find_node_mut(node: &mut SimThing, id: simthing_core::SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    for c in &mut node.children {
        if let Some(n) = find_node_mut(c, id) {
            return Some(n);
        }
    }
    None
}
