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
//! - `SimThingAdded` embeds the admitted subtree + parent id.
//! - `FissionOccurred` embeds the spawned child as an admitted subtree.
//! - `FissionLineageAdded` / `FissionLineageRemoved` persist lineage records so
//!   `ReplayDriver` can reconstruct `FusionTrigger` thresholds across sessions.
//!
//! Delta payloads do not expose raw `SimThing` or `.kind`
//! (`boundary_delta_entry_hides_raw_simthing_kind_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::BoundaryDeltaEntry;
//!
//! fn peek_delta_node_kind(entry: BoundaryDeltaEntry) {
//!     if let BoundaryDeltaEntry::SimThingAdded { node, .. } = entry {
//!         let _ = node.access(|n| n.kind.clone());
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use simthing_core::{Overlay, OverlayId, SimPropertyId, SimThing, SimThingId, SubFieldRole};
use std::collections::HashMap;

use crate::boundary::BoundaryOutcome;
use crate::fission::FissionLineageRecord;
use crate::sim_runtime_tree::SimRuntimeTree;

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
        target: SimThingId,
        overlay: Overlay,
    },

    /// A transient overlay dissolved at the boundary (lifecycle step 4).
    OverlayDissolved {
        target: SimThingId,
        overlay_id: OverlayId,
    },

    /// A suspended overlay was activated at the boundary.
    OverlayActivated {
        target: SimThingId,
        overlay_id: OverlayId,
    },

    /// An active overlay was suspended at the boundary.
    OverlaySuspended {
        target: SimThingId,
        overlay_id: OverlayId,
    },

    /// A new SimThing was added to the tree via an `AddChild` boundary request.
    /// Carries the full `SimThing` payload and the immediate parent's id so
    /// replay can re-attach the subtree verbatim. Silently omitted when the
    /// node cannot be located in the post-boundary tree (e.g. added and
    /// removed in the same boundary).
    SimThingAdded {
        parent: SimThingId,
        node: SimRuntimeTree,
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
        node: SimRuntimeTree,
    },

    /// A fusion event merged `child` back into `parent`'s subtree.
    FusionOccurred {
        parent: SimThingId,
        child: SimThingId,
    },

    /// A property was removed from a SimThing (threshold-driven or CPU decay).
    PropertyExpired {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
    },

    /// A SimThing was reparented under `new_parent`.
    SimThingReparented {
        child: SimThingId,
        new_parent: SimThingId,
    },

    /// A velocity alert threshold fired on the given SimThing's property
    /// sub-field.
    VelocityAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
        value: f32,
    },

    /// An aggregate alert threshold fired on a reduced parent column (Pass 7
    /// on `output_vectors`). Observation-only for replay.
    AggregateAlert {
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        sub_field: SubFieldRole,
        value: f32,
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
pub(crate) fn entries_from_outcome(
    outcome: &BoundaryOutcome,
    root: &SimThing,
) -> Vec<BoundaryDeltaEntry> {
    let index = DeltaLogTreeIndex::new(root);
    let mut entries = Vec::with_capacity(estimated_entry_count(outcome));

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
        if let Some(node) = index.node(child) {
            entries.push(BoundaryDeltaEntry::FissionOccurred {
                parent,
                node: SimRuntimeTree::admit(node.clone()),
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
        if let Some((parent_id, node)) = index.node_with_parent(id) {
            entries.push(BoundaryDeltaEntry::SimThingAdded {
                parent: parent_id,
                node: SimRuntimeTree::admit(node.clone()),
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
        if let Some(overlay) = index.overlay(target, oid) {
            entries.push(BoundaryDeltaEntry::OverlayAttached {
                target,
                overlay: overlay.clone(),
            });
        }
    }
    for &(target, overlay_id) in &outcome.maintainer.overlays_activated {
        entries.push(BoundaryDeltaEntry::OverlayActivated { target, overlay_id });
    }
    for &(target, overlay_id) in &outcome.maintainer.overlays_suspended {
        entries.push(BoundaryDeltaEntry::OverlaySuspended { target, overlay_id });
    }
    for &pid in &outcome.maintainer.dimensions_added {
        entries.push(BoundaryDeltaEntry::DimensionAdded { property_id: pid });
    }

    // Velocity and aggregate alerts.
    for alert in &outcome.velocity_alerts {
        entries.push(BoundaryDeltaEntry::VelocityAlert {
            sim_thing_id: alert.sim_thing_id,
            property_id: alert.property_id,
            sub_field: alert.sub_field.clone(),
            value: alert.value,
        });
    }
    for alert in &outcome.aggregate_alerts {
        entries.push(BoundaryDeltaEntry::AggregateAlert {
            sim_thing_id: alert.sim_thing_id,
            property_id: alert.property_id,
            sub_field: alert.sub_field.clone(),
            value: alert.value,
        });
    }

    entries
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Upper-bound the entries so hot fission boundaries don't repeatedly grow the
/// log vector while preserving the existing skip-if-payload-missing behavior.
fn estimated_entry_count(outcome: &BoundaryOutcome) -> usize {
    outcome.lifecycle.dissolved_overlays.len()
        + outcome.expiry.expired.len()
        + outcome.fission.fission_pairs.len()
        + outcome.fission.fusion_pairs.len()
        + outcome.fission.lineage_added.len()
        + outcome.fission.lineage_removed.len()
        + outcome.maintainer.allocated.len()
        + outcome.maintainer.tombstoned.len()
        + outcome.maintainer.reparented.len()
        + outcome.maintainer.overlays_attached.len()
        + outcome.maintainer.overlays_activated.len()
        + outcome.maintainer.overlays_suspended.len()
        + outcome.maintainer.dimensions_added.len()
        + outcome.velocity_alerts.len()
        + outcome.aggregate_alerts.len()
}

/// One boundary-local index for payload lookups while building replay deltas.
struct DeltaLogTreeIndex<'a> {
    nodes: HashMap<SimThingId, &'a SimThing>,
    parents: HashMap<SimThingId, SimThingId>,
}

impl<'a> DeltaLogTreeIndex<'a> {
    fn new(root: &'a SimThing) -> Self {
        let mut index = Self {
            nodes: HashMap::new(),
            parents: HashMap::new(),
        };
        index.walk(root, None);
        index
    }

    fn walk(&mut self, node: &'a SimThing, parent: Option<SimThingId>) {
        self.nodes.insert(node.id, node);
        if let Some(parent) = parent {
            self.parents.insert(node.id, parent);
        }
        for child in &node.children {
            self.walk(child, Some(node.id));
        }
    }

    fn node(&self, id: SimThingId) -> Option<&'a SimThing> {
        self.nodes.get(&id).copied()
    }

    fn node_with_parent(&self, id: SimThingId) -> Option<(SimThingId, &'a SimThing)> {
        Some((*self.parents.get(&id)?, self.node(id)?))
    }

    fn overlay(&self, target: SimThingId, overlay_id: OverlayId) -> Option<&'a Overlay> {
        self.node(target)?
            .overlays
            .iter()
            .find(|overlay| overlay.id == overlay_id)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::BoundaryOutcome;
    use crate::fission::FissionOutcome;
    use crate::property_expiry::ExpiryOutcome;
    use crate::threshold_registry::AggregateAlertEvent;
    use crate::threshold_registry::VelocityAlertEvent;
    use simthing_core::{
        OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
        SimPropertyId, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };
    use simthing_feeder::MaintainerOutcome;

    fn empty_outcome() -> BoundaryOutcome {
        BoundaryOutcome::default()
    }

    fn empty_root() -> SimThing {
        SimThing::new(SimThingKind::World, 0)
    }

    fn make_overlay() -> Overlay {
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::System,
            affects: Vec::new(),
            transform: PropertyTransformDelta {
                property_id: SimPropertyId(0),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.5))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        }
    }

    fn count_matching<F: Fn(&BoundaryDeltaEntry) -> bool>(
        entries: &[BoundaryDeltaEntry],
        f: F,
    ) -> usize {
        entries.iter().filter(|e| f(e)).count()
    }

}
