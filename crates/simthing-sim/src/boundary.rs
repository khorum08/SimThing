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

use simthing_core::{DimensionRegistry, SimThing};
use simthing_feeder::{
    BoundaryRequest, DispatchCoordinator, TransformPatcher, TreeMaintainer,
};
use simthing_gpu::{SlotAllocator, ThresholdEvent, WorldGpuState};

use crate::fission::{resolve_fission_fusion, FissionOutcome};
use crate::gpu_sync::{sync_gpu_buffers, GpuSyncOutcome};
use crate::overlay_lifecycle::{attach_overlay, resolve_overlay_lifecycle, LifecycleOutcome};
use crate::property_expiry::{resolve_property_expiry, ExpiryOutcome};
use crate::threshold_registry::ThresholdRegistry;

/// Everything that happened during a boundary. Useful for logging,
/// observability, replay, and tests.
#[derive(Debug, Default)]
pub struct BoundaryOutcome {
    pub day:                u64,
    pub lifecycle:          LifecycleOutcome,
    pub expiry:             ExpiryOutcome,
    pub fission:            FissionOutcome,
    pub gpu_sync:           GpuSyncOutcome,
    pub overlays_attached:  u32,
    pub boundary_requests:  u32,
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
    pub root:      SimThing,
    pub registry:  DimensionRegistry,
    pub allocator: SlotAllocator,
    cpu_threshold_registry: ThresholdRegistry,
    maintainer:    TreeMaintainer,
}

impl BoundaryProtocol {
    pub fn new(
        root:      SimThing,
        registry:  DimensionRegistry,
        allocator: SlotAllocator,
    ) -> Self {
        Self {
            root,
            registry,
            allocator,
            cpu_threshold_registry: ThresholdRegistry::new(),
            maintainer:             TreeMaintainer::new(),
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
        events:  Vec<ThresholdEvent>,
        patcher: &mut TransformPatcher,
        coord:   &DispatchCoordinator,
        state:   &mut WorldGpuState,
        day:     u64,
    ) -> BoundaryOutcome {
        let mut out = BoundaryOutcome { day, ..Default::default() };
        let n_dims  = coord.n_dims() as usize;

        // We need a mutable shadow to apply expire effects. Take a temporary
        // copy since coord owns the canonical shadow; we'll re-upload in step 9.
        let mut shadow = coord.shadow.clone();

        // Step 4: Overlay lifecycle — dissolve + expire effects.
        out.lifecycle = resolve_overlay_lifecycle(
            &mut self.root,
            &self.registry,
            &self.allocator,
            &mut shadow,
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

        // Step 6: Fission and fusion.
        out.fission = resolve_fission_fusion(
            &mut self.root,
            &self.registry,
            &mut self.allocator,
            &events,
            &self.cpu_threshold_registry,
            &shadow,
            n_dims,
            day as u32,
        );

        // Step 7: Attach new instruction overlays.
        let requests = patcher.take_boundary_requests();
        out.boundary_requests = requests.len() as u32;
        for req in &requests {
            if let BoundaryRequest::AttachOverlay { target, overlay } = req {
                if attach_overlay(&mut self.root, *target, overlay.clone()) {
                    out.overlays_attached += 1;
                }
            }
        }

        // Step 8: Structural mutations via Tree Maintainer.
        // (Execution body is a stub today; counts get recorded.)
        self.maintainer.execute(requests);

        // Step 9: Rebuild GPU buffers from current tree + upload shadow.
        let gpu_out = sync_gpu_buffers(
            &self.root,
            &self.registry,
            &self.allocator,
            coord,
            state,
        );
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
    pub fn initial_gpu_sync(
        &mut self,
        coord: &DispatchCoordinator,
        state: &mut WorldGpuState,
    ) {
        let out = sync_gpu_buffers(
            &self.root,
            &self.registry,
            &self.allocator,
            coord,
            state,
        );
        if let Some(new_reg) = out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
    use simthing_gpu::SlotAllocator;

    #[test]
    fn boundary_protocol_constructs_cleanly() {
        let mut reg   = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let root  = SimThing::new(SimThingKind::World, 0);
        let alloc = SlotAllocator::new();
        let proto = BoundaryProtocol::new(root, reg, alloc);
        assert!(proto.threshold_registry().is_empty());
    }
}
