//! GPU buffer sync — step 9 of the day boundary.
//!
//! After all structural mutations complete (overlay lifecycle, property expiry,
//! fission/fusion, slot sync), the GPU buffers must reflect the new state.
//!
//! Three synchronization targets:
//!
//! 1. **Overlay deltas (Pass 3).**
//!    Call `build_overlay_deltas(root, registry, allocator)` and upload.
//!    This is always done at the boundary even if no overlays changed, because
//!    the `slot_delta_ranges` buffer is slot-indexed and any slot mutation
//!    (AddChild, Remove) invalidates the previous ranges.
//!
//! 2. **Threshold registrations (Pass 7).**
//!    Call `ThresholdBuilder::build(root, registry, allocator)` and upload.
//!    Returns the new CPU-side `ThresholdRegistry` for the next day.
//!
//! 3. **Values shadow flush.**
//!    - New slots (from AddChild / fission) need their GPU rows zeroed/seeded.
//!    - Dissolved slots get zeroed.
//!    - Slots that received `ExpireEffect` writes (overlay dissolve) need to
//!      be uploaded.
//!    The coordinator's `upload_full_shadow` is the safe all-slots option;
//!    the caller can also pass specific dirty slots to `upload_row` for
//!    performance.
//!
//! ## AddDimension (registry expansion)
//!
//! When `BoundaryRequest::AddDimension` is processed:
//! 1. The boundary protocol widens `DispatchCoordinator::shadow`.
//! 2. It calls `WorldGpuState::rebuild_for_registry` with the new
//!    `registry.total_columns`.
//! 3. This step re-uploads overlay deltas, threshold registrations, and the
//!    full widened shadow into the rebuilt GPU buffers.
//!

use crate::fission::FissionLineageRecord;
use crate::threshold_registry::{
    AggregateAlertRegistration, ThresholdBuilder, ThresholdRegistry, VelocityAlertRegistration,
};
use simthing_core::{DimensionRegistry, SimThing};
use simthing_feeder::DispatchCoordinator;
use simthing_gpu::{
    build_column_rule_descriptors, build_topology, encode_column_rules, SlotAllocator,
    WorldGpuState,
};

/// Outcome of the GPU sync step.
#[derive(Clone, Debug, Default)]
pub struct GpuSyncOutcome {
    pub overlay_deltas_uploaded: u32,
    pub threshold_regs_uploaded: u32,
    pub new_threshold_registry: Option<ThresholdRegistry>,
    pub reduction_depths:        u32,
    pub reduction_edges:         u32,
    pub reduction_slots:         u32,
    pub boundary_upload_bytes:   u64,
    pub value_rows_uploaded:     u32,
    pub full_value_upload:       bool,
}

