use simthing_mapgenerator::{
    build_placement_context, place_nebulas, LatticeCoord, MapGenRng, MapGenSeed,
    MapGeneratorParams, NebulaError, NebulaOptions, PlacedSystemSeed, ShapePlacement,
};

fn sample_placement() -> ShapePlacement {
    ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 4, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 2,
                coord: LatticeCoord { col: 8, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 3,
                coord: LatticeCoord { col: 0, row: 4 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 4,
                coord: LatticeCoord { col: 8, row: 4 },
                bucket: None,
            },
        ],
    }
}

fn nebula_params(count: u32, size: f64, min_dist: f64, seed: u64) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.nebula.num_nebulas = count;
    params.nebula.nebula_size = size;
    params.nebula.nebula_min_dist = min_dist;
    params.seed = seed;
    params
}

#[test]
fn nebula_same_seed_is_stable() {
    let params = nebula_params(2, 20.0, 3.0, 9091);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let placement = sample_placement();
    let options = NebulaOptions::from_params(&params);
    let mut rng_a = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let mut rng_b = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let (a, _) = place_nebulas(&placement, &lattice, options, &mut rng_a).expect("place a");
    let (b, _) = place_nebulas(&placement, &lattice, options, &mut rng_b).expect("place b");
    assert_eq!(a, b);
}

#[test]
fn nebula_count_respects_request_when_possible() {
    let params = nebula_params(2, 15.0, 2.0, 9092);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let options = NebulaOptions::from_params(&params);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let (nebulas, report) =
        place_nebulas(&sample_placement(), &lattice, options, &mut rng).expect("place");
    assert_eq!(report.requested_count, 2);
    assert_eq!(report.placed_count, 2);
    assert_eq!(nebulas.len(), 2);
}

#[test]
fn nebula_min_distance_respected() {
    let params = nebula_params(2, 10.0, 4.0, 9093);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let options = NebulaOptions::from_params(&params);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let (nebulas, _) =
        place_nebulas(&sample_placement(), &lattice, options, &mut rng).expect("place");
    assert_eq!(nebulas.len(), 2);
    let d = simthing_mapgenerator::topology::grid_chebyshev_distance(
        (nebulas[0].center.col, nebulas[0].center.row),
        (nebulas[1].center.col, nebulas[1].center.row),
    );
    assert!(d >= options.min_center_distance);
}

#[test]
fn nebula_impossible_request_fails_closed() {
    let params = nebula_params(5, 10.0, 20.0, 9094);
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let options = NebulaOptions::from_params(&params);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let err = place_nebulas(&sample_placement(), &lattice, options, &mut rng).unwrap_err();
    assert!(matches!(err, NebulaError::ImpossibleRequest { .. }));
}
