//! STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — canonical gridcell placement edit commands.

use simthing_core::{SimThing, SimThingKind};

use super::scenario::{
    apply_gridcell_role_metadata, game_session_child, game_session_child_mut,
    game_session_galaxy_map, structural_property_value_u32, validate_scenario_links,
    validate_scenario_root_authority, validate_stead_mapping_consistency, ScenarioRootError,
    ScenarioRootValidationMode, SimThingScenarioSpec, SimThingStructuralGridPlacement,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_INTEGER_MAX, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridcellRoleEdit {
    Inert,
    StarSystem,
    /// Explicit unsupported role for ingestion/deferral testing only.
    UnknownUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralPlacementCommand {
    AddGridcell {
        id: String,
        col: u32,
        row: u32,
        role: GridcellRoleEdit,
    },
    MoveGridcell {
        id: String,
        new_col: u32,
        new_row: u32,
    },
    RemoveGridcell {
        id: String,
    },
    SetGridcellRole {
        id: String,
        role: GridcellRoleEdit,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuralPlacementEditErrorKind {
    NonCanonicalScenarioRoot,
    MissingGameSession,
    MissingGalaxyMap,
    DuplicateGridcellId,
    DuplicateCoordinate,
    GridcellNotFound,
    InvalidCoordinate,
    InvalidGridcellRole,
    GridcellNotUnderGalaxyMap,
    StaleStructuralPlacement,
    ValidationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralPlacementEditError {
    pub kind: StructuralPlacementEditErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralPlacementEditWarning {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructuralPlacementEditReport {
    pub command_count: u32,
    pub applied_count: u32,
    pub rejected_count: u32,
    pub warnings: Vec<StructuralPlacementEditWarning>,
    pub errors: Vec<StructuralPlacementEditError>,
}

impl std::fmt::Display for StructuralPlacementEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for StructuralPlacementEditError {}

/// Apply one structural placement command to canonical Scenario authority.
///
/// Rejected commands leave the scenario unchanged (draft validate-then-swap).
pub fn apply_structural_placement_command(
    spec: &mut SimThingScenarioSpec,
    command: StructuralPlacementCommand,
) -> Result<StructuralPlacementEditReport, StructuralPlacementEditError> {
    let mut draft = spec.clone();
    let mut report = StructuralPlacementEditReport {
        command_count: 1,
        ..Default::default()
    };
    match apply_command_to_draft(&mut draft, &command, &mut report) {
        Ok(()) => {
            validate_edited_scenario(&draft)?;
            *spec = draft;
            report.applied_count = 1;
            Ok(report)
        }
        Err(err) => {
            report.rejected_count = 1;
            report.errors.push(err.clone());
            Err(err)
        }
    }
}

fn validate_edited_scenario(
    spec: &SimThingScenarioSpec,
) -> Result<(), StructuralPlacementEditError> {
    validate_scenario_root_authority(spec, ScenarioRootValidationMode::Canonical).map_err(
        |err| {
            edit_error(
                StructuralPlacementEditErrorKind::ValidationFailed,
                err.to_string(),
            )
        },
    )?;
    validate_stead_mapping_consistency(spec).map_err(|err| {
        edit_error(
            StructuralPlacementEditErrorKind::ValidationFailed,
            err.to_string(),
        )
    })?;
    validate_scenario_links(spec).map_err(|err| {
        edit_error(
            StructuralPlacementEditErrorKind::ValidationFailed,
            err.to_string(),
        )
    })?;
    Ok(())
}

fn apply_command_to_draft(
    spec: &mut SimThingScenarioSpec,
    command: &StructuralPlacementCommand,
    report: &mut StructuralPlacementEditReport,
) -> Result<(), StructuralPlacementEditError> {
    ensure_canonical_edit_surface(spec)?;
    sync_map_container_binding(spec)?;
    match command {
        StructuralPlacementCommand::AddGridcell { id, col, row, role } => {
            add_gridcell(spec, id, *col, *row, *role)
        }
        StructuralPlacementCommand::MoveGridcell {
            id,
            new_col,
            new_row,
        } => move_gridcell(spec, id, *new_col, *new_row),
        StructuralPlacementCommand::RemoveGridcell { id } => remove_gridcell(spec, id, report),
        StructuralPlacementCommand::SetGridcellRole { id, role } => {
            set_gridcell_role(spec, id, *role)
        }
    }
}

fn ensure_canonical_edit_surface(
    spec: &SimThingScenarioSpec,
) -> Result<(), StructuralPlacementEditError> {
    if spec.root.kind != SimThingKind::Scenario {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::NonCanonicalScenarioRoot,
            format!(
                "structural edits require Scenario root, found {:?}",
                spec.root.kind
            ),
        ));
    }
    game_session_child(spec).map_err(map_root_error)?;
    game_session_galaxy_map(spec).map_err(map_root_error)?;
    Ok(())
}

fn sync_map_container_binding(
    spec: &mut SimThingScenarioSpec,
) -> Result<(), StructuralPlacementEditError> {
    let map_raw = game_session_galaxy_map(spec)
        .map_err(map_root_error)?
        .id
        .raw()
        .to_string();
    spec.structural_grid.map_container_id = map_raw;
    Ok(())
}

fn add_gridcell(
    spec: &mut SimThingScenarioSpec,
    id: &str,
    col: u32,
    row: u32,
    role: GridcellRoleEdit,
) -> Result<(), StructuralPlacementEditError> {
    let gridcell_id = normalize_gridcell_id(id)?;
    ensure_coordinate_in_frame(spec, col, row)?;
    if placement_by_id(spec, &gridcell_id).is_some() {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::DuplicateGridcellId,
            format!("gridcell id `{gridcell_id}` already exists"),
        ));
    }
    if placement_at_coordinate(spec, col, row).is_some() {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::DuplicateCoordinate,
            format!("structural coordinate ({col},{row}) is already occupied"),
        ));
    }

    let system_id = next_system_id(spec);
    let role_str = gridcell_role_str(role);
    let mut gridcell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut gridcell, role_str);
    gridcell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(row),
    );
    let mut payload = SimThing::new(SimThingKind::Cohort, 0);
    payload.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    gridcell.add_child(payload);
    let cell_raw = gridcell.id.raw();

    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    galaxy_map.add_child(gridcell);

    spec.structural_grid
        .placements
        .push(SimThingStructuralGridPlacement {
            location_id: gridcell_id.clone(),
            target_id: gridcell_id,
            system_id,
            row,
            col,
            simthing_id_raw: cell_raw,
        });
    refresh_occupied_cells(spec);
    Ok(())
}

