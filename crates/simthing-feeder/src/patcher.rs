//! Transform Patcher — continuous within-day mutation of the dense values
//! buffer.
//!
//! Per design_v4.md §11: receives `PatchTransform` work items, resolves
//! `PropertyTransformDelta` sub-field roles to global column indices via the
//! `DimensionRegistry`, then writes to the targeted SimThing's row.
//!
//! The Patcher operates on a **CPU shadow** of the `values` buffer
//! (`&mut [f32]` of length `n_slots × n_dims`). The Dispatch Coordinator
//! uploads the dirty rows to the GPU before the next tick. Going through a
//! shadow has three benefits:
//!
//! 1. **Immediate writes use GPU-fresh rows for RMW.** `TransformOp::Set` is
//!    always safe. `TransformOp::Multiply` and `TransformOp::Add` require the
//!    current integrated value; the Dispatch Coordinator refreshes affected
//!    shadow rows from the GPU before the Patcher applies those ops each tick.
//!    See `docs/state-authority.md` for the full authority doctrine.
//! 2. **Coalesced uploads.** Multiple patches hitting the same row in the
//!    same tick produce a single `queue.write_buffer` per dirty row.
//! 3. **Testability.** The Patcher has no `wgpu` dependency; unit tests run
//!    on a plain `Vec<f32>`.
//!
//! ## Invariants honored
//!
//! - **I1:** Column arithmetic exclusively through
//!   `PropertyColumnRange::col_for_role`. No `slot * n_dims + col`
//!   arithmetic in semantic paths — only the row-base computation in
//!   `apply_one`, which is exactly the `[slot * n_dims + col]` projection
//!   formula.
//! - **I5:** Patches carry `SubFieldRole`, not column indices. Resolution
//!   happens in `apply_one`.
//! - **I7:** The Patcher never mutates the SimThing tree or the registry.
//!   Boundary requests are parked in a separate Vec for the Tree Maintainer.
//!
//! ## What's silently skipped
//!
//! Mirrors `PropertyTransformDelta::apply_to_data` in the CPU evaluator:
//! - Unknown target ids (not in the `SlotAllocator`).
//! - Properties not active in the registry (tombstoned).
//! - Roles that resolve to no offset in the property's layout.
//!
//! Each skip increments a stat counter so callers can detect drift between
//! gameplay code and registry state without crashing the sim.

use crate::work::{
    AiIntentOverlay, AiReceiver, BoundaryRequest, FeederReceiver, FeederWork, PatchTransform,
    PlayerIntentOverlay,
};
use simthing_core::{DimensionRegistry, PropertyTransformDelta, TransformOp};
use simthing_gpu::{IntentDelta, SlotAllocator};
use std::collections::{HashMap, HashSet};

/// Whether the shadow rows supplied to the patcher are known to include the
/// latest GPU-integrated values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowFreshness {
    Unknown,
    GpuSynced,
}

// ── Stats ─────────────────────────────────────────────────────────────────────

/// Diagnostic counters for a single `drain` invocation. Reset to zero at the
/// start of each drain. Useful for debugging dropped patches and tests.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PatcherStats {
    /// Patches whose target id had no slot in the `SlotAllocator`.
    pub missing_targets: u32,
    /// Patches referencing a property no longer active in the registry.
    pub inactive_property: u32,
    /// (role, op) pairs whose role had no offset in the property's layout.
    pub unresolved_roles: u32,
    /// Individual sub-field writes that actually landed in the shadow.
    pub applied_writes: u32,
    /// Add/Multiply writes skipped because no GPU row sync was available.
    pub unsafe_rmw_skipped: u32,
    /// Boundary requests parked for the Tree Maintainer (not applied here).
    pub boundary_parked: u32,
    /// Player intent overlays parked for attachment at the next boundary.
    pub player_intents_parked: u32,
    /// AI intent overlays parked for attachment at the next boundary.
    pub ai_intents_parked: u32,
}

// ── Patcher ───────────────────────────────────────────────────────────────────

