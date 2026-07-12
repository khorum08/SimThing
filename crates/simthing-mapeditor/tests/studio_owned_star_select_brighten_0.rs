//! STUDIO-OWNED-STAR-SELECT-BRIGHTEN-0 — owned-set star visual highlight (presentation only).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    nameplate_rgba_from_color_rgb, owned_star_highlight_system_ids, selected_owner_id_for_system,
    star_nameplate_presentations, star_owner_id_by_system_id, star_ownership_presentations,
    star_visual_selected_for_owned_set, NEUTRAL_NAMEPLATE_RGBA,
};
use simthing_spec::{
    apply_owner_faction_identity_metadata, apply_scenario_metadata_to_root,
    apply_star_system_display_name_metadata, make_galaxy_map, make_owner_entity,
    scenario_metadata_string_value, serialize_scenario_authority, structural_property_value_u32,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, OWNER_FLOW_OWNER_REF_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn cell_owned(system_id: u32, col: u32, owner: &str, name: &str) -> (SimThing, u32) {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    cell.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value(owner),
    );
    apply_star_system_display_name_metadata(&mut cell, name);
    let raw = cell.id.raw();
    (cell, raw)
}

fn cell_unowned(system_id: u32, col: u32, name: &str) -> (SimThing, u32) {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(system_id),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(col),
    );
    cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    apply_star_system_display_name_metadata(&mut cell, name);
    let raw = cell.id.raw();
    (cell, raw)
}

/// Two terran-owned, one pirate-owned, two unowned systems.
fn multi_owner_spec() -> SimThingScenarioSpec {
    let mut terran = make_owner_entity("terran", "Terran Compact", "settler");
    apply_owner_faction_identity_metadata(&mut terran, (64, 160, 255), "Terran", "none");
    let mut pirate = make_owner_entity("pirate", "Pirate Cartel", "raider");
    apply_owner_faction_identity_metadata(&mut pirate, (220, 64, 48), "Pirate", "none");

    let (c1, r1) = cell_owned(1, 0, "terran", "Sol Gate");
    let (c2, r2) = cell_owned(2, 1, "terran", "Terra Nova");
    let (c3, r3) = cell_owned(3, 2, "pirate", "Corsair Reach");
    let (c4, r4) = cell_unowned(4, 3, "Deep Null");
    let (c5, r5) = cell_unowned(5, 4, "Empty Drift");

    let mut map = make_galaxy_map("galaxy", "Test Galaxy");
    let map_raw = map.id.raw();
    map.add_child(c1);
    map.add_child(c2);
    map.add_child(c3);
    map.add_child(c4);
    map.add_child(c5);

    let mut session = SimThing::new(SimThingKind::GameSession, 0);
    session.add_child(terran);
    session.add_child(pirate);
    session.add_child(map);

    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    root.add_child(session);

    let placements = vec![
        placement(1, 0, r1),
        placement(2, 1, r2),
        placement(3, 2, r3),
        placement(4, 3, r4),
        placement(5, 4, r5),
    ];
    let mut spec = SimThingScenarioSpec {
        scenario_id: "owned_brighten_fixture".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 5,
                height: 1,
                occupied_cells: 5,
            },
            map_container_id: map_raw.to_string(),
            placements,
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    let provenance = spec.provenance.clone();
    apply_scenario_metadata_to_root(&mut spec.root, "owned_brighten_fixture", &provenance, 1);
    spec
}

fn placement(system_id: u32, col: u32, raw: u32) -> SimThingStructuralGridPlacement {
    SimThingStructuralGridPlacement {
        location_id: format!("c{system_id}"),
        target_id: format!("t{system_id}"),
        system_id,
        row: 0,
        col,
        simthing_id_raw: raw,
    }
}

