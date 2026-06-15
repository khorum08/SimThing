use simthing_mapgenerator::{
    canonical_pair, fixture_lattice_edge_for_system_count, generate_hyperlane_topology,
    grid_chebyshev_distance, validate_hyperlane_edges, HyperlaneError, HyperlaneOptions,
    LatticeCoord, MapGenRng, MapGenSeed, MapGeneratorParams, PlacedSystemSeed, ShapePlacement,
};

fn line_placement(count: u32) -> ShapePlacement {
    ShapePlacement {
        systems: (0..count)
            .map(|id| PlacedSystemSeed {
                id,
                coord: LatticeCoord { col: id, row: id },
                bucket: None,
            })
            .collect(),
    }
}

fn hyperlane_options(fixture_edge: u32) -> HyperlaneOptions {
    let mut params = MapGeneratorParams::default();
    params.hyperlane.max_hyperlane_distance = 4.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 4;
    params.hyperlane.num_hyperlanes_default = 3;
    params.hyperlane.random_hyperlanes = false;
    HyperlaneOptions::from_params(&params, fixture_edge)
}

#[test]
fn hyperlane_edges_are_deterministic_for_same_seed() {
    let placement = line_placement(4);
    let options = hyperlane_options(3);
    let mut rng_a = MapGenRng::from_seed(MapGenSeed::new(99));
    let mut rng_b = MapGenRng::from_seed(MapGenSeed::new(99));
    let (left, _) = generate_hyperlane_topology(&placement, &options, &mut rng_a).expect("left");
    let (right, _) = generate_hyperlane_topology(&placement, &options, &mut rng_b).expect("right");
    assert_eq!(left, right);
}

#[test]
fn hyperlane_edges_change_with_different_seed_when_randomized() {
    let placement = line_placement(6);
    let mut options = hyperlane_options(3);
    options.random_hyperlanes = true;
    options.target_edge_count = 4;
    let mut rng_a = MapGenRng::from_seed(MapGenSeed::new(1));
    let mut rng_b = MapGenRng::from_seed(MapGenSeed::new(2));
    let (left, _) = generate_hyperlane_topology(&placement, &options, &mut rng_a).expect("left");
    let (right, _) = generate_hyperlane_topology(&placement, &options, &mut rng_b).expect("right");
    assert_ne!(left.edges, right.edges);
}

#[test]
fn hyperlane_edges_are_canonical_order() {
    let placement = line_placement(3);
    let options = hyperlane_options(2);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(7));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    for edge in &topology.edges {
        assert!(edge.from <= edge.to);
        assert_eq!(
            canonical_pair(&edge.from, &edge.to),
            (edge.from.clone(), edge.to.clone())
        );
    }
}

#[test]
fn hyperlane_generation_rejects_self_links() {
    let placement = line_placement(2);
    assert!(validate_hyperlane_edges(&placement, &[("0".into(), "0".into())]).is_err());
}

#[test]
fn hyperlane_generation_rejects_duplicates() {
    let placement = line_placement(2);
    assert!(validate_hyperlane_edges(
        &placement,
        &[("0".into(), "1".into()), ("0".into(), "1".into())]
    )
    .is_err());
}

#[test]
fn hyperlane_generation_respects_prevent_pairs() {
    let placement = line_placement(4);
    let mut options = hyperlane_options(3);
    options.prevent_pairs = vec![("0".into(), "1".into())];
    options.target_edge_count = 4;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(5));
    let (topology, report) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    assert!(report.rejected_prevent >= 1);
    assert!(!topology
        .edges
        .iter()
        .any(|edge| canonical_pair(&edge.from, &edge.to) == ("0".into(), "1".into())));
}

#[test]
fn hyperlane_generation_respects_max_hyperlane_distance() {
    let placement = line_placement(4);
    let mut options = hyperlane_options(3);
    options.max_hyperlane_distance = 1;
    options.target_edge_count = 10;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(11));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    // STEAD: edges must respect the bound on AUTHORED structural coordinates, not lowered index-order.
    let positions: Vec<(u32, u32)> = placement
        .systems
        .iter()
        .map(|system| (system.coord.row, system.coord.col))
        .collect();
    for edge in &topology.edges {
        let left = edge.from.parse::<usize>().expect("left id");
        let right = edge.to.parse::<usize>().expect("right id");
        assert!(grid_chebyshev_distance(positions[left], positions[right]) <= 1);
    }
}

