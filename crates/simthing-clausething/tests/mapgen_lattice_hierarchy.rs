//! MapGen PR3 — gridcell lattice hierarchy generator tests.

use simthing_clausething::{
    MAPGEN_CANONICAL_LATTICE_EDGE, MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE, MAPGEN_MAX_LATTICE_EDGE,
    MapGenLatticeOptions, assert_allowed_simthing_kinds, collect_gridcell_location_ids,
    generate_mapgen_lattice_hierarchy, parse_mapgen_neutral_document,
    validate_fixture_lattice_edge,
};
use simthing_core::SimThingKind;

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn generate_default_hierarchy() -> simthing_clausething::MapGenLatticeHierarchy {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("generate lattice hierarchy")
}

#[test]
fn tiny_raw_fixture_generates_scenario_container_hierarchy() {
    let hierarchy = generate_default_hierarchy();
    let pack = &hierarchy.pack;

    assert_eq!(pack.scenario_id, "tiny_pentad_hub_slice_raw");
    assert_eq!(
        hierarchy.canonical_lattice_edge,
        MAPGEN_CANONICAL_LATTICE_EDGE
    );
    assert_eq!(
        hierarchy.fixture_lattice_edge,
        MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE
    );
    assert_eq!(pack.root.kind, SimThingKind::World);
    assert_eq!(pack.root_node.children.len(), 1);
    assert_eq!(pack.root_node.children[0].id, "galaxy_map");
    assert_eq!(pack.root_node.children[0].kind, SimThingKind::Location);
}

#[test]
fn gridcells_are_ordinary_location_nodes_with_mapping_role_metadata() {
    let hierarchy = generate_default_hierarchy();
    let gridcell_ids = collect_gridcell_location_ids(&hierarchy.pack.root_node);
    assert_eq!(gridcell_ids, vec!["0", "9", "31", "2", "15"]);

    for id in &gridcell_ids {
        let node = find_node(&hierarchy.pack.root_node, id).expect("gridcell node");
        assert_eq!(node.kind, SimThingKind::Location);
        assert!(node.properties.iter().any(|property| {
            property.namespace == "mapgen"
                && property.name == "mapping_role"
                && property.id.starts_with("mapgen_gridcell_mapping_role_")
                && property.description.contains("gridcell")
        }));
    }
    assert_allowed_simthing_kinds(&hierarchy.pack.root_node).expect("allowed kinds");
}

#[test]
fn authored_positions_are_inert_render_metadata_only() {
    let hierarchy = generate_default_hierarchy();
    let hub = find_node(&hierarchy.pack.root_node, "0").expect("hub system");
    assert!(hub.properties.iter().any(|property| {
        property.name == "render_position_x" && property.description.contains("inert=0")
    }));
    assert!(hub.properties.iter().any(|property| {
        property.name == "render_position_y" && property.description.contains("inert=0")
    }));
    assert!(hub.properties.iter().any(|property| {
        property.name == "render_position_z" && property.description.contains("inert=0")
    }));
}

#[test]
fn one_system_per_fixture_gridcell_is_enforced() {
    let hierarchy = generate_default_hierarchy();
    assert_eq!(
        hierarchy.pack.grid_metadata.grid_size,
        MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE
    );
    assert_eq!(hierarchy.pack.grid_metadata.placements.len(), 5);

    let mut occupied = std::collections::BTreeSet::new();
    for placement in &hierarchy.pack.grid_metadata.placements {
        assert!(occupied.insert((placement.row, placement.col)));
        assert_eq!(placement.location_id, placement.target_id);
    }
}

#[test]
fn initializer_payloads_lower_as_child_metadata_not_new_kinds() {
    let hierarchy = generate_default_hierarchy();
    let rim = find_node(&hierarchy.pack.root_node, "15").expect("rim system");
    assert_eq!(rim.children.len(), 2);
    assert_eq!(rim.children[0].kind, SimThingKind::Cohort);
    assert_eq!(rim.children[1].kind, SimThingKind::Location);
    assert!(rim.children[1].properties.iter().any(|property| {
        property.name == "deposit_minerals_authored" && property.description.contains("inert=4")
    }));
}

#[test]
fn generator_emits_no_rf_palma_commitment_or_field_operator_surfaces() {
    let pack = &generate_default_hierarchy().pack;
    assert!(pack.w_impedance_compose.is_none());
    assert!(pack.stress_compose.is_none());
    assert!(pack.palma_feedstock.is_none());
    assert!(pack.commitment.is_none());
    assert!(pack.grid_metadata.links.is_empty());
}

#[test]
fn zero_or_negative_fixture_lattice_edge_is_rejected() {
    assert!(validate_fixture_lattice_edge(0).is_err());
}

#[test]
fn fixture_lattice_edge_beyond_cap_is_rejected() {
    assert!(validate_fixture_lattice_edge(MAPGEN_MAX_LATTICE_EDGE + 1).is_err());
}

#[test]
fn system_count_beyond_fixture_capacity_is_rejected() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    let err = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: 2,
        },
    )
    .expect_err("2x2 lattice cannot host five systems");
    assert!(err.message.contains("capacity"));
}

#[test]
fn generated_properties_reject_forbidden_movement_vocabulary() {
    let pack = &generate_default_hierarchy().pack;
    for property in &pack.game_mode.properties {
        let haystack = format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        );
        for forbidden in [
            "route",
            "pathfinding",
            "predecessor",
            "movement_order",
            "border_service",
            "frontline",
            "cpu_planner",
            "fleet_path",
        ] {
            assert!(
                !haystack.contains(forbidden),
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            );
        }
    }
}

fn find_node<'a>(
    node: &'a simthing_clausething::HydratedScenarioNode,
    id: &str,
) -> Option<&'a simthing_clausething::HydratedScenarioNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}