fn move_gridcell(
    spec: &mut SimThingScenarioSpec,
    id: &str,
    new_col: u32,
    new_row: u32,
) -> Result<(), StructuralPlacementEditError> {
    let gridcell_id = normalize_gridcell_id(id)?;
    ensure_coordinate_in_frame(spec, new_col, new_row)?;
    let placement_index = placement_index_by_id(spec, &gridcell_id).ok_or_else(|| {
        edit_error(
            StructuralPlacementEditErrorKind::GridcellNotFound,
            format!("gridcell `{gridcell_id}` not found in structural_grid placements"),
        )
    })?;
    if let Some((idx, other_id)) =
        placement_at_coordinate_except(spec, new_col, new_row, placement_index)
    {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::DuplicateCoordinate,
            format!(
                "structural coordinate ({new_col},{new_row}) is occupied by `{other_id}` (placement index {idx})"
            ),
        ));
    }

    let placement = &mut spec.structural_grid.placements[placement_index];
    let cell_raw = placement.simthing_id_raw;
    placement.col = new_col;
    placement.row = new_row;

    let gridcell = gridcell_mut_by_raw(spec, cell_raw)?;
    gridcell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(new_col),
    );
    gridcell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(new_row),
    );
    Ok(())
}

fn remove_gridcell(
    spec: &mut SimThingScenarioSpec,
    id: &str,
    report: &mut StructuralPlacementEditReport,
) -> Result<(), StructuralPlacementEditError> {
    let gridcell_id = normalize_gridcell_id(id)?;
    let placement_index = placement_index_by_id(spec, &gridcell_id).ok_or_else(|| {
        edit_error(
            StructuralPlacementEditErrorKind::GridcellNotFound,
            format!("gridcell `{gridcell_id}` not found in structural_grid placements"),
        )
    })?;
    let removed = spec.structural_grid.placements.remove(placement_index);
    let system_id = removed.system_id;
    let cell_raw = removed.simthing_id_raw;

    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    let child_index = galaxy_map
        .children
        .iter()
        .position(|child| child.id.raw() == cell_raw && child.kind == SimThingKind::Location)
        .ok_or_else(|| {
            edit_error(
                StructuralPlacementEditErrorKind::GridcellNotUnderGalaxyMap,
                format!("gridcell `{gridcell_id}` simthing id {cell_raw} is not under GalaxyMap"),
            )
        })?;
    galaxy_map.children.remove(child_index);

    let removed_links = spec.links.len();
    spec.links.retain(|link| {
        link.from_system_id != system_id.to_string() && link.to_system_id != system_id.to_string()
    });
    if spec.links.len() != removed_links {
        report.warnings.push(StructuralPlacementEditWarning {
            message: format!(
                "removed {links_removed} structural link(s) referencing system_id {system_id}",
                links_removed = removed_links - spec.links.len()
            ),
        });
    }

    refresh_occupied_cells(spec);
    Ok(())
}

