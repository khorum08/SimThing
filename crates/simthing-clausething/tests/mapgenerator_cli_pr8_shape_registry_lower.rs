//! MapGeneratorCLI PR8 — procedural shape lowering smoke test.

use simthing_clausething::{
    MapGenLatticeOptions, generate_mapgen_lattice_hierarchy, parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
    place_and_emit_scenario, validate_default,
};

#[test]
fn generated_procedural_spiral_scenario_parses() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_4".into();
    params.scale_core.num_stars = 8;
    params.scale_core.lattice_size = Some(24);
    params.seed = 8408;
    validate_default(&params).expect("valid params");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter)
        .expect("emit")
        .into_string();
    assert!(text.contains("static_galaxy_scenario = {"));
    parse_mapgen_neutral_document(text.as_bytes()).expect("parse neutral AST");
}

#[test]
fn generated_procedural_spiral_scenario_lowers_lattice() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_4".into();
    params.scale_core.num_stars = 8;
    params.scale_core.lattice_size = Some(24);
    params.seed = 8408;
    validate_default(&params).expect("valid params");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter)
        .expect("emit")
        .into_string();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: 3,
        },
    )
    .expect("lattice");
    assert_eq!(
        simthing_clausething::collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        8
    );
}

#[test]
fn generated_procedural_shape_does_not_require_lowerer_widening() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "ring".into();
    params.scale_core.num_stars = 6;
    params.scale_core.lattice_size = Some(20);
    params.seed = 8708;
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter)
        .expect("emit")
        .into_string();
    for term in [
        "field_operator",
        "partition",
        "route",
        "predecessor",
        "movement",
    ] {
        assert!(
            !text.to_ascii_lowercase().contains(term),
            "forbidden term {term:?}"
        );
    }
}
