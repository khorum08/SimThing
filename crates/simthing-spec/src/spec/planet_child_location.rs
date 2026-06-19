//! PLANET-CHILD-LOCATION-ADMISSION-0 — planets as child Location SimThings under star-system gridcells.

use std::collections::BTreeSet;

use simthing_core::{SimThing, SimThingKind};

use super::scenario::{
    game_session_child_mut, game_session_galaxy_map, gridcell_role, is_galaxy_map_entity,
    scenario_metadata_string, scenario_metadata_string_value, SimThingScenarioSpec,
    GALAXY_CHILD_LOCATION_ROLE_PLANET, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, PLANET_DISPLAY_NAME_PROPERTY_ID,
    PLANET_ID_PROPERTY_ID, PLANET_OWNER_REF_PROPERTY_ID,
};

// ---------------------------------------------------------------------------
// Admission types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlanetChildLocationAdmissionClassification {
    #[default]
    Admitted,
    PartiallyAdmitted,
    Rejected,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetChildLocationAdmissionErrorKind {
    PlanetUnderInertGridcell,
    PlanetMissingId,
    DuplicatePlanetIdWithinScenario,
    PlanetListedInStructuralGrid,
    UnsupportedChildLocationRole,
    DeepChildLocationDeferred,
    PlanetOwnershipResolutionDeferred,
    PlanetSimulationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildLocationAdmissionError {
    pub kind: PlanetChildLocationAdmissionErrorKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildLocationDeferral {
    pub kind: PlanetChildLocationAdmissionErrorKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanetChildLocationAdmissionReport {
    pub star_system_gridcell_count: u32,
    pub inert_gridcell_count: u32,
    pub planet_count: u32,
    pub unsupported_child_location_count: u32,
    pub classification: PlanetChildLocationAdmissionClassification,
    pub deferrals: Vec<PlanetChildLocationDeferral>,
    pub errors: Vec<PlanetChildLocationAdmissionError>,
}

// ---------------------------------------------------------------------------
// Metadata helpers
// ---------------------------------------------------------------------------

pub fn child_location_role(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID)
}

pub fn planet_id(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, PLANET_ID_PROPERTY_ID)
}

pub fn planet_display_name(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, PLANET_DISPLAY_NAME_PROPERTY_ID)
}

pub fn planet_owner_ref(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, PLANET_OWNER_REF_PROPERTY_ID)
}

pub fn is_planet_child_location(thing: &SimThing) -> bool {
    thing.kind == SimThingKind::Location
        && child_location_role(thing).as_deref() == Some(GALAXY_CHILD_LOCATION_ROLE_PLANET)
}

pub fn apply_planet_child_metadata(
    planet: &mut SimThing,
    planet_id: &str,
    display_name: Option<&str>,
) {
    debug_assert_eq!(planet.kind, SimThingKind::Location);
    planet.add_property(
        GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID,
        scenario_metadata_string_value(GALAXY_CHILD_LOCATION_ROLE_PLANET),
    );
    planet.add_property(
        PLANET_ID_PROPERTY_ID,
        scenario_metadata_string_value(planet_id),
    );
    if let Some(name) = display_name {
        planet.add_property(
            PLANET_DISPLAY_NAME_PROPERTY_ID,
            scenario_metadata_string_value(name),
        );
    }
}

pub fn make_planet_child_location(planet_id: &str, display_name: Option<&str>) -> SimThing {
    let mut planet = SimThing::new(SimThingKind::Location, 0);
    apply_planet_child_metadata(&mut planet, planet_id, display_name);
    planet
}

/// Direct star-system gridcell Location children under GalaxyMap.
pub fn star_system_gridcells(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<&SimThing>, super::scenario::ScenarioRootError> {
    let galaxy_map = game_session_galaxy_map(spec)?;
    Ok(galaxy_map
        .children
        .iter()
        .filter(|child| {
            child.kind == SimThingKind::Location
                && !is_galaxy_map_entity(child)
                && gridcell_role(child).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .collect())
}

/// Planet / child Location nodes under one gridcell.
pub fn planet_child_locations<'a>(
    spec: &'a SimThingScenarioSpec,
    gridcell: &'a SimThing,
) -> Vec<&'a SimThing> {
    let _ = spec;
    gridcell
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Location && is_planet_child_location(child))
        .collect()
}

