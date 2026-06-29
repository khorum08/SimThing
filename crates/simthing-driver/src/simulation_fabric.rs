//! Hot-path tick fabric — resolved GPU runtime resources only.
//!
//! The ordinary GPU tick must not reach scenario authority, boundary protocol,
//! spec session state, or runtime tree planning. Callers assemble a
//! [`SimulationFabric`] at the session loop edge and invoke
//! [`run_simulation_fabric_tick`] or [`run_simulation_fabric_hot_step`].
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
//!
//! Mapping hot-path state must not reach boundary-time commitment effects:
//!
//! ```compile_fail
//! fn reach_boundary_effect(hot: &crate::MappingHotPathState) {
//!     let _ = &hot.effect;
//! }
//! ```

use simthing_core::DimensionRegistry;
use simthing_feeder::{DispatchCoordinator, FeederReceiver, TickOutcome, TransformPatcher};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
use simthing_spec::CompiledFirstSliceCommitmentThreshold;

use crate::first_slice_mapping_runtime::{FirstSliceMappingSession, FirstSliceTickOptions};

/// Alias for the hot-path tick result (feeder GPU dispatch outcome).
pub type FabricTickOutcome = TickOutcome;

/// Report from one mapping hot-path dispatch (GPU scatter → seed → stencil/reduce/EML/commitment).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FabricMappingHotReport {
    pub threshold_events: Vec<ThresholdEvent>,
}

/// Outcome of one full hot step: ordinary tick + optional RF bands + optional mapping dispatch.
#[derive(Debug)]
pub struct FabricHotStepOutcome {
    pub tick: FabricTickOutcome,
    pub resource_flow_band_dispatched: bool,
    pub mapping: Option<FabricMappingHotReport>,
}

/// Parameters for the combined hot step (resolved at the session loop edge).
pub struct FabricHotStepParams<'a> {
    pub resource_flow_pipeline_enabled: bool,
    pub mapping: Option<&'a mut MappingHotPathState>,
}

/// GPU-resident mapping hot path — scatter, seed, stencil/reduce/EML/commitment only.
pub struct MappingHotPathState {
    pub mapping: FirstSliceMappingSession,
    scatter: simthing_gpu::IndexedScatterOp,
    entries: Vec<simthing_gpu::ScatterEntry>,
    cells: Vec<(u32, u32)>,
    weights: (f32, f32),
    commitment: CompiledFirstSliceCommitmentThreshold,
}

impl MappingHotPathState {
    pub fn new(
        mapping: FirstSliceMappingSession,
        scatter: simthing_gpu::IndexedScatterOp,
        entries: Vec<simthing_gpu::ScatterEntry>,
        cells: Vec<(u32, u32)>,
        weights: (f32, f32),
        commitment: CompiledFirstSliceCommitmentThreshold,
    ) -> Self {
        Self {
            mapping,
            scatter,
            entries,
            cells,
            weights,
            commitment,
        }
    }
}

/// Mapping hot-path dispatch failure (surfaced as session error at the loop edge).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MappingHotDispatchError(pub String);

/// Combined hot-step failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FabricHotStepError {
    Mapping(MappingHotDispatchError),
}

impl From<MappingHotDispatchError> for FabricHotStepError {
    fn from(value: MappingHotDispatchError) -> Self {
        Self::Mapping(value)
    }
}

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

/// Dispatch RF OrderBand ops when the pipeline flag and GPU state agree.
pub fn run_resource_flow_bands_if_active(
    fabric: &mut SimulationFabric<'_>,
    resource_flow_pipeline_enabled: bool,
) -> bool {
    if resource_flow_pipeline_enabled && fabric.state.accumulator_resource_flow_active {
        fabric
            .state
            .run_resource_flow_bands(fabric.state.accumulator_resource_flow_bands, fabric.dt);
        return true;
    }
    false
}

