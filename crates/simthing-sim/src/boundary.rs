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

use simthing_core::{
    DecayBehavior, DimensionRegistry, OverlayLifecycle, SimPropertyId, SimThing, SimThingId,
};
use simthing_feeder::{BoundaryRequest, DispatchCoordinator, MaintainerOutcome, TransformPatcher};
use simthing_gpu::{SlotAllocator, ThresholdEvent, WorldGpuState};

use crate::delta_log::{entries_from_outcome, BoundaryDeltaEntry};
use crate::fission::{resolve_fission_fusion, FissionLineageRecord, FissionOutcome};
use crate::gpu_sync::{sync_gpu_buffers, GpuSyncOutcome};
use crate::observability::{observe, ObservabilityReport, ObserveFidelity};
use crate::overlay_lifecycle::{resolve_overlay_lifecycle, LifecycleOutcome};
use crate::property_expiry::{resolve_property_expiry, ExpiryOutcome};
use crate::reduced_field::ReducedField;
use crate::threshold_registry::{
    AggregateAlertEvent, AggregateAlertRegistration, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertEvent, VelocityAlertRegistration,
};
use crate::tree_index::build_node_paths;
use crate::tree_mutation::apply_structural_mutations;
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
    cpu_threshold_registry: ThresholdRegistry,
    velocity_alerts: Vec<VelocityAlertRegistration>,
    aggregate_alerts: Vec<AggregateAlertRegistration>,
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
}

impl BoundaryProtocol {
    pub fn new(root: SimThing, registry: DimensionRegistry, allocator: SlotAllocator) -> Self {
        Self {
            root,
            registry,
            allocator,
            cpu_threshold_registry: ThresholdRegistry::new(),
            velocity_alerts: Vec::new(),
            aggregate_alerts: Vec::new(),
            delta_log: Vec::new(),
            fission_lineage: Vec::new(),
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
        events: Vec<ThresholdEvent>,
        patcher: &mut TransformPatcher,
        coord: &mut DispatchCoordinator,
        state: &mut WorldGpuState,
        day: u64,
    ) -> BoundaryOutcome {
        let mut out = BoundaryOutcome {
            day,
            ..Default::default()
        };
        let n_dims = coord.n_dims() as usize;

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
        );
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
        );
        out.timing.expiry_ms = expiry_started.elapsed().as_secs_f64() * 1000.0;

        // Pre-grow for possible fission children so seed_fission_child can
        // write the new rows immediately instead of relying on a later
        // semantic projection.
        let fission_headroom = events.len();
        let pregrow_fission_started = Instant::now();
        self.ensure_slot_capacity(
            self.allocator.capacity() + fission_headroom,
            patcher,
            coord,
            state,
        );
        out.timing.pregrow_fission_ms = pregrow_fission_started.elapsed().as_secs_f64() * 1000.0;

        // Step 6: Fission and fusion. Spawns new SimThings + allocates slots.
        // Reads from shadow for secondary-condition checks and seeds newly
        // fissioned children from the parent's current GPU row.
        let fission_paths = build_node_paths(&self.root);
        let fission_started = Instant::now();
        out.fission = resolve_fission_fusion(
            &mut self.root,
            &fission_paths,
            &self.registry,
            &mut self.allocator,
            &events,
            &self.cpu_threshold_registry,
            &mut coord.shadow,
            n_dims,
            day as u32,
        );
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
        let mut requests = patcher.take_boundary_requests();
        out.boundary_requests = requests.len() as u32;

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
        out.timing.request_drain_ms = request_drain_started.elapsed().as_secs_f64() * 1000.0;

