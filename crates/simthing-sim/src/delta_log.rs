//! Boundary delta log — per-day record of semantic state changes.
//!
//! Each `BoundaryDeltaEntry` captures one atomic change that occurred during a
//! day boundary. Together the entries form an append-only semantic log that the
//! replay system consumes to reconstruct session history.
//!
//! ## What is captured
//!
//! Entries are derived directly from the existing `BoundaryOutcome` fields.
//! Structural mutations, fission/fusion, property expiry, overlay attachments,
//! and velocity alerts emit one entry per affected entity (or entity pair).
//!
//! Replay v2 additions:
//! - `SimThingAdded` embeds the full `SimThing` payload + parent id.
//! - `FissionOccurred` embeds the spawned child as a full `SimThing`.
//! - `FissionLineageAdded` / `FissionLineageRemoved` persist lineage records so
//!   `ReplayDriver` can reconstruct `FusionTrigger` thresholds across sessions.

use serde::{Deserialize, Serialize};
use simthing_core::{Overlay, OverlayId, SimPropertyId, SimThing, SimThingId, SubFieldRole};

use crate::boundary::BoundaryOutcome;
use crate::fission::FissionLineageRecord;

// ── Entry type ────────────────────────────────────────────────────────────────

/// One semantic state change that occurred during a day boundary.
///
/// Each variant carries enough information to be replayed against a fresh
/// `SimThing` tree without consulting the original sim.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BoundaryDeltaEntry {
    /// An overlay was attached to a SimThing (player intent, AI intent, or
    /// structural `AttachOverlay` boundary request). Carries the full
    /// `Overlay` payload so replay can re-attach it without referring back
    /// to the live tree.
    OverlayAttached {
        target:  SimThingId,
        overlay: Overlay,
    },

    /// A transient overlay dissolved at the boundary (lifecycle step 4).
    OverlayDissolved {
        target:     SimThingId,
        overlay_id: OverlayId,
    },

    /// A new SimThing was added to the tree via an `AddChild` boundary request.
    /// Carries the full `SimThing` payload and the immediate parent's id so
    /// replay can re-attach the subtree verbatim. Silently omitted when the
    /// node cannot be located in the post-boundary tree (e.g. added and
    /// removed in the same boundary).
    SimThingAdded {
        parent: SimThingId,
        node:   SimThing,
    },

    /// A SimThing was removed from the tree (slot tombstoned).
    SimThingRemoved { id: SimThingId },

    /// A new property dimension was registered mid-session (`AddDimension`
    /// boundary request). Signals that the column layout grew.
    DimensionAdded { property_id: SimPropertyId },

    /// A fission event spawned a child under `parent`. Carries the full
    /// `SimThing` payload so replay can structurally re-spawn the child.
    /// Silently omitted when the child is not found in the post-boundary tree
    /// (should not occur in normal operation).
    FissionOccurred {
        parent: SimThingId,
        node:   SimThing,
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
        child:      SimThingId,
        new_parent: SimThingId,
    },

    /// A velocity alert threshold fired on the given SimThing's property
    /// sub-field.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id:  SimPropertyId,
        sub_field:    SubFieldRole,
        value:        f32,
    },

    /// An aggregate alert threshold fired on a reduced parent column (Pass 7
    /// on `output_vectors`). Observation-only for replay.
    AggregateAlert {
        sim_thing_id: SimThingId,
        property_id:  SimPropertyId,
        sub_field:    SubFieldRole,
        value:        f32,
    },

    /// A new fission lineage record was registered (child spawned from parent).
    /// Persisted in the delta log so `ReplayDriver` can reconstruct the
    /// `fission_lineage` vec — and thereby re-register `FusionTrigger`
    /// thresholds — across session boundaries.
    FissionLineageAdded { record: FissionLineageRecord },

    /// A fission lineage record was retired: the child fused back into the
    /// parent, or one of the two endpoints was tombstoned.
    FissionLineageRemoved { record: FissionLineageRecord },
}

