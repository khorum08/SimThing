//! GPU session loop — ticks, boundaries, and replay recording.

use std::path::Path;
use std::time::Instant;

use simthing_feeder::FeederWork;
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
use thiserror::Error;

use crate::install::{install_atomic, InstallError, InstallPreview};
use crate::scenario::Scenario;
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
    #[error("session mapping: {0}")]
    Mapping(String),
    #[error("resource flow opt-in: {0}")]
    ResourceFlowOptIn(String),
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
}

/// CT-3b+4a Line 3: everything the session loop needs to run the admitted
/// RF-fed heatmap chain per tick, GPU-resident end to end.
pub struct SessionMappingState {
    pub mapping: crate::first_slice_mapping_runtime::FirstSliceMappingSession,
    scatter: simthing_gpu::IndexedScatterOp,
    entries: Vec<simthing_gpu::ScatterEntry>,
    cells: Vec<(u32, u32)>,
    weights: (f32, f32),
    commitment: simthing_spec::CompiledFirstSliceCommitmentThreshold,
    effect: Option<ResolvedCommitmentEffect>,
    /// Journal watermark: crossings already considered for effect application.
    commitments_consumed: usize,
}

/// Install-resolved authored commitment consequence (CT-3b+4a closure).
struct ResolvedCommitmentEffect {
    target: simthing_core::SimThingId,
    property_id: simthing_core::SimPropertyId,
    deltas: Vec<(simthing_core::SubFieldRole, simthing_core::TransformOp)>,
    once: bool,
    fired: bool,
}

/// One mapping commitment crossing observed by the session loop.
#[derive(Clone, Debug, PartialEq)]
pub struct MappingCommitmentRecord {
    pub tick: u64,
    pub event: simthing_gpu::ThresholdEvent,
}

