//! Structural anchor-remap encode helpers (WRITE-DOOR-BAND-DELTA-0).
//!
//! Captures pre-mutation Anchored loci, derives exact post-mutation remaps, and
//! refuses GPU encode when coverage/endpoints are incomplete or fabricated.

use simthing_core::{
    derive_exact_anchor_remaps, expected_anchored_remap_keys, validate_anchor_remap_for_encode,
    validate_exact_anchor_remap_endpoints, AnchorRemapEncodeError, AnchorRemapOperation,
    AnchorRemapSection, AnchoredLocusMap, ColumnIndex, DimensionRegistry, SimPropertyId, SimThing,
    SimThingId, SubFieldRole,
};
use simthing_gpu::SlotAllocator;

use crate::boundary::BoundaryOutcome;

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

/// Required Anchored keys derived from authoritative pre/post snapshots — never
/// from the proposed section (which could omit rows and self-certify).
pub fn required_anchored_loci_for_boundary(
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    outcome: &BoundaryOutcome,
    slot_capacity_grew: bool,
) -> Vec<(SimThingId, SimPropertyId)> {
    let include_stable_identity =
        !outcome.maintainer.dimensions_added.is_empty() || slot_capacity_grew;
    expected_anchored_remap_keys(pre, post, include_stable_identity)
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
    validate_exact_anchor_remap_endpoints(&section, pre, post, include_stable_identity)?;
    Ok(section)
}

fn classify_operation(outcome: &BoundaryOutcome, slot_capacity_grew: bool) -> AnchorRemapOperation {
    let fission = !outcome.fission.fission_pairs.is_empty();
    let fusion =
        !outcome.fission.fusion_pairs.is_empty() || !outcome.maintainer.tombstoned.is_empty();
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

/// Gate with exact pre/post completeness + endpoint verification (production).
pub fn gate_structural_gpu_encode_exact(
    section: &AnchorRemapSection,
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    include_stable_identity: bool,
) -> Result<(), AnchorRemapEncodeError> {
    let required = expected_anchored_remap_keys(pre, post, include_stable_identity);
    validate_anchor_remap_for_encode(section, &required)?;
    validate_exact_anchor_remap_endpoints(section, pre, post, include_stable_identity)
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

    #[test]
    fn omitted_retire_and_move_fail_production_exact_gate() {
        let id = SimThingId::from_session_raw(20);
        let prop = SimPropertyId(2);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert(
            (id, prop),
            (
                SlotIndex::new(3),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
            ),
        );
        // Retire omitted.
        let omitted_retire = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fusion, vec![]);
        assert!(gate_structural_gpu_encode_exact(
            &omitted_retire,
            &pre,
            &AnchoredLocusMap::new(),
            false
        )
        .is_err());
        // Move omitted.
        post.insert(
            (id, prop),
            (
                SlotIndex::new(5),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
            ),
        );
        let omitted_move = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fission, vec![]);
        assert!(gate_structural_gpu_encode_exact(&omitted_move, &pre, &post, false).is_err());
    }
}
