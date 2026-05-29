//! Phase M Queue-Write Scale Hardening V1 — bulk child resource fill on first-slice bridge.

mod support;

use simthing_driver::{
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceSummaryStatus, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_first_slice_scenario_preview, compile_region_field_preview,
    deserialize_first_slice_scenario_ron, deserialize_region_field_ron, FirstSliceScenarioSpec,
    MappingExecutionProfile, RegionFieldSpec,
};
use std::sync::Mutex;

use support::first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SUMMARY_SCENARIO_RON: &str =
    include_str!("fixtures/first_slice_product_summary_validity_scenario.ron");
const PRODUCT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_suppression_field.ron");
const COMMITMENT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_field.ron");

const SEED: FirstSliceSeed = FirstSliceSeed {
    row: 4,
    col: 4,
    value: 120.0,
};
const LOW_WEIGHTS: (f32, f32) = (0.2, 0.1);
const HIGH_WEIGHTS: (f32, f32) = (0.9, 0.1);

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn product_spec() -> RegionFieldSpec {
    deserialize_region_field_ron(PRODUCT_FIXTURE_RON).expect("product fixture RON parses")
}

fn commitment_spec() -> RegionFieldSpec {
    deserialize_region_field_ron(COMMITMENT_FIXTURE_RON).expect("commitment fixture RON parses")
}

fn summary_scenario() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(SUMMARY_SCENARIO_RON).expect("scenario RON parses")
}

#[test]
fn first_slice_bridge_uses_bulk_child_resource_fill() {
    with_gpu(|ctx| {
        let spec = product_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(report.reduction_stencil_readbacks, 0);

        let r = &report.readiness;
        assert_eq!(r.gpu_bridge_bulk_col_fills, 1);
        assert_eq!(r.gpu_bridge_bulk_fill_values, r.cell_count);
        assert_eq!(r.gpu_bridge_parent_scalar_writes, 2);
        assert_eq!(r.gpu_bridge_slot_col_writes, 2);
        assert!(r.gpu_bridge_slot_col_writes < r.cell_count);
    });
}

#[test]
fn bulk_fill_path_matches_prior_diagnostic_outputs() {
    with_gpu(|ctx| {
        let spec = commitment_spec();
        let preview = compile_region_field_preview(&spec).expect("admit");
        let threshold = preview.commitment.as_ref().expect("commitment");

        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[SEED]).unwrap();

        let low = session
            .tick_with_commitment_spec_fixture(
                ctx,
                FirstSliceTickOptions::hot_path(),
                LOW_WEIGHTS,
                threshold,
            )
            .unwrap();
        assert!(low.threshold_events.is_empty());

        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session.queue_seeds(&[SEED]).unwrap();
        let high = session
            .tick_with_commitment_spec_fixture(
                ctx,
                FirstSliceTickOptions::hot_path(),
                HIGH_WEIGHTS,
                threshold,
            )
            .unwrap();
        assert_eq!(high.threshold_events.len(), 1);
        assert_eq!(high.threshold_events[0].event_kind, threshold.event_kind);

        let (threat, urgency) = session
            .diagnostic_readback_reduction_eml(ctx, HIGH_WEIGHTS)
            .unwrap();
        assert!(threat.is_finite() && threat > 0.0);
        assert!(urgency.is_finite() && urgency > threshold.threshold);
    });
}

#[test]
fn summary_validity_unaffected_by_bulk_fill() {
    with_gpu(|ctx| {
        let preview = compile_first_slice_scenario_preview(&summary_scenario()).expect("admit");
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap();
        session.queue_seeds(&[SEED]).unwrap();

        let fresh = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(fresh.summary.status, FirstSliceSummaryStatus::FreshThisTick);
        assert_eq!(fresh.readiness.gpu_bridge_bulk_col_fills, 1);

        let cached1 = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached1.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );
        assert_eq!(cached1.total_dispatches, 0);
        assert_eq!(cached1.readiness.gpu_bridge_bulk_col_fills, 0);
        assert!(!cached1.summary.summary_used_for_commitment_scan);

        let cached2 = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached2.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 2 }
        );
        assert_eq!(cached2.reduction_stencil_readbacks, 0);

        session
            .queue_seeds(&[FirstSliceSeed {
                row: 3,
                col: 3,
                value: 90.0,
            }])
            .unwrap();
        let refresh = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(refresh.summary.status, FirstSliceSummaryStatus::FreshThisTick);
        assert_eq!(refresh.readiness.gpu_bridge_bulk_col_fills, 1);
    });
}

#[test]
fn queue_write_hardening_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = product_spec();
    assert!(!spec.request_atlas_batching);
    let mut atlas = spec.clone();
    atlas.request_atlas_batching = true;
    assert!(compile_region_field_preview(&atlas).is_err());

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let fill_wgsl = include_str!("../../simthing-gpu/src/shaders/values_fill.wgsl");
    let forbidden = [
        concat!("ActiveOnly", "ExperimentalNoHalo"),
        concat!("source_", "mask"),
        concat!("LocalBounds", "Metadata"),
        concat!("AlgebraicTile", "LocalMask"),
        concat!("Physical", "Gutter"),
        concat!("semantic ", "WGSL"),
        concat!("atlas ", "packer"),
    ];
    for source in [runtime_src, fill_wgsl] {
        for needle in forbidden {
            assert!(!source.contains(needle));
        }
    }
    assert!(!runtime_src.contains("for slot in 0..cell_count"));
}
