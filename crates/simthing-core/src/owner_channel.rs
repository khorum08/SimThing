//! OWNER-CHANNEL-INTRINSIC-0 (rung 6.0) — ownership as an intrinsic, inert-by-default
//! dimension of the stem cell.
//!
//! # Why this lives in core
//!
//! Owner channels are how resource contention resolves, collaborative or attritional.
//! Before this module ownership was TWO unrelated notions: [`crate::simthing::ResourceParentEdge`]
//! (per-property, non-spatial, recursive) and a flat owner-ref string stamped individually
//! onto each RF participant from roughly ten call sites — with **no resolution anywhere**.
//! Nothing could answer "who owns this?" without being told, per node, in advance. Every
//! rung that needed ownership therefore re-invented stamping, and the only artifact that
//! authored the stamps became load-bearing.
//!
//! # Inert by default
//!
//! [`SimThing::properties`](crate::simthing::SimThing::properties) is a sparse map holding
//! "only properties that are currently meaningful for this entity", and adding a dimension
//! "never changes this struct". So ownership costs **zero bytes for the inert majority**:
//! a node with no binding stores nothing and still resolves correctly, because
//! **absence MEANS inherit**.
//!
//! # Resolution is total, pure, and never materialized
//!
//! [`resolve_owner`] returns exactly one owner for every valid admitted member. There is no
//! `Option` effective owner: an unbound member resolves to [`unowned`], a real neutral owner
//! that participates in contention as an ordinary party. Foreign ids and malformed explicit
//! bindings are invariant failures, not aliases for neutral ownership or inheritance.
//!
//! The resolved value is **never stamped back onto nodes**. Materializing it would recreate
//! the exact flat-stamping defect this module exists to delete — a 1500-node tree would
//! grow 1500 redundant properties, and every one of them would be a copy that can go stale.
//! Resolution is a pure function of the tree, so **fission is free**: rebinding one property
//! at a subtree root re-parents that entire subtree's ownership with no descendant touched.
//!
//! # Ownership is single; participation is multi-owner
//!
//! Each SimThing resolves to exactly ONE owner. A container admits **as many owners as it
//! has participants** — an ally and an enemy cohort in the same star system is the normal
//! case, not an error. Admission judges **conservation across the container, never ownership
//! uniformity**; requiring uniformity would make adversarial contention structurally
//! impossible. Alliance is a relation *between* owners and is deliberately not modelled here.

use crate::ids::{SimPropertyId, SimThingId};
use crate::property::PropertyValue;
use crate::simthing::{walk_inherited_until, SimThing};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Well-known structural property: this node's explicit owner binding.
///
/// Absence is meaningful and is the common case — it means "inherit from parent".
pub const OWNER_CHANNEL_PROPERTY_ID: SimPropertyId = SimPropertyId(0x0BE1_0001);

/// Reserved identity of the neutral owner.
///
/// Neutral ground is a real party, not an absence. Unowned territory that flips when an
/// owned unit arrives is a rebind from this owner to another — an ordinary transition
/// rather than a `None` -> `Some` special case with its own code path.
pub const UNOWNED_OWNER_REF: &str = "unowned";

/// Metadata owner/channel reference after admission resolution.
///
/// Homed in core because ownership is intrinsic to the stem cell. `simthing-spec`
/// re-exports this type, so the typed channel vocabulary is unchanged for consumers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OwnerRef(String);

impl OwnerRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    /// True when this is the neutral owner.
    pub fn is_unowned(&self) -> bool {
        self.0 == UNOWNED_OWNER_REF
    }

    /// Admit an authored Owner identity.
    ///
    /// The neutral identity is substrate-owned and cannot also name an authored Owner SimThing.
    pub fn try_new_authored(value: impl Into<String>) -> Result<Self, AuthoredOwnerRefError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AuthoredOwnerRefError::Blank);
        }
        if value.trim() == UNOWNED_OWNER_REF {
            return Err(AuthoredOwnerRefError::ReservedNeutralIdentity);
        }
        Ok(Self(value))
    }
}

