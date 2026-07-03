//! Phase M product commitment fixture: threshold event over first-slice urgency.

use simthing_driver::{
    estimate_first_slice_budget, FirstSliceCommitmentReport, FirstSliceMappingSession,
    FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_region_field_preview, deserialize_region_field_ron,
    CompiledFirstSliceCommitmentThreshold, MappingExecutionProfile, RegionFieldFormulaBindingSpec,
    RegionFieldIsolationPolicyEstimate, RegionFieldSpec, SpecError,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const PRODUCT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_suppression_field.ron");
const COMMITMENT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_field.ron");
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

fn commitment_spec() -> RegionFieldSpec {
    deserialize_region_field_ron(COMMITMENT_FIXTURE_RON).expect("commitment fixture RON parses")
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

fn open_seeded_session(ctx: &GpuContext, spec: &RegionFieldSpec) -> FirstSliceMappingSession {
    let mut session =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, spec)
            .unwrap();
    session.queue_seeds(&[SEED]).unwrap();
    session
}

fn admitted_commitment(spec: &RegionFieldSpec) -> CompiledFirstSliceCommitmentThreshold {
    compile_region_field_preview(spec)
        .expect("commitment fixture admits")
        .commitment
        .expect("commitment binding admitted")
}

fn run_commitment(ctx: &GpuContext, spec: &RegionFieldSpec, weights: (f32, f32)) -> CommitmentRun {
    let commitment = admitted_commitment(spec);
    let mut session = open_seeded_session(ctx, spec);
    let report = session
        .tick_with_commitment_spec_fixture(
            ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
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
fn product_commitment_ron_admits() {
    let spec = commitment_spec();
    assert_eq!(
        spec.parent_formula.as_ref().unwrap().formula_class,
        "field_urgency"
    );
    assert!(!spec.request_atlas_batching);

    let preview = compile_region_field_preview(&spec).expect("commitment fixture admits");
    let commitment = preview.commitment.expect("commitment binding admitted");
    assert_eq!(commitment.source_formula_class, "field_urgency");
    assert_eq!(commitment.parent_slot, 100);
    assert_eq!(commitment.urgency_col, URGENCY_COL);
    assert!(commitment.threshold.is_finite());
    assert_eq!(commitment.event_kind, 0x5345_4144);
    assert_eq!(
        preview.parent_formula_class.as_deref(),
        Some("field_urgency")
    );
    assert!(preview.reduction.is_some());

    let budget =
        estimate_first_slice_budget(&spec, RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas)
            .expect("budget preview passes");
    assert!(budget.estimated_bytes > 0);
    assert!(budget.estimated_bytes <= spec.max_region_field_vram_bytes.unwrap());
}
#[test]
fn product_commitment_default_profile_emits_no_event() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    with_gpu(|ctx| {
        let spec = commitment_spec();
        let commitment = admitted_commitment(&spec);
        let mut session =
            FirstSliceMappingSession::open(ctx, MappingExecutionProfile::Disabled, &spec).unwrap();
        session.queue_seeds(&[SEED]).unwrap();

        let report = session
            .tick_with_commitment_spec_fixture(
                ctx,
                FirstSliceTickOptions::debug_readback(),
                LOW_WEIGHTS,
                &commitment,
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
        let spec = commitment_spec();
        let threshold = admitted_commitment(&spec).threshold;
        let low = run_commitment(ctx, &spec, LOW_WEIGHTS);
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
        let spec = commitment_spec();
        let commitment = admitted_commitment(&spec);
        let threshold = commitment.threshold;
        let high = run_commitment(ctx, &spec, HIGH_WEIGHTS);
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
        assert_eq!(event.event_kind, commitment.event_kind);
        assert_eq!(event.slot, commitment.parent_slot);
        assert_eq!(event.col, commitment.urgency_col);
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
        let spec = commitment_spec();
        let first = run_commitment(ctx, &spec, HIGH_WEIGHTS);
        let second = run_commitment(ctx, &spec, HIGH_WEIGHTS);
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
    let authored = commitment_spec();
    assert!(!authored.request_atlas_batching);
    assert!(compile_region_field_preview(&authored)
        .unwrap()
        .commitment
        .is_some());
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
    for source in [
        PRODUCT_FIXTURE_RON,
        COMMITMENT_FIXTURE_RON,
        runtime_src,
        fixture_src,
    ] {
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