/// catches: no co-owned highlight set when selecting an owned star.
#[test]
fn owned_star_select_brighten_builds_owner_set_for_selected_owned_star() {
    let spec = multi_owner_spec();
    let set = owned_star_highlight_system_ids(&spec, Some(1));
    assert_eq!(set, BTreeSet::from([1, 2]));
    let set2 = owned_star_highlight_system_ids(&spec, Some(2));
    assert_eq!(set2, BTreeSet::from([1, 2]));
    let pirate = owned_star_highlight_system_ids(&spec, Some(3));
    assert_eq!(pirate, BTreeSet::from([3]));
}

/// catches: None-owner treated as a faction group.
#[test]
fn owned_star_select_brighten_does_not_group_unowned_stars() {
    let spec = multi_owner_spec();
    let set = owned_star_highlight_system_ids(&spec, Some(4));
    assert!(set.is_empty(), "unowned select must not group unowned stars: {set:?}");
    let set5 = owned_star_highlight_system_ids(&spec, Some(5));
    assert!(set5.is_empty());
    assert!(selected_owner_id_for_system(&spec, Some(4)).is_none());
}

/// catches: stale owned set after deselect.
#[test]
fn owned_star_select_brighten_deselect_clears_set() {
    let spec = multi_owner_spec();
    let set = owned_star_highlight_system_ids(&spec, Some(1));
    assert!(!set.is_empty());
    let cleared = owned_star_highlight_system_ids(&spec, None);
    assert!(cleared.is_empty());
    assert!(selected_owner_id_for_system(&spec, None).is_none());
}

/// catches: ownership inferred from color or display name.
#[test]
fn owned_star_select_brighten_uses_owner_flow_owner_ref_not_color_or_name() {
    let spec = multi_owner_spec();
    let by_id = star_owner_id_by_system_id(&spec);
    assert_eq!(by_id.get(&1).map(String::as_str), Some("terran"));
    assert_eq!(by_id.get(&3).map(String::as_str), Some("pirate"));
    assert!(!by_id.contains_key(&4));
    let pres = star_ownership_presentations(&spec);
    // Same color would not invent ownership; owner_id is the authority key.
    assert_eq!(pres.get(&1).and_then(|p| p.owner_id.as_deref()), Some("terran"));
    assert_eq!(pres.get(&2).and_then(|p| p.owner_id.as_deref()), Some("terran"));
    assert_ne!(
        pres.get(&1).map(|p| p.display_name.as_str()),
        pres.get(&2).map(|p| p.display_name.as_str())
    );
    // Source scan: helpers read owner_flow_owner_ref, not color-matching ownership.
    let src = include_str!("../src/studio_faction_nameplates.rs");
    assert!(src.contains("owner_flow_owner_ref"));
    assert!(src.contains("owned_star_highlight_system_ids"));
}

/// catches: selection model converted into faction multi-select.
#[test]
fn owned_star_select_brighten_preserves_actual_selected_system() {
    let selection_src = include_str!("../src/selection.rs");
    assert!(
        selection_src.contains("selected_system_id"),
        "selection model must remain single system id"
    );
    // Highlight set is a presentation set; actual selected stays one id.
    let spec = multi_owner_spec();
    let highlight = owned_star_highlight_system_ids(&spec, Some(1));
    assert!(highlight.len() > 1);
    let actual_selected = Some(1u32);
    // Only system 1 is "actual"; co-owned 2 is visual-only.
    assert!(star_visual_selected_for_owned_set(1, actual_selected, &highlight));
    assert!(star_visual_selected_for_owned_set(2, actual_selected, &highlight));
    assert!(!star_visual_selected_for_owned_set(3, actual_selected, &highlight));
    // Actual selected equality is still single-id.
    assert_eq!(actual_selected, Some(1));
    assert_ne!(actual_selected, Some(2));
}

