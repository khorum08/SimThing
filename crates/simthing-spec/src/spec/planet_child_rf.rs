//! PLANET-CHILD-RF-GPU-PARTICIPANT-0 — owner/channel RF participants from planet gridcells and
//! admitted non-grid children. Ownership is metadata-driven; spatial parentage is unchanged.
//!
//! PLANET-CHILD-RF-REDUCE-UP-0 — scoped owner/resource/planet reduce-up over admitted participants.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::SimThing;

use super::channel_key::{OwnerRef, ResourceKey, ScopeId};
use super::planet_child_location::{
    evaluate_planet_child_locations, is_admitted_planet_non_grid_child, planet_id,
    planet_non_grid_child_kind_label, planet_non_grid_child_owner_ref, planet_owner_ref,
    PlanetChildLocationAdmissionClassification,
};
use super::scenario::{
    game_session_owners, owner_entity_id, owner_flow_deficit, owner_flow_owner_ref,
    owner_flow_surplus, property_u32, SimThingScenarioSpec, OWNER_FLOW_DEFICIT_PROPERTY_ID,
    OWNER_FLOW_SURPLUS_PROPERTY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlanetChildRfAdmissionClassification {
    #[default]
    Admitted,
    PartiallyAdmitted,
    Rejected,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetChildRfAdmissionErrorKind {
    InvalidPlanetChildRfAmount,
    MissingOwnerChannelForActiveRfParticipant,
    MissingPlanetGridcellAdmission,
    UnsupportedPlanetChildRfKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetChildRfDeferralKind {
    PlanetChildRfSimulationDeferred,
    PlanetChildRfGpuExecutionDeferred,
    PlanetChildRfNoParticipants,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfAdmissionError {
    pub kind: PlanetChildRfAdmissionErrorKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfDeferral {
    pub kind: PlanetChildRfDeferralKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetChildRfAdmissionReport {
    pub planet_gridcell_count: u32,
    pub admitted_planet_gridcell_participant_count: u32,
    pub admitted_non_grid_child_participant_count: u32,
    pub total_participant_count: u32,
    pub owner_channel_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub classification: PlanetChildRfAdmissionClassification,
    pub deferrals: Vec<PlanetChildRfDeferral>,
    pub errors: Vec<PlanetChildRfAdmissionError>,
}

/// Default resource key when participant metadata carries surplus/deficit only.
pub const PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY: &str = "generic";

/// Default [`ResourceKey`] for planet-child RF participants after admission.
pub fn planet_child_rf_default_resource_key() -> ResourceKey {
    ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY)
}

/// Scoped owner/resource/planet RF channel key. Owner ref is metadata, not spatial parentage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanetChildRfScopeKey {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub local_scope_id: Option<ScopeId>,
    pub planet_id: Option<String>,
    pub star_system_gridcell_id_raw: Option<u32>,
}

/// Per-scope reduce-up bucket after grouping admitted participants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfReduceUpBucket {
    pub scope: PlanetChildRfScopeKey,
    pub participant_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub net_surplus: i64,
    pub net_deficit: i64,
}

/// Scoped reduce-up summary over admitted planet child RF participants.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetChildRfReduceUpReport {
    pub participant_count: u32,
    pub bucket_count: u32,
    pub planet_scope_count: u32,
    pub star_system_scope_count: u32,
    pub surplus_total: u32,
    pub deficit_total: u32,
    pub buckets: Vec<PlanetChildRfReduceUpBucket>,
    pub classification: PlanetChildRfAdmissionClassification,
    pub deferrals: Vec<PlanetChildRfDeferral>,
    pub errors: Vec<PlanetChildRfAdmissionError>,
}

/// Admitted planet/non-grid child RF participant row for driver accumulator lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfParticipantInput {
    pub simthing_id_raw: u32,
    pub planet_gridcell_id_raw: u32,
    pub planet_id: String,
    pub owner_ref: OwnerRef,
    pub surplus: u32,
    pub deficit: u32,
    pub participant_kind_label: String,
    pub spatial_parent_path: String,
}

pub fn planet_child_rf_admission_classification_label(
    classification: PlanetChildRfAdmissionClassification,
) -> &'static str {
    match classification {
        PlanetChildRfAdmissionClassification::Admitted => "admitted",
        PlanetChildRfAdmissionClassification::PartiallyAdmitted => "partially_admitted",
        PlanetChildRfAdmissionClassification::Rejected => "rejected",
        PlanetChildRfAdmissionClassification::Unsupported => "unsupported",
    }
}

