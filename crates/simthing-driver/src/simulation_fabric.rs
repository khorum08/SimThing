//! Hot-path tick fabric — resolved GPU runtime resources only.
//!
//! The ordinary GPU tick must not reach scenario authority, boundary protocol,
//! spec session state, or runtime tree planning. Callers assemble a
//! [`SimulationFabric`] at the session loop edge and invoke
//! [`run_simulation_fabric_tick`] (or [`SimulationFabric::tick`]).
//!
//! Boundary protocol, scenario, and runtime tree access are forbidden:
//!
//! ```compile_fail
//! fn reach_boundary(fabric: &crate::SimulationFabric<'_>) {
//!     let _ = &fabric.proto;
//! }
//! ```
//!
//! ```compile_fail
//! fn reach_scenario(fabric: &crate::SimulationFabric<'_>) {
//!     let _ = &fabric.scenario;
//! }
//! ```
//!
//! ```compile_fail
//! fn reach_root(fabric: &crate::SimulationFabric<'_>) {
//!     let _ = &fabric.runtime_tree;
//! }
//! ```

use simthing_core::DimensionRegistry;
use simthing_feeder::{DispatchCoordinator, FeederReceiver, TickOutcome, TransformPatcher};
use simthing_gpu::{Pipelines, SlotAllocator, WorldGpuState};

/// Alias for the hot-path tick result (feeder GPU dispatch outcome).
pub type FabricTickOutcome = TickOutcome;

/// Resolved hot-path runtime parts borrowed from an open session.
///
/// Session code uses this at the loop edge; the fabric itself holds no
/// scenario, boundary, spec, or tree planning state.
pub struct HotFabricParts<'a> {
    pub coord: &'a mut DispatchCoordinator,
    pub patcher: &'a mut TransformPatcher,
    pub rx: &'a FeederReceiver,
    pub registry: &'a DimensionRegistry,
    pub allocator: &'a SlotAllocator,
    pub pipelines: &'a Pipelines,
    pub state: &'a mut WorldGpuState,
    pub dt: f32,
}

/// GPU-resident tick resources without boundary-time planning state.
pub struct SimulationFabric<'a> {
    coord: &'a mut DispatchCoordinator,
    patcher: &'a mut TransformPatcher,
    rx: &'a FeederReceiver,
    registry: &'a DimensionRegistry,
    allocator: &'a SlotAllocator,
    pipelines: &'a Pipelines,
    state: &'a mut WorldGpuState,
    dt: f32,
}

impl<'a> SimulationFabric<'a> {
    pub fn from_hot_parts(parts: HotFabricParts<'a>) -> Self {
        Self {
            coord: parts.coord,
            patcher: parts.patcher,
            rx: parts.rx,
            registry: parts.registry,
            allocator: parts.allocator,
            pipelines: parts.pipelines,
            state: parts.state,
            dt: parts.dt,
        }
    }

    /// Run one ordinary GPU tick (feeder drain → upload → pipeline → events).
    pub fn tick(&mut self) -> FabricTickOutcome {
        run_simulation_fabric_tick(self)
    }
}

/// Canonical hot-path tick entry — accepts only the fabric.
pub fn run_simulation_fabric_tick(fabric: &mut SimulationFabric<'_>) -> FabricTickOutcome {
    fabric.coord.tick(
        fabric.rx,
        fabric.patcher,
        fabric.registry,
        fabric.allocator,
        fabric.pipelines,
        fabric.state,
        fabric.dt,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{SimProperty, SimThing, SimThingKind};
    use simthing_feeder::feeder_channel;
    use simthing_gpu::{GpuContext, WorldGpuState};

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn minimal_hot_fixture() -> (DimensionRegistry, SlotAllocator, u32) {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut alloc = SlotAllocator::new();
        let a = SimThing::new(SimThingKind::Cohort, 0).id;
        let b = SimThing::new(SimThingKind::Cohort, 0).id;
        alloc.alloc(a);
        alloc.alloc(b);
        let n_dims = reg.total_columns as u32;
        (reg, alloc, n_dims)
    }

    #[test]
    fn simulation_fabric_tick_signature_accepts_only_fabric() {
        fn assert_hot_tick(f: fn(&mut SimulationFabric<'_>) -> FabricTickOutcome) {
            let _ = f;
        }
        assert_hot_tick(run_simulation_fabric_tick);
        // `SimulationFabric::tick` delegates to `run_simulation_fabric_tick` (see behavior test).
    }

    #[test]
    fn simulation_fabric_tick_behavior_preserved() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let (reg, alloc, n_dims) = minimal_hot_fixture();
        let n_slots = alloc.capacity() as u32;
        let ticks_per_day = 4u32;
        let dt = 0.25f32;

        let mut state_direct = WorldGpuState::new(ctx, &reg, n_slots);
        let pipelines_direct = Pipelines::new(&state_direct.ctx);
        let mut patcher_direct = TransformPatcher::new(alloc.capacity());
        let mut coord_direct = DispatchCoordinator::new(n_slots, n_dims, ticks_per_day);
        let (_tx_direct, rx_direct) = feeder_channel();

        let direct = coord_direct.tick(
            &rx_direct,
            &mut patcher_direct,
            &reg,
            &alloc,
            &pipelines_direct,
            &mut state_direct,
            dt,
        );

        let mut state_fabric =
            WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
        let pipelines_fabric = Pipelines::new(&state_fabric.ctx);
        let mut patcher_fabric = TransformPatcher::new(alloc.capacity());
        let mut coord_fabric = DispatchCoordinator::new(n_slots, n_dims, ticks_per_day);
        let (_tx_fabric, rx_fabric) = feeder_channel();

        let mut fabric = SimulationFabric::from_hot_parts(HotFabricParts {
            coord: &mut coord_fabric,
            patcher: &mut patcher_fabric,
            rx: &rx_fabric,
            registry: &reg,
            allocator: &alloc,
            pipelines: &pipelines_fabric,
            state: &mut state_fabric,
            dt,
        });
        let via_fabric = run_simulation_fabric_tick(&mut fabric);

        assert_eq!(direct.tick_index, via_fabric.tick_index);
        assert_eq!(direct.day_index, via_fabric.day_index);
        assert_eq!(direct.boundary_reached, via_fabric.boundary_reached);
        assert_eq!(direct.events.len(), via_fabric.events.len());
        assert_eq!(
            direct.intent_deltas_uploaded,
            via_fabric.intent_deltas_uploaded
        );
        assert_eq!(direct.uploaded_rows, via_fabric.uploaded_rows);
        assert_eq!(direct.gpu_error, via_fabric.gpu_error);
    }
}
