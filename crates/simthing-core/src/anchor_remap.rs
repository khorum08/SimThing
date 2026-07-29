//! Typed structural anchor-remap section (WRITE-DOOR-BAND-DELTA-0).
//!
//! Slot/column-moving structural ops must carry a complete remap for every
//! Anchored property locus before GPU encode. Stable-slot reparent uses an
//! empty / not-required witness.

use crate::column_index::ColumnIndex;
use crate::ids::{SimPropertyId, SimThingId};
use crate::slot_index::SlotIndex;
use serde::{Deserialize, Serialize};

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
        return Ok(());
    }
    let mut missing = Vec::new();
    for &key in required_anchored_loci {
        if !section.remaps.iter().any(|r| r.key() == key) {
            missing.push(key);
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing,
            detail: "incomplete Anchored remap before GPU encode",
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
}
