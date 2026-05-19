//! BoundaryProtocol — the §10 day-boundary orchestrator.
//!
//! Owns the authoritative SimThing tree root and sequences the full
//! 10-step boundary protocol when the `DispatchCoordinator` signals
//! `boundary_reached = true`.
//!
//! ## Step sequence (from design_v4.md §10)
//!
//! Steps 1–3 are handled by the feeder layer (`DispatchCoordinator::tick`
//! + `TransformPatcher::drain`). `BoundaryProtocol::execute` handles 4–10.
//!
//! ```text
//! 4.  Overlay lifecycle resolves  -- overlay_lifecycle::resolve_overlay_lifecycle
//! 5.  Property expiry resolves    -- property_expiry::resolve_property_expiry
//! 6.  Fission/fusion executes     -- fission::resolve_fission_fusion
//! 7.  Instruction overlays        -- overlay_lifecycle::attach_overlay (per request)
//! 8.  Slot table + registry sync  -- TreeMaintainer::execute (structural requests)
//! 9.  GPU buffer sync             -- gpu_sync::sync_gpu_buffers
//! 10. Day N+1 dispatch ready      -- (caller resumes tick loop)
//! ```

use simthing_core::{DimensionRegistry, SimPropertyId, SimThing};
use simthing_feeder::{DispatchCoordinator, MaintainerOutcome, TransformPatcher};
use simthing_gpu::{SlotAllocator, ThresholdEvent, WorldGpuState};

use crate::fission::{resolve_fission_fusion, FissionOutcome};
use crate::gpu_sync::{sync_gpu_buffers, GpuSyncOutcome};
use crate::overlay_lifecycle::{resolve_overlay_lifecycle, LifecycleOutcome};
use crate::property_expiry::{resolve_property_expiry, ExpiryOutcome};
use crate::threshold_registry::ThresholdRegistry;
use crate::tree_mutation::apply_structural_mutations;

/// Everything that happened during a boundary. Useful for logging,
/// observability, replay, and tests.
#[derive(Debug, Default)]
pub struct BoundaryOutcome {
    pub day: u64,
    pub lifecycle: LifecycleOutcome,
    pub expiry: ExpiryOutcome,
    pub fission: FissionOutcome,
    pub maintainer: MaintainerOutcome,
    pub gpu_sync: GpuSyncOutcome,
    pub boundary_requests: u32,
}

/// Top-level boundary orchestrator.
///
/// Owns:
/// - The authoritative `SimThing` tree root.
/// - The `DimensionRegistry`.
/// - The `SlotAllocator`.
/// - The current CPU-side `ThresholdRegistry` (rebuilt each boundary).
/// - The `TreeMaintainer` (step 8).
///
/// Does NOT own the GPU state or the feeder layer — those are passed in
/// by the top-level driver (the eventual `simthing-sim` binary / thread).
pub struct BoundaryProtocol {
    pub root: SimThing,
    pub registry: DimensionRegistry,
    pub allocator: SlotAllocator,
    cpu_threshold_registry: ThresholdRegistry,
}

impl BoundaryProtocol {
    pub fn new(root: SimThing, registry: DimensionRegistry, allocator: SlotAllocator) -> Self {
        Self {
            root,
            registry,
            allocator,
            cpu_threshold_registry: ThresholdRegistry::new(),
        }
    }

