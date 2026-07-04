use simthing_mapgenerator::{
    canonical_pair, fixture_lattice_edge_for_system_count, generate_hyperlane_topology,
    grid_chebyshev_distance, HyperlaneError, HyperlaneOptions, LatticeCoord, MapGenRng,
    MapGenSeed, MapGeneratorParams, PlacedSystemSeed, ShapePlacement,
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
