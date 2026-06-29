//! RECURSIVE-SPATIAL-GRID-DEFAULTS-0 — recursive spatial local-grid admission.
//! PLANET-NON-GRID-CHILD-ADMISSION-0 — cohort/fleet/infrastructure/leader under planet gridcells.
//!
//! Owner SimThings are GameSession children and RF channel scopes, not spatial parents.
//! Ownership changes update metadata/properties, not spatial parentage.

use std::collections::BTreeSet;

use simthing_core::{SimThing, SimThingKind};

use super::channel_key::OwnerRef;
use super::scenario::{
    game_session_child_mut, game_session_galaxy_map, gridcell_role, is_galaxy_map_entity,
    scenario_metadata_string, scenario_metadata_string_value, structural_property_value_u32,
    SimThingScenarioSpec, GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, LOCAL_GRIDCELL_COL_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_INERT,
    LOCAL_GRIDCELL_ROLE_PLANET, LOCAL_GRIDCELL_ROLE_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_RECEIVER,
    LOCAL_GRIDCELL_ROLE_SURFACE, LOCAL_GRIDCELL_ROW_PROPERTY_ID, LOCAL_GRID_DEFAULT_COLS,
    LOCAL_GRID_DEFAULT_ROWS, PLANET_DISPLAY_NAME_PROPERTY_ID, PLANET_ID_PROPERTY_ID,
    PLANET_OWNER_REF_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID,
};
use super::spatial_local_grid::{
    default_local_grid_frame_for_spatial_gridcell, implicit_receiver_cell_for_gridcell,
    interior_local_grid_frame_for_gridcell, is_receiver_local_gridcell,
    local_grid_frame_for_spatial_gridcell, materialized_receiver_cell, LocalGridFrame,
    LocalGridFrameErrorKind, LocalReceiverCell,
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
    InertGridcellNonReceiverChild,
    InertGridcellReceiverCoordinateOutOfFrame,
    InertGridcellExpandedFrameUnsupported,
    PlanetGridcellMissingId,
    DuplicatePlanetIdWithinScenario,
    PlanetListedInGalaxyStructuralGrid,
    UnsupportedChildLocationRole,
    DeepPlanetChildDeferred,
    PlanetDirectGameplayChildRequiresSurfaceGridcell,
    PlanetSurfaceGridcellMissing,
    PlanetSurfaceGridcellDuplicate,
    PlanetOwnershipResolutionDeferred,
    PlanetSimulationDeferred,
    PlanetLocalGridMissingCoordinate,
    PlanetLocalGridDuplicateCoordinate,
    PlanetLocalGridCoordinateOutOfFrame,
    LocalGridFrameInvalid,
    PlanetNonGridChildHasLocalCoordinate,
    PlanetNonGridChildUnsupportedKind,
    PlanetNonGridChildSimulationDeferred,
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
    pub inert_galactic_gridcell_count: u32,
    pub local_gridcell_count: u32,
    pub local_inert_gridcell_count: u32,
    pub planet_gridcell_count: u32,
    pub surface_gridcell_count: u32,
    pub planet_surface_gridcell_count: u32,
    pub gameplay_child_under_surface_count: u32,
    pub direct_gameplay_child_under_planet_count: u32,
    pub surface_gridcell_tier_present: bool,
    pub surface_gridcell_tier_required: bool,
    pub planet_non_grid_child_count: u32,
    pub receiver_cell_count: u32,
    pub implicit_receiver_cell_count: u32,
    pub unsupported_child_location_count: u32,
    pub classification: PlanetChildLocationAdmissionClassification,
    pub deferrals: Vec<PlanetChildLocationDeferral>,
    pub errors: Vec<PlanetChildLocationAdmissionError>,
}

// ---------------------------------------------------------------------------
// Local grid metadata helpers
// ---------------------------------------------------------------------------

pub fn local_gridcell_role(thing: &SimThing) -> Option<String> {
    super::spatial_local_grid::local_gridcell_role(thing)
}

/// Deprecated alias for [`local_gridcell_role`].
pub fn child_location_role(thing: &SimThing) -> Option<String> {
    local_gridcell_role(thing)
}

pub fn local_gridcell_col(thing: &SimThing) -> Option<u32> {
    super::spatial_local_grid::local_gridcell_col(thing)
}