// ── Conversion ────────────────────────────────────────────────────────────────

/// Convert a completed `BoundaryOutcome` into its delta log entries.
///
/// `root` is the post-boundary tree, used to:
/// - Look up the full `Overlay` payload for each `(target, overlay_id)` pair in
///   `outcome.maintainer.overlays_attached`. If an overlay is not found, the
///   entry is silently skipped.
/// - Look up the spawned `SimThing` node for `SimThingAdded` entries. If the
///   node is not found (added and immediately removed in the same boundary),
///   the entry is silently skipped.
/// - Look up the fission child node for `FissionOccurred` entries. If not
///   found (shouldn't happen in normal operation), the entry is silently skipped.
///
/// Entries are emitted in boundary step order (lifecycle → expiry → fission →
/// structural mutations → velocity alerts) so the log reads chronologically
/// within a day.
pub fn entries_from_outcome(
    outcome: &BoundaryOutcome,
    root:    &SimThing,
) -> Vec<BoundaryDeltaEntry> {
    let mut entries = Vec::new();

    // Step 4: lifecycle — dissolved overlays.
    for &(target, overlay_id) in &outcome.lifecycle.dissolved_overlays {
        entries.push(BoundaryDeltaEntry::OverlayDissolved { target, overlay_id });
    }

    // Step 5: property expiry.
    for &(sim_thing_id, property_id) in &outcome.expiry.expired {
        entries.push(BoundaryDeltaEntry::PropertyExpired {
            sim_thing_id,
            property_id,
        });
    }

    // Step 6: fission / fusion.
    for &(parent, child) in &outcome.fission.fission_pairs {
        if let Some(node) = find_node(root, child) {
            entries.push(BoundaryDeltaEntry::FissionOccurred {
                parent,
                node: node.clone(),
            });
        }
        // If child not found in the post-boundary tree (shouldn't happen in
        // normal operation): silently skip. Matches OverlayAttached behavior.
    }
    for &(parent, child) in &outcome.fission.fusion_pairs {
        entries.push(BoundaryDeltaEntry::FusionOccurred { parent, child });
    }
    // Lineage changes — persisted so replay can reconstruct FusionTrigger
    // thresholds across session boundaries.
    for &record in &outcome.fission.lineage_added {
        entries.push(BoundaryDeltaEntry::FissionLineageAdded { record });
    }
    for &record in &outcome.fission.lineage_removed {
        entries.push(BoundaryDeltaEntry::FissionLineageRemoved { record });
    }

    // Steps 7+8: structural mutations from MaintainerOutcome.
    for &id in &outcome.maintainer.allocated {
        if let Some((parent_id, node)) = find_node_with_parent(root, id) {
            entries.push(BoundaryDeltaEntry::SimThingAdded {
                parent: parent_id,
                node:   node.clone(),
            });
        }
        // If not found (node was added and removed in same boundary): skip.
    }
    for &id in &outcome.maintainer.tombstoned {
        entries.push(BoundaryDeltaEntry::SimThingRemoved { id });
    }
    for &(child, new_parent) in &outcome.maintainer.reparented {
        entries.push(BoundaryDeltaEntry::SimThingReparented { child, new_parent });
    }
    for &(target, oid) in &outcome.maintainer.overlays_attached {
        if let Some(overlay) = find_overlay(root, target, oid) {
            entries.push(BoundaryDeltaEntry::OverlayAttached {
                target,
                overlay: overlay.clone(),
            });
        }
    }
    for &pid in &outcome.maintainer.dimensions_added {
        entries.push(BoundaryDeltaEntry::DimensionAdded { property_id: pid });
    }

    // Velocity and aggregate alerts.
    for alert in &outcome.velocity_alerts {
        entries.push(BoundaryDeltaEntry::VelocityAlert {
            sim_thing_id: alert.sim_thing_id,
            property_id:  alert.property_id,
            sub_field:    alert.sub_field.clone(),
            value:        alert.value,
        });
    }
    for alert in &outcome.aggregate_alerts {
        entries.push(BoundaryDeltaEntry::AggregateAlert {
            sim_thing_id: alert.sim_thing_id,
            property_id:  alert.property_id,
            sub_field:    alert.sub_field.clone(),
            value:        alert.value,
        });
    }

    entries
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Walk the tree depth-first looking for `target`, then scan its overlays for
/// the given `OverlayId`. Returns `None` if either lookup fails.
fn find_overlay<'a>(
    root:       &'a SimThing,
    target:     SimThingId,
    overlay_id: simthing_core::OverlayId,
) -> Option<&'a Overlay> {
    let node = find_node(root, target)?;
    node.overlays.iter().find(|o| o.id == overlay_id)
}

