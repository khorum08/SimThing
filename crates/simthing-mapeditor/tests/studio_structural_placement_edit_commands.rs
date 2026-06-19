//! STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — Studio structural edit wrapper proofs.

use std::fs;
use std::path::PathBuf;

use simthing_driver::evaluate_scenario_compile_readiness;
use simthing_mapeditor::{
    load_studio_session_from_scenario_path, save_scenario_authority_to_path,
    studio_apply_structural_placement_command, studio_scenario_authority_snapshot,
    StudioGridcellRole,
};
use simthing_spec::{
    deserialize_scenario_authority, gridcell_role, validate_scenario_root_authority,
    GridcellRoleEdit, ScenarioRootValidationMode, StructuralPlacementCommand,
    StructuralPlacementEditErrorKind, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
};
use tempfile::TempDir;

fn galaxymap_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json")
}

#[test]
fn studio_add_gridcell_rebuilds_document_projection_and_admission() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    let before_cells = session.scenario_document.gridcells.len();
    let outcome = studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::AddGridcell {
            id: "studio_cell".into(),
            col: 3,
            row: 2,
            role: GridcellRoleEdit::StarSystem,
        },
    )
    .expect("add");
    assert_eq!(outcome.edit_report.applied_count, 1);
    assert_eq!(outcome.gridcell_count, before_cells + 1);
    assert_eq!(session.scenario_document.gridcells.len(), before_cells + 1);
    assert!(session
        .scenario_document
        .gridcells
        .iter()
        .any(|c| c.role == StudioGridcellRole::StarSystem && c.structural_col == Some(3)));
    assert!(matches!(
        session.admission_summary.classification.as_str(),
        "Admitted" | "PartiallyAdmitted"
    ));
    assert_eq!(session.view_model.stars.len(), outcome.gridcell_count);
}

#[test]
fn studio_move_gridcell_roundtrips_save_reload() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::MoveGridcell {
            id: "cell_a".into(),
            new_col: 4,
            new_row: 3,
        },
    )
    .expect("move");

    let dir = TempDir::new().expect("tempdir");
    let save_path = dir.path().join("moved.simthing-scenario.json");
    save_scenario_authority_to_path(&save_path, &session.scenario_authority).expect("save");
    let reloaded = load_studio_session_from_scenario_path(&save_path, None).expect("reload");
    let placement = reloaded
        .scenario_authority
        .structural_grid
        .placements
        .iter()
        .find(|p| p.location_id == "cell_a")
        .expect("cell_a");
    assert_eq!(placement.col, 4);
    assert_eq!(placement.row, 3);
    validate_scenario_root_authority(
        &reloaded.scenario_authority,
        ScenarioRootValidationMode::Canonical,
    )
    .expect("canonical");
}

#[test]
fn studio_remove_gridcell_roundtrips_save_reload() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::RemoveGridcell {
            id: "cell_b".into(),
        },
    )
    .expect("remove");

    let dir = TempDir::new().expect("tempdir");
    let save_path = dir.path().join("removed.simthing-scenario.json");
    save_scenario_authority_to_path(&save_path, &session.scenario_authority).expect("save");
    let reloaded = load_studio_session_from_scenario_path(&save_path, None).expect("reload");
    assert_eq!(
        reloaded.scenario_authority.structural_grid.placements.len(),
        1
    );
    assert_eq!(reloaded.scenario_document.gridcells.len(), 1);
}

#[test]
fn studio_set_gridcell_role_updates_display() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::SetGridcellRole {
            id: "cell_a".into(),
            role: GridcellRoleEdit::StarSystem,
        },
    )
    .expect("set role");
    let view = session
        .scenario_document
        .gridcells
        .iter()
        .find(|c| c.structural_col == Some(0) && c.structural_row == Some(0))
        .expect("cell_a view");
    assert_eq!(view.role, StudioGridcellRole::StarSystem);
    let galaxy_map =
        simthing_spec::game_session_galaxy_map(&session.scenario_authority).expect("map");
    let gridcell = galaxy_map.children.first().expect("gridcell");
    assert_eq!(
        gridcell_role(gridcell).as_deref(),
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM)
    );
}

#[test]
fn studio_rejects_invalid_structural_edit_without_partial_mutation() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    let before = studio_scenario_authority_snapshot(&session.scenario_authority);
    let err = studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::AddGridcell {
            id: "dup_coord".into(),
            col: 0,
            row: 0,
            role: GridcellRoleEdit::Inert,
        },
    )
    .unwrap_err();
    assert!(matches!(
        err,
        simthing_mapeditor::StudioStructuralEditError::StructuralEdit(_)
    ));
    let inner = match err {
        simthing_mapeditor::StudioStructuralEditError::StructuralEdit(inner) => inner,
        _ => panic!("expected structural edit error"),
    };
    assert_eq!(
        inner.kind,
        StructuralPlacementEditErrorKind::DuplicateCoordinate
    );
    assert_eq!(
        studio_scenario_authority_snapshot(&session.scenario_authority),
        before
    );
}

#[test]
fn studio_structural_edit_does_not_dispatch_gpu() {
    let src = include_str!("../src/studio_structural_edit.rs");
    let lib = include_str!("../src/lib.rs");
    for forbidden in [
        "SimGpuAccumulatorTickState",
        "compile_owner_silo_gpu_tick_plan",
        "gpu_context_blocking",
        "AccumulatorOpSession",
    ] {
        assert!(
            !src.contains(forbidden) && !lib.contains(forbidden),
            "{forbidden}"
        );
    }
}

#[test]
fn studio_structural_edit_does_not_call_sim_tick() {
    let src = include_str!("../src/studio_structural_edit.rs");
    for forbidden in [
        "simthing_sim",
        "SimTickError",
        "execute_accumulator_plan_tick",
    ] {
        assert!(!src.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn studio_edited_scenario_driver_structural_n4_readiness_preserved() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::AddGridcell {
            id: "driver_cell".into(),
            col: 6,
            row: 4,
            role: GridcellRoleEdit::Inert,
        },
    )
    .expect("add");
    let readiness = evaluate_scenario_compile_readiness(&session.scenario_authority);
    assert!(readiness.structural_n4_ready);
}

#[test]
fn studio_structural_edit_reload_preserves_admission_summary() {
    let path = galaxymap_fixture_path();
    let mut session = load_studio_session_from_scenario_path(&path, None).expect("load");
    studio_apply_structural_placement_command(
        &mut session,
        StructuralPlacementCommand::AddGridcell {
            id: "persist_cell".into(),
            col: 7,
            row: 5,
            role: GridcellRoleEdit::Inert,
        },
    )
    .expect("add");
    let dir = TempDir::new().expect("tempdir");
    let save_path = dir.path().join("persist.simthing-scenario.json");
    save_scenario_authority_to_path(&save_path, &session.scenario_authority).expect("save");
    let json = fs::read_to_string(&save_path).expect("read");
    let spec = deserialize_scenario_authority(&json).expect("parse");
    let reloaded = load_studio_session_from_scenario_path(&save_path, None).expect("reload");
    assert_eq!(spec.structural_grid.placements.len(), 3);
    assert!(matches!(
        reloaded.admission_summary.classification.as_str(),
        "Admitted" | "PartiallyAdmitted"
    ));
}