impl SimSession {
    pub fn open(scenario: Scenario) -> Result<Self, SessionError> {
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
        proto.initial_gpu_sync(&coord, &mut state);

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
        })
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
        self.sync_spec_threshold_registrations();
        self.sync_resource_flow_if_enabled()?;
        self.sync_resource_economy_at_install()?;
        self.proto.initial_gpu_sync(&self.coord, &mut self.state);
        Ok(())
    }

    /// Sync E-11 resource-flow AccumulatorOps when the pipeline flag is enabled.
    pub fn sync_resource_flow_if_enabled(&mut self) -> Result<(), SessionError> {
        let enabled = self.proto.flags.use_accumulator_resource_flow;
        let root = self.proto.root.clone().into_admitted();
        crate::arena_allocation_sync::sync_resource_flow_accumulator(
            &mut self.state,
            &self.proto.registry,
            &self.spec_state.arena_registry,
            &self.spec_state.arena_participant_scaffold,
            &root,
            &self.proto.allocator,
            &self.spec_state.resolved_gated_rates,
            enabled,
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
        let mut session = Self::open(scenario)?;
        // I1: `install_atomic` clones registry/root/allocator before
        // running the install, so a failed install leaves the
        // just-built `BoundaryProtocol` untouched. See
        // `docs/adr/install_clone_then_commit.md`.
        let mut admitted = session.proto.root.clone().into_admitted();
        let spec_state = install_atomic(
            game_mode,
            &session.scenario,
            &mut session.proto.registry,
            &mut admitted,
            &mut session.proto.allocator,
        )?;
        session.proto.root.replace(admitted);
        apply_resource_economy_opt_in(&mut session.proto.flags, game_mode);
        session.resource_flow_execution_profile = game_mode.resource_flow_execution_profile;
        session.resource_flow_flag_source =
            resolve_resource_flow_execution(&mut session.proto.flags, game_mode);
        if session.proto.flags.use_accumulator_resource_flow {
            validate_resource_flow_flat_star_execution(game_mode, &spec_state)?;
        }
        session.install_spec_state(spec_state)?;
        session.install_session_mapping(game_mode)?;
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
    fn install_session_mapping(&mut self, game_mode: &GameModeSpec) -> Result<(), SessionError> {
        if !game_mode.mapping_execution_profile.enables_execution()
            || game_mode.region_fields.is_empty()
        {
            return Ok(());
        }
        if game_mode.region_fields.len() != 1 {
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
            &self.spec_state.arena_participant_scaffold,
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
                self.proto.initial_gpu_sync(&self.coord, &mut self.state);
                Some(ResolvedCommitmentEffect {
                    target,
                    property_id,
                    deltas: spec.sub_field_deltas.clone(),
                    once: spec.once,
                    fired: false,
                })
            }
        };
        let mapping = crate::first_slice_mapping_runtime::FirstSliceMappingSession::open(
            &self.state.ctx,
            game_mode.mapping_execution_profile,
            field,
        )
        .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        let scatter = simthing_gpu::IndexedScatterOp::new(&self.state.ctx);
        self.mapping = Some(SessionMappingState {
            mapping,
            scatter,
            entries,
            cells,
            weights: (weight_pressure, weight_resource),
            commitment,
            effect,
            commitments_consumed: 0,
        });
        Ok(())
    }

    /// CT-3b+4a closure: convert journaled commitment crossings into the
    /// authored `BoundaryRequest::AttachOverlay` consequence, submitted into
    /// the ordinary boundary channel (drained and applied by the existing
    /// structural machinery). Returns `true` when a request was submitted so
    /// the caller never takes the empty-boundary fast path past it.
    fn submit_commitment_effects(
        &mut self,
        summary: &mut RunSummary,
    ) -> Result<bool, SessionError> {
        let Some(m) = self.mapping.as_mut() else {
            return Ok(false);
        };
        let pending = self.mapping_commitments.len() > m.commitments_consumed;
        m.commitments_consumed = self.mapping_commitments.len();
        if !pending {
            return Ok(false);
        }
        let Some(effect) = m.effect.as_mut() else {
            return Ok(false);
        };
        if effect.once && effect.fired {
            return Ok(false);
        }
        effect.fired = true;
        let overlay = simthing_core::Overlay {
            id: simthing_core::OverlayId::new(),
            kind: simthing_core::OverlayKind::Custom("mapping_commitment".into()),
            source: simthing_core::OverlaySource::System,
            affects: vec![effect.target],
            transform: simthing_core::PropertyTransformDelta {
                property_id: effect.property_id,
                sub_field_deltas: effect.deltas.clone(),
            },
            lifecycle: simthing_core::OverlayLifecycle::Permanent,
        };
        self.tx
            .submit_boundary(simthing_feeder::BoundaryRequest::AttachOverlay {
                target: effect.target,
                overlay,
            })
            .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        summary.mapping_commitment_effects_applied += 1;
        Ok(true)
    }

    /// One in-loop mapping step: on-device pressure scatter, the bounded
    /// stencil + reduce + ai_will_do EML + commitment scan, and journal the
    /// crossings. Entirely GPU-resident; no value readback on this path.
    fn run_mapping_step(&mut self, summary: &mut RunSummary) -> Result<(), SessionError> {
        let Some(m) = self.mapping.as_mut() else {
            return Ok(());
        };
        let ctx = &self.state.ctx;
        m.scatter
            .dispatch(
                ctx,
                &self.state.values,
                m.mapping.stencil_input_buffer(),
                &m.entries,
            )
            .map_err(|e| SessionError::Mapping(format!("{e}")))?;
        m.mapping
            .queue_gpu_seed_cells(&m.cells)
            .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        let report = m
            .mapping
            .tick_with_commitment_spec(
                ctx,
                crate::first_slice_mapping_runtime::FirstSliceTickOptions::hot_path(),
                m.weights,
                &m.commitment,
            )
            .map_err(|e| SessionError::Mapping(format!("{e:?}")))?;
        summary.mapping_ticks += 1;
        summary.mapping_commitment_events += report.threshold_events.len() as u64;
        let tick = summary.ticks_run;
        self.mapping_commitments.extend(
            report
                .threshold_events
                .into_iter()
                .map(|event| MappingCommitmentRecord { tick, event }),
        );
        Ok(())
    }

    /// Run until `max_days` boundaries complete (or scenario max if smaller).
    pub fn run(&mut self, max_days: u32) -> Result<RunSummary, SessionError> {
        let cap = max_days.min(self.scenario.max_days);
        let mut summary = RunSummary::new();

        while summary.boundaries_run < cap as u64 {
            let submit_started = Instant::now();
            self.submit_tick_patches()?;
            summary.submit_tick_patches_ms += submit_started.elapsed().as_secs_f64() * 1000.0;
            let tick_started = Instant::now();
            let tick = self.coord.tick(
                &self.rx,
                &mut self.patcher,
                &self.proto.registry,
                &self.proto.allocator,
                &self.pipelines,
                &mut self.state,
                self.scenario.dt,
            );
            summary.tick_total_ms += tick_started.elapsed().as_secs_f64() * 1000.0;
            summary.ticks_run += 1;
            summary.rmw_rows_synced += tick.rmw_rows_synced as u64;
            summary.rmw_readback_bytes += tick.rmw_readback_bytes;
            summary.intent_deltas_uploaded += tick.intent_deltas_uploaded as u64;
            summary.intent_delta_bytes += tick.intent_delta_bytes;
            summary.tick_drain_ms += tick.drain_ms;
            summary.tick_intent_upload_ms += tick.intent_upload_ms;
            summary.tick_dirty_upload_ms += tick.dirty_upload_ms;
            summary.tick_gpu_pipeline_ms += tick.gpu_pipeline_ms;
            summary.tick_event_readback_ms += tick.event_readback_ms;
            summary.tick_event_readback_bytes += tick.event_readback_bytes;

            // CT-3b+4a Line 3: opt-in GPU work rides the same tick — RF
            // arena bands when the pipeline flag is on, then the admitted
            // mapping chain (scatter → stencil → reduce → EML → commitment).
            if self.proto.flags.use_accumulator_resource_flow
                && self.state.accumulator_resource_flow_active
            {
                self.state.run_resource_flow_bands(
                    self.state.accumulator_resource_flow_bands,
                    self.scenario.dt,
                );
                summary.resource_flow_band_dispatches += 1;
            }
            self.run_mapping_step(&mut summary)?;

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
                    summary.boundaries_skipped += 1;
                    summary.boundaries_run += 1;
                    continue;
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
                );
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
                summary.boundaries_run += 1;
                // S5 follow-up: register capability instances + threshold
                // registrations for any fission-cloned capability subtrees.
                self.react_to_fission_clones(&outcome);
                self.react_to_fission_resource_flow_enrollment(&outcome)?;
                self.sync_resource_economy_if_enabled()?;
            }
        }

        Ok(summary)
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
            let submit_started = Instant::now();
            self.submit_tick_patches()?;
            summary.submit_tick_patches_ms += submit_started.elapsed().as_secs_f64() * 1000.0;
            let tick_started = Instant::now();
            let tick = self.coord.tick(
                &self.rx,
                &mut self.patcher,
                &self.proto.registry,
                &self.proto.allocator,
                &self.pipelines,
                &mut self.state,
                self.scenario.dt,
            );
            summary.tick_total_ms += tick_started.elapsed().as_secs_f64() * 1000.0;
            summary.ticks_run += 1;
            summary.rmw_rows_synced += tick.rmw_rows_synced as u64;
            summary.rmw_readback_bytes += tick.rmw_readback_bytes;
            summary.intent_deltas_uploaded += tick.intent_deltas_uploaded as u64;
            summary.intent_delta_bytes += tick.intent_delta_bytes;
            summary.tick_drain_ms += tick.drain_ms;
            summary.tick_intent_upload_ms += tick.intent_upload_ms;
            summary.tick_dirty_upload_ms += tick.dirty_upload_ms;
            summary.tick_gpu_pipeline_ms += tick.gpu_pipeline_ms;
            summary.tick_event_readback_ms += tick.event_readback_ms;
            summary.tick_event_readback_bytes += tick.event_readback_bytes;

            // CT-3b+4a Line 3: opt-in GPU work rides the same tick — RF
            // arena bands when the pipeline flag is on, then the admitted
            // mapping chain (scatter → stencil → reduce → EML → commitment).
            if self.proto.flags.use_accumulator_resource_flow
                && self.state.accumulator_resource_flow_active
            {
                self.state.run_resource_flow_bands(
                    self.state.accumulator_resource_flow_bands,
                    self.scenario.dt,
                );
                summary.resource_flow_band_dispatches += 1;
            }
            self.run_mapping_step(&mut summary)?;

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
                );
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

    fn submit_tick_patches(&self) -> Result<(), SessionError> {
        for patch in &self.scenario.tick_patches {
            self.tx
                .send(FeederWork::Patch(patch.clone()))
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;
        }
        Ok(())
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
                tree_slot,
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

    /// E-2B-5 Policy A: enroll fission-spawned hosted SimThings into parent's
    /// Resource Flow arenas via arena-root sibling append.
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
        let mut admitted = self.proto.root.clone().into_admitted();
        let report =
            crate::resource_flow_fission_enrollment::react_to_fission_resource_flow_enrollment(
                &outcome.fission,
                &mut self.spec_state.arena_registry,
                &mut self.spec_state.arena_participant_scaffold,
                &mut admitted,
                &self.proto.registry,
                &mut self.proto.allocator,
            );
        self.proto.root.replace(admitted);
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
                .enables_flat_star_resource_flow()
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
    let Some(flow) = game_mode.resource_flow.as_ref() else {
        return Err(SessionError::ResourceFlowOptIn(
            "Resource Flow GPU execution requires authored ResourceFlowSpec".into(),
        ));
    };
    if flow.arenas.is_empty() {
        return Err(SessionError::ResourceFlowOptIn(
            "Resource Flow GPU execution requires at least one arena".into(),
        ));
    }
    for arena in &flow.arenas {
        if arena.wildcard_admission.is_some() {
            return Err(SessionError::ResourceFlowOptIn(format!(
                "arena `{}` wildcard admission is not supported for flat-star Resource Flow (E-11B deferred)",
                arena.name
            )));
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