pub fn local_gridcell_row(thing: &SimThing) -> Option<u32> {
    super::spatial_local_grid::local_gridcell_row(thing)
}

pub fn star_system_local_grid_frame(gridcell: &SimThing) -> (u32, u32) {
    let frame = local_grid_frame_for_spatial_gridcell(gridcell)
        .unwrap_or_else(|_| default_local_grid_frame_for_spatial_gridcell(gridcell));
    (frame.cols, frame.rows)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetNonGridChildEntry {
    pub planet_gridcell_id_raw: u32,
    pub planet_id: String,
    pub child_simthing_id_raw: u32,
    pub child_kind_label: String,
    pub owner_ref: Option<OwnerRef>,
}

pub fn planet_non_grid_child_kind_label(kind: &SimThingKind) -> String {
    match kind {
        SimThingKind::Cohort => "cohort".into(),
        SimThingKind::Fleet => "fleet".into(),
        SimThingKind::Station => "station".into(),
        SimThingKind::Custom(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

pub fn is_admitted_planet_non_grid_child(kind: &SimThingKind) -> bool {
    match kind {
        SimThingKind::Cohort | SimThingKind::Fleet | SimThingKind::Station => true,
        SimThingKind::Custom(name) => matches!(
            name.as_str(),
            "Infrastructure" | "Leader" | "infrastructure" | "leader"
        ),
        _ => false,
    }
}

pub fn planet_non_grid_child_owner_ref(child: &SimThing) -> Option<String> {
    scenario_metadata_string(child, PLANET_OWNER_REF_PROPERTY_ID).or_else(|| {
        scenario_metadata_string(child, super::scenario::OWNER_FLOW_OWNER_REF_PROPERTY_ID)
    })
}

pub fn collect_planet_non_grid_children(
    spec: &SimThingScenarioSpec,
) -> Vec<PlanetNonGridChildEntry> {
    let mut entries = Vec::new();
    for planet in all_planet_gridcells(spec) {
        let Some(planet_id) = planet_id(planet) else {
            continue;
        };
        let Some(surface) = planet_surface_gridcell(planet) else {
            continue;
        };
        for child in &surface.children {
            if child.kind == SimThingKind::Location {
                continue;
            }
            if is_admitted_planet_non_grid_child(&child.kind) {
                entries.push(PlanetNonGridChildEntry {
                    planet_gridcell_id_raw: planet.id.raw(),
                    planet_id: planet_id.clone(),
                    child_simthing_id_raw: child.id.raw(),
                    child_kind_label: planet_non_grid_child_kind_label(&child.kind),
                    owner_ref: planet_non_grid_child_owner_ref(child).map(OwnerRef::new),
                });
            }
        }
    }
    entries
}

pub fn planet_gridcell_interior_frame(planet: &SimThing) -> LocalGridFrame {
    interior_local_grid_frame_for_gridcell(planet).unwrap_or(LocalGridFrame {
        cols: LOCAL_GRID_DEFAULT_COLS,
        rows: LOCAL_GRID_DEFAULT_ROWS,
    })
}

pub fn collect_local_receiver_cells(spec: &SimThingScenarioSpec) -> Vec<LocalReceiverCell> {
    let Ok(galaxy_map) = game_session_galaxy_map(spec) else {
        return Vec::new();
    };
    let mut cells = Vec::new();
    for gridcell in galaxy_map
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location && !is_galaxy_map_entity(c))
    {
        if gridcell_role(gridcell).as_deref() != Some(GALAXY_GRIDCELL_ROLE_INERT) {
            continue;
        }
        let mut materialized = false;
        for child in &gridcell.children {
            if child.kind == SimThingKind::Location && is_receiver_local_gridcell(child) {
                cells.push(materialized_receiver_cell(child, gridcell.id.raw()));
                materialized = true;
            }
        }
        if !materialized {
            cells.push(implicit_receiver_cell_for_gridcell(gridcell));
        }
    }
    cells
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

pub fn is_surface_gridcell(thing: &SimThing) -> bool {
    thing.kind == SimThingKind::Location
        && local_gridcell_role(thing).as_deref() == Some(LOCAL_GRIDCELL_ROLE_SURFACE)
}

pub fn planet_surface_gridcell<'a>(planet: &'a SimThing) -> Option<&'a SimThing> {
    if !is_planet_gridcell(planet) {
        return None;
    }
    planet
        .children
        .iter()
        .find(|child| is_surface_gridcell(child))
}

pub fn planet_gameplay_children<'a>(planet: &'a SimThing) -> Vec<&'a SimThing> {
    let Some(surface) = planet_surface_gridcell(planet) else {
        return Vec::new();
    };
    surface
        .children
        .iter()
        .filter(|child| {
            child.kind != SimThingKind::Location && is_admitted_planet_non_grid_child(&child.kind)
        })
        .collect()
}

