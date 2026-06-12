//! PALMA-PATH-3 — Terran convoy / pirate fleet field-sampling fixture.
//!
//! Proves a movable SimThing can consume Location-owned min-plus `D` without a pathfinding
//! engine, route object, or semantic GPU code. Uses existing `BoundaryRequest::Reparent` shape
//! only as a generic structural mapping — no full movement policy or simthing-sim changes.

mod support;

use simthing_gpu::{
    cpu_min_plus_d_from_w, extract_d_flat, max_d_field_error, pack_w_and_initial_d, GpuContext,
    MinPlusStencilOp,
};
use std::sync::Mutex;

use support::palma_min_plus_oracle::cell_index;
use support::palma_terran_pirate_fixture::{
    apply_pirate_pressure, build_location_w_field, clear_blockade_gap, convoy_simthing_id,
    gridcell_simthing_id, reparent_toward_sampled_gridcell, sample_lowest_d_neighbor, GridCoord,
    LocationImpedanceField, CONVOY_START, FIXTURE_HEIGHT, FIXTURE_ITERATIONS, FIXTURE_WIDTH,
    GAP_CELL, PIRATE_ANCHOR,
};
use support::palma_terran_pirate_tree::PalmaAdmittedTree;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PALMA PATH-3 GPU parity");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn convoy_coord() -> GridCoord {
    GridCoord {
        x: CONVOY_START.0,
        y: CONVOY_START.1,
    }
}

fn gpu_d_matches_cpu(field: &LocationImpedanceField) {
    let config = field.config();
    with_gpu(|ctx| {
        let cpu_d = cpu_min_plus_d_from_w(&field.w, &config, FIXTURE_ITERATIONS).expect("cpu d");
        let values = pack_w_and_initial_d(&field.w, &config).expect("pack");
        let op = MinPlusStencilOp::new(ctx, config.clone()).expect("gpu op");
        op.upload_values(ctx, &values).expect("upload");
        let gpu_values = op.run_ping_pong(ctx, FIXTURE_ITERATIONS).expect("gpu run");
        let gpu_d = extract_d_flat(&gpu_values, &config).expect("extract d");
        assert!(
            max_d_field_error(&cpu_d, &gpu_d) < 1e-4,
            "Terran/pirate fixture GPU/CPU D parity"
        );
    });
}

#[test]
fn terran_convoy_samples_lower_d_neighbor_without_route_object() {
    let field = build_location_w_field(true, None, false);
    let d = field.compute_d().expect("d field");
    let step = sample_lowest_d_neighbor(&d, field.width, field.height, convoy_coord())
        .expect("sample step");

    assert!(
        step.to.x < step.from.x || step.to.y < step.from.y,
        "convoy should step toward destination station (lower x or y): {:?}",
        step
    );
    assert!(step.sampled_d.is_finite());
    assert!(step.sampled_d < d[step.from.idx(field.width)]);

    gpu_d_matches_cpu(&field);
}

#[test]
fn pirate_blockade_and_fuel_shortage_change_sampled_neighbor_preference() {
    let open = build_location_w_field(true, None, false);
    let pressured = build_location_w_field(true, Some(PIRATE_ANCHOR), true);

    let d_open = open.compute_d().expect("open d");
    let d_pressured = pressured.compute_d().expect("pressured d");

    let step_open =
        sample_lowest_d_neighbor(&d_open, open.width, open.height, convoy_coord()).expect("open");
    let step_pressured = sample_lowest_d_neighbor(
        &d_pressured,
        pressured.width,
        pressured.height,
        convoy_coord(),
    )
    .expect("pressured");

    assert!(
        step_pressured.sampled_d >= step_open.sampled_d,
        "pirate/blockade/fuel numeric W should not cheapen sampled step"
    );

    gpu_d_matches_cpu(&pressured);
}

#[test]
fn clearing_blockade_and_moving_pirate_updates_field_and_sample() {
    let mut field = build_location_w_field(false, Some(PIRATE_ANCHOR), false);
    let d_blocked = field.compute_d().expect("blocked d");
    let step_blocked =
        sample_lowest_d_neighbor(&d_blocked, field.width, field.height, convoy_coord())
            .expect("blocked sample");

    clear_blockade_gap(&mut field.w, field.width);
    apply_pirate_pressure(&mut field.w, field.width, (2, 2), 40.0);

    let d_cleared = field.compute_d().expect("cleared d");
    let step_cleared =
        sample_lowest_d_neighbor(&d_cleared, field.width, field.height, convoy_coord())
            .expect("cleared sample");

    assert!(
        step_cleared.sampled_d + 1e-3 < step_blocked.sampled_d,
        "clearing gap should lower sampled D: blocked={} cleared={}",
        step_blocked.sampled_d,
        step_cleared.sampled_d
    );
    let convoy_idx = cell_index(CONVOY_START.0, CONVOY_START.1, field.width);
    assert!(
        d_cleared[convoy_idx] + 1e-3 < d_blocked[convoy_idx],
        "convoy cell D should drop after W update"
    );

    gpu_d_matches_cpu(&field);
}

