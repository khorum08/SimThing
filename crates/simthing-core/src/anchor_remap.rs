//! Typed structural anchor-remap section (WRITE-DOOR-BAND-DELTA-0;
//! SLOT-LOGICAL-IDENTITY-0).
//!
//! Slot/column-moving structural ops must carry a complete remap for every
//! Anchored property locus before GPU encode. Stable-slot reparent uses an
//! empty / not-required witness. Remaps are derived from authoritative
//! pre-/post-mutation locus snapshots — never fabricated endpoints.
//!
//! ## The one remap history (DA `5194703997`)
//!
//! This record is the ONLY remap/history authority. Its subject is typed:
//! [`RemapSubject::PropertyLocus`] carries the property id and column
//! endpoints; [`RemapSubject::ObjectRow`] carries no columns by construction
//! and records one whole-row epoch rebind per moved live row — including
//! SimThings with zero anchored property loci. Slot endpoints remain
//! record-level. Absence-as-object-row encodings (`Option<SimPropertyId>`,
//! sentinel property ids, parallel record-level column `Option`s) are
//! structurally inexpressible here.
//!
//! Under [`AnchorRemapOperation::EpochRebind`], demand derives from pre/post
//! BINDING-TABLE snapshots (the id→slot map), never from anchored-locus
//! snapshots: each moved live row yields exactly one `ObjectRow` record, and
//! `PropertyLocus` records exist only for loci whose column binding actually
//! changed. Duplicate `ObjectRow` rows and unchanged-column `PropertyLocus`
//! rows are hard errors.

use crate::column_index::ColumnIndex;
use crate::ids::{SimPropertyId, SimThingId};
use crate::slot_index::SlotIndex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// Structural operation that may churn Anchored store loci or rebind rows.
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
    /// Boundary-barrier physical row rebind (compaction is a motive; rebind
    /// is the event). Demand derives from pre/post binding-table snapshots.
    EpochRebind,
}

/// Typed remap subject (DA `5194703997` constraint 1).
///
/// Column endpoints live INSIDE the locus variant; the object-row variant has
/// no column fields by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemapSubject {
    /// One Anchored property locus (birth / move / retire semantics via the
    /// record-level slot endpoints + these column endpoints).
    PropertyLocus {
        property_id: SimPropertyId,
        /// `None` = birth (first residency).
        from_col: Option<ColumnIndex>,
        /// `None` = retire / tombstone.
        to_col: Option<ColumnIndex>,
    },
    /// The object row itself — one record per moved live row per epoch
    /// rebind, independent of anchored-locus count (including zero).
    ObjectRow,
}

/// Typed key of one remap record — object rows and property loci occupy
/// distinct key spaces; no property-id sentinel exists or is needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemapKey {
    Locus(SimThingId, SimPropertyId),
    Row(SimThingId),
}

