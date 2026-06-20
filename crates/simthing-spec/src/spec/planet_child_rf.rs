//! PLANET-CHILD-RF-GPU-PARTICIPANT-0 — owner/channel RF participants from planet gridcells and
//! admitted non-grid children. Ownership is metadata-driven; spatial parentage is unchanged.

use std::collections::BTreeSet;

use simthing_core::SimThing;

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

/// Admitted planet/non-grid child RF participant row for driver accumulator lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildRfParticipantInput {
    pub simthing_id_raw: u32,
    pub planet_gridcell_id_raw: u32,
    pub planet_id: String,
    pub owner_ref: String,
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

    let owner_refs: BTreeSet<String> = game_session_owners(spec)
        .ok()
        .map(|owners| {
            owners
                .into_iter()
                .filter_map(owner_entity_id)
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
    let owner_refs: BTreeSet<String> = game_session_owners(spec)
        .ok()
        .map(|owners| {
            owners
                .into_iter()
                .filter_map(owner_entity_id)
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

fn collect_planet_child_rf_participants(
    spec: &SimThingScenarioSpec,
    owner_refs: &BTreeSet<String>,
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

            for child in &planet.children {
                if child.kind == simthing_core::SimThingKind::Location {
                    continue;
                }
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
    owner_refs: &BTreeSet<String>,
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

    let owner_ref = resolve_participant_owner_ref(node, is_planet_gridcell);
    let Some(owner_ref) = owner_ref else {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            "active surplus/deficit metadata requires owner/channel reference",
        );
        return Err(());
    };
    if owner_ref.trim().is_empty() {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            "owner/channel reference is empty",
        );
        return Err(());
    }
    if !owner_refs.contains(&owner_ref) {
        push_error(
            report,
            PlanetChildRfAdmissionErrorKind::MissingOwnerChannelForActiveRfParticipant,
            Some(path.to_string()),
            Some(node.id.raw()),
            format!("unknown owner/channel reference `{owner_ref}`"),
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

    #[test]
    fn planet_gridcell_kind_helper_matches_location_role() {
        let mut planet =
            super::super::planet_child_location::make_planet_gridcell("p1", 0, 0, None);
        assert!(is_planet_gridcell(&planet));
        planet.kind = SimThingKind::Cohort;
        assert!(!is_planet_gridcell(&planet));
    }
}
