//! STUDIO-FLEET-ICONS-0 — descriptor + production seam + lifecycle integration proofs.

use std::collections::HashMap;
use std::path::PathBuf;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    admitted_base_max_star_blur_world, admitted_base_star_blur_by_system, clamp_fleet_icon_scale,
    default_fleet_icon_silhouette, fleet_icon_descriptors_from_records,
    fleet_icon_frame_contract_fingerprint, fleet_icon_nose_faces_target,
    fleet_icon_ops_telemetry_rows, fleet_icon_plane_legible_to_view, fleet_presence_records_flat,
    galaxy_scene_cleanup_entity_ids, production_fleet_icon_render_frame,
    studio_fleet_presence_map_from_session, DummySecondFleetIconBackend, FleetIconOrientation,
    FleetIconPlacement, FleetIconRenderContext, FleetIconRenderer, FleetIconSceneState,
    FleetIconSide, MeshOutlineFleetIconRenderer, StudioGalaxyRenderMeta, StudioSession,
    StudioStarView, StudioSystemRenderAnchor, FLEET_ICON_DEFAULT_SILHOUETTE_ID,
    FLEET_ICON_MAX_STAR_BLUR_FRACTION, FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION,
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

fn blur_map(pairs: &[(u32, f32)]) -> HashMap<u32, f32> {
    pairs.iter().copied().collect()
}

fn ctx(anchors: &[StudioSystemRenderAnchor]) -> FleetIconRenderContext<'_> {
    FleetIconRenderContext {
        anchors,
        right_axis_xz: [1.0, 0.0],
    }
}

fn star_view(system_id: u32, sprite_scale: f32) -> StudioStarView {
    StudioStarView {
        system_id,
        display_name: format!("s{system_id}"),
        structural_col: 0,
        structural_row: 0,
        render_height: 0.0,
        world_x: 0.0,
        world_y: 0.0,
        world_z: 0.0,
        sprite_scale,
        emissive_strength: 1.0,
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
    let blur = blur_map(&[(3, 2.0)]);
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur);
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
    let frame = production_fleet_icon_render_frame(&descs, &ctx(&anchors));
    let right = frame
        .draw_plans
        .iter()
        .find(|p| p.fleet_simthing_id_raw == 1)
        .unwrap();
    let left = frame
        .draw_plans
        .iter()
        .find(|p| p.fleet_simthing_id_raw == 2)
        .unwrap();
    assert!(
        (right.pose.world_position[0] + left.pose.world_position[0]).abs() < 1e-4,
        "left/right must be mirror-symmetric on X for +X right axis"
    );
}

/// catches: transit icons not at ~30% or nose not toward destination.
#[test]
fn transit_thirty_percent_and_nose_toward_dest() {
    let records = vec![rec(
        9,
        Some("owner_b"),
        FleetPresenceLocation::InTransit {
            source_system_id: 1,
            dest_system_id: 2,
        },
    )];
    let blur = blur_map(&[(1, 1.0), (2, 1.0)]);
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur);
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
    let frame = production_fleet_icon_render_frame(&descs, &ctx(&anchors));
    let pose = &frame.draw_plans[0].pose;
    assert!((pose.world_position[0] - 3.0).abs() < 1e-4);
    assert!(fleet_icon_nose_faces_target(pose, [10.0, 0.0, 0.0]));
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
    let blur = blur_map(&[(1, 1.0), (2, 1.0)]);
    let before =
        fleet_icon_descriptors_from_records(&[transit], Some("owner_a"), &HashMap::new(), &blur);
    let after =
        fleet_icon_descriptors_from_records(&[arrived], Some("owner_a"), &HashMap::new(), &blur);
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

/// catches: icon scale uses global selection multiplier instead of per-system star blur.
#[test]
fn scale_bound_against_per_system_admitted_star_blur() {
    let meta = StudioGalaxyRenderMeta::default();
    let small = admitted_base_max_star_blur_world(0.5, &meta);
    let large = admitted_base_max_star_blur_world(2.0, &meta);
    assert!(large > small * 1.5);
    // Selection multiplier is not the size authority.
    assert!((meta.selected_star_scale_multiplier - 1.85).abs() < 0.01);
    assert!((clamp_fleet_icon_scale(99.0, small) - small * FLEET_ICON_MAX_STAR_BLUR_FRACTION).abs() < 1e-5);

    let stars = vec![star_view(1, 0.5), star_view(2, 2.0)];
    let blur = admitted_base_star_blur_by_system(&stars, &meta);
    let records = vec![
        rec(10, Some("a"), FleetPresenceLocation::Anchored(1)),
        rec(20, Some("a"), FleetPresenceLocation::Anchored(2)),
    ];
    let descs = fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), &blur);
    let d_small = descs.iter().find(|d| d.fleet_simthing_id_raw == 10).unwrap();
    let d_large = descs.iter().find(|d| d.fleet_simthing_id_raw == 20).unwrap();
    assert!(d_small.scale <= d_small.anchor_star_blur * FLEET_ICON_MAX_STAR_BLUR_FRACTION + 1e-5);
    assert!(d_large.scale <= d_large.anchor_star_blur * FLEET_ICON_MAX_STAR_BLUR_FRACTION + 1e-5);
    assert!(d_large.scale > d_small.scale * 1.5);
}

