//! MapGeneratorCLI PR8 — procedural shape placement and registry tests.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use simthing_mapgenerator::{
    build_placement_context, place_and_emit_scenario, validate_default, LatticeCoord,
    MapGeneratorParams, OccupancyError, ScenarioEmitter, ScenarioEmitterConfig,
    ShapePlacementError, ShapeRegistry, ValidationError,
};

const FORBIDDEN_OUTPUT_TERMS: &[&str] = &[
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "field_operator",
    "sqrt",
    "hypot",
    "distance",
    "normalize",
];

fn shape_params(shape: &str, num_stars: u32, seed: u64, lattice_size: u32) -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = shape.into();
    params.scale_core.num_stars = num_stars;
    params.scale_core.lattice_size = Some(lattice_size);
    params.scale_core.core_radius = 0.0;
    params.seed = seed;
    params
}

fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
    a.col.abs_diff(b.col).max(a.row.abs_diff(b.row))
}

fn run_shape(params: &MapGeneratorParams) -> Vec<LatticeCoord> {
    validate_default(params).expect("valid params");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(params).expect("context");
    let placement = registry
        .place(params, &lattice, &core_mask, &mut occupancy, &mut rng, None)
        .expect("placement");
    assert_eq!(
        placement.systems.len(),
        params.scale_core.num_stars as usize
    );
    placement.systems.iter().map(|s| s.coord).collect()
}

fn assert_shape_invariants(params: &MapGeneratorParams) {
    validate_default(params).expect("valid");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(params).expect("context");
    let placement = registry
        .place(params, &lattice, &core_mask, &mut occupancy, &mut rng, None)
        .expect("placement");
    assert_eq!(
        placement.systems.len(),
        params.scale_core.num_stars as usize
    );
    let mut seen = BTreeSet::new();
    for system in &placement.systems {
        assert!(core_mask.is_placeable(system.coord));
        assert!(seen.insert((system.coord.col, system.coord.row)));
    }
}

fn arm_bucket(coord: LatticeCoord, center: LatticeCoord, num_arms: u32) -> u32 {
    let dc = coord.col as f64 - center.col as f64;
    let dr = coord.row as f64 - center.row as f64;
    let mut angle = dr.atan2(dc);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    ((angle / (2.0 * PI)) * num_arms as f64) as u32 % num_arms
}

fn assert_no_forbidden_terms(text: &str) {
    let lower = text.to_ascii_lowercase();
    for term in FORBIDDEN_OUTPUT_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden term {term:?}"
        );
    }
}

fn assert_procedural_shape_suite(shape: &str, stars: u32, seed: u64, lattice: u32) {
    let params = shape_params(shape, stars, seed, lattice);
    let a = run_shape(&params);
    let b = run_shape(&params);
    assert_eq!(a, b, "{shape} same seed stable");
    assert_shape_invariants(&params);
    let other = run_shape(&shape_params(
        shape,
        stars,
        seed.wrapping_add(9999),
        lattice,
    ));
    if stars > 1 {
        assert_ne!(a, other, "{shape} different seed");
    }
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = place_and_emit_scenario(&params, &registry, None, &emitter)
        .expect("emit")
        .into_string();
    assert!(text.contains("static_galaxy_scenario = {"));
    assert_no_forbidden_terms(&text);
}

#[test]
fn spiral_2_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("spiral_2", 18, 8201, 40);
}

#[test]
fn spiral_3_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("spiral_3", 21, 8301, 40);
}

#[test]
fn spiral_4_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("spiral_4", 24, 8401, 44);
}

#[test]
fn spiral_6_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("spiral_6", 30, 8601, 48);
}

#[test]
fn ring_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("ring", 16, 8701, 36);
}

#[test]
fn bar_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("bar", 14, 8801, 32);
}

#[test]
fn starburst_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("starburst", 20, 8901, 40);
}

#[test]
fn cartwheel_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("cartwheel", 22, 9001, 44);
}

#[test]
fn spoked_same_seed_same_shape_same_placement() {
    assert_procedural_shape_suite("spoked", 18, 9101, 40);
}

