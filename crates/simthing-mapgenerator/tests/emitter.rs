use simthing_mapgenerator::{
    build_placement_context, place_and_emit_scenario, validate_default, LatticeCoord,
    MapGeneratorParams, PlacedSystemSeed, ScenarioEmitter, ScenarioEmitterConfig, ShapePlacement,
    ShapeRegistry, SquareLattice, DEFAULT_INITIALIZER_REF,
};

const FORBIDDEN_OUTPUT_TERMS: &[&str] = &[
    "link =",
    "field_operator",
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
    "add_hyperlane",
    "prevent_hyperlane",
    "nebula",
];

fn elliptical_params(seed: u64, num_stars: u32, lattice_size: u32) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.scale_core.num_stars = num_stars;
    params.scale_core.lattice_size = Some(lattice_size);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params
}

fn sample_placement() -> ShapePlacement {
    ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 12, row: 9 },
                bucket: Some("example_rim_initializer".into()),
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 3, row: 4 },
                bucket: None,
            },
        ],
    }
}

fn emit_sample(params: &MapGeneratorParams) -> String {
    let lattice =
        SquareLattice::new(params.scale_core.lattice_size.unwrap_or(32)).expect("lattice");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(params));
    emitter
        .emit(params, &lattice, &sample_placement(), None)
        .expect("emit")
        .into_string()
}

fn assert_no_forbidden_terms(text: &str) {
    let lower = text.to_ascii_lowercase();
    for term in FORBIDDEN_OUTPUT_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden term {term:?} found in output"
        );
    }
}

#[test]
fn emitter_outputs_single_root_scenario_id_block() {
    let text = emit_sample(&elliptical_params(42, 2, 32));
    assert!(text.starts_with("generated_elliptical = {\n"));
    assert!(text.ends_with("}\n"));
    assert!(!text.starts_with("scenario = "));
}

#[test]
fn emitter_outputs_static_galaxy_scenario_child() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("    static_galaxy_scenario = {\n"));
}

#[test]
fn emitter_outputs_random_hyperlanes_no() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("        random_hyperlanes = no"));
}

#[test]
fn emitter_outputs_one_system_per_placed_system() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert_eq!(text.matches("        system = {").count(), 2);
}

#[test]
fn system_id_is_quoted_scalar() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("            id = \"0\""));
    assert!(text.contains("            id = \"1\""));
}

#[test]
fn system_name_is_empty_string_or_stable_name() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("            name = \"\""));
}

#[test]
fn system_position_has_x_y_z_zero() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("            position = { x = 12 y = 9 z = 0 }"));
    assert!(text.contains("            position = { x = 3 y = 4 z = 0 }"));
}

#[test]
fn system_initializer_is_bareword_not_string() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("            initializer = example_rim_initializer"));
    assert!(!text.contains("initializer = \"example_rim_initializer\""));
}

#[test]
fn emitter_outputs_sibling_initializer_definition() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("    example_rim_initializer = {"));
    assert!(text.contains("        planet = { count = 1 }"));
}

#[test]
fn output_is_byte_stable_for_same_input() {
    let params = elliptical_params(7, 2, 16);
    let a = emit_sample(&params);
    let b = emit_sample(&params);
    assert_eq!(a, b);
}

#[test]
fn location_names_or_system_ids_are_unique_and_stable() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("            id = \"0\""));
    assert!(text.contains("            id = \"1\""));
}

#[test]
fn positions_are_integer_lattice_values() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("x = 12.0"));
    assert!(!text.contains("y = 9.0"));
}

#[test]
fn emitter_outputs_no_metadata_block() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("metadata = {"));
}

#[test]
fn emitter_outputs_no_lattice_block() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("lattice = {"));
}

#[test]
fn emitter_outputs_no_location_blocks() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("location = "));
}

#[test]
fn emitter_outputs_no_quoted_initializer_ref() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("initializer = \""));
}

#[test]
fn emitter_outputs_no_add_hyperlane() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("add_hyperlane"));
}

#[test]
fn emitter_outputs_no_nebula() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("nebula"));
}

#[test]
fn emitter_outputs_no_field_operator() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("field_operator"));
}

#[test]
fn emitter_outputs_no_links() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(!text.contains("link ="));
}

#[test]
fn emitter_outputs_no_runtime_payloads() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert_no_forbidden_terms(&text);
    assert!(!text.contains("SimThingKind"));
    assert!(!text.contains("HydratedScenario"));
}

#[test]
fn emitter_outputs_no_route_path_predecessor_movement_border_frontline_terms() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    for term in [
        "route",
        "predecessor",
        "movement",
        "border",
        "frontline",
        "pathfinding",
    ] {
        assert!(
            !text.to_ascii_lowercase().contains(term),
            "found forbidden term {term}"
        );
    }
}

