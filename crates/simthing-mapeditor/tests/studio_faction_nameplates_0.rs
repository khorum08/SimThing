//! STUDIO-FACTION-NAMEPLATES-0 — faction-colored star nameplates from authority.

use std::path::{Path, PathBuf};

use simthing_mapeditor::{
    fallback_simthing_nameplate_id, nameplate_rgba_from_color_rgb, owner_color_rgb_map_from_authority,
    star_nameplate_presentations, star_nameplate_rgba_for_gridcell, star_nameplate_rgba_for_placement,
    NEUTRAL_NAMEPLATE_RGBA,
};
use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    apply_owner_faction_identity_metadata, apply_scenario_metadata_to_root,
    apply_star_system_display_name_metadata, make_galaxy_map, make_owner_entity,
    scenario_metadata_string_value, serialize_scenario_authority, structural_property_value_u32,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, OWNER_FLOW_OWNER_REF_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn mini_spec_with_owners_and_cells() -> SimThingScenarioSpec {
    // Scenario → GameSession → { Owner terran, Owner pirate, GalaxyMap → cells }
    let mut terran = make_owner_entity("terran", "Terran Compact", "settler");
    apply_owner_faction_identity_metadata(&mut terran, (64, 160, 255), "Terran", "none");
    let mut pirate = make_owner_entity("pirate", "Pirate Cartel", "raider");
    apply_owner_faction_identity_metadata(&mut pirate, (220, 64, 48), "Pirate", "none");

    let mut owned_cell = SimThing::new(SimThingKind::Location, 0);
    owned_cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    owned_cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    owned_cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    owned_cell.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value("terran"),
    );
    apply_star_system_display_name_metadata(&mut owned_cell, "Sol Gate");
    let owned_raw = owned_cell.id.raw();

    let mut neutral_cell = SimThing::new(SimThingKind::Location, 0);
    neutral_cell.add_property(
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        structural_property_value_u32(2),
    );
    neutral_cell.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        structural_property_value_u32(1),
    );
    neutral_cell.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        structural_property_value_u32(0),
    );
    apply_star_system_display_name_metadata(&mut neutral_cell, "Deep Null");
    let neutral_raw = neutral_cell.id.raw();

    let mut map = make_galaxy_map("galaxy", "Test Galaxy");
    let map_raw = map.id.raw();
    map.add_child(owned_cell);
    map.add_child(neutral_cell);

    let mut session = SimThing::new(SimThingKind::GameSession, 0);
    session.add_child(terran);
    session.add_child(pirate);
    session.add_child(map);

    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    root.add_child(session);

    let mut spec = SimThingScenarioSpec {
        scenario_id: "nameplate_fixture".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 2,
                height: 1,
                occupied_cells: 2,
            },
            map_container_id: map_raw.to_string(),
            placements: vec![
                SimThingStructuralGridPlacement {
                    location_id: "c0".into(),
                    target_id: "t0".into(),
                    system_id: 1,
                    row: 0,
                    col: 0,
                    simthing_id_raw: owned_raw,
                },
                SimThingStructuralGridPlacement {
                    location_id: "c1".into(),
                    target_id: "t1".into(),
                    system_id: 2,
                    row: 0,
                    col: 1,
                    simthing_id_raw: neutral_raw,
                },
            ],
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    let provenance = spec.provenance.clone();
    apply_scenario_metadata_to_root(&mut spec.root, "nameplate_fixture", &provenance, 1);
    // Sanity: owners must resolve for color map.
    assert!(
        !owner_color_rgb_map_from_authority(&spec).is_empty(),
        "fixture must expose owner color_rgb via game_session_owners"
    );
    spec
}

/// catches: nameplates ignore authority star display names.
#[test]
fn faction_nameplates_use_authority_display_names() {
    let spec = mini_spec_with_owners_and_cells();
    let pres = star_nameplate_presentations(&spec);
    assert_eq!(pres.get(&1).map(|(n, _)| n.as_str()), Some("Sol Gate"));
    assert_eq!(pres.get(&2).map(|(n, _)| n.as_str()), Some("Deep Null"));
}

/// catches: owned stars not colored from owner color_rgb.
#[test]
fn faction_nameplates_apply_owner_color_rgb() {
    let spec = mini_spec_with_owners_and_cells();
    let colors = owner_color_rgb_map_from_authority(&spec);
    assert_eq!(colors.get("terran"), Some(&(64, 160, 255)));
    assert_eq!(colors.get("pirate"), Some(&(220, 64, 48)));
    let owned = star_nameplate_rgba_for_placement(
        &spec,
        &spec.structural_grid.placements[0],
        &colors,
    );
    assert_eq!(owned, nameplate_rgba_from_color_rgb((64, 160, 255)));
    let pres = star_nameplate_presentations(&spec);
    assert_eq!(
        pres.get(&1).map(|(_, c)| *c),
        Some(nameplate_rgba_from_color_rgb((64, 160, 255)))
    );
}