pub fn all_planet_child_locations(spec: &SimThingScenarioSpec) -> Vec<&SimThing> {
    let Ok(galaxy_map) = game_session_galaxy_map(spec) else {
        return Vec::new();
    };
    galaxy_map
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location && !is_galaxy_map_entity(c))
        .flat_map(|gridcell| planet_child_locations(spec, gridcell))
        .collect()
}

// ---------------------------------------------------------------------------
// Admission evaluation
// ---------------------------------------------------------------------------

pub fn evaluate_planet_child_locations(
    spec: &SimThingScenarioSpec,
) -> PlanetChildLocationAdmissionReport {
    let mut report = PlanetChildLocationAdmissionReport::default();
    let Ok(galaxy_map) = game_session_galaxy_map(spec) else {
        report.classification = PlanetChildLocationAdmissionClassification::Rejected;
        return report;
    };

    let placement_ids: BTreeSet<u32> = spec
        .structural_grid
        .placements
        .iter()
        .map(|p| p.simthing_id_raw)
        .collect();
    let mut seen_planet_ids = BTreeSet::new();

    for gridcell in galaxy_map
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location && !is_galaxy_map_entity(c))
    {
        match gridcell_role(gridcell).as_deref() {
            Some(GALAXY_GRIDCELL_ROLE_INERT) => {
                report.inert_gridcell_count += 1;
                for child in &gridcell.children {
                    if child.kind == SimThingKind::Location
                        || matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet")
                    {
                        push_error(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::PlanetUnderInertGridcell,
                            Some(format!("gridcell/inert/child:{}", child.id.raw())),
                            Some(child.id.raw()),
                            "planet child Location cannot live under inert gridcell",
                        );
                    }
                }
            }
            Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => {
                report.star_system_gridcell_count += 1;
                for child in &gridcell.children {
                    if child.kind == SimThingKind::Location {
                        evaluate_star_system_child(
                            spec,
                            gridcell,
                            child,
                            &placement_ids,
                            &mut seen_planet_ids,
                            &mut report,
                        );
                    } else if matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet")
                    {
                        report.unsupported_child_location_count += 1;
                        push_deferral(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                            Some(format!("gridcell/{}/legacy_planet:{}", gridcell.id.raw(), child.id.raw())),
                            Some(child.id.raw()),
                            "legacy Custom Planet child is not admitted; use Location + planet role metadata",
                        );
                    }
                }
            }
            _ => {
                for child in location_children(gridcell) {
                    push_error(
                        &mut report,
                        PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                        Some(format!("gridcell/unknown_role/child:{}", child.id.raw())),
                        Some(child.id.raw()),
                        "child Location under non-star-system gridcell is not admitted",
                    );
                    report.unsupported_child_location_count += 1;
                }
            }
        }
    }

    finalize_planet_classification(&mut report);
    report
}

pub fn validate_planet_child_locations(
    spec: &SimThingScenarioSpec,
) -> Result<PlanetChildLocationAdmissionReport, PlanetChildLocationAdmissionError> {
    Ok(evaluate_planet_child_locations(spec))
}

fn location_children(gridcell: &SimThing) -> Vec<&SimThing> {
    gridcell
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location)
        .collect()
}

fn evaluate_star_system_child(
    _spec: &SimThingScenarioSpec,
    gridcell: &SimThing,
    child: &SimThing,
    placement_ids: &BTreeSet<u32>,
    seen_planet_ids: &mut BTreeSet<String>,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    let child_role = child_location_role(child);
    let path_prefix = format!("gridcell/{}/child:{}", gridcell.id.raw(), child.id.raw());

    if placement_ids.contains(&child.id.raw()) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetListedInStructuralGrid,
            Some(path_prefix.clone()),
            Some(child.id.raw()),
            "planet Location must not appear in structural_grid placements",
        );
        return;
    }

    match child_role.as_deref() {
        Some(GALAXY_CHILD_LOCATION_ROLE_PLANET) => {
            let Some(id) = planet_id(child) else {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetMissingId,
                    Some(path_prefix),
                    Some(child.id.raw()),
                    "planet child Location is missing planet_id metadata",
                );
                return;
            };
            if id.trim().is_empty() {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetMissingId,
                    Some(path_prefix),
                    Some(child.id.raw()),
                    "planet_id must be non-empty",
                );
                return;
            }
            if !seen_planet_ids.insert(id.clone()) {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::DuplicatePlanetIdWithinScenario,
                    Some(path_prefix),
                    Some(child.id.raw()),
                    format!("duplicate planet_id `{id}` within scenario"),
                );
                return;
            }

            report.planet_count += 1;
            if has_deeper_location_nesting(child) {
                push_deferral(
                    report,
                    PlanetChildLocationAdmissionErrorKind::DeepChildLocationDeferred,
                    Some(path_prefix.clone()),
                    Some(child.id.raw()),
                    "deeper Location nesting below planet is deferred",
                );
            }
            if planet_owner_ref(child).is_some() {
                push_deferral(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred,
                    Some(path_prefix.clone()),
                    Some(child.id.raw()),
                    "planet ownership reference present; resolution deferred",
                );
            }
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred,
                Some(path_prefix),
                Some(child.id.raw()),
                "planet simulation/economy/population behavior remains deferred",
            );
        }
        Some(role) => {
            report.unsupported_child_location_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix),
                Some(child.id.raw()),
                format!("child-location role `{role}` is not yet admitted"),
            );
        }
        None => {
            report.unsupported_child_location_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix),
                Some(child.id.raw()),
                "Location child under star-system gridcell lacks child-location role metadata",
            );
        }
    }
}

