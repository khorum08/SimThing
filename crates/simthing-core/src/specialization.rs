//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1, P0/P3): the richer-than-kind
//! specialization protocol.
//!
//! A specialization is a **profile**: a data-declared bundle of root-contract
//! usages a SimThing either structurally CONFORMS to (derived — observation,
//! zero authoring burden) or explicitly DECLARES (validated at admission with
//! hard errors). Profiles are data, never a trait hierarchy: a SimThing at
//! rest remains a row, and no runtime path consults profiles (admission-time
//! only). Kinds remain serialization/authority markers; a profile is a kind
//! marker PLUS structural facts — richer than the kind alone, additive-only.

use crate::simthing::{SimThing, SimThingKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Stable profile identifier (data; spec-authorable growth arrives in 3.2).
pub type SpecializationProfileId = &'static str;

pub const PROFILE_SESSION_ROOT: SpecializationProfileId = "session-root";
pub const PROFILE_OWNER_SEAT: SpecializationProfileId = "owner-seat";
pub const PROFILE_SPATIAL: SpecializationProfileId = "spatial";

/// One structural requirement a profile imposes. Closed data enum — adding a
/// requirement kind is a DA-gated protocol change, adding a PROFILE composed
/// of existing requirements is ordinary data growth (the EML library law's
/// shape applied to specialization).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecializationRequirement {
    /// The SimThing carries this kind as its authority marker.
    KindMarker(String),
    /// The SimThing's tree parent carries this kind marker.
    ParentKindMarker(String),
    /// The SimThing is a tree root or the sole child of a `Scenario` root.
    SessionRootPosture,
}

/// A specialization profile: id + requirements, pure data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationProfile {
    pub id: String,
    pub description: String,
    pub requirements: Vec<SpecializationRequirement>,
}

/// The three seed profiles (3.1). All are satisfied by the existing corpus as
/// authored today — the compatibility falsifier depends on it. 3.2 enriches
/// these into full first citizens (placement conformance, hosted families).
pub fn seed_profiles() -> Vec<SpecializationProfile> {
    vec![
        SpecializationProfile {
            id: PROFILE_SESSION_ROOT.to_string(),
            description: "GameSession contract: the running session root".to_string(),
            requirements: vec![
                SpecializationRequirement::KindMarker("GameSession".to_string()),
                SpecializationRequirement::SessionRootPosture,
            ],
        },
        SpecializationProfile {
            id: PROFILE_OWNER_SEAT.to_string(),
            description: "Owner contract: operator seat, sibling child of the session root"
                .to_string(),
            requirements: vec![
                SpecializationRequirement::KindMarker("Owner".to_string()),
                SpecializationRequirement::ParentKindMarker("GameSession".to_string()),
            ],
        },
        SpecializationProfile {
            id: PROFILE_SPATIAL.to_string(),
            description: "Location contract: gridcell / spatial-arena participant (\u{a7}7: there is no non-spatial Location)"
                .to_string(),
            requirements: vec![SpecializationRequirement::KindMarker(
                "Location".to_string(),
            )],
        },
    ]
}

fn kind_marker(kind: &SimThingKind) -> String {
    match kind {
        SimThingKind::Custom(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

/// Per-SimThing conformance row in the inspectable report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationRow {
    pub simthing: u32,
    pub kind_marker: String,
    /// Profiles this SimThing structurally conforms to (derived observation).
    pub derived: Vec<String>,
    /// Profiles the SimThing explicitly declared (all validated at admission).
    pub declared: Vec<String>,
}

/// Inspectable whole-tree report (the derivation-report pattern from 1.1).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationReport {
    pub rows: Vec<SpecializationRow>,
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
}

/// Admission hard errors for declared-profile validation. Spanned where the
/// authoring surface provides spans (clausething authoring lands in 3.2; the
/// programmatic path reports ids + the precise unmet requirement).
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SpecializationError {
    #[error("SimThing {simthing} declares unknown specialization profile `{profile}`")]
    UnknownProfile { simthing: u32, profile: String },

    #[error(
        "SimThing {simthing} (kind `{kind_marker}`) declares profile `{profile}` but does not satisfy requirement {requirement:?}"
    )]
    RequirementUnmet {
        simthing: u32,
        kind_marker: String,
        profile: String,
        requirement: SpecializationRequirement,
    },
}

fn requirement_met(
    req: &SpecializationRequirement,
    node: &SimThing,
    parent_kind: Option<&SimThingKind>,
) -> bool {
    match req {
        SpecializationRequirement::KindMarker(marker) => kind_marker(&node.kind) == *marker,
        SpecializationRequirement::ParentKindMarker(marker) => {
            parent_kind.map(kind_marker).as_deref() == Some(marker.as_str())
        }
        SpecializationRequirement::SessionRootPosture => matches!(
            parent_kind,
            None | Some(SimThingKind::Scenario)
        ),
    }
}

/// Derive structural conformance and validate declared profiles for the whole
/// tree. Pure observation + validation: mutates nothing, gates nothing beyond
/// the declared-profile hard errors. Admission-time only.
pub fn derive_specializations(
    root: &SimThing,
    profiles: &[SpecializationProfile],
) -> Result<SpecializationReport, SpecializationError> {
    let by_id: BTreeMap<&str, &SpecializationProfile> =
        profiles.iter().map(|p| (p.id.as_str(), p)).collect();
    let mut report = SpecializationReport::default();
    walk(root, None, &by_id, &mut report)?;
    Ok(report)
}

fn walk(
    node: &SimThing,
    parent_kind: Option<&SimThingKind>,
    profiles: &BTreeMap<&str, &SpecializationProfile>,
    report: &mut SpecializationReport,
) -> Result<(), SpecializationError> {
    let marker = kind_marker(&node.kind);
    let derived: Vec<String> = profiles
        .values()
        .filter(|p| {
            p.requirements
                .iter()
                .all(|req| requirement_met(req, node, parent_kind))
        })
        .map(|p| p.id.clone())
        .collect();

    for declared in &node.declared_specializations {
        let profile = profiles.get(declared.as_str()).ok_or_else(|| {
            SpecializationError::UnknownProfile {
                simthing: node.id.raw(),
                profile: declared.clone(),
            }
        })?;
        if let Some(unmet) = profile
            .requirements
            .iter()
            .find(|req| !requirement_met(req, node, parent_kind))
        {
            return Err(SpecializationError::RequirementUnmet {
                simthing: node.id.raw(),
                kind_marker: marker.clone(),
                profile: declared.clone(),
                requirement: unmet.clone(),
            });
        }
    }

    report.rows.push(SpecializationRow {
        simthing: node.id.raw(),
        kind_marker: marker,
        derived,
        declared: node.declared_specializations.clone(),
    });
    for child in &node.children {
        walk(child, Some(&node.kind), profiles, report)?;
    }
    Ok(())
}
