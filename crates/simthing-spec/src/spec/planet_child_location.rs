//! PLANET-LOCAL-GRID-REMEDIATION-0 — planets as star-system-local gridcell Location SimThings.

use std::collections::BTreeSet;

use simthing_core::{SimThing, SimThingKind};

use super::scenario::{
    game_session_child_mut, game_session_galaxy_map, gridcell_role, is_galaxy_map_entity,
    scenario_metadata_string, scenario_metadata_string_value, scenario_metadata_u32,
    structural_property_value_u32, SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, LOCAL_GRIDCELL_COL_PROPERTY_ID,
    LOCAL_GRIDCELL_ROLE_INERT, LOCAL_GRIDCELL_ROLE_PLANET, LOCAL_GRIDCELL_ROLE_PROPERTY_ID,
    LOCAL_GRIDCELL_ROW_PROPERTY_ID, PLANET_DISPLAY_NAME_PROPERTY_ID, PLANET_ID_PROPERTY_ID,
    PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID,
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
    PlanetUnderInertGalaxyGridcell,
    PlanetGridcellMissingId,
    DuplicatePlanetIdWithinScenario,
    PlanetListedInGalaxyStructuralGrid,
    UnsupportedChildLocationRole,
    DeepPlanetChildDeferred,
    PlanetOwnershipResolutionDeferred,
    PlanetSimulationDeferred,
    PlanetLocalGridMissingCoordinate,
    PlanetLocalGridDuplicateCoordinate,
    PlanetLocalGridCoordinateOutOfFrame,
    LocalGridFrameInvalid,
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
    pub local_gridcell_count: u32,
    pub local_inert_gridcell_count: u32,
    pub planet_gridcell_count: u32,
    pub unsupported_child_location_count: u32,
    pub classification: PlanetChildLocationAdmissionClassification,
    pub deferrals: Vec<PlanetChildLocationDeferral>,
    pub errors: Vec<PlanetChildLocationAdmissionError>,
}

// ---------------------------------------------------------------------------
// Local grid metadata helpers
// ---------------------------------------------------------------------------

pub fn local_gridcell_role(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, LOCAL_GRIDCELL_ROLE_PROPERTY_ID)
        .or_else(|| scenario_metadata_string(thing, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID))
}

/// Deprecated alias for [`local_gridcell_role`].
pub fn child_location_role(thing: &SimThing) -> Option<String> {
    local_gridcell_role(thing)
}

pub fn local_gridcell_col(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, LOCAL_GRIDCELL_COL_PROPERTY_ID)
}

pub fn local_gridcell_row(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, LOCAL_GRIDCELL_ROW_PROPERTY_ID)
}

pub fn star_system_local_grid_frame(gridcell: &SimThing) -> (u32, u32) {
    let cols = scenario_metadata_u32(gridcell, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID)
        .unwrap_or(STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS);
    let rows = scenario_metadata_u32(gridcell, STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID)
        .unwrap_or(STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS);
    (cols, rows)
}

