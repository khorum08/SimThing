//! Funded structural products (DA, relay 5737649159).
//!
//! A source-authored, DETACHED structural template that is born through the
//! existing ActionBand crossing -> `StructuralAuthorization` ->
//! `BoundaryRequest::AddChild` door when an authored funding locus crosses
//! successive whole units. The declaration is lowered once, at session build
//! (tick zero), into the same frozen ActionBand session product the 2.1
//! construction lifecycle graduated on. It is never instantiated into the N0
//! tree and never executes through any other structural path.

use crate::spec::overlay::OverlaySpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};
use simthing_core::{SimThingKind, SubFieldRole};
use std::num::NonZeroU32;

/// One funded structural product line.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralProductSpec {
    pub id: String,
    /// The EXISTING funded locus whose whole-unit crossings fund births
    /// (typically a recipe's target quantity). No receipt property is minted.
    pub funding: StructuralFundingSpec,
    /// Bounded cardinality: product `k` (0-based) is born when the funding
    /// locus first rises past `k + 0.5`. Lowers to the existing bounded
    /// threshold / template / active-instance sequence; never an unbounded
    /// spawner inferred from a scalar counter.
    pub count: NonZeroU32,
    /// Existing structural host every product is born under. Never an owner
    /// seat: ownership is declared on the template, not by spatial parentage.
    pub parent_entity: String,
    pub template: StructuralTemplateNodeSpec,
    /// Clause token of the declaration (spanned admission provenance).
    #[serde(skip)]
    pub source_span_token: Option<usize>,
}

/// The funded locus: an existing property / role at an authored host.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralFundingSpec {
    pub property: PropertyKey,
    pub role: SubFieldRole,
    pub host_entity: String,
}

/// One node of the detached template: ordinary SimThing semantics only.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralTemplateNodeSpec {
    pub kind: SimThingKind,
    /// Explicit owner binding; absent inherits from the structural parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_values: Vec<StructuralTemplatePropertySpec>,
    /// Overlays owned by the born node itself (origin and affects = the node).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlays: Vec<OverlaySpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<StructuralTemplateNodeSpec>,
}

/// Authored scalar cells of one existing property on a template node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralTemplatePropertySpec {
    pub property: PropertyKey,
    pub values: Vec<(SubFieldRole, f32)>,
}
