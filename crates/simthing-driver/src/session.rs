//! GPU session loop — ticks, boundaries, and replay recording.

use std::path::Path;
use std::time::Instant;

use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{GpuContext, Pipelines, WorldGpuState};
use simthing_sim::{
    BoundaryOutcome, BoundaryProtocol, BoundaryTiming, ReplayFrame, ReplayWriter, SimRuntimeTree,
};
use simthing_spec::{
    CapabilityTreeInstance, CapabilityTreeState, CapabilityUnlockRegistration, GameModeSpec,
    ResourceEconomyOptInMode, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};
use std::collections::HashMap;
use std::sync::Mutex;
use thiserror::Error;

use crate::install::{install_atomic, InstallError, InstallPreview};
use crate::scenario::Scenario;
use crate::simulation_fabric::{
    run_simulation_fabric_hot_cycle, FabricHotCycleOutcome, FabricHotCycleParams,
    FabricHotStepOutcome, HotFabricParts, MappingHotPathState, SimulationFabric,
};
use crate::spec_replay::{self, make_spec_snapshot_record};
use crate::spec_session::SpecSessionState;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("gpu init: {0}")]
    Gpu(#[from] simthing_gpu::GpuInitError),
    #[error("scenario: {0}")]
    Scenario(#[from] crate::scenario::ScenarioError),
    #[error("replay: {0}")]
    Replay(#[from] simthing_sim::ReplayError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("install: {0}")]
    Install(#[from] InstallError),
    #[error("resource flow sync: {0}")]
    ResourceFlow(#[from] crate::arena_allocation_sync::ResourceFlowSyncError),
    #[error("resource economy sync: {0}")]
    ResourceEconomy(#[from] crate::resource_economy_sync::ResourceEconomySyncError),
    #[error("GPU boundary sync: {0}")]
    GpuSync(#[from] simthing_sim::GpuSyncError),
    #[error("session mapping: {0}")]
    Mapping(String),
    #[error("resource flow opt-in: {0}")]
    ResourceFlowOptIn(String),
    #[error("threshold install: {0}")]
    ThresholdInstall(String),
    #[error("player-intent admission: {0}")]
    PlayerIntentAdmission(String),
    #[error("execution posture: {0}")]
    ExecutionPosture(String),
}

/// Outcome of a single [`SimSession::step_once`] production hot-cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepOnceOutcome {
    /// Hot ticks advanced this step (normally 1 when successful).
    pub ticks_run: u64,
    /// Boundaries completed this step (0 or 1).
    pub boundaries_run: u64,
    /// Whether a day boundary was reached on this hot cycle.
    pub boundary_reached: bool,
}

/// Borrowed, generation-stamped observation of the session's existing CPU shadow.
///
/// The handle exposes no mutable slice, evaluator, submission method, or decision
/// channel. Runtime decisions remain on-device; this is presentation/readback only.
///
/// ```compile_fail,E0616
/// use simthing_driver::SessionShadowView;
/// fn forge_write(view: SessionShadowView<'_>) {
///     view.values[0] = 9.0;
/// }
/// ```
pub struct SessionShadowView<'a> {
    generation: simthing_core::GenerationStamp,
    tick_index: u64,
    n_dims: usize,
    allocator: &'a simthing_gpu::SlotAllocator,
    values: &'a [f32],
}

impl SessionShadowView<'_> {
    pub fn generation(&self) -> simthing_core::GenerationStamp {
        self.generation
    }

    pub fn tick_index(&self) -> u64 {
        self.tick_index
    }

    pub fn value(
        &self,
        simthing: simthing_core::SimThingId,
        column: simthing_core::ColumnIndex,
    ) -> Option<f32> {
        let slot = self.allocator.slot_of(simthing)?.raw() as usize;
        self.values
            .get(slot.checked_mul(self.n_dims)?.checked_add(column.raw())?)
            .copied()
    }

    pub fn row(&self, simthing: simthing_core::SimThingId) -> Option<&[f32]> {
        let slot = self.allocator.slot_of(simthing)?.raw() as usize;
        let start = slot.checked_mul(self.n_dims)?;
        self.values.get(start..start.checked_add(self.n_dims)?)
    }
}

pub struct RunSummary {
    pub ticks_run: u64,
    pub boundaries_run: u64,
    pub frames_written: u32,
    pub fission_events: u32,
    pub rmw_rows_synced: u64,
    pub rmw_readback_bytes: u64,
    pub intent_deltas_uploaded: u64,
    pub intent_delta_bytes: u64,
    pub tick_total_ms: f64,
    pub tick_drain_ms: f64,
    pub tick_intent_upload_ms: f64,
    pub tick_dirty_upload_ms: f64,
    pub tick_gpu_pipeline_ms: f64,
    pub tick_event_readback_ms: f64,
    pub tick_event_readback_bytes: u64,
    pub submit_tick_patches_ms: f64,
    pub resource_flow_band_dispatches: u64,
    pub mapping_ticks: u64,
    pub mapping_commitment_events: u64,
    pub mapping_commitment_effects_applied: u64,
    pub boundary_total_ms: f64,
    pub boundary_value_readback_ms: f64,
    pub boundary_alert_collect_ms: f64,
    pub boundary_lifecycle_ms: f64,
    pub boundary_expiry_ms: f64,
    pub boundary_pregrow_fission_ms: f64,
    pub boundary_fission_ms: f64,
    pub boundary_lineage_ms: f64,
    pub boundary_request_drain_ms: f64,
    pub boundary_pregrow_add_child_ms: f64,
    pub boundary_structural_ms: f64,
    pub boundary_dimension_rebuild_ms: f64,
    pub boundary_final_capacity_ms: f64,
    pub boundary_gpu_sync_ms: f64,
    pub boundary_delta_log_ms: f64,
    pub boundaries_skipped: u64,
    pub boundary_readback_bytes: u64,
    pub boundary_upload_bytes: u64,
    pub boundary_value_rows_uploaded: u64,
    pub boundary_full_value_uploads: u64,
    pub overlay_deltas_uploaded: u64,
    pub threshold_regs_uploaded: u64,
    pub reduction_edges_uploaded: u64,
    pub reduction_slots_uploaded: u64,
    pub reduction_depths_total: u64,
    pub reduction_depths_max: u32,
}

impl RunSummary {
    fn new() -> Self {
        Self {
            ticks_run: 0,
            boundaries_run: 0,
            frames_written: 0,
            fission_events: 0,
            rmw_rows_synced: 0,
            rmw_readback_bytes: 0,
            intent_deltas_uploaded: 0,
            intent_delta_bytes: 0,
            tick_total_ms: 0.0,
            tick_drain_ms: 0.0,
            tick_intent_upload_ms: 0.0,
            tick_dirty_upload_ms: 0.0,
            tick_gpu_pipeline_ms: 0.0,
            tick_event_readback_ms: 0.0,
            tick_event_readback_bytes: 0,
            submit_tick_patches_ms: 0.0,
            resource_flow_band_dispatches: 0,
            mapping_ticks: 0,
            mapping_commitment_events: 0,
            mapping_commitment_effects_applied: 0,
            boundary_total_ms: 0.0,
            boundary_value_readback_ms: 0.0,
            boundary_alert_collect_ms: 0.0,
            boundary_lifecycle_ms: 0.0,
            boundary_expiry_ms: 0.0,
            boundary_pregrow_fission_ms: 0.0,
            boundary_fission_ms: 0.0,
            boundary_lineage_ms: 0.0,
            boundary_request_drain_ms: 0.0,
            boundary_pregrow_add_child_ms: 0.0,
            boundary_structural_ms: 0.0,
            boundary_dimension_rebuild_ms: 0.0,
            boundary_final_capacity_ms: 0.0,
            boundary_gpu_sync_ms: 0.0,
            boundary_delta_log_ms: 0.0,
            boundaries_skipped: 0,
            boundary_readback_bytes: 0,
            boundary_upload_bytes: 0,
            boundary_value_rows_uploaded: 0,
            boundary_full_value_uploads: 0,
            overlay_deltas_uploaded: 0,
            threshold_regs_uploaded: 0,
            reduction_edges_uploaded: 0,
            reduction_slots_uploaded: 0,
            reduction_depths_total: 0,
            reduction_depths_max: 0,
        }
    }
}

fn accumulate_boundary_timing(summary: &mut RunSummary, timing: BoundaryTiming) {
    summary.boundary_value_readback_ms += timing.value_readback_ms;
    summary.boundary_alert_collect_ms += timing.alert_collect_ms;
    summary.boundary_lifecycle_ms += timing.lifecycle_ms;
    summary.boundary_expiry_ms += timing.expiry_ms;
    summary.boundary_pregrow_fission_ms += timing.pregrow_fission_ms;
    summary.boundary_fission_ms += timing.fission_ms;
    summary.boundary_lineage_ms += timing.lineage_ms;
    summary.boundary_request_drain_ms += timing.request_drain_ms;
    summary.boundary_pregrow_add_child_ms += timing.pregrow_add_child_ms;
    summary.boundary_structural_ms += timing.structural_ms;
    summary.boundary_dimension_rebuild_ms += timing.dimension_rebuild_ms;
    summary.boundary_final_capacity_ms += timing.final_capacity_ms;
    summary.boundary_gpu_sync_ms += timing.gpu_sync_ms;
    summary.boundary_delta_log_ms += timing.delta_log_ms;
}

/// Owns the full tick + boundary loop for one scenario.
pub struct SimSession {
    pub scenario: Scenario,
    pub proto: BoundaryProtocol,
    pub coord: DispatchCoordinator,
    pub patcher: TransformPatcher,
    pub state: WorldGpuState,
    pub pipelines: Pipelines,
    pub rx: simthing_feeder::FeederReceiver,
    pub tx: simthing_feeder::FeederSender,
    pub spec_state: SpecSessionState,
    /// Last boundary dynamic Resource Flow fission enrollment report (E-2B-5R).
    pub last_resource_flow_dynamic_enrollment_report:
        Option<crate::resource_flow_fission_enrollment::DynamicFissionEnrollmentReport>,
    /// RF-T3: why Resource Flow GPU execution is enabled/disabled on this session.
    pub resource_flow_flag_source: crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource,
    /// RF-T4: authored scenario-class / execution profile at session open.
    pub resource_flow_execution_profile: ResourceFlowExecutionProfile,
    /// CT-3b+4a Line 3: profile-gated in-loop mapping state. `None` unless
    /// the game mode authored `SparseRegionFieldV1` + a region field with a
    /// pressure binding; presence alone never wires anything.
    pub mapping: Option<SessionMappingState>,
    /// Commitment journal: every mapping threshold crossing observed in the
    /// session loop, in tick order. Consumed at boundaries; diagnostic
    /// readback never feeds runtime decisions.
    pub mapping_commitments: Vec<MappingCommitmentRecord>,
    /// CONTINUOUS-POSTURE-SOAK-0: scheduling policy over the same kernel.
    /// Default [`ExecutionPosture::Paced`]; continuous batches call the identical
    /// hot-cycle + boundary path — never a second kernel or semantic fork.
    execution_posture: simthing_core::ExecutionPosture,
    resolved_order_directives: Mutex<crate::order_directive::OrderDirectiveGateState>,
    order_directive_injection_log: Mutex<Vec<crate::order_directive::OrderDirectiveInjection>>,
}

/// CT-3b+4a Line 3: everything the session loop needs to run the admitted
/// RF-fed heatmap chain per tick, GPU-resident end to end.
pub struct SessionMappingState {
    pub hot: MappingHotPathState,
    boundary: MappingBoundaryState,
}

/// Boundary-time mapping state — commitment effects and journal watermarks.
struct MappingBoundaryState {
    effect: Option<ResolvedCommitmentEffect>,
    /// Journal watermark: crossings already considered for effect application.
    commitments_consumed: usize,
}

/// Install-resolved authored commitment consequence (CT-3b+4a closure).
struct ResolvedCommitmentEffect {
    consequence: crate::CrossingConsequenceBinding,
    once: bool,
    fired: bool,
}

/// One mapping commitment crossing observed by the session loop.
#[derive(Clone, Debug, PartialEq)]
pub struct MappingCommitmentRecord {
    pub tick: u64,
    pub event: simthing_gpu::ThresholdEvent,
}

fn accumulate_tick_outcome(
    summary: &mut RunSummary,
    tick: &FabricHotStepOutcome,
    tick_wall_ms: f64,
) {
    summary.tick_total_ms += tick_wall_ms;
    summary.ticks_run += 1;
    summary.rmw_rows_synced += tick.tick.rmw_rows_synced as u64;
    summary.rmw_readback_bytes += tick.tick.rmw_readback_bytes;
    summary.intent_deltas_uploaded += tick.tick.intent_deltas_uploaded as u64;
    summary.intent_delta_bytes += tick.tick.intent_delta_bytes;
    summary.tick_drain_ms += tick.tick.drain_ms;
    summary.tick_intent_upload_ms += tick.tick.intent_upload_ms;
    summary.tick_dirty_upload_ms += tick.tick.dirty_upload_ms;
    summary.tick_gpu_pipeline_ms += tick.tick.gpu_pipeline_ms;
    summary.tick_event_readback_ms += tick.tick.event_readback_ms;
    summary.tick_event_readback_bytes += tick.tick.event_readback_bytes;
    if tick.resource_flow_band_dispatched {
        summary.resource_flow_band_dispatches += 1;
    }
    if let Some(mapping) = &tick.mapping {
        summary.mapping_ticks += 1;
        summary.mapping_commitment_events += mapping.threshold_events.len() as u64;
    }
}

fn journal_mapping_commitments(
    mapping_commitments: &mut Vec<MappingCommitmentRecord>,
    tick_index: u64,
    mapping: &crate::simulation_fabric::FabricMappingHotReport,
) {
    mapping_commitments.extend(mapping.threshold_events.iter().cloned().map(|event| {
        MappingCommitmentRecord {
            tick: tick_index,
            event,
        }
    }));
}

impl SimSession {
    /// Installed kind-free owner × specialization observation.
    pub fn owner_specializations(
        &self,
    ) -> Result<Vec<simthing_core::OwnerSpecializationRow>, simthing_core::OwnerResolutionError>
    {
        self.proto
            .root
            .owner_specializations(&self.spec_state.specialization)
    }

    /// Generation-coherent, strictly read-only view over the existing CPU shadow.
    pub fn shadow_view(&self) -> SessionShadowView<'_> {
        SessionShadowView {
            generation: simthing_core::GenerationStamp::new(self.state.anchor_table_generation),
            tick_index: self.coord.tick_index(),
            n_dims: self.coord.n_dims() as usize,
            allocator: &self.proto.allocator,
            values: &self.coord.shadow,
        }
    }

    /// Hot-path cycle — pre-tick enqueue + ordinary tick + RF bands + mapping dispatch.
    fn run_hot_cycle(&mut self) -> Result<FabricHotCycleOutcome, SessionError> {
        // EVENT-GENERATION-STAMP-0: generation authority is the tree day/generation
        // counter. Bind it into sealed emission/threshold mint at the ordinary
        // production step boundary (not an optional side setter).
        self.state
            .bind_production_generation(self.coord.day_index() as u32);
        let resource_flow_pipeline_enabled = self.proto.flags.use_accumulator_resource_flow;
        let mapping_hot = self.mapping.as_mut().map(|m| &mut m.hot);
        let tick_patches = &self.scenario.tick_patches;
        let admitted = &self.spec_state.order_weight_classes;
        let resolved = &self.resolved_order_directives;
        let mut player_gate = |intent: &simthing_feeder::PlayerIntentOverlay| {
            let mut resolved = resolved
                .lock()
                .map_err(|_| "order directive admission registry poisoned".to_string())?;
            crate::order_directive::gate_ingested_player_intent(
                intent.target,
                &intent.overlay,
                admitted,
                &mut resolved,
            )
            .map_err(|error| error.to_string())
        };
        let mut fabric = SimulationFabric::from_hot_parts(HotFabricParts {
            coord: &mut self.coord,
            patcher: &mut self.patcher,
            tx: &self.tx,
            rx: &self.rx,
            registry: &self.proto.registry,
            allocator: &self.proto.allocator,
            pipelines: &self.pipelines,
            state: &mut self.state,
            dt: self.scenario.dt,
            player_intent_gate: Some(&mut player_gate),
        });
        let cycle = run_simulation_fabric_hot_cycle(
            &mut fabric,
            FabricHotCycleParams {
                tick_patches,
                resource_flow_pipeline_enabled,
                mapping: mapping_hot,
            },
        )
        .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        if !cycle.hot.tick.player_intent_rejections.is_empty() {
            return Err(SessionError::PlayerIntentAdmission(
                cycle.hot.tick.player_intent_rejections.join("; "),
            ));
        }
        Ok(cycle)
    }

    pub fn open(scenario: Scenario) -> Result<Self, SessionError> {
        // Admit the semantic projection before the slot allocator, whose
        // residency contract assumes unique logical identities. Malformed
        // trees therefore fail through the typed session door rather than an
        // allocator invariant panic; the admitted projection is rebuilt into
        // the GPU-owned cache during initial sync below.
        simthing_gpu::OverlaySpanProjection::compile(&scenario.root)
            .map_err(simthing_sim::GpuSyncError::from)?;
        let ctx = GpuContext::new_blocking()?;
        let n_dims = scenario.registry.total_columns as u32;
        let mut allocator = simthing_gpu::SlotAllocator::new();
        allocator.populate_from_tree(&scenario.root);
        let n_slots = scenario.n_slots.max(allocator.capacity() as u32);

        let mut state = WorldGpuState::new(ctx, &scenario.registry, n_slots);
        let pipelines = Pipelines::new(&state.ctx);
        let patcher = TransformPatcher::new(n_slots as usize);
        let mut coord = DispatchCoordinator::new(n_slots, n_dims, scenario.ticks_per_day);

        let projected_len = allocator.capacity() * n_dims as usize;
        let mut projected = vec![0.0; projected_len];
        simthing_gpu::project_tree_to_values(
            &scenario.root,
            &scenario.registry,
            &allocator,
            n_dims as usize,
            &mut projected,
        );
        coord.shadow[..projected_len].copy_from_slice(&projected);
        scenario.apply_shadow_seeds(&allocator, &mut coord.shadow, n_dims as usize)?;

        let (tx, rx) = feeder_channel();
        let mut proto = BoundaryProtocol::new(
            SimRuntimeTree::admit(scenario.root.clone()),
            scenario.registry.clone(),
            allocator,
        );
        proto.initial_gpu_sync(&coord, &mut state)?;

        Ok(Self {
            scenario,
            proto,
            coord,
            patcher,
            state,
            pipelines,
            rx,
            tx,
            spec_state: SpecSessionState::new(),
            last_resource_flow_dynamic_enrollment_report: None,
            resource_flow_flag_source:
                crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource::DefaultDisabled,
            resource_flow_execution_profile: ResourceFlowExecutionProfile::DefaultDisabled,
            mapping: None,
            mapping_commitments: Vec::new(),
            execution_posture: simthing_core::ExecutionPosture::Paced,
            resolved_order_directives: Mutex::new(
                crate::order_directive::OrderDirectiveGateState::default(),
            ),
            order_directive_injection_log: Mutex::new(Vec::new()),
        })
    }

    /// Scheduling posture over this session's single kernel (default paced).
    pub fn execution_posture(&self) -> simthing_core::ExecutionPosture {
        self.execution_posture
    }

    /// Select paced or continuous batching. Does not change kernel semantics.
    ///
    /// Continuous `batch_generations == 0` fails closed — never stored, never a
    /// silent no-op success on a later [`Self::run`].
    pub fn set_execution_posture(
        &mut self,
        posture: simthing_core::ExecutionPosture,
    ) -> Result<(), SessionError> {
        posture
            .ensure_admitted()
            .map_err(|e| SessionError::ExecutionPosture(e.to_string()))?;
        self.execution_posture = posture;
        Ok(())
    }

    /// Test harness only: set Resource Flow flag directly (distinct from spec opt-in).
    pub fn override_resource_flow_flag_for_tests(&mut self, enabled: bool) {
        self.proto.flags.use_accumulator_resource_flow = enabled;
        self.resource_flow_flag_source =
            crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource::TestOverride;
    }

    pub fn install_spec_state(&mut self, spec_state: SpecSessionState) -> Result<(), SessionError> {
        self.spec_state = spec_state;
        self.resync_gpu_shape_after_spec_install();
        self.reserve_resource_flow_capacity_budget();
        self.sync_spec_threshold_registrations();
        self.sync_resource_flow_if_enabled()?;
        self.sync_resource_economy_at_install()?;
        // Re-project tree (including entity-hosted Constant PropertyValue seeds)
        // then upload thresholds. No dense install_resolved_values authority.
        self.proto.initial_gpu_sync(&self.coord, &mut self.state)?;
        self.sync_resource_economy_threshold_ops_at_install()?;
        Ok(())
    }

    /// Open-time: upload economy emit_on_threshold regs + arm post-RF need rescan.
    /// Value authority remains tree PropertyValue → project_tree_to_values only.
    fn sync_resource_economy_threshold_ops_at_install(&mut self) -> Result<(), SessionError> {
        let Some(registry) = self.spec_state.resource_economy_registry.as_ref() else {
            if !self.spec_state.resolved_need_bindings.is_empty() {
                crate::need_binding::register_post_rf_need_threshold_rescan(
                    &mut self.state,
                    &self.spec_state.resolved_need_bindings,
                )
                .map_err(|e| {
                    SessionError::ThresholdInstall(format!("post-RF need threshold arm: {e}"))
                })?;
            }
            return Ok(());
        };
        if !registry.registrations.emit_on_threshold.is_empty() {
            let gpu_regs = simthing_gpu::emit_on_threshold_registrations_to_gpu(
                &registry.registrations.emit_on_threshold,
            );
            self.state.ensure_threshold_accumulator(
                simthing_gpu::DEFAULT_THRESHOLD_EMISSION_CAPACITY
                    .max(gpu_regs.len() as u32)
                    .max(1),
            );
            self.state
                .upload_accumulator_threshold_ops(&gpu_regs)
                .map_err(|e| {
                    SessionError::ThresholdInstall(format!("upload emit_on_threshold: {e}"))
                })?;
        }
        if !self.spec_state.resolved_need_bindings.is_empty() {
            crate::need_binding::register_post_rf_need_threshold_rescan(
                &mut self.state,
                &self.spec_state.resolved_need_bindings,
            )
            .map_err(|e| {
                SessionError::ThresholdInstall(format!("post-RF need threshold arm: {e}"))
            })?;
        }
        Ok(())
    }

    fn reserve_resource_flow_capacity_budget(&mut self) {
        let Some(budget) = &self.spec_state.resource_flow_capacity_budget else {
            return;
        };
        let emission_capacity = budget
            .emission_capacity
            .max(budget.threshold_emission_capacity)
            .max(simthing_gpu::DEFAULT_THRESHOLD_EMISSION_CAPACITY);
        self.state.ensure_threshold_accumulator(emission_capacity);
    }

    /// Sync E-11 resource-flow AccumulatorOps when the pipeline flag is enabled.
    pub fn sync_resource_flow_if_enabled(&mut self) -> Result<(), SessionError> {
        let enabled = self.proto.flags.use_accumulator_resource_flow;
        // Production always includes stage projections. DISCONNECT harness uses
        // `harness_resync_resource_flow_without_need_stage_projections`.
        crate::arena_allocation_sync::sync_resource_flow_accumulator(
            &mut self.state,
            &self.proto.registry,
            &self.spec_state.arena_registry,
            &self.spec_state.resolved_gated_rates,
            &self.spec_state.resolved_need_bindings,
            enabled,
        )?;
        Ok(())
    }

    /// **Harness-only DISCONNECT control** — omit Identity stage projections.
    /// Compiled only under the workshop's `rf-test-harness` feature and cannot
    /// be selected by production authoring or a normal driver build.
    #[cfg(feature = "rf-test-harness")]
    pub fn harness_resync_resource_flow_without_need_stage_projections(
        &mut self,
    ) -> Result<(), SessionError> {
        let enabled = self.proto.flags.use_accumulator_resource_flow;
        crate::arena_allocation_sync::sync_resource_flow_accumulator_with_options(
            &mut self.state,
            &self.proto.registry,
            &self.spec_state.arena_registry,
            &self.spec_state.resolved_gated_rates,
            &self.spec_state.resolved_need_bindings,
            enabled,
            false,
        )?;
        Ok(())
    }

    /// Session install: upload when flags allow; never reject populated specs with flags off.
    fn sync_resource_economy_at_install(&mut self) -> Result<(), SessionError> {
        self.sync_resource_economy_internal(false)
    }

    /// Boundary refresh: upload when flags allow; reject populated specs with flags off.
    pub fn sync_resource_economy_if_enabled(&mut self) -> Result<(), SessionError> {
        self.sync_resource_economy_internal(true)
    }

    fn sync_resource_economy_internal(
        &mut self,
        reject_flag_off_populated: bool,
    ) -> Result<(), SessionError> {
        let transfer_enabled = self.proto.flags.use_accumulator_transfer;
        let emission_enabled = self.proto.flags.use_accumulator_emission;
        let uploaded_generation = self.spec_state.resource_economy_uploaded_generation();
        let mut generation = uploaded_generation;
        crate::resource_economy_sync::sync_resource_economy_if_present(
            &mut self.state,
            self.spec_state.resource_economy_registry.as_ref(),
            &mut generation,
            transfer_enabled,
            emission_enabled,
            reject_flag_off_populated,
        )?;
        self.spec_state
            .set_resource_economy_uploaded_generation(generation);
        Ok(())
    }

    fn resync_gpu_shape_after_spec_install(&mut self) {
        let required_slots = self
            .coord
            .n_slots()
            .max(self.proto.allocator.capacity() as u32)
            .max(
                self.spec_state
                    .resource_flow_capacity_budget
                    .as_ref()
                    .map(|budget| budget.gpu_slots)
                    .unwrap_or(0),
            )
            .max(1);
        let required_dims = self.proto.registry.total_columns as u32;

        if required_slots > self.coord.n_slots() {
            self.coord.resize_slots(required_slots);
            self.patcher.resize(required_slots as usize);
        }

        let slots_changed = required_slots > self.state.n_slots;
        let dims_changed = required_dims != self.state.n_dims;
        if slots_changed {
            self.state
                .rebuild_for_slots(required_slots, &self.proto.registry);
        } else if dims_changed {
            self.state.rebuild_for_registry(&self.proto.registry);
        }

        if required_dims != self.coord.n_dims() {
            self.coord.resize_dimensions(required_dims);
        }

        self.coord.shadow.fill(0.0);
        let projected_len = self.proto.allocator.capacity() * required_dims as usize;
        let mut projected = vec![0.0; projected_len];
        self.proto.root.project_to_values(
            &self.proto.registry,
            &self.proto.allocator,
            required_dims as usize,
            &mut projected,
        );
        self.coord.shadow[..projected_len].copy_from_slice(&projected);
    }

    /// Open a session from a scenario and immediately install spec runtime
    /// state compiled from a `GameModeSpec`.
    ///
    /// Composes `SimSession::open` + `crate::install::compile_and_install` +
    /// `install_spec_state`. The scenario sets the GPU sizing (`n_slots`,
    /// `registry`, root); the spec contributes properties, capability trees
    /// (cloned per resolved owner), and scripted events.
    ///
    /// See `docs/adr/game_mode_session_installation.md`.
    pub fn open_from_spec(
        scenario: Scenario,
        game_mode: &GameModeSpec,
    ) -> Result<Self, SessionError> {
        Self::open_from_spec_inner(scenario, game_mode, None)
    }

    /// Open an ordinary production session with caller-admitted PALMA /
    /// Gu-Yang field-sweep products and explicit Triad consumer columns.
    ///
    /// The supplied registrations are appended to the existing
    /// [`crate::mapping_runtime::FirstSliceMappingSession`] chain; no second
    /// executor is constructed. Comparative projections are admitted from the
    /// ordinary-install field plan and assigned to [`SpecSessionState`] before
    /// the same mapping session is opened. The install path supplies no Triad
    /// column defaults.
    pub fn open_from_spec_with_admitted_field_sweeps(
        scenario: Scenario,
        game_mode: &GameModeSpec,
        field_sweep_registrations: Vec<simthing_gpu::FieldSweepRegistration>,
        triad_columns: (
            simthing_core::ColumnIndex,
            simthing_core::ColumnIndex,
            simthing_core::ColumnIndex,
        ),
        comparative_bands: crate::comparative_projection::ComparativeProjectionBands,
        authored_opt_out_reason: Option<&'static str>,
    ) -> Result<Self, SessionError> {
        if field_sweep_registrations.is_empty() {
            return Err(SessionError::Mapping(
                "admitted field-sweep session seam requires at least one registration".into(),
            ));
        }
        Self::open_from_spec_inner(
            scenario,
            game_mode,
            Some((
                field_sweep_registrations,
                triad_columns,
                comparative_bands,
                authored_opt_out_reason,
            )),
        )
    }

    fn open_from_spec_inner(
        scenario: Scenario,
        game_mode: &GameModeSpec,
        admitted_field_sweeps: Option<(
            Vec<simthing_gpu::FieldSweepRegistration>,
            (
                simthing_core::ColumnIndex,
                simthing_core::ColumnIndex,
                simthing_core::ColumnIndex,
            ),
            crate::comparative_projection::ComparativeProjectionBands,
            Option<&'static str>,
        )>,
    ) -> Result<Self, SessionError> {
        let mut session = Self::open(scenario)?;
        // I1: `install_atomic` clones registry/root/allocator before
        // running the install, so a failed install leaves the
        // just-built `BoundaryProtocol` untouched. See
        // `docs/adr/install_clone_then_commit.md`.
        let mut admitted = session.scenario.root.clone();
        let mut spec_state = install_atomic(
            game_mode,
            &session.scenario,
            &mut session.proto.registry,
            &mut admitted,
            &mut session.proto.allocator,
        )?;
        session.proto.root = SimRuntimeTree::admit(admitted);
        let mut field_sweep_registrations = Vec::new();
        if let Some((mut admitted, triad_columns, bands, authored_opt_out_reason)) =
            admitted_field_sweeps
        {
            if !game_mode.mapping_execution_profile.enables_execution()
                || game_mode.region_fields.is_empty()
            {
                return Err(SessionError::Mapping(
                    "admitted field-sweep session seam requires an enabled ordinary mapping profile and at least one region field"
                        .into(),
                ));
            }
            let field_plan = spec_state.field_plan_admission.as_ref().ok_or_else(|| {
                SessionError::Mapping(
                    "admitted field-sweep session seam requires ordinary-install field-plan admission"
                        .into(),
                )
            })?;
            let comparative = crate::comparative_default_birth::admit_comparative_from_field_plan(
                &mut session.proto.registry,
                field_plan,
                triad_columns.0,
                triad_columns.1,
                triad_columns.2,
                bands,
                authored_opt_out_reason,
            )
            .map_err(|error| SessionError::Mapping(error.to_string()))?;
            admitted.extend(comparative.bundle.registrations.iter().cloned());
            spec_state.comparative_projection = Some(comparative);
            spec_state.property_admission = session.proto.registry.property_admission_report();
            field_sweep_registrations = admitted;
        }
        apply_resource_economy_opt_in(&mut session.proto.flags, game_mode);
        session.resource_flow_execution_profile = game_mode.resource_flow_execution_profile;
        session.resource_flow_flag_source =
            resolve_resource_flow_execution(&mut session.proto.flags, game_mode, &spec_state);
        if session.proto.flags.use_accumulator_resource_flow {
            validate_resource_flow_flat_star_execution(game_mode, &spec_state)?;
        }
        session.install_spec_state(spec_state)?;
        session.install_session_mapping(game_mode, &field_sweep_registrations)?;
        Ok(session)
    }

    /// Apply a previously-computed `InstallPreview` to this session,
    /// replacing registry / root / allocator and installing the staged
    /// `SpecSessionState`. The mirror image of `preview_install` — the
    /// Studio "preview then accept" flow lands as:
    ///
    /// ```ignore
    /// let preview = preview_install(
    ///     game_mode, &session.scenario,
    ///     &session.proto.registry, &session.proto.root, &session.proto.allocator,
    /// )?;
    /// // ... inspect `preview` ...
    /// session.apply_install_preview(preview);
    /// ```
    ///
    /// Triggers an `initial_gpu_sync` via `install_spec_state` so the GPU
    /// buffer reflects the new tree structure on the next tick. See
    /// `docs/adr/install_clone_then_commit.md`.
    pub fn apply_install_preview(&mut self, preview: InstallPreview) -> Result<(), SessionError> {
        self.proto.registry = preview.registry;
        self.proto.root = SimRuntimeTree::admit(preview.root);
        self.proto.allocator = preview.allocator;
        self.install_spec_state(preview.state)
    }

    /// CT-3b+4a Line 3: wire the in-loop mapping chain when (and only when)
    /// the game mode authored the explicit profile, one region field, and a
    /// pressure binding. Absence of any piece leaves the session mapping-free;
    /// a half-authored configuration is a hard open error, never a silent skip.
    fn install_session_mapping(
        &mut self,
        game_mode: &GameModeSpec,
        admitted_field_registrations: &[simthing_gpu::FieldSweepRegistration],
    ) -> Result<(), SessionError> {
        if !game_mode.mapping_execution_profile.enables_execution()
            || game_mode.region_fields.is_empty()
        {
            return Ok(());
        }
        if admitted_field_registrations.is_empty() && game_mode.region_fields.len() != 1 {
            return Err(SessionError::Mapping(
                "session-loop mapping v1 integrates exactly one region field".into(),
            ));
        }
        let field = &game_mode.region_fields[0];
        let preview = simthing_spec::compile_region_field_preview(field)
            .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        let Some(commitment) = preview.commitment.clone() else {
            return Err(SessionError::Mapping(
                "session-loop mapping requires an authored commitment threshold".into(),
            ));
        };
        let formula = field.parent_formula.as_ref().ok_or_else(|| {
            SessionError::Mapping("session-loop mapping requires parent_formula".into())
        })?;
        let (Some(weight_pressure), Some(weight_resource)) =
            (formula.weight_pressure, formula.weight_resource)
        else {
            return Err(SessionError::Mapping(
                "session-loop mapping requires authored ai_will_do weights".into(),
            ));
        };
        let Some(binding) = field.pressure_binding.as_ref() else {
            return Err(SessionError::Mapping(
                "session-loop mapping requires a pressure_binding (RF-fed spine)".into(),
            ));
        };
        let (entries, cells) = crate::arena_pressure::compile_arena_pressure_scatter(
            binding,
            &self.scenario,
            &self.proto.registry,
            &self.spec_state.arena_registry,
            self.state.n_dims,
            field,
        )
        .map_err(|e| SessionError::Mapping(format!("{e}")))?;
        let effect = match field.commitment.as_ref().and_then(|c| c.effect.as_ref()) {
            None => None,
            Some(spec) => {
                let targets = self
                    .scenario
                    .install_targets
                    .get(&spec.target_id)
                    .filter(|ids| ids.len() == 1)
                    .ok_or_else(|| {
                        SessionError::Mapping(format!(
                            "commitment effect target `{}` must resolve to exactly one SimThing",
                            spec.target_id
                        ))
                    })?;
                let (namespace, name) =
                    spec.targets_property.split_once("::").ok_or_else(|| {
                        SessionError::Mapping(
                            "commitment effect targets_property must be `namespace::name`".into(),
                        )
                    })?;
                let property_id = self.proto.registry.id_of(namespace, name).ok_or_else(|| {
                    SessionError::Mapping(format!(
                        "commitment effect property `{}` is not registered",
                        spec.targets_property
                    ))
                })?;
                let target = targets[0];
                // The overlay-compile path requires the host to carry the
                // effect property; seed it now and re-sync GPU shape.
                let mut props = std::collections::HashSet::new();
                props.insert(property_id);
                self.proto
                    .root
                    .seed_properties_on_node(target, &props, &self.proto.registry);
                self.proto.initial_gpu_sync(&self.coord, &mut self.state)?;
                let overlay = simthing_core::Overlay {
                    id: simthing_core::OverlayId::new(),
                    kind: simthing_core::OverlayKind::Custom("mapping_commitment".into()),
                    source: simthing_core::OverlaySource::System,
                    origin: self.scenario.root.id,
                    affects: vec![target],
                    transform: simthing_core::PropertyTransformDelta {
                        property_id,
                        sub_field_deltas: spec.sub_field_deltas.clone(),
                    },
                    lifecycle: simthing_core::dispatch_until_dissolved(vec![
                        simthing_core::DissolveCondition::AtSessionEnd,
                    ])
                    .expect("AtSessionEnd is a non-empty authored condition"),
                };
                simthing_core::admit_dispatch_minted_overlay(&overlay)
                    .expect("dispatch-minted overlay admits under Definable Horizon");
                Some(ResolvedCommitmentEffect {
                    consequence: crate::RoutedOverlayDelivery::admit(target, overlay)
                        .map_err(|error| SessionError::Mapping(error.to_string()))?,
                    once: spec.once,
                    fired: false,
                })
            }
        };
        let mapping = if admitted_field_registrations.is_empty() {
            crate::mapping_runtime::FirstSliceMappingSession::open(
                &self.state.ctx,
                game_mode.mapping_execution_profile,
                field,
            )
        } else {
            crate::mapping_runtime::FirstSliceMappingSession::open_with_admitted_field_sweeps(
                &self.state.ctx,
                game_mode.mapping_execution_profile,
                field,
                admitted_field_registrations,
            )
        }
        .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        let scatter = simthing_gpu::IndexedScatterOp::new(&self.state.ctx);
        self.mapping = Some(SessionMappingState {
            hot: MappingHotPathState::new(
                mapping,
                scatter,
                entries,
                cells,
                (weight_pressure, weight_resource),
                commitment,
            ),
            boundary: MappingBoundaryState {
                effect,
                commitments_consumed: 0,
            },
        });
        Ok(())
    }

    /// CT-3b+4a closure: submit journaled commitment crossings through the
    /// shared admitted consequence binding and ordinary boundary channel.
    fn submit_commitment_effects(
        &mut self,
        summary: &mut RunSummary,
    ) -> Result<bool, SessionError> {
        let Some(consumed) = self
            .mapping
            .as_ref()
            .map(|mapping| mapping.boundary.commitments_consumed)
        else {
            return Ok(false);
        };
        let pending_generation = self
            .mapping_commitments
            .get(consumed..)
            .and_then(|pending| pending.last())
            .map(|record| simthing_core::GenerationStamp::new(record.event.generation()));
        let Some(m) = self.mapping.as_mut() else {
            return Ok(false);
        };
        let pending = pending_generation.is_some();
        m.boundary.commitments_consumed = self.mapping_commitments.len();
        if !pending {
            return Ok(false);
        }
        let Some(effect) = m.boundary.effect.as_mut() else {
            return Ok(false);
        };
        if effect.once && effect.fired {
            return Ok(false);
        }
        effect.fired = true;
        effect
            .consequence
            .submit_boundary(pending_generation.expect("pending checked"), &self.tx)
            .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        summary.mapping_commitment_effects_applied += 1;
        Ok(true)
    }

    /// Submit a class-bound operator directive (ORDER-WEIGHT-CLASS-0).
    ///
    /// Resolves the admitted class magnitude into an ordinary
    /// `OverlaySource::Player` Transient overlay and parks it on the existing
    /// player-intent feeder — never a second command channel.
    pub fn submit_order_directive(
        &self,
        req: crate::order_directive::OrderDirectiveRequest,
    ) -> Result<simthing_core::OverlayId, crate::order_directive::OrderDirectiveError> {
        let class_id = req.class_id.clone();
        let (overlay, _) = crate::order_directive::build_order_directive_overlay(
            &self.spec_state.order_weight_classes,
            &req,
        )?;
        let id = overlay.id;
        self.resolved_order_directives
            .lock()
            .map_err(|_| {
                crate::order_directive::OrderDirectiveError::Binding(
                    "order directive admission registry poisoned".into(),
                )
            })?
            .resolved
            .insert(id, class_id);
        if self.tx.submit_player_intent(req.target, overlay).is_err() {
            if let Ok(mut resolved) = self.resolved_order_directives.lock() {
                resolved.resolved.remove(&id);
            }
            return Err(crate::order_directive::OrderDirectiveError::FeederDisconnected);
        }
        self.order_directive_injection_log
            .lock()
            .map_err(|_| {
                crate::order_directive::OrderDirectiveError::Binding(
                    "order directive injection log poisoned".into(),
                )
            })?
            .push(crate::order_directive::OrderDirectiveInjection {
                generation: self.coord.day_index(),
                request: req,
            });
        Ok(id)
    }

    /// Submit a raw Player overlay after the runtime class-magnitude gate.
    ///
    /// Dominant magnitudes that skip [`Self::submit_order_directive`] are rejected.
    pub fn submit_player_intent_gated(
        &self,
        target: simthing_core::SimThingId,
        overlay: simthing_core::Overlay,
    ) -> Result<(), crate::order_directive::OrderDirectiveError> {
        crate::order_directive::gate_raw_player_overlay(
            &overlay,
            &self.spec_state.order_weight_classes,
        )?;
        self.tx
            .submit_player_intent(target, overlay)
            .map_err(|_| crate::order_directive::OrderDirectiveError::FeederDisconnected)
    }

    /// Execute one admitted production hot-cycle (and its boundary if reached).
    ///
    /// Studio live-session bridge / headless multi-tick proofs use this instead of
    /// coarse `run(max_days)`. Reuses the same fabric hot-cycle + boundary machinery
    /// as [`Self::run`]; does not invent a parallel tick path.
    pub fn step_once(&mut self) -> Result<StepOnceOutcome, SessionError> {
        let mut summary = RunSummary::new();
        let boundary = self.step_once_into_summary(&mut summary)?;
        Ok(StepOnceOutcome {
            ticks_run: summary.ticks_run,
            boundaries_run: summary.boundaries_run,
            boundary_reached: boundary,
        })
    }

    /// Run until `max_days` boundaries complete (or scenario max if smaller).
    ///
    /// Under [`simthing_core::ExecutionPosture::Continuous`], generations still
    /// advance through the identical hot-cycle + boundary path; the posture only
    /// batches how many barriers one scheduling call intends to pump. Cap remains
    /// authoritative. Continuous `batch_generations == 0` fails closed — never a
    /// silent `Ok` with zero generations executed.
    pub fn run(&mut self, max_days: u32) -> Result<RunSummary, SessionError> {
        self.execution_posture
            .ensure_admitted()
            .map_err(|e| SessionError::ExecutionPosture(e.to_string()))?;
        let cap = max_days.min(self.scenario.max_days);
        let mut summary = RunSummary::new();

        match self.execution_posture {
            simthing_core::ExecutionPosture::Paced => {
                while summary.boundaries_run < cap as u64 {
                    let _ = self.step_once_into_summary(&mut summary)?;
                }
            }
            simthing_core::ExecutionPosture::Continuous { batch_generations } => {
                // Continuous is a submission pump over the SAME step path.
                // Zero was rejected by ensure_admitted above.
                while summary.boundaries_run < cap as u64 {
                    let batch_cap = (cap as u64).saturating_sub(summary.boundaries_run);
                    let batch = u64::from(batch_generations).min(batch_cap);
                    for _ in 0..batch {
                        let _ = self.step_once_into_summary(&mut summary)?;
                    }
                }
            }
        }

        Ok(summary)
    }

    /// Shared hot-cycle + optional boundary body for [`Self::run`] / [`Self::step_once`].
    fn step_once_into_summary(&mut self, summary: &mut RunSummary) -> Result<bool, SessionError> {
        let cycle = self.run_hot_cycle()?;
        summary.submit_tick_patches_ms += cycle.pre_tick_enqueue_ms;
        accumulate_tick_outcome(summary, &cycle.hot, cycle.hot_step_ms);
        if let Some(mapping) = &cycle.hot.mapping {
            journal_mapping_commitments(&mut self.mapping_commitments, summary.ticks_run, mapping);
        }

        let tick = cycle.hot.tick;
        if !tick.boundary_reached {
            return Ok(false);
        }

        let day = tick.day_index;
        let commitment_effect_submitted = self.submit_commitment_effects(summary)?;
        if !commitment_effect_submitted
            && !self
                .spec_state
                .requires_boundary_tick(&tick.events, self.proto.threshold_registry())
            && self
                .proto
                .can_skip_empty_boundary(&tick.events, &self.patcher)
        {
            summary.boundaries_skipped += 1;
            summary.boundaries_run += 1;
            self.state
                .bind_production_generation((day as u32).saturating_add(1));
            return Ok(true);
        }
        summary.boundary_readback_bytes += self.state.values_len() as u64 * 4;
        let boundary_started = Instant::now();
        let spec_state = &mut self.spec_state;
        let outcome = self.proto.execute_with_boundary_hook(
            tick.events,
            &mut self.patcher,
            &mut self.coord,
            &mut self.state,
            day,
            |ctx| spec_state.run_boundary_handlers(ctx),
        )?;
        summary.boundary_total_ms += boundary_started.elapsed().as_secs_f64() * 1000.0;
        summary.fission_events += outcome.fission.fissions_executed;
        accumulate_boundary_timing(summary, outcome.timing);
        summary.boundary_upload_bytes += outcome.gpu_sync.boundary_upload_bytes;
        summary.boundary_value_rows_uploaded += outcome.gpu_sync.value_rows_uploaded as u64;
        if outcome.gpu_sync.full_value_upload {
            summary.boundary_full_value_uploads += 1;
        }
        summary.overlay_deltas_uploaded += outcome.gpu_sync.overlay_deltas_uploaded as u64;
        summary.threshold_regs_uploaded += outcome.gpu_sync.threshold_regs_uploaded as u64;
        summary.reduction_edges_uploaded += outcome.gpu_sync.reduction_edges as u64;
        summary.reduction_slots_uploaded += outcome.gpu_sync.reduction_slots as u64;
        summary.reduction_depths_total += outcome.gpu_sync.reduction_depths as u64;
        summary.reduction_depths_max = summary
            .reduction_depths_max
            .max(outcome.gpu_sync.reduction_depths);
        summary.boundaries_run += 1;
        self.react_to_fission_clones(&outcome);
        self.react_to_fission_resource_flow_enrollment(&outcome)?;
        self.sync_resource_economy_if_enabled()?;
        // Next day's fused scans stamp the upcoming day index.
        self.state
            .bind_production_generation((day as u32).saturating_add(1));
        Ok(true)
    }

    /// Run a session and write LDJSON replay (snapshot + one frame per boundary).
    pub fn record_to_path(
        &mut self,
        path: &Path,
        max_days: u32,
    ) -> Result<RunSummary, SessionError> {
        let mut file = std::fs::File::create(path)?;
        let cap = max_days.min(self.scenario.max_days);
        let mut summary = RunSummary::new();

        let mut writer = ReplayWriter::new(&mut file);
        writer.write_snapshot(&self.proto.snapshot(0))?;

        // O2 Replay v3: emit a `spec_snapshot` line right after the
        // structural snapshot when the session carries installed spec
        // state. Sim-only readers skip this line via the unknown-kind
        // fall-through in `ReplayReader::next_frame`.
        if !self.spec_state.is_empty() {
            let snap = spec_replay::collect_spec_snapshot(&self.spec_state, 0);
            writer.write_extra(&make_spec_snapshot_record(snap))?;
        }

        while summary.boundaries_run < cap as u64 {
            let cycle = self.run_hot_cycle()?;
            summary.submit_tick_patches_ms += cycle.pre_tick_enqueue_ms;
            accumulate_tick_outcome(&mut summary, &cycle.hot, cycle.hot_step_ms);
            if let Some(mapping) = &cycle.hot.mapping {
                journal_mapping_commitments(
                    &mut self.mapping_commitments,
                    summary.ticks_run,
                    mapping,
                );
            }

            let tick = cycle.hot.tick;
            if tick.boundary_reached {
                let day = tick.day_index;
                let commitment_effect_submitted = self.submit_commitment_effects(&mut summary)?;
                if !commitment_effect_submitted
                    && !self
                        .spec_state
                        .requires_boundary_tick(&tick.events, self.proto.threshold_registry())
                    && self
                        .proto
                        .can_skip_empty_boundary(&tick.events, &self.patcher)
                {
                    let frame = ReplayFrame {
                        day: day as u32,
                        entries: Vec::new(),
                        shadow_values: None,
                        spec_entries: Vec::new(),
                        injection_entries: self.take_order_directive_injections_through(day)?,
                    };
                    writer.write_frame(&frame)?;
                    summary.frames_written += 1;
                    summary.boundaries_skipped += 1;
                    summary.boundaries_run += 1;
                    continue;
                }
                summary.boundary_readback_bytes += self.state.values_len() as u64 * 4;
                let boundary_started = Instant::now();
                // O2 Replay v3: snapshot mutable spec state before the hook
                // runs so we can diff post-boundary and emit `SpecDelta`s.
                let pre_spec = self.spec_state.pre_boundary_snapshot();
                let spec_state = &mut self.spec_state;
                let outcome = self.proto.execute_with_boundary_hook(
                    tick.events,
                    &mut self.patcher,
                    &mut self.coord,
                    &mut self.state,
                    day,
                    |ctx| spec_state.run_boundary_handlers(ctx),
                )?;
                summary.boundary_total_ms += boundary_started.elapsed().as_secs_f64() * 1000.0;
                summary.fission_events += outcome.fission.fissions_executed;
                accumulate_boundary_timing(&mut summary, outcome.timing);
                summary.boundary_upload_bytes += outcome.gpu_sync.boundary_upload_bytes;
                summary.boundary_value_rows_uploaded += outcome.gpu_sync.value_rows_uploaded as u64;
                if outcome.gpu_sync.full_value_upload {
                    summary.boundary_full_value_uploads += 1;
                }
                summary.overlay_deltas_uploaded += outcome.gpu_sync.overlay_deltas_uploaded as u64;
                summary.threshold_regs_uploaded += outcome.gpu_sync.threshold_regs_uploaded as u64;
                summary.reduction_edges_uploaded += outcome.gpu_sync.reduction_edges as u64;
                summary.reduction_slots_uploaded += outcome.gpu_sync.reduction_slots as u64;
                summary.reduction_depths_total += outcome.gpu_sync.reduction_depths as u64;
                summary.reduction_depths_max = summary
                    .reduction_depths_max
                    .max(outcome.gpu_sync.reduction_depths);

                // O2 Replay v3: diff spec state, drain notifications, build
                // `spec_entries` for the frame.
                let notifications = self.spec_state.drain_notifications();
                let spec_deltas =
                    spec_replay::diff_and_emit(&pre_spec, &self.spec_state, notifications);
                let spec_entries = spec_replay::spec_deltas_to_json(&spec_deltas);

                let frame = ReplayFrame {
                    day: day as u32,
                    entries: self.proto.take_delta_log(),
                    shadow_values: Some(self.coord.shadow.clone()),
                    spec_entries,
                    injection_entries: self.take_order_directive_injections_through(day)?,
                };
                writer.write_frame(&frame)?;
                summary.frames_written += 1;
                summary.boundaries_run += 1;
                // S5 follow-up (same as `run`): register capability
                // instances + threshold registrations for fission clones.
                self.react_to_fission_clones(&outcome);
                self.react_to_fission_resource_flow_enrollment(&outcome)?;
                self.sync_resource_economy_if_enabled()?;
            }
        }

        Ok(summary)
    }

    fn take_order_directive_injections_through(
        &self,
        generation: u64,
    ) -> Result<Vec<serde_json::Value>, SessionError> {
        let mut log = self.order_directive_injection_log.lock().map_err(|_| {
            SessionError::PlayerIntentAdmission("order directive injection log poisoned".into())
        })?;
        let mut ready = Vec::new();
        let mut pending = Vec::new();
        for injection in log.drain(..) {
            if injection.generation <= generation {
                ready.push(serde_json::to_value(injection).map_err(|error| {
                    SessionError::PlayerIntentAdmission(format!(
                        "serialize order directive injection: {error}"
                    ))
                })?);
            } else {
                pending.push(injection);
            }
        }
        *log = pending;
        Ok(ready)
    }

    fn sync_spec_threshold_registrations(&mut self) {
        self.proto.set_capability_unlock_registrations(
            self.spec_state.capability_unlock_registrations.clone(),
        );
        self.proto.set_scripted_event_trigger_registrations(
            self.spec_state.scripted_event_trigger_registrations(),
        );
    }

    /// Register `CapabilityTreeInstance`s and threshold registrations for
    /// every capability subtree that fission cloned this boundary
    /// (S5 follow-up — fission-spawned trees otherwise have no thresholds
    /// and unlocks never fire). Re-syncs threshold registrations to the
    /// protocol so the GPU sees them next boundary.
    ///
    /// Returns the count of new instances registered (for telemetry / tests).
    fn react_to_fission_clones(&mut self, outcome: &BoundaryOutcome) -> u32 {
        if outcome.fission.cloned_capability_roots.is_empty() {
            return 0;
        }
        let mut registered = 0u32;
        // Snapshot existing instances so we can iterate without holding a
        // borrow on `self.spec_state` while we insert new ones.
        let source_lookup: HashMap<_, _> = self
            .spec_state
            .capability_instances
            .iter()
            .map(|(_, inst)| (inst.tree_thing_id, inst.clone()))
            .collect();
        for root in &outcome.fission.cloned_capability_roots {
            let Some(source) = source_lookup.get(&root.source_root_id) else {
                continue;
            };
            let Some(tree_slot) = self.proto.allocator.slot_of(root.cloned_root_id) else {
                continue;
            };
            // overlay_id mapping is source → clone. Build by_overlay and
            // overlay_hosts for the clone by translating the source's
            // entries through the mapping. Any overlay in the source not
            // covered by the mapping (shouldn't happen for capability
            // overlays — every overlay is re-stamped during clone) is
            // dropped from the clone's lookup.
            let id_map: HashMap<_, _> = root.overlay_id_pairs.iter().copied().collect();
            let by_overlay: HashMap<_, _> = source
                .by_overlay
                .iter()
                .filter_map(|(old_oid, entry)| {
                    id_map.get(old_oid).map(|new_oid| (*new_oid, entry.clone()))
                })
                .collect();
            // For overlay_hosts, the host of an Owner-targeted overlay was
            // the source owner — for the clone it must be the spawned
            // owner. CapabilityTree hosts were the source tree root → now
            // the cloned root. SessionRoot stays the same.
            let overlay_hosts: HashMap<_, _> = source
                .overlay_hosts
                .iter()
                .filter_map(|(old_oid, host)| {
                    let new_oid = *id_map.get(old_oid)?;
                    let new_host = if *host == source.owner_id {
                        root.spawned_owner_id
                    } else if *host == source.tree_thing_id {
                        root.cloned_root_id
                    } else {
                        // SessionRoot (or unknown — pass through).
                        *host
                    };
                    Some((new_oid, new_host))
                })
                .collect();
            // Thresholds: one per source registration, re-pointed at the
            // cloned tree id. Cheap to construct (no GPU work yet).
            let new_regs: Vec<CapabilityUnlockRegistration> = self
                .spec_state
                .capability_unlock_registrations
                .iter()
                .filter(|reg| reg.sim_thing_id == root.source_root_id)
                .map(|reg| CapabilityUnlockRegistration {
                    sim_thing_id: root.cloned_root_id,
                    property_id: reg.property_id,
                    sub_field: reg.sub_field.clone(),
                    threshold: reg.threshold,
                })
                .collect();

            let Some(definition) = self
                .spec_state
                .capability_definitions
                .get(&source.definition_id)
                .cloned()
            else {
                continue;
            };
            let instance = CapabilityTreeInstance {
                owner_id: root.spawned_owner_id,
                definition_id: source.definition_id,
                tree_thing_id: root.cloned_root_id,
                tree_slot: tree_slot.raw(),
                by_overlay,
                overlay_hosts,
            };
            let state = CapabilityTreeState {
                owner_id: root.spawned_owner_id,
                definition_id: source.definition_id,
                activation_mode_by_entry: HashMap::new(),
                active_by_category: HashMap::new(),
            };
            self.spec_state
                .add_capability_tree_instance(definition, instance, state, new_regs);
            registered += 1;
        }
        if registered > 0 {
            self.sync_spec_threshold_registrations();
        }
        registered
    }

    /// E-2B-5 Policy A: enroll fission-spawned SimThings into the parent's
    /// Resource Flow arenas on the child's existing row.
    pub fn react_to_fission_resource_flow_enrollment(
        &mut self,
        outcome: &BoundaryOutcome,
    ) -> Result<(), SessionError> {
        if outcome.fission.fission_pairs.is_empty()
            || self.spec_state.arena_registry.arenas.is_empty()
        {
            self.last_resource_flow_dynamic_enrollment_report = None;
            return Ok(());
        }
        let report =
            crate::resource_flow_fission_enrollment::react_to_fission_resource_flow_enrollment(
                &outcome.fission,
                &mut self.spec_state.arena_registry,
                &self.proto.allocator,
            );
        let should_sync = report.any_admissions() && self.proto.flags.use_accumulator_resource_flow;
        if !report.admissions.is_empty() || !report.rejections.is_empty() {
            self.last_resource_flow_dynamic_enrollment_report = Some(report);
        } else {
            self.last_resource_flow_dynamic_enrollment_report = None;
        }
        if should_sync {
            self.sync_resource_flow_if_enabled()?;
        }
        Ok(())
    }
}

fn apply_resource_economy_opt_in(
    flags: &mut simthing_sim::PipelineFlags,
    game_mode: &GameModeSpec,
) {
    let mode = game_mode
        .resource_economy
        .as_ref()
        .map(|spec| spec.opt_in_mode)
        .unwrap_or(ResourceEconomyOptInMode::Disabled);

    match mode {
        ResourceEconomyOptInMode::Disabled => {}
        ResourceEconomyOptInMode::TransferOnly => {
            flags.use_accumulator_transfer = true;
        }
        ResourceEconomyOptInMode::EmissionOnly => {
            flags.use_accumulator_eml = true;
            flags.use_accumulator_emission = true;
        }
        ResourceEconomyOptInMode::TransferAndEmission => {
            flags.use_accumulator_transfer = true;
            flags.use_accumulator_eml = true;
            flags.use_accumulator_emission = true;
        }
    }
}

fn resolve_resource_flow_execution(
    flags: &mut simthing_sim::PipelineFlags,
    game_mode: &GameModeSpec,
    spec_state: &SpecSessionState,
) -> crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource {
    use crate::resource_flow_opt_in_telemetry::ResourceFlowFlagSource;

    let opt_in = game_mode
        .resource_flow
        .as_ref()
        .map(|spec| spec.opt_in_mode)
        .unwrap_or(ResourceFlowOptInMode::Disabled);

    match opt_in {
        ResourceFlowOptInMode::FlatStarOptIn => {
            flags.use_accumulator_resource_flow = true;
            ResourceFlowFlagSource::SpecFlatStarOptIn
        }
        ResourceFlowOptInMode::Disabled => {
            if game_mode
                .resource_flow_execution_profile
                .enables_arena_resource_flow()
                && !spec_state.arena_registry.arenas.is_empty()
            {
                flags.use_accumulator_resource_flow = true;
                ResourceFlowFlagSource::ScenarioClassDefaultOn
            } else {
                ResourceFlowFlagSource::DefaultDisabled
            }
        }
    }
}

fn validate_resource_flow_flat_star_execution(
    game_mode: &GameModeSpec,
    spec_state: &SpecSessionState,
) -> Result<(), SessionError> {
    validate_resource_flow_flat_star_opt_in(game_mode, spec_state)
}

fn validate_resource_flow_flat_star_opt_in(
    game_mode: &GameModeSpec,
    spec_state: &SpecSessionState,
) -> Result<(), SessionError> {
    if spec_state.arena_registry.arenas.is_empty() {
        return Err(SessionError::ResourceFlowOptIn(
            "Resource Flow GPU execution requires at least one admitted arena".into(),
        ));
    }
    if let Some(flow) = game_mode.resource_flow.as_ref() {
        for arena in &flow.arenas {
            if arena.wildcard_admission.is_some() {
                return Err(SessionError::ResourceFlowOptIn(format!(
                    "arena `{}` wildcard admission is not supported for flat-star Resource Flow (E-11B deferred)",
                    arena.name
                )));
            }
        }
    }
    for arena in &spec_state.arena_registry.arenas {
        if arena.wildcard_max_expansion.is_some() {
            return Err(SessionError::ResourceFlowOptIn(format!(
                "arena `{}` wildcard expansion is not supported for flat-star Resource Flow",
                arena.name
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod continuous_posture_session_proofs {
    use std::sync::Mutex;

    use super::*;
    use crate::scenario::Scenario;

    static GPU_MUTEX: Mutex<()> = Mutex::new(());

    fn tiny_session(max_days: u32) -> SimSession {
        let scenario = Scenario::map_light(
            "continuous_posture_session_proof".into(),
            1,
            max_days,
            1.0,
            4,
        );
        SimSession::open(scenario).expect("session open requires a supported live GPU")
    }

    #[test]
    fn invalid_overlay_projection_returns_typed_session_error_without_panic() {
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let mut scenario = Scenario::map_light(
            "invalid_overlay_projection_session_proof".into(),
            1,
            1,
            1.0,
            4,
        );
        let duplicate_id = scenario.root.id;
        let mut duplicate = simthing_core::SimThing::new(simthing_core::SimThingKind::Cohort, 0);
        duplicate.id = duplicate_id;
        scenario.root.add_child(duplicate);

        match SimSession::open(scenario) {
            Err(SessionError::GpuSync(simthing_sim::GpuSyncError::OverlayProjection(
                simthing_gpu::DerivedSpanAdmissionError::DuplicateLogicalIdentity(id),
            ))) if id == duplicate_id => {}
            Err(other) => panic!("expected typed duplicate-identity admission error, got {other}"),
            Ok(_) => panic!("invalid overlay projection must fail session construction"),
        }
    }

    #[test]
    fn continuous_zero_batch_fails_closed_on_session_set_and_run() {
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let mut session = tiny_session(3);

        assert!(
            simthing_core::ExecutionPosture::continuous(0).is_err(),
            "admit door rejects zero"
        );
        let zero = simthing_core::ExecutionPosture::Continuous {
            batch_generations: 0,
        };
        let set_err = session
            .set_execution_posture(zero)
            .expect_err("set must reject zero continuous batch");
        assert!(matches!(set_err, SessionError::ExecutionPosture(_)));
        assert!(
            session.execution_posture().is_paced(),
            "rejected set must leave default paced stored"
        );

        // Reach the real run path with an invalid continuous zero that bypassed set:
        // must fail closed — never Ok with zero generations (prior silent success).
        session.execution_posture = zero;
        match session.run(2) {
            Err(SessionError::ExecutionPosture(_)) => {}
            Ok(summary) => panic!(
                "continuous zero must not silently succeed; boundaries_run={}",
                summary.boundaries_run
            ),
            Err(other) => panic!("expected ExecutionPosture error, got {other}"),
        }
    }

    #[test]
    fn paced_default_run_retains_boundary_count_behavior() {
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let mut session = tiny_session(5);
        assert_eq!(
            session.execution_posture(),
            simthing_core::ExecutionPosture::Paced
        );
        let summary = session.run(3).expect("default paced run");
        assert_eq!(
            summary.boundaries_run, 3,
            "paced/default run must advance exactly the requested boundary count"
        );
    }
}
