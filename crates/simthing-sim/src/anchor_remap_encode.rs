//! Structural anchor-remap encode helpers (WRITE-DOOR-BAND-DELTA-0).
//!
//! Captures pre-mutation Anchored loci, derives exact post-mutation remaps, and
//! refuses GPU encode when coverage/endpoints are incomplete or fabricated.

use simthing_core::{
    derive_exact_anchor_remaps, validate_anchor_remap_for_encode,
    validate_exact_anchor_remap_endpoints, AnchorRemapEncodeError, AnchorRemapOperation,
    AnchorRemapSection, AnchoredLocusMap, ColumnIndex, DimensionRegistry, SimPropertyId, SimThing,
    SimThingId, SubFieldRole,
};
use simthing_gpu::SlotAllocator;

use crate::boundary::BoundaryOutcome;
use crate::sim_runtime_tree::SimRuntimeTree;

/// Snapshot every live Anchored `(SimThingId, SimPropertyId)` locus.
pub fn snapshot_anchored_loci(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> AnchoredLocusMap {
    let mut map = AnchoredLocusMap::new();
    walk_nodes(root, &mut |node| {
        let Some(slot) = allocator.slot_of(node.id) else {
            return;
        };
        for &prop_id in node.properties.keys() {
            if let Some(col) = primary_anchored_col(registry, prop_id) {
                map.insert((node.id, prop_id), (slot, col));
            }
        }
    });
    map
}

/// Required Anchored keys for encode coverage = keys present in the derived section
/// plus any still-live Anchored keys demanded by structural churn witnesses.
pub fn required_anchored_loci_for_boundary(
    section: &AnchorRemapSection,
    outcome: &BoundaryOutcome,
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
) -> Vec<(SimThingId, SimPropertyId)> {
    if section.remap_not_required {
        return Vec::new();
    }
    let mut required: Vec<_> = section.remaps.iter().map(|r| r.key()).collect();
    // Retire paths for fused/tombstoned nodes that no longer sit in the tree must
    // still be covered (they appear in section remaps). Also require live births.
    for &(_, child) in &outcome.fission.fission_pairs {
        push_live_anchored(&mut required, child, root, registry);
    }
    for &id in &outcome.maintainer.allocated {
        push_live_anchored(&mut required, id, root, registry);
    }
    required.sort();
    required.dedup();
    required
}

fn push_live_anchored(
    required: &mut Vec<(SimThingId, SimPropertyId)>,
    id: SimThingId,
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
) {
    if let Some(node) = find_node(root.inner(), id) {
        for prop_id in node.properties.keys().copied() {
            if property_is_anchored(registry, prop_id) {
                let key = (id, prop_id);
                if !required.contains(&key) {
                    required.push(key);
                }
            }
        }
    }
}

/// Build exact remaps from pre-/post-mutation snapshots (production boundary path).
pub fn build_exact_anchor_remap_section(
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    outcome: &BoundaryOutcome,
    slot_capacity_grew: bool,
) -> Result<AnchorRemapSection, AnchorRemapEncodeError> {
    let only_reparent = outcome.fission.fission_pairs.is_empty()
        && outcome.fission.fusion_pairs.is_empty()
        && outcome.maintainer.allocated.is_empty()
        && outcome.maintainer.tombstoned.is_empty()
        && outcome.maintainer.dimensions_added.is_empty()
        && !outcome.maintainer.reparented.is_empty()
        && !slot_capacity_grew
        && pre == post;

    if only_reparent {
        return Ok(AnchorRemapSection::empty_not_required(
            AnchorRemapOperation::Reparent,
        ));
    }

    let no_churn = outcome.fission.fission_pairs.is_empty()
        && outcome.fission.fusion_pairs.is_empty()
        && outcome.maintainer.allocated.is_empty()
        && outcome.maintainer.tombstoned.is_empty()
        && outcome.maintainer.dimensions_added.is_empty()
        && outcome.maintainer.reparented.is_empty()
        && !slot_capacity_grew
        && pre == post;
    if no_churn {
        return Ok(AnchorRemapSection::empty_not_required(
            AnchorRemapOperation::BoundaryFlush,
        ));
    }

    let include_stable_identity =
        !outcome.maintainer.dimensions_added.is_empty() || slot_capacity_grew;
    let operation = classify_operation(outcome, slot_capacity_grew);
    let section = derive_exact_anchor_remaps(pre, post, operation, include_stable_identity)?;
    validate_exact_anchor_remap_endpoints(&section, pre, post)?;
    Ok(section)
}

fn classify_operation(outcome: &BoundaryOutcome, slot_capacity_grew: bool) -> AnchorRemapOperation {
    let fission = !outcome.fission.fission_pairs.is_empty();
    let fusion = !outcome.fission.fusion_pairs.is_empty()
        || !outcome.maintainer.tombstoned.is_empty();
    let add_child = !outcome.maintainer.allocated.is_empty();
    let add_dim = !outcome.maintainer.dimensions_added.is_empty();
    let kinds = [fission, fusion, add_child, add_dim, slot_capacity_grew]
        .iter()
        .filter(|&&b| b)
        .count();
    if kinds > 1 {
        return AnchorRemapOperation::BoundaryFlush;
    }
    if fission {
        return AnchorRemapOperation::Fission;
    }
    if fusion {
        return AnchorRemapOperation::Fusion;
    }
    if add_child {
        return AnchorRemapOperation::AddChild;
    }
    if add_dim {
        return AnchorRemapOperation::AddDimension;
    }
    if slot_capacity_grew {
        return AnchorRemapOperation::SlotCapacityGrow;
    }
    AnchorRemapOperation::BoundaryFlush
}

/// Fail closed before GPU sync when required Anchored loci lack remap coverage.
pub fn gate_structural_gpu_encode(
    section: &AnchorRemapSection,
    required: &[(SimThingId, SimPropertyId)],
) -> Result<(), AnchorRemapEncodeError> {
    validate_anchor_remap_for_encode(section, required)
}

/// Gate with exact pre/post endpoint verification (production boundary).
pub fn gate_structural_gpu_encode_exact(
    section: &AnchorRemapSection,
    required: &[(SimThingId, SimPropertyId)],
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
) -> Result<(), AnchorRemapEncodeError> {
    validate_anchor_remap_for_encode(section, required)?;
    validate_exact_anchor_remap_endpoints(section, pre, post)
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

fn walk_nodes(root: &SimThing, f: &mut dyn FnMut(&SimThing)) {
    f(root);
    for child in &root.children {
        walk_nodes(child, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        validate_anchor_remap_for_encode, AnchorRemapOperation, AnchorRemapSection, ColumnIndex,
        SimPropertyId, SimThingId, SlotIndex,
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

    #[test]
    fn exact_retire_never_fabricates_slot_zero() {
        let id = SimThingId::from_session_raw(11);
        let prop = SimPropertyId(3);
        let mut pre = AnchoredLocusMap::new();
        pre.insert(
            (id, prop),
            (
                SlotIndex::new(4),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
            ),
        );
        let post = AnchoredLocusMap::new();
        let outcome = BoundaryOutcome::default();
        let section = build_exact_anchor_remap_section(&pre, &post, &outcome, false).unwrap();
        assert_eq!(section.remaps[0].from_slot, Some(SlotIndex::new(4)));
        assert_ne!(section.remaps[0].from_slot, Some(SlotIndex::new(0)));
    }
}