/// CPU-side patch executor. Stateless — the only "state" is what's parked in
/// `pending_boundary` / `pending_player_intents` between drains, which the
/// caller is expected to hand off to the boundary protocol at the next
/// boundary.
#[derive(Debug, Default)]
pub struct TransformPatcher {
    /// Boundary requests received during the day, in arrival order. Drained
    /// by `take_boundary_requests()` at boundary time.
    pending_boundary: Vec<BoundaryRequest>,
    /// Player intent overlays parked for attachment at the next boundary.
    pending_player_intents: Vec<PlayerIntentOverlay>,
    /// AI intent overlays parked for attachment at the next boundary.
    pending_ai_intents: Vec<AiIntentOverlay>,
    /// Optional dedicated AI channel. When set, `drain()` also drains this
    /// receiver after the feeder queue, applying transform deltas mid-day and
    /// parking intents for the boundary. Stored here so the `tick()` signature
    /// needs no changes.
    ai_receiver: Option<AiReceiver>,
    /// Dirty-row tracking. A bit per slot; `true` means the row was touched
    /// since the last `take_dirty_rows()`. Used by the Dispatch Coordinator
    /// to coalesce GPU uploads.
    dirty: Vec<bool>,
    /// Sparse list of slots whose dirty bit flipped from false to true.
    dirty_slots: Vec<u32>,
    /// Reused each tick by `apply_collected_as_intents` — insertion order for folds.
    fold_order: Vec<(u32, u32)>,
    /// Reused each tick by `apply_collected_as_intents` — per-cell affine accumulators.
    fold_accum: HashMap<(u32, u32), (f32, f32)>,
}

impl TransformPatcher {
    pub fn new(n_slots: usize) -> Self {
        Self {
            pending_boundary: Vec::new(),
            pending_player_intents: Vec::new(),
            pending_ai_intents: Vec::new(),
            ai_receiver: None,
            dirty: vec![false; n_slots],
            dirty_slots: Vec::new(),
            fold_order: Vec::new(),
            fold_accum: HashMap::new(),
        }
    }

    /// Attach a dedicated AI channel. After this call `drain()` will
    /// automatically drain the AI receiver on every tick.
    pub fn set_ai_receiver(&mut self, rx: AiReceiver) {
        self.ai_receiver = Some(rx);
    }

    /// Apply every patch currently waiting on the queue. Boundary requests
    /// are routed to `pending_boundary`. Returns per-drain diagnostic stats.
    ///
    /// When `sync_row_from_gpu` is provided, rows targeted by Add/Multiply ops
    /// are refreshed from the GPU before application so RMW uses integrated values.
    pub fn drain(
        &mut self,
        receiver: &FeederReceiver,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims: usize,
        values: &mut [f32],
        sync_row_from_gpu: Option<&mut dyn FnMut(u32)>,
    ) -> PatcherStats {
        let feeder_items = receiver.drain_now();
        let ai_items = self.drain_ai_now();
        let freshness = if let Some(sync) = sync_row_from_gpu {
            for slot in rmw_slots_from_batch(&feeder_items, &ai_items, allocator) {
                sync(slot);
            }
            ShadowFreshness::GpuSynced
        } else {
            ShadowFreshness::Unknown
        };
        self.apply_collected(
            feeder_items,
            ai_items,
            registry,
            allocator,
            n_dims,
            values,
            freshness,
        )
    }

    /// Drain the connected AI channel without applying. Used by the coordinator
    /// to prefetch GPU rows before `apply_collected`.
    pub fn drain_ai_now(&mut self) -> Vec<AiIntentOverlay> {
        if let Some(ai_rx) = &self.ai_receiver {
            ai_rx.drain_now()
        } else {
            Vec::new()
        }
    }

    /// Apply work items already removed from the feeder/AI channels.
    pub fn apply_collected(
        &mut self,
        feeder_items: Vec<FeederWork>,
        ai_items: Vec<AiIntentOverlay>,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims: usize,
        values: &mut [f32],
        freshness: ShadowFreshness,
    ) -> PatcherStats {
        let mut stats = PatcherStats::default();
        for item in feeder_items {
            match item {
                FeederWork::Patch(p) => {
                    self.apply_one(
                        &p, registry, allocator, n_dims, values, &mut stats, freshness,
                    );
                }
                FeederWork::Boundary(b) => {
                    self.pending_boundary.push(b);
                    stats.boundary_parked += 1;
                }
                FeederWork::PlayerIntent(pi) => {
                    let patch = PatchTransform {
                        target: pi.target,
                        delta: pi.overlay.transform.clone(),
                    };
                    self.apply_one(
                        &patch, registry, allocator, n_dims, values, &mut stats, freshness,
                    );
                    self.pending_player_intents.push(pi);
                    stats.player_intents_parked += 1;
                }
            }
        }

        for ai in ai_items {
            let patch = PatchTransform {
                target: ai.target,
                delta: ai.overlay.transform.clone(),
            };
            self.apply_one(
                &patch, registry, allocator, n_dims, values, &mut stats, freshness,
            );
            self.pending_ai_intents.push(ai);
            stats.ai_intents_parked += 1;
        }

        stats
    }

