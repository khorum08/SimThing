use simthing_mapgenerator::{
    build_placement_context, validate_default, LatticeCoord, MapGeneratorParams, ShapeRegistry,
};

fn test_params(shape: &str, num_stars: u32, seed: u64, lattice_size: u32) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = shape.into();
    params.scale_core.num_stars = num_stars;
    params.scale_core.lattice_size = Some(lattice_size);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params
}

fn static_params(seed: u64, lattice_size: u32) -> MapGeneratorParams {
    let mut params = test_params("static", 1, seed, lattice_size);
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params
}

fn run_elliptical(params: &MapGeneratorParams) -> Vec<LatticeCoord> {
    validate_default(params).expect("valid params");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(params).expect("context");
    let placement = registry
        .place(params, &lattice, &core_mask, &mut occupancy, &mut rng, None)
        .expect("elliptical placement");
    placement.systems.iter().map(|s| s.coord).collect()
}

#[test]
fn registry_resolves_elliptical_strategy_by_name() {
    let registry = ShapeRegistry::default();
    let strategy = registry
        .resolve_strategy("elliptical")
        .expect("elliptical executable");
    assert_eq!(strategy.name(), "elliptical");
}

#[test]
fn registry_resolves_static_strategy_by_name() {
    let registry = ShapeRegistry::default();
    registry
        .resolve_strategy("static")
        .expect("static executable");
    registry
        .resolve_strategy("arbitrary_static")
        .expect("arbitrary_static executable");
}

#[test]
fn registry_unknown_shape_lists_registered_shapes() {
    let registry = ShapeRegistry::default();
    let err = match registry.resolve_strategy("not_a_shape") {
        Err(err) => err,
        Ok(_) => panic!("expected unknown shape error"),
    };
    let msg = err.to_string();
    assert!(msg.contains("not_a_shape"));
    assert!(msg.contains("elliptical"));
    assert!(msg.contains("registered shapes"));
}

#[test]
fn adding_strategy_is_registry_data_driven() {
    let registry = ShapeRegistry::default();
    assert!(!registry.contains("modded_custom_shape"));
    assert!(registry.contains("elliptical"));
    assert!(registry.resolve_strategy("spiral_4").is_ok());
}

#[test]
fn elliptical_same_seed_is_stable() {
    let params = test_params("elliptical", 12, 99, 32);
    let a = run_elliptical(&params);
    let b = run_elliptical(&params);
    assert_eq!(a, b);
    assert_eq!(a.len(), 12);
}

#[test]
fn elliptical_different_seed_differs_when_possible() {
    let mut a_params = test_params("elliptical", 20, 1, 40);
    let mut b_params = test_params("elliptical", 20, 2, 40);
    a_params.scale_core.core_radius = 5.0;
    b_params.scale_core.core_radius = 5.0;
    let a = run_elliptical(&a_params);
    let b = run_elliptical(&b_params);
    assert_ne!(a, b);
}

#[test]
fn elliptical_respects_core_mask() {
    let mut params = test_params("elliptical", 8, 7, 24);
    params.scale_core.core_radius = 40.0;
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let placement = registry
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect("placement");
    for system in &placement.systems {
        assert!(
            core_mask.is_placeable(system.coord),
            "placed inside core mask: {:?}",
            system.coord
        );
    }
}

#[test]
fn elliptical_enforces_one_system_per_cell() {
    let params = test_params("elliptical", 15, 3, 20);
    let coords = run_elliptical(&params);
    let mut seen = std::collections::BTreeSet::new();
    for coord in coords {
        assert!(seen.insert((coord.col, coord.row)));
    }
}

#[test]
fn static_strategy_accepts_explicit_integer_cells() {
    let params = static_params(1, 10);
    validate_default(&params).expect("valid");
    let explicit = [
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 3, row: 4 },
        LatticeCoord { col: 9, row: 9 },
    ];
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let placement = registry
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            Some(&explicit),
        )
        .expect("static placement");
    assert_eq!(placement.systems.len(), 3);
    assert_eq!(placement.systems[0].coord, explicit[0]);
}

#[test]
fn strategy_output_has_no_links_or_runtime_payloads() {
    let params = test_params("elliptical", 5, 11, 16);
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let placement = registry
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect("placement");
    for system in &placement.systems {
        assert!(system.bucket.is_some());
        // Output is id + coord + optional bucket label only — no links/fields/runtime structs.
        let _ = (system.id, system.coord);
    }
}

#[test]
fn crate_still_has_no_forbidden_sim_runtime_deps() {
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