/// Walk the tree depth-first and return the node with the given id.
fn find_node(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id { return Some(root); }
    for child in &root.children {
        if let Some(n) = find_node(child, id) { return Some(n); }
    }
    None
}

/// Walk the tree depth-first looking for a node with `id`. Returns a tuple of
/// `(parent_id, &node)` when found. Never returns the root itself — root has
/// no parent in this tree representation.
fn find_node_with_parent(root: &SimThing, id: SimThingId) -> Option<(SimThingId, &SimThing)> {
    for child in &root.children {
        if child.id == id {
            return Some((root.id, child));
        }
        if let Some(found) = find_node_with_parent(child, id) {
            return Some(found);
        }
    }
    None
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::BoundaryOutcome;
    use crate::fission::FissionOutcome;
    use crate::property_expiry::ExpiryOutcome;
    use crate::threshold_registry::VelocityAlertEvent;
    use crate::threshold_registry::AggregateAlertEvent;
    use simthing_feeder::MaintainerOutcome;
    use simthing_core::{
        OverlayId, OverlayKind, OverlaySource, OverlayLifecycle, PropertyTransformDelta,
        SimPropertyId, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };

    fn empty_outcome() -> BoundaryOutcome {
        BoundaryOutcome::default()
    }

    fn empty_root() -> SimThing {
        SimThing::new(SimThingKind::World, 0)
    }

    fn make_overlay() -> Overlay {
        Overlay {
            id:        OverlayId::new(),
            kind:      OverlayKind::Policy,
            source:    OverlaySource::System,
            affects:   Vec::new(),
            transform: PropertyTransformDelta {
                property_id:      SimPropertyId(0),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.5))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        }
    }

    fn count_matching<F: Fn(&BoundaryDeltaEntry) -> bool>(
        entries: &[BoundaryDeltaEntry], f: F,
    ) -> usize {
        entries.iter().filter(|e| f(e)).count()
    }

    #[test]
    fn empty_outcome_produces_no_entries() {
        assert!(entries_from_outcome(&empty_outcome(), &empty_root()).is_empty());
    }

    #[test]
    fn fission_and_fusion_pairs_produce_per_entry_variants() {
        // Build a tree with a parent carrying both fission children so that
        // entries_from_outcome can walk the tree and find their full payloads.
        let mut root = empty_root();
        let mut parent_node = SimThing::new(SimThingKind::Cohort, 0);
        let parent = parent_node.id;
        let child_a_node = SimThing::new(SimThingKind::Cohort, 0);
        let child_a = child_a_node.id;
        let child_b_node = SimThing::new(SimThingKind::Cohort, 0);
        let child_b = child_b_node.id;
        parent_node.add_child(child_a_node);
        parent_node.add_child(child_b_node);
        root.add_child(parent_node);

        let mut out = empty_outcome();
        out.fission = FissionOutcome {
            fissions_executed: 2,
            fusions_executed:  1,
            fission_pairs:     vec![(parent, child_a), (parent, child_b)],
            fusion_pairs:      vec![(parent, child_b)],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out, &root);
        assert_eq!(count_matching(&entries, |e| matches!(e,
            BoundaryDeltaEntry::FissionOccurred { parent: p, node }
                if *p == parent && node.id == child_a)), 1);
        assert_eq!(count_matching(&entries, |e| matches!(e,
            BoundaryDeltaEntry::FissionOccurred { parent: p, node }
                if *p == parent && node.id == child_b)), 1);
        assert_eq!(count_matching(&entries, |e| matches!(e,
            BoundaryDeltaEntry::FusionOccurred { parent: p, child: c }
                if *p == parent && *c == child_b)), 1);
    }

    #[test]
    fn fission_lineage_changes_produce_entries() {
        use crate::fission::FissionLineageRecord;

        let parent_id = SimThing::new(SimThingKind::Cohort, 0).id;
        let child_id  = SimThing::new(SimThingKind::Cohort, 0).id;
        let rec = FissionLineageRecord {
            parent_id,
            child_id,
            property_id:  SimPropertyId(0),
            template_idx: 0,
        };

        let mut out = empty_outcome();
        out.fission = FissionOutcome {
            lineage_added:   vec![rec],
            lineage_removed: vec![rec],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out, &empty_root());

        assert_eq!(count_matching(&entries, |e| matches!(e,
            BoundaryDeltaEntry::FissionLineageAdded { record: r }
                if r.parent_id == parent_id && r.child_id == child_id)), 1);
        assert_eq!(count_matching(&entries, |e| matches!(e,
            BoundaryDeltaEntry::FissionLineageRemoved { record: r }
                if r.parent_id == parent_id && r.child_id == child_id)), 1);
    }

    #[test]
    fn maintainer_ids_produce_per_entry_variants() {
        // Build tree: root → new_node (the "allocated" child), root → target_node.
        let mut root = empty_root();
        let root_id = root.id;

        // The "allocated" node — pre-create so we know its id, then add to tree.
        let new_node = SimThing::new(SimThingKind::Cohort, 0);
        let id_a = new_node.id;
        root.add_child(new_node);

        let id_b = SimThing::new(SimThingKind::Cohort, 0).id; // tombstoned (not in tree)
        let id_c = SimThing::new(SimThingKind::Cohort, 0).id; // reparented child
        let id_d = SimThing::new(SimThingKind::Cohort, 0).id; // reparented new_parent
        let pid  = SimPropertyId(42);

        // Build a tree that carries the overlay so find_overlay returns Some.
        let mut target_node = SimThing::new(SimThingKind::Cohort, 0);
        let target_id = target_node.id;
        let overlay = make_overlay();
        let oid = overlay.id;
        target_node.overlays.push(overlay);
        root.add_child(target_node);

        let mut out = empty_outcome();
        out.maintainer = MaintainerOutcome {
            allocated:         vec![id_a],
            tombstoned:        vec![id_b],
            overlays_attached: vec![(target_id, oid)],
            dimensions_added:  vec![pid],
            reparents:         1,
            reparented:        vec![(id_c, id_d)],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out, &root);

        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::SimThingAdded { parent, node }
                if *parent == root_id && node.id == id_a)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::SimThingRemoved { id } if *id == id_b)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::OverlayAttached { target, overlay }
                if *target == target_id && overlay.id == oid)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::DimensionAdded { property_id } if *property_id == pid)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::SimThingReparented { child, new_parent }
                if *child == id_c && *new_parent == id_d)), 1);
    }

    #[test]
    fn overlay_attached_skipped_when_not_in_tree() {
        let id_a = SimThing::new(SimThingKind::Cohort, 0).id;
        let oid  = OverlayId::new();

        let mut out = empty_outcome();
        out.maintainer = MaintainerOutcome {
            overlays_attached: vec![(id_a, oid)],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out, &empty_root());
        // The overlay was never in the tree → entry skipped.
        assert!(!entries.iter().any(|e|
            matches!(e, BoundaryDeltaEntry::OverlayAttached { .. })));
    }

    #[test]
    fn sim_thing_added_skipped_when_id_not_in_tree() {
        // A node id in `allocated` that was never added to the tree → skipped.
        let phantom_id = SimThing::new(SimThingKind::Cohort, 0).id;
        let mut out = empty_outcome();
        out.maintainer = MaintainerOutcome {
            allocated: vec![phantom_id],
            ..Default::default()
        };
        let entries = entries_from_outcome(&out, &empty_root());
        assert!(!entries.iter().any(|e| matches!(e, BoundaryDeltaEntry::SimThingAdded { .. })));
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
        let entries = entries_from_outcome(&out, &empty_root());
        assert_eq!(entries.len(), 3);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::PropertyExpired { sim_thing_id, property_id }
                if *sim_thing_id == id_a && *property_id == pid1)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::PropertyExpired { sim_thing_id, property_id }
                if *sim_thing_id == id_a && *property_id == pid2)), 1);
        assert_eq!(count_matching(&entries,
            |e| matches!(e, BoundaryDeltaEntry::PropertyExpired { sim_thing_id, property_id }
                if *sim_thing_id == id_b && *property_id == pid3)), 1);
    }

    #[test]
    fn overlay_dissolved_produces_entry() {
        let target_id = SimThing::new(SimThingKind::Cohort, 0).id;
        let oid = OverlayId::new();
        let mut out = empty_outcome();
        out.lifecycle.dissolved_overlays = vec![(target_id, oid)];
        let entries = entries_from_outcome(&out, &empty_root());
        assert_eq!(entries.len(), 1);
        match &entries[0] {
            BoundaryDeltaEntry::OverlayDissolved { target, overlay_id } => {
                assert_eq!(*target, target_id);
                assert_eq!(*overlay_id, oid);
            }
            _ => panic!("expected OverlayDissolved"),
        }
    }

    #[test]
    fn aggregate_alert_produces_entry() {
        let id  = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid = SimPropertyId(7);
        let mut out = empty_outcome();
        out.aggregate_alerts = vec![AggregateAlertEvent {
            sim_thing_id: id,
            property_id:  pid,
            sub_field:    SubFieldRole::Amount,
            value:        0.55,
        }];
        let entries = entries_from_outcome(&out, &empty_root());
        assert_eq!(entries.len(), 1);
        match &entries[0] {
            BoundaryDeltaEntry::AggregateAlert { sim_thing_id, property_id, sub_field, value } => {
                assert_eq!(*sim_thing_id, id);
                assert_eq!(*property_id, pid);
                assert_eq!(*sub_field, SubFieldRole::Amount);
                assert_eq!(value.to_bits(), 0.55f32.to_bits());
            }
            _ => panic!("expected AggregateAlert"),
        }
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
        let entries = entries_from_outcome(&out, &empty_root());
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
        // Build a tree where the fission child and the "allocated" add-child
        // are actually present, so both entries are emitted (not skipped).
        let mut root = empty_root();
        let expiry_id = SimThing::new(SimThingKind::Cohort, 0).id;
        let pid = SimPropertyId(1);

        let mut parent_node = SimThing::new(SimThingKind::Cohort, 0);
        let parent_id = parent_node.id;
        let fission_child = SimThing::new(SimThingKind::Cohort, 0);
        let child = fission_child.id;
        parent_node.add_child(fission_child);
        root.add_child(parent_node);

        let added_node = SimThing::new(SimThingKind::Cohort, 0);
        let added_id = added_node.id;
        root.add_child(added_node);

        let mut out = empty_outcome();
        out.expiry  = ExpiryOutcome {
            expired: vec![(expiry_id, pid)],
            ..Default::default()
        };
        out.fission = FissionOutcome {
            fission_pairs: vec![(parent_id, child)],
            ..Default::default()
        };
        out.maintainer = MaintainerOutcome {
            allocated: vec![added_id],
            ..Default::default()
        };
        out.velocity_alerts = vec![VelocityAlertEvent {
            sim_thing_id: expiry_id, property_id: pid,
            sub_field: SubFieldRole::Amount, value: 0.1,
        }];

        let entries = entries_from_outcome(&out, &root);
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
