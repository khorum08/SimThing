//! STUDIO-DISRUPTION-SELECT-SCREEN-0 — selected-star disruption blur/tint screen.
//! Neutral synthetic fixture (no scenario-vocabulary owner tokens).

use std::collections::BTreeSet;
use std::path::Path;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::{
    compose_disruption_blur_scale, compose_disruption_rgb, disruption_select_screen_from_raw,
    owned_star_highlight_system_ids, selected_disruption_select_screen,
    star_visual_selected_for_owned_set, DisruptionSelectScreen, StudioDisruptionReadoutMap,
};
use simthing_spec::{
    apply_scenario_metadata_to_root, apply_star_system_display_name_metadata, make_galaxy_map,
    make_owner_entity, scenario_metadata_string_value, structural_property_value_u32,
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

fn multi_owner_spec() -> SimThingScenarioSpec {
    let foundry = make_owner_entity("foundry", "Foundry Compact", "settler");
    let union = make_owner_entity("union", "Union Compact", "raider");

    let (c1, r1) = cell_owned(1, 0, "foundry", "Gate One");
    let (c2, r2) = cell_owned(2, 1, "foundry", "Gate Two");
    let (c3, r3) = cell_owned(3, 2, "union", "Reach Three");
    let (c4, r4) = cell_unowned(4, 3, "Null Four");

    let mut map = make_galaxy_map("galaxy", "Test Galaxy");
    let map_raw = map.id.raw();
    map.add_child(c1);
    map.add_child(c2);
    map.add_child(c3);
    map.add_child(c4);

    let mut session = SimThing::new(SimThingKind::GameSession, 0);
    session.add_child(foundry);
    session.add_child(union);
    session.add_child(map);

    let mut root = SimThing::new(SimThingKind::Scenario, 0);
    root.add_child(session);

    let placements = vec![
        placement(1, 0, r1),
        placement(2, 1, r2),
        placement(3, 2, r3),
        placement(4, 3, r4),
    ];
    let mut spec = SimThingScenarioSpec {
        scenario_id: "disruption_select_screen_fixture".into(),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 4,
                height: 1,
                occupied_cells: 4,
            },
            map_container_id: map_raw.to_string(),
            placements,
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    };
    let provenance = spec.provenance.clone();
    apply_scenario_metadata_to_root(
        &mut spec.root,
        "disruption_select_screen_fixture",
        &provenance,
        1,
    );
    spec
}

const BREAKPOINT_CASES: &[(f32, f32, f32)] = &[
    (0.0, 1.0, 0.0),
    (50.0, 2.0, 0.5),
    (100.0, 5.0, 1.0),
    (100.1, 5.0, 1.0),
    (250.0, 5.0, 1.0),
    (25.0, 1.5, 0.25),
    (75.0, 3.5, 0.75),
];

#[test]
fn disruption_select_screen_breakpoints_and_above_100_clamp() {
    for &(raw, blur, red) in BREAKPOINT_CASES {
        let screen = disruption_select_screen_from_raw(raw);
        assert!(
            (screen.blur_scale - blur).abs() < 1e-6 && (screen.red_fraction - red).abs() < 1e-6,
            "raw={raw}: got blur={} red={}",
            screen.blur_scale,
            screen.red_fraction
        );
    }
}

#[test]
fn disruption_select_screen_deselect_restores_identity() {
    let screen = selected_disruption_select_screen(None, &StudioDisruptionReadoutMap::default());
    assert_eq!(screen, DisruptionSelectScreen::IDENTITY);
    assert_eq!(compose_disruption_blur_scale(1.85, true, screen), 1.85);
    assert_eq!(
        compose_disruption_rgb((0.88, 0.95, 1.0), true, screen),
        (0.88, 0.95, 1.0)
    );
}

#[test]
fn disruption_select_screen_applies_to_owned_neutral_and_hostile_selection() {
    let spec = multi_owner_spec();
    // Owned foundry, hostile union, neutral unowned — any selection is eligible.
    for selected in [1u32, 3, 4] {
        let screen = disruption_select_screen_from_raw(50.0);
        let scaled = compose_disruption_blur_scale(1.0, true, screen);
        assert!(
            (scaled - 2.0).abs() < 1e-6,
            "selected system {selected} must receive disruption blur"
        );
        let highlight = owned_star_highlight_system_ids(&spec, Some(selected));
        assert!(
            star_visual_selected_for_owned_set(selected, Some(selected), &highlight),
            "actual selected remains visually selected"
        );
    }
}

#[test]
fn disruption_select_screen_coexists_with_11_6_owned_brighten() {
    let spec = multi_owner_spec();
    let selected = 1u32;
    let highlight = owned_star_highlight_system_ids(&spec, Some(selected));
    assert_eq!(
        highlight,
        BTreeSet::from([1, 2]),
        "selecting foundry star must brighten owned set"
    );
    let screen = disruption_select_screen_from_raw(100.0);
    // Co-owned star 2: 11.6 brighten yes, disruption screen no.
    assert!(star_visual_selected_for_owned_set(2, Some(selected), &highlight));
    assert_eq!(
        compose_disruption_blur_scale(1.85, false, screen),
        1.85,
        "co-owned brighten star must keep 11.6 scale without disruption mul"
    );
    // Actual selected: both effects compose.
    assert!((compose_disruption_blur_scale(1.85, true, screen) - 1.85 * 5.0).abs() < 1e-6);
}

#[test]
fn disruption_select_screen_module_has_no_wgsl_or_scenario_mutation_surface() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/studio_disruption_select_screen.rs");
    let text = std::fs::read_to_string(&path).expect("module source");
    assert!(
        !text.contains(".wgsl") && !text.contains("include_str!"),
        "12.3 must stay presentation CPU compose — no WGSL"
    );
    assert!(
        !text.contains("scenario_authority") && !text.contains("mutate"),
        "12.3 must not touch ScenarioSpec mutation surfaces"
    );
}
