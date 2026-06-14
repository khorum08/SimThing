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
fn pr2_adapter_and_pr3_hierarchy_and_pr4_rf_still_succeed_before_pr5() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    assert!(neutral.source_byte_len > 0);
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 hierarchy");
    default_rf_enrollment();
}

#[test]
fn tiny_fixture_generates_bounded_links_and_lane_couplings() {
    let enrollment = default_links_enrollment();
    assert_eq!(enrollment.pack.grid_metadata.links.len(), 3);
    assert_eq!(enrollment.lane_couplings.len(), 2);
    assert_eq!(enrollment.expansion_report.link_count, 3);
    assert_eq!(enrollment.expansion_report.lane_coupling_count, 2);
}

#[test]
fn generated_links_reference_existing_gridcell_ids() {
    let enrollment = default_links_enrollment();
    let gridcells = simthing_clausething::collect_gridcell_location_ids(&enrollment.pack.root_node);
    for link in &enrollment.pack.grid_metadata.links {
        assert!(
            gridcells.contains(&link.from),
            "link from `{from}`",
            from = link.from
        );
        assert!(gridcells.contains(&link.to), "link to `{to}`", to = link.to);
    }
    for coupling in &enrollment.lane_couplings {
        assert!(gridcells.contains(&coupling.from));
        assert!(gridcells.contains(&coupling.to));
    }
}

#[test]
fn n4_adjacent_hyperlanes_lower_to_scenario_links() {
    let enrollment = default_links_enrollment();
    let links: Vec<_> = enrollment
        .pack
        .grid_metadata
        .links
        .iter()
        .map(|link| (link.from.as_str(), link.to.as_str()))
        .collect();
    assert!(links.contains(&("0", "2")));
    assert!(links.contains(&("0", "9")));
    assert!(links.contains(&("15", "9")));
}

#[test]
fn long_range_hyperlanes_lower_to_lane_couplings_not_links() {
    let enrollment = default_links_enrollment();
    let couplings: Vec<_> = enrollment
        .lane_couplings
        .iter()
        .map(|c| (c.from.as_str(), c.to.as_str()))
        .collect();
    assert!(couplings.contains(&("0", "31")));
    assert!(couplings.contains(&("15", "31")));
    for link in &enrollment.pack.grid_metadata.links {
        assert_ne!(link.from, "31");
        assert_ne!(link.to, "31");
    }
}

#[test]
fn authored_render_positions_remain_inert_metadata() {
    let enrollment = default_links_enrollment();
    let system_31 = find_node(&enrollment.pack.root_node, "31").expect("system 31");
    let render_x = system_31
        .properties
        .iter()
        .find(|p| p.name.starts_with("render_position_x_"))
        .expect("render x");
    assert_eq!(render_x.description, "inert=-9");
    assert!(
        enrollment
            .lane_couplings
            .iter()
            .any(|c| c.from == "0" && c.to == "31"),
        "lane coupling uses lattice authority, not render coordinates"
    );
}

#[test]
fn expansion_report_declares_caps_and_rejection_counts() {
    let enrollment = default_links_enrollment();
    let report = &enrollment.expansion_report;
    assert_eq!(report.max_links, 8);
    assert_eq!(report.max_lane_coupling_count, 8);
    assert_eq!(report.max_per_node_fanout, 4);
    assert_eq!(report.max_lane_coupling_fanout, 4);
    assert_eq!(report.unknown_endpoint_rejections, 0);
    assert_eq!(report.self_link_rejections, 0);
    assert_eq!(report.duplicate_link_rejections, 0);
    assert!(report.per_node_fanout.get("0").copied().unwrap_or(0) >= 3);
}

#[test]
fn generated_output_preserves_pr4_rf_and_ordinary_hierarchy() {
    let enrollment = default_links_enrollment();
    assert_eq!(enrollment.pack.root.kind, SimThingKind::World);
    assert!(enrollment.pack.game_mode.resource_flow.is_some());
    assert!(enrollment.pack.w_impedance_compose.is_none());
    assert!(enrollment.pack.stress_compose.is_none());
    assert!(enrollment.pack.palma_feedstock.is_none());
    assert!(enrollment.pack.commitment.is_none());
    assert!(
        enrollment
            .pack
            .game_mode
            .resource_flow
            .as_ref()
            .unwrap()
            .arenas
            .iter()
            .any(|arena| arena.name == MAPGEN_RF_DEPOSIT_ARENA)
    );
}

#[test]
fn generated_properties_reject_forbidden_movement_vocabulary() {
    let enrollment = default_links_enrollment();
    for property in &enrollment.pack.game_mode.properties {
        let haystack = format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        );
        for forbidden in [
            "route",
            "path",
            "pathfinding",
            "predecessor",
            "movement_order",
            "border",
            "frontline",
        ] {
            assert!(
                !haystack.contains(forbidden),
                "generated property must not reference `{forbidden}`"
            );
        }
    }
}

#[test]
fn convenience_default_pipeline_succeeds() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_default_mapgen_links_enrollment(&neutral).expect("default pipeline");
}

#[test]
fn pr5_source_has_no_euclidean_adjacency_authority() {
    let source = include_str!("../src/mapgen_links.rs");
    for forbidden in [
        "distance",
        "magnitude",
        "norm(",
        "sqrt",
        "length(",
        "normalize(",
        "hypot",
        "euclidean",
    ] {
        assert!(
            !source.contains(forbidden),
            "mapgen_links.rs must not reference Euclidean authority `{forbidden}`"
        );
    }
}

#[test]
fn unknown_endpoint_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[("0".into(), "missing".into())],
        MapGenLinksOptions::default(),
    )
    .unwrap_err();
    assert!(err.message.contains("unknown gridcell endpoint"));
}

#[test]
fn self_link_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[("0".into(), "0".into())],
        MapGenLinksOptions::default(),
    )
    .unwrap_err();
    assert!(err.message.contains("self-link"));
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

#[test]
fn per_node_fanout_beyond_cap_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[
            ("0".into(), "9".into()),
            ("0".into(), "2".into()),
            ("0".into(), "31".into()),
            ("9".into(), "15".into()),
        ],
        MapGenLinksOptions {
            max_per_node_fanout: 2,
            ..MapGenLinksOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("topology fanout"));
}

#[test]
fn total_link_count_beyond_cap_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[
            ("0".into(), "9".into()),
            ("0".into(), "2".into()),
            ("9".into(), "15".into()),
        ],
        MapGenLinksOptions {
            max_links: 2,
            ..MapGenLinksOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("link count"));
}

#[test]
fn lane_coupling_count_beyond_cap_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[("0".into(), "31".into()), ("15".into(), "31".into())],
        MapGenLinksOptions {
            max_lane_couplings: 1,
            ..MapGenLinksOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("lane coupling count"));
}

#[test]
fn lane_coupling_fanout_beyond_cap_is_rejected() {
    let rf = default_rf_enrollment();
    let err = lower_hyperlane_topology(
        &rf.pack,
        &[("0".into(), "31".into()), ("15".into(), "31".into())],
        MapGenLinksOptions {
            max_lane_coupling_fanout: 1,
            ..MapGenLinksOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("lane coupling fanout"));
}

#[test]
fn raw_fixture_extracts_five_hyperlanes() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let hyperlanes = extract_hyperlane_declarations(&neutral).expect("extract hyperlanes");
    assert_eq!(hyperlanes.len(), 5);
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
