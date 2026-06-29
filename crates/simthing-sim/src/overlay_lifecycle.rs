//! Overlay lifecycle management — step 4 and step 7 of the day boundary.
//!
//! ## Step 4: dissolve + writeback
//!
//! Each `Overlay` carries an `OverlayLifecycle`. At the boundary:
//! - `Permanent` overlays are never removed.
//! - `Transient { dissolution_conditions }` overlays are removed when *all*
//!   conditions are met. Conditions are AND-ed (all must be satisfied).
//!
//! `DissolveCondition` variants:
//! - `PropertyReaches { property, sub_field, value }` — true when the
//!   SimThing's property sub-field is ≥ the target value (for Rising direction).
//! - `PropertyBelow { property, sub_field, value }` — true when < value.
//! - `AfterTicks { remaining }` — true when remaining reaches 0. The
//!   lifecycle manager decrements `remaining` by 1 each boundary.
//! - `OverrideReceived` — true when a new instruction overlay replaces this
//!   one. Checked by the `AttachOverlay` handler in step 7.
//! - `Never` — always false; the overlay persists until explicitly removed.
//!
//! When an overlay dissolves, its `on_expire` `ExpireEffect`s are applied
//! to the CPU shadow. These are small velocity bumps or intensity sets that
//! model "what happens when this policy ends."
//!
//! ## Step 7: attach new overlays
//!
//! `BoundaryRequest::AttachOverlay` items are applied here. Each carries a
//! new `Overlay` and a target `SimThingId`. Attachment is the only within-day
//! structural change that doesn't require a slot mutation.
//!
//! After step 7 the overlay list for every SimThing is authoritative for day
//! N+1. `gpu_sync` then calls `build_overlay_deltas` to reflect those lists.

use simthing_core::{
    DimensionRegistry, DissolveCondition, OverlayId, OverlayLifecycle, SimThing, SimThingId,
    SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use std::collections::HashMap;

use crate::tree_index::{node_at_path_mut, paths_preorder};

/// Counts from one boundary's lifecycle pass.
#[derive(Clone, Debug, Default)]
pub struct LifecycleOutcome {
    pub dissolved: u32,
    pub dissolved_overlays: Vec<(SimThingId, OverlayId)>,
    pub after_ticks_decremented: u32,
    pub overlays_attached: u32,
}

/// Walk the tree and:
/// 1. Decrement AfterTicks counters on all transient overlays.
/// 2. Remove overlays whose dissolution conditions are all met.
/// 3. Apply `on_expire` effects from dissolved overlays to the CPU shadow.
///
/// `values_shadow` is the `DispatchCoordinator::shadow` slice; it is mutated
/// directly for ExpireEffect writes. The dirty-row bitmap in `TransformPatcher`
/// is NOT updated here — callers must call `mark_slot_dirty` or use
/// `upload_full_shadow` at boundary end.
///
/// `day` is the current day counter (used for future AfterDays conditions).
pub fn resolve_overlay_lifecycle(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    _day: u32,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> LifecycleOutcome {
    let mut out = LifecycleOutcome::default();
    if let Some(paths) = node_paths {
        for path in paths_preorder(paths) {
            if let Some(node) = node_at_path_mut(root, &path) {
                process_node(node, registry, allocator, values_shadow, n_dims, &mut out);
            }
        }
    } else {
        resolve_node(root, registry, allocator, values_shadow, n_dims, &mut out);
    }
    out
}

fn resolve_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut LifecycleOutcome,
) {
    process_node(node, registry, allocator, values_shadow, n_dims, out);
    for child in &mut node.children {
        resolve_node(child, registry, allocator, values_shadow, n_dims, out);
    }
}

fn process_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut LifecycleOutcome,
) {
    let slot = allocator.slot_of(node.id);
    let base = slot.map(|s| s.as_usize() * n_dims);

    // First pass (immutable): check which overlays should dissolve.
    // We separate condition evaluation (needs immutable `node`) from
    // AfterTicks decrement (needs mutable overlay) to satisfy the borrow
    // checker — `evaluate_condition` borrows `node` immutably, but
    // iterating `node.overlays.iter_mut()` would hold a mutable borrow.
    let mut dissolved_indices = Vec::new();
    {
        let should_dissolve: Vec<bool> = node
            .overlays
            .iter()
            .map(|overlay| {
                if let OverlayLifecycle::Transient {
                    dissolution_conditions,
                } = &overlay.lifecycle
                {
                    dissolution_conditions
                        .iter()
                        .all(|cond| evaluate_condition(cond, node, registry, values_shadow, base))
                } else {
                    false
                }
            })
            .collect();

        // Second sub-pass (mutable): decrement AfterTicks on surviving overlays.
        for (i, overlay) in node.overlays.iter_mut().enumerate() {
            if let OverlayLifecycle::Transient {
                dissolution_conditions,
            } = &mut overlay.lifecycle
            {
                for cond in dissolution_conditions.iter_mut() {
                    if let DissolveCondition::AfterTicks { remaining } = cond {
                        if *remaining > 0 {
                            *remaining -= 1;
                            out.after_ticks_decremented += 1;
                        }
                    }
                }
            }
            if should_dissolve[i] {
                dissolved_indices.push(i);
            }
        }
    }

    // Second pass (reverse): remove dissolved overlays + apply expire effects.
    for i in dissolved_indices.into_iter().rev() {
        let overlay = node.overlays.remove(i);
        out.dissolved += 1;
        out.dissolved_overlays.push((node.id, overlay.id));

        // Apply on_expire effects to the CPU shadow if we have a slot.
        if let Some(base) = base {
            let pid = overlay.transform.property_id;
            if let Some(prop) = registry
                .try_property(pid)
                .filter(|_| registry.is_active(pid))
            {
                if let Some(handler) = prop.on_expire.as_ref() {
                    apply_expire_effects(handler, registry, values_shadow, base, n_dims);
                }
            }
        }
    }
}