    /// Convert collected work into folded GPU intent deltas instead of
    /// mutating the CPU shadow.
    ///
    /// This is the hot tick path: Set/Add/Multiply ops become one affine
    /// transform per resolved `(slot, col)`, preserving same-cell arrival
    /// order without a CPU readback. Boundary and overlay-intent parking stays
    /// identical to `apply_collected`.
    pub fn apply_collected_as_intents(
        &mut self,
        feeder_items: Vec<FeederWork>,
        ai_items: Vec<AiIntentOverlay>,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
    ) -> (PatcherStats, Vec<IntentDelta>) {
        let mut stats = PatcherStats::default();
        self.fold_order.clear();
        self.fold_accum.clear();

        for item in feeder_items {
            match item {
                FeederWork::Patch(p) => {
                    fold_patch_as_intents(
                        &p,
                        registry,
                        allocator,
                        &mut stats,
                        &mut self.fold_order,
                        &mut self.fold_accum,
                    );
                }
                FeederWork::Boundary(b) => {
                    self.pending_boundary.push(b);
                    stats.boundary_parked += 1;
                }
                FeederWork::PlayerIntent(pi) => {
                    let patch = PatchTransform {
                        target: pi.target,
                        delta: pi.overlay.transform.clone(),
                    };
                    fold_patch_as_intents(
                        &patch,
                        registry,
                        allocator,
                        &mut stats,
                        &mut self.fold_order,
                        &mut self.fold_accum,
                    );
                    self.pending_player_intents.push(pi);
                    stats.player_intents_parked += 1;
                }
            }
        }

        for ai in ai_items {
            let patch = PatchTransform {
                target: ai.target,
                delta: ai.overlay.transform.clone(),
            };
            fold_patch_as_intents(
                &patch,
                registry,
                allocator,
                &mut stats,
                &mut self.fold_order,
                &mut self.fold_accum,
            );
            self.pending_ai_intents.push(ai);
            stats.ai_intents_parked += 1;
        }

        let deltas = self
            .fold_order
            .iter()
            .filter_map(|&(slot, col)| {
                self.fold_accum
                    .get(&(slot, col))
                    .copied()
                    .map(|(mul, add)| IntentDelta {
                        slot,
                        col,
                        mul,
                        add,
                    })
            })
            .collect();

        (stats, deltas)
    }

    /// Single-patch path. Public so callers can drive the Patcher without
    /// going through the channel (e.g., for replaying logs deterministically
    /// in tests).
    pub fn apply_one(
        &mut self,
        patch: &PatchTransform,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims: usize,
        values: &mut [f32],
        stats: &mut PatcherStats,
        freshness: ShadowFreshness,
    ) {
        let Some(slot) = allocator.slot_of(patch.target) else {
            stats.missing_targets += 1;
            return;
        };

        let pid = patch.delta.property_id;
        if !registry.is_active(pid) {
            stats.inactive_property += 1;
            return;
        }
        let range = registry.column_range(pid);
        let layout = &registry.property(pid).layout;
        let base = slot.as_usize() * n_dims;

        let mut wrote_to_row = false;
        for (role, op) in &patch.delta.sub_field_deltas {
            // I1: only path for col arithmetic.
            let Some(col) = range.col_for_role(role, layout) else {
                stats.unresolved_roles += 1;
                continue;
            };
            let addr = base + col;
            // Defensive: a malformed shadow length would be a caller bug, but
            // bounds-checking here keeps a misuse from corrupting unrelated
            // memory in release builds.
            if addr >= values.len() {
                stats.unresolved_roles += 1;
                continue;
            }
            if matches!(op, TransformOp::Add(_) | TransformOp::Multiply(_))
                && !matches!(freshness, ShadowFreshness::GpuSynced)
            {
                stats.unsafe_rmw_skipped += 1;
                continue;
            }
            values[addr] = match op {
                TransformOp::Set(k) => *k,
                TransformOp::Add(_) | TransformOp::Multiply(_) => op.apply(values[addr]),
            };
            stats.applied_writes += 1;
            wrote_to_row = true;
        }

        if wrote_to_row {
            if let Some(flag) = self.dirty.get_mut(slot.as_usize()) {
                if !*flag {
                    *flag = true;
                    self.dirty_slots.push(slot.raw());
                }
            }
        }
    }

