//! MAPGENCLI-TOPOLOGY-STEAD-0 — producer base-hyperlane adjacency is selected from AUTHORED structural
//! gridcell coordinates (`PlacedSystemSeed.coord`), never lowered index-order / emission order.
//!
//! These regressions prove the fix end-to-end through the public selected-edge output (there is no public
//! candidate list): authored-near systems connect even when far apart in emission order, and
//! emission-order-adjacent systems do not connect when they are far apart in authored coordinates.

use std::collections::BTreeSet;

use simthing_mapgenerator::{
    build_placement_context, canonical_pair, generate_hyperlane_topology, validate_default,
    HyperlaneOptions, LatticeCoord, MapGenRng, MapGenSeed, MapGeneratorParams, PlacedSystemSeed,
    ShapePlacement, ShapeRegistry,
};

/// Authored-coordinate Chebyshev distance between two seeds (integer; no sqrt).
fn authored_chebyshev(a: &PlacedSystemSeed, b: &PlacedSystemSeed) -> u32 {
    a.coord
        .col
        .abs_diff(b.coord.col)
        .max(a.coord.row.abs_diff(b.coord.row))
}

/// A placement whose emission order is deliberately decorrelated from authored proximity:
/// - system "0" (0,0) and system "5" (1,0) are authored-ADJACENT but emission indices 0 and 5 (far).
/// - system "0" (0,0) and system "1" (20,0) are emission-ADJACENT but authored-far (Chebyshev 20).
/// The four filler systems are scattered far from everything (no incidental candidates at distance ≤ 1).
fn decorrelated_placement() -> ShapePlacement {
    let coords = [
        (0u32, 0u32), // id 0  — A
        (20, 0),      // id 1  — X (emission-adjacent to A, authored-far)
        (20, 20),     // id 2  — Y
        (0, 20),      // id 3  — Z
        (10, 10),     // id 4  — W
        (1, 0),       // id 5  — B (emission-far from A, authored-adjacent)
    ];
    ShapePlacement {
        systems: coords
            .iter()
            .enumerate()
            .map(|(id, (col, row))| PlacedSystemSeed {
                id: id as u32,
                coord: LatticeCoord {
                    col: *col,
                    row: *row,
                },
                bucket: None,
            })
            .collect(),
    }
}

fn unit_distance_options() -> HyperlaneOptions {
    let mut params = MapGeneratorParams::default();
    params.hyperlane.max_hyperlane_distance = 1.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 8;
    params.hyperlane.num_hyperlanes_default = 8;
    params.hyperlane.random_hyperlanes = false;
    // fixture_lattice_edge is deprecated for adjacency; pass a small value to prove it is unused.
    HyperlaneOptions::from_params(&params, 3)
}

fn selected_pairs(
    topology: &simthing_mapgenerator::HyperlaneTopology,
) -> BTreeSet<(String, String)> {
    topology
        .edges
        .iter()
        .map(|edge| canonical_pair(&edge.from, &edge.to))
        .collect()
}

#[test]
fn hyperlane_candidates_use_authored_structural_coords_not_index_order() {
    let placement = decorrelated_placement();
    let options = unit_distance_options();
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(99));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");

    // Every selected edge is local in AUTHORED coordinates (≤ max_hyperlane_distance Chebyshev).
    for edge in &topology.edges {
        let left = edge.from.parse::<usize>().expect("left id");
        let right = edge.to.parse::<usize>().expect("right id");
        assert!(
            authored_chebyshev(&placement.systems[left], &placement.systems[right])
                <= options.max_hyperlane_distance,
            "edge {edge:?} exceeds authored Chebyshev bound"
        );
    }
}

#[test]
fn near_in_authored_coords_gets_candidate_even_if_far_in_index_order() {
    let placement = decorrelated_placement();
    let options = unit_distance_options();
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(99));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    let pairs = selected_pairs(&topology);
    // A(0,0)/B(1,0) are authored-adjacent but emission indices 0 and 5 — must connect.
    assert!(
        pairs.contains(&("0".to_string(), "5".to_string())),
        "authored-adjacent A/B must be selected despite far emission order; got {pairs:?}"
    );
}

#[test]
fn far_in_authored_coords_does_not_get_candidate_even_if_adjacent_in_index_order() {
    let placement = decorrelated_placement();
    let options = unit_distance_options();
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(99));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    let pairs = selected_pairs(&topology);
    // A(0,0)/X(20,0) are emission-adjacent (idx 0/1) but authored Chebyshev 20 — must NOT connect.
    assert!(
        !pairs.contains(&("0".to_string(), "1".to_string())),
        "emission-adjacent but authored-far A/X must not be selected; got {pairs:?}"
    );
}

#[test]
fn generated_edges_respect_authored_chebyshev_distance_bound() {
    let mut options = unit_distance_options();
    options.max_hyperlane_distance = 1;
    let placement = decorrelated_placement();
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(7));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("topology");
    for edge in &topology.edges {
        let left = edge.from.parse::<usize>().expect("left id");
        let right = edge.to.parse::<usize>().expect("right id");
        assert!(authored_chebyshev(&placement.systems[left], &placement.systems[right]) <= 1);
    }
}

