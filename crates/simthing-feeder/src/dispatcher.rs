//! Dispatch Coordinator — sequences GPU passes and reads threshold events.
//!
//! Per design_v4.md §11 the Coordinator is one of three feeder sub-threads,
//! the one that "sequences GPU passes 0–7, reads threshold candidates,
//! signals boundary completion." This module is scaffolding for that role:
//! it does not yet spawn its own thread — the eventual top-level driver in
//! `simthing-sim` will drive `tick()` on whatever cadence the time-control
//! UI dictates.
//!
//! ## What a tick does (in order)
//!
//! 1. **Drain the work queue.** Patches fold into GPU intent deltas; boundary
//!    requests get parked on the Patcher.
//! 2. **Upload dirty rows.** Legacy/direct shadow writes still upload one row
//!    per dirty slot; normal tick-time transforms avoid row uploads.
//! 3. **Run the GPU passes.** Pass 0 (snapshot) → Pass 1 (velocity) →
//!    Pass 2 (intensity) → Pass 3 (apply overlays) → Passes 4–6 (bottom-up
//!    reduction) → Pass 7 (threshold scan). Reduction is a no-op until the
//!    boundary uploads topology + per-column rules.
//! 4. **Read events.** Pull the atomic event counter and the sparse
//!    `event_candidates` buffer back to the CPU. Per design_v4.md §12 this
//!    is at most ~3 KB even at endgame scale.
//! 5. **Advance counters.** Bump `tick_in_day`; on rollover, set
//!    `boundary_reached = true` in the outcome. The caller (eventually
//!    `simthing-sim`'s day-boundary protocol) handles the §10 sequence.
//!
//! ## Why intent deltas happen before the snapshot, not after
//!
//! Pass 0 copies `values → previous_values`. Velocity / intensity / threshold
//! all compare current to previous. If we applied patches *after* the
//! snapshot, every threshold registered on a patched cell would fire on the
//! next tick — a phantom crossing caused by the upload, not by simulation.
//! Applying intent deltas first means the patch is absorbed into the
//! previous-state reference frame, exactly the way the CPU evaluator already
//! treats continuous overlays.
//!
//! ## What's still TODO (Week 3+)
//!
//! - Pass 3 overlay-deltas upload is the caller's responsibility today.
//!   The Coordinator does *not* call `build_overlay_deltas` itself because
//!   that requires the live SimThing tree, which is owned by the Tree
//!   Maintainer's domain. The eventual driver will call
//!   `build_overlay_deltas` + `state.upload_overlay_deltas` once per day
//!   boundary (when the tree changes shape) and reuse that buffer across
//!   ticks until the next boundary.
//! - Threshold registry uploads (`state.upload_thresholds`) — same pattern.
//!   Day-boundary work; this module does not own it.

use crate::patcher::{PatcherStats, TransformPatcher};
use crate::work::FeederReceiver;
use simthing_core::DimensionRegistry;
use simthing_gpu::{IntentDelta, Pipelines, SlotAllocator, ThresholdEvent, WorldGpuState};
use std::time::Instant;

// ── Outcome ───────────────────────────────────────────────────────────────────