    /// Hand off accumulated boundary requests to the caller. Empties the
    /// internal Vec; the Tree Maintainer takes ownership.
    pub fn take_boundary_requests(&mut self) -> Vec<BoundaryRequest> {
        std::mem::take(&mut self.pending_boundary)
    }

    /// Hand off accumulated player intent overlays to the caller. Empties the
    /// internal Vec; the boundary protocol attaches them during step 7/8.
    pub fn take_player_intents(&mut self) -> Vec<PlayerIntentOverlay> {
        std::mem::take(&mut self.pending_player_intents)
    }

    /// Hand off accumulated AI intent overlays to the caller. Empties the
    /// internal Vec; the boundary protocol attaches them during step 7/8.
    pub fn take_ai_intents(&mut self) -> Vec<AiIntentOverlay> {
        std::mem::take(&mut self.pending_ai_intents)
    }

    /// Snapshot + clear the dirty-row bitmap. Returns the indices of every
    /// slot whose shadow row was modified since the last call. The Dispatch
    /// Coordinator uses this to issue exactly one `queue.write_buffer` per
    /// dirty row before the next tick.
    pub fn take_dirty_rows(&mut self) -> Vec<u32> {
        let out = std::mem::take(&mut self.dirty_slots);
        for &slot in &out {
            if let Some(flag) = self.dirty.get_mut(slot as usize) {
                *flag = false;
            }
        }
        out
    }

    /// Resize the dirty-row bitmap. Called after the Tree Maintainer grows
    /// the allocator's capacity at a day boundary.
    pub fn resize(&mut self, n_slots: usize) {
        self.dirty.resize(n_slots, false);
        self.dirty_slots.retain(|slot| (*slot as usize) < n_slots);
    }

    /// Number of currently parked boundary requests. Mostly useful for tests
    /// and observability.
    pub fn pending_boundary_count(&self) -> usize {
        self.pending_boundary.len()
    }

    pub fn pending_player_intent_count(&self) -> usize {
        self.pending_player_intents.len()
    }

    pub fn pending_ai_intent_count(&self) -> usize {
        self.pending_ai_intents.len()
    }

    pub fn pending_boundary_work_count(&self) -> usize {
        self.pending_boundary.len()
            + self.pending_player_intents.len()
            + self.pending_ai_intents.len()
    }
}

fn delta_has_rmw(delta: &PropertyTransformDelta) -> bool {
    delta
        .sub_field_deltas
        .iter()
        .any(|(_, op)| matches!(op, TransformOp::Add(_) | TransformOp::Multiply(_)))
}

fn collect_rmw_slot(
    target: simthing_core::SimThingId,
    delta: &PropertyTransformDelta,
    allocator: &SlotAllocator,
    slots: &mut HashSet<u32>,
) {
    if delta_has_rmw(delta) {
        if let Some(slot) = allocator.slot_of(target) {
            slots.insert(slot.raw());
        }
    }
}

fn fold_patch_as_intents(
    patch: &PatchTransform,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    stats: &mut PatcherStats,
    order: &mut Vec<(u32, u32)>,
    folded: &mut HashMap<(u32, u32), (f32, f32)>,
) {
    let Some(slot) = allocator.slot_of(patch.target) else {
        stats.missing_targets += 1;
        return;
    };

    let pid = patch.delta.property_id;
    if !registry.is_active(pid) {
        stats.inactive_property += 1;
        return;
    }

    let range = registry.column_range(pid);
    let layout = &registry.property(pid).layout;

    for (role, op) in &patch.delta.sub_field_deltas {
        let Some(col) = range.col_for_role(role, layout) else {
            stats.unresolved_roles += 1;
            continue;
        };
        let key = (slot.raw(), col as u32);
        let entry = folded.entry(key).or_insert_with(|| {
            order.push(key);
            (1.0, 0.0)
        });
        match op {
            TransformOp::Set(k) => {
                entry.0 = 0.0;
                entry.1 = *k;
            }
            TransformOp::Add(a) => {
                entry.1 += *a;
            }
            TransformOp::Multiply(m) => {
                entry.0 *= *m;
                entry.1 *= *m;
            }
        }
        stats.applied_writes += 1;
    }
}

