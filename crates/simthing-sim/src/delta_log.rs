//! Boundary delta log — per-day record of semantic state changes.
//!
//! Each `BoundaryDeltaEntry` captures one atomic change that occurred during a
//! day boundary. Together the entries form an append-only semantic log that a
//! future replay system can consume to reconstruct session history.
//!
//! ## What is captured now
//!
//! Entries are derived directly from the existing `BoundaryOutcome` fields.
//! Structural mutations, fission/fusion, property expiry, and velocity alerts
//! emit one entry per affected entity (or entity pair).
//!
//! ## What replay serialization still needs (deferred)
//!
//! - `OverlayAttached`: for deterministic playback the full `Overlay` struct
//!   is needed, not just the id. The serialization pass should join against
//!   the live tree to embed overlay data before writing to disk.

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

    /// A fission event spawned a child under `parent`.
    FissionOccurred {
        parent: SimThingId,
        child:  SimThingId,
    },

    /// A fusion event merged `child` back into `parent`'s subtree.
    FusionOccurred {
        parent: SimThingId,
        child:  SimThingId,
    },

    /// A property was removed from a SimThing (threshold-driven or CPU decay).
    PropertyExpired {
        sim_thing_id: SimThingId,
        property_id:  SimPropertyId,
    },

    /// A SimThing was reparented under `new_parent`.
    SimThingReparented {
        child:       SimThingId,
        new_parent:  SimThingId,
    },

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
    for &(sim_thing_id, property_id) in &outcome.expiry.expired {
        entries.push(BoundaryDeltaEntry::PropertyExpired {
            sim_thing_id,
            property_id,
        });
    }

    // Step 6: fission / fusion.
    for &(parent, child) in &outcome.fission.fission_pairs {
        entries.push(BoundaryDeltaEntry::FissionOccurred { parent, child });
    }
    for &(parent, child) in &outcome.fission.fusion_pairs {
        entries.push(BoundaryDeltaEntry::FusionOccurred { parent, child });
    }

    // Steps 7+8: structural mutations from MaintainerOutcome.
    for &id in &outcome.maintainer.allocated {
        entries.push(BoundaryDeltaEntry::SimThingAdded { id });
    }
    for &id in &outcome.maintainer.tombstoned {
        entries.push(BoundaryDeltaEntry::SimThingRemoved { id });
    }
    for &(child, new_parent) in &outcome.maintainer.reparented {
        entries.push(BoundaryDeltaEntry::SimThingReparented { child, new_parent });
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
    fn fission_and_fusion_pairs_produce_per_entry_variants() {
        let parent = SimThing::new(SimThingKind::Cohort, 0).id;
        let child_a = SimThing::new(SimThingKind::Cohort, 0).id;
        let child_b = SimThing::new(SimThingKind::Cohort, 0).id;

        let mut out = empty_outcome();
        out.fission = FissionOutcome {
            fissions_executed: 2,
            fusions_executed:  1,
            fission_pairs:     vec![(parent, child_a), (parent, child_b)],
            fusion_pairs:      vec![(parent, child_b)],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);
        assert!(entries.contains(&BoundaryDeltaEntry::FissionOccurred {
            parent,
            child: child_a,
        }));
        assert!(entries.contains(&BoundaryDeltaEntry::FissionOccurred {
            parent,
            child: child_b,
        }));
        assert!(entries.contains(&BoundaryDeltaEntry::FusionOccurred {
            parent,
            child: child_b,
        }));
    }

    #[test]
    fn maintainer_ids_produce_per_entry_variants() {
        let id_a = SimThing::new(SimThingKind::Cohort, 0).id;
        let id_b = SimThing::new(SimThingKind::Cohort, 0).id;
        let id_c = SimThing::new(SimThingKind::Cohort, 0).id;
        let id_d = SimThing::new(SimThingKind::Cohort, 0).id;
        let oid  = OverlayId::new();
        let pid  = SimPropertyId(42);

        let mut out = empty_outcome();
        out.maintainer = MaintainerOutcome {
            allocated:         vec![id_a],
            tombstoned:        vec![id_b],
            overlays_attached: vec![oid],
            dimensions_added:  vec![pid],
            reparents:         1,
            reparented:        vec![(id_c, id_d)],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);

        assert!(entries.contains(&BoundaryDeltaEntry::SimThingAdded    { id: id_a }));
        assert!(entries.contains(&BoundaryDeltaEntry::SimThingRemoved  { id: id_b }));
        assert!(entries.contains(&BoundaryDeltaEntry::OverlayAttached  { overlay_id: oid }));
        assert!(entries.contains(&BoundaryDeltaEntry::DimensionAdded   { property_id: pid }));
        assert!(entries.contains(&BoundaryDeltaEntry::SimThingReparented {
            child: id_c,
            new_parent: id_d,
        }));
    }

    #[test]
    fn property_expiry_emits_one_entry_per_removal() {
        let id_a = SimThing::new(SimThingKind::Cohort, 0).id;
        let id_b = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid1 = SimPropertyId(1);
        let pid2 = SimPropertyId(2);
        let pid3 = SimPropertyId(3);

        let mut out = empty_outcome();
        out.expiry = ExpiryOutcome {
            properties_removed: 2,
            cpu_side_removals:  1,
            expired: vec![
                (id_a, pid1),
                (id_a, pid2),
                (id_b, pid3),
            ],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out);
        assert_eq!(entries.len(), 3);
        assert!(entries.contains(&BoundaryDeltaEntry::PropertyExpired {
            sim_thing_id: id_a,
            property_id: pid1,
        }));
        assert!(entries.contains(&BoundaryDeltaEntry::PropertyExpired {
            sim_thing_id: id_a,
            property_id: pid2,
        }));
        assert!(entries.contains(&BoundaryDeltaEntry::PropertyExpired {
            sim_thing_id: id_b,
            property_id: pid3,
        }));
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
        let child = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid = SimPropertyId(1);

        let mut out = empty_outcome();
        out.expiry  = ExpiryOutcome {
            expired: vec![(id, pid)],
            ..Default::default()
        };
        out.fission = FissionOutcome {
            fission_pairs: vec![(id, child)],
            ..Default::default()
        };
        out.maintainer = MaintainerOutcome {
            allocated: vec![id],
            ..Default::default()
        };
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
