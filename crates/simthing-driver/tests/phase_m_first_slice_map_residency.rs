//! Phase M Map Residency V1 — first-slice residency status/reporting.

mod support;

use simthing_driver::{
    FirstSliceMappingSession, FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryStatus,
    FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_first_slice_scenario_preview, compile_region_field_preview,
    deserialize_first_slice_scenario_ron, deserialize_region_field_ron, FirstSliceScenarioSpec,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldSpec,
};
use std::sync::Mutex;

use support::first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const RESIDENCY_SCENARIO_RON: &str =
    include_str!("fixtures/first_slice_product_summary_validity_scenario.ron");
const DISABLED_SCENARIO_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_scenario_disabled.ron");
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

fn residency_scenario() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(RESIDENCY_SCENARIO_RON).expect("scenario RON parses")
}

fn cold_skip_spec() -> RegionFieldSpec {
    let mut spec = residency_scenario().region_field;
    spec.cadence = RegionFieldCadenceSpec::OnEvent;
    spec
}

fn open_residency_session(ctx: &GpuContext) -> FirstSliceScenarioFixtureSession {
    let preview = compile_first_slice_scenario_preview(&residency_scenario()).expect("admit");
    FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap()
}

fn open_on_event_session(ctx: &GpuContext) -> FirstSliceScenarioFixtureSession {
    let mut scenario = residency_scenario();
    scenario.region_field.cadence = RegionFieldCadenceSpec::OnEvent;
    let preview = compile_first_slice_scenario_preview(&scenario).expect("admit");
    FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap()
}

#[test]
fn disabled_profile_unavailable() {
    with_gpu(|ctx| {
        let scenario =
            deserialize_first_slice_scenario_ron(DISABLED_SCENARIO_RON).expect("disabled RON");
        let preview = compile_first_slice_scenario_preview(&scenario).expect("admit");
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap();
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

        assert_eq!(
            report.mapping.residency.status,
            FirstSliceResidencyStatus::DisabledUnavailable
        );
        assert_eq!(
            report.mapping.summary.status,
            FirstSliceSummaryStatus::InvalidOrUnavailable
        );
        assert_eq!(report.mapping.total_dispatches, 0);
        assert!(!report.mapping.reduction_executed);
        assert!(!report.mapping.eml_executed);
        assert!(!report.mapping.residency.summary_visible_to_parent);
        assert!(report.threshold_events.is_empty());
    });
}

#[test]
fn cold_skipped_before_execution() {
    with_gpu(|ctx| {
        let spec = cold_skip_spec();
        let preview = compile_region_field_preview(&spec).expect("admit");
        let scenario_preview =
            compile_first_slice_scenario_preview(&residency_scenario()).expect("scenario");
        let mut preview_bundle = scenario_preview;
        preview_bundle.region_field = preview;
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview_bundle).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), LOW_WEIGHTS)
            .unwrap();

        assert_eq!(
            report.residency.status,
            FirstSliceResidencyStatus::ColdSkipped
        );
        assert_eq!(report.summary.status, FirstSliceSummaryStatus::ZeroInitial);
        assert!(!report.summary.has_gpu_parent_summary);
        assert!(!report.residency.summary_visible_to_parent);
        assert_eq!(report.total_dispatches, 0);
    });
}

#[test]
fn hot_executed_tick() {
    with_gpu(|ctx| {
        let mut session = open_residency_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

        assert_eq!(
            report.residency.status,
            FirstSliceResidencyStatus::HotExecutedThisTick
        );
        assert_eq!(report.summary.status, FirstSliceSummaryStatus::FreshThisTick);
        assert!(report.summary.has_gpu_parent_summary);
        assert!(report.residency.summary_visible_to_parent);
        assert!(report.residency.dense_field_executed);
        assert!(report.residency.parent_summary_retained_on_gpu);
        assert!(!report.residency.cached_commitment_scan_supported);
        assert_eq!(report.source_setup_dispatches, 1);
        assert_eq!(report.propagation_dispatches, 8);
        assert_eq!(report.readiness.gpu_bridge_bulk_col_fills, 1);
        assert_eq!(report.readiness.gpu_bridge_slot_col_writes, 2);
        assert_eq!(report.reduction_stencil_readbacks, 0);
    });
}

