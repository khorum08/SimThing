//! MapGen PR5 — bounded hyperlane-to-link and lane-coupling tests.

use simthing_clausething::{
    MAPGEN_RF_DEPOSIT_ARENA, MapGenLatticeOptions, MapGenLinksOptions, MapGenResourceFlowOptions,
    extract_hyperlane_declarations, generate_default_mapgen_links_enrollment,
    generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_resource_flow_enrollment, lower_hyperlane_topology,
    parse_mapgen_neutral_document,
};
use simthing_core::SimThingKind;

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn default_rf_enrollment() -> simthing_clausething::MapGenResourceFlowEnrollment {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("generate lattice hierarchy");
    generate_mapgen_resource_flow_enrollment(&hierarchy, MapGenResourceFlowOptions::default())
        .expect("generate RF enrollment")
}

fn default_links_enrollment() -> simthing_clausething::MapGenLinksEnrollment {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    let rf = default_rf_enrollment();
    generate_mapgen_links(&rf, &neutral, MapGenLinksOptions::default()).expect("generate links")
}

#[test]
fn duplicate_hyperlane_is_canonicalized_deterministically() {
    let rf = default_rf_enrollment();
    let enrollment = lower_hyperlane_topology(
        &rf.pack,
        &[("0".into(), "9".into()), ("9".into(), "0".into())],
        MapGenLinksOptions::default(),
    )
    .expect("lower duplicate pair");
    assert_eq!(enrollment.pack.grid_metadata.links.len(), 1);
    assert_eq!(enrollment.expansion_report.duplicate_link_rejections, 1);
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
