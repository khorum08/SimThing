use simthing_mapgenerator::{
    validate_default, ArbitraryHyperlaneSourceMode, ClusterCountMethod, GenerationMode,
    MapGeneratorParams, PartitionMethod, ShapeRegistry, ValidationError,
};

fn registry() -> ShapeRegistry {
    ShapeRegistry::default()
}

#[test]
fn default_params_validate() {
    let params = MapGeneratorParams::default();
    params
        .validate(&registry())
        .expect("default params should validate");
}

#[test]
fn full_procedural_lever_params_validate() {
    let mut params = MapGeneratorParams::default();
    params.mode = GenerationMode::Procedural;
    params.shape.shape = "spiral_4".into();
    params.shape.shape_params.insert("num_arms".into(), 4.0);
    params
        .shape
        .shape_params
        .insert("arm_tightness".into(), 1.0);
    params.clustering.cluster_count_method = ClusterCountMethod::OneEveryXEmpire;
    params.partitioning.partition_method = PartitionMethod::DepthFirst;
    params.hyperlane.random_hyperlanes = false;
    params
        .validate(&registry())
        .expect("full procedural surface should validate");
}

#[test]
fn arbitrary_static_mode_params_validate() {
    let mut params = MapGeneratorParams::default();
    params.mode = GenerationMode::ArbitraryStatic;
    params.shape.shape = "arbitrary_static".into();
    params.arbitrary.explicit_point_cloud_path = Some("fixtures/points.json".into());
    params.arbitrary.explicit_graph_path = Some("fixtures/graph.json".into());
    params.arbitrary.coordinate_transform = Some("identity".into());
    params.arbitrary.hyperlane_source_mode = ArbitraryHyperlaneSourceMode::PreventHyperlane;
    params
        .validate(&registry())
        .expect("arbitrary static shell should validate");
}

#[test]
fn unknown_shape_param_rejects() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.shape.shape_params.insert("bogus_key".into(), 1.0);
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::UnknownShapeParam { .. }));
}

#[test]
fn partition_bridge_params_validate() {
    let mut params = MapGeneratorParams::default();
    params.partitioning.partition_min_systems = 3;
    params.partitioning.partition_max_systems = 10;
    params.partitioning.partition_min_bridges = 1;
    params.partitioning.partition_max_bridges = 2;
    params
        .validate(&registry())
        .expect("partition bounds should validate");
}

#[test]
fn hyperlane_geometry_params_validate() {
    let mut params = MapGeneratorParams::default();
    params.hyperlane.max_hyperlane_distance = 6.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 4;
    params.hyperlane.num_hyperlanes_default = 2;
    params
        .validate(&registry())
        .expect("hyperlane geometry should validate");
}

#[test]
fn special_route_params_validate_as_bounded_lane_coupling_inputs() {
    let mut params = MapGeneratorParams::default();
    params.special_routes.num_wormhole_pairs = 2;
    params.special_routes.num_gateways = 1;
    params
        .validate(&registry())
        .expect("special routes should validate");
}

#[test]
fn nebula_field_params_validate_as_declarative_field_operator_inputs() {
    let mut params = MapGeneratorParams::default();
    params.nebula.num_nebulas = 3;
    params.nebula.nebula_size = 40.0;
    params.nebula.nebula_min_dist = 15.0;
    params
        .validate(&registry())
        .expect("nebula params should validate");
}

#[test]
fn metadata_passthrough_is_inert_and_carried() {
    let params = MapGeneratorParams::default();
    assert_eq!(params.metadata.num_empires, 6);
    assert!(params.metadata.crisis_strength.is_finite());
    validate_default(&params).expect("metadata fields are carried inertly");
}

#[test]
fn invalid_num_stars_rejects() {
    let mut params = MapGeneratorParams::default();
    params.scale_core.num_stars = 0;
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::MustBePositive { .. }));
}

#[test]
fn invalid_hyperlane_min_max_rejects() {
    let mut params = MapGeneratorParams::default();
    params.hyperlane.num_hyperlanes_min = 5;
    params.hyperlane.num_hyperlanes_max = 2;
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::MinGreaterThanMax { .. }));
}

#[test]
fn same_params_serialize_stably() {
    let params = MapGeneratorParams::default();
    let a = serde_json::to_string(&params).expect("serialize");
    let b = serde_json::to_string(&params).expect("serialize");
    assert_eq!(a, b);
}

#[test]
fn params_file_json_parses() {
    let json = r#"{
        "mode": "procedural",
        "scale_core": { "num_stars": 50, "radius": 300.0, "core_radius": 80.0, "lattice_size": 128 },
        "shape": { "shape": "ring", "shape_params": { "band_width": 12.0 } },
        "clustering": {
            "cluster_count": 4,
            "cluster_count_method": "constant",
            "cluster_count_value": 4.0,
            "cluster_count_max": 8,
            "cluster_radius": 60.0,
            "cluster_distance_from_core": 30.0
        },
        "partitioning": {
            "home_system_partitions": 2,
            "open_space_partitions": 1,
            "partition_min_systems": 4,
            "partition_max_systems": 20,
            "partition_min_bridges": 1,
            "partition_max_bridges": 2,
            "partition_method": "breadth_first"
        },
        "hyperlane": {
            "max_hyperlane_distance": 4.0,
            "num_hyperlanes_min": 1,
            "num_hyperlanes_max": 3,
            "num_hyperlanes_default": 2,
            "random_hyperlanes": true
        },
        "special_routes": { "num_wormhole_pairs": 0, "num_gateways": 0 },
        "nebula": { "num_nebulas": 1, "nebula_size": 25.0, "nebula_min_dist": 10.0 },
        "initializers": {
            "initializer_bucket_core": "example_rim_initializer",
            "initializer_bucket_arm": "example_rim_initializer",
            "initializer_bucket_fringe": "example_rim_initializer",
            "initializer_bucket_cluster": "example_rim_initializer",
            "spawn_weight": 1.0,
            "spawn_design": null
        },
        "metadata": {
            "num_empires": 4,
            "fallen_empire_count": 0,
            "marauder_empire_count": 0,
            "advanced_empire_count": 0,
            "colonizable_planet_odds": 0.4,
            "primitive_odds": 0.05,
            "crisis_strength": 1.0,
            "extra_crisis_strength": 0.0
        },
        "arbitrary": {},
        "output": {
            "output_format": "clause",
            "output": null,
            "output_dir": ".",
            "dry_run": true
        },
        "seed": 99,
        "variation_seed": null
    }"#;
    let params = MapGeneratorParams::from_json_str(json).expect("parse json");
    params
        .validate(&registry())
        .expect("parsed params validate");
}

#[test]
fn crate_has_no_forbidden_sim_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "Cargo.toml must not depend on {forbidden}"
        );
    }
}

#[test]
fn dry_run_summary_mentions_no_generation() {
    let params = MapGeneratorParams::default();
    let summary = params.dry_run_summary();
    assert!(summary.contains("no placement"));
    assert!(summary.contains("inert"));
}
