//! BoundaryProtocol — the day-boundary orchestrator.
//!
//! Owns the authoritative SimThing tree root and runs the boundary sequence when
//! the feeder signals `boundary_reached`. Steps 1–3 (tick drain, intent upload,
//! GPU tick pipeline + Pass 7 threshold events) are handled by the feeder layer;
//! `BoundaryProtocol::execute` / `execute_with_boundary_hook` handle the CPU
//! boundary path.
//!
//! ## Boundary sequence (`execute_with_boundary_hook`)
//!
//! Verified against live code — do not reorder without updating this header.
//!
//! ```text
//! 0. GPU value readback     — `state.read_values()` into `coord.shadow`
//! 1. Alert extraction       — velocity + aggregate alerts from Pass 7 events
//! 2. Boundary hook (PR 11)  — optional `BoundaryHookContext` callback; driver
//!                             installs spec handlers here (post-readback,
//!                             pre-structural). Emits `BoundaryRequest`s into
//!                             the pending structural queue.
//! 3. Overlay lifecycle      — dissolve / expire overlays
//! 4. Property expiry        — threshold-driven + CPU decay paths
//! 5. Fission pre-grow       — slot headroom for projected spawns
//! 6. Fission / fusion         — tree shape + lineage records
//! 7. Lineage maintenance      — append/prune `FissionLineageRecord`s
//! 8. Request drain            — feeder boundary queue + player/AI intent overlays
//! 9. AddChild pre-grow        — slot headroom for structural adds
//! 10. Structural mutations    — `apply_structural_mutations` (hook + feeder requests)
//! 11. Dimension rebuild       — registry column growth if needed
//! 12. Final slot capacity     — ensure GPU/shadow sized to allocator
//! 13. GPU buffer sync         — `gpu_sync::sync_gpu_buffers` (threshold/reduction
//!                               rebuild or B2 append paths)
//! 14. Delta log               — boundary outcome → replay entries
//! ```
//!
//! `BoundaryProtocol::execute` is a thin wrapper that calls
//! `execute_with_boundary_hook` with a no-op hook. `BoundaryHookContext` uses
//! only sim/core/feeder/gpu types so this crate stays independent of
//! `simthing-spec`. External capability unlock and scripted-event threshold
//! registrations are stored as feeder-level vecs and included in full threshold
//! rebuilds during GPU sync.

use simthing_core::{
    DecayBehavior, DimensionRegistry, OverlayLifecycle, SimPropertyId, SimThing, SimThingId,
};
use simthing_feeder::{
    BoundaryRequest, CapabilityUnlockRegistration, DispatchCoordinator, MaintainerOutcome,
    ScriptedEventTriggerRegistration, TransformPatcher,
};
#[cfg(debug_assertions)]
use simthing_gpu::build_topology;
use simthing_gpu::{
    build_column_rule_descriptors, encode_column_rules, SlotAllocator, ThresholdEvent,
    ThresholdRegistration, TopologyState, WorldGpuState, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};

use crate::delta_log::{entries_from_outcome, BoundaryDeltaEntry};
use crate::fission::{
    is_capability_container, resolve_fission_fusion, FissionLineageRecord, FissionOutcome,
};
use crate::gpu_sync::{sync_gpu_buffers, GpuSyncOutcome};
use crate::observability::{observe, ObservabilityReport, ObserveFidelity};
use crate::overlay_lifecycle::{resolve_overlay_lifecycle, LifecycleOutcome};
use crate::property_expiry::{resolve_property_expiry, ExpiryOutcome};
use crate::reduced_field::ReducedField;
use crate::threshold_registry::{
    AggregateAlertEvent, AggregateAlertRegistration, ThresholdBuilder, ThresholdRegistry,
    ThresholdSemantic, VelocityAlertEvent, VelocityAlertRegistration,
};
use crate::tree_index::{build_node_paths, node_at_path};
use crate::tree_mutation::apply_structural_mutations;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Default)]
pub struct BoundaryTiming {
    pub value_readback_ms: f64,
    pub alert_collect_ms: f64,
    pub lifecycle_ms: f64,
    pub expiry_ms: f64,
    pub pregrow_fission_ms: f64,
    pub fission_ms: f64,
    pub lineage_ms: f64,
    pub request_drain_ms: f64,
    pub pregrow_add_child_ms: f64,
    pub structural_ms: f64,
    pub dimension_rebuild_ms: f64,
    pub final_capacity_ms: f64,
    pub gpu_sync_ms: f64,
    pub delta_log_ms: f64,
}

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
    pub player_intents_attached: u32,
    pub ai_intents_attached: u32,
    pub velocity_alerts: Vec<VelocityAlertEvent>,
    pub aggregate_alerts: Vec<AggregateAlertEvent>,
    pub timing: BoundaryTiming,
}

/// Spec/session extension point run after canonical GPU value readback and
/// before the normal boundary mutation sequence. This type deliberately uses
/// only sim/core/feeder/gpu concepts so `simthing-sim` stays independent of
/// `simthing-spec`.
pub struct BoundaryHookContext<'a> {
    pub events: &'a [ThresholdEvent],
    pub threshold_registry: &'a ThresholdRegistry,
    pub registry: &'a DimensionRegistry,
    pub allocator: &'a SlotAllocator,
    pub shadow: &'a mut [f32],
    pub n_dims: usize,
    pub requests: &'a mut Vec<BoundaryRequest>,
}

#[derive(Clone, Copy, Debug)]
pub struct PipelineFlags {
    pub use_accumulator_threshold_scan: bool,
    pub use_accumulator_intent: bool,
    /// C-3/C-4 compatibility flag: routes full Add/Multiply/Set overlay batches
    /// through the AccumulatorOp OrderBand planner. The name is retained for
    /// compatibility with the staged C-3 Add-only migration; after C-4, the flag
    /// no longer means Add-only. When false, legacy Pass 3 remains the runtime
    /// path and oracle until S-3 deletion.
    pub use_accumulator_overlay_add: bool,
    /// S-4: AccumulatorOp reduction (Mean/WeightedMean/Sum/Max/Min/First).
    /// Requires `use_accumulator_reduction_exact` when enabled.
    pub use_accumulator_reduction_soft: bool,
    /// S-4: full AccumulatorOp reduction path; must be enabled with soft flag.
    pub use_accumulator_reduction_exact: bool,
    /// C-7: routes GovernedPair velocity integration through AccumulatorOp.
    /// Legacy velocity_integration.wgsl remains flag-off/oracle until velocity sunset.
    pub use_accumulator_velocity: bool,
}

