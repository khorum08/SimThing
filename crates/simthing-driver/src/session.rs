//! GPU session loop — ticks, boundaries, and replay recording.

use std::path::Path;

use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
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
        let mut summary = RunSummary {
            ticks_run: 0,
            boundaries_run: 0,
            frames_written: 0,
            fission_events: 0,
            rmw_rows_synced: 0,
            rmw_readback_bytes: 0,
            intent_deltas_uploaded: 0,
            intent_delta_bytes: 0,
        };

        while summary.boundaries_run < cap as u64 {
            let tick = self.coord.tick(
                &self.rx,
                &mut self.patcher,
                &self.proto.registry,
                &self.proto.allocator,
                &self.pipelines,
                &mut self.state,
                self.scenario.dt,
            );
            summary.ticks_run += 1;
            summary.rmw_rows_synced += tick.rmw_rows_synced as u64;
            summary.rmw_readback_bytes += tick.rmw_readback_bytes;
            summary.intent_deltas_uploaded += tick.intent_deltas_uploaded as u64;
            summary.intent_delta_bytes += tick.intent_delta_bytes;

            if tick.boundary_reached {
                let day = tick.day_index;
                let outcome = self.proto.execute(
                    tick.events,
                    &mut self.patcher,
                    &mut self.coord,
                    &mut self.state,
                    day,
                );
                summary.fission_events += outcome.fission.fissions_executed;
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
        let mut summary = RunSummary {
            ticks_run: 0,
            boundaries_run: 0,
            frames_written: 0,
            fission_events: 0,
            rmw_rows_synced: 0,
            rmw_readback_bytes: 0,
            intent_deltas_uploaded: 0,
            intent_delta_bytes: 0,
        };

        let mut writer = ReplayWriter::new(&mut file);
        writer.write_snapshot(&self.proto.snapshot(0))?;

        while summary.boundaries_run < cap as u64 {
            let tick = self.coord.tick(
                &self.rx,
                &mut self.patcher,
                &self.proto.registry,
                &self.proto.allocator,
                &self.pipelines,
                &mut self.state,
                self.scenario.dt,
            );
            summary.ticks_run += 1;
            summary.rmw_rows_synced += tick.rmw_rows_synced as u64;
            summary.rmw_readback_bytes += tick.rmw_readback_bytes;
            summary.intent_deltas_uploaded += tick.intent_deltas_uploaded as u64;
            summary.intent_delta_bytes += tick.intent_delta_bytes;

            if tick.boundary_reached {
                let day = tick.day_index;
                let outcome = self.proto.execute(
                    tick.events,
                    &mut self.patcher,
                    &mut self.coord,
                    &mut self.state,
                    day,
                );
                summary.fission_events += outcome.fission.fissions_executed;

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
}
