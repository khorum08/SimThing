//! MapGeneratorCLI PR11 — producer scale envelope tests (1000-star bounded algorithms).

use simthing_mapgenerator::{
    assign_clusters, assign_partitions, build_placement_context,
    collect_farthest_pairs_with_filter, collect_pairs_within_chebyshev,
    fixture_lattice_edge_for_system_count, generate_cluster_bridges, generate_hyperlane_topology,
    generate_partition_bridges, generate_special_routes, place_and_emit_scenario_with_structure,
    validate_default, ClusterOptions, CoreMask, HyperlaneError, HyperlaneOptions, LatticeError,
    MapGenRng, MapGenSeed, MapGeneratorParams, OccupancyGrid, PartitionOptions, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, SpecialRouteOptions, SquareLattice,
    PRODUCER_PAIR_CANDIDATE_CAP,
};

const NUM_STARS_1000: u32 = 1000;
const PR11_SEED: u64 = 11_000;

fn pr11_1000_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.mode = simthing_mapgenerator::GenerationMode::Procedural;
    params.scale_core.num_stars = NUM_STARS_1000;
    params.scale_core.lattice_size = Some(50);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = PR11_SEED;
    params.hyperlane.max_hyperlane_distance = 3.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 12;
    params.hyperlane.num_hyperlanes_default = 6;
    params.hyperlane.random_hyperlanes = false;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 0;
    params.partitioning.home_system_partitions = 1;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 100;
    params.partitioning.partition_max_systems = 900;
    params.partitioning.partition_min_bridges = 0;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(4);
    params.clustering.cluster_radius = 50.0;
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 12.0;
    params.nebula.nebula_min_dist = 4.0;
    params
}

fn generate_1000_placement() -> simthing_mapgenerator::ShapePlacement {
    let params = pr11_1000_params();
    validate_default(&params).expect("params valid");
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("placement context");
    ShapeRegistry::default()
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect("1000-star placement")
}

#[test]
fn lattice_capacity_uses_u64_or_fails_closed() {
    let edge = 65_536u32;
    let lattice = SquareLattice::new(edge).expect("edge");
    assert_eq!(lattice.cell_count_u64(), 4_294_967_296);
    assert_eq!(
        lattice.try_cell_count(),
        Err(LatticeError::CapacityOverflow {
            edge: edge as u64,
            capacity: 4_294_967_296,
        })
    );
}

#[test]
fn lattice_capacity_does_not_wrap() {
    let lattice = SquareLattice::new(65_535).expect("edge");
    assert_eq!(lattice.cell_count(), 4_294_836_225);
    assert_eq!(lattice.try_cell_count().expect("fits u32"), 4_294_836_225);
}

#[test]
fn occupancy_relocation_does_not_rebuild_placeable_cells_per_insert() {
    let lattice = SquareLattice::new(40).expect("lattice");
    let mask = CoreMask::new(20, 20, 0);
    let mut grid = OccupancyGrid::new(lattice, mask);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(42));
    for _ in 0..500 {
        grid.insert_next(&mut rng).expect("place");
    }
    assert_eq!(grid.placeable_full_scan_count(), 0);
}

#[test]
fn occupancy_1000_star_same_seed_is_stable() {
    let run = |seed: u64| {
        let mut params = pr11_1000_params();
        params.seed = seed;
        let (lattice, core_mask, mut occupancy, mut rng) =
            build_placement_context(&params).expect("context");
        ShapeRegistry::default()
            .place(
                &params,
                &lattice,
                &core_mask,
                &mut occupancy,
                &mut rng,
                None,
            )
            .expect("place")
            .systems
            .iter()
            .map(|system| (system.id, system.coord))
            .collect::<Vec<_>>()
    };
    assert_eq!(run(PR11_SEED), run(PR11_SEED));
}

#[test]
fn occupancy_1000_star_one_system_per_cell() {
    let placement = generate_1000_placement();
    assert_eq!(placement.systems.len(), NUM_STARS_1000 as usize);
    let mut seen = std::collections::BTreeSet::new();
    for system in &placement.systems {
        assert!(seen.insert((system.coord.col, system.coord.row)));
    }
}

#[test]
fn occupancy_1000_star_core_mask_respected() {
    let params = pr11_1000_params();
    let (lattice, core_mask, _, _) = build_placement_context(&params).expect("context");
    let _ = core_mask;
    let placement = generate_1000_placement();
    for system in &placement.systems {
        assert!(lattice.contains(system.coord));
        assert!(core_mask.is_placeable(system.coord));
    }
}