#[test]
fn emitter_outputs_no_euclidean_authority_terms() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    for term in ["sqrt", "hypot", "distance", "normalize", "nearest"] {
        assert!(
            !text.to_ascii_lowercase().contains(term),
            "found forbidden term {term}"
        );
    }
}

#[test]
fn emitter_uses_bucket_as_initializer_ref() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    assert!(text.contains("            initializer = example_rim_initializer"));
}

#[test]
fn emitter_uses_safe_default_initializer_when_bucket_missing() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    let first_system = text
        .split("            id = \"0\"")
        .nth(1)
        .expect("system 0")
        .split("\n        }")
        .next()
        .expect("system 0 body");
    assert!(first_system.contains(&format!("initializer = {DEFAULT_INITIALIZER_REF}")));
    let second_system = text
        .split("            id = \"1\"")
        .nth(1)
        .expect("system 1")
        .split("\n        }")
        .next()
        .expect("system 1 body");
    assert!(second_system.contains(&format!("initializer = {DEFAULT_INITIALIZER_REF}")));
}

#[test]
fn emitter_scenario_display_name_reflects_seed() {
    let a = emit_sample(&elliptical_params(1, 2, 16));
    let b = emit_sample(&elliptical_params(2, 2, 16));
    assert!(a.contains("seed 1"));
    assert!(b.contains("seed 2"));
    assert_ne!(a, b);
}

#[test]
fn crate_still_has_no_forbidden_sim_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
        "simthing-clausething",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "Cargo.toml must not depend on {forbidden}"
        );
    }
}

#[test]
fn place_and_emit_pipeline_produces_static_galaxy_scenario_text() {
    let params = elliptical_params(99, 5, 24);
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter).expect("pipeline");
    assert!(text.as_str().starts_with("generated_elliptical = {"));
    assert!(text.as_str().contains("static_galaxy_scenario = {"));
    assert_eq!(text.as_str().matches("        system = {").count(), 5);
    assert_no_forbidden_terms(text.as_str());
}

#[test]
fn emitter_scenario_id_follows_shape() {
    let params = elliptical_params(1, 1, 8);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let placement = ShapePlacement {
        systems: vec![PlacedSystemSeed {
            id: 0,
            coord: LatticeCoord { col: 1, row: 1 },
            bucket: None,
        }],
    };
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = emitter
        .emit(&params, &lattice, &placement, None)
        .expect("emit");
    assert!(text.as_str().starts_with("generated_elliptical = {"));
}

fn hyperlane_placement() -> ShapePlacement {
    ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 1, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 2,
                coord: LatticeCoord { col: 2, row: 0 },
                bucket: None,
            },
        ],
    }
}

fn hyperlane_topology() -> simthing_mapgenerator::HyperlaneTopology {
    simthing_mapgenerator::HyperlaneTopology {
        edges: vec![
            simthing_mapgenerator::HyperlaneEdge {
                from: "0".into(),
                to: "1".into(),
            },
            simthing_mapgenerator::HyperlaneEdge {
                from: "1".into(),
                to: "2".into(),
            },
        ],
    }
}

const ROUTE_FORBIDDEN_TERMS: &[&str] = &[
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
];

#[test]
fn emitter_outputs_add_hyperlane_blocks_when_edges_exist() {
    let params = elliptical_params(1, 3, 8);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = emitter
        .emit(
            &params,
            &lattice,
            &hyperlane_placement(),
            Some(&hyperlane_topology()),
        )
        .expect("emit");
    assert_eq!(
        text.as_str().matches("        add_hyperlane = {").count(),
        2
    );
    assert!(text.as_str().contains("            from = \"0\""));
    assert!(text.as_str().contains("            to = \"1\""));
}

#[test]
fn emitter_outputs_no_add_hyperlane_when_edges_empty() {
    let params = elliptical_params(1, 2, 8);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = emitter
        .emit(
            &params,
            &lattice,
            &sample_placement(),
            Some(&simthing_mapgenerator::HyperlaneTopology { edges: vec![] }),
        )
        .expect("emit");
    assert!(!text.as_str().contains("add_hyperlane"));
}

#[test]
fn emitter_output_has_no_route_path_predecessor_movement_border_frontline_terms() {
    let params = elliptical_params(1, 3, 8);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = emitter
        .emit(
            &params,
            &lattice,
            &hyperlane_placement(),
            Some(&hyperlane_topology()),
        )
        .expect("emit")
        .into_string();
    let lower = text.to_ascii_lowercase();
    for term in ROUTE_FORBIDDEN_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden {term:?}"
        );
    }
}