fn set_gridcell_role(
    spec: &mut SimThingScenarioSpec,
    id: &str,
    role: GridcellRoleEdit,
) -> Result<(), StructuralPlacementEditError> {
    let gridcell_id = normalize_gridcell_id(id)?;
    let placement = placement_by_id(spec, &gridcell_id).ok_or_else(|| {
        edit_error(
            StructuralPlacementEditErrorKind::GridcellNotFound,
            format!("gridcell `{gridcell_id}` not found in structural_grid placements"),
        )
    })?;
    let cell_raw = placement.simthing_id_raw;
    let gridcell = gridcell_mut_by_raw(spec, cell_raw)?;
    apply_gridcell_role_metadata(gridcell, gridcell_role_str(role));
    Ok(())
}

fn game_session_galaxy_map_mut(
    spec: &mut SimThingScenarioSpec,
) -> Result<&mut SimThing, StructuralPlacementEditError> {
    let map_raw = game_session_galaxy_map(spec)
        .map_err(map_root_error)?
        .id
        .raw();
    let game_session = game_session_child_mut(spec).map_err(map_root_error)?;
    game_session
        .children
        .iter_mut()
        .find(|child| child.id.raw() == map_raw)
        .ok_or_else(|| {
            edit_error(
                StructuralPlacementEditErrorKind::MissingGalaxyMap,
                "GalaxyMap child missing under GameSession",
            )
        })
}

fn gridcell_mut_by_raw(
    spec: &mut SimThingScenarioSpec,
    cell_raw: u32,
) -> Result<&mut SimThing, StructuralPlacementEditError> {
    let galaxy_map = game_session_galaxy_map_mut(spec)?;
    galaxy_map
        .children
        .iter_mut()
        .find(|child| child.id.raw() == cell_raw && child.kind == SimThingKind::Location)
        .ok_or_else(|| {
            edit_error(
                StructuralPlacementEditErrorKind::GridcellNotUnderGalaxyMap,
                format!("gridcell simthing id {cell_raw} is not a direct GalaxyMap child"),
            )
        })
}

fn placement_by_id<'a>(
    spec: &'a SimThingScenarioSpec,
    id: &str,
) -> Option<&'a SimThingStructuralGridPlacement> {
    spec.structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == id || p.target_id == id)
}

fn placement_index_by_id(spec: &SimThingScenarioSpec, id: &str) -> Option<usize> {
    spec.structural_grid
        .placements
        .iter()
        .position(|p| p.location_id == id || p.target_id == id)
}

fn placement_at_coordinate(
    spec: &SimThingScenarioSpec,
    col: u32,
    row: u32,
) -> Option<&SimThingStructuralGridPlacement> {
    spec.structural_grid
        .placements
        .iter()
        .find(|p| p.col == col && p.row == row)
}

fn placement_at_coordinate_except(
    spec: &SimThingScenarioSpec,
    col: u32,
    row: u32,
    except_index: usize,
) -> Option<(usize, String)> {
    spec.structural_grid
        .placements
        .iter()
        .enumerate()
        .find(|(idx, p)| *idx != except_index && p.col == col && p.row == row)
        .map(|(idx, p)| (idx, p.location_id.clone()))
}