/// One remap record: an Anchored store locus move (or birth/retire), or one
/// whole object-row epoch rebind. Slot endpoints are record-level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorLocusRemap {
    pub sim_thing_id: SimThingId,
    pub subject: RemapSubject,
    /// `None` = birth (first residency); locus subjects only.
    pub from_slot: Option<SlotIndex>,
    /// `None` = retire / tombstone; locus subjects only.
    pub to_slot: Option<SlotIndex>,
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
            subject: RemapSubject::PropertyLocus {
                property_id,
                from_col: None,
                to_col: Some(col),
            },
            from_slot: None,
            to_slot: Some(slot),
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
            subject: RemapSubject::PropertyLocus {
                property_id,
                from_col: Some(col),
                to_col: None,
            },
            from_slot: Some(slot),
            to_slot: None,
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
            subject: RemapSubject::PropertyLocus {
                property_id,
                from_col: Some(from_col),
                to_col: Some(to_col),
            },
            from_slot: Some(from_slot),
            to_slot: Some(to_slot),
        }
    }

    /// One whole-row epoch rebind (EpochRebind sections only). No columns by
    /// construction — the row moves, every column binding is preserved.
    pub fn object_row(sim_thing_id: SimThingId, from_slot: SlotIndex, to_slot: SlotIndex) -> Self {
        Self {
            sim_thing_id,
            subject: RemapSubject::ObjectRow,
            from_slot: Some(from_slot),
            to_slot: Some(to_slot),
        }
    }

    pub fn key(&self) -> RemapKey {
        match self.subject {
            RemapSubject::PropertyLocus { property_id, .. } => {
                RemapKey::Locus(self.sim_thing_id, property_id)
            }
            RemapSubject::ObjectRow => RemapKey::Row(self.sim_thing_id),
        }
    }

    /// Locus key when this record is a property-locus subject.
    pub fn locus_key(&self) -> Option<(SimThingId, SimPropertyId)> {
        match self.subject {
            RemapSubject::PropertyLocus { property_id, .. } => {
                Some((self.sim_thing_id, property_id))
            }
            RemapSubject::ObjectRow => None,
        }
    }

    pub fn property_id(&self) -> Option<SimPropertyId> {
        match self.subject {
            RemapSubject::PropertyLocus { property_id, .. } => Some(property_id),
            RemapSubject::ObjectRow => None,
        }
    }

    pub fn from_col(&self) -> Option<ColumnIndex> {
        match self.subject {
            RemapSubject::PropertyLocus { from_col, .. } => from_col,
            RemapSubject::ObjectRow => None,
        }
    }

    pub fn to_col(&self) -> Option<ColumnIndex> {
        match self.subject {
            RemapSubject::PropertyLocus { to_col, .. } => to_col,
            RemapSubject::ObjectRow => None,
        }
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

/// Pre/post snapshot of THE binding table (`id → slot`) — the demand source
/// for [`AnchorRemapOperation::EpochRebind`] sections. Zero-anchor objects
/// are present here even though no anchored-locus snapshot ever names them.
pub type BindingTableSnapshot = BTreeMap<SimThingId, SlotIndex>;

/// Failure to encode a structural GPU flush without a complete Anchored remap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRemapEncodeError {
    pub operation: AnchorRemapOperation,
    pub missing: Vec<RemapKey>,
    pub detail: &'static str,
}

impl std::fmt::Display for AnchorRemapEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "anchor remap encode refused for {:?}: {} ({} offending keys)",
            self.operation,
            self.detail,
            self.missing.len()
        )
    }
}

impl std::error::Error for AnchorRemapEncodeError {}

fn locus_keys(keys: &[(SimThingId, SimPropertyId)]) -> Vec<RemapKey> {
    keys.iter().map(|&(id, prop)| RemapKey::Locus(id, prop)).collect()
}

