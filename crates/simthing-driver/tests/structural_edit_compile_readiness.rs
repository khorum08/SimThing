//! STRUCTURAL-PLACEMENT-EDIT-COMMANDS-0 — driver structural readiness after structural edits.

use std::fs;
use std::path::PathBuf;

use simthing_driver::{
    compile_structural_n4_theater, evaluate_scenario_compile_readiness, StructuralTheaterAdmission,
};
use simthing_spec::{
    apply_structural_placement_command, deserialize_scenario_authority, GridcellRoleEdit,
    MappingExecutionProfile, StructuralPlacementCommand, StructuralPlacementEditErrorKind,
};

fn galaxymap_fixture() -> simthing_spec::SimThingScenarioSpec {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json");
    let json = fs::read_to_string(path).expect("corpus");
    deserialize_scenario_authority(&json).expect("parse")
}

#[test]
fn edited_scenario_reaches_structural_n4_admission() {
    let mut spec = galaxymap_fixture();
    apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::AddGridcell {
            id: "driver_ready".into(),
            col: 3,
            row: 1,
            role: GridcellRoleEdit::StarSystem,
        },
    )
    .expect("add");
    let readiness = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness.structural_n4_ready);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("compile");
    assert!(matches!(admission, StructuralTheaterAdmission::Admit(_)));
}

#[test]
fn invalid_structural_edit_does_not_reach_driver_compile() {
    let mut spec = galaxymap_fixture();
    let readiness_before = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness_before.structural_n4_ready);
    let err = apply_structural_placement_command(
        &mut spec,
        StructuralPlacementCommand::MoveGridcell {
            id: "cell_a".into(),
            new_col: 1,
            new_row: 0,
        },
    )
    .unwrap_err();
    assert_eq!(
        err.kind,
        StructuralPlacementEditErrorKind::DuplicateCoordinate
    );
    let readiness_after = evaluate_scenario_compile_readiness(&spec);
    assert!(readiness_after.structural_n4_ready);
    let admission =
        compile_structural_n4_theater(&spec, MappingExecutionProfile::SparseRegionFieldV1)
            .expect("unchanged compile");
    assert!(matches!(admission, StructuralTheaterAdmission::Admit(_)));
}