#[test]
fn resident_cached_tick() {
    with_gpu(|ctx| {
        let mut session = open_residency_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        session
            .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

        let cached = session
            .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached.mapping.residency.status,
            FirstSliceResidencyStatus::ResidentCached
        );
        assert_eq!(
            cached.mapping.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );
        assert!(cached.mapping.summary.has_gpu_parent_summary);
        assert!(cached.mapping.residency.summary_visible_to_parent);
        assert_eq!(cached.mapping.total_dispatches, 0);
        assert_eq!(cached.mapping.readiness.gpu_bridge_bulk_col_fills, 0);
        assert!(!cached.mapping.reduction_executed);
        assert!(!cached.mapping.eml_executed);
        assert!(!cached.mapping.summary.summary_used_for_commitment_scan);
        assert!(cached.threshold_events.is_empty());
    });
}

#[test]
fn dirty_refresh_from_cached() {
    with_gpu(|ctx| {
        let mut session = open_residency_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

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

        assert_eq!(
            refresh.residency.status,
            FirstSliceResidencyStatus::HotExecutedThisTick
        );
        assert_eq!(refresh.summary.status, FirstSliceSummaryStatus::FreshThisTick);
        assert_eq!(refresh.summary.age_ticks, 0);
        assert!(refresh.scheduled);
        assert_eq!(refresh.readiness.gpu_bridge_bulk_col_fills, 1);
        assert!(refresh.summary.has_gpu_parent_summary);
    });
}

#[test]
fn map_residency_sequence_is_deterministic() {
    with_gpu(|ctx| {
        let run = |ctx: &GpuContext| {
            let mut session = open_on_event_session(ctx);
            let cold = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            session.queue_seeds(&[SEED]).unwrap();
            let hot = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            let cached1 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            let cached2 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            session
                .queue_seeds(&[FirstSliceSeed {
                    row: 2,
                    col: 2,
                    value: 80.0,
                }])
                .unwrap();
            let refresh = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            (cold, hot, cached1, cached2, refresh)
        };

        let first = run(ctx);
        let second = run(ctx);
        assert_eq!(first.0.residency, second.0.residency);
        assert_eq!(first.1.residency, second.1.residency);
        assert_eq!(first.2.residency, second.2.residency);
        assert_eq!(first.3.residency, second.3.residency);
        assert_eq!(first.4.residency, second.4.residency);
        assert_eq!(first.0.summary, second.0.summary);
        assert_eq!(first.1.summary, second.1.summary);
        assert_eq!(first.2.summary, second.2.summary);
        assert_eq!(first.3.summary, second.3.summary);
        assert_eq!(first.4.summary, second.4.summary);
        assert_eq!(first.1.total_dispatches, second.1.total_dispatches);
    });
}

#[test]
fn map_residency_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = deserialize_region_field_ron(COMMITMENT_FIXTURE_RON).expect("product RON");
    assert!(!spec.request_atlas_batching);
    let mut atlas = spec.clone();
    atlas.request_atlas_batching = true;
    assert!(compile_region_field_preview(&atlas).is_err());

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let forbidden = [
        concat!("ActiveOnly", "ExperimentalNoHalo"),
        concat!("source_", "mask"),
        concat!("LocalBounds", "Metadata"),
        concat!("AlgebraicTile", "LocalMask"),
        concat!("Physical", "Gutter"),
        concat!("semantic ", "WGSL"),
        concat!("atlas ", "packer"),
    ];
    for needle in forbidden {
        assert!(!runtime_src.contains(needle));
    }
    let cpu_emit = concat!("emit_commitment_", "cpu");
    let cpu_decision = concat!("cpu_side_", "commitment");
    assert!(!runtime_src.contains(cpu_emit));
    assert!(!runtime_src.contains(cpu_decision));
    assert!(runtime_src.contains("FirstSliceResidencyStatus"));
}
