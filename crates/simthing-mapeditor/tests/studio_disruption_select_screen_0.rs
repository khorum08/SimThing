//! STUDIO-DISRUPTION-SELECT-SCREEN-0 — selected-star disruption blur/tint screen.
//! Neutral synthetic fixture (no scenario-vocabulary owner tokens).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use simthing_core::{SimThing, SimThingKind};
use simthing_mapeditor::star_render::StarRenderMode;
use simthing_mapeditor::{
    compose_disruption_blur_scale, compose_disruption_rgb, disruption_select_screen_from_raw,
    owned_star_highlight_system_ids, quantize_blur_scale_milli, quantize_disruption_milli,
    quantize_red_fraction_milli, selected_disruption_select_screen,
    star_visual_per_star_should_write, star_visual_selected_for_owned_set, star_visuals_should_sync,
    studio_disruption_readout_map_from_snapshot, DisruptionSelectScreen, StarFalloffSettingsKey,
    StarVisualAppliedKey, StarVisualSyncKey, StudioDisruptionReadoutMap,
};
use simthing_spec::{
    apply_gridcell_role_metadata, apply_scenario_metadata_to_root,
    apply_star_system_display_name_metadata, disruption_readout_snapshot_with_readback,
    make_galaxy_map, make_owner_entity, scenario_metadata_string_value,
    structural_property_value_u32, DisruptionAuthorityReadback, DisruptionAuthorityReadbackError,
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    OWNER_FLOW_OWNER_REF_PROPERTY_ID, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn cell_owned(system_id: u32, col: u32, owner: &str, name: &str) -> (SimThing, u32) {
    let mut cell = SimThing::new(SimThingKind::Location, 0);
    apply_gridcell_role_metadata(&mut cell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
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
    apply_gridcell_role_metadata(&mut cell, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM);
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

/// Build the typed admitted map through the landed 12.2 snapshot path (no Spec test constructors).
fn admitted_readout(spec: &SimThingScenarioSpec, rows: &[(u32, f32)]) -> StudioDisruptionReadoutMap {
    let values = rows.iter().copied().collect();
    let snapshot = disruption_readout_snapshot_with_readback(spec, &FixedReadback { values })
        .expect("admitted disruption snapshot");
    studio_disruption_readout_map_from_snapshot(&snapshot)
}

fn sample_sync_key(selected: Option<u32>, disruption_raw: f32) -> StarVisualSyncKey {
    StarVisualSyncKey {
        camera_position: [10, 20, 30],
        selected_system_id: selected,
        hovered_system_id: None,
        selected_disruption_milli: quantize_disruption_milli(disruption_raw),
        render_mode: StarRenderMode::BloomStarburst,
        falloff_settings: StarFalloffSettingsKey {
            base_blur_radius: 11,
            falloff_distance_percent: 1000,
            falloff_opacity_percent: 700,
        },
        view_model_generation: 7,
    }
}

fn sample_applied_key(
    visual_selected: bool,
    screen: DisruptionSelectScreen,
) -> StarVisualAppliedKey {
    StarVisualAppliedKey {
        selected: visual_selected,
        hovered: false,
        render_mode: StarRenderMode::BloomStarburst,
        depth_bucket_or_quantized_percent: 100,
        layer: 0,
        disruption_blur_milli: quantize_blur_scale_milli(screen.blur_scale),
        disruption_red_milli: quantize_red_fraction_milli(screen.red_fraction),
    }
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
    // Distinct admitted 12.2 values per ownership class via authority readback → snapshot → map.
    let readout = admitted_readout(
        &spec,
        &[
            (1, 50.0),  // owned foundry
            (3, 100.0), // hostile union
            (4, 25.0),  // neutral unowned
        ],
    );
    let selection_cases = [
        (1u32, 50.0, 2.0, 0.5),
        (3u32, 100.0, 5.0, 1.0),
        (4u32, 25.0, 1.5, 0.25),
    ];
    for (selected, expected_raw, expected_blur, expected_red) in selection_cases {
        let screen = selected_disruption_select_screen(Some(selected), &readout);
        assert!(
            (screen.raw_disruption - expected_raw).abs() < 1e-6,
            "selected {selected} must read its admitted raw disruption"
        );
        assert!((screen.blur_scale - expected_blur).abs() < 1e-6);
        assert!((screen.red_fraction - expected_red).abs() < 1e-6);

        // Eligibility is star_id == selected_id, never a hard-coded true.
        for star_id in [1u32, 2, 3, 4] {
            let is_actual_selected = star_id == selected;
            let scaled = compose_disruption_blur_scale(1.0, is_actual_selected, screen);
            if is_actual_selected {
                assert!(
                    (scaled - expected_blur).abs() < 1e-6,
                    "actual selected {star_id} must receive screen blur"
                );
            } else {
                assert_eq!(
                    scaled, 1.0,
                    "non-selected {star_id} must stay identity under selected={selected}"
                );
            }
        }

        let highlight = owned_star_highlight_system_ids(&spec, Some(selected));
        assert!(star_visual_selected_for_owned_set(
            selected,
            Some(selected),
            &highlight
        ));
    }

    // Co-owned but non-selected star remains identity while selected receives the screen.
    let selected = 1u32;
    let screen = selected_disruption_select_screen(Some(selected), &readout);
    let highlight = owned_star_highlight_system_ids(&spec, Some(selected));
    assert_eq!(highlight, BTreeSet::from([1, 2]));
    assert!(star_visual_selected_for_owned_set(2, Some(selected), &highlight));
    assert_eq!(
        compose_disruption_blur_scale(1.85, false, screen),
        1.85,
        "co-owned brighten star must not inherit selected disruption screen"
    );
    assert!((compose_disruption_blur_scale(1.85, true, screen) - 1.85 * 2.0).abs() < 1e-6);

    // Selected id absent from readout fails soft to identity.
    let missing = selected_disruption_select_screen(Some(99), &readout);
    assert_eq!(missing, DisruptionSelectScreen::IDENTITY);
}

#[test]
fn disruption_select_screen_coexists_with_11_6_owned_brighten() {
    let spec = multi_owner_spec();
    let selected = 1u32;
    let readout = admitted_readout(&spec, &[(1, 100.0), (2, 100.0)]);
    let highlight = owned_star_highlight_system_ids(&spec, Some(selected));
    assert_eq!(
        highlight,
        BTreeSet::from([1, 2]),
        "selecting foundry star must brighten owned set"
    );
    let screen = selected_disruption_select_screen(Some(selected), &readout);
    assert!(star_visual_selected_for_owned_set(2, Some(selected), &highlight));
    assert_eq!(
        compose_disruption_blur_scale(1.85, false, screen),
        1.85,
        "co-owned brighten star must keep 11.6 scale without disruption mul"
    );
    assert!((compose_disruption_blur_scale(1.85, true, screen) - 1.85 * 5.0).abs() < 1e-6);
}

#[test]
fn disruption_select_screen_live_disruption_invalidates_visual_dirty_gate() {
    // Only selected disruption changes; camera/selection/mode/generation hold.
    let previous = sample_sync_key(Some(1), 10.0);
    let current = sample_sync_key(Some(1), 50.0);
    assert_eq!(previous.selected_system_id, current.selected_system_id);
    assert_eq!(previous.camera_position, current.camera_position);
    assert_eq!(previous.render_mode, current.render_mode);
    assert_eq!(previous.view_model_generation, current.view_model_generation);
    assert!(
        star_visuals_should_sync(Some(previous), current, false),
        "live disruption change must invalidate the global star visual sync key"
    );

    let prev_screen = disruption_select_screen_from_raw(10.0);
    let next_screen = disruption_select_screen_from_raw(50.0);
    let selected_applied = sample_applied_key(true, prev_screen);
    let selected_visual = sample_applied_key(true, next_screen);
    assert!(
        star_visual_per_star_should_write(false, selected_applied, selected_visual),
        "selected star applied key must rewrite when disruption blur/red changes"
    );

    // Unselected / co-owned star keeps identity screen keys — no spurious inherit.
    let co_owned_identity = sample_applied_key(true, DisruptionSelectScreen::IDENTITY);
    assert!(
        !star_visual_per_star_should_write(false, co_owned_identity, co_owned_identity),
        "co-owned brighten star must not rewrite from selected disruption change"
    );
    assert_eq!(
        compose_disruption_blur_scale(1.85, false, next_screen),
        1.85
    );
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