impl Default for PipelineFlags {
    fn default() -> Self {
        Self {
            use_accumulator_threshold_scan: false,
            use_accumulator_intent: false,
            use_accumulator_overlay_add: false,
            use_accumulator_reduction_soft: true,
            use_accumulator_reduction_exact: true,
            use_accumulator_velocity: false,
        }
    }
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
    pub flags: PipelineFlags,
    cpu_threshold_registry: ThresholdRegistry,
    velocity_alerts: Vec<VelocityAlertRegistration>,
    aggregate_alerts: Vec<AggregateAlertRegistration>,
    capability_unlocks: Vec<CapabilityUnlockRegistration>,
    scripted_event_triggers: Vec<ScriptedEventTriggerRegistration>,
    threshold_config_revision: u64,
    synced_threshold_config_revision: u64,
    /// Bumped when tree/lifecycle changes can alter `build_overlay_deltas`.
    overlay_compile_revision: u64,
    /// Append-only log of semantic state changes. Each boundary appends its
    /// entries; callers drain with `take_delta_log()`. The serialization
    /// format and playback logic are deferred to the replay system (Week 5).
    delta_log: Vec<BoundaryDeltaEntry>,
    /// Persistent fission lineage records. Each successful fission adds one;
    /// each successful fusion removes one. Records whose parent or child has
    /// been tombstoned (e.g. by `BoundaryRequest::Remove`) are pruned at the
    /// start of each boundary, before any new threshold registrations are
    /// emitted, so that no `FusionTrigger` ever points at a vanished slot.
    fission_lineage: Vec<FissionLineageRecord>,
    /// B2 Approach C: canonical per-slot child layout + per-slot depth.
    /// Refreshed from a full tree walk inside `gpu_sync` when the
    /// reduction topology rebuild path runs; patched in-place by the
    /// boundary when the append-only fission path applies. Stays in
    /// lockstep with the `child_starts` / `child_indices` / `depth_slots`
    /// buffers on `WorldGpuState`.
    cached_topology_state: TopologyState,
}

impl BoundaryProtocol {
    pub fn new(root: SimThing, registry: DimensionRegistry, allocator: SlotAllocator) -> Self {
        Self {
            root,
            registry,
            allocator,
            flags: PipelineFlags::default(),
            cpu_threshold_registry: ThresholdRegistry::new(),
            velocity_alerts: Vec::new(),
            aggregate_alerts: Vec::new(),
            capability_unlocks: Vec::new(),
            scripted_event_triggers: Vec::new(),
            threshold_config_revision: 0,
            synced_threshold_config_revision: 0,
            overlay_compile_revision: 0,
            delta_log: Vec::new(),
            fission_lineage: Vec::new(),
            cached_topology_state: TopologyState::default(),
        }
    }