/// One tick's worth of observable result. Stats are diagnostic;
/// `events` is the authoritative crossings list the simulation must act on.
#[derive(Debug, Default)]
pub struct TickOutcome {
    /// Patcher counters (applied writes, missing targets, etc.).
    pub patcher_stats: PatcherStats,
    /// Slots whose CPU shadow rows were uploaded to the GPU this tick.
    pub uploaded_rows: u32,
    /// Rows synchronously refreshed from GPU before Add/Multiply patch ops.
    pub rmw_rows_synced: u32,
    /// Bytes synchronously read back for RMW row refresh.
    pub rmw_readback_bytes: u64,
    /// Folded GPU intent deltas uploaded for this tick.
    pub intent_deltas_uploaded: u32,
    /// Bytes uploaded to the per-tick GPU intent delta buffer.
    pub intent_delta_bytes: u64,
    /// Wall-clock time spent draining queues and folding intents.
    pub drain_ms: f64,
    /// Wall-clock time spent uploading GPU intent deltas.
    pub intent_upload_ms: f64,
    /// Wall-clock time spent uploading legacy/direct dirty shadow rows.
    pub dirty_upload_ms: f64,
    /// Wall-clock time spent recording/submitting the GPU tick pipeline.
    pub gpu_pipeline_ms: f64,
    /// Wall-clock time spent reading threshold event count/candidates.
    pub event_readback_ms: f64,
    /// Bytes read back for threshold event count/candidates this tick.
    pub event_readback_bytes: u64,
    /// Threshold crossings detected by Pass 7. Order is GPU-nondeterministic
    /// (atomicAdd race). Callers that need a canonical order must sort.
    pub events: Vec<ThresholdEvent>,
    /// True iff this tick's completion rolled the day counter over. The
    /// caller is responsible for executing the §10 boundary sequence
    /// (overlay lifecycle, structural mutations, etc.) before the next tick.
    pub boundary_reached: bool,
    /// Monotonic tick id post-increment. Useful for log correlation.
    pub tick_index: u64,
    /// Monotonic day id; bumps once per `ticks_per_day` ticks.
    pub day_index: u64,
}

// ── Coordinator ───────────────────────────────────────────────────────────────

/// Sequences the per-tick GPU work and maintains the CPU shadow of `values`.
/// Stateless w.r.t. simulation semantics — just orchestration + the shadow.
pub struct DispatchCoordinator {
    /// Row-major `[n_slots × n_dims]` shadow of the GPU `values` buffer.
    /// The Patcher mutates this; `tick()` uploads dirty rows to GPU.
    pub shadow: Vec<f32>,
    n_slots: u32,
    n_dims: u32,
    ticks_per_day: u32,
    tick_in_day: u32,
    tick_counter: u64,
    day_counter: u64,
}

impl DispatchCoordinator {
    /// `shadow` is initialized to zeros. Callers that want to start with a
    /// non-trivial state should write into `coord.shadow` before the first
    /// tick — those rows will be uploaded as part of the first dirty-row
    /// flush (after the caller calls `mark_all_dirty()`).
    pub fn new(n_slots: u32, n_dims: u32, ticks_per_day: u32) -> Self {
        assert!(ticks_per_day > 0, "ticks_per_day must be > 0");
        Self {
            shadow: vec![0.0; (n_slots as usize) * (n_dims as usize)],
            n_slots,
            n_dims,
            ticks_per_day,
            tick_in_day: 0,
            tick_counter: 0,
            day_counter: 0,
        }
    }