/// catches: all co-owned labels becoming focused.
#[test]
fn owned_star_select_brighten_preserves_nameplate_focus_actual_selected_or_hovered_only() {
    let focus = include_str!("../src/app/galaxy_render.rs");
    assert!(
        focus.contains("Nameplate focus stays actual selected/hovered only"),
        "nameplate focus system must document actual-only focus"
    );
    // Focus path uses selection.selected_system_id / hovered only — not owned_highlight.
    let focus_fn = focus
        .split("fn sync_star_nameplate_focus_system")
        .nth(1)
        .expect("focus system present");
    let focus_body = focus_fn.split("fn ").next().unwrap_or(focus_fn);
    assert!(
        !focus_body.contains("owned_star_highlight"),
        "nameplate focus must not consume owned highlight set"
    );
    assert!(
        focus_body.contains("selected_system_id") && focus_body.contains("hovered_system_id"),
        "focus remains actual selection/hover"
    );
}

/// catches: regression of owner-color nameplates.
#[test]
fn owned_star_select_brighten_preserves_11_5_owner_nameplate_colors() {
    let spec = multi_owner_spec();
    let pres = star_nameplate_presentations(&spec);
    assert_eq!(
        pres.get(&1).map(|(_, c)| *c),
        Some(nameplate_rgba_from_color_rgb((64, 160, 255)))
    );
    assert_eq!(
        pres.get(&3).map(|(_, c)| *c),
        Some(nameplate_rgba_from_color_rgb((220, 64, 48)))
    );
    assert_eq!(pres.get(&4).map(|(_, c)| *c), Some(NEUTRAL_NAMEPLATE_RGBA));
    assert_eq!(pres.get(&1).map(|(n, _)| n.as_str()), Some("Sol Gate"));
}

/// catches: presentation path mutating ScenarioSpec.
#[test]
fn owned_star_select_brighten_no_spec_mutation() {
    let spec = multi_owner_spec();
    let before = serialize_scenario_authority(&spec).expect("ser");
    for sel in [Some(1u32), Some(4), None, Some(2)] {
        let _ = owned_star_highlight_system_ids(&spec, sel);
        let _ = star_ownership_presentations(&spec);
        let _ = selected_owner_id_for_system(&spec, sel);
    }
    let after = serialize_scenario_authority(&spec).expect("ser after");
    assert_eq!(before, after);
}

/// catches: 11.7/GPU creep.
#[test]
fn owned_star_select_brighten_no_wgsl_or_gpu_pipeline() {
    let src = include_str!("../src/studio_faction_nameplates.rs");
    let render = include_str!("../src/app/galaxy_render.rs");
    for banned in [".wgsl", "frosted", "simthing_gpu", "simthing-gpu"] {
        assert!(!src.contains(banned), "nameplates module: {banned}");
        assert!(!render.contains(banned), "galaxy_render: {banned}");
    }
}

/// catches: source_base / loader / telemetry regression (11.4 surface still present).
#[test]
fn owned_star_select_brighten_11_4_loader_regression() {
    let ingest = include_str!("../src/clause_scenario_ingest.rs");
    assert!(
        ingest.contains("hydrate_scenario_with_source_base")
            || ingest.contains("source_base"),
        "11.4 source_base wire must remain in clause ingest"
    );
    let library = include_str!("../src/studio_scenario_library_ui.rs");
    assert!(
        library.contains("StudioScenarioTelemetryReadout")
            || library.contains("build_studio_scenario_telemetry_readout"),
        "11.4 scenario telemetry readout must remain"
    );
    let _ = Path::new(".");
    let _ = PathBuf::new();
}

/// catches: star visual sync not wiring owned highlight (render path bypass).
#[test]
fn owned_star_select_brighten_star_visual_sync_uses_owned_highlight() {
    let render = include_str!("../src/app/galaxy_render.rs");
    assert!(
        render.contains("owned_star_highlight_system_ids"),
        "sync_star_visuals_system must compute owned highlight set"
    );
    assert!(
        render.contains("star_visual_selected_for_owned_set"),
        "sync must use visual_selected for co-owned brightness"
    );
    // Keep wire on admitted galaxy_render surface (not app/picking.rs class gap).
    assert!(
        render.contains("fn sync_star_visuals_system"),
        "star visual sync lives on galaxy_render admitted surface"
    );
}
