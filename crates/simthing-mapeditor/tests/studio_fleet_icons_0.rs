//! STUDIO-FLEET-ICONS-0 — descriptor layer + renderer-seam integration proofs.

use std::collections::HashMap;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    clamp_fleet_icon_scale, default_fleet_icon_silhouette, fleet_icon_descriptors_from_records,
    fleet_icon_mesh_draw_plans, fleet_icon_ops_telemetry_rows, fleet_presence_records_flat,
    resolve_fleet_icon_world_pose, studio_fleet_presence_map_from_session, DummySecondFleetIconBackend,
    FleetIconOrientation, FleetIconPlacement, FleetIconRenderer, FleetIconSide,
    RecordingFleetIconRenderer, StudioSession, StudioSystemRenderAnchor,
    FLEET_ICON_DEFAULT_SILHOUETTE_ID, FLEET_ICON_MAX_STAR_BLUR_FRACTION,
    FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION,
};
use simthing_spec::{
    apply_galaxy_map_metadata, apply_gridcell_role_metadata, apply_owner_entity_metadata,
    apply_participant_owner_flow_metadata, apply_scenario_metadata_to_root, make_planet_gridcell,
    structural_property_value_u32, FleetPresenceLocation, FleetPresenceRecord, OwnerRef,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn rec(id: u32, owner: Option<&str>, loc: FleetPresenceLocation) -> FleetPresenceRecord {
    FleetPresenceRecord {
        fleet_simthing_id_raw: id,
        owner_ref: owner.map(OwnerRef::new),
        posture: None,
        location: loc,
    }
}

fn two_owner_session() -> StudioSession {
    let mut scenario = SimThing::new(SimThingKind::Scenario, 0);
    apply_scenario_metadata_to_root(
        &mut scenario,
        "studio_fleet_icons_0",
        &SimThingScenarioProvenance::default(),
        SCENARIO_SCHEMA_VERSION,
    );

    let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
    let mut owner_a = SimThing::new(SimThingKind::Owner, 0);
    apply_owner_entity_metadata(&mut owner_a, "owner_a", "Owner A", "player");
    let mut owner_b = SimThing::new(SimThingKind::Owner, 0);
    apply_owner_entity_metadata(&mut owner_b, "owner_b", "Owner B", "ai");
    game_session.add_child(owner_a);
    game_session.add_child(owner_b);

    let mut galaxy_map = SimThing::new(SimThingKind::Location, 0);
    apply_galaxy_map_metadata(&mut galaxy_map, "galaxy", "Galaxy");
    let map_raw = galaxy_map.id.raw();

    let mut system = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut system, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
    system.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(3),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    system.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    let system_raw = system.id.raw();
    let mut planet = make_planet_gridcell("planet", 0, 0, Some("Planet"));
    let surface = planet.children.first_mut().expect("surface");
    let mut fleet_a = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut fleet_a, "owner_a", 0, 0);
    let mut fleet_b = SimThing::new(SimThingKind::Fleet, 0);
    apply_participant_owner_flow_metadata(&mut fleet_b, "owner_b", 0, 0);
    surface.add_child(fleet_a);
    surface.add_child(fleet_b);
    system.add_child(planet);
    galaxy_map.add_child(system);
    game_session.add_child(galaxy_map);
    scenario.add_child(game_session);

    let spec = SimThingScenarioSpec {
        scenario_id: "studio_fleet_icons_0".into(),
        root: scenario,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 8,
                height: 8,
                occupied_cells: 1,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![SimThingStructuralGridPlacement {
                location_id: "system_3".into(),
                target_id: "system_3".into(),
                system_id: 3,
                row: 2,
                col: 1,
                simthing_id_raw: system_raw,
            }],
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    StudioSession::from_loaded_scenario(spec, PathBuf::from("fixture.simthing-scenario.json"), None)
        .expect("loaded StudioSession")
}

/// catches: selected-owner fleets not sitting right / others not left (mirror law).
#[test]
fn selected_owner_right_others_left_mirror() {
    let records = vec![
        rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(3)),
        rec(2, Some("owner_b"), FleetPresenceLocation::Anchored(3)),
        rec(3, None, FleetPresenceLocation::Anchored(3)),
    ];
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), 2.0);
    let side = |id| {
        descs
            .iter()
            .find(|d| d.fleet_simthing_id_raw == id)
            .map(|d| d.side)
            .expect("fleet")
    };
    assert_eq!(side(1), FleetIconSide::Right);
    assert_eq!(side(2), FleetIconSide::Left);
    assert_eq!(side(3), FleetIconSide::Left);

    let anchors = vec![StudioSystemRenderAnchor {
        system_id: 3,
        structural_col: 1,
        structural_row: 2,
        world_position: [0.0, 0.0, 0.0],
        render_height: 0.0,
    }];
    let right = descs.iter().find(|d| d.fleet_simthing_id_raw == 1).unwrap();
    let left = descs.iter().find(|d| d.fleet_simthing_id_raw == 2).unwrap();
    let pr = resolve_fleet_icon_world_pose(right, &anchors, [1.0, 0.0], 2.0).unwrap();
    let pl = resolve_fleet_icon_world_pose(left, &anchors, [1.0, 0.0], 2.0).unwrap();
    assert!(
        (pr.world_position[0] + pl.world_position[0]).abs() < 1e-4,
        "left/right must be mirror-symmetric on X for +X right axis"
    );
}

