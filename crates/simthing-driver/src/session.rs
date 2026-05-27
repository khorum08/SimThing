//! GPU session loop — ticks, boundaries, and replay recording.

use std::path::Path;
use std::time::Instant;

use simthing_feeder::FeederWork;
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{GpuContext, Pipelines, WorldGpuState};
use simthing_sim::{BoundaryOutcome, BoundaryProtocol, BoundaryTiming, ReplayFrame, ReplayWriter};
use simthing_spec::{
    CapabilityTreeInstance, CapabilityTreeState, CapabilityUnlockRegistration, GameModeSpec,
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
        let mut proto =
            BoundaryProtocol::new(scenario.root.clone(), scenario.registry.clone(), allocator);
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
        })
    }

    pub fn install_spec_state(&mut self, spec_state: SpecSessionState) -> Result<(), SessionError> {
        self.spec_state = spec_state;
        self.resync_gpu_shape_after_spec_install();
        self.sync_spec_threshold_registrations();
        self.sync_resource_flow_if_enabled()?;
        self.proto.initial_gpu_sync(&self.coord, &mut self.state);
        Ok(())
    }

    /// Sync E-11 resource-flow AccumulatorOps when the pipeline flag is enabled.
    pub fn sync_resource_flow_if_enabled(&mut self) -> Result<(), SessionError> {
        let enabled = self.proto.flags.use_accumulator_resource_flow;
        crate::arena_allocation_sync::sync_resource_flow_accumulator(
            &mut self.state,
            &self.proto.registry,
            &self.spec_state.arena_registry,
            &self.spec_state.arena_participant_scaffold,
            &self.proto.root,
            &self.proto.allocator,
            enabled,
        )?;
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
        simthing_gpu::project_tree_to_values(
            &self.proto.root,
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
        let spec_state = install_atomic(
            game_mode,
            &session.scenario,
            &mut session.proto.registry,
            &mut session.proto.root,
            &mut session.proto.allocator,
        )?;
        session.install_spec_state(spec_state)?;
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
        self.proto.root = preview.root;
        self.proto.allocator = preview.allocator;
        self.install_spec_state(preview.state)
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

            if tick.boundary_reached {
                let day = tick.day_index;
                if !self
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

            if tick.boundary_reached {
                let day = tick.day_index;
                if !self
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
}