/// catches: production Bevy path can bypass seam while dummy test stays green.
#[test]
fn production_mesh_seam_required_and_matches_dummy_contract() {
    let blur = blur_map(&[(1, 1.5), (2, 1.5)]);
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
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur);
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
    let context = ctx(&anchors);
    let production = production_fleet_icon_render_frame(&descs, &context);
    let mut mesh = MeshOutlineFleetIconRenderer::default();
    let mesh_frame = mesh.render_descriptors(&descs, &context);
    let mut dummy = DummySecondFleetIconBackend::default();
    let dummy_frame = dummy.render_descriptors(&descs, &context);
    assert_eq!(mesh.render_calls, 1);
    assert_eq!(
        fleet_icon_frame_contract_fingerprint(&production),
        fleet_icon_frame_contract_fingerprint(&mesh_frame)
    );
    assert_eq!(
        fleet_icon_frame_contract_fingerprint(&production),
        fleet_icon_frame_contract_fingerprint(&dummy_frame)
    );
    // Wrong descriptors (bypass simulation) diverge from production contract.
    let other = fleet_icon_descriptors_from_records(
        &[rec(1, Some("owner_b"), FleetPresenceLocation::Anchored(1))],
        Some("owner_b"),
        &HashMap::new(),
        &blur,
    );
    let bypass = production_fleet_icon_render_frame(&other, &context);
    assert_ne!(
        fleet_icon_frame_contract_fingerprint(&production),
        fleet_icon_frame_contract_fingerprint(&bypass)
    );
}

/// catches: silhouette shape not data at one site / default id drift.
#[test]
fn silhouette_is_one_site_data() {
    let sil = default_fleet_icon_silhouette();
    assert_eq!(sil.id, FLEET_ICON_DEFAULT_SILHOUETTE_ID);
    assert!(sil.outline_xy.len() >= 3);
    let blur = blur_map(&[(1, 1.0)]);
    let descs = fleet_icon_descriptors_from_records(
        &[rec(1, Some("a"), FleetPresenceLocation::Anchored(1))],
        None,
        &HashMap::new(),
        &blur,
    );
    assert_eq!(descs[0].silhouette_id, sil.id);
}

/// catches: mapeditor presence map cannot feed descriptor construction (12.4 wire break).
#[test]
fn mapeditor_presence_map_feeds_descriptors() {
    let session = two_owner_session();
    let map = studio_fleet_presence_map_from_session(&session).expect("presence");
    assert!(map.total_fleets >= 2);
    let records = fleet_presence_records_flat(&map.by_system_id);
    let blur = admitted_base_star_blur_by_system(
        &session.view_model.stars,
        &session.view_model.render_meta,
    );
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur);
    assert_eq!(descs.len(), map.total_fleets);
    let rows = fleet_icon_ops_telemetry_rows(&descs);
    assert_eq!(rows.len(), descs.len());
    assert!(rows.iter().any(|r| r.side == FleetIconSide::Right));
    assert!(rows.iter().any(|r| r.side == FleetIconSide::Left));
}

/// catches: nose/plane proofs missing for production pose.
#[test]
fn nose_and_plane_legibility_for_anchored_and_transit() {
    let blur = blur_map(&[(5, 2.0), (6, 2.0)]);
    let records = vec![
        rec(1, Some("a"), FleetPresenceLocation::Anchored(5)),
        rec(2, Some("b"), FleetPresenceLocation::Anchored(5)),
        rec(
            3,
            Some("a"),
            FleetPresenceLocation::InTransit {
                source_system_id: 5,
                dest_system_id: 6,
            },
        ),
    ];
    let descs =
        fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), &blur);
    let anchors = vec![
        StudioSystemRenderAnchor {
            system_id: 5,
            structural_col: 0,
            structural_row: 0,
            world_position: [0.0, 1.0, 0.0],
            render_height: 0.0,
        },
        StudioSystemRenderAnchor {
            system_id: 6,
            structural_col: 1,
            structural_row: 0,
            world_position: [10.0, 1.0, 0.0],
            render_height: 0.0,
        },
    ];
    let frame = production_fleet_icon_render_frame(&descs, &ctx(&anchors));
    for plan in &frame.draw_plans {
        match plan.side {
            FleetIconSide::Right | FleetIconSide::Left => {
                assert!(fleet_icon_nose_faces_target(&plan.pose, [0.0, 1.0, 0.0]));
            }
            FleetIconSide::Transit => {
                assert!(fleet_icon_nose_faces_target(&plan.pose, [10.0, 1.0, 0.0]));
                assert!((plan.pose.world_position[0] - 3.0).abs() < 1e-4);
            }
        }
        assert!(fleet_icon_plane_legible_to_view(
            &plan.pose,
            [0.0, -1.0, 0.0]
        ));
    }
}