pub fn make_surface_gridcell() -> SimThing {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_local_gridcell_metadata(&mut cell, LOCAL_GRIDCELL_ROLE_SURFACE, 0, 0);
    cell
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
        Some(LOCAL_GRIDCELL_ROLE_PLANET)
            | Some(LOCAL_GRIDCELL_ROLE_INERT)
            | Some(LOCAL_GRIDCELL_ROLE_SURFACE)
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
    planet.add_child(make_surface_gridcell());
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
    report.surface_gridcell_tier_required = true;
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
                evaluate_inert_galactic_gridcell(gridcell, &placement_ids, &mut report);
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

fn evaluate_inert_galactic_gridcell(
    gridcell: &SimThing,
    placement_ids: &BTreeSet<u32>,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    report.inert_galactic_gridcell_count += 1;
    let path_prefix = format!("gridcell/inert/{}", gridcell.id.raw());

    match local_grid_frame_for_spatial_gridcell(gridcell) {
        Err(err) => {
            let kind = match err.kind {
                LocalGridFrameErrorKind::FrameTooLarge => {
                    PlanetChildLocationAdmissionErrorKind::InertGridcellExpandedFrameUnsupported
                }
                LocalGridFrameErrorKind::FrameZeroDimension => {
                    PlanetChildLocationAdmissionErrorKind::LocalGridFrameInvalid
                }
            };
            push_error(
                report,
                kind,
                Some(path_prefix),
                Some(gridcell.id.raw()),
                err.message,
            );
            return;
        }
        Ok(frame) => {
            if frame.cols > LOCAL_GRID_DEFAULT_COLS || frame.rows > LOCAL_GRID_DEFAULT_ROWS {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::InertGridcellExpandedFrameUnsupported,
                    Some(path_prefix.clone()),
                    Some(gridcell.id.raw()),
                    "inert galactic gridcell cannot expand local frame beyond 1x1 yet",
                );
                return;
            }
        }
    }

    let mut materialized_receiver = false;
    for child in &gridcell.children {
        if child.kind != SimThingKind::Location {
            continue;
        }
        let child_path = format!("{path_prefix}/local:{}", child.id.raw());
        if placement_ids.contains(&child.id.raw()) {
            push_error(
                report,
                PlanetChildLocationAdmissionErrorKind::PlanetListedInGalaxyStructuralGrid,
                Some(child_path.clone()),
                Some(child.id.raw()),
                "local receiver cell must not appear in GalaxyMap structural_grid placements",
            );
            continue;
        }

        match local_gridcell_role(child).as_deref() {
            Some(LOCAL_GRIDCELL_ROLE_PLANET) => {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild,
                    Some(child_path),
                    Some(child.id.raw()),
                    "planet local gridcell cannot live under inert galactic gridcell",
                );
            }
            Some(LOCAL_GRIDCELL_ROLE_RECEIVER) | Some(LOCAL_GRIDCELL_ROLE_INERT) => {
                let col = local_gridcell_col(child).unwrap_or(0);
                let row = local_gridcell_row(child).unwrap_or(0);
                if col != 0 || row != 0 {
                    push_error(
                        report,
                        PlanetChildLocationAdmissionErrorKind::InertGridcellReceiverCoordinateOutOfFrame,
                        Some(child_path),
                        Some(child.id.raw()),
                        "inert receiver local gridcell must occupy (0,0) in 1x1 frame",
                    );
                    continue;
                }
                materialized_receiver = true;
                report.receiver_cell_count += 1;
                report.local_gridcell_count += 1;
                report.local_inert_gridcell_count += 1;
            }
            Some(_) => {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild,
                    Some(child_path),
                    Some(child.id.raw()),
                    "inert galactic gridcell admits only 1x1 receiver/inert local gridcells",
                );
                report.unsupported_child_location_count += 1;
            }
            None => {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild,
                    Some(child_path),
                    Some(child.id.raw()),
                    "Location child under inert gridcell lacks receiver/inert local role",
                );
                report.unsupported_child_location_count += 1;
            }
        }
    }

    if !materialized_receiver {
        report.implicit_receiver_cell_count += 1;
        report.receiver_cell_count += 1;
    }

    for child in &gridcell.children {
        if matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet") {
            report.unsupported_child_location_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(format!("{path_prefix}/legacy_planet:{}", child.id.raw())),
                Some(child.id.raw()),
                "legacy Custom Planet child is not admitted; use local gridcell Location + planet role",
            );
        }
    }
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
            placement_ids,
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
    placement_ids: &BTreeSet<u32>,
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

    let interior_frame = planet_gridcell_interior_frame(child);
    let mut surface_admitted = false;
    let mut seen_interior_coords = BTreeSet::new();

    for grandchild in &child.children {
        let grandchild_path = format!("{path_prefix}/interior:{}", grandchild.id.raw());
        if grandchild.kind == SimThingKind::Location {
            evaluate_planet_interior_location_child(
                grandchild,
                interior_frame,
                &grandchild_path,
                placement_ids,
                &mut seen_interior_coords,
                &mut surface_admitted,
                report,
            );
            continue;
        }
        if is_admitted_planet_non_grid_child(&grandchild.kind) {
            report.direct_gameplay_child_under_planet_count += 1;
            push_error(
                report,
                PlanetChildLocationAdmissionErrorKind::PlanetDirectGameplayChildRequiresSurfaceGridcell,
                Some(grandchild_path),
                Some(grandchild.id.raw()),
                "gameplay SimThings must be children of the planet surface gridcell, not the planet gridcell directly",
            );
            continue;
        }
        push_deferral(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildUnsupportedKind,
            Some(grandchild_path),
            Some(grandchild.id.raw()),
            format!(
                "direct child kind `{}` under planet gridcell is not yet admitted",
                planet_non_grid_child_kind_label(&grandchild.kind)
            ),
        );
    }

    report.surface_gridcell_tier_present = surface_admitted;
    if !surface_admitted {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetSurfaceGridcellMissing,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet gridcell requires exactly one 1x1 surface gridcell Location at (0,0)",
        );
    }
}

