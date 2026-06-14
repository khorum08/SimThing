//! 0.0.8.2.5 MapGen lowerer child-id amendment — shared initializer collision guard.

use std::collections::BTreeSet;

use simthing_clausething::{
    MapGenLatticeOptions, generate_mapgen_lattice_hierarchy, parse_mapgen_neutral_document,
};
use simthing_core::SimThingKind;

const PENTAD_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

const SHARED_INITIALIZER_TWO_SYSTEMS: &str = r#"
shared_initializer_child_id_slice = {
    static_galaxy_scenario = {
        name = "Shared Initializer Child ID Slice"
        random_hyperlanes = no
        system = {
            id = "0"
            name = ""
            position = { x = 0 y = 0 z = 0 }
            initializer = example_rim_initializer
        }
        system = {
            id = "1"
            name = ""
            position = { x = 1 y = 1 z = 0 }
            initializer = example_rim_initializer
        }
    }
    example_rim_initializer = {
        name = "Example Rim"
        planet = { count = 1 }
        deposit = {
            resources = { minerals = 4 }
        }
    }
}
"#;

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

fn collect_node_ids(node: &simthing_clausething::HydratedScenarioNode, ids: &mut BTreeSet<String>) {
    ids.insert(node.id.clone());
    for child in &node.children {
        collect_node_ids(child, ids);
    }
}

fn lower(text: &str) -> simthing_clausething::MapGenLatticeHierarchy {
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse neutral AST");
    generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: 3,
        },
    )
    .expect("generate lattice hierarchy")
}

#[test]
fn pentad_fixture_still_lowers_after_child_id_amendment() {
    let hierarchy = lower(PENTAD_FIXTURE);
    assert_eq!(hierarchy.pack.scenario_id, "tiny_pentad_hub_slice_raw");
    assert_eq!(hierarchy.pack.grid_metadata.placements.len(), 5);
}

#[test]
fn two_systems_sharing_one_initializer_lower_without_child_id_collisions() {
    let hierarchy = lower(SHARED_INITIALIZER_TWO_SYSTEMS);
    let mut ids = BTreeSet::new();
    collect_node_ids(&hierarchy.pack.root_node, &mut ids);
    assert_eq!(ids.len(), count_nodes(&hierarchy.pack.root_node));
}

fn count_nodes(node: &simthing_clausething::HydratedScenarioNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

#[test]
fn shared_initializer_planet_child_ids_are_unique_and_system_scoped() {
    let hierarchy = lower(SHARED_INITIALIZER_TWO_SYSTEMS);
    let system0 = find_node(&hierarchy.pack.root_node, "0").expect("system 0");
    let system1 = find_node(&hierarchy.pack.root_node, "1").expect("system 1");
    assert_eq!(system0.children.len(), 2);
    assert_eq!(system1.children.len(), 2);
    assert_eq!(system0.children[0].id, "0_example_rim_initializer_planet");
    assert_eq!(system1.children[0].id, "1_example_rim_initializer_planet");
    assert_eq!(system0.children[0].kind, SimThingKind::Cohort);
    assert_eq!(system1.children[0].kind, SimThingKind::Cohort);
}

#[test]
fn shared_initializer_deposit_child_ids_are_unique_and_system_scoped() {
    let hierarchy = lower(SHARED_INITIALIZER_TWO_SYSTEMS);
    let system0 = find_node(&hierarchy.pack.root_node, "0").expect("system 0");
    let system1 = find_node(&hierarchy.pack.root_node, "1").expect("system 1");
    assert_eq!(system0.children[1].id, "0_example_rim_initializer_deposit");
    assert_eq!(system1.children[1].id, "1_example_rim_initializer_deposit");
    assert_eq!(system0.children[1].kind, SimThingKind::Location);
    assert_eq!(system1.children[1].kind, SimThingKind::Location);
}