/// catches: transit icons not at ~30% or not oriented toward destination.
#[test]
fn transit_thirty_percent_and_orientation_toward_dest() {
    let records = vec![rec(
        9,
        Some("owner_b"),
        FleetPresenceLocation::InTransit {
            source_system_id: 1,
            dest_system_id: 2,
        },
    )];
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), 1.0);
    assert_eq!(descs[0].side, FleetIconSide::Transit);
    assert_eq!(
        descs[0].orientation,
        FleetIconOrientation::TowardTransitDestination
    );
    match &descs[0].placement {
        FleetIconPlacement::InTransit {
            along_fraction, ..
        } => assert!((along_fraction - FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION).abs() < 1e-6),
        other => panic!("expected InTransit, got {other:?}"),
    }
    let anchors = vec![
        StudioSystemRenderAnchor {
            system_id: 1,
            structural_col: 0,
            structural_row: 0,
            world_position: [0.0, 0.0, 0.0],
            render_height: 0.0,
        },
        StudioSystemRenderAnchor {
            system_id: 2,
            structural_col: 1,
            structural_row: 0,
            world_position: [10.0, 0.0, 0.0],
            render_height: 0.0,
        },
    ];
    let pose = resolve_fleet_icon_world_pose(&descs[0], &anchors, [1.0, 0.0], 1.0).unwrap();
    assert!((pose.world_position[0] - 3.0).abs() < 1e-4);
}

/// catches: arrival leaves a sticky transit placement instead of snapping to anchor slot.
#[test]
fn arrival_snap_to_anchor_slot() {
    let transit = rec(
        9,
        Some("owner_a"),
        FleetPresenceLocation::InTransit {
            source_system_id: 1,
            dest_system_id: 2,
        },
    );
    let arrived = rec(9, Some("owner_a"), FleetPresenceLocation::Anchored(2));
    let before =
        fleet_icon_descriptors_from_records(&[transit], Some("owner_a"), &HashMap::new(), 1.0);
    let after =
        fleet_icon_descriptors_from_records(&[arrived], Some("owner_a"), &HashMap::new(), 1.0);
    assert!(matches!(
        before[0].placement,
        FleetIconPlacement::InTransit { .. }
    ));
    match &after[0].placement {
        FleetIconPlacement::Anchored {
            system_id, side, ..
        } => {
            assert_eq!(*system_id, 2);
            assert_eq!(*side, FleetIconSide::Right);
        }
        other => panic!("arrival must snap to Anchored, got {other:?}"),
    }
}

