use simthing_mapgenerator::{
    assign_clusters, generate_cluster_bridges, validate_cluster_options, ClusterAssignment,
    ClusterError, ClusterOptions, LatticeCoord, MapGenRng, MapGenSeed, MapGeneratorParams,
    PlacedSystemSeed, ShapePlacement, DEFAULT_MAX_PER_NODE_FANOUT,
};

fn clustered_placement() -> ShapePlacement {
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
                coord: LatticeCoord { col: 8, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 3,
                coord: LatticeCoord { col: 9, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 4,
                coord: LatticeCoord { col: 0, row: 8 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 5,
                coord: LatticeCoord { col: 1, row: 8 },
                bucket: None,
            },
        ],
    }
}

fn cluster_options(count: u32, radius: u32) -> ClusterOptions {
    ClusterOptions {
        target_cluster_count: count,
        cluster_radius: radius,
        cluster_distance_from_core: 0,
    }
}

#[test]
fn cluster_assignment_same_seed_is_stable() {
    let placement = clustered_placement();
    let options = cluster_options(3, 10);
    let (left, _) = assign_clusters(&placement, &options).expect("left");
    let (right, _) = assign_clusters(&placement, &options).expect("right");
    assert_eq!(left, right);
}

#[test]
fn cluster_assignment_respects_requested_cluster_count_when_possible() {
    let placement = clustered_placement();
    let options = cluster_options(3, 10);
    let (assignments, report) = assign_clusters(&placement, &options).expect("assign");
    assert_eq!(report.target_cluster_count, 3);
    assert_eq!(assignments.len(), 6);
    assert!(report.assigned_cluster_count >= 2);
}

#[test]
fn cluster_assignment_fails_closed_when_impossible() {
    let placement = clustered_placement();
    let options = cluster_options(9, 10);
    assert_eq!(
        assign_clusters(&placement, &options).unwrap_err(),
        ClusterError::UnsatisfiedClusterCount {
            target_cluster_count: 9,
            system_count: 6,
        }
    );
}

#[test]
fn cluster_assignment_fails_when_radius_too_small() {
    let placement = clustered_placement();
    let options = cluster_options(3, 1);
    assert!(matches!(
        assign_clusters(&placement, &options).unwrap_err(),
        ClusterError::UnsatisfiedClusterRadius { .. }
    ));
}

#[test]
fn cluster_bridge_generation_produces_cross_cluster_pairs() {
    let placement = clustered_placement();
    let options = cluster_options(3, 10);
    let (assignments, _) = assign_clusters(&placement, &options).expect("assign");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(2));
    let (bridges, report) = generate_cluster_bridges(
        &placement,
        &assignments,
        3,
        1,
        2,
        DEFAULT_MAX_PER_NODE_FANOUT,
        &[],
        &mut rng,
    )
    .expect("bridges");
    assert!(report.cluster_bridge_count >= 1);
    for bridge in &bridges {
        assert_ne!(bridge.from_cluster, bridge.to_cluster);
    }
}

#[test]
fn cluster_options_from_params_roundtrip() {
    let params = MapGeneratorParams::default();
    let options = ClusterOptions::from_params(&params);
    validate_cluster_options(&options).expect("valid");
}

#[test]
fn cluster_assignments_cover_all_systems() {
    let placement = clustered_placement();
    let options = cluster_options(2, 10);
    let (assignments, _) = assign_clusters(&placement, &options).expect("assign");
    assert_eq!(assignments.len(), placement.systems.len());
}