fn has_deeper_location_nesting(planet: &SimThing) -> bool {
    planet
        .children
        .iter()
        .any(|c| c.kind == SimThingKind::Location)
}

fn finalize_planet_classification(report: &mut PlanetChildLocationAdmissionReport) {
    if !report.errors.is_empty() {
        report.classification = PlanetChildLocationAdmissionClassification::Rejected;
        return;
    }
    if report.planet_count == 0 && report.unsupported_child_location_count > 0 {
        report.classification = PlanetChildLocationAdmissionClassification::Unsupported;
        return;
    }
    let hard_deferrals = report.deferrals.iter().filter(|d| {
        !matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred
                | PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred
                | PlanetChildLocationAdmissionErrorKind::DeepChildLocationDeferred
        )
    });
    if hard_deferrals.count() > 0 {
        report.classification = PlanetChildLocationAdmissionClassification::PartiallyAdmitted;
        return;
    }
    if report.deferrals.is_empty() && report.planet_count == 0 {
        report.classification = PlanetChildLocationAdmissionClassification::Admitted;
        return;
    }
    if report.planet_count > 0 {
        report.classification = PlanetChildLocationAdmissionClassification::PartiallyAdmitted;
        return;
    }
    report.classification = PlanetChildLocationAdmissionClassification::Admitted;
}

fn push_error(
    report: &mut PlanetChildLocationAdmissionReport,
    kind: PlanetChildLocationAdmissionErrorKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    message: impl Into<String>,
) {
    report.errors.push(PlanetChildLocationAdmissionError {
        kind,
        path,
        simthing_id_raw,
        message: message.into(),
    });
}

fn push_deferral(
    report: &mut PlanetChildLocationAdmissionReport,
    kind: PlanetChildLocationAdmissionErrorKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    reason: impl Into<String>,
) {
    report.deferrals.push(PlanetChildLocationDeferral {
        kind,
        path,
        simthing_id_raw,
        reason: reason.into(),
    });
}