/// Derive exact remaps from pre-/post-mutation Anchored locus snapshots.
///
/// - birth: absent in `pre`, present in `post`
/// - retire: present in `pre`, absent in `post` (uses the pre slot/col — never a fallback)
/// - move / identity: present in both; identity rows included when `include_stable_identity`
///
/// Rejects duplicate keys while building. Locus-op sections only; epoch
/// rebinds derive through [`derive_epoch_rebind_section`].
pub fn derive_exact_anchor_remaps(
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    operation: AnchorRemapOperation,
    include_stable_identity: bool,
) -> Result<AnchorRemapSection, AnchorRemapEncodeError> {
    if operation == AnchorRemapOperation::EpochRebind {
        return Err(AnchorRemapEncodeError {
            operation,
            missing: Vec::new(),
            detail: "epoch rebind sections derive from binding-table snapshots, not locus maps",
        });
    }
    let mut remaps = Vec::new();
    let mut seen = HashSet::new();
    let mut keys: Vec<_> = pre.keys().copied().chain(post.keys().copied()).collect();
    keys.sort();
    keys.dedup();

    for key in keys {
        if !seen.insert(key) {
            return Err(AnchorRemapEncodeError {
                operation,
                missing: locus_keys(&[key]),
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
/// `ObjectRow` records are lawful only under `EpochRebind`.
pub fn validate_anchor_remap_for_encode(
    section: &AnchorRemapSection,
    required_anchored_loci: &[(SimThingId, SimPropertyId)],
) -> Result<(), AnchorRemapEncodeError> {
    validate_object_rows_only_under_epoch_rebind(section)?;
    if section.remap_not_required {
        if required_anchored_loci.is_empty() {
            return Ok(());
        }
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: locus_keys(required_anchored_loci),
            detail: "remap_not_required set but Anchored loci require remap coverage",
        });
    }
    if required_anchored_loci.is_empty() {
        return validate_no_duplicate_remap_keys(section);
    }
    let mut missing = Vec::new();
    for &key in required_anchored_loci {
        if !section.remaps.iter().any(|r| r.locus_key() == Some(key)) {
            missing.push(RemapKey::Locus(key.0, key.1));
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

/// Derive the authoritative Anchored remap key set from pre-/post-mutation
/// snapshots. Independent of any proposed section — used so encode cannot
/// self-certify by omitting rows.
pub fn expected_anchored_remap_keys(
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    include_stable_identity: bool,
) -> Vec<(SimThingId, SimPropertyId)> {
    let mut keys = Vec::new();
    let mut all: Vec<_> = pre.keys().copied().chain(post.keys().copied()).collect();
    all.sort();
    all.dedup();
    for key in all {
        match (pre.get(&key), post.get(&key)) {
            (None, Some(_)) | (Some(_), None) => keys.push(key),
            (Some(&(from_slot, from_col)), Some(&(to_slot, to_col))) => {
                if from_slot != to_slot || from_col != to_col || include_stable_identity {
                    keys.push(key);
                }
            }
            (None, None) => {}
        }
    }
    keys
}

/// Validate remaps bit-exact against pre-/post-mutation locus snapshots and
/// independently fail-closed on omitted, extra, or duplicate keys. Locus-op
/// sections only; epoch rebinds validate through
/// [`validate_epoch_rebind_section`].
pub fn validate_exact_anchor_remap_endpoints(
    section: &AnchorRemapSection,
    pre: &AnchoredLocusMap,
    post: &AnchoredLocusMap,
    include_stable_identity: bool,
) -> Result<(), AnchorRemapEncodeError> {
    if section.operation == AnchorRemapOperation::EpochRebind {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: Vec::new(),
            detail: "epoch rebind sections validate through validate_epoch_rebind_section",
        });
    }
    validate_object_rows_only_under_epoch_rebind(section)?;
    let expected = expected_anchored_remap_keys(pre, post, include_stable_identity);
    if section.remap_not_required {
        if expected.is_empty() {
            return Ok(());
        }
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: locus_keys(&expected),
            detail: "remap_not_required but Anchored loci require remap coverage",
        });
    }
    validate_no_duplicate_remap_keys(section)?;

    let mut missing = Vec::new();
    for &key in &expected {
        if !section.remaps.iter().any(|r| r.locus_key() == Some(key)) {
            missing.push(RemapKey::Locus(key.0, key.1));
        }
    }
    if !missing.is_empty() {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing,
            detail: "incomplete Anchored remap before GPU encode (omitted pre/post keys)",
        });
    }

    let mut extras = Vec::new();
    for remap in &section.remaps {
        let Some(key) = remap.locus_key() else {
            continue;
        };
        if !expected.contains(&key) {
            extras.push(RemapKey::Locus(key.0, key.1));
        }
    }
    if !extras.is_empty() {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: extras,
            detail: "unexpected Anchored remap keys not demanded by pre/post snapshots",
        });
    }

    for remap in &section.remaps {
        let Some(key) = remap.locus_key() else {
            continue;
        };
        match (
            remap.from_slot,
            remap.from_col(),
            remap.to_slot,
            remap.to_col(),
            pre.get(&key),
            post.get(&key),
        ) {
            (None, None, Some(ts), Some(tc), None, Some(&(ps, pc))) => {
                if ts != ps || tc != pc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "birth remap to-endpoint does not match post locus",
                    });
                }
            }
            (Some(fs), Some(fc), None, None, Some(&(ps, pc)), None) => {
                if fs != ps || fc != pc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "retire remap from-endpoint does not match pre locus",
                    });
                }
            }
            (Some(fs), Some(fc), Some(ts), Some(tc), Some(&(ps, pc)), Some(&(qs, qc))) => {
                if fs != ps || fc != pc || ts != qs || tc != qc {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "move remap endpoints do not match pre/post loci",
                    });
                }
            }
            _ => {
                return Err(AnchorRemapEncodeError {
                    operation: section.operation,
                    missing: vec![RemapKey::Locus(key.0, key.1)],
                    detail: "remap endpoints inconsistent with pre/post locus presence",
                });
            }
        }
    }
    Ok(())
}

