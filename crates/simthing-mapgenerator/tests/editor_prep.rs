//! MAPGENCLI-EDITOR-PREP-0 / 0R — fail-closed shape params + JSON generation report + quality gates.

use std::collections::BTreeMap;

use simthing_mapgenerator::{
    apply_cli_shape_params, build_generation_report, generate_galaxy_with_structure,
    normalized_report_json, structure_options_from_params,
    validate_shape_params_for_params, write_generation_report_json, GenerationReport,
    MapGeneratorParams, ReportArtifacts, ScenarioEmitter, ScenarioEmitterConfig,
    ShapeRegistry, ValidationError, CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD,
    MAP_QUALITY_FAIL, MAP_QUALITY_PASS, REPORT_SCHEMA_VERSION,
    TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD,
};

fn registry() -> ShapeRegistry {
    ShapeRegistry::default()
}

fn spiral_2_params_with_shape_params(shape_params: BTreeMap<String, f64>) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_2".into();
    params.scale_core.num_stars = 48;
    params.scale_core.lattice_size = Some(64);
    params.shape.shape_params = shape_params;
    params
}

fn generate_small_spiral_report(
    seed: u64,
    shape_params: BTreeMap<String, f64>,
) -> GenerationReport {
    generate_spiral_report_with_hyperlane_settings(seed, shape_params, 40, 80, 4.0)
}

fn generate_spiral_report_with_hyperlane_settings(
    seed: u64,
    shape_params: BTreeMap<String, f64>,
    target: u32,
    max: u32,
    max_distance: f64,
) -> GenerationReport {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_2".into();
    params.scale_core.num_stars = 80;
    params.scale_core.lattice_size = Some(64);
    params.seed = seed;
    params.hyperlane.num_hyperlanes_default = target;
    params.hyperlane.num_hyperlanes_max = max;
    params.hyperlane.max_hyperlane_distance = max_distance;
    params.hyperlane.ensure_connected = true;
    params.shape.shape_params = shape_params;
    params
        .validate(&registry())
        .expect("params should validate");
    let (hyperlane, special, partition, cluster) =
        structure_options_from_params(&params).expect("structure options");
    let result = generate_galaxy_with_structure(
        &params,
        &registry(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        Some(partition),
        Some(cluster),
    )
    .expect("generation");
    build_generation_report(&params, &result, ReportArtifacts::new())
}

fn editor_prep_dense_spiral_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_2".into();
    params.scale_core.num_stars = 3000;
    params.scale_core.lattice_size = Some(300);
    params.seed = 42;
    params.hyperlane.num_hyperlanes_default = 6000;
    params.hyperlane.num_hyperlanes_max = 6000;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.max_hyperlane_distance = 8.0;
    params.hyperlane.ensure_connected = true;
    params.shape.shape_params = BTreeMap::from([
        ("arm_width".into(), 14.0),
        ("arm_tightness".into(), 0.6),
        ("jitter".into(), 2.0),
    ]);
    params.clustering.cluster_count = Some(4);
    params.clustering.cluster_radius = 500.0;
    params.partitioning.home_system_partitions = 0;
    params.partitioning.open_space_partitions = 0;
    params
}

fn generate_editor_prep_dense_report() -> GenerationReport {
    let params = editor_prep_dense_spiral_params();
    params.validate(&registry()).expect("dense params valid");
    let (hyperlane, special, _partition, cluster) =
        structure_options_from_params(&params).expect("structure options");
    let result = generate_galaxy_with_structure(
        &params,
        &registry(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        None,
        Some(cluster),
    )
    .expect("dense generation");
    build_generation_report(&params, &result, ReportArtifacts::new())
}

fn bad_editor_prep_class_report() -> GenerationReport {
    let mut params = editor_prep_dense_spiral_params();
    // Reproduce #723 sample failure mode: target 6000 but max still 3 → 3 topology edges.
    params.hyperlane.num_hyperlanes_max = 3;
    params
        .validate(&registry())
        .expect("bad-class params valid");
    let (hyperlane, special, _partition, cluster) =
        structure_options_from_params(&params).expect("structure options");
    let result = generate_galaxy_with_structure(
        &params,
        &registry(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        None,
        Some(cluster),
    )
    .expect("bad-class generation");
    build_generation_report(&params, &result, ReportArtifacts::new())
}

#[test]
fn report_json_is_deterministic_for_same_seed_and_options() {
    let shape_params = BTreeMap::from([("arm_width".into(), 14.0), ("jitter".into(), 2.0)]);
    let first = normalized_report_json(&generate_small_spiral_report(99, shape_params.clone()))
        .expect("json");
    let second =
        normalized_report_json(&generate_small_spiral_report(99, shape_params)).expect("json");
    assert_eq!(first, second);
}