fn next_system_id(spec: &SimThingScenarioSpec) -> u32 {
    spec.structural_grid
        .placements
        .iter()
        .map(|p| p.system_id)
        .max()
        .map(|max| max.saturating_add(1))
        .unwrap_or(1)
}

fn refresh_occupied_cells(spec: &mut SimThingScenarioSpec) {
    spec.structural_grid.frame.occupied_cells = spec.structural_grid.placements.len() as u64;
}

fn ensure_coordinate_in_frame(
    spec: &SimThingScenarioSpec,
    col: u32,
    row: u32,
) -> Result<(), StructuralPlacementEditError> {
    if col > SCENARIO_STRUCTURAL_INTEGER_MAX || row > SCENARIO_STRUCTURAL_INTEGER_MAX {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::InvalidCoordinate,
            format!("coordinate ({col},{row}) exceeds structural integer max"),
        ));
    }
    let width = spec.structural_grid.frame.width;
    let height = spec.structural_grid.frame.height;
    if width > 0 && col >= width {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::InvalidCoordinate,
            format!("col {col} is outside structural frame width {width}"),
        ));
    }
    if height > 0 && row >= height {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::InvalidCoordinate,
            format!("row {row} is outside structural frame height {height}"),
        ));
    }
    Ok(())
}

fn normalize_gridcell_id(id: &str) -> Result<String, StructuralPlacementEditError> {
    let trimmed = id.trim();
    if trimmed.is_empty() {
        return Err(edit_error(
            StructuralPlacementEditErrorKind::GridcellNotFound,
            "gridcell id must be non-empty",
        ));
    }
    Ok(trimmed.to_string())
}

fn gridcell_role_str(role: GridcellRoleEdit) -> &'static str {
    match role {
        GridcellRoleEdit::Inert => GALAXY_GRIDCELL_ROLE_INERT,
        GridcellRoleEdit::StarSystem => GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
        GridcellRoleEdit::UnknownUnsupported => "unknown_unsupported",
    }
}

fn edit_error(
    kind: StructuralPlacementEditErrorKind,
    message: impl Into<String>,
) -> StructuralPlacementEditError {
    StructuralPlacementEditError {
        kind,
        message: message.into(),
    }
}

fn map_root_error(err: ScenarioRootError) -> StructuralPlacementEditError {
    let kind = match err {
        ScenarioRootError::RootIsNotScenario | ScenarioRootError::ArbitraryRootKind { .. } => {
            StructuralPlacementEditErrorKind::NonCanonicalScenarioRoot
        }
        ScenarioRootError::MissingGameSessionChild
        | ScenarioRootError::MultipleGameSessionChildren { .. }
        | ScenarioRootError::GameSessionChildWrongKind { .. } => {
            StructuralPlacementEditErrorKind::MissingGameSession
        }
        ScenarioRootError::MissingGalaxyMap
        | ScenarioRootError::MultipleGalaxyMaps { .. }
        | ScenarioRootError::GalaxyMapNotDirectGameSessionChild
        | ScenarioRootError::GalaxyMapMissingId => {
            StructuralPlacementEditErrorKind::MissingGalaxyMap
        }
        _ => StructuralPlacementEditErrorKind::ValidationFailed,
    };
    edit_error(kind, err.to_string())
}

/// Validate structural_grid placements reference GalaxyMap children (read-only guard).
pub fn validate_structural_placements_under_galaxymap(
    spec: &SimThingScenarioSpec,
) -> Result<(), StructuralPlacementEditError> {
    ensure_canonical_edit_surface(spec)?;
    let galaxy_map = game_session_galaxy_map(spec).map_err(map_root_error)?;
    let child_ids: std::collections::BTreeSet<u32> = galaxy_map
        .children
        .iter()
        .filter(|c| c.kind == SimThingKind::Location)
        .map(|c| c.id.raw())
        .collect();
    for placement in &spec.structural_grid.placements {
        if !child_ids.contains(&placement.simthing_id_raw) {
            return Err(edit_error(
                StructuralPlacementEditErrorKind::StaleStructuralPlacement,
                format!(
                    "placement `{}` references simthing id {} not under GalaxyMap",
                    placement.location_id, placement.simthing_id_raw
                ),
            ));
        }
    }
    Ok(())
}
