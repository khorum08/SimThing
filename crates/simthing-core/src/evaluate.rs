//! CPU reference evaluator — the oracle used to verify GPU output in Week 2.
//!
//! Rules:
//!   - Single-threaded and deterministic. No rayon here.
//!   - Reads properties and applies transforms in a consistent order.
//!   - Does NOT mutate the SimThing tree (no fission/fusion). That belongs to
//!     the generation-boundary protocol.
//!   - Returns a `FieldSnapshot` so callers can diff against GPU output.

use crate::ids::SimPropertyId;
use crate::overlay::{Overlay, OverlayKind, PropertyTransformDelta};
use crate::property::{PropertyLayout, PropertyValue};
use crate::registry::DimensionRegistry;
use crate::simthing::{walk_inherited_until, SimThing};
use crate::{inherit_active_overlays, LiveOverlayRoutes};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Transform stack ───────────────────────────────────────────────────────────

/// The composed ancestor-transform context passed downward during tree traversal.
/// Accumulates deltas from root to current node; the leaf applies them all in order.
#[derive(Clone, Debug, Default)]
pub struct TransformStack {
    deltas: Vec<StackedTransform>,
}

#[derive(Clone, Debug)]
struct StackedTransform {
    delta: PropertyTransformDelta,
    predicate_restriction: bool,
}

impl TransformStack {
    pub fn push(&self, transform: &PropertyTransformDelta) -> Self {
        let mut next = self.clone();
        next.deltas.push(StackedTransform {
            delta: transform.clone(),
            predicate_restriction: false,
        });
        next
    }

    /// Push one overlay while retaining whether it is a policy restriction.
    /// Numeric value composition remains sequential and last-wins-capable; the
    /// marker is consulted only by routed predicate evaluation.
    pub fn push_overlay(&self, overlay: &Overlay) -> Self {
        let mut next = self.clone();
        next.deltas.push(StackedTransform {
            delta: overlay.transform.clone(),
            predicate_restriction: matches!(
                overlay.kind,
                OverlayKind::Policy | OverlayKind::Governance
            ),
        });
        next
    }

    /// Apply all accumulated transforms to a mutable property value.
    /// Delegates offset arithmetic to layout — no hardcoded indices here.
    pub fn apply_to(
        &self,
        prop_id: SimPropertyId,
        value: &mut PropertyValue,
        layout: &PropertyLayout,
    ) {
        for stacked in &self.deltas {
            if stacked.delta.property_id == prop_id {
                stacked.delta.apply_to_data(value.raw_lanes_mut(), layout);
            }
        }
    }

    /// Test a routed predicate under the explicit policy-chain rule.
    ///
    /// The selector must hold for the unmodified candidate and after every
    /// matching policy/governance restriction. Results are conjoined, so once
    /// an ancestor rejects a candidate no descendant transform can restore it.
    /// This is intentionally distinct from ordinary value composition, where
    /// a later `Set` may overwrite an earlier operation.
    pub fn allows_routed_predicate(
        &self,
        predicate: &RoutedPredicate,
        value: &PropertyValue,
        layout: &PropertyLayout,
    ) -> bool {
        let Some(offset) = layout.offset_of(&predicate.sub_field) else {
            return false;
        };
        let Some(mut candidate) = value
            .raw_lanes_for_serialization()
            .get(offset.lane())
            .copied()
        else {
            return false;
        };
        let mut allowed = predicate.comparison.matches(candidate, predicate.threshold);
        for stacked in &self.deltas {
            if stacked.delta.property_id != predicate.property_id {
                continue;
            }
            for (role, op) in &stacked.delta.sub_field_deltas {
                if role != &predicate.sub_field {
                    continue;
                }
                candidate = op.apply(candidate);
                if stacked.predicate_restriction {
                    allowed &= predicate.comparison.matches(candidate, predicate.threshold);
                }
            }
        }
        allowed && predicate.comparison.matches(candidate, predicate.threshold)
    }
}

/// Numeric comparison used by the paid, one-walk predicate-broadcast mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutedPredicateComparison {
    AtLeast,
    AtMost,
    EqualBits,
}

impl RoutedPredicateComparison {
    fn matches(self, value: f32, threshold: f32) -> bool {
        match self {
            Self::AtLeast => value >= threshold,
            Self::AtMost => value <= threshold,
            Self::EqualBits => value.to_bits() == threshold.to_bits(),
        }
    }
}

/// Generic property selector for predicate-broadcast reception.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutedPredicate {
    pub property_id: SimPropertyId,
    pub sub_field: crate::property::SubFieldRole,
    pub comparison: RoutedPredicateComparison,
    pub threshold: f32,
}

// ── FieldSnapshot ─────────────────────────────────────────────────────────────

