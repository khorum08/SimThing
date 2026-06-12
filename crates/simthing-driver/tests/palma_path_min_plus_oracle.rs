//! PALMA-PATH-1 — CPU oracle for min-plus Location gridcell traversal-cost fields.
//!
//! Proves numeric `D` relaxation over numeric `W` on a small Location-owned grid:
//! - high-impedance pirate/blockade corridor bends effective cost;
//! - clearing blockade `W` lowers `D` on the next iterations;
//! - output is a scalar field only (no route object, no pathfinding engine).

mod support;

use support::palma_min_plus_oracle::{
    cell_index, run_min_plus_relaxation, terran_pirate_grid_w_field, INF,
};

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const ITERATIONS: usize = 32;

/// Starport / destination at the northwest corner; Terran convoy would sample `D` at southeast later.
const DEST: (usize, usize) = (0, 0);
const CONVOY_QUERY: (usize, usize) = (4, 4);

/// Pirate blockade band — full middle row (numeric `W` only). Every Manhattan route from
/// northwest dest to southeast convoy query must detour around y = 2.
const BLOCKADE_CELLS: &[(usize, usize)] = &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)];

fn dest_idx() -> usize {
    cell_index(DEST.0, DEST.1, WIDTH)
}

fn convoy_idx() -> usize {
    cell_index(CONVOY_QUERY.0, CONVOY_QUERY.1, WIDTH)
}

#[test]
fn palma_min_plus_uniform_grid_matches_manhattan_cost() {
    let w = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, &[], 1.0);
    let d = run_min_plus_relaxation(&w, WIDTH, HEIGHT, dest_idx(), ITERATIONS);
    assert!(
        (d[convoy_idx()] - 8.0).abs() < 1e-4,
        "expected Manhattan cost 8 from (0,0) to (4,4), got {}",
        d[convoy_idx()]
    );
}

#[test]
fn palma_min_plus_pirate_blockade_corridor_raises_convoy_d() {
    let w_clear = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, &[], 1.0);
    let w_blocked = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, BLOCKADE_CELLS, 100.0);

    let d_clear = run_min_plus_relaxation(&w_clear, WIDTH, HEIGHT, dest_idx(), ITERATIONS);
    let d_blocked = run_min_plus_relaxation(&w_blocked, WIDTH, HEIGHT, dest_idx(), ITERATIONS);

    assert!(
        d_blocked[convoy_idx()] > d_clear[convoy_idx()] + 1.0,
        "blockade should raise traversal cost: clear={} blocked={}",
        d_clear[convoy_idx()],
        d_blocked[convoy_idx()]
    );

    // The corridor cells themselves carry high accumulated cost when blockaded.
    for &(x, y) in BLOCKADE_CELLS {
        let i = cell_index(x, y, WIDTH);
        assert!(
            d_blocked[i] > d_clear[i] + 50.0,
            "blockade cell ({x},{y}) should carry high D: clear={} blocked={}",
            d_clear[i],
            d_blocked[i]
        );
    }
}

#[test]
fn palma_min_plus_clearing_blockade_lowers_d_field() {
    let w_blocked = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, BLOCKADE_CELLS, 100.0);
    let w_cleared = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, &[], 1.0);

    let d_before = run_min_plus_relaxation(&w_blocked, WIDTH, HEIGHT, dest_idx(), ITERATIONS);
    let d_after = run_min_plus_relaxation(&w_cleared, WIDTH, HEIGHT, dest_idx(), ITERATIONS);

    assert!(
        d_after[convoy_idx()] + 1e-4 < d_before[convoy_idx()],
        "clearing W should lower D: before={} after={}",
        d_before[convoy_idx()],
        d_after[convoy_idx()]
    );
    assert!(
        (d_after[convoy_idx()] - 8.0).abs() < 1e-4,
        "after clear, convoy cell should return to Manhattan baseline, got {}",
        d_after[convoy_idx()]
    );
}

#[test]
fn palma_min_plus_emits_scalar_field_only_not_route_object() {
    let w = terran_pirate_grid_w_field(WIDTH, HEIGHT, 1.0, BLOCKADE_CELLS, 50.0);
    let d = run_min_plus_relaxation(&w, WIDTH, HEIGHT, dest_idx(), 8);

    assert_eq!(d.len(), WIDTH * HEIGHT);
    assert!(d.iter().all(|v| v.is_finite() || *v == INF));
    // No path polyline, no predecessor table, no semantic route id — field values only.
    assert!(d[dest_idx()].abs() < 1e-6);
}
