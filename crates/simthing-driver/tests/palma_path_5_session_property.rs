//! PALMA-PATH-5 — feed GPU min-plus field from admitted Location/gridcell property columns.

mod support;

use simthing_gpu::{extract_d_flat, GpuContext, MinPlusStencilOp};
use std::sync::Mutex;

use support::palma_path_5_property_fixture::{
    max_d_field_error_public, PalmaPath5BlockerLedger, PalmaPath5PropertyTree,
    GRID_TRAVERSAL_D_ROLE, GRID_TRAVERSAL_W_ROLE,
};
use support::palma_terran_pirate_fixture::{
    build_location_w_field, reparent_toward_sampled_gridcell, GridCoord, FIXTURE_ITERATIONS,
    FIXTURE_WIDTH,
};
use support::palma_terran_pirate_tree::find_node;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PALMA PATH-5");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn w_seeded_through_admitted_property_columns() {
    let field = build_location_w_field(true, None, false);
    let tree = PalmaPath5PropertyTree::build_with_field(&field);

    let from_props = tree.gather_w_flat_from_properties();
    assert_eq!(
        from_props, field.w,
        "W must round-trip through gridcell properties"
    );

    let from_shadow = tree.gather_w_flat_from_shadow();
    assert_eq!(
        from_shadow, field.w,
        "project_tree_to_values must expose W at registry column offsets"
    );

    assert!(tree
        .read_w_from_property(GridCoord { x: 0, y: 0 })
        .is_finite());
    assert_eq!(
        tree.read_d_from_property(GridCoord { x: 0, y: 0 }),
        0.0,
        "destination D seed on property column"
    );
}

#[test]
fn property_columns_use_named_w_and_d_roles() {
    let tree = PalmaPath5PropertyTree::build_default();
    let layout = tree
        .inner
        .reg
        .property(tree.grid_traversal_id)
        .layout
        .clone();
    assert!(layout
        .offset_of(&simthing_core::SubFieldRole::Named(
            GRID_TRAVERSAL_W_ROLE.into()
        ))
        .is_some());
    assert!(layout
        .offset_of(&simthing_core::SubFieldRole::Named(
            GRID_TRAVERSAL_D_ROLE.into()
        ))
        .is_some());
}

#[test]
fn gpu_min_plus_from_property_gather_matches_cpu_oracle() {
    let tree = PalmaPath5PropertyTree::build_default();
    let config = tree.min_plus_config();
    let cpu_d = tree
        .cpu_oracle_d_from_property_w()
        .expect("cpu oracle from property W");

    with_gpu(|ctx| {
        let values = tree
            .pack_stencil_values_from_properties()
            .expect("gather W from properties into stencil buffer");
        let op = MinPlusStencilOp::new(ctx, config.clone()).expect("gpu op");
        op.upload_values(ctx, &values).expect("upload");
        let gpu_values = op.run_ping_pong(ctx, FIXTURE_ITERATIONS).expect("gpu run");
        let gpu_d = extract_d_flat(&gpu_values, &config).expect("extract d");
        assert!(
            max_d_field_error_public(&cpu_d, &gpu_d) < 1e-4,
            "GPU MinPlusStencilOp must match CPU oracle on property-sourced W"
        );
    });
}

#[test]
fn d_writeback_to_property_columns_matches_oracle() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    let cpu_d = tree.cpu_oracle_d_from_property_w().expect("cpu oracle d");

    tree.write_d_flat_to_properties(&cpu_d)
        .expect("write D to gridcell properties");

    let from_props = tree.gather_d_flat_from_properties();
    assert_eq!(from_props.len(), cpu_d.len());
    for (i, (&expected, &actual)) in cpu_d.iter().zip(from_props.iter()).enumerate() {
        if expected.is_infinite() && actual.is_infinite() {
            continue;
        }
        assert!(
            (expected - actual).abs() < 1e-4,
            "D writeback mismatch at cell {i}: cpu={expected} prop={actual}"
        );
    }

    let from_shadow = {
        let width = FIXTURE_WIDTH as usize;
        let height = support::palma_terran_pirate_fixture::FIXTURE_HEIGHT as usize;
        let n_dims = tree.inner.n_dims;
        let mut out = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let id = support::palma_terran_pirate_fixture::gridcell_simthing_id(x, y);
                let slot = tree.inner.alloc.slot_of(id).expect("slot") as usize;
                out.push(tree.inner.shadow[slot * n_dims + tree.d_global_col]);
            }
        }
        out
    };
    assert!(
        max_d_field_error_public(&cpu_d, &from_shadow) < 1e-4,
        "shadow buffer must reflect D writeback at registry column offsets"
    );
}

#[test]
fn movable_samples_d_from_property_columns() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    let cpu_d = tree.cpu_oracle_d_from_property_w().expect("cpu oracle d");
    tree.write_d_flat_to_properties(&cpu_d)
        .expect("seed D on properties");

    let step = tree
        .sample_lowest_d_neighbor_from_properties(tree.convoy_coord())
        .expect("sample from property columns");

    assert!(
        step.to.x < step.from.x || step.to.y < step.from.y,
        "convoy should step toward destination by property-column D: {:?}",
        step
    );
    assert_eq!(
        step.sampled_d,
        tree.read_d_from_property(step.to),
        "sampled D must match property column read"
    );
}

#[test]
fn property_sample_maps_to_generic_reparent_on_admitted_tree() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    let cpu_d = tree.cpu_oracle_d_from_property_w().expect("cpu oracle d");
    tree.write_d_flat_to_properties(&cpu_d)
        .expect("seed D on properties");

    let step = tree
        .sample_lowest_d_neighbor_from_properties(tree.convoy_coord())
        .expect("sample");
    let target = tree.gridcell_id_at(step.to);

    assert!(tree.inner.is_gridcell_child_of_location(target));
    let request = reparent_toward_sampled_gridcell(tree.convoy_id(), target);
    match request {
        simthing_feeder::BoundaryRequest::Reparent { child, new_parent } => {
            assert_eq!(child, tree.convoy_id());
            assert_eq!(new_parent, target);
        }
        other => panic!("expected generic Reparent, got {other:?}"),
    }

    let out = tree
        .inner
        .apply_reparent(reparent_toward_sampled_gridcell(tree.convoy_id(), target));
    assert_eq!(out.reparents, 1);
    assert_eq!(tree.inner.parent_id(tree.convoy_id()), Some(target));

    let location = find_node(&tree.inner.root, tree.location_id()).expect("location");
    assert_eq!(
        location.children.len(),
        (FIXTURE_WIDTH * FIXTURE_WIDTH) as usize
    );
}

#[test]
fn path5_blocker_ledger_session_scheduling_not_wired() {
    let ledger = PalmaPath5BlockerLedger::current();
    assert!(
        ledger.session_region_field_min_plus_scheduling,
        "RegionField session band for min-plus is not default-scheduled — test-local adapter used"
    );
    assert!(
        ledger.simsession_default_tick_wiring,
        "SimSession default tick does not invoke MinPlusStencilOp — fixture-only proof"
    );
    assert!(
        !ledger.install_round_trip_required,
        "PATH-5 proof does not require full install round-trip; property columns on admitted tree suffice"
    );
}
