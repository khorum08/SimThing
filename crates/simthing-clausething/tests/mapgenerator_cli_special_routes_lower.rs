//! MapGeneratorCLI PR6b — prove generated special-route `add_hyperlane` output lowers through closed MapGen paths.

use simthing_clausething::{
    MapGenLatticeOptions, MapGenLinksOptions, MapGenResourceFlowOptions,
    extract_hyperlane_declarations, generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_resource_flow_enrollment, parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    LatticeCoord, MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
    SpecialRouteOptions, place_and_emit_scenario_with_couplings, validate_default,
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

fn special_route_params(seed: u64) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.scale_core.num_stars = 6;
    params.scale_core.lattice_size = Some(8);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 1;
    params
}

fn static_cells() -> Vec<LatticeCoord> {
    vec![
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 5, row: 3 },
        LatticeCoord { col: 2, row: 6 },
        LatticeCoord { col: 7, row: 1 },
        LatticeCoord { col: 0, row: 5 },
        LatticeCoord { col: 6, row: 4 },
    ]
}

fn inject_deposit_for_rf(text: &str) -> String {
    text.replace(
        "        planet = { count = 1 }",
        "        planet = { count = 1 }\n        deposit = { resources = { minerals = 4 } }",
    )
}

fn emit_special_route_scenario(seed: u64) -> String {
    let params = special_route_params(seed);
    let options = SpecialRouteOptions::from_params(&params, FIXTURE_LATTICE_EDGE);
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    validate_default(&params).expect("params valid");
    inject_deposit_for_rf(
        &place_and_emit_scenario_with_couplings(
            &params,
            &registry,
            Some(&static_cells()),
            &emitter,
            None,
            Some(options),
        )
        .expect("emit with special routes")
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
            deposit_max_participants: 8,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("generate RF enrollment");
    generate_mapgen_links(
        &rf,
        &neutral,
        MapGenLinksOptions {
            max_links: 8,
            max_lane_couplings: 8,
            max_lane_coupling_fanout: 4,
            max_per_node_fanout: 4,
        },
    )
    .expect("generate links")
}

#[test]
fn generated_special_routes_parse() {
    let text = emit_special_route_scenario(5150);
    assert!(text.contains("static_galaxy_scenario = {"));
    assert!(text.contains("add_hyperlane = {"));
    assert_no_forbidden_terms(&text);
    parse_mapgen_neutral_document(text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_special_routes_lower_lattice() {
    let text = emit_special_route_scenario(5150);
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
        6
    );
}

#[test]
fn generated_special_routes_lower_as_lane_couplings() {
    let text = emit_special_route_scenario(5150);
    let enrollment = full_links_enrollment(&text);
    assert!(
        !enrollment.lane_couplings.is_empty(),
        "special routes must lower as lane couplings"
    );
    assert_eq!(
        enrollment.expansion_report.lane_coupling_count as usize,
        enrollment.lane_couplings.len()
    );
    assert_eq!(enrollment.expansion_report.unknown_endpoint_rejections, 0);
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
fn generated_special_routes_do_not_require_lowerer_widening() {
    let text = emit_special_route_scenario(5150);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hyperlanes = extract_hyperlane_declarations(&neutral).expect("extract hyperlanes");
    assert_eq!(hyperlanes.len(), 2, "one wormhole pair + one gateway");
    for (from, to) in &hyperlanes {
        assert_ne!(from, to);
        assert!(from.parse::<u32>().is_ok());
        assert!(to.parse::<u32>().is_ok());
    }
}

#[test]
fn generated_special_routes_have_no_field_operator_rf_palma_gpu_surfaces() {
    let text = emit_special_route_scenario(5150);
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
