use simthing_mapgenerator::{
    generate_special_routes, place_and_emit_scenario_with_couplings, validate_default,
    validate_special_route_edges, validate_special_route_options, HyperlaneEdge, LatticeCoord,
    MapGenRng, MapGenSeed, MapGeneratorParams, PlacedSystemSeed, ScenarioEmitter,
    ScenarioEmitterConfig, ShapePlacement, ShapeRegistry, SpecialRouteEdge, SpecialRouteError,
    SpecialRouteKind, SpecialRouteOptions, DEFAULT_MAX_PER_NODE_FANOUT,
};

fn grid_placement(count: u32) -> ShapePlacement {
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

fn special_route_options(fixture_edge: u32, wormholes: u32, gateways: u32) -> SpecialRouteOptions {
    SpecialRouteOptions {
        fixture_lattice_edge: fixture_edge,
        num_wormhole_pairs: wormholes,
        num_gateways: gateways,
        max_per_node_fanout: DEFAULT_MAX_PER_NODE_FANOUT,
    }
}

#[test]
fn special_routes_same_seed_are_stable() {
    let placement = grid_placement(6);
    let options = special_route_options(3, 1, 1);
    let mut rng_a = MapGenRng::from_seed(MapGenSeed::new(11));
    let mut rng_b = MapGenRng::from_seed(MapGenSeed::new(11));
    let (left, _) = generate_special_routes(&placement, &options, &[], &mut rng_a).expect("left");
    let (right, _) = generate_special_routes(&placement, &options, &[], &mut rng_b).expect("right");
    assert_eq!(left, right);
}

#[test]
fn special_routes_different_seed_changes_when_possible() {
    let placement = grid_placement(6);
    let options = special_route_options(3, 2, 0);
    let mut rng_a = MapGenRng::from_seed(MapGenSeed::new(1));
    let mut rng_b = MapGenRng::from_seed(MapGenSeed::new(2));
    let (left, _) = generate_special_routes(&placement, &options, &[], &mut rng_a).expect("left");
    let (right, _) = generate_special_routes(&placement, &options, &[], &mut rng_b).expect("right");
    assert_ne!(left.edges, right.edges);
}

#[test]
fn special_routes_reject_self_pairs() {
    let placement = grid_placement(2);
    assert!(validate_special_route_edges(
        &placement,
        &[SpecialRouteEdge {
            kind: SpecialRouteKind::WormholePair,
            from: "0".into(),
            to: "0".into(),
        }]
    )
    .is_err());
}

#[test]
fn special_routes_reject_duplicate_pairs() {
    let placement = grid_placement(2);
    assert!(validate_special_route_edges(
        &placement,
        &[
            SpecialRouteEdge {
                kind: SpecialRouteKind::WormholePair,
                from: "0".into(),
                to: "1".into(),
            },
            SpecialRouteEdge {
                kind: SpecialRouteKind::Gateway,
                from: "0".into(),
                to: "1".into(),
            },
        ]
    )
    .is_err());
}

#[test]
fn special_routes_reject_unknown_endpoint() {
    let placement = grid_placement(2);
    assert!(validate_special_route_edges(
        &placement,
        &[SpecialRouteEdge {
            kind: SpecialRouteKind::Gateway,
            from: "0".into(),
            to: "missing".into(),
        }]
    )
    .is_err());
}

#[test]
fn special_routes_respect_requested_wormhole_pair_count() {
    let placement = grid_placement(6);
    let options = special_route_options(3, 2, 0);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(5));
    let (topology, report) =
        generate_special_routes(&placement, &options, &[], &mut rng).expect("topology");
    assert_eq!(report.wormhole_pair_count, 2);
    assert_eq!(report.gateway_count, 0);
    assert_eq!(
        topology
            .edges
            .iter()
            .filter(|edge| edge.kind == SpecialRouteKind::WormholePair)
            .count(),
        2
    );
}

#[test]
fn special_routes_respect_requested_gateway_count() {
    let placement = grid_placement(6);
    let options = special_route_options(3, 0, 1);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(6));
    let (topology, report) =
        generate_special_routes(&placement, &options, &[], &mut rng).expect("topology");
    assert_eq!(report.gateway_count, 1);
    assert_eq!(
        topology
            .edges
            .iter()
            .filter(|edge| edge.kind == SpecialRouteKind::Gateway)
            .count(),
        1
    );
}