pub fn apply_star_system_local_grid_frame_metadata(gridcell: &mut SimThing, cols: u32, rows: u32) {
    debug_assert_eq!(gridcell.kind, SimThingKind::Location);
    gridcell.add_property(
        STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
        structural_property_value_u32(cols),
    );
    gridcell.add_property(
        STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID,
        structural_property_value_u32(rows),
    );
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

pub fn is_planet_gridcell(thing: &SimThing) -> bool {
    thing.kind == SimThingKind::Location
        && local_gridcell_role(thing).as_deref() == Some(LOCAL_GRIDCELL_ROLE_PLANET)
}

/// Deprecated alias for [`is_planet_gridcell`].
pub fn is_planet_child_location(thing: &SimThing) -> bool {
    is_planet_gridcell(thing)
}

pub fn is_local_gridcell(thing: &SimThing) -> bool {
    if thing.kind != SimThingKind::Location {
        return false;
    }
    matches!(
        local_gridcell_role(thing).as_deref(),
        Some(LOCAL_GRIDCELL_ROLE_PLANET) | Some(LOCAL_GRIDCELL_ROLE_INERT)
    )
}

pub fn apply_local_gridcell_metadata(cell: &mut SimThing, role: &str, col: u32, row: u32) {
    debug_assert_eq!(cell.kind, SimThingKind::Location);
    cell.add_property(
        LOCAL_GRIDCELL_ROLE_PROPERTY_ID,
        scenario_metadata_string_value(role),
    );
    cell.add_property(
        LOCAL_GRIDCELL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        LOCAL_GRIDCELL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );
}

pub fn apply_planet_gridcell_metadata(
    planet: &mut SimThing,
    planet_id: &str,
    col: u32,
    row: u32,
    display_name: Option<&str>,
) {
    apply_local_gridcell_metadata(planet, LOCAL_GRIDCELL_ROLE_PLANET, col, row);
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

/// Deprecated alias — prefer [`apply_planet_gridcell_metadata`].
pub fn apply_planet_child_metadata(
    planet: &mut SimThing,
    planet_id: &str,
    display_name: Option<&str>,
) {
    apply_planet_gridcell_metadata(planet, planet_id, 0, 0, display_name);
}

pub fn make_planet_gridcell(
    planet_id: &str,
    col: u32,
    row: u32,
    display_name: Option<&str>,
) -> SimThing {
    let mut planet = SimThing::new(SimThingKind::Location, 0);
    apply_planet_gridcell_metadata(&mut planet, planet_id, col, row, display_name);
    planet
}

/// Deprecated alias for [`make_planet_gridcell`] at local (0, 0).
pub fn make_planet_child_location(planet_id: &str, display_name: Option<&str>) -> SimThing {
    make_planet_gridcell(planet_id, 0, 0, display_name)
}

pub fn make_local_inert_gridcell(col: u32, row: u32) -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_local_gridcell_metadata(&mut cell, LOCAL_GRIDCELL_ROLE_INERT, col, row);
    cell
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

/// Planet local gridcells under one star-system gridcell.
pub fn planet_gridcells<'a>(
    spec: &'a SimThingScenarioSpec,
    star_system: &'a SimThing,
) -> Vec<&'a SimThing> {
    let _ = spec;
    star_system
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Location && is_planet_gridcell(child))
        .collect()
}

/// Deprecated alias for [`planet_gridcells`].
pub fn planet_child_locations<'a>(
    spec: &'a SimThingScenarioSpec,
    gridcell: &'a SimThing,
) -> Vec<&'a SimThing> {
    planet_gridcells(spec, gridcell)
}

pub fn all_planet_gridcells(spec: &SimThingScenarioSpec) -> Vec<&SimThing> {
    let Ok(galaxy_map) = game_session_galaxy_map(spec) else {
        return Vec::new();
    };
    galaxy_map
        .children
        .iter()
        .filter(|c| {
            c.kind == SimThingKind::Location
                && !is_galaxy_map_entity(c)
                && gridcell_role(c).as_deref() == Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
        })
        .flat_map(|gridcell| planet_gridcells(spec, gridcell))
        .collect()
}

