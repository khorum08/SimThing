//! CPU reference evaluator — the oracle used to verify GPU output in Week 2.
//!
//! Rules:
//!   - Single-threaded and deterministic. No rayon here.
//!   - Reads properties and applies transforms in a consistent order.
//!   - Does NOT mutate the SimThing tree (no fission/fusion). That belongs to
//!     the day-boundary protocol.
//!   - Returns a `FieldSnapshot` so callers can diff against GPU output.

use crate::ids::SimPropertyId;
use crate::overlay::{OverlayLifecycle, PropertyTransformDelta};
use crate::property::{PropertyLayout, PropertyValue};
use crate::registry::DimensionRegistry;
use crate::simthing::SimThing;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Transform stack ───────────────────────────────────────────────────────────

/// The composed ancestor-transform context passed downward during tree traversal.
/// Accumulates deltas from root to current node; the leaf applies them all in order.
#[derive(Clone, Debug, Default)]
pub struct TransformStack {
    deltas: Vec<PropertyTransformDelta>,
}

impl TransformStack {
    pub fn push(&self, transform: &PropertyTransformDelta) -> Self {
        let mut next = self.clone();
        next.deltas.push(transform.clone());
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
        for delta in &self.deltas {
            if delta.property_id == prop_id {
                delta.apply_to_data(&mut value.data, layout);
            }
        }
    }
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
    pub day: u32,
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

    pub fn evaluate(&self, root: &SimThing, day: u32) -> FieldSnapshot {
        let mut entities = Vec::new();
        self.evaluate_node(root, &TransformStack::default(), &mut entities);
        FieldSnapshot { day, entities }
    }

    fn evaluate_node(
        &self,
        node: &SimThing,
        ancestors: &TransformStack,
        out: &mut Vec<EntitySnapshot>,
    ) {
        // 1. Compose this node's overlay transforms into the stack.
        let local_stack = node
            .overlays
            .iter()
            .fold(ancestors.clone(), |stack, overlay| {
                match &overlay.lifecycle {
                    OverlayLifecycle::Permanent => stack.push(&overlay.transform),
                    OverlayLifecycle::Transient { .. } => stack.push(&overlay.transform),
                    OverlayLifecycle::Suspended { .. } => stack,
                }
            });

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

        // 5. Apply the full ancestor + local transform stack to each property.
        for (id, pv) in &mut resolved {
            let layout = &self.registry.property(*id).layout;
            local_stack.apply_to(*id, pv, layout);
        }

        out.push(EntitySnapshot {
            id: node.id,
            properties: resolved,
        });

        // 6. Recurse children — they inherit the composed local_stack.
        for child in &node.children {
            self.evaluate_node(child, &local_stack, out);
        }
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
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        pv.data[a_off] = amount;
        cohort.add_property(lid, pv);
        cohort
    }

    /// Velocity integration: amount changes at `velocity * dt`
    #[test]
    fn velocity_integration() {
        let (reg, lid) = bootstrap();
        let layout = &reg.property(lid).layout;
        let v_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

        let mut cohort = make_cohort(&reg, lid, 0.5);
        cohort.property_mut(lid).unwrap().data[v_off] = 0.1;

        let eval = Evaluator::new(&reg, 1.0);
        let snap = eval.evaluate(&cohort, 1);

        let e = snap.get(cohort.id).unwrap();
        let a_off = reg
            .property(lid)
            .layout
            .offset_of(&SubFieldRole::Amount)
            .unwrap();
        let amount = e.properties[&lid].data[a_off];
        // 0.5 + 0.1 * 1.0 = 0.6
        assert!((amount - 0.6).abs() < 1e-5, "amount was {amount}");
    }

    /// Ancestor transforms propagate down: a world-level loyalty penalty
    /// (e.g. extraction policy) reaches a cohort two levels below.
    #[test]
    fn ancestor_transform_propagates() {
        let (reg, lid) = bootstrap();

        let world_overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            affects: vec![],
            transform: crate::overlay::PropertyTransformDelta {
                property_id: lid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(0.9))],
            },
            lifecycle: OverlayLifecycle::Permanent,
        };

        let mut world = SimThing::new(SimThingKind::World, 0);
        world.add_overlay(world_overlay);

        let cohort = make_cohort(&reg, lid, 1.0);
        let cohort_id = cohort.id;
        let mut location = SimThing::new(SimThingKind::Location, 0);
        location.add_child(cohort);
        world.add_child(location);

        let eval = Evaluator::new(&reg, 1.0);
        let snap = eval.evaluate(&world, 1);

        let e = snap.get(cohort_id).unwrap();
        let a_off = reg
            .property(lid)
            .layout
            .offset_of(&SubFieldRole::Amount)
            .unwrap();
        let amount = e.properties[&lid].data[a_off];
        // 1.0 * 0.9 = 0.9  (velocity=0, no integration change)
        assert!((amount - 0.9).abs() < 1e-5, "amount was {amount}");
    }

    /// Determinism: two evaluations of identical state produce identical output.
    #[test]
    fn deterministic() {
        let (reg, lid) = bootstrap();
        let v_off = reg
            .property(lid)
            .layout
            .offset_of(&SubFieldRole::Velocity)
            .unwrap();

        let mut loc = SimThing::new(SimThingKind::Location, 0);
        for _ in 0..4 {
            let mut c = make_cohort(&reg, lid, 0.7);
            c.property_mut(lid).unwrap().data[v_off] = -0.02;
            loc.add_child(c);
        }

        let eval = Evaluator::new(&reg, 1.0);
        let snap_a = eval.evaluate(&loc, 1);
        let snap_b = eval.evaluate(&loc, 1);

        for (a, b) in snap_a.entities.iter().zip(snap_b.entities.iter()) {
            for (pid, pv_a) in &a.properties {
                let pv_b = &b.properties[pid];
                for (x, y) in pv_a.data.iter().zip(pv_b.data.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits(), "non-deterministic float");
                }
            }
        }
    }

    /// Serialize → deserialize a FieldSnapshot and verify lossless round-trip.
    #[test]
    fn snapshot_round_trip() {
        let (reg, lid) = bootstrap();
        let a_off = reg
            .property(lid)
            .layout
            .offset_of(&SubFieldRole::Amount)
            .unwrap();
        let cohort = make_cohort(&reg, lid, 0.42);
        let eval = Evaluator::new(&reg, 1.0);
        let snap = eval.evaluate(&cohort, 5);

        let json = serde_json::to_string(&snap).expect("serialize");
        let back: FieldSnapshot = serde_json::from_str(&json).expect("deserialize");

        let original = snap.get(cohort.id).unwrap();
        let restored = back.get(cohort.id).unwrap();
        assert_eq!(
            original.properties[&lid].data[a_off],
            restored.properties[&lid].data[a_off]
        );
    }
}