fn evaluate_condition(
    cond: &DissolveCondition,
    node: &SimThing,
    registry: &DimensionRegistry,
    values_shadow: &[f32],
    base: Option<usize>,
) -> bool {
    match cond {
        DissolveCondition::Never => false,
        DissolveCondition::OverrideReceived => false, // handled by attach step
        DissolveCondition::AfterTicks { remaining } => *remaining == 0,
        DissolveCondition::PropertyReaches {
            property,
            sub_field,
            value,
        } => read_sub_field(node, registry, values_shadow, base, *property, sub_field)
            .map(|v| v >= *value)
            .unwrap_or(false),
        DissolveCondition::PropertyBelow {
            property,
            sub_field,
            value,
        } => read_sub_field(node, registry, values_shadow, base, *property, sub_field)
            .map(|v| v < *value)
            .unwrap_or(false),
    }
}

/// Read a sub-field value from the CPU shadow. Prefers shadow over
/// SimThing::properties because the shadow reflects GPU integration output.
fn read_sub_field(
    node: &SimThing,
    registry: &DimensionRegistry,
    shadow: &[f32],
    base: Option<usize>,
    pid: simthing_core::SimPropertyId,
    role: &SubFieldRole,
) -> Option<f32> {
    if !node.properties.contains_key(&pid) {
        return None;
    }
    if !registry.is_active(pid) {
        return None;
    }
    let base = base?;
    let range = registry.try_column_range(pid)?;
    let layout = &registry.try_property(pid)?.layout;
    let col = range.col_for_role(role, layout)?;
    shadow.get(base + col).copied()
}

fn apply_expire_effects(
    handler: &simthing_core::ExpireHandler,
    registry: &DimensionRegistry,
    shadow: &mut [f32],
    base: usize,
    _n_dims: usize,
) {
    for effect in &handler.write_back {
        match effect {
            simthing_core::ExpireEffect::AddVelocity {
                property,
                sub_field,
                delta,
            } => {
                if !registry.is_active(*property) {
                    continue;
                }
                let Some(range) = registry.try_column_range(*property) else {
                    continue;
                };
                let Some(prop) = registry.try_property(*property) else {
                    continue;
                };
                let layout = &prop.layout;
                if let Some(col) = range.col_for_role(sub_field, layout) {
                    if let Some(v) = shadow.get_mut(base + col) {
                        *v += delta;
                    }
                }
            }
            simthing_core::ExpireEffect::SetIntensity { property, value } => {
                if !registry.is_active(*property) {
                    continue;
                }
                let Some(range) = registry.try_column_range(*property) else {
                    continue;
                };
                let Some(prop) = registry.try_property(*property) else {
                    continue;
                };
                let layout = &prop.layout;
                if let Some(col) = range.col_for_role(&SubFieldRole::Intensity, layout) {
                    if let Some(v) = shadow.get_mut(base + col) {
                        *v = *value;
                    }
                }
            }
        }
    }
}