/// Deprecated alias for [`all_planet_gridcells`].
pub fn all_planet_child_locations(spec: &SimThingScenarioSpec) -> Vec<&SimThing> {
    all_planet_gridcells(spec)
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
                for child in &gridcell.children {
                    if child.kind == SimThingKind::Location
                        && local_gridcell_role(child).as_deref() == Some(LOCAL_GRIDCELL_ROLE_PLANET)
                    {
                        push_error(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::PlanetUnderInertGalaxyGridcell,
                            Some(format!("gridcell/inert/local:{}", child.id.raw())),
                            Some(child.id.raw()),
                            "planet local gridcell cannot live under inert galactic gridcell",
                        );
                    } else if child.kind == SimThingKind::Location
                        || matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet")
                    {
                        push_error(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::PlanetUnderInertGalaxyGridcell,
                            Some(format!("gridcell/inert/child:{}", child.id.raw())),
                            Some(child.id.raw()),
                            "local gridcell cannot live under inert galactic gridcell",
                        );
                    }
                }
            }
            Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => {
                report.star_system_gridcell_count += 1;
                let (frame_cols, frame_rows) = star_system_local_grid_frame(gridcell);
                if frame_cols == 0 || frame_rows == 0 {
                    push_error(
                        &mut report,
                        PlanetChildLocationAdmissionErrorKind::LocalGridFrameInvalid,
                        Some(format!("gridcell/{}", gridcell.id.raw())),
                        Some(gridcell.id.raw()),
                        "star-system local grid frame must be non-zero",
                    );
                    continue;
                }
                let mut seen_local_coords = BTreeSet::new();
                for child in &gridcell.children {
                    if child.kind == SimThingKind::Location {
                        evaluate_star_system_local_child(
                            gridcell,
                            child,
                            frame_cols,
                            frame_rows,
                            &placement_ids,
                            &mut seen_planet_ids,
                            &mut seen_local_coords,
                            &mut report,
                        );
                    } else if matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet")
                    {
                        report.unsupported_child_location_count += 1;
                        push_deferral(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                            Some(format!(
                                "gridcell/{}/legacy_planet:{}",
                                gridcell.id.raw(),
                                child.id.raw()
                            )),
                            Some(child.id.raw()),
                            "legacy Custom Planet child is not admitted; use local gridcell Location + planet role",
                        );
                    }
                }
            }
            _ => {
                for child in location_children(gridcell) {
                    if local_gridcell_role(child).as_deref() == Some(LOCAL_GRIDCELL_ROLE_PLANET) {
                        push_error(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                            Some(format!("gridcell/unknown_role/planet:{}", child.id.raw())),
                            Some(child.id.raw()),
                            "planet local gridcell under non-star-system galactic gridcell is not admitted",
                        );
                    } else {
                        push_error(
                            &mut report,
                            PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                            Some(format!("gridcell/unknown_role/child:{}", child.id.raw())),
                            Some(child.id.raw()),
                            "local gridcell under non-star-system galactic gridcell is not admitted",
                        );
                    }
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
    let report = evaluate_planet_child_locations(spec);
    if let Some(err) = report.errors.first().cloned() {
        return Err(err);
    }
    Ok(report)
}

fn location_children(gridcell: &SimThing) -> Vec<&SimThing> {
    gridcell
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location)
        .collect()
}