#[test]
fn special_routes_fail_closed_when_count_impossible() {
    let placement = grid_placement(2);
    let options = special_route_options(3, 1, 0);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(1));
    let err = generate_special_routes(&placement, &options, &[], &mut rng).unwrap_err();
    assert_eq!(
        err,
        SpecialRouteError::UnsatisfiedRouteCount {
            kind: SpecialRouteKind::WormholePair,
            requested: 1,
            selected: 0,
        }
    );
}

#[test]
fn special_routes_respect_fanout_cap() {
    let placement = grid_placement(6);
    let mut options = special_route_options(3, 4, 0);
    options.max_per_node_fanout = 1;
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(9));
    let err = generate_special_routes(&placement, &options, &[], &mut rng).unwrap_err();
    assert!(matches!(
        err,
        SpecialRouteError::UnsatisfiedRouteCount { .. }
    ));
}

#[test]
fn special_routes_emit_as_add_hyperlane_pairs_only() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.scale_core.num_stars = 6;
    params.scale_core.lattice_size = Some(8);
    params.scale_core.core_radius = 0.0;
    params.seed = 77;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 1;
    validate_default(&params).expect("params valid");
    let cells: Vec<_> = vec![
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 5, row: 3 },
        LatticeCoord { col: 2, row: 6 },
        LatticeCoord { col: 7, row: 1 },
        LatticeCoord { col: 0, row: 5 },
        LatticeCoord { col: 6, row: 4 },
    ];
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let options = SpecialRouteOptions::from_params(&params, 3);
    let text = place_and_emit_scenario_with_couplings(
        &params,
        &registry,
        Some(&cells),
        &emitter,
        None,
        Some(options),
    )
    .expect("emit")
    .into_string();
    assert!(text.contains("add_hyperlane = {"));
    let lower = text.to_ascii_lowercase();
    for forbidden in ["wormhole", "gateway", "special_route", " route", " path"] {
        assert!(
            !lower.contains(forbidden),
            "forbidden term {forbidden:?} found"
        );
    }
}

#[test]
fn special_routes_emit_no_route_path_predecessor_movement_border_frontline_terms() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.scale_core.num_stars = 6;
    params.scale_core.lattice_size = Some(8);
    params.scale_core.core_radius = 0.0;
    params.seed = 88;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 1;
    validate_default(&params).expect("params valid");
    let cells: Vec<_> = vec![
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 5, row: 3 },
        LatticeCoord { col: 2, row: 6 },
        LatticeCoord { col: 7, row: 1 },
        LatticeCoord { col: 0, row: 5 },
        LatticeCoord { col: 6, row: 4 },
    ];
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let options = SpecialRouteOptions::from_params(&params, 3);
    let text = place_and_emit_scenario_with_couplings(
        &params,
        &registry,
        Some(&cells),
        &emitter,
        None,
        Some(options),
    )
    .expect("emit")
    .into_string();
    let lower = text.to_ascii_lowercase();
    for forbidden in [
        " route",
        " path",
        "predecessor",
        "movement",
        "border",
        "frontline",
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

#[test]
fn validate_special_route_options_rejects_zero_fanout() {
    let mut options = special_route_options(3, 0, 0);
    options.max_per_node_fanout = 0;
    assert_eq!(
        validate_special_route_options(&options).unwrap_err(),
        SpecialRouteError::InvalidFanoutCap
    );
}

#[test]
fn special_routes_skip_pairs_already_selected_as_hyperlanes() {
    let placement = grid_placement(6);
    let options = special_route_options(3, 1, 0);
    let existing = vec![HyperlaneEdge {
        from: "0".into(),
        to: "5".into(),
    }];
    let existing_pairs = vec![(existing[0].from.clone(), existing[0].to.clone())];
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(3));
    let (topology, report) =
        generate_special_routes(&placement, &options, &existing_pairs, &mut rng).expect("topology");
    assert_eq!(report.wormhole_pair_count, 1);
    assert_ne!(topology.edges[0].from, "0");
}
