use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    HydratedOwnedSystem, HydratedOwnershipVolume, HydratedScenarioPack, hydrate_scenario,
    parse_raw_document,
};
use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    OWNER_FLOW_OWNER_REF_PROPERTY_ID, game_session_child, game_session_galaxy_map,
    game_session_owners, owner_entity_id, owner_flow_owner_ref, scenario_metadata_string_value,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn fixture_path_text() -> String {
    fixture_path().to_string_lossy().replace('\\', "/")
}

fn combined_clause() -> String {
    format!(
        r#"
scenario = tp_ownership_columns_0 {{
    metadata = {{
        display_name = "TP Ownership Columns 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
    }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 200
        selection = chebyshev_contiguous
        seed = 770421
        anchor_row = 199
        anchor_col = 80
    }}
    ownership_volume = pirate_border {{
        owner = "pirate"
        count = 50
        selection = chebyshev_contiguous
        adjacent_to = "terran_core"
        seed = 770421
    }}
}}
"#,
        fixture_path_text()
    )
}

fn hydrate_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(combined_clause().as_bytes()).expect("parse combined clause");
    hydrate_scenario(&document).expect("hydrate ownership columns clause")
}

fn authority_root(pack: &HydratedScenarioPack) -> SimThing {
    pack.authority_root
        .clone()
        .expect("ownership pack carries authority root")
}

fn owner_counts(pack: &HydratedScenarioPack) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for volume in &pack.ownership_volumes {
        *counts.entry(volume.owner.clone()).or_default() += volume.assigned_systems.len();
    }
    counts
}

fn volume<'a>(pack: &'a HydratedScenarioPack, id: &str) -> &'a HydratedOwnershipVolume {
    pack.ownership_volumes
        .iter()
        .find(|volume| volume.id == id)
        .expect("volume exists")
}

#[test]
fn embedded_base_owner_siblings_and_ownership_volumes_parse() {
    let pack = hydrate_pack();

    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.owners.len(), 2);
    assert_eq!(pack.ownership_volumes.len(), 2);
    assert_eq!(volume(&pack, "terran_core").owner, "terran");
    assert_eq!(volume(&pack, "pirate_border").owner, "pirate");
}

#[test]
fn ownership_counts_are_200_50_1250_and_non_overlapping() {
    let pack = hydrate_pack();
    let counts = owner_counts(&pack);
    let assigned = pack
        .ownership_volumes
        .iter()
        .flat_map(|volume| volume.assigned_systems.iter())
        .collect::<Vec<_>>();
    let unique_targets: BTreeSet<_> = assigned
        .iter()
        .map(|system| system.target_id.as_str())
        .collect();

    assert_eq!(counts.get("terran").copied(), Some(200));
    assert_eq!(counts.get("pirate").copied(), Some(50));
    assert_eq!(
        assigned.len(),
        unique_targets.len(),
        "no system has two owners"
    );
    assert_eq!(1500 - assigned.len(), 1250);
}

#[test]
fn terran_and_pirate_volumes_are_chebyshev_contiguous_and_adjacent() {
    let pack = hydrate_pack();
    let terran = volume(&pack, "terran_core");
    let pirate = volume(&pack, "pirate_border");

    assert!(is_chebyshev_neighborhood_prefix(
        terran,
        &pack.grid_metadata.placements
    ));
    assert!(is_nearest_unassigned_border_prefix(
        pirate,
        terran,
        &pack.grid_metadata.placements
    ));
    assert!(are_chebyshev_adjacent(
        &terran.assigned_systems,
        &pirate.assigned_systems
    ));
}

#[test]
fn owner_references_resolve_to_gamesession_sibling_owners() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let scenario = scenario_from_root(root);
    let owners: BTreeSet<_> = game_session_owners(&scenario)
        .expect("GameSession owners")
        .into_iter()
        .filter_map(owner_entity_id)
        .collect();

    for system in pack
        .ownership_volumes
        .iter()
        .flat_map(|volume| volume.assigned_systems.iter())
    {
        assert!(owners.contains(&system.owner_ref));
    }
}

#[test]
fn owned_gridcells_remain_under_galaxymap_not_owners() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let scenario = scenario_from_root(root);
    let game_session = game_session_child(&scenario).expect("GameSession child");
    let galaxy_map = game_session_galaxy_map(&scenario).expect("GalaxyMap sibling");
    let owner_children = game_session
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Owner)
        .collect::<Vec<_>>();
    let owned_under_map = galaxy_map
        .children
        .iter()
        .filter(|child| owner_flow_owner_ref(child).is_some())
        .count();

    assert_eq!(galaxy_map.children.len(), 1500);
    assert_eq!(owned_under_map, 250);
    assert!(owner_children.iter().all(|owner| owner.children.is_empty()));
}

#[test]
fn galaxymap_remains_gamesession_sibling() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let scenario = scenario_from_root(root);
    let game_session = game_session_child(&scenario).expect("GameSession child");
    let galaxy_map = game_session_galaxy_map(&scenario).expect("GalaxyMap sibling");

    assert_eq!(game_session.children.len(), 3);
    assert_eq!(galaxy_map.kind, SimThingKind::Location);
}

#[test]
fn unknown_owner_reference_hard_errors_with_span() {
    let source = combined_clause().replace("owner = \"pirate\"", "owner = \"unknown\"");
    let document = parse_raw_document(source.as_bytes()).expect("parse unknown owner source");
    let err = hydrate_scenario(&document).expect_err("unknown owner must hard-error");

    assert!(
        err.to_string().contains("references unknown owner"),
        "{err}"
    );
    assert!(err.span.is_some(), "unknown-owner error must carry a span");
}

