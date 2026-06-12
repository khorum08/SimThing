//! PALMA-PATH-6 — opt-in session/RegionField min-plus band over admitted W/D columns.

mod support;

use simthing_driver::{
    FieldCadence, PalmaMinPlusFieldBandSession, PalmaMinPlusGridBinding,
    TraversalFieldExecutionMode, PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED,
    PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID,
};
use simthing_gpu::GpuContext;
use std::sync::Mutex;

use support::palma_path_5_property_fixture::{
    max_d_field_error_public, PalmaPath5PropertyTree, GRID_TRAVERSAL_D_ROLE, GRID_TRAVERSAL_W_ROLE,
};
use support::palma_terran_pirate_fixture::{
    reparent_toward_sampled_gridcell, DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS,
    FIXTURE_WIDTH,
};
use support::palma_terran_pirate_tree::find_node;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for PALMA PATH-6");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn grid_binding(tree: &PalmaPath5PropertyTree) -> PalmaMinPlusGridBinding {
    PalmaMinPlusGridBinding {
        width: FIXTURE_WIDTH,
        height: FIXTURE_HEIGHT,
        dest_x: DESTINATION.0,
        dest_y: DESTINATION.1,
        iterations: FIXTURE_ITERATIONS,
        w_global_col: tree.w_global_col,
        d_global_col: tree.d_global_col,
        gridcell_ids: tree.gridcell_ids_row_major(),
    }
}

fn snapshot_d_shadow(tree: &PalmaPath5PropertyTree) -> Vec<f32> {
    let width = FIXTURE_WIDTH as usize;
    let height = FIXTURE_HEIGHT as usize;
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
}

#[test]
fn min_plus_band_default_off() {
    assert!(
        !PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED,
        "PALMA min-plus band must be opt-in / default off"
    );
    let tree = PalmaPath5PropertyTree::build_default();
    let binding = grid_binding(&tree);
    let mut band =
        PalmaMinPlusFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    assert!(!band.enabled());
    with_gpu(|ctx| {
        let mut tree = tree;
        let report = band
            .tick(
                ctx,
                &mut tree.inner.shadow,
                tree.inner.n_dims,
                &tree.inner.alloc,
                TraversalFieldExecutionMode::GpuResident,
                false,
            )
            .expect("disabled tick");
        assert!(report.dispatch.is_none());
        assert_eq!(report.utility_id, PALMA_MIN_PLUS_FIELD_BAND_PROFILE_ID);
    });
}

#[test]
fn session_band_gathers_w_from_admitted_shadow_columns() {
    let tree = PalmaPath5PropertyTree::build_default();
    let binding = grid_binding(&tree);
    let w_props = tree.gather_w_flat_from_properties();
    let w_shadow = PalmaMinPlusFieldBandSession::gather_w_from_shadow(
        &tree.inner.shadow,
        tree.inner.n_dims,
        &tree.inner.alloc,
        &binding,
    )
    .expect("gather w");
    assert_eq!(w_shadow, w_props);
}

#[test]
fn session_band_dispatches_gpu_min_plus_not_manual_test_body() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band =
        PalmaMinPlusFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .tick(
                ctx,
                &mut tree.inner.shadow,
                tree.inner.n_dims,
                &tree.inner.alloc,
                TraversalFieldExecutionMode::OracleVerification,
                false,
            )
            .expect("band tick");
        assert!(report.scheduled);
        let dispatch = report.dispatch.expect("dispatch report");
        assert!(
            dispatch.gpu_dispatched,
            "GPU must be invoked by band tick, not test body"
        );
        assert_eq!(dispatch.iterations, FIXTURE_ITERATIONS);
        assert!(dispatch.diagnostic_readback);
        let err = dispatch.max_oracle_error.expect("oracle compare");
        assert!(
            err < 1e-4,
            "CPU oracle parity via band verification readback: max err {err}"
        );
    });
}