    fn sync_accumulator_intent_session(&self, state: &mut WorldGpuState) {
        if !self.flags.use_accumulator_intent {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_intent();
            }
            return;
        }
        state.ensure_intent_accumulator();
    }

    fn sync_accumulator_overlay_add_session(&self, state: &mut WorldGpuState) {
        if !self.flags.use_accumulator_overlay_add {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_overlay_add();
            }
            state.set_overlay_add_dispatch(false, 0);
            return;
        }
        state.ensure_overlay_add_accumulator();
    }

    fn sync_accumulator_reduction_soft_session(&self, state: &mut WorldGpuState) {
        if self.flags.use_accumulator_reduction_exact && !self.flags.use_accumulator_reduction_soft {
            panic!("use_accumulator_reduction_exact requires use_accumulator_reduction_soft");
        }
        if self.flags.use_accumulator_reduction_soft && !self.flags.use_accumulator_reduction_exact {
            panic!(
                "S-4: soft-only reduction bridge removed; enable use_accumulator_reduction_exact"
            );
        }
        if !self.flags.use_accumulator_reduction_soft {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_reduction_soft();
            }
            state.set_reduction_soft_dispatch(false, 0);
            state.set_reduction_exact_dispatch(false);
            return;
        }
        state.ensure_reduction_soft_accumulator();
    }

    fn sync_accumulator_velocity_session(&self, state: &mut WorldGpuState) {
        if !self.flags.use_accumulator_velocity {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_velocity();
            }
            state.set_velocity_dispatch(false, 0);
            return;
        }
        state.ensure_velocity_accumulator();
    }

    fn sync_accumulator_threshold_ops(
        &self,
        state: &mut WorldGpuState,
        gpu_regs: &[ThresholdRegistration],
    ) {
        if !self.flags.use_accumulator_threshold_scan {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_threshold();
            }
            return;
        }

        let cap = state.n_thresholds.max(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
        state.ensure_threshold_accumulator(cap);
        state
            .upload_accumulator_threshold_ops(gpu_regs)
            .expect("AccumulatorOp threshold op upload failed");
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
        self.execute_with_boundary_hook(events, patcher, coord, state, day, |_| {})
    }

    /// Run the boundary sequence and allow an upper layer to inject additional
    /// boundary requests from already-read canonical shadow values. The hook is
    /// intentionally generic: callers can run spec handlers here without making
    /// this crate depend on `simthing-spec`.
    pub fn execute_with_boundary_hook<F>(
        &mut self,
        events: Vec<ThresholdEvent>,
        patcher: &mut TransformPatcher,
        coord: &mut DispatchCoordinator,
        state: &mut WorldGpuState,
        day: u64,
        mut hook: F,
    ) -> BoundaryOutcome
    where
        F: FnMut(&mut BoundaryHookContext<'_>),
    {
        let mut out = BoundaryOutcome {
            day,
            ..Default::default()
        };
        let n_dims = coord.n_dims() as usize;
        let mut requests = Vec::new();
        let mut dirty_value_slots = Vec::new();
        let mut force_full_value_upload = false;
        let mut topology_dirty = false;
        let mut threshold_dirty =
            self.threshold_config_revision != self.synced_threshold_config_revision;

        // The CPU shadow reflects only CPU-side patches; integration output
        // from Pass 1/2 lives only on the GPU. Before mutating the shadow
        // at the boundary, pull the canonical GPU values back so our
        // structural mutations (zeroing new rows, expire writebacks, etc.)
        // operate on the correct base — otherwise the eventual
        // `upload_full_shadow` would wipe out a day's worth of integration.
        // Endgame cost: ~3 MB once per boundary; negligible.
        let value_readback_started = Instant::now();
        coord.shadow = state.read_values();
        let needed = coord.n_slots() as usize * n_dims;
        if coord.shadow.len() < needed {
            coord.shadow.resize(needed, 0.0);
        }
        out.timing.value_readback_ms = value_readback_started.elapsed().as_secs_f64() * 1000.0;

        let alert_collect_started = Instant::now();
        out.velocity_alerts = collect_velocity_alerts(&events, &self.cpu_threshold_registry);
        out.aggregate_alerts = collect_aggregate_alerts(&events, &self.cpu_threshold_registry);
        out.timing.alert_collect_ms = alert_collect_started.elapsed().as_secs_f64() * 1000.0;

        {
            let mut hook_ctx = BoundaryHookContext {
                events: &events,
                threshold_registry: &self.cpu_threshold_registry,
                registry: &self.registry,
                allocator: &self.allocator,
                shadow: &mut coord.shadow,
                n_dims,
                requests: &mut requests,
            };
            hook(&mut hook_ctx);
        }

        let boundary_paths = build_node_paths(&self.root);

        // Step 4: Overlay lifecycle — dissolve + expire effects.
        // Mutates coord.shadow directly (apply_expire_effects writes into it).
        let lifecycle_started = Instant::now();
        out.lifecycle = resolve_overlay_lifecycle(
            &mut self.root,
            &self.registry,
            &self.allocator,
            &mut coord.shadow,
            n_dims,
            day as u32,
            Some(&boundary_paths),
        );
        for &(target, _) in &out.lifecycle.dissolved_overlays {
            push_slot_for_id(&self.allocator, target, &mut dirty_value_slots);
        }
        if out.lifecycle.dissolved > 0 {
            self.bump_overlay_compile_revision();
        }
        out.timing.lifecycle_ms = lifecycle_started.elapsed().as_secs_f64() * 1000.0;

        // Step 5: Property expiry (threshold-driven + CPU-side TowardZero/AfterTicks).
        let expiry_started = Instant::now();
        out.expiry = resolve_property_expiry(
            &mut self.root,
            &mut self.registry,
            &self.allocator,
            &coord.shadow,
            n_dims,
            &events,
            &self.cpu_threshold_registry,
            Some(&boundary_paths),
        );
        if !out.expiry.expired.is_empty() {
            threshold_dirty = true;
            topology_dirty = true;
        }
        if out.expiry.properties_removed > 0 || out.expiry.cpu_side_removals > 0 {
            self.bump_overlay_compile_revision();
        }
        out.timing.expiry_ms = expiry_started.elapsed().as_secs_f64() * 1000.0;

        // Pre-grow for possible fission children so seed_fission_child can
        // write the new rows immediately instead of relying on a later
        // semantic projection.
        let fission_headroom = projected_fission_slots(
            &events,
            &self.cpu_threshold_registry,
            &self.root,
            &boundary_paths,
            &self.registry,
        );
        let pregrow_fission_started = Instant::now();
        // Growth no longer forces a full value upload: `rebuild_for_slots`
        // preserves existing GPU buffer contents via copy_buffer_to_buffer,
        // and the freshly-grown region is zero-initialized in both the GPU
        // buffer (wgpu default) and the CPU shadow (resize fill). Fission
        // children are tracked individually via `fission_pairs` further down.
        let grew_for_fission = self.ensure_slot_capacity(
            self.allocator.capacity() + fission_headroom,
            patcher,
            coord,
            state,
        );
        topology_dirty |= grew_for_fission;
        if grew_for_fission {
            self.bump_overlay_compile_revision();
        }
        out.timing.pregrow_fission_ms = pregrow_fission_started.elapsed().as_secs_f64() * 1000.0;

        // Step 6: Fission and fusion. Spawns new SimThings + allocates slots.
        // Reads from shadow for secondary-condition checks and seeds newly
        // fissioned children from the parent's current GPU row.
        // Lifecycle/expiry do not change tree shape — reuse the same index.
        let fission_started = Instant::now();
        out.fission = resolve_fission_fusion(
            &mut self.root,
            &boundary_paths,
            &self.registry,
            &mut self.allocator,
            &events,
            &self.cpu_threshold_registry,
            &mut coord.shadow,
            n_dims,
            day as u32,
        );
        for &(_, child) in &out.fission.fission_pairs {
            push_slot_for_id(&self.allocator, child, &mut dirty_value_slots);
        }
        for &(parent, _) in &out.fission.fusion_pairs {
            push_slot_for_id(&self.allocator, parent, &mut dirty_value_slots);
        }
        if out.fission.fissions_executed > 0 || out.fission.fusions_executed > 0 {
            topology_dirty = true;
            threshold_dirty = true;
            self.bump_overlay_compile_revision();
        }
        out.timing.fission_ms = fission_started.elapsed().as_secs_f64() * 1000.0;

        // Lineage maintenance:
        //   - New fissions append records; new fusions remove them.
        //   - Records whose parent or child is no longer in the allocator
        //     are pruned (e.g. tombstoned via Remove between boundaries, or
        //     just now via fusion above).
        let lineage_started = Instant::now();
        for rec in &out.fission.lineage_added {
            self.fission_lineage.push(*rec);
        }
        if !out.fission.lineage_removed.is_empty() {
            let removed = &out.fission.lineage_removed;
            self.fission_lineage.retain(|r| !removed.contains(r));
            threshold_dirty = true;
        }
        // Prune tombstoned endpoints from fission/fusion (step 6). Remove
        // requests in step 7/8 tombstone later — pruned again below.
        self.prune_stale_fission_lineage();
        out.timing.lineage_ms = lineage_started.elapsed().as_secs_f64() * 1000.0;

        // Steps 7 + 8: Structural mutations (AddChild, Remove, Reparent,
        // AttachOverlay, AddDimension). One pass through `apply_structural_mutations`
        // handles every BoundaryRequest variant.
        //
        // Player intent overlays route through the same path: each is converted
        // to `BoundaryRequest::AttachOverlay` and appended to the request list
        // so attachment and slot-table updates happen in one consistent pass.
        let request_drain_started = Instant::now();
        requests.extend(patcher.take_boundary_requests());

        let player_intents = patcher.take_player_intents();
        out.player_intents_attached = player_intents.len() as u32;
        for pi in player_intents {
            requests.push(BoundaryRequest::AttachOverlay {
                target: pi.target,
                overlay: pi.overlay,
            });
        }

        let ai_intents = patcher.take_ai_intents();
        out.ai_intents_attached = ai_intents.len() as u32;
        for ai in ai_intents {
            requests.push(BoundaryRequest::AttachOverlay {
                target: ai.target,
                overlay: ai.overlay,
            });
        }
        out.boundary_requests = requests.len() as u32;
        out.timing.request_drain_ms = request_drain_started.elapsed().as_secs_f64() * 1000.0;

        // Pre-grow for AddChild subtrees so apply_structural_mutations can
        // project initialized semantic properties into the dense shadow.
        // Like fission pre-grow, growth here preserves existing GPU data
        // (rebuild_for_slots copies old → new); newly-allocated subtree
        // slots are tracked individually via `out.maintainer.allocated`.
        let pregrow_add_child_started = Instant::now();
        let grew_for_add_child = self.ensure_slot_capacity(
            self.allocator.capacity() + projected_add_child_slots(&requests),
            patcher,
            coord,
            state,
        );
        topology_dirty |= grew_for_add_child;
        if grew_for_add_child {
            self.bump_overlay_compile_revision();
        }
        out.timing.pregrow_add_child_ms =
            pregrow_add_child_started.elapsed().as_secs_f64() * 1000.0;

        // Grow shadow to cover any new slots allocated during fission (step 6)
        // before applying structural mutations. apply_structural_mutations
        // expects values_shadow to be sized for the current allocator capacity.
        let needed = self.allocator.capacity() * n_dims;
        if coord.shadow.len() < needed {
            coord.shadow.resize(needed, 0.0);
        }

        let structural_paths = build_node_paths(&self.root);
        let structural_started = Instant::now();
        out.maintainer = apply_structural_mutations(
            requests,
            &mut self.root,
            &mut self.allocator,
            &mut self.registry,
            &mut coord.shadow,
            n_dims,
            Some(&structural_paths),
        );
        for &id in &out.maintainer.allocated {
            push_slot_for_id(&self.allocator, id, &mut dirty_value_slots);
        }
        if !out.maintainer.tombstoned.is_empty() {
            force_full_value_upload = true;
        }
        if !out.maintainer.allocated.is_empty()
            || !out.maintainer.tombstoned.is_empty()
            || !out.maintainer.reparented.is_empty()
            || !out.maintainer.dimensions_added.is_empty()
        {
            topology_dirty = true;
            threshold_dirty = true;
        }
        if !out.maintainer.allocated.is_empty()
            || !out.maintainer.tombstoned.is_empty()
            || !out.maintainer.reparented.is_empty()
            || !out.maintainer.dimensions_added.is_empty()
            || !out.maintainer.overlays_attached.is_empty()
            || !out.maintainer.overlays_activated.is_empty()
            || !out.maintainer.overlays_suspended.is_empty()
        {
            self.bump_overlay_compile_revision();
        }

        // Remove / reparent tombstones may have invalidated lineage endpoints.
        self.prune_stale_fission_lineage();
        out.timing.structural_ms = structural_started.elapsed().as_secs_f64() * 1000.0;

        let dimension_rebuild_started = Instant::now();
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
            force_full_value_upload = true;
            topology_dirty = true;
            threshold_dirty = true;
            self.bump_overlay_compile_revision();
        } else if !out.maintainer.dimensions_added.is_empty() {
            state.rebuild_for_registry(&self.registry);
            force_full_value_upload = true;
            topology_dirty = true;
            threshold_dirty = true;
            self.bump_overlay_compile_revision();
        }
        out.timing.dimension_rebuild_ms =
            dimension_rebuild_started.elapsed().as_secs_f64() * 1000.0;

        // After structural mutations the allocator may have grown again
        // (AddChild). Resize shadow once more so step 9 covers the full
        // capacity. Growth preserves existing GPU data; any newly-allocated
        // rows are already tracked in `out.maintainer.allocated`.
        let final_capacity_started = Instant::now();
        let grew_final_capacity =
            self.ensure_slot_capacity(self.allocator.capacity(), patcher, coord, state);
        topology_dirty |= grew_final_capacity;
        if grew_final_capacity {
            self.bump_overlay_compile_revision();
        }
        out.timing.final_capacity_ms = final_capacity_started.elapsed().as_secs_f64() * 1000.0;

        // B2 Approach B: append-only threshold rebuild for pure-fission
        // growth boundaries. The full `ThresholdBuilder::build_with_lineage`
        // walk re-derives every registration from scratch (60k entries in
        // `fission_stress`). When the only source of threshold churn is
        // fission spawning new children — no fusions, no expiry, no
        // structural add/remove, no dimension or config change — we can
        // derive registrations only for the new children + new lineage
        // and append them to both the CPU registry and the GPU buffer.
        // Existing event_kind indices stay stable (we only push at the
        // tail), so this is safe with respect to in-flight threshold
        // events resolved via the CPU `ThresholdRegistry` lookup.
        let mut threshold_regs_appended = 0u32;
        let append_eligible = threshold_dirty
            && out.fission.fissions_executed > 0
            && out.fission.fusions_executed == 0
            && out.fission.lineage_removed.is_empty()
            && out.expiry.expired.is_empty()
            && out.maintainer.allocated.is_empty()
            && out.maintainer.tombstoned.is_empty()
            && out.maintainer.dimensions_added.is_empty()
            && out.maintainer.reparented.is_empty()
            && self.threshold_config_revision == self.synced_threshold_config_revision;
        if append_eligible {
            // Reuse `structural_paths` (built once before apply_structural_mutations).
            // In the append-eligible case the maintainer's allocated/tombstoned
            // lists are both empty, so the tree shape — and therefore the
            // index — has not changed since `structural_paths` was built.
            let mut new_regs = Vec::new();
            for &(_, child_id) in &out.fission.fission_pairs {
                if let Some(path) = structural_paths.get(&child_id) {
                    if let Some(child_node) = node_at_path(&self.root, path) {
                        ThresholdBuilder::append_subtree(
                            child_node,
                            &self.registry,
                            &self.allocator,
                            &mut new_regs,
                            &mut self.cpu_threshold_registry,
                        );
                    }
                }
            }
            ThresholdBuilder::append_lineage(
                &self.registry,
                &self.allocator,
                &out.fission.lineage_added,
                &mut new_regs,
                &mut self.cpu_threshold_registry,
            );
            state.append_thresholds(&new_regs);
            threshold_regs_appended = new_regs.len() as u32;
            threshold_dirty = false;
        }

        // B2 Approach C: append-only reduction-topology patch on pure-fission
        // growth boundaries. Eligibility predicate is identical to Approach B
        // (no fusions, no expiry, no add/remove, no dimension/config change),
        // because every condition that breaks one breaks the other:
        // structural mutations reshape the tree, dimension changes invalidate
        // column rules, fusion removes children. The `cached_topology_state`
        // owned by this BoundaryProtocol is the canonical CSR source —
        // `gpu_sync`'s full-rebuild path refreshes it; here we patch it
        // in-place with the new parent→child edges before re-flattening.
        let mut topology_regs_appended = 0u32;
        let mut topology_appended_edges = 0u32;
        let mut topology_appended_depths = 0u32;
        let mut topology_appended_slots = 0u32;
        let topology_append_eligible = topology_dirty
            && out.fission.fissions_executed > 0
            && out.fission.fusions_executed == 0
            && out.fission.lineage_removed.is_empty()
            // S5: cloned capability subtrees introduce parent→child edges
            // INSIDE the spawned child (cloned_root → its descendants).
            // `fission_pairs` only captures (original_parent → new_child);
            // the append path would miss the subtree edges and drift from
            // a full rebuild. Conservative fix: fall back to full rebuild
            // whenever any fission this boundary cloned subtrees.
            && !out.fission.cloned_capability_subtrees
            && out.expiry.expired.is_empty()
            && out.maintainer.allocated.is_empty()
            && out.maintainer.tombstoned.is_empty()
            && out.maintainer.dimensions_added.is_empty()
            && out.maintainer.reparented.is_empty()
            && self.threshold_config_revision == self.synced_threshold_config_revision;
        let topology_full_rebuild_pending = topology_dirty && !topology_append_eligible;
        if topology_append_eligible {
            // Extend cache to cover newly-grown slot capacity.
            self.cached_topology_state
                .ensure_capacity(self.allocator.capacity());
            // Apply each new (parent, child) edge. SlotAllocator hands out
            // monotonically increasing indices, so the new child has the
            // highest slot in the world — `add_child` push preserves the
            // ascending-slot invariant without re-sorting.
            for &(parent_id, child_id) in &out.fission.fission_pairs {
                if let (Some(p), Some(c)) = (
                    self.allocator.slot_of(parent_id),
                    self.allocator.slot_of(child_id),
                ) {
                    self.cached_topology_state.add_child(p, c);
                    topology_regs_appended += 1;
                }
            }
            // Re-flatten cache → CSR + depth buckets, then upload. The same
            // amount of GPU bytes flows as a full rebuild, but the CPU
            // tree walk + sort is replaced by a linear pass over the
            // already-sorted cache.
            let topo = self.cached_topology_state.flatten();
            let descriptors = build_column_rule_descriptors(&self.registry, state.n_dims as usize);
            let rules_u32 = encode_column_rules(&descriptors);
            let mut depth_slots: Vec<u32> = Vec::new();
            let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
            for bucket in &topo.depth_buckets {
                let offset = depth_slots.len() as u32;
                depth_slots.extend_from_slice(bucket);
                depth_ranges.push((offset, bucket.len() as u32));
            }
            // `upload_reduction_topology` requires `child_starts.len() == n_slots + 1`.
            let n_slots = state.n_slots as usize;
            let mut child_starts = topo.child_starts.clone();
            if child_starts.len() < n_slots + 1 {
                let last = *child_starts.last().unwrap_or(&0);
                child_starts.resize(n_slots + 1, last);
            }
            topology_appended_edges = topo.child_indices.len() as u32;
            topology_appended_depths = topo.depth_buckets.len() as u32;
            topology_appended_slots = depth_slots.len() as u32;
            state.upload_reduction_topology(
                &child_starts,
                &topo.child_indices,
                &rules_u32,
                &depth_slots,
                depth_ranges,
            );
            topology_dirty = false;
        }

        // Step 9: Rebuild GPU buffers from current tree + upload shadow.
        let gpu_sync_started = Instant::now();
        let dirty_value_slots = if force_full_value_upload {
            None
        } else {
            Some(dedup_slots(dirty_value_slots))
        };
        let gpu_out = sync_gpu_buffers(
            &self.root,
            &self.registry,
            &self.allocator,
            coord,
            state,
            &self.velocity_alerts,
            &self.aggregate_alerts,
            &self.capability_unlocks,
            &self.scripted_event_triggers,
            &self.fission_lineage,
            dirty_value_slots.as_deref(),
            threshold_dirty,
            topology_dirty,
            self.flags.use_accumulator_overlay_add,
            self.flags.use_accumulator_reduction_soft,
            self.flags.use_accumulator_reduction_exact,
            self.flags.use_accumulator_velocity,
            self.overlay_compile_revision,
            &mut self.cached_topology_state,
        );
        #[cfg(debug_assertions)]
        if topology_full_rebuild_pending {
            debug_assert_topology_cache_matches_tree(
                &self.root,
                &self.allocator,
                &self.cached_topology_state,
            );
        }
        #[cfg(not(debug_assertions))]
        let _ = topology_full_rebuild_pending;
        out.timing.gpu_sync_ms = gpu_sync_started.elapsed().as_secs_f64() * 1000.0;
        // Adopt the new threshold registry for the next day.
        if let Some(new_reg) = gpu_out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
            self.synced_threshold_config_revision = self.threshold_config_revision;
        }
        if let Some(regs) = gpu_out.rebuilt_threshold_regs.as_deref() {
            self.sync_accumulator_threshold_ops(state, regs);
        }
        self.sync_accumulator_intent_session(state);
        self.sync_accumulator_overlay_add_session(state);
        self.sync_accumulator_reduction_soft_session(state);
        self.sync_accumulator_velocity_session(state);
        out.gpu_sync = GpuSyncOutcome {
            overlay_deltas_uploaded: gpu_out.overlay_deltas_uploaded,
            // Sum: gpu_out.threshold_regs_uploaded counts entries written by
            // the full rebuild path (0 when we took the append path);
            // threshold_regs_appended counts entries written by Approach B's
            // tail-append path.
            threshold_regs_uploaded: gpu_out
                .threshold_regs_uploaded
                .saturating_add(threshold_regs_appended),
            new_threshold_registry: None, // moved into self above
            rebuilt_threshold_regs: None,
            // When gpu_sync's full reduction rebuild path ran, these come
            // from gpu_out; when Approach C's append path ran, gpu_out
            // values are 0 and the cached `topology_appended_*` values
            // describe what was uploaded. They are mutually exclusive
            // (set by exactly one of the two paths).
            reduction_depths: gpu_out
                .reduction_depths
                .saturating_add(topology_appended_depths),
            reduction_edges: gpu_out
                .reduction_edges
                .saturating_add(topology_appended_edges),
            reduction_slots: gpu_out
                .reduction_slots
                .saturating_add(topology_appended_slots),
            boundary_upload_bytes: gpu_out.boundary_upload_bytes,
            value_rows_uploaded: gpu_out.value_rows_uploaded,
            full_value_upload: gpu_out.full_value_upload,
        };
        let _ = topology_regs_appended; // count for future telemetry

        let delta_log_started = Instant::now();
        self.delta_log
            .extend(entries_from_outcome(&out, &self.root));
        out.timing.delta_log_ms = delta_log_started.elapsed().as_secs_f64() * 1000.0;

        out
    }

    /// True when a day boundary has no semantic work that requires CPU shadow
    /// authority or GPU buffer rebuilds. Safe empty boundaries can be counted
    /// without reading back `values` or uploading the full shadow again.
    pub fn can_skip_empty_boundary(
        &self,
        events: &[ThresholdEvent],
        patcher: &TransformPatcher,
    ) -> bool {
        events.is_empty()
            && patcher.pending_boundary_work_count() == 0
            && self.threshold_config_revision == self.synced_threshold_config_revision
            && !tree_has_boundary_lifecycle_work(&self.root, &self.registry)
    }

    /// Read-only access to the current threshold registry (for diagnostics).
    pub fn threshold_registry(&self) -> &ThresholdRegistry {
        &self.cpu_threshold_registry
    }

    pub fn register_velocity_alert(&mut self, alert: VelocityAlertRegistration) {
        self.velocity_alerts.push(alert);
        self.threshold_config_revision += 1;
    }

    pub fn register_aggregate_alert(&mut self, alert: AggregateAlertRegistration) {
        self.aggregate_alerts.push(alert);
        self.threshold_config_revision += 1;
    }

    pub fn set_capability_unlock_registrations(
        &mut self,
        registrations: Vec<CapabilityUnlockRegistration>,
    ) {
        self.capability_unlocks = registrations;
        self.threshold_config_revision += 1;
    }

    pub fn set_scripted_event_trigger_registrations(
        &mut self,
        registrations: Vec<ScriptedEventTriggerRegistration>,
    ) {
        self.scripted_event_triggers = registrations;
        self.threshold_config_revision += 1;
    }

    pub fn clear_velocity_alerts(&mut self) {
        self.velocity_alerts.clear();
        self.threshold_config_revision += 1;
    }

    pub fn clear_aggregate_alerts(&mut self) {
        self.aggregate_alerts.clear();
        self.threshold_config_revision += 1;
    }

    pub fn velocity_alerts(&self) -> &[VelocityAlertRegistration] {
        &self.velocity_alerts
    }

    pub fn aggregate_alerts(&self) -> &[AggregateAlertRegistration] {
        &self.aggregate_alerts
    }

    /// Read the GPU `output_vectors` buffer back to the CPU as a
    /// `ReducedField` — the post-reduction view of the world at presentation
    /// cadence. Safe to call any time the GPU is idle (typically after
    /// `execute` or between ticks). Leaf rows mirror the post-Pass-3 `values`;
    /// inner-node rows carry per-column reductions over their children.
    pub fn read_reduced_field(&self, state: &WorldGpuState) -> ReducedField {
        ReducedField {
            n_dims: state.n_dims as usize,
            values: state.read_output_vectors(),
        }
    }

    /// Build a read-only observability report for `target` from the CPU shadow.
    ///
    /// Cheap and sufficient after a boundary (shadow was GPU-readback at
    /// `execute` start). Mid-day, numeric values may lag integration on rows
    /// that were not patched — use [`Self::observe_live`] for UI/debug.
    pub fn observe(
        &self,
        coord: &DispatchCoordinator,
        target: SimThingId,
    ) -> Option<ObservabilityReport> {
        self.observe_at(coord, None, target, ObserveFidelity::Shadow)
    }

    /// Like [`Self::observe`], but reads the target's integrated row from GPU.
    ///
    /// One `read_values_row` per call — intended for inspector UI, not per-tick
    /// batch queries.
    pub fn observe_live(
        &self,
        coord: &DispatchCoordinator,
        state: &WorldGpuState,
        target: SimThingId,
    ) -> Option<ObservabilityReport> {
        self.observe_at(coord, Some(state), target, ObserveFidelity::GpuRow)
    }

    fn observe_at(
        &self,
        coord: &DispatchCoordinator,
        state: Option<&WorldGpuState>,
        target: SimThingId,
        fidelity: ObserveFidelity,
    ) -> Option<ObservabilityReport> {
        let slot = self.allocator.slot_of(target)?;
        let n_dims = coord.n_dims() as usize;
        match fidelity {
            ObserveFidelity::Shadow => {
                let base = slot as usize * n_dims;
                let row = &coord.shadow[base..base + n_dims];
                observe(&self.root, &self.registry, &self.allocator, row, target)
            }
            ObserveFidelity::GpuRow => {
                let state = state?;
                let row = state.read_values_row(slot);
                observe(&self.root, &self.registry, &self.allocator, &row, target)
            }
        }
    }

    /// All delta entries accumulated since the last `take_delta_log` call.
    /// Entries are in boundary step order within each day, and days are
    /// appended in chronological order.
    pub fn delta_log(&self) -> &[BoundaryDeltaEntry] {
        &self.delta_log
    }

    /// Drain the accumulated delta log. Returns all entries since the last
    /// call and empties the internal buffer. The caller (replay writer) is
    /// responsible for associating entries with the correct day number.
    pub fn take_delta_log(&mut self) -> Vec<BoundaryDeltaEntry> {
        std::mem::take(&mut self.delta_log)
    }

    /// Capture an initial-state snapshot for the replay writer. Should be
    /// called once at session start, before any ticks, so that the recording
    /// has a baseline tree + registry to replay deltas against.
    ///
    /// Includes the current `fission_lineage` so that `ReplayDriver` can
    /// re-register `FusionTrigger` thresholds for fissions that occurred
    /// before recording started.
    pub fn snapshot(&self, day: u32) -> crate::replay::ReplaySnapshot {
        crate::replay::ReplaySnapshot {
            day,
            root: self.root.clone(),
            registry: self.registry.clone(),
            fission_lineage: self.fission_lineage.clone(),
        }
    }

    /// Manually seed the GPU threshold registry at session start (before any
    /// ticks). Normally called once after constructing the protocol, so that
    /// Pass 7 has registrations from tick 1 onward.
    pub fn initial_gpu_sync(&mut self, coord: &DispatchCoordinator, state: &mut WorldGpuState) {
        let out = sync_gpu_buffers(
            &self.root,
            &self.registry,
            &self.allocator,
            coord,
            state,
            &self.velocity_alerts,
            &self.aggregate_alerts,
            &self.capability_unlocks,
            &self.scripted_event_triggers,
            &self.fission_lineage,
            None,
            true,
            true,
            self.flags.use_accumulator_overlay_add,
            self.flags.use_accumulator_reduction_soft,
            self.flags.use_accumulator_reduction_exact,
            self.flags.use_accumulator_velocity,
            self.overlay_compile_revision,
            &mut self.cached_topology_state,
        );
        if let Some(new_reg) = out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
            self.synced_threshold_config_revision = self.threshold_config_revision;
        }
        if let Some(regs) = out.rebuilt_threshold_regs.as_deref() {
            self.sync_accumulator_threshold_ops(state, regs);
        }
        self.sync_accumulator_intent_session(state);
        self.sync_accumulator_overlay_add_session(state);
        self.sync_accumulator_reduction_soft_session(state);
        self.sync_accumulator_velocity_session(state);
    }

    /// Read-only access to the persistent fission lineage. Useful for tests
    /// and observability. Mutation goes through `execute` (fission adds,
    /// fusion / tombstone removes).
    pub fn fission_lineage(&self) -> &[FissionLineageRecord] {
        &self.fission_lineage
    }

    pub fn overlay_compile_revision(&self) -> u64 {
        self.overlay_compile_revision
    }

    pub fn bump_overlay_compile_revision_for_test(&mut self) {
        self.bump_overlay_compile_revision();
    }

    fn bump_overlay_compile_revision(&mut self) {
        self.overlay_compile_revision = self.overlay_compile_revision.wrapping_add(1);
    }

    /// Test helper (S5): whether cached reduction topology matches a fresh tree
    /// walk. After boundary execute, CSR from Approach C append must agree
    /// with `TopologyState::build` when append is used.
    #[doc(hidden)]
    pub fn reduction_topology_matches_tree(&self) -> bool {
        let direct = TopologyState::build(&self.root, &self.allocator).flatten();
        let via_cache = self.cached_topology_state.flatten();
        direct.child_starts == via_cache.child_starts
            && direct.child_indices == via_cache.child_indices
            && direct.depth_buckets == via_cache.depth_buckets
    }

    /// Drop lineage records whose parent or child no longer has a live slot.
    fn prune_stale_fission_lineage(&mut self) {
        let allocator = &self.allocator;
        self.fission_lineage.retain(|r| {
            allocator.slot_of(r.parent_id).is_some() && allocator.slot_of(r.child_id).is_some()
        });
    }

    fn ensure_slot_capacity(
        &self,
        required: usize,
        patcher: &mut TransformPatcher,
        coord: &mut DispatchCoordinator,
        state: &mut WorldGpuState,
    ) -> bool {
        if required as u32 <= coord.n_slots() {
            return false;
        }

        let mut new_n_slots = coord.n_slots().max(1);
        while new_n_slots < required as u32 {
            new_n_slots = new_n_slots
                .checked_mul(2)
                .expect("slot capacity overflow while growing GPU state");
        }

        coord.resize_slots(new_n_slots);
        patcher.resize(new_n_slots as usize);
        state.rebuild_for_slots(new_n_slots, &self.registry);
        true
    }
}

