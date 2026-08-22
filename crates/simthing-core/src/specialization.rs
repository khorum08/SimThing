//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1, P0/P3): the richer-than-kind
//! specialization protocol.
//!
//! A specialization is a **profile**: a data-declared bundle of root-contract
//! usages a SimThing either structurally CONFORMS to (derived — observation
//! against authoritative admission artifacts) or explicitly DECLARES
//! (validated at admission with spanned hard errors). Profiles are data,
//! never a trait hierarchy; no runtime path consults them (admission-time
//! only). A profile is typed kind identity PLUS structural facts — richer
//! than the kind alone, additive-only. Custom kinds can never impersonate
//! built-in authority kinds.

use crate::ids::SimThingId;
use crate::owner_channel::{resolve_owners_in_order, OwnerRef, OwnerResolutionError};
use crate::property::SimThingKindTag;
use crate::simthing::{SimThing, SimThingKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub type SpecializationProfileId = &'static str;

pub const PROFILE_SESSION_ROOT: SpecializationProfileId = "session-root";
pub const PROFILE_OWNER_SEAT: SpecializationProfileId = "owner-seat";
pub const PROFILE_SPATIAL: SpecializationProfileId = "spatial";

/// Typed kind identity for requirements. `Custom("Location")` is NOT
/// [`SimThingKindTag::Location`]: a custom tag never satisfies a built-in
/// identity requirement (remand `5098201367` collision fence).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindIdentity {
    BuiltIn(SimThingKindTag),
    Custom(String),
}

pub fn kind_identity(kind: &SimThingKind) -> KindIdentity {
    match kind {
        SimThingKind::Custom(name) => KindIdentity::Custom(name.clone()),
        SimThingKind::Scenario => KindIdentity::BuiltIn(SimThingKindTag::Scenario),
        SimThingKind::GameSession => KindIdentity::BuiltIn(SimThingKindTag::GameSession),
        SimThingKind::World => KindIdentity::BuiltIn(SimThingKindTag::World),
        SimThingKind::Owner => KindIdentity::BuiltIn(SimThingKindTag::Owner),
        #[allow(deprecated)]
        SimThingKind::Faction => KindIdentity::BuiltIn(SimThingKindTag::Faction),
        #[allow(deprecated)]
        SimThingKind::StarSystem => KindIdentity::BuiltIn(SimThingKindTag::StarSystem),
        SimThingKind::Location => KindIdentity::BuiltIn(SimThingKindTag::Location),
        SimThingKind::Cohort => KindIdentity::BuiltIn(SimThingKindTag::Cohort),
        SimThingKind::Fleet => KindIdentity::BuiltIn(SimThingKindTag::Fleet),
        #[allow(deprecated)]
        SimThingKind::Station => KindIdentity::BuiltIn(SimThingKindTag::Station),
    }
}

/// One structural requirement. Closed data enum — new requirement KINDS are
/// DA-gated protocol changes; new PROFILES composed from existing kinds are
/// ordinary data growth (the library growth law's shape).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecializationRequirement {
    /// Typed kind identity (built-in vs custom is never conflated).
    Kind(KindIdentity),
    /// The tree parent carries this typed kind identity.
    ParentKind(KindIdentity),
    /// The actual GameSession-root posture: the node is the absolute tree
    /// root, or the SOLE DIRECT child of a `Scenario` tree root (child count
    /// exactly one, and that child is the built-in `GameSession`).
    SoleSessionRootPosture,
    /// The node has an authoritative structural grid placement (coordinate
    /// posture + membership of the spatial field lattice; §7: unoccupied
    /// cells carrying ambient field are still spatial, so placement — not
    /// arena enrollment — is the spatial-field participation fact).
    StructurallyPlaced,
    /// The node hosts the ADMITTED policy/weight locus. The concrete artifact
    /// is caller-observed: for the canonical corpus it is the owner-silo
    /// metadata locus (`simthing_spec::owner_has_silo_metadata`, the same fact
    /// the owner-silo flow admission consumes). A random production/stockpile
    /// accumulator on an Owner does NOT make it a seat.
    HostsAdmittedPolicyWeightLocus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationProfile {
    pub id: String,
    pub description: String,
    pub requirements: Vec<SpecializationRequirement>,
}

