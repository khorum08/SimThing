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
use crate::tree_index::{node_at_path_mut, paths_preorder};
use simthing_gpu::{SlotAllocator, ThresholdEvent};
use std::collections::HashMap;

/// Results of one boundary's property expiry pass.
#[derive(Clone, Debug, Default)]
pub struct ExpiryOutcome {
    /// Properties removed from SimThing `properties` maps.
    pub properties_removed: u32,
    /// Registry columns tombstoned (only when last instance of that property expired).
    pub columns_tombstoned: u32,
    /// Properties removed via AfterTicks/TowardZero (non-threshold path).
    pub cpu_side_removals:  u32,
    /// Each property removal: `(sim_thing_id, property_id)`.
    pub expired: Vec<(SimThingId, SimPropertyId)>,
}

/// Step 5 main entry point. Process all PropertyExpiry threshold events.
/// Also runs a CPU-side AfterTicks / TowardZero sweep across the tree.
pub fn resolve_property_expiry(
    root:       &mut SimThing,
    registry:   &mut DimensionRegistry,
    allocator:  &SlotAllocator,
    values_shadow: &[f32],
    n_dims:     usize,
    events:     &[ThresholdEvent],
    cpu_reg:    &ThresholdRegistry,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> ExpiryOutcome {
    let mut out = ExpiryOutcome::default();

    // GPU-threshold-driven expiry.
    for event in events {
        let Some(sem) = cpu_reg.get(event.event_kind) else { continue };
        let ThresholdSemantic::PropertyExpiry { sim_thing_id, property_id } = sem else { continue };
        let (stid, pid) = (*sim_thing_id, *property_id);

        if remove_property(root, stid, pid, node_paths) {
            out.properties_removed += 1;
            out.expired.push((stid, pid));
            if !tree_has_property(root, pid, node_paths) {
                registry.tombstone(pid);
                out.columns_tombstoned += 1;
            }
        }
    }

    // CPU-side sweep: AfterTicks decay and TowardZero decay.
    cpu_decay_sweep(root, registry, allocator, values_shadow, n_dims, node_paths, &mut out);

    out
}

fn remove_property(
    root:   &mut SimThing,
    target: SimThingId,
    pid:    SimPropertyId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> bool {
    if let Some(paths) = node_paths {
        if let Some(path) = paths.get(&target) {
            return node_at_path_mut(root, path)
                .and_then(|node| node.remove_property(&pid))
                .is_some();
        }
        return false;
    }
    remove_property_from_tree(root, target, pid)
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

fn tree_has_property(
    root: &SimThing,
    pid: SimPropertyId,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> bool {
    if let Some(paths) = node_paths {
        use crate::tree_index::node_at_path;
        for path in paths_preorder(paths) {
            if let Some(node) = node_at_path(root, &path) {
                if node.properties.contains_key(&pid) {
                    return true;
                }
            }
        }
        return false;
    }
    tree_has_property_recursive(root, pid)
}

fn tree_has_property_recursive(node: &SimThing, pid: SimPropertyId) -> bool {
    if node.properties.contains_key(&pid) {
        return true;
    }
    node.children.iter().any(|c| tree_has_property_recursive(c, pid))
}

/// CPU-side decay that doesn't map to GPU thresholds:
/// - `DecayBehavior::AfterTicks { remaining: 0 }` — remove immediately.
/// - `DecayBehavior::TowardZero` — remove when |amount| < 1e-4.
///   (The rate is applied by Pass 1 velocity integration; we just check here.)
fn cpu_decay_sweep(
    root:     &mut SimThing,
    registry: &mut DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &[f32],
    n_dims: usize,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    out:      &mut ExpiryOutcome,
) {
    let mut removals = Vec::new();
    cpu_decay_collect(root, registry, allocator, values_shadow, n_dims, node_paths, &mut removals);

    let mut removed_pids = Vec::new();
    for (stid, pid) in removals {
        if remove_property(root, stid, pid, node_paths) {
            out.cpu_side_removals += 1;
            out.expired.push((stid, pid));
            removed_pids.push(pid);
        }
    }

    removed_pids.sort_by_key(|pid| pid.index());
    removed_pids.dedup();
    for pid in removed_pids {
        if !tree_has_property(root, pid, node_paths) {
            registry.tombstone(pid);
            out.columns_tombstoned += 1;
        }
    }
}

fn cpu_decay_collect(
    root:     &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &[f32],
    n_dims: usize,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
    out:      &mut Vec<(SimThingId, SimPropertyId)>,
) {
    if let Some(paths) = node_paths {
        use crate::tree_index::node_at_path;
        for path in paths_preorder(paths) {
            if let Some(node) = node_at_path(root, &path) {
                cpu_decay_collect_node(node, registry, allocator, values_shadow, n_dims, out);
            }
        }
        return;
    }
    cpu_decay_collect_recursive(root, registry, allocator, values_shadow, n_dims, out);
}

fn cpu_decay_collect_recursive(
    node:     &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &[f32],
    n_dims: usize,
    out:      &mut Vec<(SimThingId, SimPropertyId)>,
) {
    cpu_decay_collect_node(node, registry, allocator, values_shadow, n_dims, out);
    for child in &node.children {
        cpu_decay_collect_recursive(child, registry, allocator, values_shadow, n_dims, out);
    }
}

fn cpu_decay_collect_node(
    node:     &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &[f32],
    n_dims: usize,
    out:      &mut Vec<(SimThingId, SimPropertyId)>,
) {
    for (&pid, _pval) in &node.properties {
        if !registry.is_active(pid) { continue; }
        let prop = registry.property(pid);
        match &prop.decay {
            Some(DecayBehavior::AfterTicks { remaining: 0 }) => {
                out.push((node.id, pid));
            }
            Some(DecayBehavior::TowardZero { .. }) => {
                // Check the boundary-synchronized shadow, not stale semantic data.
                let layout = &prop.layout;
                if let Some(slot) = allocator.slot_of(node.id) {
                    let range = registry.column_range(pid);
                    if let Some(col) = range.col_for_role(&simthing_core::SubFieldRole::Amount, layout) {
                        let addr = slot as usize * n_dims + col;
                        if values_shadow.get(addr).map(|v| v.abs() < 1e-4).unwrap_or(false) {
                            out.push((node.id, pid));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DecayBehavior, DimensionRegistry, PropertyValue, SimProperty, SimThing, SimThingKind,
        SubFieldRole,
    };
    use crate::threshold_registry::ThresholdRegistry;
    use simthing_gpu::SlotAllocator;

    #[test]
    fn no_events_no_removals() {
        let mut reg  = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut root = SimThing::new(SimThingKind::World, 0);
        let alloc = SlotAllocator::new();
        let cpu_reg  = ThresholdRegistry::new();
        let n_dims = reg.total_columns;
        let out = resolve_property_expiry(&mut root, &mut reg, &alloc, &[], n_dims, &[], &cpu_reg, None);
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
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let n_dims = reg.total_columns;
        let shadow = vec![0.0; alloc.capacity() * n_dims];

        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 0, col: 0, value: 0.0, event_kind: ek,
        }];
        let out = resolve_property_expiry(
            &mut root,
            &mut reg,
            &alloc,
            &shadow,
            n_dims,
            &events,
            &cpu_reg,
            None,
        );

        assert_eq!(out.properties_removed, 1);
        assert_eq!(out.columns_tombstoned, 1);
        assert!(!reg.is_active(pid));
        assert!(root.children[0].properties.is_empty());
    }

    #[test]
    fn toward_zero_uses_shadow_value_not_stale_property_value() {
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.decay = Some(DecayBehavior::TowardZero { rate: 0.1 });
        let pid = reg.register(prop);
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let mut pval = PropertyValue::from_layout(&layout);
        pval.data[amount] = 0.5;
        cohort.add_property(pid, pval);
        let cohort_id = cohort.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0; alloc.capacity() * n_dims];
        let slot = alloc.slot_of(cohort_id).unwrap() as usize;
        shadow[slot * n_dims + amount] = 0.0;

        let out = resolve_property_expiry(
            &mut root,
            &mut reg,
            &alloc,
            &shadow,
            n_dims,
            &[],
            &ThresholdRegistry::new(),
            None,
        );

        assert_eq!(out.cpu_side_removals, 1);
        assert!(root.children[0].properties.is_empty());
        assert!(!reg.is_active(pid));
    }

    #[test]
    fn cpu_decay_keeps_registry_live_when_sibling_still_has_property() {
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "loyalty", 0);
        prop.decay = Some(DecayBehavior::TowardZero { rate: 0.1 });
        let pid = reg.register(prop);
        let layout = reg.property(pid).layout.clone();
        let amount = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut a = SimThing::new(SimThingKind::Cohort, 0);
        a.add_property(pid, reg.property(pid).default_value());
        let a_id = a.id;
        let mut b = SimThing::new(SimThingKind::Cohort, 0);
        b.add_property(pid, reg.property(pid).default_value());
        let b_id = b.id;

        let mut loc_a = SimThing::new(SimThingKind::Location, 0);
        let mut loc_b = SimThing::new(SimThingKind::Location, 0);
        loc_a.add_child(a);
        loc_b.add_child(b);

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(loc_a);
        root.add_child(loc_b);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0; alloc.capacity() * n_dims];
        let a_slot = alloc.slot_of(a_id).unwrap() as usize;
        let b_slot = alloc.slot_of(b_id).unwrap() as usize;
        shadow[a_slot * n_dims + amount] = 0.0;
        shadow[b_slot * n_dims + amount] = 0.5;

        let out = resolve_property_expiry(
            &mut root,
            &mut reg,
            &alloc,
            &shadow,
            n_dims,
            &[],
            &ThresholdRegistry::new(),
            None,
        );

        assert_eq!(out.cpu_side_removals, 1);
        assert_eq!(out.columns_tombstoned, 0);
        assert!(reg.is_active(pid));
    }

    #[test]
    fn indexed_path_matches_recursive_expiry() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::PropertyExpiry {
            sim_thing_id: cohort.id,
            property_id: pid,
        });

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let n_dims = reg.total_columns;
        let shadow = vec![0.0; alloc.capacity() * n_dims];
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 0,
            col: 0,
            value: 0.0,
            event_kind: ek,
        }];

        let mut recursive_root = root.clone();
        let mut recursive_reg = reg.clone();
        let paths = crate::tree_index::build_node_paths(&root);

        let indexed = resolve_property_expiry(
            &mut root,
            &mut reg,
            &alloc,
            &shadow,
            n_dims,
            &events,
            &cpu_reg,
            Some(&paths),
        );
        let recursive = resolve_property_expiry(
            &mut recursive_root,
            &mut recursive_reg,
            &alloc,
            &shadow,
            n_dims,
            &events,
            &cpu_reg,
            None,
        );

        assert_eq!(indexed.properties_removed, recursive.properties_removed);
        assert_eq!(indexed.columns_tombstoned, recursive.columns_tombstoned);
    }
}