pub fn evaluate_planet_child_rf_admission(
    spec: &SimThingScenarioSpec,
) -> PlanetChildRfAdmissionReport {
    let mut report = PlanetChildRfAdmissionReport {
        classification: PlanetChildRfAdmissionClassification::Admitted,
        ..Default::default()
    };

    let planet_location = evaluate_planet_child_locations(spec);
    report.planet_gridcell_count = planet_location.planet_gridcell_count;

    if planet_location.classification == PlanetChildLocationAdmissionClassification::Rejected {
        push_error(
            &mut report,
            PlanetChildRfAdmissionErrorKind::MissingPlanetGridcellAdmission,
            None,
            None,
            "planet/local-grid admission rejected; planet child RF participants blocked",
        );
        report.classification = PlanetChildRfAdmissionClassification::Rejected;
        return report;
    }

    if planet_location.classification == PlanetChildLocationAdmissionClassification::Unsupported {
        report.classification = PlanetChildRfAdmissionClassification::Unsupported;
        return report;
    }

    let owner_refs: BTreeSet<OwnerRef> = game_session_owners(spec)
        .ok()
        .map(|owners| {
            owners
                .into_iter()
                .filter_map(owner_entity_id)
                .map(OwnerRef::new)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();

    let participants = match collect_planet_child_rf_participants(spec, &owner_refs, &mut report) {
        Ok(participants) => participants,
        Err(()) => {
            report.classification = PlanetChildRfAdmissionClassification::Rejected;
            return report;
        }
    };

    let mut owner_channels = BTreeSet::new();
    for participant in &participants {
        report.total_participant_count += 1;
        report.surplus_total = report.surplus_total.saturating_add(participant.surplus);
        report.deficit_total = report.deficit_total.saturating_add(participant.deficit);
        owner_channels.insert(participant.owner_ref.clone());
        if participant.participant_kind_label == "planet_gridcell" {
            report.admitted_planet_gridcell_participant_count += 1;
        } else {
            report.admitted_non_grid_child_participant_count += 1;
        }
    }
    report.owner_channel_count = owner_channels.len() as u32;

    if participants.is_empty() {
        push_deferral(
            &mut report,
            PlanetChildRfDeferralKind::PlanetChildRfNoParticipants,
            None,
            None,
            "no planet gridcell or non-grid child RF participants with active surplus/deficit metadata",
        );
    } else {
        push_deferral(
            &mut report,
            PlanetChildRfDeferralKind::PlanetChildRfSimulationDeferred,
            None,
            None,
            "planet child RF simulation and economic disburse-down remain deferred",
        );
        push_deferral(
            &mut report,
            PlanetChildRfDeferralKind::PlanetChildRfGpuExecutionDeferred,
            None,
            None,
            "GPU participant accumulation is proof-only; full owner-silo state mutation remains deferred",
        );
    }

    finalize_planet_child_rf_classification(&mut report);
    report
}

/// Explicit admitted planet/non-grid child RF participant inputs for generic accumulator lowering.
pub fn planet_child_rf_participant_inputs(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<PlanetChildRfParticipantInput>, PlanetChildRfAdmissionReport> {
    let report = evaluate_planet_child_rf_admission(spec);
    if report.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(report);
    }
    let owner_refs: BTreeSet<OwnerRef> = game_session_owners(spec)
        .ok()
        .map(|owners| {
            owners
                .into_iter()
                .filter_map(owner_entity_id)
                .map(OwnerRef::new)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let mut scratch = PlanetChildRfAdmissionReport::default();
    let participants = collect_planet_child_rf_participants(spec, &owner_refs, &mut scratch)
        .map_err(|_| report.clone())?;
    if participants.is_empty() {
        return Err(report);
    }
    Ok(participants)
}

/// Group admitted planet child RF participants into scoped owner/resource/planet reduce-up buckets.
pub fn evaluate_planet_child_rf_reduce_up(
    spec: &SimThingScenarioSpec,
) -> PlanetChildRfReduceUpReport {
    let admission = evaluate_planet_child_rf_admission(spec);
    let mut report = PlanetChildRfReduceUpReport {
        classification: admission.classification,
        deferrals: admission.deferrals.clone(),
        errors: admission.errors.clone(),
        ..Default::default()
    };

    if admission.classification == PlanetChildRfAdmissionClassification::Rejected {
        return report;
    }
    if admission.classification == PlanetChildRfAdmissionClassification::Unsupported {
        return report;
    }

    let participants = match planet_child_rf_participant_inputs(spec) {
        Ok(participants) => participants,
        Err(no_participants) => {
            report.classification = no_participants.classification;
            report.deferrals = no_participants.deferrals;
            report.errors = no_participants.errors;
            return report;
        }
    };

    let mut bucket_map: BTreeMap<PlanetChildRfScopeKey, PlanetChildRfReduceUpBucket> =
        BTreeMap::new();
    let mut participant_count: u32 = 0;
    let mut surplus_total: u32 = 0;
    let mut deficit_total: u32 = 0;

    for participant in participants {
        participant_count = match participant_count.checked_add(1) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };
        surplus_total = match surplus_total.checked_add(participant.surplus) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };
        deficit_total = match deficit_total.checked_add(participant.deficit) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };

        let scope = scope_key_from_participant(&participant);
        let entry =
            bucket_map
                .entry(scope.clone())
                .or_insert_with(|| PlanetChildRfReduceUpBucket {
                    scope,
                    participant_count: 0,
                    surplus_total: 0,
                    deficit_total: 0,
                    net_surplus: 0,
                    net_deficit: 0,
                });
        entry.participant_count = match entry.participant_count.checked_add(1) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };
        entry.surplus_total = match entry.surplus_total.checked_add(participant.surplus) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };
        entry.deficit_total = match entry.deficit_total.checked_add(participant.deficit) {
            Some(v) => v,
            None => {
                push_reduce_up_overflow_error(&mut report);
                report.classification = PlanetChildRfAdmissionClassification::Rejected;
                return report;
            }
        };
    }

    let mut buckets: Vec<PlanetChildRfReduceUpBucket> = bucket_map.into_values().collect();
    for bucket in &mut buckets {
        let surplus = bucket.surplus_total as i64;
        let deficit = bucket.deficit_total as i64;
        bucket.net_surplus = (surplus - deficit).max(0);
        bucket.net_deficit = (deficit - surplus).max(0);
    }

    let planet_scope_count = buckets
        .iter()
        .filter_map(|b| b.scope.planet_id.as_ref())
        .collect::<BTreeSet<_>>()
        .len() as u32;
    let star_system_scope_count = buckets
        .iter()
        .filter_map(|b| b.scope.star_system_gridcell_id_raw)
        .collect::<BTreeSet<_>>()
        .len() as u32;

    report.participant_count = participant_count;
    report.bucket_count = buckets.len() as u32;
    report.planet_scope_count = planet_scope_count;
    report.star_system_scope_count = star_system_scope_count;
    report.surplus_total = surplus_total;
    report.deficit_total = deficit_total;
    report.buckets = buckets;

    push_deferral_reduce_up(
        &mut report,
        PlanetChildRfDeferralKind::PlanetChildRfSimulationDeferred,
        None,
        None,
        "scoped reduce-up is oracle/proof only; full owner-silo state mutation and disburse-down remain deferred",
    );

    if report.participant_count > 0 {
        report.classification = PlanetChildRfAdmissionClassification::PartiallyAdmitted;
    } else {
        report.classification = PlanetChildRfAdmissionClassification::PartiallyAdmitted;
    }

    report
}