/// The three seed profiles (3.1; 3.2 enriches into full first citizens).
pub fn seed_profiles() -> Vec<SpecializationProfile> {
    vec![
        SpecializationProfile {
            id: PROFILE_SESSION_ROOT.to_string(),
            description: "GameSession contract: the sole running session root".to_string(),
            requirements: vec![
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::GameSession)),
                SpecializationRequirement::SoleSessionRootPosture,
            ],
        },
        SpecializationProfile {
            id: PROFILE_OWNER_SEAT.to_string(),
            description:
                "Owner contract: session-root child hosting the admitted policy/weight (owner-silo) locus"
                    .to_string(),
            requirements: vec![
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::Owner)),
                SpecializationRequirement::ParentKind(KindIdentity::BuiltIn(
                    SimThingKindTag::GameSession,
                )),
                SpecializationRequirement::HostsAdmittedPolicyWeightLocus,
            ],
        },
        SpecializationProfile {
            id: PROFILE_SPATIAL.to_string(),
            description:
                "Location contract: structurally placed gridcell of the spatial field lattice (\u{a7}7)"
                    .to_string(),
            requirements: vec![
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::Location)),
                SpecializationRequirement::StructurallyPlaced,
            ],
        },
    ]
}

/// Authoritative admission-artifact observations the caller assembles for
/// derivation. Facts, not behavior: which SimThings hold structural grid
/// placements, and which host populated resource-bearing properties. Callers
/// with partial artifacts (e.g. a driver install without spec-side grid
/// metadata) pass what they have; requirements over absent artifacts simply
/// do not derive — honesty over vacuous conformance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpecializationObservations {
    pub structurally_placed: BTreeSet<u32>,
    /// SimThings hosting the admitted policy/weight locus (canonical corpus:
    /// owner-silo metadata; see the requirement doc).
    pub policy_weight_hosts: BTreeSet<u32>,
}

/// One explicitly declared profile with its authored source token (clause
/// scalar token index when authored; `None` for programmatic declarations).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredSpecialization {
    pub profile: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_token: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationRow {
    pub simthing: u32,
    pub kind: KindIdentity,
    pub derived: Vec<String>,
    pub declared: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationReport {
    pub rows: Vec<SpecializationRow>,
}

/// Kind-free owner × specialization query row.
///
/// Ownership comes only from the intrinsic owner channel and specialization
/// comes only from the admitted report. Deliberately omits `SimThingKind`: a
/// consumer cannot branch on kind to answer this question through the query.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerSpecializationRow {
    pub simthing: SimThingId,
    pub owner: OwnerRef,
    pub derived: Vec<String>,
    pub declared: Vec<String>,
}

/// Join intrinsic ownership to an already-derived specialization report.
///
/// This is a pure, read-only query. A report normally contains one row per
/// tree node; a missing row is represented honestly by empty profile lists.
pub fn query_owner_specializations(
    root: &SimThing,
    report: &SpecializationReport,
) -> Result<Vec<OwnerSpecializationRow>, OwnerResolutionError> {
    resolve_owners_in_order(root).map(|owners| {
        owners
            .into_iter()
            .map(|(simthing, owner)| {
                let row = report.row_for(simthing.raw());
                OwnerSpecializationRow {
                    simthing,
                    owner,
                    derived: row.map(|row| row.derived.clone()).unwrap_or_default(),
                    declared: row.map(|row| row.declared.clone()).unwrap_or_default(),
                }
            })
            .collect()
    })
}

impl SpecializationReport {
    pub fn row_for(&self, simthing: u32) -> Option<&SpecializationRow> {
        self.rows.iter().find(|r| r.simthing == simthing)
    }
    pub fn derived_ids(&self, simthing: u32) -> Vec<&str> {
        self.row_for(simthing)
            .map(|r| r.derived.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// FIRST-CITIZEN-SPECIALISTS-0: per-profile conformance totals from an
    /// installed `SpecSessionState.specialization` report (Consumer Law board
    /// / orientation surface — never a hand-edited mirror).
    pub fn citizen_counts(&self) -> SpecializationCitizenCounts {
        let mut counts = SpecializationCitizenCounts::default();
        for row in &self.rows {
            for profile in &row.derived {
                match profile.as_str() {
                    x if x == PROFILE_SPATIAL => counts.spatial += 1,
                    x if x == PROFILE_OWNER_SEAT => counts.owner_seat += 1,
                    x if x == PROFILE_SESSION_ROOT => counts.session_root += 1,
                    _ => {}
                }
            }
        }
        counts
    }
}

/// Per-seed-profile conformance counts (board/orientation generator source).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationCitizenCounts {
    pub spatial: usize,
    pub owner_seat: usize,
    pub session_root: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SpecializationError {
    #[error(
        "SimThing {simthing} declares unknown specialization profile `{profile}` (span_token={span_token:?})"
    )]
    UnknownProfile {
        simthing: u32,
        profile: String,
        span_token: Option<usize>,
    },

