use simthing_mapgenerator::{
    assign_partitions, generate_partition_bridges, place_and_emit_scenario_with_structure,
    validate_bridge_edges, validate_default, validate_partition_options, BridgeEdge,
    ClusterOptions, HyperlaneEdge, LatticeCoord, MapGenRng, MapGenSeed, MapGeneratorParams,
    PartitionAssignment, PartitionError, PartitionId, PartitionKind, PartitionOptions,
    PlacedSystemSeed, ShapePlacement, DEFAULT_MAX_PER_NODE_FANOUT,
};

fn grid_placement(count: u32) -> ShapePlacement {
    ShapePlacement {
        systems: (0..count)
            .map(|id| PlacedSystemSeed {
                id,
                coord: simthing_mapgenerator::LatticeCoord { col: id, row: id },
                bucket: None,
            })
            .collect(),
    }
}

fn partition_options(
    home: u32,
    open: u32,
    min_systems: u32,
    max_systems: u32,
    min_bridges: u32,
    max_bridges: u32,
) -> PartitionOptions {
    PartitionOptions {
        fixture_lattice_edge: 3,
        home_system_partitions: home,
        open_space_partitions: open,
        min_systems,
        max_systems,
        min_bridges,
        max_bridges,
        method: simthing_mapgenerator::PartitionMethod::BreadthFirst,
        max_per_node_fanout: DEFAULT_MAX_PER_NODE_FANOUT,
    }
}

#[test]
fn partition_assignment_same_seed_is_stable() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 2);
    let (left, _) = assign_partitions(&placement, &options).expect("left");
    let (right, _) = assign_partitions(&placement, &options).expect("right");
    assert_eq!(left, right);
}

#[test]
fn partition_assignment_rejects_impossible_min_max() {
    let placement = grid_placement(4);
    let options = partition_options(2, 1, 2, 2, 0, 0);
    assert!(matches!(
        assign_partitions(&placement, &options).unwrap_err(),
        PartitionError::UnsatisfiedPartitionStructure { .. }
    ));
}

#[test]
fn partition_assignment_respects_min_max_systems_when_possible() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 2);
    let (assignments, report) = assign_partitions(&placement, &options).expect("assign");
    assert_eq!(report.partition_count, 3);
    let mut sizes = vec![0u32; 3];
    for assignment in &assignments {
        sizes[assignment.partition_id.0 as usize] += 1;
    }
    for size in sizes {
        assert!(size >= options.min_systems);
        assert!(size <= options.max_systems);
    }
}

#[test]
fn bridge_selection_crosses_partitions() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 2);
    let (assignments, _) = assign_partitions(&placement, &options).expect("assign");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(3));
    let (bridges, report) =
        generate_partition_bridges(&placement, &assignments, &options, &[], &mut rng)
            .expect("bridges");
    assert!(report.bridge_count >= options.min_bridges);
    for bridge in &bridges {
        assert_ne!(bridge.from_partition, bridge.to_partition);
    }
}

#[test]
fn bridge_selection_respects_min_max_bridge_counts() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 1);
    let (assignments, _) = assign_partitions(&placement, &options).expect("assign");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(4));
    let (bridges, report) =
        generate_partition_bridges(&placement, &assignments, &options, &[], &mut rng)
            .expect("bridges");
    assert_eq!(bridges.len(), 1);
    assert_eq!(report.bridge_count, 1);
}

#[test]
fn bridge_selection_rejects_duplicate_pairs() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 2);
    assert!(validate_bridge_edges(
        &placement,
        &[
            BridgeEdge {
                from: "0".into(),
                to: "1".into(),
                from_partition: PartitionId(0),
                to_partition: PartitionId(1),
            },
            BridgeEdge {
                from: "0".into(),
                to: "1".into(),
                from_partition: PartitionId(0),
                to_partition: PartitionId(1),
            },
        ]
    )
    .is_err());
}

#[test]
fn bridge_selection_respects_fanout_cap() {
    let placement = grid_placement(9);
    let mut options = partition_options(2, 1, 2, 4, 5, 5);
    options.max_per_node_fanout = 1;
    let (assignments, _) = assign_partitions(&placement, &options).expect("assign");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(5));
    assert!(matches!(
        generate_partition_bridges(&placement, &assignments, &options, &[], &mut rng).unwrap_err(),
        PartitionError::UnsatisfiedBridgeCount { .. }
    ));
}

