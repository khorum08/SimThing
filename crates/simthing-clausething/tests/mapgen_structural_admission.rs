//! STEAD-SCALE-1 — structural grid admission (budget-based, no fixed edge cap).
//!
//! Structural gridcell layout scales by **explicit admission budgets + memory**, never a magic edge
//! constant. These prove: large/vast sparse layouts pass structurally; budget overruns fail explicitly
//! naming the budget; capacity math is checked `u128` (never wraps); there is no fixed edge cap; and the
//! bounded-theater Movement-Front deferral does **not** invalidate the structural layout.

use simthing_clausething::{
    MapGenLatticeOptions, MapgenStructuralGridBudget, admit_structural_grid,
    generate_mapgen_lattice_hierarchy, parse_mapgen_neutral_document,
};

const UNBOUNDED: MapgenStructuralGridBudget = MapgenStructuralGridBudget {
    max_cells: None,
    max_occupied_cells: None,
    max_links: None,
    max_metadata_bytes: None,
};

#[test]
fn structural_grid_accepts_300_by_300_1500_occupied() {
    let stats =
        admit_structural_grid(300, 300, 1500, 1800, &UNBOUNDED).expect("admit 300x300/1500");
    assert_eq!(stats.cell_count, 90_000);
    assert_eq!(stats.occupied_cells, 1500);
}

#[test]
fn real_generated_1500_star_spiral_lowers_structurally() {
    // The actual MapGeneratorCLI producer: 1500-star `spiral_4` on a 300-edge lattice — generated, emitted,
    // and lowered end-to-end (NOT a five-system coordinate-span fixture). Structural layout must admit.
    let result = simthing_mapgenerator::generate_visual_spiral_1500(
        &simthing_mapgenerator::ShapeRegistry::default(),
    )
    .expect("generate 1500-star spiral");
    let neutral = parse_mapgen_neutral_document(result.scenario.as_str().as_bytes())
        .expect("parse generated spiral");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("1500-star spiral admits structurally under the default unbounded budget");

    // One gridcell per star (sparse, one-system-per-cell), honored spiral positions on a large lattice.
    assert_eq!(hierarchy.pack.grid_metadata.placements.len(), 1500);
    assert!(
        hierarchy.pack.grid_metadata.grid_size > 200,
        "a 300-edge spiral lattice is larger than the SMALL 200 reference (got {})",
        hierarchy.pack.grid_metadata.grid_size
    );
}

#[test]
fn structural_grid_accepts_1000_by_1000_sparse() {
    let stats =
        admit_structural_grid(1000, 1000, 5000, 6000, &UNBOUNDED).expect("admit 1000x1000 sparse");
    assert_eq!(stats.cell_count, 1_000_000);
}

#[test]
fn structural_grid_rejects_budget_exceeded_by_cell_count() {
    let budget = MapgenStructuralGridBudget {
        max_cells: Some(10_000),
        ..Default::default()
    };
    let err = admit_structural_grid(300, 300, 1500, 0, &budget).expect_err("cell budget exceeded");
    assert!(
        err.message.contains("max_cells"),
        "must name the budget: {}",
        err.message
    );
    assert!(
        err.message.contains("cell_count"),
        "must name the actual: {}",
        err.message
    );
}

#[test]
fn structural_grid_rejects_budget_exceeded_by_metadata_estimate() {
    let budget = MapgenStructuralGridBudget {
        max_metadata_bytes: Some(1_000),
        ..Default::default()
    };
    // 1500 occupied × 256 bytes/cell ≫ 1000.
    let err =
        admit_structural_grid(300, 300, 1500, 0, &budget).expect_err("metadata budget exceeded");
    assert!(
        err.message.contains("max_metadata_bytes"),
        "must name the budget: {}",
        err.message
    );
}

#[test]
fn structural_grid_capacity_math_does_not_wrap() {
    // edge² for 100_000 is 10^10, which OVERFLOWS u32 — a native `u32` edge*edge would wrap to a small
    // value. The checked-u128 admission must produce the exact product instead.
    let edge = 100_000u32;
    assert!(
        edge.checked_mul(edge).is_none(),
        "u32 edge*edge would overflow here"
    );
    let stats = admit_structural_grid(edge, edge, 0, 0, &UNBOUNDED).expect("admit huge edge");
    assert_eq!(stats.cell_count, 10_000_000_000u128); // exact 10^10, not a u32 wrap
    // The extreme u32 edge also yields its exact u128 product.
    let huge =
        admit_structural_grid(u32::MAX, u32::MAX, 0, 0, &UNBOUNDED).expect("u32::MAX edge admits");
    assert_eq!(huge.cell_count, (u32::MAX as u128) * (u32::MAX as u128));
}

#[test]
fn structural_grid_has_no_fixed_edge_cap() {
    // Edges far beyond the old 65_535 ceiling admit structurally under the default (unbounded) budget.
    assert!(admit_structural_grid(70_000, 70_000, 10, 0, &UNBOUNDED).is_ok());
    assert!(admit_structural_grid(2_000_000, 2_000_000, 10, 0, &UNBOUNDED).is_ok());
}

#[test]
fn rf_capacity_limit_is_not_lattice_edge_limit() {
    // The structural layout admits a vast edge; any RF/execution capacity limit is a SEPARATE,
    // execution-profile concern (≤10/32-per-edge bounded theater) — never a structural edge cap.
    let big = admit_structural_grid(5000, 5000, 25_000, 30_000, &UNBOUNDED)
        .expect("vast structural layout admits regardless of any RF execution cap");
    assert_eq!(big.width, 5000);
    // Configuring an occupied-cell budget is how you bound structural memory — not an edge constant.
    let budget = MapgenStructuralGridBudget {
        max_occupied_cells: Some(1000),
        ..Default::default()
    };
    let err = admit_structural_grid(5000, 5000, 25_000, 0, &budget)
        .expect_err("occupied budget exceeded");
    assert!(
        err.message.contains("max_occupied_cells"),
        "{}",
        err.message
    );
}

#[test]
fn full_hierarchy_lowers_a_vast_sparse_layout_under_default_unbounded_budget() {
    // End-to-end: a >65_535-edge sparse layout lowers structurally with the default (unbounded) budget.
    let doc = r#"
huge = {
    static_galaxy_scenario = {
        name = "Huge"
        system = { id = "0" name = "" position = { x = 0 y = 0 z = 0 } }
        system = { id = "1" name = "" position = { x = 100000 y = 100000 z = 0 } }
    }
}
"#;
    let neutral = parse_mapgen_neutral_document(doc.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("vast sparse layout lowers under default unbounded budget");
    assert_eq!(hierarchy.pack.grid_metadata.grid_size, 100_001);
}
