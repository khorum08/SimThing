//! Read-only observability query for a single SimThing.
//!
//! Answers the §13 design question "why is X high on Y?" by decomposing:
//! - Current sub-field values (from a caller-supplied row slice).
//! - Which overlays affect each property, attributed to ancestor vs local.
//!
//! ## Value fidelity
//!
//! [`ObserveFidelity::Shadow`] (default via `BoundaryProtocol::observe`) reads
//! the CPU shadow row — cheap, but may lag mid-tick GPU integration. Rows
//! touched by the normal **intent-delta** tick path are not written to shadow
//! until the next boundary readback; shadow-path legacy patches can still be
//! current mid-tick.
//!
//! [`ObserveFidelity::GpuRow`] (`BoundaryProtocol::observe_live`) reads one
//! integrated row from the GPU via `read_values_row` — for UI/debug when
//! mid-tick values on intent-patched entities must be exact. See
//! `docs/state-authority.md` §Observability mid-tick.

use crate::sim_runtime_tree::SimRuntimeTree;
use simthing_core::{
    DimensionRegistry, OverlayId, OverlaySource, SimPropertyId, SimThing, SimThingId, SubFieldRole,
    TransformOp,
};
use simthing_gpu::SlotAllocator;

// ── Fidelity ──────────────────────────────────────────────────────────────────

/// Where numeric sub-field values are sourced for an observability query.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ObserveFidelity {
    /// CPU shadow row (cheap; may lag mid-tick integration).
    #[default]
    Shadow,
    /// One GPU row readback for the target slot (live integrated values).
    GpuRow,
}

// ── Report types ──────────────────────────────────────────────────────────────

/// Snapshot of one sub-field's current value.
#[derive(Clone, Debug)]
pub struct SubFieldObservation {
    pub role: SubFieldRole,
    pub value: f32,
}

/// One overlay's contribution to a property on the queried SimThing.
#[derive(Clone, Debug)]
pub struct OverlayContribution {
    pub overlay_id: OverlayId,
    pub source: OverlaySource,
    /// True when this overlay currently participates in GPU overlay prep.
    pub active: bool,
    /// The transform ops this overlay applies to the property's sub-fields.
    pub deltas: Vec<(SubFieldRole, TransformOp)>,
    /// True when the overlay lives on an ancestor node, not the queried node.
    pub inherited: bool,
}

/// All observations for one property on the queried SimThing.
#[derive(Clone, Debug)]
pub struct PropertyObservation {
    pub property_id: SimPropertyId,
    /// `"namespace::name"` display label.
    pub property_name: String,
    pub sub_fields: Vec<SubFieldObservation>,
    pub overlay_contributions: Vec<OverlayContribution>,
}

/// Full observability report for one SimThing.
#[derive(Clone, Debug)]
pub struct ObservabilityReport {
    pub sim_thing_id: SimThingId,
    pub properties: Vec<PropertyObservation>,
}

// ── Core query ────────────────────────────────────────────────────────────────

/// Build an observability report for `target`.
///
/// `target_row` must hold the target slot's `[n_dims]` values (shadow slice or
/// GPU readback). Overlay attribution always comes from the tree.
///
/// Returns `None` when `target` is not in the tree or has no allocated slot.
pub fn observe(
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    target_row: &[f32],
    target: SimThingId,
) -> Option<ObservabilityReport> {
    let path = find_path(root.inner(), target)?;
    let node = *path.last()?;

    let _slot = allocator.slot_of(node.id)?;

    let mut properties = Vec::new();

    for (&pid, _) in &node.properties {
        if !registry.is_active(pid) {
            continue;
        }
        let prop = registry.property(pid);
        let range = registry.column_range(pid);
        let layout = &prop.layout;

        let sub_fields: Vec<SubFieldObservation> = layout
            .sub_fields
            .iter()
            .filter_map(|spec| {
                let col = range.col_for_role(&spec.role, layout)?;
                Some(SubFieldObservation {
                    role: spec.role.clone(),
                    value: target_row.get(col).copied().unwrap_or(0.0),
                })
            })
            .collect();

        // Overlay contributions: walk ancestor chain then local overlays.
        let mut contributions = Vec::new();
        for (depth, &ancestor) in path.iter().enumerate() {
            let inherited = depth < path.len() - 1;
            for overlay in &ancestor.overlays {
                if overlay.transform.property_id != pid {
                    continue;
                }
                contributions.push(OverlayContribution {
                    overlay_id: overlay.id,
                    source: overlay.source.clone(),
                    active: overlay.is_active(),
                    deltas: overlay.transform.sub_field_deltas.clone(),
                    inherited,
                });
            }
        }

        properties.push(PropertyObservation {
            property_id: pid,
            property_name: format!("{}::{}", prop.namespace, prop.name),
            sub_fields,
            overlay_contributions: contributions,
        });
    }

    Some(ObservabilityReport {
        sim_thing_id: target,
        properties,
    })
}

// ── Tree walk ─────────────────────────────────────────────────────────────────

/// Depth-first search returning the path from `root` to `target` (inclusive).
fn find_path<'a>(root: &'a SimThing, target: SimThingId) -> Option<Vec<&'a SimThing>> {
    if root.id == target {
        return Some(vec![root]);
    }
    for child in &root.children {
        if let Some(mut path) = find_path(child, target) {
            path.insert(0, root);
            return Some(path);
        }
    }
    None
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim_runtime_tree::SimRuntimeTree;
    use simthing_core::{
        DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
        PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind, SubFieldRole,
        TransformOp,
    };
    use simthing_gpu::SlotAllocator;

    fn fixture() -> (DimensionRegistry, SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        (reg, pid)
    }

    fn make_overlay(
        pid: SimPropertyId,
        role: SubFieldRole,
        op: TransformOp,
        source: OverlaySource,
        inherited: bool,
    ) -> Overlay {
        let _ = inherited; // only used by tests as a label
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source,
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(role, op)],
            },
            lifecycle: OverlayLifecycle::Permanent,
        }
    }

    fn cohort_row(shadow: &[f32], slot: usize, n_dims: usize) -> &[f32] {
        &shadow[slot * n_dims..slot * n_dims + n_dims]
    }

}
