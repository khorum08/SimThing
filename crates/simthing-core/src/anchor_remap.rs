//! Typed structural anchor-remap section (WRITE-DOOR-BAND-DELTA-0).
//!
//! Slot/column-moving structural ops must carry a complete remap for every
//! Anchored property locus before GPU encode. Stable-slot reparent uses an
//! empty / not-required witness. Remaps are derived from authoritative
//! pre-/post-mutation locus snapshots — never fabricated endpoints.

use crate::column_index::ColumnIndex;
use crate::ids::{SimPropertyId, SimThingId};
use crate::slot_index::SlotIndex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// Structural operation that may churn Anchored store loci.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorRemapOperation {
    Fission,
    Fusion,
    Remove,
    AddChild,
    AddDimension,
    SlotCapacityGrow,
    /// Relation-only; slots do not move.
    Reparent,
    /// Consolidated boundary section covering mixed churn in one flush.
    BoundaryFlush,
}

/// One Anchored store locus move (or birth/retire).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorLocusRemap {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    /// `None` = birth (first residency).
    pub from_slot: Option<SlotIndex>,
    pub from_col: Option<ColumnIndex>,
    /// `None` = retire / tombstone.
    pub to_slot: Option<SlotIndex>,
    pub to_col: Option<ColumnIndex>,
}

impl AnchorLocusRemap {
    pub fn birth(
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        slot: SlotIndex,
        col: ColumnIndex,
    ) -> Self {
        Self {
            sim_thing_id,
            property_id,
            from_slot: None,
            from_col: None,
            to_slot: Some(slot),
            to_col: Some(col),
        }
    }

    pub fn retire(
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        slot: SlotIndex,
        col: ColumnIndex,
    ) -> Self {
        Self {
            sim_thing_id,
            property_id,
            from_slot: Some(slot),
            from_col: Some(col),
            to_slot: None,
            to_col: None,
        }
    }

    pub fn move_locus(
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        from_slot: SlotIndex,
        from_col: ColumnIndex,
        to_slot: SlotIndex,
        to_col: ColumnIndex,
    ) -> Self {
        Self {
            sim_thing_id,
            property_id,
            from_slot: Some(from_slot),
            from_col: Some(from_col),
            to_slot: Some(to_slot),
            to_col: Some(to_col),
        }
    }

    pub fn key(&self) -> (SimThingId, SimPropertyId) {
        (self.sim_thing_id, self.property_id)
    }
}

/// Remap section attached to a structural encode / boundary flush.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapSection {
    pub operation: AnchorRemapOperation,
    pub remaps: Vec<AnchorLocusRemap>,
    /// When true, the op proves no Anchored locus churn (stable-slot reparent).
    pub remap_not_required: bool,
}

impl Default for AnchorRemapSection {
    fn default() -> Self {
        Self::empty_not_required(AnchorRemapOperation::BoundaryFlush)
    }
}

impl AnchorRemapSection {
    pub fn empty_not_required(operation: AnchorRemapOperation) -> Self {
        Self {
            operation,
            remaps: Vec::new(),
            remap_not_required: true,
        }
    }

    pub fn with_remaps(operation: AnchorRemapOperation, remaps: Vec<AnchorLocusRemap>) -> Self {
        Self {
            operation,
            remaps,
            remap_not_required: false,
        }
    }
}

/// Authoritative Anchored locus table: `(SimThingId, SimPropertyId) → (slot, col)`.
pub type AnchoredLocusMap = BTreeMap<(SimThingId, SimPropertyId), (SlotIndex, ColumnIndex)>;

/// Failure to encode a structural GPU flush without a complete Anchored remap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRemapEncodeError {
    pub operation: AnchorRemapOperation,
    pub missing: Vec<(SimThingId, SimPropertyId)>,
    pub detail: &'static str,
}

impl std::fmt::Display for AnchorRemapEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "anchor remap encode refused for {:?}: {} (missing {} Anchored loci)",
            self.operation,
            self.detail,
            self.missing.len()
        )
    }
}

impl std::error::Error for AnchorRemapEncodeError {}