/// Rebuild Pass 3 and Pass 7 GPU buffers from the current tree state.
/// Returns the new CPU-side `ThresholdRegistry` to replace the old one.
pub fn sync_gpu_buffers(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    coord: &DispatchCoordinator,
    state: &mut WorldGpuState,
    velocity_alerts: &[VelocityAlertRegistration],
    aggregate_alerts: &[AggregateAlertRegistration],
    fission_lineage: &[FissionLineageRecord],
    dirty_value_slots: Option<&[u32]>,
) -> GpuSyncOutcome {
    let mut out = GpuSyncOutcome::default();

    // 1. Overlay deltas — always rebuild at boundary.
    //
    // `build_overlay_deltas` returns one range per allocated slot, so
    // `ranges.len() == allocator.capacity()`. `state.upload_overlay_deltas`
    // requires `ranges.len() == state.n_slots` (the static GPU buffer size).
    // Pad with zero-length ranges for slots that don't exist yet; Pass 3's
    // shader skips those naturally because `range.length == 0`.
    let (deltas, mut ranges) = simthing_gpu::build_overlay_deltas(root, registry, allocator);
    if (ranges.len() as u32) < state.n_slots {
        ranges.resize(
            state.n_slots as usize,
            simthing_gpu::SlotDeltaRange::default(),
        );
    }
    let n_deltas = deltas.len() as u32;
    state.upload_overlay_deltas(&deltas, &ranges);
    out.overlay_deltas_uploaded = n_deltas;

    // 2. Threshold registrations (incl. fission lineage → FusionTrigger regs).
    let (gpu_regs, cpu_reg) = ThresholdBuilder::build_with_lineage(
        root,
        registry,
        allocator,
        velocity_alerts,
        aggregate_alerts,
        fission_lineage,
    );
    let n_regs = gpu_regs.len() as u32;
    state.upload_thresholds(&gpu_regs);
    out.threshold_regs_uploaded = n_regs;
    out.new_threshold_registry = Some(cpu_reg);

    // 3. Values shadow flush. Boundaries that only mutate known slots can
    //    upload those rows; structural rebuilds and conservative tombstone
    //    cases still force a full shadow flush.
    let value_upload_bytes = if let Some(slots) = dirty_value_slots {
        for &slot in slots {
            coord.upload_row(state, slot);
        }
        out.value_rows_uploaded = slots.len() as u32;
        out.full_value_upload = false;
        slots.len() as u64 * state.n_dims as u64 * std::mem::size_of::<f32>() as u64
    } else {
        coord.upload_full_shadow(state);
        out.value_rows_uploaded = state.n_slots;
        out.full_value_upload = true;
        coord.shadow.len() as u64 * std::mem::size_of::<f32>() as u64
    };

    // 4. Reduction topology + per-column rule table (Passes 4–6).
    //    Topology depends on tree shape and slot assignments; rebuilt every
    //    boundary. Column rules depend on `DimensionRegistry` and only change
    //    when properties are added / tombstoned, but rebuilding them is cheap
    //    (one walk over `registry.properties`).
    let topo = build_topology(root, allocator);
    let descriptors = build_column_rule_descriptors(registry, state.n_dims as usize);
    let rules_u32 = encode_column_rules(&descriptors);

    let mut depth_slots: Vec<u32> = Vec::new();
    let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
    for bucket in &topo.depth_buckets {
        let offset = depth_slots.len() as u32;
        depth_slots.extend_from_slice(bucket);
        depth_ranges.push((offset, bucket.len() as u32));
    }
    out.reduction_depths = depth_ranges.len() as u32;
    out.reduction_slots = depth_slots.len() as u32;

    // `upload_reduction_topology` asserts `child_starts.len() == n_slots + 1`.
    // `build_topology` produces a CSR sized to `allocator.capacity()`, which
    // can be less than `state.n_slots` when WorldGpuState has growth headroom.
    // Pad with the sentinel value so unallocated slots have empty child ranges.
    let n_slots = state.n_slots as usize;
    let mut child_starts = topo.child_starts.clone();
    if child_starts.len() < n_slots + 1 {
        let last = *child_starts.last().unwrap_or(&0);
        child_starts.resize(n_slots + 1, last);
    }

    state.upload_reduction_topology(
        &child_starts,
        &topo.child_indices,
        &rules_u32,
        &depth_slots,
        depth_ranges,
    );
    out.reduction_edges = topo.child_indices.len() as u32;
    out.boundary_upload_bytes = value_upload_bytes
        + deltas.len() as u64 * std::mem::size_of::<simthing_gpu::OverlayDelta>() as u64
        + ranges.len() as u64 * std::mem::size_of::<simthing_gpu::SlotDeltaRange>() as u64
        + gpu_regs.len() as u64 * std::mem::size_of::<simthing_gpu::ThresholdRegistration>() as u64
        + child_starts.len() as u64 * std::mem::size_of::<u32>() as u64
        + topo.child_indices.len() as u64 * std::mem::size_of::<u32>() as u64
        + rules_u32.len() as u64 * std::mem::size_of::<u32>() as u64
        + depth_slots.len() as u64 * std::mem::size_of::<u32>() as u64;

    out
}

#[cfg(test)]
mod tests {
    // Integration tests for gpu_sync require a GPU adapter.
    // They live in tests/boundary_integration.rs to keep this file light.
}