#[test]
fn session_band_writes_d_to_shadow_and_property_columns() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let cpu_d = tree
        .cpu_oracle_d_from_property_w()
        .expect("cpu oracle before band");

    let binding = grid_binding(&tree);
    let mut band =
        PalmaMinPlusFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        band.tick(
            ctx,
            &mut tree.inner.shadow,
            tree.inner.n_dims,
            &tree.inner.alloc,
            TraversalFieldExecutionMode::DiagnosticReadback,
            true,
        )
        .expect("band tick");
    });

    assert!(band.last_dispatch().unwrap().diagnostic_readback);
    tree.sync_d_from_shadow_to_properties()
        .expect("property writeback from shadow");
    let from_props = tree.gather_d_flat_from_properties();
    assert!(
        max_d_field_error_public(&cpu_d, &from_props) < 1e-4,
        "D property columns must match CPU oracle after band"
    );
}

#[test]
fn gpu_resident_band_does_not_readback_or_write_shadow_d() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let d_before = snapshot_d_shadow(&tree);
    let binding = grid_binding(&tree);
    let mut band =
        PalmaMinPlusFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .tick(
                ctx,
                &mut tree.inner.shadow,
                tree.inner.n_dims,
                &tree.inner.alloc,
                TraversalFieldExecutionMode::GpuResident,
                false,
            )
            .expect("gpu resident tick");
        assert!(report.scheduled);
        let dispatch = report.dispatch.expect("dispatch");
        assert!(dispatch.gpu_resident);
        assert!(!dispatch.diagnostic_readback);
        assert!(dispatch.values.is_none());
        assert!(!report.shadow_writeback);
    });

    let d_after = snapshot_d_shadow(&tree);
    assert_eq!(
        d_before, d_after,
        "GpuResident must not mutate CPU shadow D"
    );
}

#[test]
fn after_band_movable_samples_d_and_reparents_generically() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band =
        PalmaMinPlusFieldBandSession::new(binding, FieldCadence::EveryTick).expect("band");
    band.enable();

    with_gpu(|ctx| {
        band.tick(
            ctx,
            &mut tree.inner.shadow,
            tree.inner.n_dims,
            &tree.inner.alloc,
            TraversalFieldExecutionMode::DiagnosticReadback,
            true,
        )
        .expect("band tick");
    });
    tree.sync_d_from_shadow_to_properties()
        .expect("property writeback");

    let step = tree
        .sample_lowest_d_neighbor_from_properties(tree.convoy_coord())
        .expect("sample after band");
    assert!(
        step.to.x < step.from.x || step.to.y < step.from.y,
        "convoy steps toward destination by property D after band: {:?}",
        step
    );

    let target = tree.gridcell_id_at(step.to);
    let out = tree
        .inner
        .apply_reparent(reparent_toward_sampled_gridcell(tree.convoy_id(), target));
    assert_eq!(out.reparents, 1);
    assert_eq!(tree.inner.parent_id(tree.convoy_id()), Some(target));

    let location = find_node(&tree.inner.root, tree.location_id()).expect("location");
    assert_eq!(
        location.children.len(),
        (FIXTURE_WIDTH * FIXTURE_HEIGHT) as usize
    );
}

#[test]
fn path6_blocker_ledger_default_simsession_not_wired() {
    assert!(!PALMA_MIN_PLUS_FIELD_BAND_DEFAULT_ENABLED);
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
fn on_event_cadence_skips_until_dirty() {
    let mut tree = PalmaPath5PropertyTree::build_default();
    tree.sync_shadow_from_tree();
    let binding = grid_binding(&tree);
    let mut band = PalmaMinPlusFieldBandSession::new(binding, FieldCadence::OnEvent).expect("band");
    band.enable();

    with_gpu(|ctx| {
        let report = band
            .tick(
                ctx,
                &mut tree.inner.shadow,
                tree.inner.n_dims,
                &tree.inner.alloc,
                TraversalFieldExecutionMode::GpuResident,
                false,
            )
            .expect("on-event tick without pending event");
        assert!(
            report.dispatch.is_none(),
            "OnEvent cadence must skip without event_pending"
        );
    });
}