/// Derive exact remaps from pre-/post-mutation Anchored locus snapshots.
///
/// - birth: absent in `pre`, present in `post`
/// - retire: present in `pre`, absent in `post` (uses the pre slot/col — never a fallback)
/// - move / identity: present in both; identity rows included when `include_stable_identity`
///
/// Rejects duplicate keys while building.
pub fn derive_exact_anchor_remaps(
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    operation: AnchorRemapOperation,
    include_stable_identity: bool,
) -> Result<AnchorRemapSection, AnchorRemapEncodeError> {
    let mut remaps = Vec::new();
    let mut seen = HashSet::new();
    let mut keys: Vec<_> = pre.keys().copied().chain(post.keys().copied()).collect();
    keys.sort();
    keys.dedup();

    for key in keys {
        if !seen.insert(key) {
            return Err(AnchorRemapEncodeError {
                operation,
                missing: vec![key],
                detail: "duplicate Anchored locus key while deriving remaps",
            });
        }
        let (id, prop) = key;
        match (pre.get(&key), post.get(&key)) {
            (None, Some(&(to_slot, to_col))) => {
                remaps.push(AnchorLocusRemap::birth(id, prop, to_slot, to_col));
            }
            (Some(&(from_slot, from_col)), None) => {
                remaps.push(AnchorLocusRemap::retire(id, prop, from_slot, from_col));
            }
            (Some(&(from_slot, from_col)), Some(&(to_slot, to_col))) => {
                if from_slot != to_slot || from_col != to_col || include_stable_identity {
                    remaps.push(AnchorLocusRemap::move_locus(
                        id, prop, from_slot, from_col, to_slot, to_col,
                    ));
                }
            }
            (None, None) => {}
        }
    }

    Ok(AnchorRemapSection::with_remaps(operation, remaps))
}

/// Fail closed before GPU encode when required Anchored loci lack remap rows.
pub fn validate_anchor_remap_for_encode(
    section: &AnchorRemapSection,
    required_anchored_loci: &[(SimThingId, SimPropertyId)],
) -> Result<(), AnchorRemapEncodeError> {
    if section.remap_not_required {
        if required_anchored_loci.is_empty() {
            return Ok(());
        }
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: required_anchored_loci.to_vec(),
            detail: "remap_not_required set but Anchored loci require remap coverage",
        });
    }
    if required_anchored_loci.is_empty() {
        return validate_no_duplicate_remap_keys(section);
    }
    let mut missing = Vec::new();
    for &key in required_anchored_loci {
        if !section.remaps.iter().any(|r| r.key() == key) {
            missing.push(key);
        }
    }
    if !missing.is_empty() {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing,
            detail: "incomplete Anchored remap before GPU encode",
        });
    }
    validate_no_duplicate_remap_keys(section)
}

/// Validate remaps bit-exact against pre-/post-mutation locus snapshots.
pub fn validate_exact_anchor_remap_endpoints(
    section: &AnchorRemapSection,
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
) -> Result<(), AnchorRemapEncodeError> {
    if section.remap_not_required {
        if pre == post {
            return Ok(());
        }
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: Vec::new(),
            detail: "remap_not_required but pre/post Anchored loci differ",
        });
    }
    validate_no_duplicate_remap_keys(section)?;
    for remap in &section.remaps {
        let key = remap.key();
        match (
            remap.from_slot,
            remap.from_col,
            remap.to_slot,
            remap.to_col,
            pre.get(&key),
            post.get(&key),
        ) {
            (None, None, Some(ts), Some(tc), None, Some(&(ps, pc))) => {
                if ts != ps || tc != pc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![key],
                        detail: "birth remap to-endpoint does not match post locus",
                    });
                }
            }
            (Some(fs), Some(fc), None, None, Some(&(ps, pc)), None) => {
                if fs != ps || fc != pc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![key],
                        detail: "retire remap from-endpoint does not match pre locus",
                    });
                }
            }
            (Some(fs), Some(fc), Some(ts), Some(tc), Some(&(ps, pc)), Some(&(qs, qc))) => {
                if fs != ps || fc != pc || ts != qs || tc != qc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![key],
                        detail: "move remap endpoints do not match pre/post loci",
                    });
                }
            }
            _ => {
                return Err(AnchorRemapEncodeError {
                    operation: section.operation,
                    missing: vec![key],
                    detail: "remap endpoints inconsistent with pre/post locus presence",
                });
            }
        }
    }
    Ok(())
}

