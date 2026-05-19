//! Boundary delta log — per-day record of semantic state changes.
//!
//! Each `BoundaryDeltaEntry` captures one atomic change that occurred during a
//! day boundary. Together the entries form an append-only semantic log that a
//! future replay system can consume to reconstruct session history.
//!
//! ## What is captured now
//!
//! Entries are derived directly from the existing `BoundaryOutcome` fields.
//! All id-bearing outcome vecs (`MaintainerOutcome::allocated/tombstoned/
//! overlays_attached/dimensions_added`) and `VelocityAlertEvent`s translate
//! one-to-one. Fission and fusion are recorded as count entries because
//! `FissionOutcome` currently carries only counts, not spawned-child ids.
//!
//! ## What Opus will extend for full replay
//!
//! - `FissionOccurred` / `FusionOccurred`: extend `FissionOutcome` to carry
//!   `Vec<(SimThingId, SimThingId)>` (parent, child) pairs, then replace the
//!   count entry with per-event entries carrying full ids.
//! - `OverlayAttached`: for deterministic playback the full `Overlay` struct
//!   is needed, not just the id. The serialization pass should join against
//!   the live tree to embed overlay data before writing to disk.
//! - `SimThingReparented`: `MaintainerOutcome` has a reparent count but not
//!   the (child, new_parent) pairs. Extend `MaintainerOutcome` to carry them.
//! - `PropertyExpired`: `ExpiryOutcome` has a count. For full replay, extend
//!   to carry `Vec<(SimThingId, SimPropertyId)>`.

use simthing_core::{OverlayId, SimPropertyId, SimThingId, SubFieldRole};

use crate::boundary::BoundaryOutcome;

// ── Entry type ────────────────────────────────────────────────────────────────

/// One semantic state change that occurred during a day boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum BoundaryDeltaEntry {
    /// An overlay was attached to a SimThing (player intent, AI intent, or
    /// structural `AttachOverlay` boundary request). The `overlay_id` can be
    /// used to look up the full `Overlay` in the live tree.
    OverlayAttached { overlay_id: OverlayId },

    /// A new SimThing was added to the tree via an `AddChild` boundary request.
    SimThingAdded { id: SimThingId },

    /// A SimThing was removed from the tree (slot tombstoned).
    SimThingRemoved { id: SimThingId },

    /// A new property dimension was registered mid-session (`AddDimension`
    /// boundary request). Signals that the column layout grew.
    DimensionAdded { property_id: SimPropertyId },

    /// N fission events fired this boundary. Child ids are not yet tracked;
    /// see module-level doc for the Opus extension path.
    FissionOccurred { count: u32 },

    /// N fusion events fired this boundary.
    FusionOccurred { count: u32 },

    /// N properties expired from SimThings this boundary (threshold-driven or
    /// CPU-side AfterTicks/TowardZero).
    PropertyExpired { count: u32 },

    /// N SimThings were reparented this boundary. Reparent ids are not yet
    /// tracked; see module-level doc for the Opus extension path.
    SimThingReparented { count: u32 },

    /// A velocity alert threshold fired on the given SimThing's property
    /// sub-field.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id:  SimPropertyId,
        sub_field:    SubFieldRole,
        value:        f32,
    },
}

// ── Conversion ────────────────────────────────────────────────────────────────