    /// Run one tick. See module-level doc for the step order.
    pub fn tick(
        &mut self,
        receiver: &FeederReceiver,
        patcher: &mut TransformPatcher,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
        pipelines: &Pipelines,
        state: &mut WorldGpuState,
        dt: f32,
    ) -> TickOutcome {
        // 1. Drain feeder queue into folded GPU intent deltas.
        let drain_started = Instant::now();
        let feeder_items = receiver.drain_now();
        let ai_items = patcher.drain_ai_now();
        let (patcher_stats, intent_deltas) =
            patcher.apply_collected_as_intents(feeder_items, ai_items, registry, allocator);
        let drain_ms = drain_started.elapsed().as_secs_f64() * 1000.0;
        let intent_deltas_uploaded = intent_deltas.len() as u32;
        let intent_delta_bytes =
            intent_deltas.len() as u64 * std::mem::size_of::<IntentDelta>() as u64;
        let intent_upload_started = Instant::now();
        state.upload_intent_deltas(&intent_deltas);
        let intent_upload_ms = intent_upload_started.elapsed().as_secs_f64() * 1000.0;
        let rmw_rows_synced = 0;
        let rmw_readback_bytes = 0;

        // 2. Upload dirty rows (one write per touched row, coalesced).
        let dirty_upload_started = Instant::now();
        let dirty = patcher.take_dirty_rows();
        let uploaded_rows = dirty.len() as u32;
        for slot in dirty {
            self.upload_row(state, slot);
        }
        let dirty_upload_ms = dirty_upload_started.elapsed().as_secs_f64() * 1000.0;

        // 3. GPU passes (order matters — see module-level doc).
        let gpu_pipeline_started = Instant::now();
        pipelines.run_tick_pipeline(state, dt);
        let gpu_pipeline_ms = gpu_pipeline_started.elapsed().as_secs_f64() * 1000.0;

        // 4. Event readback. Cheap even at endgame scale (~3 KB).
        let event_readback_started = Instant::now();
        let events = if state.n_thresholds == 0 {
            Vec::new()
        } else {
            let count = state.read_event_count();
            if count == 0 {
                Vec::new()
            } else {
                state.read_event_candidates(count)
            }
        };
        let event_readback_bytes = if state.n_thresholds == 0 {
            0
        } else {
            std::mem::size_of::<u32>() as u64
                + events.len() as u64 * std::mem::size_of::<ThresholdEvent>() as u64
        };
        let event_readback_ms = event_readback_started.elapsed().as_secs_f64() * 1000.0;

        // 5. Advance counters.
        self.tick_counter += 1;
        self.tick_in_day += 1;
        let boundary_reached = self.tick_in_day >= self.ticks_per_day;
        if boundary_reached {
            self.tick_in_day = 0;
            self.day_counter += 1;
        }

        TickOutcome {
            patcher_stats,
            uploaded_rows,
            rmw_rows_synced,
            rmw_readback_bytes,
            intent_deltas_uploaded,
            intent_delta_bytes,
            drain_ms,
            intent_upload_ms,
            dirty_upload_ms,
            gpu_pipeline_ms,
            event_readback_ms,
            event_readback_bytes,
            events,
            boundary_reached,
            tick_index: self.tick_counter,
            day_index: self.day_counter,
        }
    }

    /// Write the full shadow to GPU. Use this once after seeding `coord.shadow`
    /// with the projection of the initial SimThing tree, before the first
    /// `tick()`. After that, dirty-row tracking handles incremental uploads.
    pub fn upload_full_shadow(&self, state: &WorldGpuState) {
        assert_eq!(
            self.shadow.len(),
            state.values_len(),
            "shadow length {} != state values length {}",
            self.shadow.len(),
            state.values_len(),
        );
        state
            .ctx
            .queue
            .write_buffer(&state.values, 0, bytemuck::cast_slice(&self.shadow));
    }

    /// Upload one slot's row from the shadow to the GPU. Internal helper.
    pub fn upload_row(&self, state: &WorldGpuState, slot: u32) {
        let n_dims = self.n_dims as usize;
        let base = (slot as usize) * n_dims;
        let row = &self.shadow[base..base + n_dims];
        let offset = (slot as u64) * (n_dims as u64) * 4;
        state
            .ctx
            .queue
            .write_buffer(&state.values, offset, bytemuck::cast_slice(row));
    }

    /// Upload a contiguous block of slot rows `[slot_start..slot_start + count)`
    /// in a single `queue.write_buffer` call. Cheap for callers that have
    /// already coalesced adjacent dirty slots — avoids the per-row driver
    /// overhead that dominates when `count` is large.
    pub fn upload_row_range(&self, state: &WorldGpuState, slot_start: u32, count: u32) {
        if count == 0 {
            return;
        }
        let n_dims = self.n_dims as usize;
        let base = (slot_start as usize) * n_dims;
        let span = (count as usize) * n_dims;
        let rows = &self.shadow[base..base + span];
        let offset = (slot_start as u64) * (n_dims as u64) * 4;
        state
            .ctx
            .queue
            .write_buffer(&state.values, offset, bytemuck::cast_slice(rows));
    }

    /// Current monotonic tick id (post-last-tick value).
    pub fn tick_index(&self) -> u64 {
        self.tick_counter
    }
    /// Current day id (post-last-tick value).
    pub fn day_index(&self) -> u64 {
        self.day_counter
    }
    /// Where we are inside the current day, in ticks.
    pub fn tick_in_day(&self) -> u32 {
        self.tick_in_day
    }
    pub fn ticks_per_day(&self) -> u32 {
        self.ticks_per_day
    }