/// Post-evaluation snapshot of one SimThing: fully-resolved property values
/// after ancestor transforms, velocity integration, and local overlay application.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: crate::ids::SimThingId,
    pub properties: HashMap<SimPropertyId, PropertyValue>,
}

/// Complete evaluated world state. Used as oracle vs. GPU output.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldSnapshot {
    /// Generation stamp for this snapshot (P0 generation ruling).
    /// Serde alias preserves the historical wire field name for load compatibility.
    #[serde(alias = "day")]
    pub generation: u32,
    pub entities: Vec<EntitySnapshot>,
}

impl FieldSnapshot {
    pub fn get(&self, id: crate::ids::SimThingId) -> Option<&EntitySnapshot> {
        self.entities.iter().find(|e| e.id == id)
    }
}

// ── Evaluator ─────────────────────────────────────────────────────────────────

pub struct Evaluator<'r> {
    registry: &'r DimensionRegistry,
    delta_time: f32,
}

impl<'r> Evaluator<'r> {
    pub fn new(registry: &'r DimensionRegistry, delta_time: f32) -> Self {
        Self {
            registry,
            delta_time,
        }
    }

    pub fn evaluate(&self, root: &SimThing, generation: u32) -> FieldSnapshot {
        let mut entities = Vec::new();
        let live_routes = LiveOverlayRoutes::for_tree(root);
        let seed = TransformStack::default();
        let walked: Result<Option<()>, std::convert::Infallible> = walk_inherited_until(
            root,
            &seed,
            &mut |node, ancestors| Ok(inherit_active_overlays(node, ancestors)),
            &mut |node, effective| {
                self.evaluate_node(node, effective, live_routes.as_ref(), &mut entities);
                Ok(None)
            },
        );
        match walked {
            Ok(_) => {}
            Err(never) => match never {},
        }
        FieldSnapshot {
            generation,
            entities,
        }
    }

    fn evaluate_node(
        &self,
        node: &SimThing,
        local_stack: &TransformStack,
        live_routes: Option<&LiveOverlayRoutes<'_>>,
        out: &mut Vec<EntitySnapshot>,
    ) {
        // 1. Compose this node's overlay transforms into the stack.
        // 2. Clone this node's properties.
        let mut resolved: HashMap<SimPropertyId, PropertyValue> = node
            .properties
            .iter()
            .map(|(id, pv)| (*id, pv.clone()))
            .collect();

        // 3. Velocity integration — layout-aware, no hardcoded indices.
        for (id, pv) in &mut resolved {
            let prop = self.registry.property(*id);
            pv.integrate(&prop.layout, self.delta_time);
        }

        // 4. Intensity update.
        for (id, pv) in &mut resolved {
            let prop = self.registry.property(*id);
            if let Some(ib) = &prop.intensity_behavior {
                pv.update_intensity(ib, &prop.layout, self.delta_time);
            }
        }

        // 5. Derive a routed order at most once for this node, then apply the
        // full ancestor + local transform stack to each property.
        let routed_overlays =
            live_routes.and_then(|routes| routes.ordered_active_overlays(node.id));
        for (id, pv) in &mut resolved {
            let layout = &self.registry.property(*id).layout;
            if let Some(overlays) = routed_overlays.as_ref() {
                for overlay in overlays {
                    if overlay.transform.property_id == *id {
                        overlay.transform.apply_to_data(pv.raw_lanes_mut(), layout);
                    }
                }
            } else {
                local_stack.apply_to(*id, pv, layout);
            }
        }

        out.push(EntitySnapshot {
            id: node.id,
            properties: resolved,
        });

        // 6. Recurse children — they inherit the composed local_stack.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::OverlayId;
    use crate::overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource};
    use crate::property::{SimProperty, SubFieldRole, TransformOp};
    use crate::registry::DimensionRegistry;
    use crate::simthing::{SimThing, SimThingKind};

    fn bootstrap() -> (DimensionRegistry, SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let lid = reg.register(SimProperty::simple("core", "loyalty", 3));
        (reg, lid)
    }

    fn make_cohort(reg: &DimensionRegistry, lid: SimPropertyId, amount: f32) -> SimThing {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let prop = reg.property(lid);
        let layout = &prop.layout;
        let mut pv = prop.default_value();
        pv.set_role(&SubFieldRole::Amount, layout, amount);
        cohort.add_property(lid, pv);
        cohort
    }
    /// SESSION-WIRING-KILL-SWEEP-0: historical wire key loads into generation stamp.
    #[test]
    fn field_snapshot_deserializes_legacy_generation_wire_alias() {
        // Fixture retains the historical JSON key only; identifier is generation-vocabulary.
        let json = r#"{"day":7,"entities":[]}"#;
        let snap: FieldSnapshot =
            serde_json::from_str(json).expect("legacy generation wire alias load");
        assert_eq!(snap.generation, 7);
    }
}