#[test]
fn overlapping_ownership_selections_hard_error_with_span() {
    let source = format!(
        r#"
scenario = overlapping_ownership {{
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{ owner_key = "terran" display_name = "Terran Compact" }}
    owner = pirate {{ owner_key = "pirate" display_name = "Pirate Cartel" }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 10
        selection = chebyshev_contiguous
        seed = 1
    }}
    ownership_volume = pirate_overlap {{
        owner = "pirate"
        count = 10
        selection = chebyshev_contiguous
        seed = 1
    }}
}}
"#,
        fixture_path_text()
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse overlap source");
    let err = hydrate_scenario(&document).expect_err("overlap must hard-error");

    assert!(err.to_string().contains("overlaps"), "{err}");
    assert!(err.span.is_some(), "overlap error must carry a span");
}

#[test]
fn capture_as_column_flip_preserves_id_parentage_and_children() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let scenario = scenario_from_root(root.clone());
    let game_session = game_session_child(&scenario).expect("GameSession child");
    let galaxy_map = game_session_galaxy_map(&scenario).expect("GalaxyMap sibling");
    let terran_gridcell = galaxy_map
        .children
        .iter()
        .find(|child| owner_flow_owner_ref(child).as_deref() == Some("terran"))
        .expect("terran gridcell");
    let original_id = terran_gridcell.id;
    let original_child_count = terran_gridcell.children.len();
    let owner_child_counts = game_session
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Owner)
        .map(|owner| owner.children.len())
        .collect::<Vec<_>>();

    let mut flipped = terran_gridcell.clone();
    flipped.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value("pirate"),
    );

    assert_eq!(flipped.id, original_id);
    assert_eq!(flipped.children.len(), original_child_count);
    assert_eq!(owner_flow_owner_ref(&flipped).as_deref(), Some("pirate"));
    assert!(owner_child_counts.iter().all(|count| *count == 0));
}

fn scenario_from_root(root: SimThing) -> simthing_spec::SimThingScenarioSpec {
    simthing_spec::SimThingScenarioSpec {
        scenario_id: "tp_ownership_columns_0".to_string(),
        root,
        structural_grid: simthing_spec::SimThingScenarioGrid {
            frame: simthing_spec::SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: simthing_spec::SimThingScenarioProvenance::default(),
    }
}

fn is_chebyshev_neighborhood_prefix(
    volume: &HydratedOwnershipVolume,
    placements: &[simthing_clausething::HydratedScenarioGridPlacement],
) -> bool {
    if volume.assigned_systems.is_empty() {
        return false;
    }
    let selected: BTreeSet<_> = volume
        .assigned_systems
        .iter()
        .map(|system| system.target_id.as_str())
        .collect();
    let max_selected_distance = volume
        .assigned_systems
        .iter()
        .map(|system| {
            chebyshev_distance(
                (system.row, system.col),
                (volume.anchor_row, volume.anchor_col),
            )
        })
        .max()
        .expect("selected systems");
    placements.iter().all(|placement| {
        selected.contains(placement.target_id.as_str())
            || chebyshev_distance(
                (placement.row, placement.col),
                (volume.anchor_row, volume.anchor_col),
            ) >= max_selected_distance
    })
}

fn are_chebyshev_adjacent(left: &[HydratedOwnedSystem], right: &[HydratedOwnedSystem]) -> bool {
    let right_coords: BTreeSet<_> = right
        .iter()
        .map(|system| (system.row, system.col))
        .collect();
    left.iter().any(|system| {
        chebyshev_neighbors(system.row, system.col)
            .iter()
            .any(|coord| right_coords.contains(coord))
    })
}

fn is_nearest_unassigned_border_prefix(
    volume: &HydratedOwnershipVolume,
    reference: &HydratedOwnershipVolume,
    placements: &[simthing_clausething::HydratedScenarioGridPlacement],
) -> bool {
    let selected: BTreeSet<_> = volume
        .assigned_systems
        .iter()
        .map(|system| system.target_id.as_str())
        .collect();
    let reference_targets: BTreeSet<_> = reference
        .assigned_systems
        .iter()
        .map(|system| system.target_id.as_str())
        .collect();
    let reference_coords = reference
        .assigned_systems
        .iter()
        .map(|system| (system.row, system.col))
        .collect::<Vec<_>>();
    let max_selected_distance = volume
        .assigned_systems
        .iter()
        .map(|system| min_distance_to_coords((system.row, system.col), &reference_coords))
        .max()
        .expect("selected systems");

    placements.iter().all(|placement| {
        selected.contains(placement.target_id.as_str())
            || reference_targets.contains(placement.target_id.as_str())
            || min_distance_to_coords((placement.row, placement.col), &reference_coords)
                >= max_selected_distance
    })
}

fn min_distance_to_coords(coord: (u32, u32), coords: &[(u32, u32)]) -> u32 {
    coords
        .iter()
        .map(|other| chebyshev_distance(coord, *other))
        .min()
        .unwrap_or(u32::MAX)
}

fn chebyshev_neighbors(row: u32, col: u32) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::new();
    for dr in -1_i32..=1 {
        for dc in -1_i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let Some(next_row) = offset_u32(row, dr) else {
                continue;
            };
            let Some(next_col) = offset_u32(col, dc) else {
                continue;
            };
            neighbors.push((next_row, next_col));
        }
    }
    neighbors
}

fn chebyshev_distance(left: (u32, u32), right: (u32, u32)) -> u32 {
    left.0.abs_diff(right.0).max(left.1.abs_diff(right.1))
}

fn offset_u32(value: u32, delta: i32) -> Option<u32> {
    if delta.is_negative() {
        value.checked_sub(delta.unsigned_abs())
    } else {
        value.checked_add(delta as u32)
    }
}
