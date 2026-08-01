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
//! [`resolve_owner`] always returns exactly one owner. There is no `Option`: an unbound
//! tree resolves to [`unowned`], a real neutral owner that participates in contention as
//! an ordinary party. That deletes a partial function from the RF path instead of adding
//! one, and makes an ownership flip an ordinary rebind rather than a nothing-to-something
//! transition.
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
use crate::simthing::SimThing;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Bind `owner` onto `node` as its explicit owner.
///
/// This is the ONLY write in this module, and it is a *binding*, not a stamp of a resolved
/// value: it declares intent at one node. Ownership fission is exactly one call to this at
/// a subtree root.
pub fn bind_owner(node: &mut SimThing, owner: &OwnerRef) {
    node.add_property(OWNER_CHANNEL_PROPERTY_ID, encode_owner_property(owner.as_str()));
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
pub fn declared_owner(node: &SimThing) -> Option<OwnerRef> {
    let text = decode_owner_property(node.property(OWNER_CHANNEL_PROPERTY_ID)?)?;
    if text.trim().is_empty() {
        return None;
    }
    Some(OwnerRef::new(text))
}

/// Resolve the effective owner of `target` within `root`. **Total**: always exactly one.
///
/// Walks the explicit binding, then the nearest bound ancestor, and bottoms out at
/// [`unowned`]. Never materializes the answer.
///
/// Returns [`unowned`] when `target` is not present in `root` — a node outside the tree has
/// no ancestry to inherit from, and reporting neutral keeps the function total rather than
/// introducing an error path that every RF caller would have to handle.
pub fn resolve_owner(root: &SimThing, target: SimThingId) -> OwnerRef {
    fn walk(node: &SimThing, target: SimThingId, inherited: &OwnerRef) -> Option<OwnerRef> {
        let effective = declared_owner(node).unwrap_or_else(|| inherited.clone());
        if node.id == target {
            return Some(effective);
        }
        for child in &node.children {
            if let Some(found) = walk(child, target, &effective) {
                return Some(found);
            }
        }
        None
    }
    walk(root, target, &unowned()).unwrap_or_else(unowned)
}

/// Resolve the effective owner of every node in `root`, parent-first.
///
/// One traversal instead of one per node. Emits `(id, owner)` pairs in deterministic
/// pre-order so downstream bucketing has a canonical order without sorting.
pub fn resolve_owners_in_order(root: &SimThing) -> Vec<(SimThingId, OwnerRef)> {
    fn walk(node: &SimThing, inherited: &OwnerRef, out: &mut Vec<(SimThingId, OwnerRef)>) {
        let effective = declared_owner(node).unwrap_or_else(|| inherited.clone());
        out.push((node.id, effective.clone()));
        for child in &node.children {
            walk(child, &effective, out);
        }
    }
    let mut out = Vec::new();
    walk(root, &unowned(), &mut out);
    out
}

/// True when `node`'s effective owner differs from `parent_owner` — i.e. the edge from its
/// parent is an ownership CROSSING.
///
/// Crossings are the only flows that need per-node recording: identity flows are
/// reconstructible from the node's own aggregate, so recording them at every level would
/// make retained state O(nodes x owners x resources) instead of O(crossings).
pub fn is_ownership_crossing(node: &SimThing, parent_owner: &OwnerRef) -> bool {
    declared_owner(node).is_some_and(|own| &own != parent_owner)
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

fn decode_owner_property(value: &PropertyValue) -> Option<String> {
    let lanes = value.raw_lanes_for_serialization();
    if lanes.is_empty() {
        return None;
    }
    let len = lanes[0] as usize;
    let mut bytes = Vec::with_capacity(len);
    for lane in lanes.iter().skip(1) {
        bytes.extend_from_slice(&lane.to_bits().to_le_bytes());
    }
    bytes.truncate(len);
    String::from_utf8(bytes).ok()
}