/// catches: unowned systems not kept neutral.
#[test]
fn faction_nameplates_unowned_are_neutral() {
    let spec = mini_spec_with_owners_and_cells();
    let colors = owner_color_rgb_map_from_authority(&spec);
    let neutral = star_nameplate_rgba_for_placement(
        &spec,
        &spec.structural_grid.placements[1],
        &colors,
    );
    assert_eq!(neutral, NEUTRAL_NAMEPLATE_RGBA);
    let pres = star_nameplate_presentations(&spec);
    assert_eq!(pres.get(&2).map(|(_, c)| *c), Some(NEUTRAL_NAMEPLATE_RGBA));
}

/// catches: presentation path mutating ScenarioSpec.
#[test]
fn faction_nameplates_do_not_mutate_scenario_spec() {
    let spec = mini_spec_with_owners_and_cells();
    let before = serialize_scenario_authority(&spec).expect("ser");
    for _ in 0..10 {
        let _ = star_nameplate_presentations(&spec);
        let _ = owner_color_rgb_map_from_authority(&spec);
    }
    let after = serialize_scenario_authority(&spec).expect("ser after");
    assert_eq!(before, after);
}

/// catches: nameplate color path becoming selection-driven (11.5 purity; 11.6 uses separate highlight helpers).
#[test]
fn faction_nameplates_colors_independent_of_selection_state() {
    let spec = mini_spec_with_owners_and_cells();
    let pres = star_nameplate_presentations(&spec);
    // Colors derive from owner_flow_owner_ref + color_rgb only — no selection argument.
    assert_eq!(
        pres.get(&1).map(|(_, c)| *c),
        Some(nameplate_rgba_from_color_rgb((64, 160, 255)))
    );
    assert_eq!(pres.get(&2).map(|(_, c)| *c), Some(NEUTRAL_NAMEPLATE_RGBA));
    // Nameplate color map is selection-free: presentations have no Selection type / StudioAppState.
    let src = include_str!("../src/studio_faction_nameplates.rs");
    assert!(!src.contains("StudioAppState"));
    assert!(!src.contains("struct Selection"));
    assert!(
        src.contains("star_nameplate_presentations"),
        "11.5 color path retained"
    );
}

/// catches: frosted glass / WGSL creep.
#[test]
fn faction_nameplates_no_frosted_glass_or_wgsl() {
    let src = include_str!("../src/studio_faction_nameplates.rs");
    assert!(!src.contains(".wgsl"));
    assert!(!src.contains("frosted"));
    assert!(!src.contains("simthing_gpu"));
}

/// catches: galaxy_render not using presentation helper.
#[test]
fn faction_nameplates_galaxy_render_uses_helper() {
    let render = include_str!("../src/app/galaxy_render.rs");
    assert!(
        render.contains("star_nameplate_presentations"),
        "galaxy_render must call star_nameplate_presentations"
    );
    assert!(
        !render.contains("[0.92, 0.96, 1.0, 1.0]"),
        "hardcoded neutral rgba must not remain as sole nameplate color"
    );
}

/// catches: fallback id formatting regressions.
#[test]
fn faction_nameplates_fallback_id_format() {
    assert_eq!(fallback_simthing_nameplate_id(0xAB), "#000000AB");
}

/// catches: gridcell without owner map entry stays neutral even with owner_ref.
#[test]
fn faction_nameplates_unknown_owner_ref_is_neutral() {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    cell.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value("unknown_faction"),
    );
    let colors = std::collections::HashMap::new();
    assert_eq!(
        star_nameplate_rgba_for_gridcell(&cell, &colors),
        NEUTRAL_NAMEPLATE_RGBA
    );
}

/// catches: authority crates not modified by this rung (source scan).
#[test]
fn faction_nameplates_no_authority_crate_imports_beyond_read() {
    let src = include_str!("../src/studio_faction_nameplates.rs");
    assert!(!src.contains("simthing_mapgenerator"));
    assert!(!src.contains("simthing_workshop"));
    assert!(!src.contains("simthing_driver"));
    // Read-only helpers from spec only.
    assert!(src.contains("owner_faction_color_rgb"));
    assert!(src.contains("owner_flow_owner_ref"));
    let _ = Path::new(".");
    let _ = PathBuf::new();
}
