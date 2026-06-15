//! MapGeneratorCLI PR6 — prove generated `add_hyperlane` output lowers through closed MapGen paths.

use simthing_clausething::{
    MapGenLatticeOptions, MapGenLinksOptions, extract_hyperlane_declarations,
    generate_mapgen_lattice_hierarchy, lower_hyperlane_topology, parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    HyperlaneOptions, LatticeCoord, MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig,
    ShapeRegistry, place_and_emit_scenario_with_hyperlanes, validate_default,
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

fn hyperlane_params(seed: u64) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params.scale_core.num_stars = 4;
    params.scale_core.lattice_size = Some(8);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params.hyperlane.max_hyperlane_distance = 4.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 4;
    params.hyperlane.num_hyperlanes_default = 3;
    params.hyperlane.random_hyperlanes = false;
    params
}

fn static_cells() -> Vec<LatticeCoord> {
    vec![
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 5, row: 3 },
        LatticeCoord { col: 2, row: 6 },
        LatticeCoord { col: 7, row: 1 },
    ]
}

fn emit_hyperlane_scenario(seed: u64) -> String {
    let params = hyperlane_params(seed);
    let options = HyperlaneOptions::from_params(&params, FIXTURE_LATTICE_EDGE);
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    validate_default(&params).expect("params valid");
    place_and_emit_scenario_with_hyperlanes(
        &params,
        &registry,
        Some(&static_cells()),
        &emitter,
        Some(options),
    )
    .expect("emit with hyperlanes")
    .into_string()
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

fn lower_links(text: &str) -> simthing_clausething::MapGenLinksEnrollment {
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse neutral AST");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("generate lattice hierarchy");
    let hyperlanes = extract_hyperlane_declarations(&neutral).expect("extract hyperlanes");
    lower_hyperlane_topology(
        &hierarchy.pack,
        &hyperlanes,
        MapGenLinksOptions {
            max_links: 8,
            max_lane_couplings: 8,
            max_lane_coupling_fanout: 4,
            max_per_node_fanout: 4,
        },
    )
    .expect("lower hyperlane topology")
}

#[test]
fn generated_hyperlane_scenario_parses() {
    let text = emit_hyperlane_scenario(4242);
    assert!(text.contains("static_galaxy_scenario = {"));
    assert!(text.contains("add_hyperlane = {"));
    assert_no_forbidden_terms(&text);
    parse_mapgen_neutral_document(text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_hyperlane_scenario_lowers_lattice() {
    let text = emit_hyperlane_scenario(4242);
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
        4
    );
}

#[test]
fn generated_hyperlane_scenario_lowers_links_or_lane_couplings() {
    let text = emit_hyperlane_scenario(4242);
    let enrollment = lower_links(&text);
    assert!(
        !enrollment.pack.grid_metadata.links.is_empty() || !enrollment.lane_couplings.is_empty()
    );
    assert_eq!(
        enrollment.expansion_report.unknown_endpoint_rejections, 0,
        "producer must not emit unknown endpoints"
    );
    assert_eq!(enrollment.expansion_report.self_link_rejections, 0);
    assert_eq!(enrollment.expansion_report.duplicate_link_rejections, 0);
    assert!(
        enrollment
            .expansion_report
            .unsafe_expansion_flags
            .is_empty()
    );
}

#[test]
fn generated_hyperlane_blocks_are_extracted_by_existing_reader() {
    let text = emit_hyperlane_scenario(4242);
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
fn generated_hyperlane_scenario_rejects_unknown_endpoint_without_widening() {
    let text = emit_hyperlane_scenario(4242);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("lattice");
    let hyperlanes = vec![("0".into(), "missing".into())];
    let err = lower_hyperlane_topology(&hierarchy.pack, &hyperlanes, MapGenLinksOptions::default())
        .expect_err("unknown endpoint must fail closed");
    assert!(err.message.contains("unknown gridcell endpoint"));
}

#[test]
fn generated_hyperlane_scenario_rejects_self_link_without_widening() {
    let text = emit_hyperlane_scenario(4242);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("lattice");
    let hyperlanes = vec![("1".into(), "1".into())];
    let err = lower_hyperlane_topology(&hierarchy.pack, &hyperlanes, MapGenLinksOptions::default())
        .expect_err("self-link must fail closed");
    assert!(err.message.contains("self-link"));
}

#[test]
fn generated_hyperlane_scenario_rejects_duplicate_link_without_widening() {
    let text = emit_hyperlane_scenario(4242);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
        },
    )
    .expect("lattice");
    let hyperlanes = vec![("0".into(), "1".into()), ("1".into(), "0".into())];
    let enrollment =
        lower_hyperlane_topology(&hierarchy.pack, &hyperlanes, MapGenLinksOptions::default())
            .expect("duplicate hyperlane pair is rejected without widening");
    assert_eq!(enrollment.expansion_report.duplicate_link_rejections, 1);
    assert_eq!(
        enrollment.pack.grid_metadata.links.len() + enrollment.lane_couplings.len(),
        1
    );
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
