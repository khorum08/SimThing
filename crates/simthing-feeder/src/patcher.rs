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
//! 1. **Read-modify-write is local.** `TransformOp::Multiply` and
//!    `TransformOp::Add` both need the current value. Doing this directly on
//!    a GPU buffer would require a read-back round trip per patch.
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

use crate::work::{BoundaryRequest, FeederReceiver, FeederWork, PatchTransform};
use simthing_core::{DimensionRegistry, TransformOp};
use simthing_gpu::SlotAllocator;

// ── Stats ─────────────────────────────────────────────────────────────────────

/// Diagnostic counters for a single `drain` invocation. Reset to zero at the
/// start of each drain. Useful for debugging dropped patches and tests.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PatcherStats {
    /// Patches whose target id had no slot in the `SlotAllocator`.
    pub missing_targets:   u32,
    /// Patches referencing a property no longer active in the registry.
    pub inactive_property: u32,
    /// (role, op) pairs whose role had no offset in the property's layout.
    pub unresolved_roles:  u32,
    /// Individual sub-field writes that actually landed in the shadow.
    pub applied_writes:    u32,
    /// Boundary requests parked for the Tree Maintainer (not applied here).
    pub boundary_parked:   u32,
}

// ── Patcher ───────────────────────────────────────────────────────────────────

/// CPU-side patch executor. Stateless — the only "state" is what's parked in
/// `pending_boundary` between drains, which the caller is expected to hand
/// off to the Tree Maintainer at the next boundary.
#[derive(Debug, Default)]
pub struct TransformPatcher {
    /// Boundary requests received during the day, in arrival order. Drained
    /// by `take_boundary_requests()` at boundary time.
    pending_boundary: Vec<BoundaryRequest>,
    /// Dirty-row tracking. A bit per slot; `true` means the row was touched
    /// since the last `take_dirty_rows()`. Used by the Dispatch Coordinator
    /// to coalesce GPU uploads.
    dirty: Vec<bool>,
}

impl TransformPatcher {
    pub fn new(n_slots: usize) -> Self {
        Self {
            pending_boundary: Vec::new(),
            dirty:            vec![false; n_slots],
        }
    }

    /// Apply every patch currently waiting on the queue. Boundary requests
    /// are routed to `pending_boundary`. Returns per-drain diagnostic stats.
    ///
    /// `values` must be the row-major `[n_slots × n_dims]` shadow buffer
    /// matching the registry currently used by the GPU. Caller is responsible
    /// for keeping shadow + GPU buffer in sync.
    pub fn drain(
        &mut self,
        receiver:  &FeederReceiver,
        registry:  &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims:    usize,
        values:    &mut [f32],
    ) -> PatcherStats {
        let mut stats = PatcherStats::default();
        for item in receiver.drain_now() {
            match item {
                FeederWork::Patch(p) => {
                    self.apply_one(&p, registry, allocator, n_dims, values, &mut stats);
                }
                FeederWork::Boundary(b) => {
                    self.pending_boundary.push(b);
                    stats.boundary_parked += 1;
                }
            }
        }
        stats
    }

    /// Single-patch path. Public so callers can drive the Patcher without
    /// going through the channel (e.g., for replaying logs deterministically
    /// in tests).
    pub fn apply_one(
        &mut self,
        patch:     &PatchTransform,
        registry:  &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims:    usize,
        values:    &mut [f32],
        stats:     &mut PatcherStats,
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
        let range  = registry.column_range(pid);
        let layout = &registry.property(pid).layout;
        let base   = (slot as usize) * n_dims;

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
            values[addr] = match op {
                TransformOp::Multiply(k) => values[addr] * *k,
                TransformOp::Add(k)      => values[addr] + *k,
                TransformOp::Set(k)      => *k,
            };
            stats.applied_writes += 1;
            wrote_to_row = true;
        }