/// Derive the scoped reduce-up key for a participant row.
pub fn scope_key_from_participant(
    participant: &PlanetChildRfParticipantInput,
) -> PlanetChildRfScopeKey {
    PlanetChildRfScopeKey {
        owner_ref: participant.owner_ref.clone(),
        resource_key: planet_child_rf_default_resource_key(),
        local_scope_id: Some(ScopeId::new(participant.planet_id.clone())),
        planet_id: Some(participant.planet_id.clone()),
        star_system_gridcell_id_raw: star_system_gridcell_id_from_path(
            &participant.spatial_parent_path,
        ),
    }
}

fn star_system_gridcell_id_from_path(path: &str) -> Option<u32> {
    path.strip_prefix("galaxymap/star_system/")
        .and_then(|rest| rest.split('/').next())
        .and_then(|raw| raw.parse().ok())
}

fn push_reduce_up_overflow_error(report: &mut PlanetChildRfReduceUpReport) {
    report.errors.push(PlanetChildRfAdmissionError {
        kind: PlanetChildRfAdmissionErrorKind::InvalidPlanetChildRfAmount,
        path: None,
        simthing_id_raw: None,
        message: "scoped reduce-up arithmetic overflow".to_string(),
    });
}

fn push_deferral_reduce_up(
    report: &mut PlanetChildRfReduceUpReport,
    kind: PlanetChildRfDeferralKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    reason: &str,
) {
    report.deferrals.push(PlanetChildRfDeferral {
        kind,
        path,
        simthing_id_raw,
        reason: reason.to_string(),
    });
}