#[test]
fn hyperlane_generation_respects_min_max_edge_counts() {
    let placement = line_placement(5);
    let mut options = hyperlane_options(3);
    options.min_edge_count = 2;
    options.max_edge_count = 2;
    options.target_edge_count = 10;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(3));
    let (topology, report) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    assert_eq!(topology.edges.len(), 2);
    assert_eq!(report.selected_count, 2);
}

#[test]
fn hyperlane_generation_respects_per_node_fanout_cap() {
    let placement = line_placement(5);
    let mut options = hyperlane_options(3);
    options.max_per_node_fanout = 1;
    options.target_edge_count = 10;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(8));
    let (topology, report) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    let mut fanout = std::collections::BTreeMap::<String, u32>::new();
    for edge in &topology.edges {
        *fanout.entry(edge.from.clone()).or_insert(0) += 1;
        *fanout.entry(edge.to.clone()).or_insert(0) += 1;
    }
    for count in fanout.values() {
        assert!(*count <= options.max_per_node_fanout);
    }
    assert!(report.rejected_fanout >= 1);
}

#[test]
fn fixture_lattice_edge_for_system_count_matches_capacity() {
    assert_eq!(fixture_lattice_edge_for_system_count(4).expect("edge"), 2);
    assert_eq!(fixture_lattice_edge_for_system_count(5).expect("edge"), 3);
}

#[test]
fn hyperlane_generation_rejects_min_greater_than_max() {
    let placement = line_placement(4);
    let mut options = hyperlane_options(3);
    options.min_edge_count = 5;
    options.max_edge_count = 2;
    options.target_edge_count = 2;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(1));
    let err = generate_hyperlane_topology(&placement, &options, &mut rng).unwrap_err();
    assert_eq!(
        err,
        HyperlaneError::InvalidEdgeCounts {
            min_edge_count: 5,
            max_edge_count: 2,
            target_edge_count: 2,
        }
    );
}

#[test]
fn hyperlane_generation_rejects_zero_fanout_cap() {
    let placement = line_placement(4);
    let mut options = hyperlane_options(3);
    options.max_per_node_fanout = 0;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(1));
    assert_eq!(
        generate_hyperlane_topology(&placement, &options, &mut rng).unwrap_err(),
        HyperlaneError::InvalidFanoutCap
    );
}

#[test]
fn hyperlane_generation_errors_when_min_edge_count_cannot_be_satisfied() {
    let placement = line_placement(2);
    let mut options = hyperlane_options(3);
    options.min_edge_count = 2;
    options.max_edge_count = 4;
    options.target_edge_count = 4;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(1));
    let err = generate_hyperlane_topology(&placement, &options, &mut rng).unwrap_err();
    assert_eq!(
        err,
        HyperlaneError::UnsatisfiedMinEdgeCount {
            selected_count: 1,
            min_edge_count: 2,
        }
    );
}

#[test]
fn hyperlane_generation_does_not_panic_on_invalid_public_options() {
    let placement = line_placement(4);
    let base = hyperlane_options(3);
    let invalid_options = [
        HyperlaneOptions {
            fixture_lattice_edge: 0,
            ..base.clone()
        },
        HyperlaneOptions {
            min_edge_count: 10,
            max_edge_count: 1,
            target_edge_count: 1,
            ..base.clone()
        },
        HyperlaneOptions {
            max_per_node_fanout: 0,
            ..base.clone()
        },
    ];
    for options in invalid_options {
        let mut rng = MapGenRng::from_seed(MapGenSeed::new(42));
        assert!(generate_hyperlane_topology(&placement, &options, &mut rng).is_err());
    }
}

#[test]
fn existing_hyperlane_generation_happy_path_still_passes() {
    let placement = line_placement(4);
    let options = hyperlane_options(3);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(99));
    let (topology, report) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("happy path");
    assert!(!topology.edges.is_empty());
    assert!(report.selected_count >= options.min_edge_count);
    assert!(report.selected_count <= options.max_edge_count);
}