/// Attach a new overlay to a target SimThing anywhere in the tree.
/// Returns `true` if the target was found.
pub fn attach_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay: simthing_core::Overlay,
) -> bool {
    if root.id == target {
        root.add_overlay(overlay);
        return true;
    }
    for child in &mut root.children {
        if attach_overlay(child, target, overlay.clone()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
        OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingKind,
        SubFieldRole, TransformOp,
    };
    use simthing_gpu::SlotAllocator;

    fn make_overlay(lifecycle: OverlayLifecycle, pid: SimPropertyId) -> Overlay {
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Transient,
            source: OverlaySource::System,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.1))],
            },
            lifecycle,
        }
    }

    #[test]
    fn permanent_overlays_are_never_dissolved() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(OverlayLifecycle::Permanent, pid));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);
        assert_eq!(out.dissolved, 0);
        assert_eq!(root.overlays.len(), 1);
    }

    #[test]
    fn after_ticks_zero_dissolves_overlay() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 0 }],
            },
            pid,
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);
        assert_eq!(out.dissolved, 1);
        assert!(root.overlays.is_empty());
    }

    #[test]
    fn after_ticks_nonzero_decrements_and_survives() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 2 }],
            },
            pid,
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);
        assert_eq!(out.dissolved, 0);
        assert_eq!(root.overlays.len(), 1);
        // Remaining should have decremented to 1.
        if let OverlayLifecycle::Transient {
            dissolution_conditions,
        } = &root.overlays[0].lifecycle
        {
            if let DissolveCondition::AfterTicks { remaining } = &dissolution_conditions[0] {
                assert_eq!(*remaining, 1);
            }
        }
    }

    #[test]
    fn attach_overlay_finds_nested_target() {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        let target_id = cohort.id;
        loc.add_child(cohort);
        root.add_child(loc);

        let pid = SimPropertyId(0);
        let overlay = make_overlay(OverlayLifecycle::Permanent, pid);
        assert!(attach_overlay(&mut root, target_id, overlay));
        // Cohort is at root.children[0].children[0]
        assert_eq!(root.children[0].children[0].overlays.len(), 1);
    }

    #[test]
    fn property_below_absent_property_does_not_dissolve() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "food", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::PropertyBelow {
                    property: pid,
                    sub_field: SubFieldRole::Amount,
                    value: 0.2,
                }],
            },
            pid,
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);

        assert_eq!(out.dissolved, 0);
        assert_eq!(root.overlays.len(), 1);
    }

    #[test]
    fn property_reaches_absent_property_does_not_dissolve() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "food", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::PropertyReaches {
                    property: pid,
                    sub_field: SubFieldRole::Amount,
                    value: 0.0,
                }],
            },
            pid,
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);

        assert_eq!(out.dissolved, 0);
        assert_eq!(root.overlays.len(), 1);
    }

    #[test]
    fn property_below_present_zero_property_does_dissolve_if_below() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "food", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_property(pid, reg.property(pid).default_value());
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::PropertyBelow {
                    property: pid,
                    sub_field: SubFieldRole::Amount,
                    value: 0.2,
                }],
            },
            pid,
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);

        assert_eq!(out.dissolved, 1);
        assert!(root.overlays.is_empty());
    }

    #[test]
    fn dissolving_overlay_with_invalid_property_id_does_not_panic() {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "food", 0));
        let mut alloc = SlotAllocator::new();
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_overlay(make_overlay(
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 0 }],
            },
            SimPropertyId(999),
        ));

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; n_dims];
        let out = resolve_overlay_lifecycle(&mut root, &reg, &alloc, &mut shadow, n_dims, 0, None);

        assert_eq!(out.dissolved, 1);
        assert!(root.overlays.is_empty());
    }
}