/// catches: scale exceeds 75% of base max star blur size.
#[test]
fn scale_bound_seventy_five_percent_of_base_max_star_blur() {
    let base = 4.0;
    let cap = base * FLEET_ICON_MAX_STAR_BLUR_FRACTION;
    assert!((clamp_fleet_icon_scale(99.0, base) - cap).abs() < 1e-6);
    let descs = fleet_icon_descriptors_from_records(
        &[rec(1, Some("a"), FleetPresenceLocation::Anchored(1))],
        Some("a"),
        &HashMap::new(),
        base,
    );
    assert!(descs[0].scale <= cap + 1e-6);
}

/// catches: second backend cannot consume the same descriptors (forward-compat break).
#[test]
fn dummy_second_backend_consumes_identical_descriptors() {
    let records = vec![
        rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(1)),
        rec(
            2,
            Some("owner_b"),
            FleetPresenceLocation::InTransit {
                source_system_id: 1,
                dest_system_id: 2,
            },
        ),
    ];
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), 1.5);
    let mut primary = RecordingFleetIconRenderer::default();
    let mut dummy = DummySecondFleetIconBackend::default();
    let _ = primary.render_descriptors(&descs);
    let _ = dummy.render_descriptors(&descs);
    assert_eq!(primary.last_frame, dummy.accepted);
    assert_eq!(dummy.accepted, descs);
}

/// catches: silhouette shape not data at one site / default id drift.
#[test]
fn silhouette_is_one_site_data() {
    let sil = default_fleet_icon_silhouette();
    assert_eq!(sil.id, FLEET_ICON_DEFAULT_SILHOUETTE_ID);
    assert!(sil.outline_xy.len() >= 3);
    let records = vec![rec(1, Some("a"), FleetPresenceLocation::Anchored(1))];
    let descs = fleet_icon_descriptors_from_records(&records, None, &HashMap::new(), 1.0);
    assert_eq!(descs[0].silhouette_id, sil.id);
}

/// catches: mapeditor presence map cannot feed descriptor construction (12.4 wire break).
#[test]
fn mapeditor_presence_map_feeds_descriptors() {
    let session = two_owner_session();
    let map = studio_fleet_presence_map_from_session(&session).expect("presence");
    assert!(map.total_fleets >= 2);
    let records = fleet_presence_records_flat(&map.by_system_id);
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), 1.85);
    assert_eq!(descs.len(), map.total_fleets);
    let rows = fleet_icon_ops_telemetry_rows(&descs);
    assert_eq!(rows.len(), descs.len());
    assert!(rows.iter().any(|r| r.side == FleetIconSide::Right));
    assert!(rows.iter().any(|r| r.side == FleetIconSide::Left));
}

/// catches: mesh draw plans require a new pipeline type instead of outline + pose data.
#[test]
fn mesh_draw_plans_are_outline_pose_only() {
    let records = vec![rec(1, Some("a"), FleetPresenceLocation::Anchored(5))];
    let descs = fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), 1.0);
    let anchors = vec![StudioSystemRenderAnchor {
        system_id: 5,
        structural_col: 0,
        structural_row: 0,
        world_position: [1.0, 2.0, 3.0],
        render_height: 0.0,
    }];
    let plans = fleet_icon_mesh_draw_plans(&descs, &anchors, [1.0, 0.0], 1.0);
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].silhouette_id, FLEET_ICON_DEFAULT_SILHOUETTE_ID);
    assert!(!plans[0].outline_xy.is_empty());
}

/// catches: presentation module grows ScenarioSpec mutation or WGSL surfaces.
#[test]
fn fleet_icons_module_has_no_wgsl_or_spec_mutation_surface() {
    let source = include_str!("../src/studio_fleet_icons.rs");
    assert!(
        !source.contains(".wgsl") && !source.contains("include_str!"),
        "fleet icon base must stay presentation-data only (no WGSL)"
    );
    assert!(
        !source.contains("scenario_authority") && !source.contains("mutate"),
        "fleet icon descriptors must not mutate ScenarioSpec"
    );
    assert!(
        source.contains("trait FleetIconRenderer"),
        "narrow renderer seam must exist"
    );
}
