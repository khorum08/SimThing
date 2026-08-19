//! CONTINUOUS-POSTURE-SOAK-0 Remand 2 — derived staleness on the real STEAD values plane.
//!
//! Biting production-path proof: magnitude is installed into
//! `WorldGpuState.resolved.values` and read back through `read_values_row`.
//! Substituting a parallel CPU vector on `AsyncStalenessColumn` cannot satisfy
//! this referee.

use std::collections::BTreeMap;
use std::sync::Mutex;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{DimensionRegistry, GenerationStamp, SimThing, SimThingKind, SlotIndex};
use simthing_gpu::{set_debug_readback_allowed, GpuContext, WorldGpuState};
use simthing_spec::{
    derive_staleness_f32, reduce_owner_channel_rf, AsyncStalenessColumn, AuthoredStalenessHorizon,
    OwnerChannelRfOwnAggregate, ResourceKey,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("stead-lane".into()), 0)
}

fn own(id: simthing_core::SimThingId, surplus: u32, deficit: u32) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id: id,
        resource_key: ResourceKey::new("r0"),
        surplus,
        deficit,
    }
}

fn crossing_tree() -> (
    SimThing,
    Vec<OwnerChannelRfOwnAggregate>,
    BTreeMap<simthing_core::SimThingId, SlotIndex>,
) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("owner-0"));
    let mut child = node();
    bind_owner(&mut child, &OwnerRef::new("owner-1"));
    let mut leaf = node();
    bind_owner(&mut leaf, &OwnerRef::new("owner-1"));
    child.add_child(leaf);
    root.add_child(child);

    let mut ids = Vec::new();
    fn collect(node: &SimThing, ids: &mut Vec<simthing_core::SimThingId>) {
        ids.push(node.id);
        for c in &node.children {
            collect(c, ids);
        }
    }
    collect(&root, &mut ids);
    let rows: Vec<_> = ids.iter().map(|&id| own(id, 2, 1)).collect();
    let mut slots = BTreeMap::new();
    for (i, id) in ids.into_iter().enumerate() {
        slots.insert(id, SlotIndex::new(i as u32));
    }
    (root, rows, slots)
}

#[test]
fn derived_staleness_reads_from_world_gpu_stead_values_plane() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    set_debug_readback_allowed(true);

    let (root, rows, slots) = crossing_tree();
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).expect("reduce");
    let seeds = AsyncStalenessColumn::seeds_from_crossings(&stamped.product().stead.crossing_flows);
    assert!(
        !seeds.is_empty(),
        "fixture must retain at least one ownership crossing"
    );

    let mut registry = DimensionRegistry::new();
    // Pre-existing STEAD width so staleness is not a solo private plane.
    registry.register(simthing_core::SimProperty::simple("session", "boot", 1));
    let n_slots = slots.len();
    let mut column = AsyncStalenessColumn::admit(
        &mut registry,
        n_slots,
        seeds.clone(),
        AuthoredStalenessHorizon::new(1),
    )
    .expect("admit derived STEAD lane");
    assert!(
        column.n_dims() >= 2,
        "staleness lane shares the world n_dims"
    );
    assert_eq!(column.n_dims(), registry.total_columns as usize);

    let mut stead = vec![0.0; n_slots * column.n_dims()];
    let parent = GenerationStamp::new(10);
    let mut latest = BTreeMap::new();
    for &seed in &seeds {
        latest.insert(seed, GenerationStamp::new(7));
    }
    let visits = column
        .sweep_seeded(&mut stead, &root, &slots, parent, &latest)
        .expect("seeded sweep into STEAD plane");
    assert!(visits > 0);

    let state = WorldGpuState::new(ctx, &registry, n_slots as u32);
    assert_eq!(state.n_dims as usize, column.n_dims());
    assert_eq!(stead.len(), state.values_len());
    state.install_resolved_values_at_boundary(&stead);

    let seed_slot = *slots.get(&seeds[0]).expect("seed slotted");
    let expected = derive_staleness_f32(parent, GenerationStamp::new(7));
    let row = state.read_values_row(seed_slot.raw());
    let gpu_lane = row[column.col().raw()];
    assert_eq!(
        gpu_lane.to_bits(),
        expected.to_bits(),
        "staleness must be readable from WorldGpuState.resolved.values \
         (parallel AsyncStalenessColumn Vec cannot satisfy this door)"
    );
    assert_eq!(
        column
            .value_at(&stead, seed_slot)
            .expect("shadow lane")
            .to_bits(),
        gpu_lane.to_bits(),
        "CPU STEAD shadow and GPU values plane must agree on the same lane"
    );

    // Structural RED: production metadata type must not own a values mirror.
    let debug = format!("{column:?}");
    assert!(
        !debug.contains("values: Some(") && !debug.contains("values: None"),
        "AsyncStalenessColumn must not carry a production values Vec: {debug}"
    );
}