pub fn planet_child_location_error_kind_label(
    kind: PlanetChildLocationAdmissionErrorKind,
) -> &'static str {
    match kind {
        PlanetChildLocationAdmissionErrorKind::PlanetUnderInertGridcell => {
            "PlanetUnderInertGridcell"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetMissingId => "PlanetMissingId",
        PlanetChildLocationAdmissionErrorKind::DuplicatePlanetIdWithinScenario => {
            "DuplicatePlanetIdWithinScenario"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetListedInStructuralGrid => {
            "PlanetListedInStructuralGrid"
        }
        PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole => {
            "UnsupportedChildLocationRole"
        }
        PlanetChildLocationAdmissionErrorKind::DeepChildLocationDeferred => {
            "DeepChildLocationDeferred"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred => {
            "PlanetOwnershipResolutionDeferred"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred => {
            "PlanetSimulationDeferred"
        }
    }
}

pub fn planet_child_location_classification_label(
    classification: PlanetChildLocationAdmissionClassification,
) -> &'static str {
    match classification {
        PlanetChildLocationAdmissionClassification::Admitted => "Admitted",
        PlanetChildLocationAdmissionClassification::PartiallyAdmitted => "PartiallyAdmitted",
        PlanetChildLocationAdmissionClassification::Rejected => "Rejected",
        PlanetChildLocationAdmissionClassification::Unsupported => "Unsupported",
    }
}

// ---------------------------------------------------------------------------
// Edit commands (separate from structural placement commands)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanetChildLocationCommand {
    AddPlanet {
        star_system_gridcell_id: String,
        planet_id: String,
        display_name: Option<String>,
    },
    RemovePlanet {
        planet_id: String,
    },
    SetPlanetDisplayName {
        planet_id: String,
        display_name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetChildLocationEditErrorKind {
    MissingGalaxyMap,
    GridcellNotFound,
    GridcellNotStarSystem,
    PlanetUnderInertGridcell,
    DuplicatePlanetId,
    PlanetNotFound,
    ValidationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetChildLocationEditError {
    pub kind: PlanetChildLocationEditErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanetChildLocationEditReport {
    pub command_count: u32,
    pub applied_count: u32,
    pub rejected_count: u32,
}

impl std::fmt::Display for PlanetChildLocationEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for PlanetChildLocationEditError {}

pub fn apply_planet_child_location_command(
    spec: &mut SimThingScenarioSpec,
    command: PlanetChildLocationCommand,
) -> Result<PlanetChildLocationEditReport, PlanetChildLocationEditError> {
    let mut draft = spec.clone();
    let mut report = PlanetChildLocationEditReport {
        command_count: 1,
        ..Default::default()
    };
    match apply_command_to_draft(&mut draft, &command) {
        Ok(()) => {
            let admission = evaluate_planet_child_locations(&draft);
            if admission.classification == PlanetChildLocationAdmissionClassification::Rejected {
                return Err(edit_error(
                    PlanetChildLocationEditErrorKind::ValidationFailed,
                    admission
                        .errors
                        .first()
                        .map(|e| e.message.clone())
                        .unwrap_or_else(|| "planet child-location validation failed".into()),
                ));
            }
            *spec = draft;
            report.applied_count = 1;
            Ok(report)
        }
        Err(err) => {
            report.rejected_count = 1;
            Err(err)
        }
    }
}

fn apply_command_to_draft(
    spec: &mut SimThingScenarioSpec,
    command: &PlanetChildLocationCommand,
) -> Result<(), PlanetChildLocationEditError> {
    match command {
        PlanetChildLocationCommand::AddPlanet {
            star_system_gridcell_id,
            planet_id,
            display_name,
        } => add_planet(
            spec,
            star_system_gridcell_id,
            planet_id,
            display_name.as_deref(),
        ),
        PlanetChildLocationCommand::RemovePlanet { planet_id } => remove_planet(spec, planet_id),
        PlanetChildLocationCommand::SetPlanetDisplayName {
            planet_id,
            display_name,
        } => set_planet_display_name(spec, planet_id, display_name),
    }
}

fn add_planet(
    spec: &mut SimThingScenarioSpec,
    gridcell_id: &str,
    target_planet_id: &str,
    display_name: Option<&str>,
) -> Result<(), PlanetChildLocationEditError> {
    let trimmed_id = target_planet_id.trim();
    if trimmed_id.is_empty() {
        return Err(edit_error(
            PlanetChildLocationEditErrorKind::ValidationFailed,
            "planet_id must be non-empty",
        ));
    }
    if all_planet_child_locations(spec)
        .iter()
        .any(|p| planet_id(p).as_deref() == Some(trimmed_id))
    {
        return Err(edit_error(
            PlanetChildLocationEditErrorKind::DuplicatePlanetId,
            format!("planet_id `{trimmed_id}` already exists"),
        ));
    }

    let gridcell_raw = resolve_gridcell_raw(spec, gridcell_id)?;
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .ok_or_else(|| {
            edit_error(
                PlanetChildLocationEditErrorKind::GridcellNotFound,
                format!("gridcell `{gridcell_id}` not found under GalaxyMap"),
            )
        })?;

    match gridcell_role(gridcell).as_deref() {
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => {}
        Some(GALAXY_GRIDCELL_ROLE_INERT) => {
            return Err(edit_error(
                PlanetChildLocationEditErrorKind::PlanetUnderInertGridcell,
                "cannot add planet under inert gridcell",
            ));
        }
        _ => {
            return Err(edit_error(
                PlanetChildLocationEditErrorKind::GridcellNotStarSystem,
                "planet child Locations require star-system gridcell parent",
            ));
        }
    }

    let planet = make_planet_child_location(trimmed_id, display_name);
    gridcell.add_child(planet);
    Ok(())
}

fn remove_planet(
    spec: &mut SimThingScenarioSpec,
    target_planet_id: &str,
) -> Result<(), PlanetChildLocationEditError> {
    let trimmed = target_planet_id.trim();
    let (gridcell_raw, child_raw) = locate_planet(spec, trimmed)?;
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("parent gridcell");
    let idx = gridcell
        .children
        .iter()
        .position(|c| c.id.raw() == child_raw)
        .ok_or_else(|| {
            edit_error(
                PlanetChildLocationEditErrorKind::PlanetNotFound,
                format!("planet `{trimmed}` not found"),
            )
        })?;
    gridcell.children.remove(idx);
    Ok(())
}

fn set_planet_display_name(
    spec: &mut SimThingScenarioSpec,
    target_planet_id: &str,
    display_name: &str,
) -> Result<(), PlanetChildLocationEditError> {
    let trimmed = target_planet_id.trim();
    let (gridcell_raw, child_raw) = locate_planet(spec, trimmed)?;
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("parent gridcell");
    let planet = gridcell
        .children
        .iter_mut()
        .find(|c| c.id.raw() == child_raw)
        .expect("planet child");
    planet.add_property(
        PLANET_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
    Ok(())
}

fn locate_planet(
    spec: &SimThingScenarioSpec,
    target_planet_id: &str,
) -> Result<(u32, u32), PlanetChildLocationEditError> {
    let galaxy_map = game_session_galaxy_map(spec).map_err(|_| {
        edit_error(
            PlanetChildLocationEditErrorKind::MissingGalaxyMap,
            "GalaxyMap child missing",
        )
    })?;
    for gridcell in galaxy_map
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location && !is_galaxy_map_entity(c))
    {
        for child in &gridcell.children {
            if is_planet_child_location(child)
                && planet_id(child).as_deref() == Some(target_planet_id)
            {
                return Ok((gridcell.id.raw(), child.id.raw()));
            }
        }
    }
    Err(edit_error(
        PlanetChildLocationEditErrorKind::PlanetNotFound,
        format!("planet `{target_planet_id}` not found"),
    ))
}

fn resolve_gridcell_raw(
    spec: &SimThingScenarioSpec,
    gridcell_id: &str,
) -> Result<u32, PlanetChildLocationEditError> {
    let trimmed = gridcell_id.trim();
    if let Some(placement) = spec
        .structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == trimmed || p.target_id == trimmed)
    {
        return Ok(placement.simthing_id_raw);
    }
    let galaxy_map = game_session_galaxy_map(spec).map_err(|_| {
        edit_error(
            PlanetChildLocationEditErrorKind::MissingGalaxyMap,
            "GalaxyMap child missing",
        )
    })?;
    for child in &galaxy_map.children {
        if child.kind == SimThingKind::Location
            && !is_galaxy_map_entity(child)
            && child.id.raw().to_string() == trimmed
        {
            return Ok(child.id.raw());
        }
    }
    Err(edit_error(
        PlanetChildLocationEditErrorKind::GridcellNotFound,
        format!("gridcell `{trimmed}` not found"),
    ))
}

fn game_session_galaxy_map_mut(
    spec: &mut SimThingScenarioSpec,
) -> Result<&mut SimThing, PlanetChildLocationEditError> {
    let map_raw = game_session_galaxy_map(spec)
        .map_err(|err| {
            edit_error(
                PlanetChildLocationEditErrorKind::MissingGalaxyMap,
                err.to_string(),
            )
        })?
        .id
        .raw();
    let game_session = game_session_child_mut(spec).map_err(|err| {
        edit_error(
            PlanetChildLocationEditErrorKind::ValidationFailed,
            err.to_string(),
        )
    })?;
    game_session
        .children
        .iter_mut()
        .find(|child| child.id.raw() == map_raw)
        .ok_or_else(|| {
            edit_error(
                PlanetChildLocationEditErrorKind::MissingGalaxyMap,
                "GalaxyMap child missing under GameSession",
            )
        })
}

fn edit_error(
    kind: PlanetChildLocationEditErrorKind,
    message: impl Into<String>,
) -> PlanetChildLocationEditError {
    PlanetChildLocationEditError {
        kind,
        message: message.into(),
    }
}
