//! Overlay lifecycle management — step 4 and step 7 of the day boundary.
//!
//! ## Step 4: dissolve + writeback
//!
//! Each `Overlay` carries an `OverlayLifecycle`. At the boundary:
//! - `UntilDissolved` overlays are removed only by explicit removal (no automatic
//!   condition on the unit form).
//! - `UntilDissolvedWith { dissolution_conditions }` and
//!   `Transient { dissolution_conditions }` overlays are removed when *all*
//!   conditions are met. Conditions are AND-ed (all must be satisfied).
//!
//! `DissolveCondition` variants:
//! - `PropertyReaches { property, sub_field, value }` — true when the
//!   SimThing's property sub-field is ≥ the target value (for Rising direction).
//! - `PropertyBelow { property, sub_field, value }` — true when < value.
//! - `AfterTicks { remaining }` — authored duration. Production compares
//!   `deadline_generation = g_activation + duration` and never decrements.
//! - `OverrideReceived` — rejected at admission; not a dissolve arm.
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
    authored_after_ticks_duration, deadline_reached, establish_deadline, DimensionRegistry,
    DissolveCondition, GenerationStamp, OverlayId, OverlayLifecycle, SimThing, SimThingId,
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
/// 1. Compare AfterTicks deadlines (never decrement remaining).
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
    generation: u32,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> LifecycleOutcome {
    let mut deadlines = HashMap::new();
    resolve_overlay_lifecycle_oracle(
        root,
        registry,
        allocator,
        values_shadow,
        n_dims,
        GenerationStamp::new(generation),
        &mut deadlines,
        node_paths,
    )
}

/// CPU oracle: compare deadlines, never decrement AfterTicks.
pub fn resolve_overlay_lifecycle_oracle(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    generation: GenerationStamp,
    deadlines: &mut HashMap<OverlayId, GenerationStamp>,
    node_paths: Option<&HashMap<SimThingId, Vec<usize>>>,
) -> LifecycleOutcome {
    let mut out = LifecycleOutcome::default();
    if let Some(paths) = node_paths {
        for path in paths_preorder(paths) {
            if let Some(node) = node_at_path_mut(root, &path) {
                process_node(
                    node,
                    registry,
                    allocator,
                    values_shadow,
                    n_dims,
                    generation,
                    deadlines,
                    &mut out,
                );
            }
        }
    } else {
        resolve_node(
            root,
            registry,
            allocator,
            values_shadow,
            n_dims,
            generation,
            deadlines,
            &mut out,
        );
    }
    out
}

fn resolve_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    generation: GenerationStamp,
    deadlines: &mut HashMap<OverlayId, GenerationStamp>,
    out: &mut LifecycleOutcome,
) {
    process_node(
        node,
        registry,
        allocator,
        values_shadow,
        n_dims,
        generation,
        deadlines,
        out,
    );
    for child in &mut node.children {
        resolve_node(
            child,
            registry,
            allocator,
            values_shadow,
            n_dims,
            generation,
            deadlines,
            out,
        );
    }
}

fn process_node(
    node: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    generation: GenerationStamp,
    deadlines: &mut HashMap<OverlayId, GenerationStamp>,
    out: &mut LifecycleOutcome,
) {
    let slot = allocator.slot_of(node.id);
    let base = slot.map(|s| s.as_usize() * n_dims);

    let mut dissolved_indices = Vec::new();
    {
        let should_dissolve: Vec<bool> = node
            .overlays
            .iter()
            .map(|overlay| {
                if let Some(duration) = authored_after_ticks_duration(&overlay.lifecycle) {
                    deadlines.entry(overlay.id).or_insert_with(|| {
                        establish_deadline(generation, duration)
                            .unwrap_or(GenerationStamp::new(u32::MAX))
                    });
                }
                match &overlay.lifecycle {
                    OverlayLifecycle::Transient {
                        dissolution_conditions,
                    }
                    | OverlayLifecycle::UntilDissolvedWith {
                        dissolution_conditions,
                    } => dissolution_conditions.iter().all(|cond| {
                        evaluate_condition(
                            cond,
                            node,
                            registry,
                            values_shadow,
                            base,
                            generation,
                            deadlines.get(&overlay.id).copied(),
                        )
                    }),
                    _ => false,
                }
            })
            .collect();

        // AfterTicks remaining is authored duration. Never decrement.
        out.after_ticks_decremented = 0;
        for (i, should) in should_dissolve.into_iter().enumerate() {
            if should {
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
    generation: GenerationStamp,
    deadline: Option<GenerationStamp>,
) -> bool {
    match cond {
        DissolveCondition::AtSessionEnd => false,
        DissolveCondition::OverrideReceived => false,
        DissolveCondition::AfterTicks { remaining } => {
            if let Some(deadline) = deadline {
                deadline_reached(generation, deadline)
            } else {
                *remaining == 0
            }
        }
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
    shadow.get(base + col.raw()).copied()
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
                    if let Some(v) = shadow.get_mut(base + col.raw()) {
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
                    if let Some(v) = shadow.get_mut(base + col.raw()) {
                        *v = *value;
                    }
                }
            }
        }
    }
}

/// Route a new overlay from its required origin to a target SimThing.
/// Returns `true` only when both endpoints belong to the supplied tree.
pub fn attach_overlay(
    root: &mut SimThing,
    target: SimThingId,
    overlay: simthing_core::Overlay,
) -> bool {
    simthing_core::deliver_routed_overlay(root, target, overlay).is_ok()
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
            origin: SimThingId::new(),
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.1))],
            },
            lifecycle,
        }
    }

}
