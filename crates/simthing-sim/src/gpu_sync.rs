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
//! 1. Register the new `SimProperty` in the `DimensionRegistry`.
//! 2. Rebuild `WorldGpuState` with the new `total_columns` as `n_dims`.
//!    This is a full GPU buffer reallocation. Expensive, but rare (only on
//!    mid-session DLC/mod content).
//! 3. Re-upload the entire values shadow to the new buffer.
//!
//! `AddDimension` handling is intentionally left as a TODO in this stub —
//! it requires rebuilding `WorldGpuState` which requires moving the
//! `GpuContext` out of the old state and into the new one.

use simthing_core::{DimensionRegistry, SimThing};
use simthing_feeder::DispatchCoordinator;
use simthing_gpu::{SlotAllocator, WorldGpuState};
use crate::threshold_registry::{ThresholdBuilder, ThresholdRegistry};

/// Outcome of the GPU sync step.
#[derive(Clone, Debug, Default)]
pub struct GpuSyncOutcome {
    pub overlay_deltas_uploaded:  u32,
    pub threshold_regs_uploaded:  u32,
    pub new_threshold_registry:   Option<ThresholdRegistry>,
}

/// Rebuild Pass 3 and Pass 7 GPU buffers from the current tree state.
/// Returns the new CPU-side `ThresholdRegistry` to replace the old one.
pub fn sync_gpu_buffers(
    root:       &SimThing,
    registry:   &DimensionRegistry,
    allocator:  &SlotAllocator,
    coord:      &DispatchCoordinator,
    state:      &mut WorldGpuState,
) -> GpuSyncOutcome {
    let mut out = GpuSyncOutcome::default();

    // 1. Overlay deltas — always rebuild at boundary.
    let (deltas, ranges) = simthing_gpu::build_overlay_deltas(root, registry, allocator);
    let n_deltas = deltas.len() as u32;
    state.upload_overlay_deltas(&deltas, &ranges);
    out.overlay_deltas_uploaded = n_deltas;

    // 2. Threshold registrations.
    let (gpu_regs, cpu_reg) = ThresholdBuilder::build(root, registry, allocator);
    let n_regs = gpu_regs.len() as u32;
    state.upload_thresholds(&gpu_regs);
    out.threshold_regs_uploaded = n_regs;
    out.new_threshold_registry  = Some(cpu_reg);

    // 3. Values shadow flush — upload everything after structural changes.
    //    Callers that only had dirty-row patches can call upload_row individually;
    //    here we flush the full shadow to keep correctness simple at boundary time.
    coord.upload_full_shadow(state);

    out
}

#[cfg(test)]
mod tests {
    // Integration tests for gpu_sync require a GPU adapter.
    // They live in tests/boundary_integration.rs to keep this file light.
}
