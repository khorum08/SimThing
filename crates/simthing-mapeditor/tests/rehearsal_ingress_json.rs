//! Plain JSON interchange uses spec admission before Studio replacement.
use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::app::scenario_io::{load_scenario_action, ScenarioActionResult};
use simthing_mapeditor::app::StudioAppState;
use simthing_mapeditor::{
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    save_current_session_scenario_to_path, StudioScenarioAuthorityKind,
};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_scenario_metadata_to_root,
    apply_star_system_local_grid_frame_metadata, load_scenario_spec_from_json_str, make_galaxy_map,
    make_local_inert_gridcell, make_owner_entity, save_scenario_spec_to_canonical_json,
    serialize_scenario_authority, structural_property_value_u32, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn document(canonical: bool, local_child_col: u32) -> SimThingScenarioSpec {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    for (property, value) in [
        (SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, 1),
        (SCENARIO_STRUCTURAL_COL_PROPERTY_ID, 0),
        (SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, 0),
    ] {
        cell.add_property(property, structural_property_value_u32(value));
    }
    apply_gridcell_role_metadata(&mut cell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    apply_star_system_local_grid_frame_metadata(&mut cell, 2, 2);
    cell.add_child(make_local_inert_gridcell(local_child_col, 0));
    let cell_raw = cell.id.raw();
    let mut map = make_galaxy_map("json_map", "JSON map");
    let map_raw = map.id.raw();
    map.add_child(cell);
    let scenario_id = "json_interchange".to_string();
    let provenance = SimThingScenarioProvenance {
        source: "rehearsal_ingress_json".into(),
        generator_seed: 11,
        generator_shape: "test".into(),
        ..Default::default()
    };
    let mut root = SimThing::new(
        if canonical {
            SimThingKind::Scenario
        } else {
            SimThingKind::World
        },
        0,
    );
    if canonical {
        apply_scenario_metadata_to_root(
            &mut root,
            &scenario_id,
            &provenance,
            SCENARIO_SCHEMA_VERSION,
        );
        let mut session = SimThing::new(SimThingKind::GameSession, 0);
        session.add_child(make_owner_entity("json_owner", "JSON owner", "player"));
        session.add_child(map);
        root.add_child(session);
    } else {
        root.add_child(map);
    }
    SimThingScenarioSpec {
        scenario_id,
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 1,
                height: 1,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "json_cell".into(),
                target_id: "json_cell".into(),
                system_id: 1,
                row: 0,
                col: 0,
                simthing_id_raw: cell_raw,
            }],
        },
        links: vec![],
        provenance,
    }
}

fn canonical_identity(scenario: &SimThingScenarioSpec) -> Result<String, simthing_spec::SpecError> {
    Ok(save_scenario_spec_to_canonical_json(scenario)?.canonical_json)
}

#[test]
fn rehearsal_ingress_json_preserves_admitted_canonical_and_compatibility_records() {
    let directory = tempfile::tempdir().unwrap();
    for canonical in [true, false] {
        let path = directory
            .path()
            .join(format!("{canonical}.simthing-scenario.json"));
        let text = serialize_scenario_authority(&document(canonical, 0)).unwrap();
        std::fs::write(&path, &text).unwrap();
        let (admitted, report) = load_scenario_spec_from_json_str("direct", &text).unwrap();
        assert!(report.ingestion_ready);
        let expected = canonical_identity(&admitted).unwrap();
        let loaded = load_scenario_authority_from_path(&path).unwrap();
        assert_eq!(canonical_identity(&loaded).unwrap(), expected);
        let session = load_studio_session_from_scenario_path(&path, None).unwrap();
        assert_eq!(
            canonical_identity(&session.scenario_authority).unwrap(),
            expected
        );
        assert_eq!(
            session.scenario_document.authority_kind,
            if canonical {
                StudioScenarioAuthorityKind::CanonicalScenario
            } else {
                StudioScenarioAuthorityKind::LegacyWorldRoot
            }
        );
        assert_eq!(session.admission_summary.legacy_world_root, !canonical);
        let saved = directory
            .path()
            .join(format!("saved-{canonical}.simthing-scenario.json"));
        save_current_session_scenario_to_path(&session, &saved).unwrap();
        let (roundtrip, report) =
            load_scenario_spec_from_json_str("saved", &std::fs::read_to_string(&saved).unwrap())
                .unwrap();
        assert!(report.ingestion_ready);
        assert_eq!(canonical_identity(&roundtrip).unwrap(), expected);
    }
}

#[test]
fn rehearsal_ingress_json_refuses_nonadmitted_document_before_replacement() {
    let directory = tempfile::tempdir().unwrap();
    let valid = directory.path().join("valid.simthing-scenario.json");
    std::fs::write(
        &valid,
        serialize_scenario_authority(&document(true, 0)).unwrap(),
    )
    .unwrap();
    let mut state = StudioAppState::default();
    state.session = Some(load_studio_session_from_scenario_path(&valid, None).unwrap());
    let before = canonical_identity(&state.session.as_ref().unwrap().scenario_authority).unwrap();
    let rejected = serialize_scenario_authority(&document(true, 3)).unwrap();
    // Structural serde succeeds, but canonical admission rejects the out-of-frame local cell.
    assert!(simthing_spec::deserialize_scenario_authority(&rejected).is_ok());
    let (_, report) = load_scenario_spec_from_json_str("nonadmitted", &rejected).unwrap();
    assert!(!report.ingestion_ready);
    for (name, text) in [
        ("nonadmitted", rejected.as_str()),
        ("malformed", "{invalid JSON"),
    ] {
        let path = directory
            .path()
            .join(format!("{name}.simthing-scenario.json"));
        std::fs::write(&path, text).unwrap();
        assert!(
            load_scenario_authority_from_path(&path).is_err(),
            "{name} must refuse"
        );
        let result = load_scenario_action(&mut state, &path);
        assert!(matches!(result, ScenarioActionResult::Failed { .. }));
        assert!(result.message().contains(path.to_string_lossy().as_ref()));
        assert!(result.message().contains("canonical JSON admission"));
        assert_eq!(
            canonical_identity(&state.session.as_ref().unwrap().scenario_authority).unwrap(),
            before
        );
        assert_eq!(
            state.session.as_ref().unwrap().scenario_path.as_ref(),
            Some(&valid)
        );
    }
}
