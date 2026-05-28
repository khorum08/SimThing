//! Phase M product commitment fixture: threshold event over first-slice urgency.

use simthing_driver::{
    FirstSliceCommitmentReport, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_region_field_preview, deserialize_region_field_ron, MappingExecutionProfile,
    RegionFieldSpec, SpecError,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const PRODUCT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_suppression_field.ron");
const COMMITMENT_EVENT_KIND: u32 = 0x5345_4144;
const LOW_WEIGHTS: (f32, f32) = (0.2, 0.1);
const HIGH_WEIGHTS: (f32, f32) = (0.9, 0.1);
const SEED: FirstSliceSeed = FirstSliceSeed {
    row: 4,
    col: 4,
    value: 120.0,
};
const URGENCY_COL: u32 = 4;

#[derive(Debug)]
struct CommitmentRun {
    report: FirstSliceCommitmentReport,
    threat: f32,
    urgency: f32,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn product_spec() -> RegionFieldSpec {
    deserialize_region_field_ron(PRODUCT_FIXTURE_RON).expect("product fixture RON parses")
}

fn assert_region_field_err(spec: &RegionFieldSpec, reason_substr: &str) {
    let err = compile_region_field_preview(spec).expect_err("expected admission failure");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

fn open_seeded_session(ctx: &GpuContext) -> FirstSliceMappingSession {
    let spec = product_spec();
    let mut session =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, &spec)
            .unwrap();
    session.queue_seeds(&[SEED]).unwrap();
    session
}

fn measure_profile(ctx: &GpuContext, weights: (f32, f32)) -> (f32, f32) {
    let mut session = open_seeded_session(ctx);
    let hot_report = session
        .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
        .unwrap();
    assert!(hot_report.enabled);
    assert!(hot_report.scheduled);
    assert_eq!(hot_report.reduction_stencil_readbacks, 0);
    session
        .diagnostic_readback_reduction_eml(ctx, weights)
        .unwrap()
}

fn commitment_threshold(ctx: &GpuContext) -> f32 {
    let (low_threat, low_urgency) = measure_profile(ctx, LOW_WEIGHTS);
    let (high_threat, high_urgency) = measure_profile(ctx, HIGH_WEIGHTS);
    assert!(low_threat.is_finite() && low_threat > 0.0);
    assert!(high_threat.is_finite() && high_threat > 0.0);
    assert!((low_threat - high_threat).abs() < 0.01);
    assert!(low_urgency.is_finite());
    assert!(high_urgency.is_finite());
    assert!(high_urgency > low_urgency);
    (low_urgency + high_urgency) * 0.5
}

fn run_commitment(ctx: &GpuContext, weights: (f32, f32), threshold: f32) -> CommitmentRun {
    let mut session = open_seeded_session(ctx);
    let report = session
        .tick_with_commitment_threshold_fixture(
            ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            threshold,
            COMMITMENT_EVENT_KIND,
        )
        .unwrap();
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert!(report.mapping.reduction_executed);
    assert!(report.mapping.eml_executed);
    assert_eq!(report.mapping.reduction_stencil_readbacks, 0);
    assert!(report.mapping.field_values.is_none());
    assert!(report.mapping.reduction_parent_value.is_none());
    assert!(report.mapping.eml_output.is_none());

    let (threat, urgency) = session
        .diagnostic_readback_reduction_eml(ctx, weights)
        .unwrap();
    CommitmentRun {
        report,
        threat,
        urgency,
    }
}

#[test]
fn product_commitment_default_profile_emits_no_event() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    with_gpu(|ctx| {
        let spec = product_spec();
        let mut session =
            FirstSliceMappingSession::open(ctx, MappingExecutionProfile::Disabled, &spec).unwrap();
        session.queue_seeds(&[SEED]).unwrap();

        let report = session
            .tick_with_commitment_threshold_fixture(
                ctx,
                FirstSliceTickOptions::debug_readback(),
                LOW_WEIGHTS,
                1.0,
                COMMITMENT_EVENT_KIND,
            )
            .unwrap();
        assert!(!report.mapping.enabled);
        assert!(!report.mapping.scheduled);
        assert_eq!(report.mapping.total_dispatches, 0);
        assert!(!report.mapping.reduction_executed);
        assert!(!report.mapping.eml_executed);
        assert_eq!(report.mapping.reduction_stencil_readbacks, 0);
        assert!(report.threshold_events.is_empty());
    });
}

#[test]
fn product_commitment_low_urgency_stays_below_threshold_without_event() {
    with_gpu(|ctx| {
        let threshold = commitment_threshold(ctx);
        let low = run_commitment(ctx, LOW_WEIGHTS, threshold);
        assert!(low.threat.is_finite() && low.threat > 0.0);
        assert!(low.urgency.is_finite());
        assert!(
            low.urgency < threshold,
            "low urgency should remain below threshold: {} >= {}",
            low.urgency,
            threshold
        );
        assert!(low.report.threshold_events.is_empty());
        println!(
            "commitment_low low_threat={} low_urgency={} threshold={} event_count_low={} dispatches={} reduction_stencil_readbacks={}",
            low.threat,
            low.urgency,
            threshold,
            low.report.threshold_events.len(),
            low.report.mapping.total_dispatches,
            low.report.mapping.reduction_stencil_readbacks
        );
    });
}

#[test]
fn product_commitment_high_urgency_crosses_threshold_and_emits_event() {
    with_gpu(|ctx| {
        let threshold = commitment_threshold(ctx);
        let high = run_commitment(ctx, HIGH_WEIGHTS, threshold);
        let parent_slot = product_spec().reduction.unwrap().parent_slot;
        assert!(high.threat.is_finite() && high.threat > 0.0);
        assert!(high.urgency.is_finite());
        assert!(
            high.urgency > threshold,
            "high urgency should cross threshold: {} <= {}",
            high.urgency,
            threshold
        );
        assert_eq!(high.report.threshold_events.len(), 1);
        let event = high.report.threshold_events[0];
        assert_eq!(event.event_kind, COMMITMENT_EVENT_KIND);
        assert_eq!(event.slot, parent_slot);
        assert_eq!(event.col, URGENCY_COL);
        assert!((event.value - high.urgency).abs() < 0.001);
        println!(
            "commitment_high high_threat={} high_urgency={} threshold={} event_count_high={} dispatches={} reduction_stencil_readbacks={}",
            high.threat,
            high.urgency,
            threshold,
            high.report.threshold_events.len(),
            high.report.mapping.total_dispatches,
            high.report.mapping.reduction_stencil_readbacks
        );
    });
}

#[test]
fn product_commitment_high_urgency_event_is_deterministic() {
    with_gpu(|ctx| {
        let threshold = commitment_threshold(ctx);
        let first = run_commitment(ctx, HIGH_WEIGHTS, threshold);
        let second = run_commitment(ctx, HIGH_WEIGHTS, threshold);
        assert_eq!(
            first.report.threshold_events.len(),
            second.report.threshold_events.len()
        );
        assert_eq!(
            first.report.threshold_events,
            second.report.threshold_events
        );
        assert!((first.urgency - second.urgency).abs() < 0.001);
        assert_eq!(
            first.report.mapping.total_dispatches,
            second.report.mapping.total_dispatches
        );
        assert_eq!(
            first.report.mapping.reduction_stencil_readbacks,
            second.report.mapping.reduction_stencil_readbacks
        );
    });
}

#[test]
fn product_commitment_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = product_spec();
    assert!(!spec.request_atlas_batching);
    let mut atlas_request = spec.clone();
    atlas_request.request_atlas_batching = true;
    assert_region_field_err(&atlas_request, "atlas batching");

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("MappingExecutionProfile"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let fixture_src = include_str!("phase_m_first_slice_product_commitment_fixture.rs");
    let forbidden = [
        concat!("ActiveOnly", "ExperimentalNoHalo"),
        concat!("source_", "mask"),
        concat!("LocalBounds", "Metadata"),
        concat!("AlgebraicTile", "LocalMask"),
        concat!("Physical", "Gutter"),
        concat!("atlas ", "packer"),
        concat!("semantic ", "WGSL"),
    ];
    for source in [PRODUCT_FIXTURE_RON, runtime_src, fixture_src] {
        for needle in forbidden {
            assert!(!source.contains(needle));
        }
    }
    let cpu_emit = concat!("emit_commitment_", "cpu");
    let cpu_decision = concat!("cpu_side_", "commitment");
    assert!(!fixture_src.contains(cpu_emit));
    assert!(!fixture_src.contains(cpu_decision));
    assert!(!runtime_src.contains(cpu_emit));
    assert!(!runtime_src.contains(cpu_decision));
}