fn evaluate_star_system_local_child(
    gridcell: &SimThing,
    child: &SimThing,
    frame_cols: u32,
    frame_rows: u32,
    placement_ids: &BTreeSet<u32>,
    seen_planet_ids: &mut BTreeSet<String>,
    seen_local_coords: &mut BTreeSet<(u32, u32)>,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    let path_prefix = format!("gridcell/{}/local:{}", gridcell.id.raw(), child.id.raw());

    if placement_ids.contains(&child.id.raw()) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetListedInGalaxyStructuralGrid,
            Some(path_prefix.clone()),
            Some(child.id.raw()),
            "local gridcell must not appear in GalaxyMap structural_grid placements",
        );
        return;
    }

    let child_role = local_gridcell_role(child);
    match child_role.as_deref() {
        Some(LOCAL_GRIDCELL_ROLE_PLANET) => evaluate_planet_local_gridcell(
            child,
            frame_cols,
            frame_rows,
            &path_prefix,
            seen_planet_ids,
            seen_local_coords,
            report,
        ),
        Some(LOCAL_GRIDCELL_ROLE_INERT) => {
            if let Some((col, row)) = validate_local_coordinates(
                child,
                frame_cols,
                frame_rows,
                &path_prefix,
                report,
                true,
            ) {
                if !seen_local_coords.insert((col, row)) {
                    push_error(
                        report,
                        PlanetChildLocationAdmissionErrorKind::PlanetLocalGridDuplicateCoordinate,
                        Some(path_prefix),
                        Some(child.id.raw()),
                        format!("duplicate local coordinate ({col},{row}) within star system"),
                    );
                    return;
                }
                report.local_gridcell_count += 1;
                report.local_inert_gridcell_count += 1;
            }
        }
        Some(role) => {
            report.unsupported_child_location_count += 1;
            report.local_gridcell_count += 1;
            if validate_local_coordinates(child, frame_cols, frame_rows, &path_prefix, report, true)
                .is_some()
            {
                let coord = (
                    local_gridcell_col(child).unwrap_or(0),
                    local_gridcell_row(child).unwrap_or(0),
                );
                seen_local_coords.insert(coord);
            }
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix),
                Some(child.id.raw()),
                format!("local gridcell role `{role}` is not yet admitted"),
            );
        }
        None => {
            report.unsupported_child_location_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix),
                Some(child.id.raw()),
                "Location child under star-system gridcell lacks local gridcell role metadata",
            );
        }
    }
}

fn evaluate_planet_local_gridcell(
    child: &SimThing,
    frame_cols: u32,
    frame_rows: u32,
    path_prefix: &str,
    seen_planet_ids: &mut BTreeSet<String>,
    seen_local_coords: &mut BTreeSet<(u32, u32)>,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    let Some((col, row)) =
        validate_local_coordinates(child, frame_cols, frame_rows, path_prefix, report, true)
    else {
        return;
    };

    if !seen_local_coords.insert((col, row)) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetLocalGridDuplicateCoordinate,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            format!("duplicate local coordinate ({col},{row}) within star system"),
        );
        return;
    }

    let Some(id) = planet_id(child) else {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetGridcellMissingId,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet local gridcell is missing planet_id metadata",
        );
        return;
    };
    if id.trim().is_empty() {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetGridcellMissingId,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet_id must be non-empty",
        );
        return;
    }
    if !seen_planet_ids.insert(id.clone()) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::DuplicatePlanetIdWithinScenario,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            format!("duplicate planet_id `{id}` within scenario"),
        );
        return;
    }

    report.local_gridcell_count += 1;
    report.planet_gridcell_count += 1;

    if has_deeper_location_nesting(child) {
        push_deferral(
            report,
            PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "deeper Location nesting below planet local gridcell is deferred",
        );
    }
    if planet_owner_ref(child).is_some() {
        push_deferral(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet ownership reference present; resolution deferred",
        );
    }
    push_deferral(
        report,
        PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred,
        Some(path_prefix.to_string()),
        Some(child.id.raw()),
        "planet simulation/economy/population behavior remains deferred",
    );
}