/// Convert a completed `BoundaryOutcome` into its delta log entries.
///
/// Entries are emitted in boundary step order (lifecycle → expiry → fission →
/// structural mutations → velocity alerts) so the log reads chronologically
/// within a day.
pub fn entries_from_outcome(outcome: &BoundaryOutcome) -> Vec<BoundaryDeltaEntry> {
    let mut entries = Vec::new();

    // Step 4: lifecycle — dissolved overlays are not individually id-tracked yet.
    // Step 5: property expiry.
    let expired = outcome.expiry.properties_removed + outcome.expiry.cpu_side_removals;
    if expired > 0 {
        entries.push(BoundaryDeltaEntry::PropertyExpired { count: expired });
    }

    // Step 6: fission / fusion.
    if outcome.fission.fissions_executed > 0 {
        entries.push(BoundaryDeltaEntry::FissionOccurred {
            count: outcome.fission.fissions_executed,
        });
    }
    if outcome.fission.fusions_executed > 0 {
        entries.push(BoundaryDeltaEntry::FusionOccurred {
            count: outcome.fission.fusions_executed,
        });
    }

    // Steps 7+8: structural mutations from MaintainerOutcome.
    for &id in &outcome.maintainer.allocated {
        entries.push(BoundaryDeltaEntry::SimThingAdded { id });
    }
    for &id in &outcome.maintainer.tombstoned {
        entries.push(BoundaryDeltaEntry::SimThingRemoved { id });
    }
    if outcome.maintainer.reparents > 0 {
        entries.push(BoundaryDeltaEntry::SimThingReparented {
            count: outcome.maintainer.reparents,
        });
    }
    for &oid in &outcome.maintainer.overlays_attached {
        entries.push(BoundaryDeltaEntry::OverlayAttached { overlay_id: oid });
    }
    for &pid in &outcome.maintainer.dimensions_added {
        entries.push(BoundaryDeltaEntry::DimensionAdded { property_id: pid });
    }

    // Velocity alerts.
    for alert in &outcome.velocity_alerts {
        entries.push(BoundaryDeltaEntry::VelocityAlert {
            sim_thing_id: alert.sim_thing_id,
            property_id:  alert.property_id,
            sub_field:    alert.sub_field.clone(),
            value:        alert.value,
        });
    }

    entries
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::BoundaryOutcome;
    use crate::fission::FissionOutcome;
    use crate::property_expiry::ExpiryOutcome;
    use crate::threshold_registry::VelocityAlertEvent;
    use simthing_feeder::MaintainerOutcome;
    use simthing_core::{OverlayId, SimPropertyId, SimThing, SimThingKind, SubFieldRole};

    fn empty_outcome() -> BoundaryOutcome {
        BoundaryOutcome::default()
    }

    #[test]
    fn empty_outcome_produces_no_entries() {
        assert!(entries_from_outcome(&empty_outcome()).is_empty());
    }

    #[test]
    fn fission_and_fusion_counts_produce_entries() {
        let mut out = empty_outcome();
        out.fission = FissionOutcome {
            fissions_executed: 2,
            fusions_executed:  1,
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);
        assert!(entries.contains(&BoundaryDeltaEntry::FissionOccurred { count: 2 }));
        assert!(entries.contains(&BoundaryDeltaEntry::FusionOccurred  { count: 1 }));
    }

    #[test]
    fn maintainer_ids_produce_per_entry_variants() {
        let id_a = SimThing::new(SimThingKind::Cohort, 0).id;
        let id_b = SimThing::new(SimThingKind::Cohort, 0).id;
        let oid  = OverlayId::new();
        let pid  = SimPropertyId(42);

        let mut out = empty_outcome();
        out.maintainer = MaintainerOutcome {
            allocated:         vec![id_a],
            tombstoned:        vec![id_b],
            overlays_attached: vec![oid],
            dimensions_added:  vec![pid],
            reparents:         1,
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);

        assert!(entries.contains(&BoundaryDeltaEntry::SimThingAdded    { id: id_a }));
        assert!(entries.contains(&BoundaryDeltaEntry::SimThingRemoved  { id: id_b }));
        assert!(entries.contains(&BoundaryDeltaEntry::OverlayAttached  { overlay_id: oid }));
        assert!(entries.contains(&BoundaryDeltaEntry::DimensionAdded   { property_id: pid }));
        assert!(entries.contains(&BoundaryDeltaEntry::SimThingReparented { count: 1 }));
    }

    #[test]
    fn property_expiry_combines_threshold_and_cpu_side_removals() {
        let mut out = empty_outcome();
        out.expiry = ExpiryOutcome {
            properties_removed: 2,
            cpu_side_removals:  1,
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);
        assert!(entries.contains(&BoundaryDeltaEntry::PropertyExpired { count: 3 }));
    }

    #[test]
    fn velocity_alerts_produce_per_entry_variants() {
        let id  = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid = SimPropertyId(7);
        let mut out = empty_outcome();
        out.velocity_alerts = vec![VelocityAlertEvent {
            sim_thing_id: id,
            property_id:  pid,
            sub_field:    SubFieldRole::Velocity,
            value:        -0.21,
        }];
        let entries = entries_from_outcome(&out);
        assert_eq!(entries.len(), 1);
        match &entries[0] {
            BoundaryDeltaEntry::VelocityAlert { sim_thing_id, property_id, sub_field, value } => {
                assert_eq!(*sim_thing_id, id);
                assert_eq!(*property_id, pid);
                assert_eq!(*sub_field, SubFieldRole::Velocity);
                assert_eq!(value.to_bits(), (-0.21f32).to_bits());
            }
            _ => panic!("expected VelocityAlert"),
        }
    }

    #[test]
    fn step_order_is_expiry_then_fission_then_structural_then_alerts() {
        let id  = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid = SimPropertyId(1);

        let mut out = empty_outcome();
        out.expiry  = ExpiryOutcome { properties_removed: 1, ..Default::default() };
        out.fission = FissionOutcome { fissions_executed: 1, ..Default::default() };
        out.maintainer = MaintainerOutcome { allocated: vec![id], ..Default::default() };
        out.velocity_alerts = vec![VelocityAlertEvent {
            sim_thing_id: id, property_id: pid,
            sub_field: SubFieldRole::Amount, value: 0.1,
        }];

        let entries = entries_from_outcome(&out);
        let positions: Vec<&str> = entries.iter().map(|e| match e {
            BoundaryDeltaEntry::PropertyExpired    { .. } => "expiry",
            BoundaryDeltaEntry::FissionOccurred    { .. } => "fission",
            BoundaryDeltaEntry::SimThingAdded      { .. } => "add",
            BoundaryDeltaEntry::VelocityAlert      { .. } => "alert",
            _ => "other",
        }).collect();

        let expiry_pos  = positions.iter().position(|&s| s == "expiry").unwrap();
        let fission_pos = positions.iter().position(|&s| s == "fission").unwrap();
        let add_pos     = positions.iter().position(|&s| s == "add").unwrap();
        let alert_pos   = positions.iter().position(|&s| s == "alert").unwrap();

        assert!(expiry_pos  < fission_pos, "expiry before fission");
        assert!(fission_pos < add_pos,     "fission before structural");
        assert!(add_pos     < alert_pos,   "structural before alerts");
    }
}