pub(crate) fn rmw_slots_from_batch(
    feeder_items: &[FeederWork],
    ai_items: &[AiIntentOverlay],
    allocator: &SlotAllocator,
) -> Vec<u32> {
    let mut slots = HashSet::new();
    for item in feeder_items {
        match item {
            FeederWork::Patch(p) => collect_rmw_slot(p.target, &p.delta, allocator, &mut slots),
            FeederWork::PlayerIntent(pi) => {
                collect_rmw_slot(pi.target, &pi.overlay.transform, allocator, &mut slots);
            }
            FeederWork::Boundary(_) => {}
        }
    }
    for ai in ai_items {
        collect_rmw_slot(ai.target, &ai.overlay.transform, allocator, &mut slots);
    }
    slots.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::work::{feeder_channel, BoundaryRequest, FeederWork, PatchTransform};
    use simthing_core::{
        PropertyTransformDelta, SimProperty, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };

    /// Build a one-property registry (`core::loyalty`, standard layout
    /// stride=6) and a 2-slot allocator with two cohort SimThings.
    fn fixture() -> (
        DimensionRegistry,
        SlotAllocator,
        simthing_core::SimPropertyId,
        [simthing_core::SimThingId; 2],
        usize,
    ) {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 3));

        let n_dims = reg.total_columns;
        let mut alloc = SlotAllocator::new();
        let a = SimThing::new(SimThingKind::Cohort, 0).id;
        let b = SimThing::new(SimThingKind::Cohort, 0).id;
        alloc.alloc(a);
        alloc.alloc(b);
        (reg, alloc, pid, [a, b], n_dims)
    }

    #[test]
    fn add_op_applies_when_shadow_is_current() {
        let (reg, alloc, pid, [_a, b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        values[n_dims + 0] = 0.5;
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: b,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::GpuSynced,
        );

        assert_eq!(values[n_dims + 0].to_bits(), 0.75f32.to_bits());
        assert_eq!(stats.applied_writes, 1);
        assert_eq!(stats.unsafe_rmw_skipped, 0);
    }

    #[test]
    fn apply_one_add_skips_when_shadow_freshness_unknown() {
        let (reg, alloc, pid, [_a, b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        values[n_dims] = 0.5;
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: b,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::Unknown,
        );

        assert_eq!(values[n_dims].to_bits(), 0.5f32.to_bits());
        assert_eq!(stats.applied_writes, 0);
        assert_eq!(stats.unsafe_rmw_skipped, 1);
        assert!(p.take_dirty_rows().is_empty());
    }

    #[test]
    fn apply_one_multiply_skips_when_shadow_freshness_unknown() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        values[0] = 2.0;
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(3.0))],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::Unknown,
        );

        assert_eq!(values[0].to_bits(), 2.0f32.to_bits());
        assert_eq!(stats.applied_writes, 0);
        assert_eq!(stats.unsafe_rmw_skipped, 1);
        assert!(p.take_dirty_rows().is_empty());
    }

    #[test]
    fn multiply_then_set_compose_in_order_on_same_field() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        values[0] = 2.0; // amount col
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![
                        (SubFieldRole::Amount, TransformOp::Multiply(3.0)),
                        (SubFieldRole::Amount, TransformOp::Set(7.0)),
                    ],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::GpuSynced,
        );

        assert_eq!(values[0], 7.0);
        assert_eq!(stats.applied_writes, 2);
        assert_eq!(stats.unsafe_rmw_skipped, 0);
    }

    #[test]
    fn tombstoned_property_increments_inactive_counter() {
        let (mut reg, alloc, pid, [a, _b], n_dims) = fixture();
        reg.tombstone(pid);
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(1.0))],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::Unknown,
        );

        assert_eq!(stats.inactive_property, 1);
        assert_eq!(stats.applied_writes, 0);
    }

    #[test]
    fn drain_routes_patch_and_boundary_correctly() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let (tx, rx) = feeder_channel();
        tx.send(FeederWork::Patch(PatchTransform {
            target: a,
            delta: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Intensity, TransformOp::Set(0.5))],
            },
        }))
        .unwrap();
        tx.send(FeederWork::Boundary(BoundaryRequest::Remove { target: a }))
            .unwrap();

        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let stats = p.drain(&rx, &reg, &alloc, n_dims, &mut values, None);

        // slot 0 (a), Intensity col = 2. Value = 0.5.
        assert_eq!(values[2], 0.5);
        assert_eq!(stats.applied_writes, 1);
        assert_eq!(stats.boundary_parked, 1);
        assert_eq!(p.pending_boundary_count(), 1);

        let parked = p.take_boundary_requests();
        assert_eq!(parked.len(), 1);
        assert!(matches!(parked[0], BoundaryRequest::Remove { .. }));
        // take consumes — second take returns empty.
        assert_eq!(p.take_boundary_requests().len(), 0);
    }

    #[test]
    fn dirty_rows_flip_only_for_safe_writes() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(1.0))],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::Unknown,
        );

        let dirty = p.take_dirty_rows();
        assert_eq!(dirty, vec![0]);
        // Bitmap was cleared by take.
        assert!(p.take_dirty_rows().is_empty());
    }

    #[test]
    fn player_intent_parks_in_pending_and_take_drains_it() {
        use simthing_core::{
            Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
            PropertyTransformDelta, SimPropertyId,
        };
        let (reg, alloc, _pid, [a, _b], n_dims) = fixture();
        let (tx, rx) = feeder_channel();

        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: SimPropertyId(0),
                sub_field_deltas: vec![],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        let overlay_id = overlay.id;
        tx.send(FeederWork::PlayerIntent(crate::work::PlayerIntentOverlay {
            target: a,
            overlay: overlay.clone(),
        }))
        .unwrap();

        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let stats = p.drain(&rx, &reg, &alloc, n_dims, &mut values, None);

        assert_eq!(stats.player_intents_parked, 1);
        assert_eq!(stats.applied_writes, 0);

        let intents = p.take_player_intents();
        assert_eq!(intents.len(), 1);
        assert_eq!(intents[0].target, a);
        assert_eq!(intents[0].overlay.id, overlay_id);
        // take empties the Vec
        assert!(p.take_player_intents().is_empty());
    }

    #[test]
    fn player_intent_applies_transform_to_shadow_and_marks_row_dirty() {
        use simthing_core::{
            Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
            PropertyTransformDelta,
        };
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let (tx, rx) = feeder_channel();

        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![a],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.75))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        tx.send(FeederWork::PlayerIntent(crate::work::PlayerIntentOverlay {
            target: a,
            overlay,
        }))
        .unwrap();

        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let stats = p.drain(&rx, &reg, &alloc, n_dims, &mut values, None);

        // Transform applied: slot 0, Amount col 0 = 0.75.
        assert_eq!(
            values[0], 0.75,
            "mid-day shadow mutation must fire immediately"
        );
        assert_eq!(stats.applied_writes, 1);
        assert_eq!(stats.player_intents_parked, 1);
        // Row marked dirty so the Dispatch Coordinator uploads it this tick.
        assert_eq!(p.take_dirty_rows(), vec![0]);
        // Intent still parked for boundary attach.
        assert_eq!(p.take_player_intents().len(), 1);
    }

    #[test]
    fn ai_intent_applies_transform_to_shadow_and_parks_with_urgency() {
        use crate::work::ai_channel;
        use simthing_core::{
            Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
            PropertyTransformDelta,
        };

        let (reg, alloc, pid, [_a, b], n_dims) = fixture();

        let (ai_tx, ai_rx) = ai_channel();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        p.set_ai_receiver(ai_rx);

        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Ai,
            affects: vec![b],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.42))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };
        let overlay_id = overlay.id;
        ai_tx.submit_ai_intent(b, overlay, 0.9).unwrap();

        // Drain the feeder queue (empty) — AI channel drained automatically.
        let (_, rx) = feeder_channel();
        let stats = p.drain(&rx, &reg, &alloc, n_dims, &mut values, None);

        // Transform applied: slot 1 (b), Amount col = 0.42.
        assert_eq!(
            values[n_dims + 0],
            0.42,
            "AI intent must mutate shadow mid-day"
        );
        assert_eq!(stats.applied_writes, 1);
        assert_eq!(stats.ai_intents_parked, 1);
        // Row 1 dirty.
        assert_eq!(p.take_dirty_rows(), vec![1]);
        // Intent parked with urgency intact.
        let ai = p.take_ai_intents();
        assert_eq!(ai.len(), 1);
        assert_eq!(ai[0].target, b);
        assert_eq!(ai[0].overlay.id, overlay_id);
        assert_eq!(ai[0].urgency.to_bits(), 0.9f32.to_bits());
        assert!(p.take_ai_intents().is_empty());
    }

    #[test]
    fn no_op_patch_does_not_mark_row_dirty() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        // Role doesn't exist in layout → no write → row stays clean.
        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id: pid,
                    sub_field_deltas: vec![(
                        SubFieldRole::Named("vec_99".into()),
                        TransformOp::Add(1.0),
                    )],
                },
            },
            &reg,
            &alloc,
            n_dims,
            &mut values,
            &mut stats,
            ShadowFreshness::Unknown,
        );

        assert!(p.take_dirty_rows().is_empty());
    }
}