fn validate_no_duplicate_remap_keys(
    section: &AnchorRemapSection,
) -> Result<(), AnchorRemapEncodeError> {
    let mut seen = HashSet::new();
    let mut dupes = Vec::new();
    for remap in &section.remaps {
        if !seen.insert(remap.key()) {
            dupes.push(remap.key());
        }
    }
    if dupes.is_empty() {
        Ok(())
    } else {
        Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: dupes,
            detail: "duplicate Anchored remap keys",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column_index::ColumnIndex;
    use crate::ids::{SimPropertyId, SimThingId};
    use crate::slot_index::SlotIndex;

    #[test]
    fn remap_not_required_allows_empty_when_no_required_loci() {
        let section = AnchorRemapSection::empty_not_required(AnchorRemapOperation::Reparent);
        assert!(validate_anchor_remap_for_encode(&section, &[]).is_ok());
    }

    #[test]
    fn incomplete_remap_refuses_encode() {
        let id = SimThingId::from_session_raw(1);
        let prop = SimPropertyId(9);
        let section = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fission, vec![]);
        let err = validate_anchor_remap_for_encode(&section, &[(id, prop)]).unwrap_err();
        assert_eq!(err.missing, vec![(id, prop)]);
    }

    #[test]
    fn complete_birth_remap_admits_encode() {
        let id = SimThingId::from_session_raw(1);
        let prop = SimPropertyId(9);
        let section = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::AddChild,
            vec![AnchorLocusRemap::birth(
                id,
                prop,
                SlotIndex::new(3),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            )],
        );
        assert!(validate_anchor_remap_for_encode(&section, &[(id, prop)]).is_ok());
    }

    #[test]
    fn retire_from_nonzero_slot_is_exact() {
        let id = SimThingId::from_session_raw(2);
        let prop = SimPropertyId(4);
        let mut pre = AnchoredLocusMap::new();
        pre.insert(
            (id, prop),
            (
                SlotIndex::new(3),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
            ),
        );
        let post = AnchoredLocusMap::new();
        let section =
            derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::Fusion, false).unwrap();
        assert_eq!(section.remaps.len(), 1);
        assert_eq!(section.remaps[0].from_slot, Some(SlotIndex::new(3)));
        assert!(validate_exact_anchor_remap_endpoints(&section, &pre, &post).is_ok());
    }

    #[test]
    fn column_shift_records_pre_to_post_endpoints() {
        let id = SimThingId::from_session_raw(5);
        let prop = SimPropertyId(8);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert(
            (id, prop),
            (
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
            ),
        );
        post.insert(
            (id, prop),
            (
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(5),
            ),
        );
        let section = derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::AddDimension, true)
            .unwrap();
        assert_eq!(
            section.remaps[0].from_col,
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(2))
        );
        assert_eq!(
            section.remaps[0].to_col,
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(5))
        );
        assert!(validate_exact_anchor_remap_endpoints(&section, &pre, &post).is_ok());
    }

    #[test]
    fn wrong_endpoint_and_duplicate_remap_are_rejected() {
        let id = SimThingId::from_session_raw(9);
        let prop = SimPropertyId(1);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert(
            (id, prop),
            (
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
        );
        post.insert(
            (id, prop),
            (
                SlotIndex::new(2),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
        );
        let wrong = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![AnchorLocusRemap::move_locus(
                id,
                prop,
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                SlotIndex::new(9), // wrong
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            )],
        );
        assert!(validate_exact_anchor_remap_endpoints(&wrong, &pre, &post).is_err());

        let dup = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![
                AnchorLocusRemap::move_locus(
                    id,
                    prop,
                    SlotIndex::new(1),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    SlotIndex::new(2),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                ),
                AnchorLocusRemap::move_locus(
                    id,
                    prop,
                    SlotIndex::new(1),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    SlotIndex::new(2),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                ),
            ],
        );
        assert!(validate_anchor_remap_for_encode(&dup, &[(id, prop)]).is_err());
    }
}