    #[error(
        "SimThing {simthing} ({kind:?}) declares profile `{profile}` but does not satisfy requirement {requirement:?} (span_token={span_token:?})"
    )]
    RequirementUnmet {
        simthing: u32,
        kind: KindIdentity,
        profile: String,
        requirement: SpecializationRequirement,
        span_token: Option<usize>,
    },
}

struct NodeContext<'a> {
    parent_kind: Option<&'a SimThingKind>,
    /// True when the node is the tree root, or the sole GameSession child of
    /// a Scenario that is itself the tree root.
    sole_session_root: bool,
}

fn requirement_met(
    req: &SpecializationRequirement,
    node: &SimThing,
    ctx: &NodeContext<'_>,
    obs: &SpecializationObservations,
) -> bool {
    match req {
        SpecializationRequirement::Kind(identity) => kind_identity(&node.kind) == *identity,
        SpecializationRequirement::ParentKind(identity) => ctx
            .parent_kind
            .map(kind_identity)
            .is_some_and(|k| k == *identity),
        SpecializationRequirement::SoleSessionRootPosture => ctx.sole_session_root,
        SpecializationRequirement::StructurallyPlaced => {
            obs.structurally_placed.contains(&node.id.raw())
        }
        SpecializationRequirement::HostsAdmittedPolicyWeightLocus => {
            obs.policy_weight_hosts.contains(&node.id.raw())
        }
    }
}

/// Derive structural conformance and validate declared profiles for the
/// whole tree against caller-assembled admission-artifact observations.
pub fn derive_specializations(
    root: &SimThing,
    profiles: &[SpecializationProfile],
    observations: &SpecializationObservations,
) -> Result<SpecializationReport, SpecializationError> {
    let by_id: BTreeMap<&str, &SpecializationProfile> =
        profiles.iter().map(|p| (p.id.as_str(), p)).collect();
    let mut report = SpecializationReport::default();
    let root_ctx = NodeContext {
        parent_kind: None,
        sole_session_root: true,
    };
    walk(root, &root_ctx, root, &by_id, observations, &mut report)?;
    Ok(report)
}

fn walk(
    node: &SimThing,
    ctx: &NodeContext<'_>,
    tree_root: &SimThing,
    profiles: &BTreeMap<&str, &SpecializationProfile>,
    obs: &SpecializationObservations,
    report: &mut SpecializationReport,
) -> Result<(), SpecializationError> {
    let identity = kind_identity(&node.kind);
    let derived: Vec<String> = profiles
        .values()
        .filter(|p| {
            p.requirements
                .iter()
                .all(|req| requirement_met(req, node, ctx, obs))
        })
        .map(|p| p.id.clone())
        .collect();

    for declared in &node.declared_specializations {
        let profile = profiles.get(declared.profile.as_str()).ok_or_else(|| {
            SpecializationError::UnknownProfile {
                simthing: node.id.raw(),
                profile: declared.profile.clone(),
                span_token: declared.span_token,
            }
        })?;
        if let Some(unmet) = profile
            .requirements
            .iter()
            .find(|req| !requirement_met(req, node, ctx, obs))
        {
            return Err(SpecializationError::RequirementUnmet {
                simthing: node.id.raw(),
                kind: identity.clone(),
                profile: declared.profile.clone(),
                requirement: unmet.clone(),
                span_token: declared.span_token,
            });
        }
    }

    report.rows.push(SpecializationRow {
        simthing: node.id.raw(),
        kind: identity,
        derived,
        declared: node
            .declared_specializations
            .iter()
            .map(|d| d.profile.clone())
            .collect(),
    });

    // Strict sole/direct-child invariant: the Scenario tree root must have
    // EXACTLY ONE direct child, and that child must be the built-in
    // GameSession, for that child to carry session-root posture.
    let root_is_scenario = matches!(tree_root.kind, SimThingKind::Scenario);
    let sole_direct_session_child = root_is_scenario
        && tree_root.children.len() == 1
        && tree_root.children[0].kind == SimThingKind::GameSession;
    for child in &node.children {
        let child_ctx = NodeContext {
            parent_kind: Some(&node.kind),
            sole_session_root: std::ptr::eq(node, tree_root) && sole_direct_session_child,
        };
        walk(child, &child_ctx, tree_root, profiles, obs, report)?;
    }
    Ok(())
}
