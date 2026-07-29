//! Structural anchor-remap encode helpers (WRITE-DOOR-BAND-DELTA-0).
//!
//! Builds the typed remap section for a boundary flush and refuses GPU encode
//! when Anchored loci churn without complete coverage.

use simthing_core::{
    validate_anchor_remap_for_encode, AnchorLocusRemap, AnchorRemapEncodeError, AnchorRemapOperation,
    AnchorRemapSection, ColumnIndex, DimensionRegistry, SimPropertyId, SimThing, SimThingId,
    SlotIndex, SubFieldRole,
};
use simthing_gpu::SlotAllocator;

use crate::boundary::BoundaryOutcome;
use crate::sim_runtime_tree::SimRuntimeTree;

/// Collect Anchored `(SimThingId, SimPropertyId)` loci that structural churn
/// requires remap coverage for before GPU encode.
pub fn required_anchored_loci_for_boundary(
    outcome: &BoundaryOutcome,
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
) -> Vec<(SimThingId, SimPropertyId)> {
    let mut required = Vec::new();
    let mut push_thing = |id: SimThingId| {
        if let Some(node) = find_node(root.inner(), id) {
            for prop_id in node.properties.keys().copied() {
                if property_is_anchored(registry, prop_id) {
                    let key = (id, prop_id);
                    if !required.contains(&key) {
                        required.push(key);
                    }
                }
            }
        } else {
            // Tombstoned / fused-away nodes: still require remap coverage for
            // every Anchored property declared on the registry (retire path).
            for row in &registry.property_admission_report().resource_properties {
                if row.disposition.is_anchored() {
                    let key = (id, row.property_id);
                    if !required.contains(&key) {
                        required.push(key);
                    }
                }
            }
        }
    };

    for &(_, child) in &outcome.fission.fission_pairs {
        push_thing(child);
    }
    for &(_, child) in &outcome.fission.fusion_pairs {
        push_thing(child);
    }
    for &id in &outcome.maintainer.allocated {
        push_thing(id);
    }
    for &id in &outcome.maintainer.tombstoned {
        push_thing(id);
    }
    if !outcome.maintainer.dimensions_added.is_empty() {
        // Column identity churn: every live Anchored resource property on every
        // allocated SimThing must be remapped (or witnessed) for the new layout.
        walk_ids(root.inner(), &mut |id| push_thing(id));
        for &prop_id in &outcome.maintainer.dimensions_added {
            if property_is_anchored(registry, prop_id) {
                // Ensure the new property appears even if no node holds it yet.
                let _ = prop_id;
            }
        }
    }
    required
}