    pub fn n_slots(&self) -> u32 {
        self.n_slots
    }
    pub fn n_dims(&self) -> u32 {
        self.n_dims
    }

    /// Grow the row-major shadow after the allocator's slot high-water mark
    /// exceeds the current GPU/shadow slot capacity. Existing rows keep their
    /// slot indices; newly-created rows are zero-filled.
    pub fn resize_slots(&mut self, new_n_slots: u32) {
        if new_n_slots == self.n_slots {
            return;
        }
        assert!(
            new_n_slots > self.n_slots,
            "slot shrink is not supported: {} -> {}",
            self.n_slots,
            new_n_slots,
        );

        let n_dims = self.n_dims as usize;
        let mut next = vec![0.0; (new_n_slots as usize) * n_dims];
        let rows_to_copy = (self.shadow.len() / n_dims).min(new_n_slots as usize);
        for slot in 0..rows_to_copy {
            let base = slot * n_dims;
            next[base..base + n_dims].copy_from_slice(&self.shadow[base..base + n_dims]);
        }

        self.shadow = next;
        self.n_slots = new_n_slots;
    }

    /// Widen the row-major shadow after a registry dimension expansion.
    /// Existing columns keep their positions because registry columns are
    /// append-only within a session.
    pub fn resize_dimensions(&mut self, new_n_dims: u32) {
        if new_n_dims == self.n_dims {
            return;
        }
        assert!(
            new_n_dims > self.n_dims,
            "dimension shrink is not supported: {} -> {}",
            self.n_dims,
            new_n_dims,
        );

        let old_n_dims = self.n_dims as usize;
        let new_n_dims_usize = new_n_dims as usize;
        let mut next = vec![0.0; (self.n_slots as usize) * new_n_dims_usize];
        for slot in 0..self.n_slots as usize {
            let old_base = slot * old_n_dims;
            let new_base = slot * new_n_dims_usize;
            next[new_base..new_base + old_n_dims]
                .copy_from_slice(&self.shadow[old_base..old_base + old_n_dims]);
        }

        self.shadow = next;
        self.n_dims = new_n_dims;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_counters_roll_at_day_boundary() {
        // Construct a coordinator independently of the GPU — we only need
        // counter behavior here. We can't call `tick()` without a real
        // pipeline + state, so this test exercises the bookkeeping
        // arithmetic directly.
        let coord = DispatchCoordinator::new(2, 6, 3);
        assert_eq!(coord.n_slots(), 2);
        assert_eq!(coord.n_dims(), 6);
        assert_eq!(coord.ticks_per_day(), 3);
        assert_eq!(coord.tick_in_day(), 0);
        assert_eq!(coord.tick_index(), 0);
        assert_eq!(coord.day_index(), 0);
        assert_eq!(coord.shadow.len(), 12);
        assert!(coord.shadow.iter().all(|v| *v == 0.0));
    }

    #[test]
    #[should_panic]
    fn zero_ticks_per_day_panics() {
        let _ = DispatchCoordinator::new(1, 1, 0);
    }

    #[test]
    fn resize_dimensions_preserves_existing_columns() {
        let mut coord = DispatchCoordinator::new(2, 3, 1);
        coord.shadow = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        coord.resize_dimensions(5);

        assert_eq!(coord.n_dims(), 5);
        assert_eq!(
            coord.shadow,
            vec![1.0, 2.0, 3.0, 0.0, 0.0, 4.0, 5.0, 6.0, 0.0, 0.0,]
        );
    }

    #[test]
    fn resize_slots_preserves_existing_shadow_rows() {
        let mut coord = DispatchCoordinator::new(2, 3, 1);
        coord.shadow = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        coord.resize_slots(4);

        assert_eq!(coord.n_slots(), 4);
        assert_eq!(
            coord.shadow,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }
}