        if wrote_to_row {
            if let Some(flag) = self.dirty.get_mut(slot as usize) {
                *flag = true;
            }
        }
    }

    /// Hand off accumulated boundary requests to the caller. Empties the
    /// internal Vec; the Tree Maintainer takes ownership.
    pub fn take_boundary_requests(&mut self) -> Vec<BoundaryRequest> {
        std::mem::take(&mut self.pending_boundary)
    }

    /// Snapshot + clear the dirty-row bitmap. Returns the indices of every
    /// slot whose shadow row was modified since the last call. The Dispatch
    /// Coordinator uses this to issue exactly one `queue.write_buffer` per
    /// dirty row before the next tick.
    pub fn take_dirty_rows(&mut self) -> Vec<u32> {
        let mut out = Vec::new();
        for (slot, flag) in self.dirty.iter_mut().enumerate() {
            if *flag {
                out.push(slot as u32);
                *flag = false;
            }
        }
        out
    }

    /// Resize the dirty-row bitmap. Called after the Tree Maintainer grows
    /// the allocator's capacity at a day boundary.
    pub fn resize(&mut self, n_slots: usize) {
        self.dirty.resize(n_slots, false);
    }

    /// Number of currently parked boundary requests. Mostly useful for tests
    /// and observability.
    pub fn pending_boundary_count(&self) -> usize {
        self.pending_boundary.len()
    }
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
    fn fixture() -> (DimensionRegistry, SlotAllocator, simthing_core::SimPropertyId, [simthing_core::SimThingId; 2], usize) {
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
    fn add_op_writes_to_amount_column_of_correct_slot() {
        let (reg, alloc, pid, [_a, b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: b,
                delta: PropertyTransformDelta {
                    property_id:      pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.25))],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        // slot 1 (b), col 0 (Amount). Slot 0 row untouched.
        assert_eq!(values[n_dims + 0], 0.25);
        assert_eq!(values[0], 0.0);
        assert_eq!(stats.applied_writes, 1);
        assert_eq!(stats.missing_targets, 0);
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
                    property_id:      pid,
                    sub_field_deltas: vec![
                        (SubFieldRole::Amount, TransformOp::Multiply(3.0)),
                        (SubFieldRole::Amount, TransformOp::Set(7.0)),
                    ],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        // Multiply: 2 → 6, then Set: → 7. Final = 7.
        assert_eq!(values[0], 7.0);
        assert_eq!(stats.applied_writes, 2);
    }

    #[test]
    fn unknown_target_increments_missing_targets() {
        let (reg, alloc, pid, _, n_dims) = fixture();
        let ghost = SimThing::new(SimThingKind::Cohort, 0).id;
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: ghost,
                delta: PropertyTransformDelta {
                    property_id:      pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        assert_eq!(stats.missing_targets, 1);
        assert_eq!(stats.applied_writes, 0);
        assert_eq!(values[0], 0.0);
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
                    property_id:      pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        assert_eq!(stats.inactive_property, 1);
        assert_eq!(stats.applied_writes, 0);
    }

    #[test]
    fn role_missing_from_layout_increments_unresolved() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        // standard(3) has vec_0..vec_2; "vec_99" is not in the layout.
        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id:      pid,
                    sub_field_deltas: vec![(
                        SubFieldRole::Named("vec_99".into()),
                        TransformOp::Add(1.0),
                    )],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        assert_eq!(stats.unresolved_roles, 1);
        assert_eq!(stats.applied_writes, 0);
    }

    #[test]
    fn drain_routes_patch_and_boundary_correctly() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let (tx, rx) = feeder_channel();
        tx.send(FeederWork::Patch(PatchTransform {
            target: a,
            delta: PropertyTransformDelta {
                property_id:      pid,
                sub_field_deltas: vec![(SubFieldRole::Intensity, TransformOp::Set(0.5))],
            },
        })).unwrap();
        tx.send(FeederWork::Boundary(BoundaryRequest::Remove { target: a })).unwrap();

        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let stats = p.drain(&rx, &reg, &alloc, n_dims, &mut values);

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
    fn dirty_rows_flips_only_the_touched_slot() {
        let (reg, alloc, pid, [a, _b], n_dims) = fixture();
        let mut values = vec![0.0f32; 2 * n_dims];
        let mut p = TransformPatcher::new(2);
        let mut stats = PatcherStats::default();

        p.apply_one(
            &PatchTransform {
                target: a,
                delta: PropertyTransformDelta {
                    property_id:      pid,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        let dirty = p.take_dirty_rows();
        assert_eq!(dirty, vec![0]);
        // Bitmap was cleared by take.
        assert!(p.take_dirty_rows().is_empty());
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
                    property_id:      pid,
                    sub_field_deltas: vec![(
                        SubFieldRole::Named("vec_99".into()),
                        TransformOp::Add(1.0),
                    )],
                },
            },
            &reg, &alloc, n_dims, &mut values, &mut stats,
        );

        assert!(p.take_dirty_rows().is_empty());
    }
}