#[test]
fn hyperlane_1000_star_candidate_bound_respected() {
    let placement = generate_1000_placement();
    let fixture_edge =
        fixture_lattice_edge_for_system_count(placement.systems.len()).expect("edge");
    let options = HyperlaneOptions::from_params(&pr11_1000_params(), fixture_edge);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(PR11_SEED));
    let (_topology, report) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("hyperlanes");
    assert!(report.candidate_count <= PRODUCER_PAIR_CANDIDATE_CAP as u32);
    assert!(!report.candidate_cap_hit);
}

#[test]
fn special_route_1000_star_candidate_bound_respected() {
    let placement = generate_1000_placement();
    let fixture_edge =
        fixture_lattice_edge_for_system_count(placement.systems.len()).expect("edge");
    let options = SpecialRouteOptions::from_params(&pr11_1000_params(), fixture_edge);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(PR11_SEED));
    let (_topology, report) =
        generate_special_routes(&placement, &options, &[], &mut rng).expect("special routes");
    assert!(report.long_range_candidate_count <= PRODUCER_PAIR_CANDIDATE_CAP as u32);
}

#[test]
fn partition_bridge_1000_star_candidate_bound_respected() {
    let placement = generate_1000_placement();
    let params = pr11_1000_params();
    let fixture_edge =
        fixture_lattice_edge_for_system_count(placement.systems.len()).expect("edge");
    let options = PartitionOptions::from_params(&params, fixture_edge);
    let (assignments, _) = assign_partitions(&placement, &options).expect("partitions");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(PR11_SEED));
    let (_, report) = generate_partition_bridges(&placement, &assignments, &options, &[], &mut rng)
        .expect("partition bridges");
    assert!(report.bridge_candidate_count <= PRODUCER_PAIR_CANDIDATE_CAP as u32);
}

#[test]
fn cluster_bridge_1000_star_candidate_bound_respected() {
    let placement = generate_1000_placement();
    let params = pr11_1000_params();
    let fixture_edge =
        fixture_lattice_edge_for_system_count(placement.systems.len()).expect("edge");
    let cluster_options = ClusterOptions::from_params(&params);
    let (assignments, _) = assign_clusters(&placement, &cluster_options).expect("clusters");
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(PR11_SEED));
    let (_, report) = generate_cluster_bridges(
        &placement,
        &assignments,
        fixture_edge,
        0,
        2,
        4,
        &[],
        &mut rng,
    )
    .expect("cluster bridges");
    assert!(report.cluster_bridge_count <= 2);
    assert!(report.examined_pairs > 0);
}

#[test]
fn generated_1000_star_output_has_no_forbidden_terms() {
    let params = pr11_1000_params();
    validate_default(&params).expect("valid");
    let fixture_edge =
        fixture_lattice_edge_for_system_count(NUM_STARS_1000 as usize).expect("edge");
    let text = place_and_emit_scenario_with_structure(
        &params,
        &ShapeRegistry::default(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(HyperlaneOptions::from_params(&params, fixture_edge)),
        Some(SpecialRouteOptions::from_params(&params, fixture_edge)),
        Some(PartitionOptions::from_params(&params, fixture_edge)),
        Some(ClusterOptions::from_params(&params)),
    )
    .expect("emit 1000-star scenario")
    .into_string();
    let lower = text.to_ascii_lowercase();
    for forbidden in [
        "pathfinding",
        "predecessor",
        "movement_order",
        " route",
        " border",
        "frontline",
        "field_operator",
        "semantic_wgsl",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden term {forbidden:?} in emitted text"
        );
    }
    assert!(text.contains("static_galaxy_scenario"));
    assert!(text.matches("    system = {").count() >= NUM_STARS_1000 as usize);
}

#[test]
fn producer_still_has_no_forbidden_runtime_deps() {
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
            "producer must not depend on {forbidden}"
        );
    }
}

#[test]
fn pair_enumeration_helpers_bound_examined_pairs_for_local_window() {
    let positions: Vec<(u32, u32)> = (0..1000u32).map(|index| (index / 32, index % 32)).collect();
    let (pairs, stats) = collect_pairs_within_chebyshev(&positions, 3);
    assert!(stats.examined_pairs < 1_000_000);
    assert!(!pairs.is_empty());
}

#[test]
fn farthest_pair_helper_respects_cap() {
    let positions: Vec<(u32, u32)> = (0..120u32).map(|index| (index / 12, index % 12)).collect();
    let (pairs, stats) =
        collect_farthest_pairs_with_filter(&positions, |_, _, distance| distance > 1);
    assert!(pairs.len() <= PRODUCER_PAIR_CANDIDATE_CAP);
    assert!(stats.examined_pairs >= pairs.len() as u64);
}

#[test]
fn fixture_lattice_edge_rejects_impossible_system_count() {
    let err = fixture_lattice_edge_for_system_count(usize::MAX).unwrap_err();
    assert!(matches!(err, HyperlaneError::FixtureLatticeOverflow { .. }));
}
