use simthing_mapgenerator::{
    build_placement_context, place_and_emit_scenario, validate_default, LatticeCoord,
    MapGeneratorParams, PlacedSystemSeed, ScenarioEmitter, ScenarioEmitterConfig, ShapePlacement,
    ShapeRegistry, SquareLattice, DEFAULT_INITIALIZER_REF,
};

const FORBIDDEN_OUTPUT_TERMS: &[&str] = &[
    "link =",
    "hyperlane",
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
        .emit(params, &lattice, &sample_placement())
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
fn emitter_outputs_metadata_block() {
    let params = elliptical_params(42, 2, 32);
    let text = emit_sample(&params);
    assert!(text.contains("metadata = {"));
    assert!(text.contains("generated_by = \"MapGeneratorCLI\""));
    assert!(text.contains("shape = \"elliptical\""));
    assert!(text.contains("seed = 42"));
}

#[test]
fn emitter_outputs_square_lattice_block() {
    let params = elliptical_params(1, 2, 32);
    let text = emit_sample(&params);
    assert!(text.contains("lattice = { width = 32 height = 32 }"));
}

#[test]
fn emitter_outputs_one_location_per_placed_system() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert_eq!(text.matches("location = system_").count(), 2);
}

#[test]
fn emitter_uses_integer_lattice_positions() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("position = { x = 12 y = 9 }"));
    assert!(text.contains("position = { x = 3 y = 4 }"));
    assert!(!text.contains("position = { x = 12.0"));
}

#[test]
fn emitter_location_names_are_unique_and_stable() {
    let text = emit_sample(&elliptical_params(1, 2, 32));
    assert!(text.contains("location = system_000000"));
    assert!(text.contains("location = system_000001"));
    let names: Vec<_> = text
        .lines()
        .filter(|line| line.trim_start().starts_with("location = system_"))
        .collect();
    assert_eq!(names.len(), 2);
}

#[test]
fn emitter_same_input_is_byte_stable() {
    let params = elliptical_params(7, 2, 16);
    let a = emit_sample(&params);
    let b = emit_sample(&params);
    assert_eq!(a, b);
}

#[test]
fn emitter_different_seed_metadata_changes_if_seed_differs() {
    let a = emit_sample(&elliptical_params(1, 2, 16));
    let b = emit_sample(&elliptical_params(2, 2, 16));
    assert_ne!(a, b);
    assert!(a.contains("seed = 1"));
    assert!(b.contains("seed = 2"));
}

#[test]
fn emitter_uses_bucket_as_initializer_ref() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    assert!(text.contains("initializer = \"example_rim_initializer\""));
}

#[test]
fn emitter_uses_safe_default_initializer_when_bucket_missing() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    let second_block = text
        .split("location = system_000001")
        .nth(1)
        .expect("second loc");
    assert!(second_block.contains(&format!("initializer = \"{DEFAULT_INITIALIZER_REF}\"")));
}

#[test]
fn emitter_outputs_no_links() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    assert!(!text.contains("link ="));
}

#[test]
fn emitter_outputs_no_field_operators() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
    assert!(!text.contains("field_operator"));
}

#[test]
fn emitter_outputs_no_runtime_payloads() {
    let text = emit_sample(&elliptical_params(1, 2, 16));
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
fn place_and_emit_pipeline_produces_scenario_text() {
    let params = elliptical_params(99, 5, 24);
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter).expect("pipeline");
    assert!(text.as_str().contains("scenario = generated_elliptical"));
    assert_eq!(text.as_str().matches("location = system_").count(), 5);
    assert_no_forbidden_terms(text.as_str());
}

#[test]
fn emitter_scenario_name_follows_shape() {
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
    let text = emitter.emit(&params, &lattice, &placement).expect("emit");
    assert!(text.as_str().starts_with("scenario = generated_elliptical"));
}