#[test]
fn spiral_2_places_across_two_arms() {
    let params = shape_params("spiral_2", 20, 42, 48);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 24, row: 24 };
    let arms: BTreeSet<_> = coords.iter().map(|c| arm_bucket(*c, center, 2)).collect();
    assert_eq!(arms.len(), 2);
}

#[test]
fn spiral_3_places_across_three_arms() {
    let params = shape_params("spiral_3", 24, 43, 48);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 24, row: 24 };
    let arms: BTreeSet<_> = coords.iter().map(|c| arm_bucket(*c, center, 3)).collect();
    assert!(arms.len() >= 3);
}

#[test]
fn spiral_4_places_across_four_arms() {
    let params = shape_params("spiral_4", 28, 44, 52);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 26, row: 26 };
    let arms: BTreeSet<_> = coords.iter().map(|c| arm_bucket(*c, center, 4)).collect();
    assert!(arms.len() >= 4);
}

#[test]
fn spiral_6_places_across_six_arms() {
    let params = shape_params("spiral_6", 36, 46, 56);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 28, row: 28 };
    let arms: BTreeSet<_> = coords.iter().map(|c| arm_bucket(*c, center, 6)).collect();
    assert!(arms.len() >= 6);
}

#[test]
fn ring_places_on_annulus_not_core() {
    let mut params = shape_params("ring", 12, 47, 32);
    params.scale_core.core_radius = 3.0;
    params.scale_core.radius = 10.0;
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let center = lattice.center();
    let placement = registry
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect("ring placement");
    for system in &placement.systems {
        assert!(core_mask.is_placeable(system.coord));
        let dist = chebyshev_distance(system.coord, center);
        assert!(dist >= 2, "ring systems should not cluster at hub: {dist}");
    }
}

#[test]
fn bar_places_on_bounded_elongated_axis() {
    let params = shape_params("bar", 10, 48, 28);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 14, row: 14 };
    let row_spread = coords
        .iter()
        .map(|c| c.row.abs_diff(center.row))
        .max()
        .unwrap_or(0);
    let col_spread = coords
        .iter()
        .map(|c| c.col.abs_diff(center.col))
        .max()
        .unwrap_or(0);
    assert!(col_spread > row_spread);
}

#[test]
fn starburst_places_radial_distribution() {
    let params = shape_params("starburst", 24, 49, 40);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 20, row: 20 };
    let min_dist = coords
        .iter()
        .map(|c| chebyshev_distance(*c, center))
        .min()
        .unwrap_or(0);
    assert!(min_dist >= 1);
}

#[test]
fn cartwheel_places_ring_plus_spokes_or_hub() {
    let params = shape_params("cartwheel", 20, 50, 44);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 22, row: 22 };
    let dists: BTreeSet<_> = coords
        .iter()
        .map(|c| chebyshev_distance(*c, center))
        .collect();
    assert!(dists.len() > 3);
}

#[test]
fn spoked_places_on_radial_spokes() {
    let params = shape_params("spoked", 18, 51, 40);
    let coords = run_shape(&params);
    let center = LatticeCoord { col: 20, row: 20 };
    let arms: BTreeSet<_> = coords.iter().map(|c| arm_bucket(*c, center, 6)).collect();
    assert!(arms.len() >= 3);
}

#[test]
fn insufficient_cells_fail_closed() {
    let params = shape_params("spiral_4", 500, 1, 8);
    validate_default(&params).expect("valid");
    let registry = ShapeRegistry::default();
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let err = registry
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect_err("too many stars");
    assert!(matches!(
        err,
        ShapePlacementError::InsufficientCandidates { .. }
            | ShapePlacementError::Occupancy(OccupancyError::LatticeExhausted)
    ));
}

#[test]
fn procedural_mode_rejects_static_shape_without_explicit_cells() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    let err = validate_default(&params).unwrap_err();
    assert!(matches!(
        err,
        ValidationError::ExplicitShapeInProceduralMode { .. }
    ));
}

#[test]
fn crate_still_has_no_forbidden_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
        "simthing-clausething",
    ] {
        let deps = manifest
            .split("[dev-dependencies]")
            .next()
            .and_then(|s| s.split("[dependencies]").nth(1))
            .unwrap_or("");
        assert!(
            !deps.contains(forbidden),
            "Cargo.toml [dependencies] must not depend on {forbidden}"
        );
    }
}
