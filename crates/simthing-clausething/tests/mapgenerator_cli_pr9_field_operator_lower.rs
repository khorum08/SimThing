//! MapGeneratorCLI PR9 — prove generated nebula feedstock lowers through closed MapGen lattice + Movement-Front surfaces.

use simthing_clausething::{
    MapGenLatticeOptions, MapGenLinksOptions, MapGenMovementFrontOptions,
    MapGenResourceFlowOptions, generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_movement_front_authoring, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    HyperlaneOptions, LatticeCoord, MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig,
    ShapeRegistry, forbidden_field_surface_term, place_and_emit_scenario_with_hyperlanes,
    validate_default,
};
use simthing_spec::RegionFieldOperatorSpec;

const FIXTURE_LATTICE_EDGE: u32 = 3;

const ROUTE_FORBIDDEN_TERMS: &[&str] = &[
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "field_operator",
    "gpu",
    "palma",
];

fn nebula_field_params(seed: u64) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params.scale_core.num_stars = 5;
    params.scale_core.lattice_size = Some(10);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 20.0;
    params.nebula.nebula_min_dist = 2.0;
    params.initializers.initializer_bucket_core = "core_initializer".into();
    params.initializers.initializer_bucket_arm = "arm_initializer".into();
    params.hyperlane.max_hyperlane_distance = 4.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 4;
    params.hyperlane.num_hyperlanes_default = 3;
    params.hyperlane.random_hyperlanes = false;
    params
}

fn static_cells() -> Vec<LatticeCoord> {
    vec![
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 1, row: 0 },
        LatticeCoord { col: 2, row: 1 },
        LatticeCoord { col: 0, row: 2 },
        LatticeCoord { col: 1, row: 2 },
    ]
}

fn inject_deposit_for_rf(text: &str) -> String {
    text.replace(
        "        planet = { count = 1 }",
        "        planet = { count = 1 }\n        deposit = { resources = { minerals = 4 } }",
    )
}

fn emit_nebula_scenario(seed: u64) -> String {
    let params = nebula_field_params(seed);
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    validate_default(&params).expect("params valid");
    let hyperlane_options = HyperlaneOptions::from_params(&params, FIXTURE_LATTICE_EDGE);
    inject_deposit_for_rf(
        &place_and_emit_scenario_with_hyperlanes(
            &params,
            &registry,
            Some(&static_cells()),
            &emitter,
            Some(hyperlane_options),
        )
        .expect("emit")
        .into_string(),
    )
}

fn full_links_enrollment(text: &str) -> simthing_clausething::MapGenLinksEnrollment {
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse neutral AST");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
            ..Default::default()
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
fn generated_field_operator_scenario_parses() {
    let text = emit_nebula_scenario(9090);
    assert!(text.contains("static_galaxy_scenario = {"));
    assert!(text.contains("nebula = {"));
    assert!(text.contains("generated_nebula_0"));
    parse_mapgen_neutral_document(text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_field_operator_scenario_lowers_lattice() {
    let text = emit_nebula_scenario(9090);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: FIXTURE_LATTICE_EDGE,
            ..Default::default()
        },
    )
    .expect("lattice");
    assert_eq!(
        simthing_clausething::collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        5
    );
    assert!(text.contains("initializer = core_initializer"));
    assert_eq!(text.matches("    core_initializer = {").count(), 1);
}

#[test]
fn generated_field_operator_scenario_lowers_field_operator_or_region_field_spec() {
    let text = emit_nebula_scenario(9090);
    let links = full_links_enrollment(&text);
    let authoring =
        generate_mapgen_movement_front_authoring(&links, MapGenMovementFrontOptions::default())
            .expect("movement front authoring");
    assert_eq!(authoring.expansion_report.l1_field_operator_count, 1);
    let field = &authoring.pack.game_mode.region_fields[0];
    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux { .. }
    ));
}

#[test]
fn generated_field_operator_scenario_does_not_require_lowerer_widening() {
    let text = emit_nebula_scenario(9090);
    assert!(forbidden_field_surface_term(&text).is_none());
    let lower = text.to_ascii_lowercase();
    for term in ROUTE_FORBIDDEN_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden term {term:?}"
        );
    }
}

#[test]
fn generated_field_operator_scenario_has_no_runtime_gpu_surfaces() {
    let text = emit_nebula_scenario(9090);
    for term in ["gpu", "wgsl", "runtime_field", "cpu_planner"] {
        assert!(
            !text.to_ascii_lowercase().contains(term),
            "forbidden runtime surface {term:?}"
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