fn evaluate_planet_interior_location_child(
    child: &SimThing,
    interior_frame: LocalGridFrame,
    path_prefix: &str,
    placement_ids: &BTreeSet<u32>,
    seen_interior_coords: &mut BTreeSet<(u32, u32)>,
    surface_admitted: &mut bool,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    if placement_ids.contains(&child.id.raw()) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetListedInGalaxyStructuralGrid,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet interior local gridcell must not appear in GalaxyMap structural_grid placements",
        );
        return;
    }

    let Some((col, row)) = validate_local_coordinates(
        child,
        interior_frame.cols,
        interior_frame.rows,
        path_prefix,
        report,
        true,
    ) else {
        return;
    };

    if !seen_interior_coords.insert((col, row)) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetLocalGridDuplicateCoordinate,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            format!("duplicate planet interior coordinate ({col},{row})"),
        );
        return;
    }

    match local_gridcell_role(child).as_deref() {
        Some(LOCAL_GRIDCELL_ROLE_SURFACE) => {
            if col != 0 || row != 0 {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetLocalGridCoordinateOutOfFrame,
                    Some(path_prefix.to_string()),
                    Some(child.id.raw()),
                    "surface gridcell must occupy (0,0) in default 1x1 planet interior frame",
                );
                return;
            }
            if *surface_admitted {
                push_error(
                    report,
                    PlanetChildLocationAdmissionErrorKind::PlanetSurfaceGridcellDuplicate,
                    Some(path_prefix.to_string()),
                    Some(child.id.raw()),
                    "planet gridcell admits only one surface gridcell",
                );
                return;
            }
            *surface_admitted = true;
            report.surface_gridcell_count += 1;
            report.planet_surface_gridcell_count += 1;
            report.local_gridcell_count += 1;

            for gameplay_child in &child.children {
                let gameplay_path = format!("{path_prefix}/gameplay:{}", gameplay_child.id.raw());
                if gameplay_child.kind == SimThingKind::Location {
                    push_deferral(
                        report,
                        PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred,
                        Some(gameplay_path),
                        Some(gameplay_child.id.raw()),
                        "deeper Location nesting below surface gridcell is deferred",
                    );
                    continue;
                }
                evaluate_planet_non_grid_child(
                    gameplay_child,
                    &gameplay_path,
                    placement_ids,
                    report,
                );
                if is_admitted_planet_non_grid_child(&gameplay_child.kind) {
                    report.gameplay_child_under_surface_count += 1;
                }
            }
        }
        Some(LOCAL_GRIDCELL_ROLE_INERT) => {
            report.local_gridcell_count += 1;
            report.local_inert_gridcell_count += 1;
        }
        Some(role) => {
            report.unsupported_child_location_count += 1;
            report.local_gridcell_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix.to_string()),
                Some(child.id.raw()),
                format!("planet interior local gridcell role `{role}` is not yet admitted"),
            );
        }
        None => {
            report.unsupported_child_location_count += 1;
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole,
                Some(path_prefix.to_string()),
                Some(child.id.raw()),
                "Location child under planet gridcell lacks local gridcell role metadata",
            );
        }
    }
}