/// One mapping hot-path step: scatter → seed → stencil/reduce/EML/commitment scan.
pub fn run_mapping_hot_dispatch(
    state: &WorldGpuState,
    hot: &mut MappingHotPathState,
) -> Result<FabricMappingHotReport, MappingHotDispatchError> {
    let ctx = &state.ctx;
    hot.scatter
        .dispatch(
            ctx,
            &state.values,
            hot.mapping.stencil_input_buffer(),
            &hot.entries,
        )
        .map_err(|e| MappingHotDispatchError(format!("{e}")))?;
    hot.mapping
        .queue_gpu_seed_cells(&hot.cells)
        .map_err(|e| MappingHotDispatchError(format!("{e:?}")))?;
    let report = hot
        .mapping
        .tick_with_commitment_spec(
            ctx,
            FirstSliceTickOptions::hot_path(),
            hot.weights,
            &hot.commitment,
        )
        .map_err(|e| MappingHotDispatchError(format!("{e:?}")))?;
    Ok(FabricMappingHotReport {
        threshold_events: report.threshold_events,
    })
}

/// Full hot step: ordinary tick, then RF bands, then mapping hot dispatch.
pub fn run_simulation_fabric_hot_step(
    fabric: &mut SimulationFabric<'_>,
    params: FabricHotStepParams<'_>,
) -> Result<FabricHotStepOutcome, FabricHotStepError> {
    let tick = run_simulation_fabric_tick(fabric);
    let resource_flow_band_dispatched =
        run_resource_flow_bands_if_active(fabric, params.resource_flow_pipeline_enabled);
    let mapping = match params.mapping {
        Some(hot) => Some(run_mapping_hot_dispatch(fabric.state, hot)?),
        None => None,
    };
    Ok(FabricHotStepOutcome {
        tick,
        resource_flow_band_dispatched,
        mapping,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{SimProperty, SimThing, SimThingKind};
    use simthing_feeder::feeder_channel;
    use simthing_gpu::WorldGpuState;
    use simthing_spec::{
        compile_region_field_preview, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
        MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
        RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
        RegionFieldSourcePolicySpec, RegionFieldSpec,
    };

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

    fn first_slice_spec() -> RegionFieldSpec {
        let mut spec = RegionFieldSpec {
            name: "fabric_hot_mapping".into(),
            grid_size: 10,
            n_dims: 8,
            source_col: 0,
            target_col: 0,
            operator: RegionFieldOperatorSpec::SourceCappedNormalized,
            horizon: 8,
            allow_extended_horizon: false,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: Some(500.0),
            source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
            cadence: RegionFieldCadenceSpec::EveryTick,
            grid_profile: RegionFieldGridProfile::StandardSquare,
            reduction: Some(RegionFieldReductionSpec {
                child_slot_start: 0,
                child_slot_count: 100,
                child_col: 0,
                parent_slot: 100,
                parent_col: 0,
                order_band: 0,
            }),
            parent_formula: Some(RegionFieldFormulaBindingSpec {
                formula_class: "field_urgency".into(),
                tree_id: Some(1),
                weight_pressure: None,
                weight_resource: None,
            }),
            commitment: None,
            request_atlas_batching: false,
            max_region_field_vram_bytes: None,
            summary_policy: Default::default(),
            pressure_binding: None,
        };
        spec.commitment = Some(FirstSliceCommitmentSpec {
            source_formula_class: "field_urgency".into(),
            parent_slot: 100,
            urgency_col: 4,
            threshold: 5490.8657,
            direction: FirstSliceCommitmentDirectionSpec::Upward,
            event_kind: 0x5345_4144,
            effect: None,
        });
        spec
    }

    fn minimal_mapping_hot(ctx: &GpuContext) -> MappingHotPathState {
        let spec = first_slice_spec();
        let preview = compile_region_field_preview(&spec).expect("preview");
        let commitment = preview
            .commitment
            .clone()
            .expect("default commitment from preview");
        let mapping = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .expect("mapping session");
        let scatter = simthing_gpu::IndexedScatterOp::new(ctx);
        MappingHotPathState::new(
            mapping,
            scatter,
            Vec::new(),
            vec![(0, 0)],
            (1.0, 1.0),
            commitment,
        )
    }

    #[test]
    fn simulation_fabric_tick_signature_accepts_only_fabric() {
        fn assert_hot_tick(f: fn(&mut SimulationFabric<'_>) -> FabricTickOutcome) {
            let _ = f;
        }
        assert_hot_tick(run_simulation_fabric_tick);
    }

    #[test]
    fn simulation_fabric_hot_step_signature_accepts_only_fabric() {
        fn assert_hot_step(
            f: fn(
                &mut SimulationFabric<'_>,
                FabricHotStepParams<'_>,
            ) -> Result<FabricHotStepOutcome, FabricHotStepError>,
        ) {
            let _ = f;
        }
        assert_hot_step(run_simulation_fabric_hot_step);
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

    #[test]
    fn simulation_fabric_rf_band_dispatch_behavior_preserved() {
        let Some(_ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let (reg, alloc, n_dims) = minimal_hot_fixture();
        let n_slots = alloc.capacity() as u32;
        let dt = 0.25f32;

        for (pipeline, active, bands) in [
            (false, true, 2u32),
            (true, false, 2u32),
            (true, true, 0u32),
            (true, true, 3u32),
        ] {
            let mut state = WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
            state.set_resource_flow_dispatch(active, bands);

            let session_would_dispatch = pipeline && active;
            let mut direct_state = state;
            if session_would_dispatch {
                direct_state.run_resource_flow_bands(bands, dt);
            }

            let mut fabric_state =
                WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
            fabric_state.set_resource_flow_dispatch(active, bands);
            let pipelines_f = Pipelines::new(&fabric_state.ctx);
            let mut patcher_f = TransformPatcher::new(alloc.capacity());
            let mut coord_f = DispatchCoordinator::new(n_slots, n_dims, 4);
            let (_tx_f, rx_f) = feeder_channel();

            let mut fabric = SimulationFabric::from_hot_parts(HotFabricParts {
                coord: &mut coord_f,
                patcher: &mut patcher_f,
                rx: &rx_f,
                registry: &reg,
                allocator: &alloc,
                pipelines: &pipelines_f,
                state: &mut fabric_state,
                dt,
            });
            let dispatched = run_resource_flow_bands_if_active(&mut fabric, pipeline);

            assert_eq!(
                dispatched, session_would_dispatch,
                "pipeline={pipeline} active={active} bands={bands}"
            );
            let _ = direct_state;
        }
    }

    #[test]
    fn simulation_fabric_mapping_hot_dispatch_behavior_preserved() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let (reg, alloc, n_dims) = minimal_hot_fixture();
        let n_slots = alloc.capacity() as u32;
        let dt = 0.25f32;

        let mut state = WorldGpuState::new(ctx, &reg, n_slots);
        let mut hot_direct = minimal_mapping_hot(&state.ctx);
        let direct =
            run_mapping_hot_dispatch(&state, &mut hot_direct).expect("direct mapping hot dispatch");

        let mut state_fabric =
            WorldGpuState::new(GpuContext::new_blocking().unwrap(), &reg, n_slots);
        let mut hot_fabric = minimal_mapping_hot(&state_fabric.ctx);
        let via_fabric = run_mapping_hot_dispatch(&state_fabric, &mut hot_fabric)
            .expect("fabric mapping hot dispatch");

        assert_eq!(
            direct.threshold_events.len(),
            via_fabric.threshold_events.len()
        );

        let pipelines = Pipelines::new(&state_fabric.ctx);
        let mut patcher = TransformPatcher::new(alloc.capacity());
        let mut coord = DispatchCoordinator::new(n_slots, n_dims, 4);
        let (_tx, rx) = feeder_channel();
        let mut hot_step = minimal_mapping_hot(&state_fabric.ctx);

        let mut fabric = SimulationFabric::from_hot_parts(HotFabricParts {
            coord: &mut coord,
            patcher: &mut patcher,
            rx: &rx,
            registry: &reg,
            allocator: &alloc,
            pipelines: &pipelines,
            state: &mut state_fabric,
            dt,
        });
        let step = run_simulation_fabric_hot_step(
            &mut fabric,
            FabricHotStepParams {
                resource_flow_pipeline_enabled: false,
                mapping: Some(&mut hot_step),
            },
        )
        .expect("hot step with mapping");
        let step_mapping = step.mapping.expect("mapping report");
        assert_eq!(
            step_mapping.threshold_events.len(),
            via_fabric.threshold_events.len()
        );
        let _ = state;
    }
}