fn push_slot_for_id(allocator: &SlotAllocator, id: SimThingId, slots: &mut Vec<u32>) {
    if let Some(slot) = allocator.slot_of(id) {
        slots.push(slot);
    }
}

fn dedup_slots(mut slots: Vec<u32>) -> Vec<u32> {
    let mut seen = HashSet::new();
    slots.retain(|slot| seen.insert(*slot));
    slots
}

fn projected_add_child_slots(requests: &[BoundaryRequest]) -> usize {
    requests
        .iter()
        .map(|req| match req {
            BoundaryRequest::AddChild { child, .. } => subtree_size(child),
            _ => 0,
        })
        .sum()
}

fn projected_fission_slots(
    events: &[ThresholdEvent],
    registry: &ThresholdRegistry,
    root: &SimThing,
    node_paths: &std::collections::HashMap<SimThingId, Vec<usize>>,
    dim_reg: &DimensionRegistry,
) -> usize {
    let mut seen = HashSet::new();
    events
        .iter()
        .filter_map(|event| match registry.get(event.event_kind)? {
            ThresholdSemantic::FissionTrigger {
                sim_thing_id,
                property_id,
                template_idx,
            } => Some((*sim_thing_id, *property_id, *template_idx)),
            _ => None,
        })
        .filter(|(sim_thing_id, _, template_idx)| seen.insert((*sim_thing_id, *template_idx)))
        .map(|(sim_thing_id, property_id, template_idx)| {
            let mut slots = 1;
            if dim_reg.is_active(property_id) {
                let prop = dim_reg.property(property_id);
                if let Some(ft) = prop.fission_templates.get(template_idx) {
                    if ft.template.clone_capability_children {
                        if let Some(parent) = node_paths
                            .get(&sim_thing_id)
                            .and_then(|path| crate::tree_index::node_at_path(root, path))
                        {
                            let container_kinds = &ft.template.capability_container_kinds;
                            slots += parent
                                .children
                                .iter()
                                .filter(|child| {
                                    is_capability_container(&child.kind, container_kinds)
                                })
                                .map(subtree_size)
                                .sum::<usize>();
                        }
                    }
                }
            }
            slots
        })
        .sum()
}