#[test]
fn sampled_step_maps_to_generic_reparent_boundary_request() {
    let field = build_location_w_field(true, None, false);
    let d = field.compute_d().expect("d field");
    let step =
        sample_lowest_d_neighbor(&d, field.width, field.height, convoy_coord()).expect("sample");

    let request = reparent_toward_sampled_gridcell(
        convoy_simthing_id(),
        gridcell_simthing_id(step.to.x, step.to.y),
    );

    match request {
        simthing_feeder::BoundaryRequest::Reparent { child, new_parent } => {
            assert_eq!(child, convoy_simthing_id());
            assert_eq!(new_parent, gridcell_simthing_id(step.to.x, step.to.y));
        }
        other => panic!("expected generic Reparent, got {other:?}"),
    }

    // No predecessor table, polyline, or route id — only the sampled gridcell parent target.
    assert!(step.sampled_d.is_finite());
    assert_ne!(step.to, step.from);
}

#[test]
fn gap_corridor_yields_lower_d_at_convoy_than_closed_gap() {
    let open = build_location_w_field(true, None, false);
    let closed = build_location_w_field(false, None, false);

    let d_open = open.compute_d().expect("open");
    let d_closed = closed.compute_d().expect("closed");
    let query = cell_index(CONVOY_START.0, CONVOY_START.1, FIXTURE_WIDTH as usize);

    assert!(
        d_open[query] + 1e-3 < d_closed[query],
        "open gap lowers convoy-cell D"
    );

    let gap = cell_index(GAP_CELL.0, GAP_CELL.1, FIXTURE_WIDTH as usize);
    let wall = cell_index(4, 3, FIXTURE_WIDTH as usize);
    assert!(d_open[gap] + 10.0 < d_open[wall]);
}

#[test]
fn admitted_location_gridcell_tree_maps_sample_to_reparent() {
    let tree = PalmaAdmittedTree::build();

    assert_eq!(
        tree.gridcell_ids().len(),
        (FIXTURE_WIDTH * FIXTURE_HEIGHT) as usize
    );
    for id in tree.gridcell_ids() {
        assert!(tree.is_gridcell_child_of_location(id));
    }
    assert_eq!(
        tree.parent_id(tree.convoy_id),
        Some(tree.convoy_parent_gridcell_id)
    );
    assert!(tree.is_gridcell_child_of_location(tree.convoy_parent_gridcell_id));

    let field = build_location_w_field(true, None, false);
    let d = field.compute_d().expect("d field");
    let step =
        sample_lowest_d_neighbor(&d, field.width, field.height, convoy_coord()).expect("sample");
    let target = gridcell_simthing_id(step.to.x, step.to.y);

    assert!(tree.is_gridcell_child_of_location(target));
    assert_ne!(target, tree.convoy_id);

    let request = reparent_toward_sampled_gridcell(tree.convoy_id, target);
    match request {
        simthing_feeder::BoundaryRequest::Reparent { child, new_parent } => {
            assert_eq!(child, tree.convoy_id);
            assert_eq!(new_parent, target);
        }
        other => panic!("expected generic Reparent, got {other:?}"),
    }

    assert!(step.sampled_d.is_finite());
    gpu_d_matches_cpu(&field);
}

#[test]
fn reparent_request_updates_live_parent_if_supported() {
    let mut tree = PalmaAdmittedTree::build();
    let field = build_location_w_field(true, None, false);
    let d = field.compute_d().expect("d field");
    let step =
        sample_lowest_d_neighbor(&d, field.width, field.height, convoy_coord()).expect("sample");
    let target = gridcell_simthing_id(step.to.x, step.to.y);
    let original_slot = tree.alloc.slot_of(tree.convoy_id).expect("convoy slot");

    assert_eq!(
        tree.parent_id(tree.convoy_id),
        Some(tree.convoy_parent_gridcell_id)
    );

    let out = tree.apply_reparent(reparent_toward_sampled_gridcell(tree.convoy_id, target));

    assert_eq!(out.reparents, 1);
    assert_eq!(out.rejected_unknown_target, 0);
    assert_eq!(tree.parent_id(tree.convoy_id), Some(target));
    assert_eq!(
        tree.alloc.slot_of(tree.convoy_id),
        Some(original_slot),
        "reparent preserves slot — no movement engine"
    );
}

#[test]
fn fixture_ledgers_missing_reparent_application_if_not_supported() {
    // PALMA-PATH-3R: `simthing_sim::apply_structural_mutations` accepts generic Reparent on
    // admitted trees — no PATH-3R blocker for structural application in driver tests.
    let mut tree = PalmaAdmittedTree::build();
    let gridcells = tree.gridcell_ids();
    assert_eq!(gridcells.len(), 64);
    assert!(gridcells.contains(&tree.convoy_parent_gridcell_id));

    let neighbor = gridcell_simthing_id(CONVOY_START.0 - 1, CONVOY_START.1);
    assert!(tree.is_gridcell_child_of_location(neighbor));

    let out = tree.apply_reparent(reparent_toward_sampled_gridcell(tree.convoy_id, neighbor));
    assert_eq!(out.reparents, 1, "harness supports Reparent — not blocked");
    assert_eq!(tree.parent_id(tree.convoy_id), Some(neighbor));
}