fn validate_local_coordinates(
    child: &SimThing,
    frame_cols: u32,
    frame_rows: u32,
    path_prefix: &str,
    report: &mut PlanetChildLocationAdmissionReport,
    required: bool,
) -> Option<(u32, u32)> {
    let col = local_gridcell_col(child);
    let row = local_gridcell_row(child);
    match (col, row) {
        (Some(col), Some(row)) => {
            if col >= frame_cols || row >= frame_rows {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetLocalGridCoordinateOutOfFrame,
                    Some(path_prefix.to_string()),
                    Some(child.id.raw()),
                    format!(
                        "local coordinate ({col},{row}) outside star-system frame ({frame_cols}x{frame_rows})"
                    ),
                );
                None
            } else {
                Some((col, row))
            }
        }
        _ if required => {
            push_error(
                report,
                PlanetChildLocationAdmissionErrorKind::PlanetLocalGridMissingCoordinate,
                Some(path_prefix.to_string()),
                Some(child.id.raw()),
                "local gridcell is missing local col/row metadata",
            );
            None
        }
        _ => None,
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
    if report.planet_gridcell_count == 0 && report.unsupported_child_location_count > 0 {
        report.classification = PlanetChildLocationAdmissionClassification::Unsupported;
        return;
    }
    let hard_deferrals = report.deferrals.iter().filter(|d| {
        !matches!(
            d.kind,
            PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred
                | PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred
                | PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred
        )
    });
    if hard_deferrals.count() > 0 {
        report.classification = PlanetChildLocationAdmissionClassification::PartiallyAdmitted;
        return;
    }
    if report.deferrals.is_empty()
        && report.planet_gridcell_count == 0
        && report.local_gridcell_count == 0
    {
        report.classification = PlanetChildLocationAdmissionClassification::Admitted;
        return;
    }
    if report.planet_gridcell_count > 0 || report.local_gridcell_count > 0 {
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
        PlanetChildLocationAdmissionErrorKind::PlanetUnderInertGalaxyGridcell => {
            "PlanetUnderInertGalaxyGridcell"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetGridcellMissingId => "PlanetGridcellMissingId",
        PlanetChildLocationAdmissionErrorKind::DuplicatePlanetIdWithinScenario => {
            "DuplicatePlanetIdWithinScenario"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetListedInGalaxyStructuralGrid => {
            "PlanetListedInGalaxyStructuralGrid"
        }
        PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole => {
            "UnsupportedChildLocationRole"
        }
        PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred => "DeepPlanetChildDeferred",
        PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred => {
            "PlanetOwnershipResolutionDeferred"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred => {
            "PlanetSimulationDeferred"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetLocalGridMissingCoordinate => {
            "PlanetLocalGridMissingCoordinate"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetLocalGridDuplicateCoordinate => {
            "PlanetLocalGridDuplicateCoordinate"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetLocalGridCoordinateOutOfFrame => {
            "PlanetLocalGridCoordinateOutOfFrame"
        }
        PlanetChildLocationAdmissionErrorKind::LocalGridFrameInvalid => "LocalGridFrameInvalid",
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
// Local grid edit commands (separate from GalaxyMap structural placement commands)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalGridcellRoleEdit {
    Inert,
    Planet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanetLocalGridCommand {
    AddLocalGridcell {
        star_system_gridcell_id: String,
        local_gridcell_id: String,
        col: u32,
        row: u32,
        role: LocalGridcellRoleEdit,
    },
    AddPlanetGridcell {
        star_system_gridcell_id: String,
        planet_gridcell_id: String,
        planet_id: String,
        col: u32,
        row: u32,
        display_name: Option<String>,
    },
    MoveLocalGridcell {
        star_system_gridcell_id: String,
        local_gridcell_id: String,
        new_col: u32,
        new_row: u32,
    },
    RemoveLocalGridcell {
        star_system_gridcell_id: String,
        local_gridcell_id: String,
    },
    SetPlanetDisplayName {
        planet_gridcell_id: String,
        display_name: String,
    },
}

/// Deprecated alias — "planet child location" means local gridcell under star-system grid.
pub type PlanetChildLocationCommand = PlanetLocalGridCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetChildLocationEditErrorKind {
    MissingGalaxyMap,
    GridcellNotFound,
    GridcellNotStarSystem,
    PlanetUnderInertGridcell,
    DuplicatePlanetId,
    DuplicateLocalCoordinate,
    CoordinateOutOfFrame,
    PlanetNotFound,
    LocalGridcellNotFound,
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

pub fn apply_planet_local_grid_command(
    spec: &mut SimThingScenarioSpec,
    command: PlanetLocalGridCommand,
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
                        .unwrap_or_else(|| "planet local-grid validation failed".into()),
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

/// Deprecated alias for [`apply_planet_local_grid_command`].
pub fn apply_planet_child_location_command(
    spec: &mut SimThingScenarioSpec,
    command: PlanetChildLocationCommand,
) -> Result<PlanetChildLocationEditReport, PlanetChildLocationEditError> {
    apply_planet_local_grid_command(spec, command)
}

fn apply_command_to_draft(
    spec: &mut SimThingScenarioSpec,
    command: &PlanetLocalGridCommand,
) -> Result<(), PlanetChildLocationEditError> {
    match command {
        PlanetLocalGridCommand::AddLocalGridcell {
            star_system_gridcell_id,
            local_gridcell_id: _,
            col,
            row,
            role,
        } => {
            let role_str = match role {
                LocalGridcellRoleEdit::Inert => LOCAL_GRIDCELL_ROLE_INERT,
                LocalGridcellRoleEdit::Planet => LOCAL_GRIDCELL_ROLE_PLANET,
            };
            if *role == LocalGridcellRoleEdit::Planet {
                return Err(edit_error(
                    PlanetChildLocationEditErrorKind::ValidationFailed,
                    "use AddPlanetGridcell for planet local gridcells",
                ));
            }
            add_local_gridcell(spec, star_system_gridcell_id, *col, *row, role_str, None)
        }
        PlanetLocalGridCommand::AddPlanetGridcell {
            star_system_gridcell_id,
            planet_gridcell_id: _,
            planet_id,
            col,
            row,
            display_name,
        } => add_planet_gridcell(
            spec,
            star_system_gridcell_id,
            planet_id,
            *col,
            *row,
            display_name.as_deref(),
        ),
        PlanetLocalGridCommand::MoveLocalGridcell {
            star_system_gridcell_id,
            local_gridcell_id,
            new_col,
            new_row,
        } => move_local_gridcell(
            spec,
            star_system_gridcell_id,
            local_gridcell_id,
            *new_col,
            *new_row,
        ),
        PlanetLocalGridCommand::RemoveLocalGridcell {
            star_system_gridcell_id,
            local_gridcell_id,
        } => remove_local_gridcell(spec, star_system_gridcell_id, local_gridcell_id),
        PlanetLocalGridCommand::SetPlanetDisplayName {
            planet_gridcell_id,
            display_name,
        } => set_planet_display_name_by_gridcell_id(spec, planet_gridcell_id, display_name),
    }
}

fn add_local_gridcell(
    spec: &mut SimThingScenarioSpec,
    gridcell_id: &str,
    col: u32,
    row: u32,
    role: &str,
    planet_id_opt: Option<(&str, Option<&str>)>,
) -> Result<(), PlanetChildLocationEditError> {
    let gridcell_raw = resolve_star_system_gridcell_raw(spec, gridcell_id)?;
    ensure_coordinate_in_frame(spec, gridcell_raw, col, row)?;
    ensure_coordinate_available(spec, gridcell_raw, col, row, None)?;

    let new_child = if let Some((pid, display)) = planet_id_opt {
        let trimmed = pid.trim();
        if trimmed.is_empty() {
            return Err(edit_error(
                PlanetChildLocationEditErrorKind::ValidationFailed,
                "planet_id must be non-empty",
            ));
        }
        if all_planet_gridcells(spec)
            .iter()
            .any(|p| planet_id(p).as_deref() == Some(trimmed))
        {
            return Err(edit_error(
                PlanetChildLocationEditErrorKind::DuplicatePlanetId,
                format!("planet_id `{trimmed}` already exists"),
            ));
        }
        make_planet_gridcell(trimmed, col, row, display)
    } else {
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        apply_local_gridcell_metadata(&mut cell, role, col, row);
        cell
    };

    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    gridcell.add_child(new_child);
    Ok(())
}

fn add_planet_gridcell(
    spec: &mut SimThingScenarioSpec,
    gridcell_id: &str,
    target_planet_id: &str,
    col: u32,
    row: u32,
    display_name: Option<&str>,
) -> Result<(), PlanetChildLocationEditError> {
    add_local_gridcell(
        spec,
        gridcell_id,
        col,
        row,
        LOCAL_GRIDCELL_ROLE_PLANET,
        Some((target_planet_id, display_name)),
    )
}

fn move_local_gridcell(
    spec: &mut SimThingScenarioSpec,
    gridcell_id: &str,
    local_gridcell_id: &str,
    new_col: u32,
    new_row: u32,
) -> Result<(), PlanetChildLocationEditError> {
    let gridcell_raw = resolve_star_system_gridcell_raw(spec, gridcell_id)?;
    ensure_coordinate_in_frame(spec, gridcell_raw, new_col, new_row)?;
    ensure_coordinate_available(
        spec,
        gridcell_raw,
        new_col,
        new_row,
        Some(local_gridcell_id),
    )?;

    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    let child_raw = resolve_local_gridcell_raw(gridcell, local_gridcell_id)?;
    let child = gridcell
        .children
        .iter_mut()
        .find(|c| c.id.raw() == child_raw)
        .expect("local gridcell");
    child.add_property(
        LOCAL_GRIDCELL_COL_PROPERTY_ID,
        structural_property_value_u32(new_col),
    );
    child.add_property(
        LOCAL_GRIDCELL_ROW_PROPERTY_ID,
        structural_property_value_u32(new_row),
    );
    Ok(())
}

fn remove_local_gridcell(
    spec: &mut SimThingScenarioSpec,
    gridcell_id: &str,
    local_gridcell_id: &str,
) -> Result<(), PlanetChildLocationEditError> {
    let gridcell_raw = resolve_star_system_gridcell_raw(spec, gridcell_id)?;
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    let child_raw = resolve_local_gridcell_raw(gridcell, local_gridcell_id)?;
    let idx = gridcell
        .children
        .iter()
        .position(|c| c.id.raw() == child_raw)
        .ok_or_else(|| {
            edit_error(
                PlanetChildLocationEditErrorKind::LocalGridcellNotFound,
                format!("local gridcell `{local_gridcell_id}` not found"),
            )
        })?;
    gridcell.children.remove(idx);
    Ok(())
}

fn set_planet_display_name_by_gridcell_id(
    spec: &mut SimThingScenarioSpec,
    planet_gridcell_id: &str,
    display_name: &str,
) -> Result<(), PlanetChildLocationEditError> {
    let (gridcell_raw, child_raw) = locate_planet_gridcell(spec, planet_gridcell_id)?;
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let gridcell = galaxy_map
        .children
        .iter_mut()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    let planet = gridcell
        .children
        .iter_mut()
        .find(|c| c.id.raw() == child_raw)
        .expect("planet gridcell");
    planet.add_property(
        PLANET_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
    Ok(())
}

fn locate_planet_gridcell(
    spec: &SimThingScenarioSpec,
    planet_gridcell_id: &str,
) -> Result<(u32, u32), PlanetChildLocationEditError> {
    let trimmed = planet_gridcell_id.trim();
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
            if is_planet_gridcell(child)
                && (child.id.raw().to_string() == trimmed
                    || planet_id(child).as_deref() == Some(trimmed))
            {
                return Ok((gridcell.id.raw(), child.id.raw()));
            }
        }
    }
    Err(edit_error(
        PlanetChildLocationEditErrorKind::PlanetNotFound,
        format!("planet gridcell `{trimmed}` not found"),
    ))
}

fn resolve_star_system_gridcell_raw(
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
        let galaxy_map = game_session_galaxy_map(spec).map_err(|_| {
            edit_error(
                PlanetChildLocationEditErrorKind::MissingGalaxyMap,
                "GalaxyMap child missing",
            )
        })?;
        let gridcell = galaxy_map
            .children
            .iter()
            .find(|c| c.id.raw() == placement.simthing_id_raw)
            .ok_or_else(|| {
                edit_error(
                    PlanetChildLocationEditErrorKind::GridcellNotFound,
                    format!("gridcell `{trimmed}` not found under GalaxyMap"),
                )
            })?;
        return validate_star_system_parent(gridcell);
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
            return validate_star_system_parent(child);
        }
    }
    Err(edit_error(
        PlanetChildLocationEditErrorKind::GridcellNotFound,
        format!("gridcell `{trimmed}` not found"),
    ))
}

fn validate_star_system_parent(gridcell: &SimThing) -> Result<u32, PlanetChildLocationEditError> {
    match gridcell_role(gridcell).as_deref() {
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => Ok(gridcell.id.raw()),
        Some(GALAXY_GRIDCELL_ROLE_INERT) => Err(edit_error(
            PlanetChildLocationEditErrorKind::PlanetUnderInertGridcell,
            "cannot add local gridcell under inert galactic gridcell",
        )),
        _ => Err(edit_error(
            PlanetChildLocationEditErrorKind::GridcellNotStarSystem,
            "local gridcells require star-system galactic gridcell parent",
        )),
    }
}

fn resolve_local_gridcell_raw(
    gridcell: &SimThing,
    local_gridcell_id: &str,
) -> Result<u32, PlanetChildLocationEditError> {
    let trimmed = local_gridcell_id.trim();
    for child in &gridcell.children {
        if child.kind == SimThingKind::Location
            && (child.id.raw().to_string() == trimmed
                || planet_id(child).as_deref() == Some(trimmed))
        {
            return Ok(child.id.raw());
        }
    }
    Err(edit_error(
        PlanetChildLocationEditErrorKind::LocalGridcellNotFound,
        format!("local gridcell `{trimmed}` not found"),
    ))
}

fn ensure_coordinate_in_frame(
    spec: &SimThingScenarioSpec,
    gridcell_raw: u32,
    col: u32,
    row: u32,
) -> Result<(), PlanetChildLocationEditError> {
    let galaxy_map = game_session_galaxy_map(spec).map_err(|_| {
        edit_error(
            PlanetChildLocationEditErrorKind::MissingGalaxyMap,
            "GalaxyMap child missing",
        )
    })?;
    let gridcell = galaxy_map
        .children
        .iter()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    let (frame_cols, frame_rows) = star_system_local_grid_frame(gridcell);
    if col >= frame_cols || row >= frame_rows {
        return Err(edit_error(
            PlanetChildLocationEditErrorKind::CoordinateOutOfFrame,
            format!(
                "local coordinate ({col},{row}) outside star-system frame ({frame_cols}x{frame_rows})"
            ),
        ));
    }
    Ok(())
}

fn ensure_coordinate_available(
    spec: &SimThingScenarioSpec,
    gridcell_raw: u32,
    col: u32,
    row: u32,
    exclude_id: Option<&str>,
) -> Result<(), PlanetChildLocationEditError> {
    let galaxy_map = game_session_galaxy_map(spec).map_err(|_| {
        edit_error(
            PlanetChildLocationEditErrorKind::MissingGalaxyMap,
            "GalaxyMap child missing",
        )
    })?;
    let gridcell = galaxy_map
        .children
        .iter()
        .find(|c| c.id.raw() == gridcell_raw)
        .expect("star-system gridcell");
    for child in &gridcell.children {
        if child.kind != SimThingKind::Location {
            continue;
        }
        if let Some(ex) = exclude_id {
            if child.id.raw().to_string() == ex || planet_id(child).as_deref() == Some(ex) {
                continue;
            }
        }
        if local_gridcell_col(child) == Some(col) && local_gridcell_row(child) == Some(row) {
            return Err(edit_error(
                PlanetChildLocationEditErrorKind::DuplicateLocalCoordinate,
                format!("local coordinate ({col},{row}) already occupied"),
            ));
        }
    }
    Ok(())
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