        // Pre-grow for AddChild subtrees so apply_structural_mutations can
        // project initialized semantic properties into the dense shadow.
        let pregrow_add_child_started = Instant::now();
        self.ensure_slot_capacity(
            self.allocator.capacity() + projected_add_child_slots(&requests),
            patcher,
            coord,
            state,
        );
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
        } else if !out.maintainer.dimensions_added.is_empty() {
            state.rebuild_for_registry(&self.registry);
        }
        out.timing.dimension_rebuild_ms =
            dimension_rebuild_started.elapsed().as_secs_f64() * 1000.0;

        // After structural mutations the allocator may have grown again
        // (AddChild). Resize shadow once more so step 9 uploads the full
        // capacity.
        let final_capacity_started = Instant::now();
        self.ensure_slot_capacity(self.allocator.capacity(), patcher, coord, state);
        out.timing.final_capacity_ms = final_capacity_started.elapsed().as_secs_f64() * 1000.0;

        // Step 9: Rebuild GPU buffers from current tree + upload shadow.
        let gpu_sync_started = Instant::now();
        let gpu_out = sync_gpu_buffers(
            &self.root,
            &self.registry,
            &self.allocator,
            coord,
            state,
            &self.velocity_alerts,
            &self.aggregate_alerts,
            &self.fission_lineage,
        );
        out.timing.gpu_sync_ms = gpu_sync_started.elapsed().as_secs_f64() * 1000.0;
        // Adopt the new threshold registry for the next day.
        if let Some(new_reg) = gpu_out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
        }
        out.gpu_sync = GpuSyncOutcome {
            overlay_deltas_uploaded: gpu_out.overlay_deltas_uploaded,
            threshold_regs_uploaded: gpu_out.threshold_regs_uploaded,
            new_threshold_registry: None, // moved into self above
            reduction_depths: gpu_out.reduction_depths,
            reduction_edges: gpu_out.reduction_edges,
            reduction_slots: gpu_out.reduction_slots,
            boundary_upload_bytes: gpu_out.boundary_upload_bytes,
        };

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
            && !tree_has_boundary_lifecycle_work(&self.root, &self.registry)
    }

    /// Read-only access to the current threshold registry (for diagnostics).
    pub fn threshold_registry(&self) -> &ThresholdRegistry {
        &self.cpu_threshold_registry
    }

    pub fn register_velocity_alert(&mut self, alert: VelocityAlertRegistration) {
        self.velocity_alerts.push(alert);
    }

    pub fn register_aggregate_alert(&mut self, alert: AggregateAlertRegistration) {
        self.aggregate_alerts.push(alert);
    }

    pub fn clear_velocity_alerts(&mut self) {
        self.velocity_alerts.clear();
    }

    pub fn clear_aggregate_alerts(&mut self) {
        self.aggregate_alerts.clear();
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
            &self.fission_lineage,
        );
        if let Some(new_reg) = out.new_threshold_registry {
            self.cpu_threshold_registry = new_reg;
        }
    }

    /// Read-only access to the persistent fission lineage. Useful for tests
    /// and observability. Mutation goes through `execute` (fission adds,
    /// fusion / tombstone removes).
    pub fn fission_lineage(&self) -> &[FissionLineageRecord] {
        &self.fission_lineage
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
    ) {
        if required as u32 <= coord.n_slots() {
            return;
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
    }
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

fn tree_has_boundary_lifecycle_work(node: &SimThing, registry: &DimensionRegistry) -> bool {
    if node.overlays.iter().any(|overlay| {
        matches!(overlay.lifecycle, OverlayLifecycle::Transient { .. })
            || registry
                .try_property(overlay.transform.property_id)
                .and_then(|prop| prop.on_expire.as_ref())
                .is_some()
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

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DecayBehavior, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlaySource,
        PropertyTransformDelta, SimProperty, SimThingKind, SubFieldRole, TransformOp,
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
    fn boundary_with_cpu_decay_cannot_skip() {
        let (mut proto, patcher, pid) = simple_proto();
        proto.registry.properties[pid.index()].decay =
            Some(DecayBehavior::TowardZero { rate: 0.1 });
        assert!(!proto.can_skip_empty_boundary(&[], &patcher));
    }
}