/// The moved-live-row demand of one epoch rebind, derived from pre/post
/// binding-table snapshots. A rebind may not create or destroy rows — that is
/// boundary-flush business — so differing key sets are refused.
pub fn expected_epoch_rebind_row_moves(
    pre_rows: &BindingTableSnapshot,
    post_rows: &BindingTableSnapshot,
) -> Result<Vec<(SimThingId, SlotIndex, SlotIndex)>, AnchorRemapEncodeError> {
    let mut mismatched: Vec<RemapKey> = pre_rows
        .keys()
        .filter(|id| !post_rows.contains_key(id))
        .map(|&id| RemapKey::Row(id))
        .collect();
    mismatched.extend(
        post_rows
            .keys()
            .filter(|id| !pre_rows.contains_key(id))
            .map(|&id| RemapKey::Row(id)),
    );
    if !mismatched.is_empty() {
        return Err(AnchorRemapEncodeError {
            operation: AnchorRemapOperation::EpochRebind,
            missing: mismatched,
            detail: "epoch rebind may not create or destroy live rows",
        });
    }
    Ok(pre_rows
        .iter()
        .filter_map(|(&id, &from)| {
            let to = post_rows[&id];
            (from != to).then_some((id, from, to))
        })
        .collect())
}

/// Validate one `EpochRebind` section against pre/post binding-table and
/// anchored-locus snapshots (DA `5194703997` constraint 2):
///
/// - exactly one `ObjectRow` record per moved live row — including rows whose
///   objects have zero anchored loci; omission, duplication, un-demanded rows,
///   and endpoint mismatches are hard errors;
/// - `PropertyLocus` records only for loci whose COLUMN binding changed;
///   an unchanged-column locus row is a hard error (the row move is already
///   carried by the object-row record).
pub fn validate_epoch_rebind_section(
    section: &AnchorRemapSection,
    pre_rows: &BindingTableSnapshot,
    post_rows: &BindingTableSnapshot,
    pre_loci: &AnchoredLocusMap,
    post_loci: &AnchoredLocusMap,
) -> Result<(), AnchorRemapEncodeError> {
    if section.operation != AnchorRemapOperation::EpochRebind {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: Vec::new(),
            detail: "epoch rebind validation demands an EpochRebind section",
        });
    }
    let moves = expected_epoch_rebind_row_moves(pre_rows, post_rows)?;
    if section.remap_not_required {
        if moves.is_empty() {
            return Ok(());
        }
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: moves.iter().map(|&(id, _, _)| RemapKey::Row(id)).collect(),
            detail: "remap_not_required but live rows moved in the binding table",
        });
    }

    // Duplicate ObjectRow RED (typed key space keeps this exact).
    validate_no_duplicate_remap_keys(section)?;

    // Every moved live row — zero-anchor rows included — exactly once, exact
    // endpoints.
    let mut missing = Vec::new();
    for &(id, from, to) in &moves {
        match section
            .remaps
            .iter()
            .find(|r| r.key() == RemapKey::Row(id))
        {
            None => missing.push(RemapKey::Row(id)),
            Some(row) => {
                if row.from_slot != Some(from) || row.to_slot != Some(to) {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Row(id)],
                        detail: "object-row endpoints do not match binding-table snapshots",
                    });
                }
            }
        }
    }
    if !missing.is_empty() {
        return Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing,
            detail: "epoch rebind omits moved live rows (zero-anchor rows count)",
        });
    }

    for remap in &section.remaps {
        match remap.subject {
            RemapSubject::ObjectRow => {
                let id = remap.sim_thing_id;
                if !moves.iter().any(|&(mid, _, _)| mid == id) {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Row(id)],
                        detail: "ObjectRow record not demanded by binding-table snapshots",
                    });
                }
            }
            RemapSubject::PropertyLocus {
                property_id,
                from_col,
                to_col,
            } => {
                let key = (remap.sim_thing_id, property_id);
                let (Some(&(pre_slot, pre_col)), Some(&(post_slot, post_col))) =
                    (pre_loci.get(&key), post_loci.get(&key))
                else {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "epoch rebind may not birth or retire loci",
                    });
                };
                if pre_col == post_col {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "PropertyLocus emitted for unchanged column binding",
                    });
                }
                if from_col != Some(pre_col)
                    || to_col != Some(post_col)
                    || remap.from_slot != Some(pre_slot)
                    || remap.to_slot != Some(post_slot)
                {
                    return Err(AnchorRemapEncodeError {
                        operation: section.operation,
                        missing: vec![RemapKey::Locus(key.0, key.1)],
                        detail: "locus endpoints do not match pre/post loci under epoch rebind",
                    });
                }
            }
        }
    }

    // Changed-column loci must be present (no silent column drift).
    for (&key, &(_, pre_col)) in pre_loci {
        let Some(&(_, post_col)) = post_loci.get(&key) else {
            return Err(AnchorRemapEncodeError {
                operation: section.operation,
                missing: vec![RemapKey::Locus(key.0, key.1)],
                detail: "epoch rebind may not birth or retire loci",
            });
        };
        if pre_col != post_col
            && !section
                .remaps
                .iter()
                .any(|r| r.locus_key() == Some(key))
        {
            return Err(AnchorRemapEncodeError {
                operation: section.operation,
                missing: vec![RemapKey::Locus(key.0, key.1)],
                detail: "epoch rebind omits a changed-column locus",
            });
        }
    }

    Ok(())
}

