//! Property expiry — step 5 of the day boundary.
//!
//! Consumes `ThresholdEvent`s whose `event_kind` maps to
//! `ThresholdSemantic::PropertyExpiry`. For each such event:
//!
//! 1. Remove the property from the target SimThing's `properties` HashMap.
//! 2. If no other live SimThing in the tree carries that property, tombstone
//!    the `DimensionRegistry` column.
//!
//! Also handles `DecayBehavior::AfterTicks` and `DecayBehavior::TowardZero`
//! which are not GPU-threshold-based:
//! - `AfterTicks`: the lifecycle manager already decrements `remaining`; here
//!   we remove the property from any SimThing where `remaining == 0` in the
//!   `DecayBehavior` stored in the `SimProperty` definition. Wait — `remaining`
//!   is on `DissolveCondition`, not the decay behavior. The decay behavior
//!   `AfterTicks { remaining }` works the same way: checked here each boundary.
//! - `TowardZero`: not threshold-based; we check if amount is effectively zero
//!   (|amount| < epsilon) each boundary and remove then.

use simthing_core::{DecayBehavior, DimensionRegistry, SimPropertyId, SimThing, SimThingId};
use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
use simthing_gpu::ThresholdEvent;

/// Results of one boundary's property expiry pass.
#[derive(Clone, Debug, Default)]
pub struct ExpiryOutcome {
    /// Properties removed from SimThing `properties` maps.
    pub properties_removed: u32,
    /// Registry columns tombstoned (only when last instance of that property expired).
    pub columns_tombstoned: u32,
    /// Properties removed via AfterTicks/TowardZero (non-threshold path).
    pub cpu_side_removals:  u32,
}

/// Step 5 main entry point. Process all PropertyExpiry threshold events.
/// Also runs a CPU-side AfterTicks / TowardZero sweep across the tree.
pub fn resolve_property_expiry(
    root:       &mut SimThing,
    registry:   &mut DimensionRegistry,
    events:     &[ThresholdEvent],
    cpu_reg:    &ThresholdRegistry,
) -> ExpiryOutcome {
    let mut out = ExpiryOutcome::default();

    // GPU-threshold-driven expiry.
    for event in events {
        let Some(sem) = cpu_reg.get(event.event_kind) else { continue };
        let ThresholdSemantic::PropertyExpiry { sim_thing_id, property_id } = sem else { continue };
        let (stid, pid) = (*sim_thing_id, *property_id);

        if remove_property_from_tree(root, stid, pid) {
            out.properties_removed += 1;
            if !tree_has_property(root, pid) {
                registry.tombstone(pid);
                out.columns_tombstoned += 1;
            }
        }
    }

    // CPU-side sweep: AfterTicks decay and TowardZero decay.
    cpu_decay_sweep(root, registry, &mut out);

    out
}

fn remove_property_from_tree(
    node:   &mut SimThing,
    target: SimThingId,
    pid:    SimPropertyId,
) -> bool {
    if node.id == target {
        return node.remove_property(&pid).is_some();
    }
    for child in &mut node.children {
        if remove_property_from_tree(child, target, pid) { return true; }
    }
    false
}

fn tree_has_property(node: &SimThing, pid: SimPropertyId) -> bool {
    if node.properties.contains_key(&pid) { return true; }
    node.children.iter().any(|c| tree_has_property(c, pid))
}

/// CPU-side decay that doesn't map to GPU thresholds:
/// - `DecayBehavior::AfterTicks { remaining: 0 }` — remove immediately.
/// - `DecayBehavior::TowardZero` — remove when |amount| < 1e-4.
///   (The rate is applied by Pass 1 velocity integration; we just check here.)
fn cpu_decay_sweep(
    root:     &mut SimThing,
    registry: &mut DimensionRegistry,
    out:      &mut ExpiryOutcome,
) {
    cpu_decay_node(root, registry, out);
}

fn cpu_decay_node(
    node:     &mut SimThing,
    registry: &mut DimensionRegistry,
    out:      &mut ExpiryOutcome,
) {
    let mut to_remove: Vec<SimPropertyId> = Vec::new();

    for (&pid, pval) in &node.properties {
        if !registry.is_active(pid) { continue; }
        let prop = registry.property(pid);
        match &prop.decay {
            Some(DecayBehavior::AfterTicks { remaining: 0 }) => {
                to_remove.push(pid);
            }
            Some(DecayBehavior::TowardZero { .. }) => {
                // Check amount col.
                let layout = &prop.layout;
                let offset = layout.offset_of(&simthing_core::SubFieldRole::Amount);
                if let Some(off) = offset {
                    if pval.data.get(off).map(|v| v.abs() < 1e-4).unwrap_or(false) {
                        to_remove.push(pid);
                    }
                }
            }
            _ => {}
        }
    }

    for pid in to_remove {
        node.remove_property(&pid);
        out.cpu_side_removals += 1;
        if !tree_has_property(node, pid) {
            registry.tombstone(pid);
        }
    }

    for child in &mut node.children {
        cpu_decay_node(child, registry, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, SimProperty, SimThing, SimThingKind,
    };
    use crate::threshold_registry::ThresholdRegistry;

    #[test]
    fn no_events_no_removals() {
        let mut reg  = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let cpu_reg  = ThresholdRegistry::new();
        let out = resolve_property_expiry(&mut root, &mut reg, &[], &cpu_reg);
        assert_eq!(out.properties_removed, 0);
        assert_eq!(out.columns_tombstoned, 0);
    }

    #[test]
    fn property_expiry_event_removes_and_tombstones() {
        let mut reg = DimensionRegistry::new();
        let pid     = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let pval = reg.property(pid).default_value();
        cohort.add_property(pid, pval);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::PropertyExpiry {
            sim_thing_id: cohort.id,
            property_id:  pid,
        });

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);

        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 0, col: 0, value: 0.0, event_kind: ek,
        }];
        let out = resolve_property_expiry(&mut root, &mut reg, &events, &cpu_reg);

        assert_eq!(out.properties_removed, 1);
        assert_eq!(out.columns_tombstoned, 1);
        assert!(!reg.is_active(pid));
        assert!(root.children[0].properties.is_empty());
    }
}