/// catches: selection flip, presence refresh, tint, zero-fleet, scene cleanup orphans.
#[test]
fn production_lifecycle_side_flip_presence_tint_cleanup() {
    let anchors = [StudioSystemRenderAnchor {
        system_id: 1,
        structural_col: 0,
        structural_row: 0,
        world_position: [0.0, 0.0, 0.0],
        render_height: 0.0,
    }];
    let blur = blur_map(&[(1, 2.0)]);
    let records = vec![
        rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(1)),
        rec(2, Some("owner_b"), FleetPresenceLocation::Anchored(1)),
    ];
    let mut scene = FleetIconSceneState::default();
    let frame_a = production_fleet_icon_render_frame(
        &fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur),
        &ctx(&anchors),
    );
    scene.apply_frame(&frame_a);
    assert_eq!(scene.by_id.len(), 2);
    assert_eq!(scene.by_id[&1].side, FleetIconSide::Right);
    assert_eq!(scene.by_id[&2].side, FleetIconSide::Left);

    let frame_b = production_fleet_icon_render_frame(
        &fleet_icon_descriptors_from_records(&records, Some("owner_b"), &HashMap::new(), &blur),
        &ctx(&anchors),
    );
    scene.apply_frame(&frame_b);
    assert_eq!(scene.by_id.len(), 2);
    assert_eq!(scene.by_id[&1].side, FleetIconSide::Left);
    assert_eq!(scene.by_id[&2].side, FleetIconSide::Right);

    // Overlapping fleet id after full scene cleanup → exactly one entity plan.
    scene.clear_for_scene_cleanup();
    assert!(scene.by_id.is_empty());
    let mut tints = HashMap::new();
    tints.insert("owner_a".into(), [0.1, 0.2, 0.9, 1.0]);
    let only = vec![rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(1))];
    scene.apply_frame(&production_fleet_icon_render_frame(
        &fleet_icon_descriptors_from_records(&only, Some("owner_a"), &tints, &blur),
        &ctx(&anchors),
    ));
    assert_eq!(scene.by_id.len(), 1);
    assert_eq!(scene.by_id[&1].tint_rgba, [0.1, 0.2, 0.9, 1.0]);

    // Structural-shell / no fleets.
    scene.apply_frame(&production_fleet_icon_render_frame(&[], &ctx(&anchors)));
    assert!(scene.by_id.is_empty());
}

/// catches: fleet icons omitted from batched galaxy scene cleanup entity list.
#[test]
fn scene_cleanup_entity_list_includes_fleet_icons() {
    let ids = galaxy_scene_cleanup_entity_ids(
        &[(7, 100)],
        &[200],
        &[(1, 300), (2, 301)],
        &[Some(400)],
        None,
        None,
    );
    assert!(ids.contains(&300) && ids.contains(&301));
    assert_eq!(ids.iter().filter(|&&e| e == 300 || e == 301).count(), 2);
}

/// catches: presentation module grows ScenarioSpec mutation or WGSL surfaces.
#[test]
fn fleet_icons_module_has_no_wgsl_or_spec_mutation_surface() {
    let source = include_str!("../src/studio_fleet_icons.rs");
    assert!(!source.contains(".wgsl"));
    assert!(source.contains("trait FleetIconRenderer"));
    assert!(source.contains("production_fleet_icon_render_frame"));
    assert!(source.contains("MeshOutlineFleetIconRenderer"));
    assert!(source.contains("select_fleet_presence_records_for_icons"));
}

/// catches: render/telemetry diverge from shared attachment-truthful source selector.
#[test]
fn render_and_telemetry_share_fleet_presence_source_selector() {
    let icons = include_str!("../src/studio_fleet_icons.rs");
    let render = include_str!("../src/app/galaxy_render.rs");
    let ui = include_str!("../src/app/ui.rs");
    assert!(icons.contains("pub fn select_fleet_presence_records_for_icons"));
    assert!(
        render.contains("select_fleet_presence_records_for_icons"),
        "sync_fleet_icons_system must call shared selector"
    );
    assert!(
        ui.contains("select_fleet_presence_records_for_icons"),
        "Studio_ops telemetry must call shared selector"
    );
    // Emptiness-based attachment inference must not reappear in production paths.
    assert!(!render.contains("fleet_presence.by_system_id.is_empty()"));
    assert!(!ui.contains("fleet_presence.by_system_id.is_empty()"));
}