fn evaluate_planet_non_grid_child(
    child: &SimThing,
    path_prefix: &str,
    placement_ids: &BTreeSet<u32>,
    report: &mut PlanetChildLocationAdmissionReport,
) {
    if placement_ids.contains(&child.id.raw()) {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetListedInGalaxyStructuralGrid,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet non-grid child must not appear in GalaxyMap structural_grid placements",
        );
        return;
    }

    if local_gridcell_col(child).is_some() || local_gridcell_row(child).is_some() {
        push_error(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildHasLocalCoordinate,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            "planet non-grid child must not carry local gridcell col/row metadata",
        );
        return;
    }

    if is_admitted_planet_non_grid_child(&child.kind) {
        report.planet_non_grid_child_count += 1;
        if planet_non_grid_child_owner_ref(child).is_some() {
            push_deferral(
                report,
                PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred,
                Some(path_prefix.to_string()),
                Some(child.id.raw()),
                "planet non-grid child owner/channel metadata present; resolution deferred",
            );
        }
        push_deferral(
            report,
            PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildSimulationDeferred,
            Some(path_prefix.to_string()),
            Some(child.id.raw()),
            format!(
                "planet non-grid child `{}` admitted; simulation behavior remains deferred",
                planet_non_grid_child_kind_label(&child.kind)
            ),
        );
        return;
    }

    push_deferral(
        report,
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildUnsupportedKind,
        Some(path_prefix.to_string()),
        Some(child.id.raw()),
        format!(
            "planet non-grid child kind `{}` is not yet admitted",
            planet_non_grid_child_kind_label(&child.kind)
        ),
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
                | PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildSimulationDeferred
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
        PlanetChildLocationAdmissionErrorKind::InertGridcellNonReceiverChild => {
            "InertGridcellNonReceiverChild"
        }
        PlanetChildLocationAdmissionErrorKind::InertGridcellReceiverCoordinateOutOfFrame => {
            "InertGridcellReceiverCoordinateOutOfFrame"
        }
        PlanetChildLocationAdmissionErrorKind::InertGridcellExpandedFrameUnsupported => {
            "InertGridcellExpandedFrameUnsupported"
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
        PlanetChildLocationAdmissionErrorKind::PlanetDirectGameplayChildRequiresSurfaceGridcell => {
            "PlanetDirectGameplayChildRequiresSurfaceGridcell"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetSurfaceGridcellMissing => {
            "PlanetSurfaceGridcellMissing"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetSurfaceGridcellDuplicate => {
            "PlanetSurfaceGridcellDuplicate"
        }
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
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildHasLocalCoordinate => {
            "PlanetNonGridChildHasLocalCoordinate"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildUnsupportedKind => {
            "PlanetNonGridChildUnsupportedKind"
        }
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildSimulationDeferred => {
            "PlanetNonGridChildSimulationDeferred"
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