/// Derive one exact `EpochRebind` section: one `ObjectRow` per moved live row
/// from the binding-table snapshots, plus `PropertyLocus` rows only for loci
/// whose column binding changed.
pub fn derive_epoch_rebind_section(
    pre_rows: &BindingTableSnapshot,
    post_rows: &BindingTableSnapshot,
    pre_loci: &AnchoredLocusMap,
    post_loci: &AnchoredLocusMap,
) -> Result<AnchorRemapSection, AnchorRemapEncodeError> {
    let moves = expected_epoch_rebind_row_moves(pre_rows, post_rows)?;
    let mut remaps: Vec<AnchorLocusRemap> = moves
        .iter()
        .map(|&(id, from, to)| AnchorLocusRemap::object_row(id, from, to))
        .collect();
    for (&key, &(pre_slot, pre_col)) in pre_loci {
        let Some(&(post_slot, post_col)) = post_loci.get(&key) else {
            return Err(AnchorRemapEncodeError {
                operation: AnchorRemapOperation::EpochRebind,
                missing: vec![RemapKey::Locus(key.0, key.1)],
                detail: "epoch rebind may not birth or retire loci",
            });
        };
        if pre_col != post_col {
            remaps.push(AnchorLocusRemap::move_locus(
                key.0, key.1, pre_slot, pre_col, post_slot, post_col,
            ));
        }
    }
    let section = AnchorRemapSection::with_remaps(AnchorRemapOperation::EpochRebind, remaps);
    validate_epoch_rebind_section(&section, pre_rows, post_rows, pre_loci, post_loci)?;
    Ok(section)
}

