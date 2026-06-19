//! STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — structural placement edit command proofs.

use std::fs;
use std::path::PathBuf;

use simthing_core::SimThingKind;
use simthing_spec::{
    apply_structural_placement_command, deserialize_scenario_authority, game_session_galaxy_map,
    gridcell_role, gridcell_structural_col, gridcell_structural_row, ingest_scenario,
    ingest_scenario_from_str, is_galaxy_map_entity, serialize_scenario_authority,
    studio_canonical_ingestion_profile, validate_scenario_root_authority,
    validate_stead_mapping_consistency, validate_structural_placements_under_galaxymap,
    GridcellRoleEdit, ScenarioIngestionClassification, ScenarioIngestionProfile,
    ScenarioRootValidationMode, SimThingScenarioSpec, StructuralPlacementCommand,
    StructuralPlacementEditErrorKind, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_galaxymap_fixture() -> SimThingScenarioSpec {
    let json = fs::read_to_string(corpus_path(
        "minimal_scenario_galaxymap.simthing-scenario.json",
    ))
    .expect("corpus");
    deserialize_scenario_authority(&json).expect("parse")
}

fn snapshot(spec: &SimThingScenarioSpec) -> String {
    serialize_scenario_authority(spec).expect("serialize")
}

#[test]
fn structural_edit_add_gridcell_updates_tree_and_structural_grid() {
    let mut spec = load_galaxymap_fixture();
    let before = snapshot(&spec);
    let report = apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "cell_c".into(),
            col: 2,
            row: 0,
            role: GridcellRoleEdit::StarSystem,
        },
    )
    .expect("add");
    assert_eq!(report.applied_count, 1);
    assert_eq!(spec.structural_grid.placements.len(), 3);
    assert_eq!(spec.structural_grid.frame.occupied_cells, 3);
    let placement = spec
        .structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == "cell_c")
        .expect("placement");
    assert_eq!(placement.col, 2);
    assert_eq!(placement.row, 0);
    let galaxy_map = game_session_galaxy_map(&spec).expect("map");
    assert_eq!(galaxy_map.children.len(), 3);
    validate_stead_mapping_consistency(&spec).expect("stead");
    assert_ne!(before, snapshot(&spec));
}

#[test]
fn structural_edit_move_gridcell_updates_coordinate_mirrors() {
    let mut spec = load_galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::MoveGridcell {
            id: "cell_a".into(),
            new_col: 3,
            new_row: 1,
        },
    )
    .expect("move");
    let placement = spec
        .structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == "cell_a")
        .expect("cell_a");
    assert_eq!(placement.col, 3);
    assert_eq!(placement.row, 1);
    let galaxy_map = game_session_galaxy_map(&spec).expect("map");
    let gridcell = galaxy_map
        .children
        .iter()
        .find(|c| c.id.raw() == placement.simthing_id_raw)
        .expect("gridcell");
    assert_eq!(gridcell_structural_col(gridcell), Some(3));
    assert_eq!(gridcell_structural_row(gridcell), Some(1));
    validate_stead_mapping_consistency(&spec).expect("stead");
}

#[test]
fn structural_edit_remove_gridcell_updates_tree_and_structural_grid() {
    let mut spec = load_galaxymap_fixture();
    let before_count = spec.structural_grid.placements.len();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::RemoveGridcell {
            id: "cell_b".into(),
        },
    )
    .expect("remove");
    assert_eq!(spec.structural_grid.placements.len(), before_count - 1);
    assert!(spec
        .structural_grid
        .placements
        .iter()
        .all(|p| p.location_id != "cell_b"));
    let galaxy_map = game_session_galaxy_map(&spec).expect("map");
    assert_eq!(galaxy_map.children.len(), 1);
    validate_stead_mapping_consistency(&spec).expect("stead");
}

#[test]
fn structural_edit_set_gridcell_role_roundtrips() {
    let mut spec = load_galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::SetGridcellRole {
            id: "cell_a".into(),
            role: GridcellRoleEdit::StarSystem,
        },
    )
    .expect("set role");
    let galaxy_map = game_session_galaxy_map(&spec).expect("map");
    let placement = spec
        .structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == "cell_a")
        .expect("cell_a");
    let gridcell = galaxy_map
        .children
        .iter()
        .find(|c| c.id.raw() == placement.simthing_id_raw)
        .expect("gridcell");
    assert_eq!(
        gridcell_role(gridcell).as_deref(),
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
    );
}

#[test]
fn structural_edit_rejects_duplicate_coordinate() {
    let mut spec = load_galaxymap_fixture();
    let before = snapshot(&spec);
    let err = apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "cell_dup".into(),
            col: 0,
            row: 0,
            role: GridcellRoleEdit::Inert,
        },
    )
    .unwrap_err();
    assert_eq!(
        err.kind,
        StructuralPlacementEditErrorKind::DuplicateCoordinate
    );
    assert_eq!(snapshot(&spec), before);
}

#[test]
fn structural_edit_rejects_missing_galaxymap() {
    let mut spec = load_galaxymap_fixture();
    let game_session = spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession");
    game_session
        .children
        .retain(|child| !is_galaxy_map_entity(child));
    let before = snapshot(&spec);
    let err = apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "cell_x".into(),
            col: 0,
            row: 0,
            role: GridcellRoleEdit::Inert,
        },
    )
    .unwrap_err();
    assert_eq!(err.kind, StructuralPlacementEditErrorKind::MissingGalaxyMap);
    assert_eq!(snapshot(&spec), before);
}

#[test]
fn structural_edit_rejects_gridcell_not_under_galaxymap() {
    let mut spec = load_galaxymap_fixture();
    let owner_raw = spec
        .root
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::GameSession)
        .expect("gs")
        .children
        .iter()
        .find(|c| c.kind == SimThingKind::Owner)
        .expect("owner")
        .id
        .raw();
    spec.structural_grid.placements[0].simthing_id_raw = owner_raw;
    let err = validate_structural_placements_under_galaxymap(&spec).unwrap_err();
    assert_eq!(
        err.kind,
        StructuralPlacementEditErrorKind::StaleStructuralPlacement
    );
}

#[test]
fn structural_edit_preserves_canonical_validation() {
    let mut spec = load_galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "cell_d".into(),
            col: 4,
            row: 2,
            role: GridcellRoleEdit::Inert,
        },
    )
    .expect("add");
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("canonical");
}

#[test]
fn structural_edit_preserves_stead_mapping_consistency() {
    let mut spec = load_galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::MoveGridcell {
            id: "cell_b".into(),
            new_col: 2,
            new_row: 1,
        },
    )
    .expect("move");
    validate_stead_mapping_consistency(&spec).expect("stead");
}

#[test]
fn structural_edit_ingestion_report_remains_admitted_or_partially_admitted() {
    let mut spec = load_galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "cell_ingest".into(),
            col: 5,
            row: 3,
            role: GridcellRoleEdit::Inert,
        },
    )
    .expect("add");
    let result = ingest_scenario(
        "edited_galaxymap",
        &spec,
        ScenarioIngestionProfile {
            require_canonical_tree: true,
            admit_legacy_world_root: true,
        },
    );
    assert!(matches!(
        result.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
    let json = serialize_scenario_authority(&spec).expect("serialize");
    let (from_str, _) = ingest_scenario_from_str(
        "edited_galaxymap_str",
        &json,
        studio_canonical_ingestion_profile(),
    );
    assert!(matches!(
        from_str.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ));
}