fn collect_planet_child_rf_participants(
    spec: &SimThingScenarioSpec,
    owner_refs: &BTreeSet<OwnerRef>,
    report: &mut PlanetChildRfAdmissionReport,
) -> Result<Vec<PlanetChildRfParticipantInput>, ()> {
    use super::planet_child_location::star_system_gridcells;
    use super::scenario::game_session_galaxy_map;

    let _galaxy_map = game_session_galaxy_map(spec).map_err(|_| ())?;
    let mut participants = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for star_system in star_system_gridcells(spec).map_err(|_| ())? {
        let star_path = format!("galaxymap/star_system/{}", star_system.id.raw());
        for planet in super::planet_child_location::planet_gridcells(spec, star_system) {
            let planet_path = format!(
                "{}/planet/{}",
                star_path,
                planet_id(planet).unwrap_or_else(|| planet.id.raw().to_string())
            );
            collect_rf_participant_from_node(
                planet,
                planet.id.raw(),
                planet_id(planet).unwrap_or_default(),
                "planet_gridcell",
                &planet_path,
                owner_refs,
                &mut seen_ids,
                &mut participants,
                report,
                true,
            )?;

            for child in super::planet_child_location::planet_gameplay_children(planet) {
                if !is_admitted_planet_non_grid_child(&child.kind) {
                    if has_active_rf_metadata(child) {
                        push_error(
                            report,
                            PlanetChildRfAdmissionErrorKind::UnsupportedPlanetChildRfKind,
                            Some(format!("{}/child/{}", planet_path, child.id.raw())),
                            Some(child.id.raw()),
                            format!(
                                "unsupported non-grid child kind {:?} cannot contribute RF participants",
                                child.kind
                            ),
                        );
                        return Err(());
                    }
                    continue;
                }
                let child_path = format!(
                    "{}/child/{}/{}",
                    planet_path,
                    planet_non_grid_child_kind_label(&child.kind),
                    child.id.raw()
                );
                collect_rf_participant_from_node(
                    child,
                    planet.id.raw(),
                    planet_id(planet).unwrap_or_default(),
                    &planet_non_grid_child_kind_label(&child.kind),
                    &child_path,
                    owner_refs,
                    &mut seen_ids,
                    &mut participants,
                    report,
                    false,
                )?;
            }
        }
    }

    Ok(participants)
}

fn collect_rf_participant_from_node(
    node: &SimThing,
    planet_gridcell_id_raw: u32,
    planet_id: String,
    kind_label: &str,
    path: &str,
    owner_refs: &BTreeSet<OwnerRef>,
    seen_ids: &mut BTreeSet<u32>,
    participants: &mut Vec<PlanetChildRfParticipantInput>,
    report: &mut PlanetChildRfAdmissionReport,
    is_planet_gridcell: bool,
) -> Result<(), ()> {
    let surplus = read_rf_amount(
        node,
        OWNER_FLOW_SURPLUS_PROPERTY_ID,
        path,
        report,
        PlanetChildRfAdmissionErrorKind::InvalidPlanetChildRfAmount,
    )?;
    let deficit = read_rf_amount(
        node,
        OWNER_FLOW_DEFICIT_PROPERTY_ID,
        path,
        report,
        PlanetChildRfAdmissionErrorKind::InvalidPlanetChildRfAmount,
    )?;

    if surplus == 0 && deficit == 0 {
        return Ok(());
    }

    let owner_ref_str = resolve_participant_owner_ref(node, is_planet_gridcell);
    let Some(owner_ref_str) = owner_ref_str else {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            "active surplus/deficit metadata requires owner/channel reference",
        );
        return Err(());
    };
    if owner_ref_str.trim().is_empty() {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            "owner/channel reference is empty",
        );
        return Err(());
    }
    let owner_ref = OwnerRef::new(&owner_ref_str);
    if !owner_refs.contains(&owner_ref) {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            format!("unknown owner/channel reference `{owner_ref_str}`"),
        );
        return Err(());
    }
    if !seen_ids.insert(node.id.raw()) {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::InvalidPlanetChildRfAmount,
            Some(path.to_string()),
            Some(node.id.raw()),
            format!("duplicate RF participant id {}", node.id.raw()),
        );
        return Err(());
    }

    participants.push(PlanetChildRfParticipantInput {
        simthing_id_raw: node.id.raw(),
        planet_gridcell_id_raw,
        planet_id,
        owner_ref,
        surplus,
        deficit,
        participant_kind_label: kind_label.to_string(),
        spatial_parent_path: path.to_string(),
    });
    Ok(())
}

