//! GPU session loop — ticks, boundaries, and replay recording.

use std::path::Path;
use std::time::Instant;

use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_feeder::FeederWork;
use simthing_gpu::{GpuContext, Pipelines, WorldGpuState};
use simthing_sim::{BoundaryProtocol, ReplayFrame, ReplayWriter};
use thiserror::Error;

use crate::scenario::Scenario;

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
    pub submit_tick_patches_ms: f64,
    pub boundary_total_ms: f64,
    pub boundaries_skipped: u64,
    pub boundary_readback_bytes: u64,
    pub boundary_upload_bytes: u64,
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
            submit_tick_patches_ms: 0.0,
            boundary_total_ms: 0.0,
            boundaries_skipped: 0,
            boundary_readback_bytes: 0,
            boundary_upload_bytes: 0,
            overlay_deltas_uploaded: 0,
            threshold_regs_uploaded: 0,
            reduction_edges_uploaded: 0,
            reduction_slots_uploaded: 0,
            reduction_depths_total: 0,
            reduction_depths_max: 0,
        }
    }
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
        })
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

            if tick.boundary_reached {
                let day = tick.day_index;
                if self
                    .proto
                    .can_skip_empty_boundary(&tick.events, &self.patcher)
                {
                    summary.boundaries_skipped += 1;
                    summary.boundaries_run += 1;
                    continue;
                }
                summary.boundary_readback_bytes += self.state.values_len() as u64 * 4;
                let boundary_started = Instant::now();
                let outcome = self.proto.execute(
                    tick.events,
                    &mut self.patcher,
                    &mut self.coord,
                    &mut self.state,
                    day,
                );
                summary.boundary_total_ms += boundary_started.elapsed().as_secs_f64() * 1000.0;
                summary.fission_events += outcome.fission.fissions_executed;
                summary.boundary_upload_bytes += outcome.gpu_sync.boundary_upload_bytes;
                summary.overlay_deltas_uploaded += outcome.gpu_sync.overlay_deltas_uploaded as u64;
                summary.threshold_regs_uploaded += outcome.gpu_sync.threshold_regs_uploaded as u64;
                summary.reduction_edges_uploaded += outcome.gpu_sync.reduction_edges as u64;
                summary.reduction_slots_uploaded += outcome.gpu_sync.reduction_slots as u64;
                summary.reduction_depths_total += outcome.gpu_sync.reduction_depths as u64;
                summary.reduction_depths_max =
                    summary.reduction_depths_max.max(outcome.gpu_sync.reduction_depths);
                summary.boundaries_run += 1;
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

            if tick.boundary_reached {
                let day = tick.day_index;
                if self
                    .proto
                    .can_skip_empty_boundary(&tick.events, &self.patcher)
                {
                    let frame = ReplayFrame {
                        day: day as u32,
                        entries: Vec::new(),
                        shadow_values: None,
                    };
                    writer.write_frame(&frame)?;
                    summary.frames_written += 1;
                    summary.boundaries_skipped += 1;
                    summary.boundaries_run += 1;
                    continue;
                }
                summary.boundary_readback_bytes += self.state.values_len() as u64 * 4;
                let boundary_started = Instant::now();
                let outcome = self.proto.execute(
                    tick.events,
                    &mut self.patcher,
                    &mut self.coord,
                    &mut self.state,
                    day,
                );
                summary.boundary_total_ms += boundary_started.elapsed().as_secs_f64() * 1000.0;
                summary.fission_events += outcome.fission.fissions_executed;
                summary.boundary_upload_bytes += outcome.gpu_sync.boundary_upload_bytes;
                summary.overlay_deltas_uploaded += outcome.gpu_sync.overlay_deltas_uploaded as u64;
                summary.threshold_regs_uploaded += outcome.gpu_sync.threshold_regs_uploaded as u64;
                summary.reduction_edges_uploaded += outcome.gpu_sync.reduction_edges as u64;
                summary.reduction_slots_uploaded += outcome.gpu_sync.reduction_slots as u64;
                summary.reduction_depths_total += outcome.gpu_sync.reduction_depths as u64;
                summary.reduction_depths_max =
                    summary.reduction_depths_max.max(outcome.gpu_sync.reduction_depths);

                let frame = ReplayFrame {
                    day: day as u32,
                    entries: self.proto.take_delta_log(),
                    shadow_values: Some(self.coord.shadow.clone()),
                };
                writer.write_frame(&frame)?;
                summary.frames_written += 1;
                summary.boundaries_run += 1;
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
}