impl AsRef<str> for OwnerRef {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The neutral owner. Total resolution bottoms out here, never at `None`.
pub fn unowned() -> OwnerRef {
    OwnerRef::new(UNOWNED_OWNER_REF)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AuthoredOwnerRefError {
    #[error("authored Owner identity is blank")]
    Blank,
    #[error("authored Owner identity collides with reserved neutral identity `unowned`")]
    ReservedNeutralIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OwnerResolutionError {
    #[error("target SimThing {target:?} is not a member of the supplied authority tree")]
    TargetNotInTree { target: SimThingId },
    #[error("SimThing {simthing_id:?} has malformed owner binding: {reason}")]
    MalformedBinding {
        simthing_id: SimThingId,
        reason: &'static str,
    },
    #[error("SimThing {simthing_id:?} has a present but blank owner binding")]
    BlankBinding { simthing_id: SimThingId },
}

/// Pure authoring-shape validation for intrinsic owner boundaries.
///
/// An explicit binding is lawful only when it changes the inherited owner. A
/// binding equal to the inherited value is a redundant materialization of the
/// resolved answer—the flat stamp-every-node shape this channel replaces.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OwnerBoundaryValidationError {
    #[error(transparent)]
    Resolution(#[from] OwnerResolutionError),
    #[error(
        "SimThing {simthing_id:?} redundantly stamps inherited owner `{owner}`; absence must inherit"
    )]
    RedundantBinding {
        simthing_id: SimThingId,
        owner: String,
    },
}

/// Bind `owner` onto `node` as its explicit owner.
///
/// This is the ONLY write in this module, and it is a *binding*, not a stamp of a resolved
/// value: it declares intent at one node. Ownership fission is exactly one call to this at
/// a subtree root.
pub fn bind_owner(node: &mut SimThing, owner: &OwnerRef) {
    node.add_property(
        OWNER_CHANNEL_PROPERTY_ID,
        encode_owner_property(owner.as_str()),
    );
}

/// Remove an explicit binding, returning the node to inherited ownership.
pub fn unbind_owner(node: &mut SimThing) {
    node.remove_property(&OWNER_CHANNEL_PROPERTY_ID);
}

/// This node's EXPLICIT binding, if it declares one.
///
/// `None` means "inherits" — it does not mean "unowned". Callers wanting the effective
/// answer must use [`resolve_owner`]; this accessor exists for admission and authoring,
/// not for RF consumers.
pub fn declared_owner(node: &SimThing) -> Result<Option<OwnerRef>, OwnerResolutionError> {
    let Some(value) = node.property(OWNER_CHANNEL_PROPERTY_ID) else {
        return Ok(None);
    };
    let text =
        decode_owner_property(value).map_err(|reason| OwnerResolutionError::MalformedBinding {
            simthing_id: node.id,
            reason,
        })?;
    if text.trim().is_empty() {
        return Err(OwnerResolutionError::BlankBinding {
            simthing_id: node.id,
        });
    }
    Ok(Some(OwnerRef::new(text)))
}

/// Resolve the effective owner of `target` within `root`.
///
/// Walks the explicit binding, then the nearest bound ancestor, and bottoms out at
/// [`unowned`]. Valid admitted members are total and never return `Option`; a foreign target
/// or invalid present binding fails closed. Never materializes the answer.
pub fn resolve_owner(
    root: &SimThing,
    target: SimThingId,
) -> Result<OwnerRef, OwnerResolutionError> {
    let seed = unowned();
    walk_inherited_until(
        root,
        &seed,
        &mut |node, inherited| Ok(declared_owner(node)?.unwrap_or_else(|| inherited.clone())),
        &mut |node, effective| Ok((node.id == target).then(|| effective.clone())),
    )?
    .ok_or(OwnerResolutionError::TargetNotInTree { target })
}

/// Resolve the effective owner of every node in `root`, parent-first.
///
/// One traversal instead of one per node. Emits `(id, owner)` pairs in deterministic
/// pre-order so downstream bucketing has a canonical order without sorting.
pub fn resolve_owners_in_order(
    root: &SimThing,
) -> Result<Vec<(SimThingId, OwnerRef)>, OwnerResolutionError> {
    let mut out = Vec::new();
    let seed = unowned();
    let _: Option<()> = walk_inherited_until(
        root,
        &seed,
        &mut |node, inherited| Ok(declared_owner(node)?.unwrap_or_else(|| inherited.clone())),
        &mut |node, effective| {
            out.push((node.id, effective.clone()));
            Ok(None)
        },
    )?;
    Ok(out)
}

/// Validate that explicit owner properties encode crossings only.
///
/// This is a read-only query over authored tree data. It never normalizes,
/// stamps, or stores resolved ownership.
pub fn validate_owner_binding_boundaries(
    root: &SimThing,
) -> Result<(), OwnerBoundaryValidationError> {
    let seed = unowned();
    let _: Option<()> = walk_inherited_until(
        root,
        &seed,
        &mut |node, inherited| {
            let declared = declared_owner(node)?;
            if declared.as_ref().is_some_and(|owner| owner == inherited) {
                let owner = declared.expect("checked Some");
                return Err(OwnerBoundaryValidationError::RedundantBinding {
                    simthing_id: node.id,
                    owner: owner.into_inner(),
                });
            }
            Ok(declared.unwrap_or_else(|| inherited.clone()))
        },
        &mut |_node, _effective| Ok(None),
    )?;
    Ok(())
}

/// True when `node`'s effective owner differs from `parent_owner` — i.e. the edge from its
/// parent is an ownership CROSSING.
///
/// Crossings are the only flows that need per-node recording: identity flows are
/// reconstructible from the node's own aggregate, so recording them at every level would
/// make retained state O(nodes x owners x resources) instead of O(crossings).
pub fn is_ownership_crossing(
    node: &SimThing,
    parent_owner: &OwnerRef,
) -> Result<bool, OwnerResolutionError> {
    Ok(declared_owner(node)?.is_some_and(|own| &own != parent_owner))
}

/// Length-prefixed lane packing, matching the established core convention for opaque
/// string-valued structural properties (see `fission_clone_source`).
fn encode_owner_property(owner: &str) -> PropertyValue {
    let mut lanes = vec![owner.len() as f32];
    for chunk in owner.as_bytes().chunks(4) {
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        lanes.push(f32::from_bits(u32::from_le_bytes(bytes)));
    }
    PropertyValue::from_raw_lanes(lanes)
}

fn decode_owner_property(value: &PropertyValue) -> Result<String, &'static str> {
    let lanes = value.raw_lanes_for_serialization();
    if lanes.is_empty() {
        return Err("missing length lane");
    }
    let encoded_len = lanes[0];
    if !encoded_len.is_finite()
        || encoded_len < 0.0
        || encoded_len.fract() != 0.0
        || encoded_len > 16_777_216.0
    {
        return Err("length lane is not an exact non-negative integer");
    }
    let len = encoded_len as usize;
    let expected_lane_count = 1usize
        .checked_add(len.saturating_add(3) / 4)
        .ok_or("encoded length overflows lane count")?;
    if lanes.len() != expected_lane_count {
        return Err("encoded length does not match payload lane count");
    }
    let mut bytes = Vec::with_capacity(len);
    for lane in lanes.iter().skip(1) {
        bytes.extend_from_slice(&lane.to_bits().to_le_bytes());
    }
    if bytes.iter().skip(len).any(|byte| *byte != 0) {
        return Err("payload padding is non-zero");
    }
    bytes.truncate(len);
    String::from_utf8(bytes).map_err(|_| "payload is not valid UTF-8")
}

