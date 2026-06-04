//! Phase M SummaryValidity V1 — first-slice summary policy/status for skipped/clean ticks.

mod support;

use simthing_driver::{FirstSliceSeed, FirstSliceSummaryStatus, FirstSliceTickOptions};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_first_slice_scenario_preview, compile_region_field_preview,
    deserialize_first_slice_scenario_ron, CompiledRegionFieldSummaryPolicy, FirstSliceScenarioSpec,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldSpec, RegionFieldSummaryPolicySpec,
};
use std::sync::Mutex;

use support::first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SUMMARY_SCENARIO_RON: &str =
    include_str!("fixtures/first_slice_product_summary_validity_scenario.ron");
const DISABLED_SCENARIO_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_scenario_disabled.ron");
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

fn summary_scenario() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(SUMMARY_SCENARIO_RON).expect("scenario RON parses")
}

fn open_summary_session(ctx: &GpuContext) -> FirstSliceScenarioFixtureSession {
    let preview = compile_first_slice_scenario_preview(&summary_scenario()).expect("admit");
    FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap()
}

#[test]
fn summary_policy_ron_admits() {
    let scenario = summary_scenario();
    assert_eq!(
        scenario.region_field.summary_policy,
        RegionFieldSummaryPolicySpec::CachedUntilDirtyWithZeroInitial
    );
    assert!(!scenario.region_field.request_atlas_batching);

    let preview = compile_first_slice_scenario_preview(&scenario).expect("scenario admits");
    assert_eq!(
        preview.region_field.summary_policy,
        CompiledRegionFieldSummaryPolicy::CachedUntilDirtyWithZeroInitial
    );

    let default_spec = compile_region_field_preview(&RegionFieldSpec {
        summary_policy: RegionFieldSummaryPolicySpec::default(),
        ..scenario.region_field.clone()
    })
    .expect("default summary policy admits");
    assert_eq!(
        default_spec.summary_policy,
        CompiledRegionFieldSummaryPolicy::CachedUntilDirtyWithZeroInitial
    );

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!MappingExecutionProfile::Disabled.enables_execution());
}

#[test]
fn summary_status_is_driver_owned() {
    let spec_src = include_str!("../../simthing-spec/src/spec/region_field.rs");
    let spec_lib = include_str!("../../simthing-spec/src/lib.rs");
    let spec_mod = include_str!("../../simthing-spec/src/spec/mod.rs");
    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");

    assert!(
        !spec_src.contains("RegionFieldSummaryStatus"),
        "simthing-spec region_field.rs must not define runtime summary status"
    );
    assert!(
        !spec_lib.contains("RegionFieldSummaryStatus"),
        "simthing-spec lib.rs must not re-export RegionFieldSummaryStatus"
    );
    assert!(
        !spec_mod.contains("RegionFieldSummaryStatus"),
        "simthing-spec spec/mod.rs must not re-export RegionFieldSummaryStatus"
    );
    assert!(
        runtime_src.contains("FirstSliceSummaryStatus"),
        "simthing-driver first_slice_mapping_runtime.rs must own FirstSliceSummaryStatus"
    );
}

#[test]
fn disabled_summary_status_semantics() {
    with_gpu(|ctx| {
        let scenario =
            deserialize_first_slice_scenario_ron(DISABLED_SCENARIO_RON).expect("disabled RON");
        assert_eq!(
            scenario.mapping_execution_profile,
            MappingExecutionProfile::Disabled
        );
        let preview = compile_first_slice_scenario_preview(&scenario).expect("admit");
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap();
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), LOW_WEIGHTS)
            .unwrap();
        assert_eq!(
            report.summary.status,
            FirstSliceSummaryStatus::InvalidOrUnavailable
        );
        assert!(!report.summary.has_gpu_parent_summary);
        assert!(!report.summary.summary_used_for_commitment_scan);
        assert_eq!(
            report.summary.policy,
            CompiledRegionFieldSummaryPolicy::CachedUntilDirtyWithZeroInitial
        );
    });
}

#[test]
fn summary_policy_does_not_enable_execution() {
    let scenario = summary_scenario();
    assert_eq!(
        scenario.region_field.summary_policy,
        RegionFieldSummaryPolicySpec::CachedUntilDirtyWithZeroInitial
    );
    let mut disabled = scenario.clone();
    disabled.mapping_execution_profile = MappingExecutionProfile::Disabled;
    let preview = compile_first_slice_scenario_preview(&disabled).expect("admit with policy");

    with_gpu(|ctx| {
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap();
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(report.total_dispatches, 0);
        assert_eq!(
            report.summary.status,
            FirstSliceSummaryStatus::InvalidOrUnavailable
        );
    });
}

fn zero_initial_spec() -> RegionFieldSpec {
    let mut spec = summary_scenario().region_field;
    spec.cadence = RegionFieldCadenceSpec::OnEvent;
    spec
}