#[test]
fn bridge_selection_dedups_against_existing_hyperlanes_and_special_routes() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 1, 2);
    let (assignments, _) = assign_partitions(&placement, &options).expect("assign");
    let existing = vec![HyperlaneEdge {
        from: "0".into(),
        to: "8".into(),
    }];
    let existing_pairs = vec![(existing[0].from.clone(), existing[0].to.clone())];
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(6));
    let (bridges, _) = generate_partition_bridges(
        &placement,
        &assignments,
        &options,
        &existing_pairs,
        &mut rng,
    )
    .expect("bridges");
    assert!(bridges
        .iter()
        .all(|bridge| !(bridge.from == "0" && bridge.to == "8")));
}

#[test]
fn partition_options_from_params_roundtrip() {
    let params = MapGeneratorParams::default();
    let options = PartitionOptions::from_params(&params, 3);
    assert_eq!(
        options.home_system_partitions,
        params.partitioning.home_system_partitions
    );
    validate_partition_options(&options).expect("valid");
}

#[test]
fn home_and_open_partition_kinds_are_assigned() {
    let placement = grid_placement(9);
    let options = partition_options(2, 1, 2, 4, 0, 0);
    let (assignments, _) = assign_partitions(&placement, &options).expect("assign");
    assert!(assignments
        .iter()
        .any(|a| a.kind == PartitionKind::HomeSystemPartition));
    assert!(assignments
        .iter()
        .any(|a| a.kind == PartitionKind::OpenSpacePartition));
}

#[test]
fn emitted_partition_bridge_output_uses_add_hyperlane_only() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.scale_core.num_stars = 9;
    params.scale_core.lattice_size = Some(10);
    params.scale_core.core_radius = 0.0;
    params.partitioning.home_system_partitions = 2;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 2;
    params.partitioning.partition_max_systems = 4;
    params.partitioning.partition_min_bridges = 1;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(3);
    params.clustering.cluster_radius = 12.0;
    validate_default(&params).expect("params valid");
    let cells: Vec<_> = vec![
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 1, row: 0 },
        LatticeCoord { col: 2, row: 1 },
        LatticeCoord { col: 0, row: 2 },
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 8, row: 0 },
        LatticeCoord { col: 9, row: 1 },
        LatticeCoord { col: 8, row: 2 },
        LatticeCoord { col: 9, row: 2 },
    ];
    let registry = simthing_mapgenerator::ShapeRegistry::default();
    let emitter = simthing_mapgenerator::ScenarioEmitter::new(
        simthing_mapgenerator::ScenarioEmitterConfig::from_params(&params),
    );
    let text = simthing_mapgenerator::place_and_emit_scenario_with_structure(
        &params,
        &registry,
        Some(&cells),
        &emitter,
        None,
        None,
        Some(PartitionOptions::from_params(&params, 3)),
        Some(ClusterOptions::from_params(&params)),
    )
    .expect("emit")
    .into_string();
    assert!(text.contains("add_hyperlane = {"));
    let lower = text.to_ascii_lowercase();
    for forbidden in ["partition", "cluster", "bridge =", " route", " path"] {
        assert!(
            !lower.contains(forbidden),
            "forbidden term {forbidden:?} found"
        );
    }
}

#[test]
fn emitted_partition_bridge_output_has_no_partition_cluster_bridge_route_path_predecessor_terms() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.scale_core.num_stars = 9;
    params.scale_core.lattice_size = Some(10);
    params.scale_core.core_radius = 0.0;
    params.partitioning.home_system_partitions = 2;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 2;
    params.partitioning.partition_max_systems = 4;
    params.partitioning.partition_min_bridges = 1;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(3);
    params.clustering.cluster_radius = 12.0;
    validate_default(&params).expect("params valid");
    let cells: Vec<_> = vec![
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 1, row: 0 },
        LatticeCoord { col: 2, row: 1 },
        LatticeCoord { col: 0, row: 2 },
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 8, row: 0 },
        LatticeCoord { col: 9, row: 1 },
        LatticeCoord { col: 8, row: 2 },
        LatticeCoord { col: 9, row: 2 },
    ];
    let registry = simthing_mapgenerator::ShapeRegistry::default();
    let emitter = simthing_mapgenerator::ScenarioEmitter::new(
        simthing_mapgenerator::ScenarioEmitterConfig::from_params(&params),
    );
    let text = simthing_mapgenerator::place_and_emit_scenario_with_structure(
        &params,
        &registry,
        Some(&cells),
        &emitter,
        None,
        None,
        Some(PartitionOptions::from_params(&params, 3)),
        Some(ClusterOptions::from_params(&params)),
    )
    .expect("emit")
    .into_string();
    let lower = text.to_ascii_lowercase();
    for forbidden in [
        "predecessor",
        "movement",
        "border",
        "frontline",
        " route",
        " path",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden term {forbidden:?} found"
        );
    }
}

#[test]
fn crate_still_has_no_forbidden_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-clausething",
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "forbidden dependency {forbidden} in Cargo.toml"
        );
    }
}