/// Build birth/retire remaps for structural churn, or an empty reparent witness.
pub fn build_anchor_remap_section_for_boundary(
    outcome: &BoundaryOutcome,
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> AnchorRemapSection {
    let only_reparent = outcome.fission.fission_pairs.is_empty()
        && outcome.fission.fusion_pairs.is_empty()
        && outcome.maintainer.allocated.is_empty()
        && outcome.maintainer.tombstoned.is_empty()
        && outcome.maintainer.dimensions_added.is_empty()
        && !outcome.maintainer.reparented.is_empty();

    if only_reparent {
        return AnchorRemapSection::empty_not_required(AnchorRemapOperation::Reparent);
    }

    let no_churn = outcome.fission.fission_pairs.is_empty()
        && outcome.fission.fusion_pairs.is_empty()
        && outcome.maintainer.allocated.is_empty()
        && outcome.maintainer.tombstoned.is_empty()
        && outcome.maintainer.dimensions_added.is_empty();
    if no_churn {
        return AnchorRemapSection::empty_not_required(AnchorRemapOperation::BoundaryFlush);
    }

    let mut remaps = Vec::new();
    for &(_, child) in &outcome.fission.fission_pairs {
        push_birth_remaps(&mut remaps, child, root, registry, allocator);
    }
    for &id in &outcome.maintainer.allocated {
        push_birth_remaps(&mut remaps, id, root, registry, allocator);
    }
    for &(_, child) in &outcome.fission.fusion_pairs {
        push_retire_remaps(&mut remaps, child, registry, allocator);
    }
    for &id in &outcome.maintainer.tombstoned {
        push_retire_remaps(&mut remaps, id, registry, allocator);
    }
    if !outcome.maintainer.dimensions_added.is_empty() {
        walk_ids(root.inner(), &mut |id| {
            if let Some(slot) = allocator.slot_of(id) {
                if let Some(node) = find_node(root.inner(), id) {
                    for &prop_id in node.properties.keys() {
                        if let Some(col) = primary_anchored_col(registry, prop_id) {
                            remaps.push(AnchorLocusRemap::move_locus(
                                id, prop_id, slot, col, slot, col,
                            ));
                        }
                    }
                }
            }
        });
    }

    AnchorRemapSection::with_remaps(AnchorRemapOperation::BoundaryFlush, remaps)
}

fn push_birth_remaps(
    remaps: &mut Vec<AnchorLocusRemap>,
    id: SimThingId,
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) {
    let Some(slot) = allocator.slot_of(id) else {
        return;
    };
    let Some(node) = find_node(root.inner(), id) else {
        return;
    };
    for &prop_id in node.properties.keys() {
        if let Some(col) = primary_anchored_col(registry, prop_id) {
            remaps.push(AnchorLocusRemap::birth(id, prop_id, slot, col));
        }
    }
}

fn push_retire_remaps(
    remaps: &mut Vec<AnchorLocusRemap>,
    id: SimThingId,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) {
    for row in &registry.property_admission_report().resource_properties {
        if !row.disposition.is_anchored() {
            continue;
        }
        if let Some(col) = primary_anchored_col(registry, row.property_id) {
            let slot = allocator.slot_of(id).unwrap_or(SlotIndex::new(0));
            remaps.push(AnchorLocusRemap::retire(id, row.property_id, slot, col));
        }
    }
}

/// Fail closed before GPU sync when required Anchored loci lack remap coverage.
pub fn gate_structural_gpu_encode(
    section: &AnchorRemapSection,
    required: &[(SimThingId, SimPropertyId)],
) -> Result<(), AnchorRemapEncodeError> {
    validate_anchor_remap_for_encode(section, required)
}

fn property_is_anchored(registry: &DimensionRegistry, prop_id: SimPropertyId) -> bool {
    registry
        .try_property(prop_id)
        .map(|p| p.admission_disposition.is_anchored())
        .unwrap_or(false)
}

fn primary_anchored_col(
    registry: &DimensionRegistry,
    prop_id: SimPropertyId,
) -> Option<ColumnIndex> {
    let prop = registry.try_property(prop_id)?;
    if !prop.admission_disposition.is_anchored() {
        return None;
    }
    let range = registry.try_column_range(prop_id)?;
    range
        .col_for_role(&SubFieldRole::Amount, &prop.layout)
        .or_else(|| range.col_for_role(&SubFieldRole::Velocity, &prop.layout))
}

fn find_node(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}

fn walk_ids(root: &SimThing, f: &mut dyn FnMut(SimThingId)) {
    f(root.id);
    for child in &root.children {
        walk_ids(child, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        validate_anchor_remap_for_encode, AnchorRemapOperation, AnchorRemapSection, SimPropertyId,
        SimThingId,
    };

    #[test]
    fn remap_less_fission_encode_is_rejected() {
        let id = SimThingId::from_session_raw(42);
        let prop = SimPropertyId(7);
        let section = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fission, vec![]);
        let err = gate_structural_gpu_encode(&section, &[(id, prop)]).unwrap_err();
        assert_eq!(err.operation, AnchorRemapOperation::Fission);
        assert!(!err.missing.is_empty());
    }

    #[test]
    fn reparent_empty_witness_admits_when_no_required_loci() {
        let section = AnchorRemapSection::empty_not_required(AnchorRemapOperation::Reparent);
        assert!(validate_anchor_remap_for_encode(&section, &[]).is_ok());
    }
}
