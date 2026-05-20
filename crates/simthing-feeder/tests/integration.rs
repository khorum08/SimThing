//! End-to-end integration tests for the feeder crate. Requires a GPU
//! adapter; skips cleanly when none is available, matching the convention
//! used in `simthing-gpu`'s test suite.

use simthing_core::{
    DimensionRegistry, IntensityBehavior, PropertyTransformDelta, SimProperty, SimThing,
    SimThingKind, SubFieldRole, TransformOp,
};
use simthing_feeder::{
    feeder_channel, BoundaryRequest, DispatchCoordinator, FeederWork, PatchTransform,
    TransformPatcher, TreeMaintainer,
};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn loyalty_property() -> SimProperty {
    let mut p = SimProperty::simple("core", "loyalty", 0);
    p.intensity_behavior = Some(IntensityBehavior::default());
    p
}

/// Build a fresh registry + 2-slot allocator with two cohort SimThings,
/// loyalty property registered.
fn fixture() -> (
    DimensionRegistry,
    SlotAllocator,
    simthing_core::SimPropertyId,
    [simthing_core::SimThingId; 2],
) {
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(loyalty_property());
    let mut alloc = SlotAllocator::new();
    let a = SimThing::new(SimThingKind::Cohort, 0).id;
    let b = SimThing::new(SimThingKind::Cohort, 0).id;
    alloc.alloc(a);
    alloc.alloc(b);
    (reg, alloc, pid, [a, b])
}

/// A patch sent through the channel reaches the GPU `values` buffer after
/// one tick — proving the full Sender → Patcher → shadow → dirty-row
/// upload → Pass 0/1/2/3/7 → readback chain works end-to-end.
#[test]
fn patch_through_channel_lands_on_gpu_after_one_tick() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    let (reg, alloc, pid, [a, _b]) = fixture();
    let n_dims = reg.total_columns as u32;

    let mut state    = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines    = Pipelines::new(&state.ctx);
    let mut patcher  = TransformPatcher::new(alloc.capacity());
    let mut coord    = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 4);

    let (tx, rx) = feeder_channel();

    // Drop an Amount → 0.75 set on slot 0 onto the channel.
    tx.send(FeederWork::Patch(PatchTransform {
        target: a,
        delta:  PropertyTransformDelta {
            property_id:      pid,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.75))],
        },
    })).unwrap();

    // Run one tick. dt small so velocity integration is a no-op
    // (initial velocity = 0).
    let outcome = coord.tick(
        &rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0,
    );

    assert_eq!(outcome.patcher_stats.applied_writes, 1);
    assert_eq!(outcome.uploaded_rows, 1);
    assert_eq!(outcome.tick_index, 1);
    assert_eq!(outcome.day_index,  0);
    assert!(!outcome.boundary_reached);

    // GPU `values`: slot 0, col 0 (Amount) = 0.75. Slot 1 untouched.
    let values = state.read_values();
    assert_eq!(values[0], 0.75);
    // Slot 1 row is all zero (or whatever was set; here, zero-init).
    let n = n_dims as usize;
    assert!(values[n..n * 2].iter().all(|v| *v == 0.0));
}

/// Four ticks at `ticks_per_day = 4` produce exactly one boundary signal,
/// and the day counter advances. Verifies the bookkeeping in tick().
#[test]
fn day_boundary_fires_on_ticks_per_day() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    let (reg, alloc, _pid, _) = fixture();
    let n_dims    = reg.total_columns as u32;
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(alloc.capacity());
    let mut coord   = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 4);
    let (_tx, rx)   = feeder_channel();

    for i in 1..=4 {
        let out = coord.tick(
            &rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0,
        );
        assert_eq!(out.tick_index, i);
        if i < 4 {
            assert!(!out.boundary_reached, "tick {i} should not signal boundary");
            assert_eq!(out.day_index, 0);
        } else {
            assert!(out.boundary_reached, "tick 4 must signal boundary");
            assert_eq!(out.day_index, 1);
        }
    }
}

/// Boundary requests submitted via the channel survive a tick and reach
/// the Tree Maintainer at boundary time, in arrival order.
#[test]
fn boundary_requests_reach_tree_maintainer() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    let (reg, alloc, _pid, [a, b]) = fixture();
    let n_dims    = reg.total_columns as u32;
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher    = TransformPatcher::new(alloc.capacity());
    let mut coord      = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 1);
    let mut maintainer = TreeMaintainer::new();
    let (tx, rx) = feeder_channel();

    tx.send(FeederWork::Boundary(BoundaryRequest::Remove { target: a })).unwrap();
    tx.send(FeederWork::Boundary(BoundaryRequest::Remove { target: b })).unwrap();

    let out = coord.tick(
        &rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0,
    );
    assert!(out.boundary_reached, "ticks_per_day=1 → every tick is a boundary");
    assert_eq!(out.patcher_stats.boundary_parked, 2);

    let parked = patcher.take_boundary_requests();
    assert_eq!(parked.len(), 2);
    let outcome = maintainer.execute(parked);
    assert_eq!(outcome.removes,  2);
    assert_eq!(outcome.deferred, 2);
}

/// Patches that hit the same row in the same tick coalesce into a single
/// dirty-row upload. Verifies the bandwidth optimization in `take_dirty_rows`.
#[test]
fn many_patches_same_row_coalesce_to_one_upload() {
    let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

    let (reg, alloc, pid, [a, _b]) = fixture();
    let n_dims    = reg.total_columns as u32;
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(alloc.capacity());
    let mut coord   = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 8);
    let (tx, rx)    = feeder_channel();

    let mk = |op: TransformOp| PropertyTransformDelta {
        property_id:      pid,
        sub_field_deltas: vec![(SubFieldRole::Amount, op)],
    };
    for i in 1..=10 {
        tx.send(FeederWork::Patch(PatchTransform {
            target: a,
            delta: mk(TransformOp::Set(i as f32 * 0.01)),
        })).unwrap();
    }

    let out = coord.tick(
        &rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0,
    );

    // 10 writes, but 1 row upload because they all hit slot 0.
    assert_eq!(out.patcher_stats.applied_writes, 10);
    assert_eq!(out.uploaded_rows, 1);

    // Last Set wins on the GPU.
    let values = state.read_values();
    let v = values[0];
    assert!(
        (v - 0.10).abs() < 1e-5,
        "expected ≈0.10 at slot 0, col 0; got {v}",
    );
}
