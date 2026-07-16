//! STUDIO-DISRUPTION-READOUT-0 typed read-only snapshot contract proof.

use std::collections::BTreeMap;

use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_galaxy_map_metadata, apply_gridcell_role_metadata, apply_scenario_metadata_to_root,
    disruption_readout_snapshot, disruption_readout_snapshot_with_readback,
    serialize_scenario_authority, structural_property_value_u32, DisruptionAuthorityReadback,
    DisruptionAuthorityReadbackError, DisruptionReadoutSnapshotError, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn structural_shell_spec() -> SimThingScenarioSpec {
    let mut scenario = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut scenario,
        "studio_disruption_readout_0",
        &SimThingScenarioProvenance::default(),
        SCENARIO_SCHEMA_VERSION,
    );

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    let mut galaxy_map = SimThing::new(SimThingKind::Location, 0);
    apply_galaxy_map_metadata(&mut galaxy_map, "galaxy", "Galaxy");
    let map_raw = galaxy_map.id.raw();

    let mut placements = Vec::new();
    for (system_id, row, col) in [(7, 2, 3), (8, 2, 4)] {
        let mut system = SimThing::new(SimThingKind::Location, 0);
        apply_gridcell_role_metadata(&mut system, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
        system.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(system_id),
        );
        system.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(col),
        );
        system.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(row),
        );
        let system_raw = system.id.raw();
        galaxy_map.add_child(system);
        placements.push(SimThingStructuralGridPlacement {
            location_id: format!("system_{system_id}"),
            target_id: format!("system_{system_id}"),
            system_id,
            row,
            col,
            simthing_id_raw: system_raw,
        });
    }

    game_session.add_child(galaxy_map);
    scenario.add_child(game_session);

    SimThingScenarioSpec {
        scenario_id: "studio_disruption_readout_0".into(),
        root: scenario,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements,
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    }
}

#[derive(Debug)]
struct FixedReadback {
    values: BTreeMap<u32, f32>,
}

impl DisruptionAuthorityReadback for FixedReadback {
    fn max_disruption_accreted_by_system_id(
        &self,
    ) -> Result<Option<BTreeMap<u32, f32>>, DisruptionAuthorityReadbackError> {
        Ok(Some(self.values.clone()))
    }
}

#[derive(Debug)]
struct FailingReadback;

impl DisruptionAuthorityReadback for FailingReadback {
    fn max_disruption_accreted_by_system_id(
        &self,
    ) -> Result<Option<BTreeMap<u32, f32>>, DisruptionAuthorityReadbackError> {
        Err(DisruptionAuthorityReadbackError::new(
            "fixture readback failure",
        ))
    }
}

/// catches: absent 12.8 field being treated as an error or omitted instead of neutral 0.0.
#[test]
fn absent_disruption_field_fails_soft_to_zero_per_system() {
    let spec = structural_shell_spec();
    let snapshot = disruption_readout_snapshot(&spec).expect("absent field snapshot");
    assert_eq!(snapshot.records().len(), 2);
    assert_eq!(
        snapshot
            .records()
            .iter()
            .map(|record| (record.system_id(), record.max_disruption_accreted()))
            .collect::<Vec<_>>(),
        vec![(7, 0.0), (8, 0.0)]
    );
}

/// catches: structural-shell live bridge fallback losing per-system neutral rows.
#[test]
fn structural_shell_absent_field_holds_neutral_snapshot() {
    let spec = structural_shell_spec();
    let snapshot = disruption_readout_snapshot(&spec).expect("structural shell snapshot");
    let by_system = snapshot.by_system_id();
    assert_eq!(by_system.keys().copied().collect::<Vec<_>>(), vec![7, 8]);
    assert!(by_system
        .values()
        .all(|record| record.max_disruption_accreted() == 0.0));
}

/// catches: readback authority errors being converted to neutral 0.0.
#[test]
fn authority_readback_error_fails_loud_with_typed_error() {
    let spec = structural_shell_spec();
    let err = disruption_readout_snapshot_with_readback(&spec, &FailingReadback)
        .expect_err("readback error must surface");
    assert!(matches!(
        err,
        DisruptionReadoutSnapshotError::AuthorityReadback(_)
    ));
    assert!(err.to_string().contains("fixture readback failure"));
}

/// catches: snapshot assembly mixing generated ids or re-reading inconsistent values.
#[test]
fn disruption_readout_snapshot_is_consistent_for_one_tick_readback() {
    let spec = structural_shell_spec();
    let snapshot = disruption_readout_snapshot_with_readback(
        &spec,
        &FixedReadback {
            values: BTreeMap::from([(7, 12.5), (8, 50.0), (99, 100.0)]),
        },
    )
    .expect("live readback snapshot");
    assert_eq!(
        snapshot
            .records()
            .iter()
            .map(|record| (record.system_id(), record.max_disruption_accreted()))
            .collect::<Vec<_>>(),
        vec![(7, 12.5), (8, 50.0)]
    );
}

/// catches: read-only snapshot path mutating ScenarioSpec authority.
#[test]
fn disruption_readout_does_not_mutate_scenario_spec() {
    let spec = structural_shell_spec();
    let before = serialize_scenario_authority(&spec).expect("serialize before");
    let _ = disruption_readout_snapshot_with_readback(
        &spec,
        &FixedReadback {
            values: BTreeMap::from([(7, 1.0), (8, 2.0)]),
        },
    )
    .expect("snapshot");
    let after = serialize_scenario_authority(&spec).expect("serialize after");
    assert_eq!(before, after);
}