fn tree_has_boundary_lifecycle_work(node: &SimThing, registry: &DimensionRegistry) -> bool {
    if node.overlays.iter().any(|overlay| {
        overlay.is_active()
            && (matches!(overlay.lifecycle, OverlayLifecycle::Transient { .. })
                || registry
                    .try_property(overlay.transform.property_id)
                    .and_then(|prop| prop.on_expire.as_ref())
                    .is_some())
    }) {
        return true;
    }

    if node.properties.keys().any(|pid| {
        registry
            .try_property(*pid)
            .and_then(|prop| prop.decay.as_ref())
            .is_some_and(|decay| {
                matches!(
                    decay,
                    DecayBehavior::TowardZero { .. } | DecayBehavior::AfterTicks { .. }
                )
            })
    }) {
        return true;
    }

    node.children
        .iter()
        .any(|child| tree_has_boundary_lifecycle_work(child, registry))
}

fn subtree_size(node: &SimThing) -> usize {
    1 + node.children.iter().map(subtree_size).sum::<usize>()
}

fn collect_velocity_alerts(
    events: &[ThresholdEvent],
    registry: &ThresholdRegistry,
) -> Vec<VelocityAlertEvent> {
    events
        .iter()
        .filter_map(|event| {
            let ThresholdSemantic::VelocityAlert {
                sim_thing_id,
                property_id,
                sub_field,
            } = registry.get(event.event_kind)?
            else {
                return None;
            };
            Some(VelocityAlertEvent {
                sim_thing_id: *sim_thing_id,
                property_id: *property_id,
                sub_field: sub_field.clone(),
                value: event.value,
            })
        })
        .collect()
}