#[test]
fn zero_initial_skip_before_execution() {
    with_gpu(|ctx| {
        let spec = zero_initial_spec();
        let preview = compile_region_field_preview(&spec).expect("admit");
        let scenario_preview =
            compile_first_slice_scenario_preview(&summary_scenario()).expect("scenario");
        let mut preview_bundle = scenario_preview;
        preview_bundle.region_field = preview;
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview_bundle).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), LOW_WEIGHTS)
            .unwrap();
        assert!(!report.scheduled);
        assert_eq!(report.total_dispatches, 0);
        assert_eq!(report.summary.status, FirstSliceSummaryStatus::ZeroInitial);
        assert!(!report.summary.has_gpu_parent_summary);
        assert_eq!(report.summary.age_ticks, 0);
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
    });
}

#[test]
fn fresh_summary_after_executed_tick() {
    with_gpu(|ctx| {
        let mut session = open_summary_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        let report = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert!(report.scheduled);
        assert_eq!(report.source_setup_dispatches, 1);
        assert_eq!(report.propagation_dispatches, 8);
        assert_eq!(report.total_dispatches, 9);
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(
            report.summary.status,
            FirstSliceSummaryStatus::FreshThisTick
        );
        assert_eq!(report.summary.age_ticks, 0);
        assert!(report.summary.has_gpu_parent_summary);
        assert_eq!(report.reduction_stencil_readbacks, 0);
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
    });
}

#[test]
fn cached_summary_on_skipped_clean_tick() {
    with_gpu(|ctx| {
        let mut session = open_summary_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        let fresh = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(fresh.summary.status, FirstSliceSummaryStatus::FreshThisTick);

        let cached1 = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert!(!cached1.scheduled);
        assert_eq!(cached1.total_dispatches, 0);
        assert_eq!(
            cached1.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );
        assert!(cached1.summary.has_gpu_parent_summary);
        assert!(cached1.field_values.is_none());
        assert!(cached1.reduction_parent_value.is_none());
        assert!(cached1.eml_output.is_none());
        assert_eq!(cached1.reduction_stencil_readbacks, 0);

        let cached2 = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached2.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 2 }
        );
    });
}

#[test]
fn dirty_seed_invalidates_cached_and_refreshes() {
    with_gpu(|ctx| {
        let mut session = open_summary_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        let fresh = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(fresh.summary.status, FirstSliceSummaryStatus::FreshThisTick);

        let cached = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );

        session
            .queue_seeds(&[FirstSliceSeed {
                row: 3,
                col: 3,
                value: 90.0,
            }])
            .unwrap();
        let refreshed = session
            .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert!(refreshed.scheduled);
        assert_eq!(refreshed.total_dispatches, 9);
        assert_eq!(
            refreshed.summary.status,
            FirstSliceSummaryStatus::FreshThisTick
        );
        assert_eq!(refreshed.summary.age_ticks, 0);
    });
}

#[test]
fn cached_summary_does_not_cpu_emit_event() {
    with_gpu(|ctx| {
        let mut session = open_summary_session(ctx);
        session.queue_seeds(&[SEED]).unwrap();
        session
            .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();

        let cached = session
            .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
            .unwrap();
        assert_eq!(
            cached.mapping.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );
        assert!(!cached.mapping.summary.summary_used_for_commitment_scan);
        assert!(cached.threshold_events.is_empty());
    });
}

#[test]
fn summary_validity_sequence_is_deterministic() {
    with_gpu(|ctx| {
        let run = |ctx: &GpuContext| {
            let mut session = open_summary_session(ctx);
            session.queue_seeds(&[SEED]).unwrap();
            let r0 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            let r1 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            let r2 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            session
                .queue_seeds(&[FirstSliceSeed {
                    row: 3,
                    col: 3,
                    value: 90.0,
                }])
                .unwrap();
            let r3 = session
                .tick_mapping(ctx, FirstSliceTickOptions::hot_path(), HIGH_WEIGHTS)
                .unwrap();
            (r0, r1, r2, r3)
        };

        let first = run(ctx);
        let second = run(ctx);
        assert_eq!(first.0.summary, second.0.summary);
        assert_eq!(first.1.summary, second.1.summary);
        assert_eq!(first.2.summary, second.2.summary);
        assert_eq!(first.3.summary, second.3.summary);
        assert_eq!(first.0.total_dispatches, second.0.total_dispatches);
    });
}

#[test]
fn summary_validity_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let scenario = summary_scenario();
    assert!(!scenario.region_field.request_atlas_batching);
    let mut atlas = scenario.clone();
    atlas.region_field.request_atlas_batching = true;
    assert!(compile_first_slice_scenario_preview(&atlas).is_err());

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let test_src = include_str!("phase_m_first_slice_summary_validity.rs");
    let forbidden = [
        concat!("ActiveOnly", "ExperimentalNoHalo"),
        concat!("source_", "mask"),
        concat!("LocalBounds", "Metadata"),
        concat!("AlgebraicTile", "LocalMask"),
        concat!("Physical", "Gutter"),
        concat!("atlas ", "packer"),
        concat!("semantic ", "WGSL"),
    ];
    for source in [SUMMARY_SCENARIO_RON, runtime_src, test_src] {
        for needle in forbidden {
            assert!(!source.contains(needle));
        }
    }
    let cpu_emit = concat!("emit_commitment_", "cpu");
    let cpu_decision = concat!("cpu_side_", "commitment");
    assert!(!test_src.contains(cpu_emit));
    assert!(!test_src.contains(cpu_decision));
    assert!(!runtime_src.contains(cpu_emit));
    assert!(!runtime_src.contains(cpu_decision));
}