    /// Run the full §10 boundary sequence (steps 4–9).
    ///
    /// `events`   — GPU threshold events from the last tick's Pass 7 readback.
    /// `patcher`  — from `TransformPatcher::take_boundary_requests()`.
    /// `coord`    — owns the CPU values shadow; receives dirty-row uploads.
    /// `state`    — GPU buffer owner.
    /// `day`      — current day index (for logging + AfterTicks tracking).
    pub fn execute(
        &mut self,
        events: Vec<ThresholdEvent>,
        patcher: &mut TransformPatcher,
        coord: &mut DispatchCoordinator,
        state: &mut WorldGpuState,
        day: u64,
    ) -> BoundaryOutcome {
        let mut out = BoundaryOutcome {
            day,
            ..Default::default()
        };
        let n_dims = coord.n_dims() as usize;

        // The CPU shadow reflects only CPU-side patches; integration output
        // from Pass 1/2 lives only on the GPU. Before mutating the shadow
        // at the boundary, pull the canonical GPU values back so our
        // structural mutations (zeroing new rows, expire writebacks, etc.)
        // operate on the correct base — otherwise the eventual
        // `upload_full_shadow` would wipe out a day's worth of integration.
        // Endgame cost: ~3 MB once per boundary; negligible.
        coord.shadow = state.read_values();
        let needed = coord.n_slots() as usize * n_dims;
        if coord.shadow.len() < needed {
            coord.shadow.resize(needed, 0.0);
        }

        // Step 4: Overlay lifecycle — dissolve + expire effects.
        // Mutates coord.shadow directly (apply_expire_effects writes into it).
        out.lifecycle = resolve_overlay_lifecycle(
            &mut self.root,
            &self.registry,
            &self.allocator,
            &mut coord.shadow,
            n_dims,
            day as u32,
        );

        // Step 5: Property expiry (threshold-driven + CPU-side TowardZero/AfterTicks).
        out.expiry = resolve_property_expiry(
            &mut self.root,
            &mut self.registry,
            &events,
            &self.cpu_threshold_registry,
        );

        // Step 6: Fission and fusion. Spawns new SimThings + allocates slots.
        // Reads from shadow for secondary-condition checks and seeds newly
        // fissioned children from the parent's current GPU row.
        out.fission = resolve_fission_fusion(
            &mut self.root,
            &self.registry,
            &mut self.allocator,
            &events,
            &self.cpu_threshold_registry,
            &mut coord.shadow,
            n_dims,
            day as u32,
        );

        // Steps 7 + 8: Structural mutations (AddChild, Remove, Reparent,
        // AttachOverlay, AddDimension). One pass through `apply_structural_mutations`
        // handles every BoundaryRequest variant.
        let requests = patcher.take_boundary_requests();
        out.boundary_requests = requests.len() as u32;

        // Grow shadow to cover any new slots allocated during fission (step 6)
        // before applying structural mutations. apply_structural_mutations
        // expects values_shadow to be sized for the current allocator capacity.
        let needed = self.allocator.capacity() * n_dims;
        if coord.shadow.len() < needed {
            coord.shadow.resize(needed, 0.0);
        }

        out.maintainer = apply_structural_mutations(
            requests,
            &mut self.root,
            &mut self.allocator,
            &mut self.registry,
            &mut coord.shadow,
            n_dims,
        );

        if self.registry.total_columns as u32 != coord.n_dims() {
            let old_n_dims = coord.n_dims() as usize;
            coord.resize_dimensions(self.registry.total_columns as u32);
            let new_n_dims = coord.n_dims() as usize;
            seed_dimension_values(
                &self.root,
                &self.registry,
                &self.allocator,
                &out.maintainer.dimensions_added,
                &mut coord.shadow,
                old_n_dims,
                new_n_dims,
            );
            state.rebuild_for_registry(&self.registry);
        } else if !out.maintainer.dimensions_added.is_empty() {
            state.rebuild_for_registry(&self.registry);
        }

        // After structural mutations the allocator may have grown again
        // (AddChild). Resize shadow once more so step 9 uploads the full
        // capacity.
        let final_n_dims = coord.n_dims() as usize;
        let final_capacity = self.allocator.capacity() * final_n_dims;
        if coord.shadow.len() < final_capacity {
            coord.shadow.resize(final_capacity, 0.0);
        }

        // Step 9: Rebuild GPU buffers from current tree + upload shadow.
        //
        // Pre-condition: allocator.capacity() must not exceed the
        // WorldGpuState's n_slots, otherwise upload_full_shadow writes past
        // the GPU buffer end. Pre-sizing WorldGpuState with growth headroom
        // is the caller's responsibility today. AddDimension-style buffer
        // reallocation is a follow-up.
        assert!(
            self.allocator.capacity() as u32 <= coord.n_slots(),
            "allocator capacity {} exceeds shadow n_slots {}; \
             WorldGpuState was built without enough headroom",
            self.allocator.capacity(),
            coord.n_slots(),
        );

        let gpu_out = sync_gpu_buffers(&self.root, &self.registry, &self.allocator, coord, state);
        // Adopt the new threshold registry for the next day.
        if let Some(new_reg) = gpu_out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
        }
        out.gpu_sync = GpuSyncOutcome {
            overlay_deltas_uploaded: gpu_out.overlay_deltas_uploaded,
            threshold_regs_uploaded: gpu_out.threshold_regs_uploaded,
            new_threshold_registry: None, // moved into self above
        };

        out
    }

    /// Read-only access to the current threshold registry (for diagnostics).
    pub fn threshold_registry(&self) -> &ThresholdRegistry {
        &self.cpu_threshold_registry
    }

    /// Manually seed the GPU threshold registry at session start (before any
    /// ticks). Normally called once after constructing the protocol, so that
    /// Pass 7 has registrations from tick 1 onward.
    pub fn initial_gpu_sync(&mut self, coord: &DispatchCoordinator, state: &mut WorldGpuState) {
        let out = sync_gpu_buffers(&self.root, &self.registry, &self.allocator, coord, state);
        if let Some(new_reg) = out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
        }
    }
}

fn seed_dimension_values(
    node: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    properties: &[SimPropertyId],
    shadow: &mut [f32],
    old_n_dims: usize,
    new_n_dims: usize,
) {
    if let Some(slot) = allocator.slot_of(node.id) {
        let base = slot as usize * new_n_dims;
        for &pid in properties {
            if pid.index() >= registry.properties.len() {
                continue;
            }
            let Some(value) = node.property(pid) else {
                continue;
            };
            let range = registry.column_range(pid);
            if range.start < old_n_dims {
                continue;
            }
            let start = base + range.start;
            let end = start + value.data.len();
            if end <= shadow.len() {
                shadow[start..end].copy_from_slice(&value.data);
            }
        }
    }

    for child in &node.children {
        seed_dimension_values(
            child, registry, allocator, properties, shadow, old_n_dims, new_n_dims,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
    use simthing_gpu::SlotAllocator;

    #[test]
    fn boundary_protocol_constructs_cleanly() {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let root = SimThing::new(SimThingKind::World, 0);
        let alloc = SlotAllocator::new();
        let proto = BoundaryProtocol::new(root, reg, alloc);
        assert!(proto.threshold_registry().is_empty());
    }
}