// ---- 1500-star spiral: base hyperlanes are local in the authored layout, with no degenerate edges ----

const NUM_STARS_1500: u32 = 1500;
const SPIRAL_SEED: u64 = 1_500_777;

fn spiral_1500_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_4".into();
    params.mode = simthing_mapgenerator::GenerationMode::Procedural;
    params.scale_core.num_stars = NUM_STARS_1500;
    params.scale_core.lattice_size = Some(300);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = SPIRAL_SEED;
    params.hyperlane.max_hyperlane_distance = 3.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 12;
    params.hyperlane.num_hyperlanes_default = 8;
    params.hyperlane.random_hyperlanes = false;
    params
}

fn spiral_1500_placement() -> ShapePlacement {
    let params = spiral_1500_params();
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
        .expect("1500-star spiral placement")
}

fn spiral_1500_topology() -> (ShapePlacement, simthing_mapgenerator::HyperlaneTopology) {
    let placement = spiral_1500_placement();
    let options = HyperlaneOptions::from_params(&spiral_1500_params(), 1);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(SPIRAL_SEED));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut rng).expect("spiral hyperlanes");
    (placement, topology)
}

#[test]
fn spiral_1500_base_hyperlanes_have_no_self_links() {
    let (_, topology) = spiral_1500_topology();
    for edge in &topology.edges {
        assert_ne!(edge.from, edge.to, "self-link {edge:?}");
    }
}

#[test]
fn spiral_1500_base_hyperlanes_have_no_unknown_endpoints() {
    let (placement, topology) = spiral_1500_topology();
    let ids: BTreeSet<String> = placement.systems.iter().map(|s| s.id.to_string()).collect();
    for edge in &topology.edges {
        assert!(ids.contains(&edge.from), "unknown endpoint {}", edge.from);
        assert!(ids.contains(&edge.to), "unknown endpoint {}", edge.to);
    }
}

#[test]
fn spiral_1500_placement_is_spatially_dispersed_not_a_brick() {
    // Integrity guard: the 1500 gridcells must be a SPARSE, dispersed spiral over the lattice — never a
    // contiguous row-major brick. The renderer draws each star at its authored `coord` (+ sub-cell jitter),
    // so this is exactly what the preview shows. A brick would be near-100% fill in a tight rectangle with
    // ~edge stars per row; a spiral is low-fill across a large bbox with few stars per row.
    let placement = spiral_1500_placement();
    let n = placement.systems.len();
    assert_eq!(n, NUM_STARS_1500 as usize);

    let distinct: BTreeSet<(u32, u32)> = placement
        .systems
        .iter()
        .map(|s| (s.coord.col, s.coord.row))
        .collect();
    assert_eq!(
        distinct.len(),
        n,
        "one system per cell — no coincident coords (not a brick)"
    );

    let (mut min_c, mut max_c, mut min_r, mut max_r) = (u32::MAX, 0u32, u32::MAX, 0u32);
    let mut per_row: std::collections::BTreeMap<u32, u32> = std::collections::BTreeMap::new();
    for s in &placement.systems {
        min_c = min_c.min(s.coord.col);
        max_c = max_c.max(s.coord.col);
        min_r = min_r.min(s.coord.row);
        max_r = max_r.max(s.coord.row);
        *per_row.entry(s.coord.row).or_insert(0) += 1;
    }
    let bbox_w = (max_c - min_c + 1) as u64;
    let bbox_h = (max_r - min_r + 1) as u64;
    let bbox_area = bbox_w * bbox_h;
    // Dispersed: stars occupy a small fraction of their bounding box (a brick would be ~1.0).
    assert!(
        (n as f64) < 0.25 * bbox_area as f64,
        "fill ratio {:.3} too high — looks like a packed brick, not a dispersed spiral",
        n as f64 / bbox_area as f64
    );
    // No single row is densely packed (a brick row would hold ~bbox_w stars).
    let max_in_row = per_row.values().copied().max().unwrap_or(0) as u64;
    assert!(
        max_in_row < bbox_w / 4,
        "row packing {max_in_row} is brick-like vs bbox width {bbox_w}"
    );
    // The layout genuinely spreads across the lattice, not a tiny corner block.
    assert!(
        bbox_w >= 100 && bbox_h >= 100,
        "spiral must span the lattice, got {bbox_w}x{bbox_h}"
    );
}

#[test]
fn spiral_1500_base_hyperlanes_are_local_in_authored_grid() {
    let (placement, topology) = spiral_1500_topology();
    let by_id: std::collections::BTreeMap<String, &PlacedSystemSeed> = placement
        .systems
        .iter()
        .map(|s| (s.id.to_string(), s))
        .collect();
    let bound = spiral_1500_params().hyperlane.max_hyperlane_distance as u32;
    assert!(
        !topology.edges.is_empty(),
        "spiral must produce base hyperlanes"
    );
    for edge in &topology.edges {
        let a = by_id[&edge.from];
        let b = by_id[&edge.to];
        assert!(
            authored_chebyshev(a, b) <= bound,
            "edge {edge:?} spans authored Chebyshev {} > bound {bound} — not local in the spiral layout",
            authored_chebyshev(a, b)
        );
    }
}