fn resolve_participant_owner_ref(node: &SimThing, is_planet_gridcell: bool) -> Option<String> {
    if is_planet_gridcell {
        planet_owner_ref(node).or_else(|| owner_flow_owner_ref(node))
    } else {
        planet_non_grid_child_owner_ref(node)
    }
}

fn has_active_rf_metadata(node: &SimThing) -> bool {
    owner_flow_surplus(node).is_some() || owner_flow_deficit(node).is_some()
}

fn read_rf_amount(
    node: &SimThing,
    property_id: simthing_core::SimPropertyId,
    path: &str,
    report: &mut PlanetChildRfAdmissionReport,
    error_kind: PlanetChildRfAdmissionErrorKind,
) -> Result<u32, ()> {
    let Some(value) = node.properties.get(&property_id) else {
        return Ok(0);
    };
    match property_u32(value) {
        Some(amount) => Ok(amount),
        None => {
            push_error(
                report,
                error_kind,
                Some(path.to_string()),
                Some(node.id.raw()),
                "RF amount must be a non-negative exact integer f32 mirror",
            );
            Err(())
        }
    }
}

fn push_deferral(
    report: &mut PlanetChildRfAdmissionReport,
    kind: PlanetChildRfDeferralKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    reason: &str,
) {
    report.deferrals.push(PlanetChildRfDeferral {
        kind,
        path,
        simthing_id_raw,
        reason: reason.to_string(),
    });
}

fn push_error(
    report: &mut PlanetChildRfAdmissionReport,
    kind: PlanetChildRfAdmissionErrorKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    message: impl Into<String>,
) {
    report.errors.push(PlanetChildRfAdmissionError {
        kind,
        path,
        simthing_id_raw,
        message: message.into(),
    });
}

fn finalize_planet_child_rf_classification(report: &mut PlanetChildRfAdmissionReport) {
    if !report.errors.is_empty() {
        report.classification = PlanetChildRfAdmissionClassification::Rejected;
        return;
    }
    if report.deferrals.is_empty() {
        report.classification = PlanetChildRfAdmissionClassification::Admitted;
    } else if report.total_participant_count > 0 {
        report.classification = PlanetChildRfAdmissionClassification::PartiallyAdmitted;
    } else {
        report.classification = PlanetChildRfAdmissionClassification::PartiallyAdmitted;
    }
}

impl Default for PlanetChildRfReduceUpReport {
    fn default() -> Self {
        Self {
            participant_count: 0,
            bucket_count: 0,
            planet_scope_count: 0,
            star_system_scope_count: 0,
            surplus_total: 0,
            deficit_total: 0,
            buckets: Vec::new(),
            classification: PlanetChildRfAdmissionClassification::Admitted,
            deferrals: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl Default for PlanetChildRfAdmissionReport {
    fn default() -> Self {
        Self {
            planet_gridcell_count: 0,
            admitted_planet_gridcell_participant_count: 0,
            admitted_non_grid_child_participant_count: 0,
            total_participant_count: 0,
            owner_channel_count: 0,
            surplus_total: 0,
            deficit_total: 0,
            classification: PlanetChildRfAdmissionClassification::Admitted,
            deferrals: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::SimThingKind;

}