fn validate_object_rows_only_under_epoch_rebind(
    section: &AnchorRemapSection,
) -> Result<(), AnchorRemapEncodeError> {
    if section.operation == AnchorRemapOperation::EpochRebind {
        return Ok(());
    }
    let rows: Vec<RemapKey> = section
        .remaps
        .iter()
        .filter(|r| r.subject == RemapSubject::ObjectRow)
        .map(|r| r.key())
        .collect();
    if rows.is_empty() {
        Ok(())
    } else {
        Err(AnchorRemapEncodeError {
            operation: section.operation,
            missing: rows,
            detail: "ObjectRow records are lawful only under EpochRebind",
        })
    }
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

    fn col(raw: usize) -> ColumnIndex {
        ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
    }

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
        assert_eq!(err.missing, vec![RemapKey::Locus(id, prop)]);
    }

    #[test]
    fn complete_birth_remap_admits_encode() {
        let id = SimThingId::from_session_raw(1);
        let prop = SimPropertyId(9);
        let section = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::AddChild,
            vec![AnchorLocusRemap::birth(id, prop, SlotIndex::new(3), col(0))],
        );
        assert!(validate_anchor_remap_for_encode(&section, &[(id, prop)]).is_ok());
    }

    #[test]
    fn retire_from_nonzero_slot_is_exact() {
        let id = SimThingId::from_session_raw(2);
        let prop = SimPropertyId(4);
        let mut pre = AnchoredLocusMap::new();
        pre.insert((id, prop), (SlotIndex::new(3), col(1)));
        let post = AnchoredLocusMap::new();
        let section =
            derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::Fusion, false).unwrap();
        assert_eq!(section.remaps.len(), 1);
        assert_eq!(section.remaps[0].from_slot, Some(SlotIndex::new(3)));
        assert!(validate_exact_anchor_remap_endpoints(&section, &pre, &post, false).is_ok());
    }

    #[test]
    fn column_shift_records_pre_to_post_endpoints() {
        let id = SimThingId::from_session_raw(5);
        let prop = SimPropertyId(8);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert((id, prop), (SlotIndex::new(1), col(2)));
        post.insert((id, prop), (SlotIndex::new(1), col(5)));
        let section =
            derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::AddDimension, true)
                .unwrap();
        assert_eq!(section.remaps[0].from_col(), Some(col(2)));
        assert_eq!(section.remaps[0].to_col(), Some(col(5)));
        assert!(validate_exact_anchor_remap_endpoints(&section, &pre, &post, true).is_ok());
    }

    #[test]
    fn wrong_endpoint_and_duplicate_remap_are_rejected() {
        let id = SimThingId::from_session_raw(9);
        let prop = SimPropertyId(1);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert((id, prop), (SlotIndex::new(1), col(0)));
        post.insert((id, prop), (SlotIndex::new(2), col(0)));
        let wrong = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![AnchorLocusRemap::move_locus(
                id,
                prop,
                SlotIndex::new(1),
                col(0),
                SlotIndex::new(9), // wrong
                col(0),
            )],
        );
        assert!(validate_exact_anchor_remap_endpoints(&wrong, &pre, &post, false).is_err());

        let dup = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![
                AnchorLocusRemap::move_locus(id, prop, SlotIndex::new(1), col(0), SlotIndex::new(2), col(0)),
                AnchorLocusRemap::move_locus(id, prop, SlotIndex::new(1), col(0), SlotIndex::new(2), col(0)),
            ],
        );
        assert!(validate_anchor_remap_for_encode(&dup, &[(id, prop)]).is_err());
    }

    #[test]
    fn omitted_retire_row_is_rejected_by_exact_gate() {
        let id = SimThingId::from_session_raw(12);
        let prop = SimPropertyId(6);
        let mut pre = AnchoredLocusMap::new();
        pre.insert((id, prop), (SlotIndex::new(4), col(1)));
        let post = AnchoredLocusMap::new();
        // Empty section self-certifies under the old required-from-section logic.
        let omitted = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fusion, vec![]);
        let err = validate_exact_anchor_remap_endpoints(&omitted, &pre, &post, false).unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Locus(id, prop)]);
        assert!(err.detail.contains("omitted"));
    }

    #[test]
    fn omitted_move_row_is_rejected_by_exact_gate() {
        let id = SimThingId::from_session_raw(13);
        let prop = SimPropertyId(7);
        let mut pre = AnchoredLocusMap::new();
        let mut post = AnchoredLocusMap::new();
        pre.insert((id, prop), (SlotIndex::new(1), col(0)));
        post.insert((id, prop), (SlotIndex::new(2), col(0)));
        let omitted = AnchorRemapSection::with_remaps(AnchorRemapOperation::Fission, vec![]);
        let err = validate_exact_anchor_remap_endpoints(&omitted, &pre, &post, false).unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Locus(id, prop)]);
    }

    #[test]
    fn object_rows_are_lawful_only_under_epoch_rebind() {
        let id = SimThingId::from_session_raw(21);
        let section = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![AnchorLocusRemap::object_row(
                id,
                SlotIndex::new(1),
                SlotIndex::new(2),
            )],
        );
        let err = validate_anchor_remap_for_encode(&section, &[]).unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Row(id)]);
        assert!(err.detail.contains("EpochRebind"));
    }

    #[test]
    fn epoch_rebind_zero_anchor_row_is_demanded_and_exact_once() {
        let zero_anchor = SimThingId::from_session_raw(30);
        let anchored = SimThingId::from_session_raw(31);
        let prop_a = SimPropertyId(2);
        let prop_b = SimPropertyId(3);
        let mut pre_rows = BindingTableSnapshot::new();
        let mut post_rows = BindingTableSnapshot::new();
        pre_rows.insert(zero_anchor, SlotIndex::new(4));
        pre_rows.insert(anchored, SlotIndex::new(7));
        post_rows.insert(zero_anchor, SlotIndex::new(1));
        post_rows.insert(anchored, SlotIndex::new(2));
        // The anchored object carries TWO loci; both move slot with the row,
        // columns unchanged — no locus rows may be emitted for them.
        let mut pre_loci = AnchoredLocusMap::new();
        let mut post_loci = AnchoredLocusMap::new();
        pre_loci.insert((anchored, prop_a), (SlotIndex::new(7), col(0)));
        pre_loci.insert((anchored, prop_b), (SlotIndex::new(7), col(1)));
        post_loci.insert((anchored, prop_a), (SlotIndex::new(2), col(0)));
        post_loci.insert((anchored, prop_b), (SlotIndex::new(2), col(1)));

        let section =
            derive_epoch_rebind_section(&pre_rows, &post_rows, &pre_loci, &post_loci).unwrap();
        // Exactly one ObjectRow per moved live row — the zero-anchor object
        // included, the two-locus object NOT duplicated per locus.
        assert_eq!(section.remaps.len(), 2);
        assert!(section
            .remaps
            .iter()
            .all(|r| r.subject == RemapSubject::ObjectRow));
        assert!(validate_epoch_rebind_section(
            &section, &pre_rows, &post_rows, &pre_loci, &post_loci
        )
        .is_ok());

        // Omitting the zero-anchor row REDs.
        let omitted = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::EpochRebind,
            vec![AnchorLocusRemap::object_row(
                anchored,
                SlotIndex::new(7),
                SlotIndex::new(2),
            )],
        );
        let err = validate_epoch_rebind_section(
            &omitted, &pre_rows, &post_rows, &pre_loci, &post_loci,
        )
        .unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Row(zero_anchor)]);
        assert!(err.detail.contains("zero-anchor"));
    }

    #[test]
    fn epoch_rebind_duplicate_object_row_and_unchanged_column_locus_red() {
        let id = SimThingId::from_session_raw(40);
        let prop = SimPropertyId(5);
        let mut pre_rows = BindingTableSnapshot::new();
        let mut post_rows = BindingTableSnapshot::new();
        pre_rows.insert(id, SlotIndex::new(3));
        post_rows.insert(id, SlotIndex::new(9));
        let mut pre_loci = AnchoredLocusMap::new();
        let mut post_loci = AnchoredLocusMap::new();
        pre_loci.insert((id, prop), (SlotIndex::new(3), col(0)));
        post_loci.insert((id, prop), (SlotIndex::new(9), col(0)));

        // Duplicate ObjectRow for one moved row REDs.
        let dup = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::EpochRebind,
            vec![
                AnchorLocusRemap::object_row(id, SlotIndex::new(3), SlotIndex::new(9)),
                AnchorLocusRemap::object_row(id, SlotIndex::new(3), SlotIndex::new(9)),
            ],
        );
        let err =
            validate_epoch_rebind_section(&dup, &pre_rows, &post_rows, &pre_loci, &post_loci)
                .unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Row(id)]);
        assert!(err.detail.contains("duplicate"));

        // Unchanged-column PropertyLocus over-emission REDs.
        let over = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::EpochRebind,
            vec![
                AnchorLocusRemap::object_row(id, SlotIndex::new(3), SlotIndex::new(9)),
                AnchorLocusRemap::move_locus(
                    id,
                    prop,
                    SlotIndex::new(3),
                    col(0),
                    SlotIndex::new(9),
                    col(0),
                ),
            ],
        );
        let err =
            validate_epoch_rebind_section(&over, &pre_rows, &post_rows, &pre_loci, &post_loci)
                .unwrap_err();
        assert_eq!(err.missing, vec![RemapKey::Locus(id, prop)]);
        assert!(err.detail.contains("unchanged column"));
    }

    #[test]
    fn epoch_rebind_carries_changed_column_locus_and_rejects_row_churn() {
        let id = SimThingId::from_session_raw(50);
        let prop = SimPropertyId(6);
        let mut pre_rows = BindingTableSnapshot::new();
        let mut post_rows = BindingTableSnapshot::new();
        pre_rows.insert(id, SlotIndex::new(2));
        post_rows.insert(id, SlotIndex::new(5));
        let mut pre_loci = AnchoredLocusMap::new();
        let mut post_loci = AnchoredLocusMap::new();
        pre_loci.insert((id, prop), (SlotIndex::new(2), col(1)));
        post_loci.insert((id, prop), (SlotIndex::new(5), col(4)));

        let section =
            derive_epoch_rebind_section(&pre_rows, &post_rows, &pre_loci, &post_loci).unwrap();
        assert_eq!(section.remaps.len(), 2);
        assert!(section
            .remaps
            .iter()
            .any(|r| r.subject == RemapSubject::ObjectRow));
        assert!(section
            .remaps
            .iter()
            .any(|r| r.from_col() == Some(col(1)) && r.to_col() == Some(col(4))));

        // A rebind that creates a row is refused outright.
        post_rows.insert(SimThingId::from_session_raw(51), SlotIndex::new(6));
        assert!(expected_epoch_rebind_row_moves(&pre_rows, &post_rows).is_err());
    }
}