fn collect_aggregate_alerts(
    events: &[ThresholdEvent],
    registry: &ThresholdRegistry,
) -> Vec<AggregateAlertEvent> {
    events
        .iter()
        .filter_map(|event| {
            let ThresholdSemantic::AggregateAlert {
                sim_thing_id,
                property_id,
                sub_field,
            } = registry.get(event.event_kind)?
            else {
                return None;
            };
            Some(AggregateAlertEvent {
                sim_thing_id: *sim_thing_id,
                property_id: *property_id,
                sub_field: sub_field.clone(),
                value: event.value,
            })
        })
        .collect()
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

/// S3 integrity guard: incremental cache and full-rebuild paths must produce
/// the same CSR as `build_topology` (see `simthing-gpu` reduction tests).
#[cfg(debug_assertions)]
fn debug_assert_topology_cache_matches_tree(
    root: &SimThing,
    allocator: &SlotAllocator,
    cached: &TopologyState,
) {
    let direct = build_topology(root, allocator);
    let via_cache = cached.flatten();
    debug_assert_eq!(
        direct.child_starts, via_cache.child_starts,
        "topology cache child_starts drift"
    );
    debug_assert_eq!(
        direct.child_indices, via_cache.child_indices,
        "topology cache child_indices drift"
    );
    debug_assert_eq!(
        direct.depth_buckets, via_cache.depth_buckets,
        "topology cache depth_buckets drift"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DecayBehavior, Direction, DissolveCondition, FissionTemplate, FissionThreshold, Overlay,
        OverlayId, OverlayKind, OverlaySource, PropertyTransformDelta, SimProperty, SimThingKind,
        SimThingKindTag, SubFieldRole, TransformOp,
    };
    use simthing_feeder::{BoundaryRequest, FeederWork};
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

    fn simple_proto() -> (BoundaryProtocol, TransformPatcher, SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut child = SimThing::new(SimThingKind::Cohort, 0);
        child.add_property(pid, reg.property(pid).default_value());
        root.add_child(child);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let patcher = TransformPatcher::new(alloc.capacity());
        (BoundaryProtocol::new(root, reg, alloc), patcher, pid)
    }

    #[test]
    fn empty_static_boundary_can_skip() {
        let (proto, patcher, _) = simple_proto();
        assert!(proto.can_skip_empty_boundary(&[], &patcher));
    }

    #[test]
    fn boundary_with_events_cannot_skip() {
        let (proto, patcher, _) = simple_proto();
        let events = vec![ThresholdEvent {
            slot: 0,
            col: 0,
            value: 0.5,
            event_kind: 0,
        }];
        assert!(!proto.can_skip_empty_boundary(&events, &patcher));
    }

    #[test]
    fn boundary_with_pending_request_cannot_skip() {
        let (proto, mut patcher, _) = simple_proto();
        let target = proto.root.children[0].id;
        patcher.apply_collected_as_intents(
            vec![FeederWork::Boundary(BoundaryRequest::Remove { target })],
            Vec::new(),
            &proto.registry,
            &proto.allocator,
        );
        assert!(!proto.can_skip_empty_boundary(&[], &patcher));
    }

    #[test]
    fn boundary_with_unsynced_alert_config_cannot_skip() {
        let (mut proto, patcher, pid) = simple_proto();
        let target = proto.root.children[0].id;
        proto.register_velocity_alert(VelocityAlertRegistration {
            sim_thing_id: target,
            property_id: pid,
            sub_field: SubFieldRole::Velocity,
            threshold: -0.1,
            direction: simthing_core::Direction::Falling,
        });
        assert!(!proto.can_skip_empty_boundary(&[], &patcher));
    }

    #[test]
    fn boundary_with_transient_overlay_cannot_skip() {
        let (mut proto, patcher, pid) = simple_proto();
        proto.root.children[0].add_overlay(Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Transient,
            source: OverlaySource::System,
            affects: Vec::new(),
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.4))],
            },
            lifecycle: OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
            },
        });
        assert!(!proto.can_skip_empty_boundary(&[], &patcher));
    }

    #[test]
    fn boundary_with_only_suspended_overlay_can_skip() {
        let (mut proto, patcher, pid) = simple_proto();
        proto.root.children[0].add_overlay(Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Transient,
            source: OverlaySource::System,
            affects: Vec::new(),
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.4))],
            },
            lifecycle: OverlayLifecycle::Suspended {
                when_activated: Box::new(OverlayLifecycle::Transient {
                    dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
                }),
            },
        });
        assert!(proto.can_skip_empty_boundary(&[], &patcher));
    }

    #[test]
    fn projected_fission_slots_counts_cloned_capability_subtrees() {
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Faction,
                fusion_intensity_threshold: 0.8,
                fusion_scar_coefficient: 0.05,
                resolution_label: "resolved".into(),
                clone_capability_children: true,
                capability_container_kinds: vec!["tech_tree".into()],
            },
            secondary: None,
        }];
        let pid = reg.register(prop);

        let mut faction = SimThing::new(SimThingKind::Faction, 0);
        let faction_id = faction.id;
        faction.add_property(pid, reg.property(pid).default_value());
        let mut tech_tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        tech_tree.add_child(SimThing::new(SimThingKind::Custom("tech_leaf".into()), 0));
        faction.add_child(tech_tree);
        faction.add_child(SimThing::new(
            SimThingKind::Custom("ordinary_child".into()),
            0,
        ));

        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(faction);
        let paths = build_node_paths(&root);

        let mut threshold_registry = ThresholdRegistry::new();
        let event_kind = threshold_registry.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: faction_id,
            property_id: pid,
            template_idx: 0,
        });
        let events = vec![ThresholdEvent {
            slot: 0,
            col: 0,
            value: 0.2,
            event_kind,
        }];

        assert_eq!(
            projected_fission_slots(&events, &threshold_registry, &root, &paths, &reg),
            3,
            "one fission child plus two nodes in the cloned tech tree"
        );
    }

    #[test]
    fn boundary_with_cpu_decay_cannot_skip() {
        let (mut proto, patcher, pid) = simple_proto();
        proto.registry.properties[pid.index()].decay =
            Some(DecayBehavior::TowardZero { rate: 0.1 });
        assert!(!proto.can_skip_empty_boundary(&[], &patcher));
    }
}
