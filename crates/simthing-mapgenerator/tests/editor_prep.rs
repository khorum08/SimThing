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

#[test]
fn shape_param_accepts_valid_spiral_params() {
    let mut params = spiral_2_params_with_shape_params(BTreeMap::new());
    apply_cli_shape_params(
        &mut params.shape,
        &[
            "arm_width=14".into(),
            "arm_tightness=0.6".into(),
            "jitter=2".into(),
        ],
    )
    .expect("valid assignments");
    validate_shape_params_for_params(&params, &registry()).expect("valid spiral params");
}

#[test]
fn coordinate_transform_is_not_accepted_as_numeric_label_param() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params
        .shape
        .shape_params
        .insert("coordinate_transform".into(), 1.0);
    let err = validate_shape_params_for_params(&params, &registry()).unwrap_err();
    assert!(matches!(err, ValidationError::NonNumericShapeParam { .. }));
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
fn report_json_is_written() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    let path = std::env::temp_dir().join("simthing_mapgen_editor_prep_report.json");
    write_generation_report_json(&report, &path).expect("write report");
    let text = std::fs::read_to_string(&path).expect("read report");
    assert!(text.contains(REPORT_SCHEMA_VERSION));
    let _ = std::fs::remove_file(path);
}

#[test]
fn report_json_has_schema_version() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    assert_eq!(report.schema_version, REPORT_SCHEMA_VERSION);
}

#[test]
fn report_json_records_request_shape_seed_star_count_lattice() {
    let mut shape_params = BTreeMap::new();
    shape_params.insert("arm_width".into(), 14.0);
    let report = generate_small_spiral_report(42, shape_params);
    assert_eq!(report.request.shape, "spiral_2");
    assert_eq!(report.generator.seed, 42);
    assert_eq!(report.request.star_count, 80);
    assert_eq!(report.request.lattice_width, 64);
    assert_eq!(report.request.lattice_height, 64);
}

#[test]
fn report_json_records_shape_params() {
    let mut shape_params = BTreeMap::new();
    shape_params.insert("arm_width".into(), 14.0);
    shape_params.insert("arm_tightness".into(), 0.6);
    shape_params.insert("jitter".into(), 2.0);
    let report = generate_small_spiral_report(42, shape_params.clone());
    assert_eq!(report.request.shape_params, shape_params);
}

#[test]
fn report_json_records_connectivity_stats() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    assert_eq!(report.output.component_count, 1);
    assert_eq!(report.request.ensure_connected, true);
    assert!(report.output.components_before.is_some());
    assert!(report.output.longest_bridge_chebyshev.is_some());
}

#[test]
fn report_json_records_degree_stats() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    assert!(report.output.max_degree >= report.output.min_degree);
    assert!(report.output.average_degree >= 0.0);
    assert_eq!(report.output.isolated_system_count, 0);
}

#[test]
fn report_json_records_constitution_flags() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    assert_eq!(
        report.constitution.structural_coordinates,
        "authored_gridcell"
    );
    assert!(!report.constitution.render_coordinates_authoritative);
    assert!(!report.constitution.uses_native_sqrt_authority);
    assert!(!report.constitution.uses_pathfinding_semantics);
    assert!(!report.constitution.uses_runtime_simulation);
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

#[test]
fn report_json_changes_when_seed_changes_where_expected() {
    let first =
        normalized_report_json(&generate_small_spiral_report(1, BTreeMap::new())).expect("json");
    let second =
        normalized_report_json(&generate_small_spiral_report(2, BTreeMap::new())).expect("json");
    assert_ne!(first, second);
}

#[test]
fn report_json_distinguishes_topology_edges_from_connectivity_bridges() {
    let report = generate_small_spiral_report(42, BTreeMap::new());
    assert!(
        report.output.actual_topology_hyperlanes <= report.output.actual_base_hyperlanes,
        "topology edges must not exceed post-connectivity base count"
    );
    assert_eq!(
        report.output.actual_base_hyperlanes,
        report.output.actual_topology_hyperlanes + report.output.connectivity_bridge_count
    );
}

#[test]
fn report_json_flags_topology_target_deficit() {
    let report = bad_editor_prep_class_report();
    assert_eq!(report.output.requested_target_hyperlanes, 6000);
    assert!(
        report.output.actual_topology_hyperlanes < 3000,
        "bad class should have far fewer topology edges than requested"
    );
    assert!(report.output.topology_target_deficit > 0);
    assert!(report.output.topology_target_ratio < TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD);
    assert_eq!(report.output.map_quality_status, MAP_QUALITY_FAIL);
}

#[test]
fn report_json_flags_high_connectivity_bridge_ratio() {
    let report = bad_editor_prep_class_report();
    assert!(report.output.connectivity_bridge_ratio > CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD);
    assert_eq!(report.output.map_quality_status, MAP_QUALITY_FAIL);
}

#[test]
fn report_json_flags_low_average_degree_for_dense_preview() {
    let report = bad_editor_prep_class_report();
    assert!(report.output.average_degree < 2.5);
    assert!(report
        .output
        .map_quality_warnings
        .iter()
        .any(|w| w.contains("average_degree")));
}

#[test]
fn report_json_quality_passes_for_healthy_dense_spiral_sample() {
    let report = generate_editor_prep_dense_report();
    assert_eq!(report.output.component_count, 1);
    assert_eq!(report.output.isolated_system_count, 0);
    assert!(
        report.output.actual_topology_hyperlanes * 2 >= report.output.requested_target_hyperlanes,
        "topology should reach at least half of requested target (actual={})",
        report.output.actual_topology_hyperlanes
    );
    assert!(report.output.connectivity_bridge_ratio <= 0.25);
    assert!(report.output.average_degree >= 2.5);
    assert_eq!(report.output.map_quality_status, MAP_QUALITY_PASS);
}

#[test]
fn editor_prep_sample_does_not_have_three_topology_edges() {
    let report = generate_editor_prep_dense_report();
    assert!(
        report.output.actual_topology_hyperlanes > 100,
        "healthy editor-prep sample must not collapse to a handful of topology edges (got {})",
        report.output.actual_topology_hyperlanes
    );
}