/// Session-local interned owner identity for persistent RF layout keys.
///
/// Not semantic identity, not seam currency, and not serializable. Independent
/// sessions may assign different ids to the same [`OwnerRef`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerLayoutId(u32);

impl OwnerLayoutId {
    pub fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OwnerInternError {
    #[error("owner `{owner}` is not interned in this session")]
    UnknownOwner { owner: String },
    #[error("owner `{owner}` is already interned")]
    AlreadyInterned { owner: String },
}

/// Session-local first-seen intern of [`OwnerRef`] for layout metadata only.
///
/// Rebuild from tree-walk/first-seen order, or [`Self::rebind`] a flag-switch
/// so the interned id survives an `alpha -> zulu` string change.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnerInterner {
    ids: std::collections::HashMap<OwnerRef, OwnerLayoutId>,
    refs: Vec<OwnerRef>,
}

impl OwnerInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, owner: &OwnerRef) -> OwnerLayoutId {
        if let Some(id) = self.ids.get(owner) {
            return *id;
        }
        let id = OwnerLayoutId(self.refs.len() as u32);
        self.refs.push(owner.clone());
        self.ids.insert(owner.clone(), id);
        id
    }

    pub fn id_of(&self, owner: &OwnerRef) -> Option<OwnerLayoutId> {
        self.ids.get(owner).copied()
    }

    pub fn owner_of(&self, id: OwnerLayoutId) -> Option<&OwnerRef> {
        self.refs.get(id.0 as usize)
    }

    pub fn len(&self) -> usize {
        self.refs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    /// Keep the same layout id while the canonical [`OwnerRef`] string changes.
    pub fn rebind(
        &mut self,
        from: &OwnerRef,
        to: OwnerRef,
    ) -> Result<OwnerLayoutId, OwnerInternError> {
        if from == &to {
            return self
                .id_of(from)
                .ok_or_else(|| OwnerInternError::UnknownOwner {
                    owner: from.as_str().to_string(),
                });
        }
        if self.ids.contains_key(&to) {
            return Err(OwnerInternError::AlreadyInterned {
                owner: to.as_str().to_string(),
            });
        }
        let id = self
            .ids
            .remove(from)
            .ok_or_else(|| OwnerInternError::UnknownOwner {
                owner: from.as_str().to_string(),
            })?;
        self.refs[id.0 as usize] = to.clone();
        self.ids.insert(to, id);
        Ok(id)
    }
}
