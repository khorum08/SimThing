//! MapGeneratorCLI PR7 — prove generated partition/bridge `add_hyperlane` output lowers through closed MapGen paths.

use simthing_clausething::{
    MapGenLatticeOptions, MapGenLinksOptions, MapGenResourceFlowOptions,
    extract_hyperlane_declarations, generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_resource_flow_enrollment, parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    ClusterOptions, LatticeCoord, MapGeneratorParams, PartitionOptions, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, place_and_emit_scenario_with_structure, validate_default,
};

const FIXTURE_LATTICE_EDGE: u32 = 3;

const FORBIDDEN_OUTPUT_TERMS: &[&str] = &[
    "metadata = {",
    "lattice = {",
    "location = ",
    "field_operator",
    "nebula",
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "partition",
    "cluster",
    "bridge =",
    "wormhole",
    "gateway",
    "special_route",
    "sqrt",
    "hypot",
    "distance",
    "normalize",
    "PALMA",
    "ResourceFlow",
    "Movement-Front",
    "commitment",
    "BoundaryRequest",
    "link =",
];

fn partition_bridge_params(seed: u64) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params.scale_core.num_stars = 9;
    params.scale_core.lattice_size = Some(10);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params.partitioning.home_system_partitions = 2;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 2;
    params.partitioning.partition_max_systems = 4;
    params.partitioning.partition_min_bridges = 1;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(3);
    params.clustering.cluster_radius = 12.0;
    params.special_routes.num_wormhole_pairs = 0;
    params.special_routes.num_gateways = 0;
    params
}

fn static_cells() -> Vec<LatticeCoord> {
    vec![
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 1, row: 0 },
        LatticeCoord { col: 2, row: 1 },
        LatticeCoord { col: 0, row: 2 },
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 8, row: 0 },
        LatticeCoord { col: 9, row: 1 },
        LatticeCoord { col: 8, row: 2 },
        LatticeCoord { col: 9, row: 2 },
    ]
}

fn inject_deposit_for_rf(text: &str) -> String {
    text.replace(
        "        planet = { count = 1 }",
        "        planet = { count = 1 }\n        deposit = { resources = { minerals = 4 } }",
    )
}

fn emit_partition_bridge_scenario(seed: u64) -> String {
    let params = partition_bridge_params(seed);
    let partition_options = PartitionOptions::from_params(&params, FIXTURE_LATTICE_EDGE);
    let cluster_options = ClusterOptions::from_params(&params);
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    validate_default(&params).expect("params valid");
    inject_deposit_for_rf(
        &place_and_emit_scenario_with_structure(
            &params,
            &registry,
            Some(&static_cells()),
            &emitter,
            None,
            None,
            Some(partition_options),
            Some(cluster_options),
        )
        .expect("emit with partition bridges")
        .into_string(),
    )
}

fn assert_no_forbidden_terms(text: &str) {
    let lower = text.to_ascii_lowercase();
    for term in FORBIDDEN_OUTPUT_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden term {term:?} found in emitted text"
        );
    }
}

fn full_links_enrollment(text: &str) -> simthing_clausething::MapGenLinksEnrollment {
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse neutral AST");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("generate lattice hierarchy");
    let rf = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            deposit_max_participants: 12,
            suppression_max_participants: 12,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("generate RF enrollment");
    generate_mapgen_links(
        &rf,
        &neutral,
        MapGenLinksOptions {
            max_links: 12,
            max_lane_couplings: 12,
            max_lane_coupling_fanout: 4,
            max_per_node_fanout: 4,
        },
    )
    .expect("generate links")
}

#[test]
fn generated_partition_bridge_scenario_parses() {
    let text = emit_partition_bridge_scenario(7070);
    assert!(text.contains("static_galaxy_scenario = {"));
    assert!(text.contains("add_hyperlane = {"));
    assert_no_forbidden_terms(&text);
    parse_mapgen_neutral_document(text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_partition_bridge_scenario_lowers_lattice() {
    let text = emit_partition_bridge_scenario(7070);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("lattice");
    assert_eq!(
        simthing_clausething::collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        9
    );
}

#[test]
fn generated_partition_bridge_scenario_lowers_links_or_lane_couplings() {
    let text = emit_partition_bridge_scenario(7070);
    let enrollment = full_links_enrollment(&text);
    assert!(
        !enrollment.pack.grid_metadata.links.is_empty() || !enrollment.lane_couplings.is_empty(),
        "partition/cluster bridges must lower as links or lane couplings"
    );
    assert_eq!(enrollment.expansion_report.unknown_endpoint_rejections, 0);
    assert_eq!(enrollment.expansion_report.self_link_rejections, 0);
    assert_eq!(enrollment.expansion_report.duplicate_link_rejections, 0);
}

#[test]
fn generated_partition_bridge_scenario_does_not_require_lowerer_widening() {
    let text = emit_partition_bridge_scenario(7070);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hyperlanes = extract_hyperlane_declarations(&neutral).expect("extract hyperlanes");
    assert!(!hyperlanes.is_empty());
    for (from, to) in &hyperlanes {
        assert_ne!(from, to);
        assert!(from.parse::<u32>().is_ok());
        assert!(to.parse::<u32>().is_ok());
    }
}

#[test]
fn generated_partition_bridge_scenario_has_no_field_operator_rf_palma_gpu_surfaces() {
    let text = emit_partition_bridge_scenario(7070);
    assert_no_forbidden_terms(&text);
    for term in [
        "field_operator",
        "palma",
        "gpu",
        "movement-front",
        "resourceflow",
    ] {
        assert!(
            !text.to_ascii_lowercase().contains(term),
            "forbidden surface {term:?} in emitted scenario text"
        );
    }
}

#[test]
fn mapgenerator_is_dev_dependency_only_in_clausething() {
    let manifest = include_str!("../Cargo.toml");
    let dependencies = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("[dependencies] section")
        .split("[dependencies]")
        .nth(1)
        .expect("[dependencies] section");
    assert!(
        !dependencies.contains("simthing-mapgenerator"),
        "simthing-mapgenerator must not appear under [dependencies]"
    );
}
