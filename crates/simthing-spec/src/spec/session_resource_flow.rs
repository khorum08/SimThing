//! SESSION-RESOURCE-FLOW-SILOS-0 — generic Owner silo reduce-up / disburse-down admission oracle.
//!
//! Non-mutating deterministic evaluation over canonical Scenario authority. Ownership is expressed
//! through owner-reference properties on spatial participants, not spatial parenting.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{SimThing, SimThingKind};

use super::scenario::{
    game_session_galaxy_map, game_session_owners, is_galaxy_map_entity, owner_entity_id,
    owner_flow_owner_ref, owner_has_silo_metadata, owner_silo_capacity, owner_silo_current,
    property_u32, SimThingScenarioSpec, OWNER_FLOW_DEFICIT_PROPERTY_ID,
    OWNER_FLOW_SURPLUS_PROPERTY_ID,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerSiloAdmissionClassification {
    Admitted,
    PartiallyAdmitted,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerSiloDeferralKind {
    ResourceFlowExecutionDeferred,
    CapacityClampDeferred,
    MultiResourceVectorDeferred,
    CrossOwnerTransferDeferred,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerSiloAdmissionErrorKind {
    MissingOwnerReference,
    UnknownOwnerReference,
    OwnerMissingSilo,
    InvalidSiloAmount,
    InvalidSurplusAmount,
    InvalidDeficitAmount,
    DuplicateParticipantId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnerSiloDeferral {
    pub kind: OwnerSiloDeferralKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnerSiloAdmissionError {
    pub kind: OwnerSiloAdmissionErrorKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnerSiloAdmissionReport {
    pub owner_count: u32,
    pub silo_owner_count: u32,
    pub participant_count: u32,
    pub surplus_participant_count: u32,
    pub deficit_participant_count: u32,
    pub rejected_participant_count: u32,
    pub reducible_surplus_total: f32,
    pub resolvable_deficit_total: f32,
    pub unresolved_deficit_total: f32,
    pub classification: OwnerSiloAdmissionClassification,
    pub deferrals: Vec<OwnerSiloDeferral>,
    pub errors: Vec<OwnerSiloAdmissionError>,
}

#[derive(Debug, Clone, PartialEq)]
struct FlowParticipant {
    simthing_id_raw: u32,
    owner_id: String,
    surplus: u32,
    deficit: u32,
    path: String,
}

pub fn evaluate_owner_silo_flow(spec: &SimThingScenarioSpec) -> OwnerSiloAdmissionReport {
    let mut report = OwnerSiloAdmissionReport {
        classification: OwnerSiloAdmissionClassification::Admitted,
        ..Default::default()
    };

    if spec.root.kind != SimThingKind::Scenario {
        push_deferral(
            &mut report,
            OwnerSiloDeferralKind::ResourceFlowExecutionDeferred,
            None,
            None,
            "owner silo flow requires canonical Scenario root",
        );
        report.classification = OwnerSiloAdmissionClassification::PartiallyAdmitted;
        return report;
    }

    let owners = match game_session_owners(spec) {
        Ok(owners) => owners,
        Err(_) => {
            report.classification = OwnerSiloAdmissionClassification::Rejected;
            return report;
        }
    };
    report.owner_count = owners.len() as u32;

    let mut owner_ids = BTreeMap::new();
    for owner in owners {
        if let Some(id) = owner_entity_id(owner) {
            if owner_has_silo_metadata(owner) {
                report.silo_owner_count += 1;
            }
            owner_ids.insert(id, owner);
        }
    }

    let participants = match collect_flow_participants(spec, &mut report) {
        Ok(participants) => participants,
        Err(()) => {
            report.classification = OwnerSiloAdmissionClassification::Rejected;
            return report;
        }
    };

    if participants.is_empty() && report.silo_owner_count > 0 {
        push_deferral(
            &mut report,
            OwnerSiloDeferralKind::ResourceFlowExecutionDeferred,
            None,
            None,
            "owner silo metadata present but no participant flow properties yet",
        );
    }

    if !participants.is_empty() {
        push_deferral(
            &mut report,
            OwnerSiloDeferralKind::CapacityClampDeferred,
            None,
            None,
            "capacity clamp semantics use per-owner silo capacity when present",
        );
        push_deferral(
            &mut report,
            OwnerSiloDeferralKind::MultiResourceVectorDeferred,
            None,
            None,
            "single generic flow vector per participant in first slice",
        );
        push_deferral(
            &mut report,
            OwnerSiloDeferralKind::CrossOwnerTransferDeferred,
            None,
            None,
            "cross-owner silo transfer remains deferred",
        );
    }

    let mut surplus_by_owner: BTreeMap<String, u32> = BTreeMap::new();
    let mut deficit_by_owner: BTreeMap<String, u32> = BTreeMap::new();
    for participant in &participants {
        report.participant_count += 1;
        if participant.surplus > 0 {
            report.surplus_participant_count += 1;
            *surplus_by_owner
                .entry(participant.owner_id.clone())
                .or_default() += participant.surplus;
        }
        if participant.deficit > 0 {
            report.deficit_participant_count += 1;
            *deficit_by_owner
                .entry(participant.owner_id.clone())
                .or_default() += participant.deficit;
        }
    }

    for (owner_id, owner) in &owner_ids {
        let surplus = *surplus_by_owner.get(owner_id).unwrap_or(&0);
        let deficit = *deficit_by_owner.get(owner_id).unwrap_or(&0);
        if surplus == 0 && deficit == 0 {
            continue;
        }
        if !owner_has_silo_metadata(owner) {
            push_error(
                &mut report,
                OwnerSiloAdmissionErrorKind::OwnerMissingSilo,
                Some(format!("owner/{owner_id}")),
                Some(owner.id.raw()),
                format!("owner `{owner_id}` is referenced by flow participants but has no silo metadata"),
            );
            continue;
        }

        let current = owner_silo_current(owner).unwrap_or(0);
        let capacity = owner_silo_capacity(owner).unwrap_or(u32::MAX);
        let absorbed_surplus = surplus.min(capacity.saturating_sub(current));
        let silo_after_surplus = current.saturating_add(absorbed_surplus);
        let resolved = deficit.min(silo_after_surplus);
        let unresolved = deficit.saturating_sub(resolved);

        report.reducible_surplus_total += absorbed_surplus as f32;
        report.resolvable_deficit_total += resolved as f32;
        report.unresolved_deficit_total += unresolved as f32;
    }

    finalize_owner_silo_classification(&mut report);
    report
}

impl Default for OwnerSiloAdmissionReport {
    fn default() -> Self {
        Self {
            owner_count: 0,
            silo_owner_count: 0,
            participant_count: 0,
            surplus_participant_count: 0,
            deficit_participant_count: 0,
            rejected_participant_count: 0,
            reducible_surplus_total: 0.0,
            resolvable_deficit_total: 0.0,
            unresolved_deficit_total: 0.0,
            classification: OwnerSiloAdmissionClassification::Admitted,
            deferrals: Vec::new(),
            errors: Vec::new(),
        }
    }
}

fn collect_flow_participants(
    spec: &SimThingScenarioSpec,
    report: &mut OwnerSiloAdmissionReport,
) -> Result<Vec<FlowParticipant>, ()> {
    let galaxy_map = match game_session_galaxy_map(spec) {
        Ok(map) => map,
        Err(_) => return Ok(Vec::new()),
    };

    let mut participants = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let owner_ids: BTreeSet<String> = game_session_owners(spec)
        .ok()
        .map(|owners| {
            owners
                .into_iter()
                .filter_map(owner_entity_id)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();

    for gridcell in galaxy_map
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Location && !is_galaxy_map_entity(child))
    {
        collect_participant_from_node(
            gridcell,
            &format!("galaxymap/gridcell/{}", gridcell.id.raw()),
            &owner_ids,
            &mut seen_ids,
            &mut participants,
            report,
        )?;
        for (idx, child) in gridcell.children.iter().enumerate() {
            collect_participant_from_node(
                child,
                &format!("galaxymap/gridcell/{}/child_{idx}", gridcell.id.raw()),
                &owner_ids,
                &mut seen_ids,
                &mut participants,
                report,
            )?;
        }
    }

    Ok(participants)
}

fn collect_participant_from_node(
    node: &SimThing,
    path: &str,
    owner_ids: &BTreeSet<String>,
    seen_ids: &mut BTreeSet<u32>,
    participants: &mut Vec<FlowParticipant>,
    report: &mut OwnerSiloAdmissionReport,
) -> Result<(), ()> {
    let Some(owner_id) = owner_flow_owner_ref(node) else {
        return Ok(());
    };
    if owner_id.trim().is_empty() {
        push_error(
            report,
            OwnerSiloAdmissionErrorKind::MissingOwnerReference,
            Some(path.to_string()),
            Some(node.id.raw()),
            "participant flow owner reference is empty",
        );
        return Err(());
    }
    if !owner_ids.contains(&owner_id) {
        push_error(
            report,
            OwnerSiloAdmissionErrorKind::UnknownOwnerReference,
            Some(path.to_string()),
            Some(node.id.raw()),
            format!("unknown owner reference `{owner_id}`"),
        );
        return Err(());
    }
    if !seen_ids.insert(node.id.raw()) {
        push_error(
            report,
            OwnerSiloAdmissionErrorKind::DuplicateParticipantId,
            Some(path.to_string()),
            Some(node.id.raw()),
            format!("duplicate flow participant id {}", node.id.raw()),
        );
        return Err(());
    }

    let surplus = read_flow_amount(node, OWNER_FLOW_SURPLUS_PROPERTY_ID, path, report)?;
    let deficit = read_flow_amount(node, OWNER_FLOW_DEFICIT_PROPERTY_ID, path, report)?;

    if surplus == 0 && deficit == 0 {
        push_deferral(
            report,
            OwnerSiloDeferralKind::ResourceFlowExecutionDeferred,
            Some(path.to_string()),
            Some(node.id.raw()),
            "participant has owner reference but no surplus/deficit flow amounts",
        );
        return Ok(());
    }

    participants.push(FlowParticipant {
        simthing_id_raw: node.id.raw(),
        owner_id,
        surplus,
        deficit,
        path: path.to_string(),
    });
    Ok(())
}

fn read_flow_amount(
    node: &SimThing,
    property_id: simthing_core::SimPropertyId,
    path: &str,
    report: &mut OwnerSiloAdmissionReport,
) -> Result<u32, ()> {
    let Some(value) = node.properties.get(&property_id) else {
        return Ok(0);
    };
    match property_u32(value) {
        Some(amount) => Ok(amount),
        None => {
            let kind = if property_id == OWNER_FLOW_SURPLUS_PROPERTY_ID {
                OwnerSiloAdmissionErrorKind::InvalidSurplusAmount
            } else {
                OwnerSiloAdmissionErrorKind::InvalidDeficitAmount
            };
            push_error(
                report,
                kind,
                Some(path.to_string()),
                Some(node.id.raw()),
                "flow amount must be a non-negative exact integer f32 mirror",
            );
            Err(())
        }
    }
}

fn push_deferral(
    report: &mut OwnerSiloAdmissionReport,
    kind: OwnerSiloDeferralKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    reason: &str,
) {
    report.deferrals.push(OwnerSiloDeferral {
        kind,
        path,
        simthing_id_raw,
        reason: reason.to_string(),
    });
}

fn push_error(
    report: &mut OwnerSiloAdmissionReport,
    kind: OwnerSiloAdmissionErrorKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    message: impl Into<String>,
) {
    report.rejected_participant_count += 1;
    report.errors.push(OwnerSiloAdmissionError {
        kind,
        path,
        simthing_id_raw,
        message: message.into(),
    });
}

fn finalize_owner_silo_classification(report: &mut OwnerSiloAdmissionReport) {
    if !report.errors.is_empty() {
        report.classification = OwnerSiloAdmissionClassification::Rejected;
        return;
    }
    if report.deferrals.is_empty() {
        report.classification = OwnerSiloAdmissionClassification::Admitted;
    } else {
        report.classification = OwnerSiloAdmissionClassification::PartiallyAdmitted;
    }
}

/// Session-local SimThing raw ids for admitted owner-silo flow participants.
pub fn owner_silo_flow_participant_roots(spec: &SimThingScenarioSpec) -> Vec<u32> {
    let mut report = OwnerSiloAdmissionReport::default();
    collect_flow_participants(spec, &mut report)
        .ok()
        .map(|participants| {
            participants
                .iter()
                .map(|p| p.simthing_id_raw)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Returns true when owner silo flow is admitted enough to suppress blanket ingestion deferral.
pub fn owner_silo_flow_suppresses_ingestion_deferral(report: &OwnerSiloAdmissionReport) -> bool {
    report.classification != OwnerSiloAdmissionClassification::Rejected
        && report.participant_count > 0
        && report.errors.is_empty()
}
