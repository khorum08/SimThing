//! Phase M FirstSliceScenarioSpec: scenario-level RON authoring wrapper for first-slice mapping + commitment.

mod support;

use simthing_driver::{
    estimate_first_slice_budget, FirstSliceCommitmentReport, FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_first_slice_scenario_preview, compile_region_field_preview,
    deserialize_first_slice_scenario_ron, CompiledFirstSliceScenarioPreview,
    FirstSliceScenarioSpec, MappingExecutionProfile, RegionFieldFormulaBindingSpec,
    RegionFieldIsolationPolicyEstimate, RegionFieldSpec, SpecError,
};
use std::sync::Mutex;

use support::first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const SCENARIO_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_scenario.ron");
const SCENARIO_DISABLED_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_commitment_scenario_disabled.ron");
const LOW_WEIGHTS: (f32, f32) = (0.2, 0.1);
const HIGH_WEIGHTS: (f32, f32) = (0.9, 0.1);
const SEED: FirstSliceSeed = FirstSliceSeed {
    row: 4,
    col: 4,
    value: 120.0,
};
const URGENCY_COL: u32 = 4;

#[derive(Debug)]
struct ScenarioCommitmentRun {
    report: FirstSliceCommitmentReport,
    threat: f32,
    urgency: f32,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn scenario_spec() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(SCENARIO_FIXTURE_RON).expect("scenario RON parses")
}

fn disabled_scenario_spec() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(SCENARIO_DISABLED_FIXTURE_RON)
        .expect("disabled scenario RON parses")
}

fn compile_scenario(spec: &FirstSliceScenarioSpec) -> CompiledFirstSliceScenarioPreview {
    compile_first_slice_scenario_preview(spec).expect("scenario admits")
}

fn assert_scenario_err(spec: &FirstSliceScenarioSpec, reason_substr: &str) {
    let err = compile_first_slice_scenario_preview(spec).expect_err("expected admission failure");
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

fn open_seeded_scenario(
    ctx: &GpuContext,
    preview: &CompiledFirstSliceScenarioPreview,
) -> FirstSliceScenarioFixtureSession {
    let mut session = FirstSliceScenarioFixtureSession::open(ctx, preview).unwrap();
    session.queue_seeds(&[SEED]).unwrap();
    session
}

fn run_scenario_commitment(
    ctx: &GpuContext,
    preview: &CompiledFirstSliceScenarioPreview,
    weights: (f32, f32),
) -> ScenarioCommitmentRun {
    let mut session = open_seeded_scenario(ctx, preview);
    let report = session
        .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), weights)
        .unwrap();
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert_eq!(report.mapping.source_setup_dispatches, 1);
    assert_eq!(report.mapping.propagation_dispatches, 8);
    assert_eq!(report.mapping.total_dispatches, 9);
    assert!(report.mapping.reduction_executed);
    assert!(report.mapping.eml_executed);
    assert_eq!(report.mapping.reduction_stencil_readbacks, 0);
    assert!(report.mapping.field_values.is_none());
    assert!(report.mapping.reduction_parent_value.is_none());
    assert!(report.mapping.eml_output.is_none());

    let (threat, urgency) = session
        .diagnostic_readback_reduction_eml(ctx, weights)
        .unwrap();
    ScenarioCommitmentRun {
        report,
        threat,
        urgency,
    }
}

#[test]
fn scenario_ron_admits() {
    let spec = scenario_spec();
    assert_eq!(spec.name, "first_slice_product_commitment_scenario");
    assert_eq!(
        spec.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    assert!(!spec.region_field.request_atlas_batching);

    let preview = compile_scenario(&spec);
    assert_eq!(preview.name, spec.name);
    assert_eq!(
        preview.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    assert_eq!(
        preview.region_field.parent_formula_class.as_deref(),
        Some("field_urgency")
    );
    assert!(preview.region_field.reduction.is_some());

    let commitment = preview
        .region_field
        .commitment
        .as_ref()
        .expect("commitment admits");
    assert_eq!(commitment.source_formula_class, "field_urgency");
    assert_eq!(commitment.parent_slot, 100);
    assert_eq!(commitment.urgency_col, URGENCY_COL);
    assert!(commitment.threshold.is_finite());
    assert_eq!(commitment.event_kind, 0x5345_4144);

    let budget = estimate_first_slice_budget(
        &spec.region_field,
        RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
    )
    .expect("budget preview passes");
    assert!(budget.estimated_bytes > 0);
    assert!(budget.estimated_bytes <= spec.region_field.max_region_field_vram_bytes.unwrap());
    assert_eq!(
        preview.budget_estimate_bytes,
        Some(budget.estimated_bytes),
        "scenario preview must retain budget estimate"
    );
}

#[test]
fn disabled_scenario_admits_but_does_not_execute() {
    let spec = disabled_scenario_spec();
    assert_eq!(
        spec.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );

    let preview = compile_scenario(&spec);
    assert_eq!(
        preview.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );

    with_gpu(|ctx| {
        let mut session = open_seeded_scenario(ctx, &preview);
        let report = session
            .tick_with_scenario_commitment(
                ctx,
                FirstSliceTickOptions::debug_readback(),
                LOW_WEIGHTS,
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
fn authored_scenario_executes_gpu_resident_hot_path() {
    with_gpu(|ctx| {
        let preview = compile_scenario(&scenario_spec());
        let run = run_scenario_commitment(ctx, &preview, HIGH_WEIGHTS);
        println!(
            "hot_path dispatches={} reduction_stencil_readbacks={} urgency={}",
            run.report.mapping.total_dispatches,
            run.report.mapping.reduction_stencil_readbacks,
            run.urgency
        );
    });
}

#[test]
fn authored_scenario_low_profile_emits_no_event() {
    with_gpu(|ctx| {
        let preview = compile_scenario(&scenario_spec());
        let threshold = preview
            .region_field
            .commitment
            .as_ref()
            .expect("commitment")
            .threshold;
        let low = run_scenario_commitment(ctx, &preview, LOW_WEIGHTS);
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
            "scenario_low low_threat={} low_urgency={} threshold={} event_count={}",
            low.threat,
            low.urgency,
            threshold,
            low.report.threshold_events.len()
        );
    });
}

#[test]
fn authored_scenario_high_profile_emits_expected_event() {
    with_gpu(|ctx| {
        let preview = compile_scenario(&scenario_spec());
        let commitment = preview
            .region_field
            .commitment
            .as_ref()
            .expect("commitment");
        let threshold = commitment.threshold;
        let high = run_scenario_commitment(ctx, &preview, HIGH_WEIGHTS);
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
            "scenario_high high_threat={} high_urgency={} threshold={} event_count={} event_kind={:#x}",
            high.threat,
            high.urgency,
            threshold,
            high.report.threshold_events.len(),
            event.event_kind
        );
    });
}

#[test]
fn authored_scenario_high_profile_event_is_deterministic() {
    with_gpu(|ctx| {
        let preview = compile_scenario(&scenario_spec());
        let first = run_scenario_commitment(ctx, &preview, HIGH_WEIGHTS);
        let second = run_scenario_commitment(ctx, &preview, HIGH_WEIGHTS);
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
fn invalid_scenario_specs_reject() {
    assert!(deserialize_first_slice_scenario_ron(
        "(name: \"missing_region_field\", mapping_execution_profile: SparseRegionFieldV1,)"
    )
    .is_err());

    let mut atlas = scenario_spec();
    atlas.region_field.request_atlas_batching = true;
    assert_scenario_err(&atlas, "atlas batching");

    let base = scenario_spec();
    let mut nan_threshold = base.clone();
    nan_threshold
        .region_field
        .commitment
        .as_mut()
        .unwrap()
        .threshold = f32::NAN;
    assert_scenario_err(&nan_threshold, "commitment threshold must be finite");

    let mut zero_event = base.clone();
    zero_event
        .region_field
        .commitment
        .as_mut()
        .unwrap()
        .event_kind = 0;
    assert_scenario_err(&zero_event, "commitment event_kind must be nonzero");

    let mut wrong_parent_slot = base.clone();
    wrong_parent_slot
        .region_field
        .commitment
        .as_mut()
        .unwrap()
        .parent_slot = 99;
    assert_scenario_err(
        &wrong_parent_slot,
        "commitment parent_slot must match reduction parent_slot",
    );

    let mut wrong_col = base.clone();
    wrong_col
        .region_field
        .commitment
        .as_mut()
        .unwrap()
        .urgency_col = 5;
    assert_scenario_err(&wrong_col, "commitment urgency_col must be 4");

    let mut wrong_formula = base.clone();
    wrong_formula.region_field.parent_formula = Some(RegionFieldFormulaBindingSpec {
        formula_class: "field_pressure".into(),
        tree_id: Some(7),
    });
    assert_scenario_err(
        &wrong_formula,
        "commitment requires parent_formula field_urgency",
    );

    let mut wrong_source_formula = base.clone();
    wrong_source_formula
        .region_field
        .commitment
        .as_mut()
        .unwrap()
        .source_formula_class = "field_pressure".into();
    assert_scenario_err(
        &wrong_source_formula,
        "commitment source_formula_class must be field_urgency",
    );

    let mut over_budget = base.clone();
    over_budget.region_field.max_region_field_vram_bytes = Some(64);
    assert_scenario_err(&over_budget, "VRAM budget exceeded");
    assert_region_field_err(&over_budget.region_field, "VRAM budget exceeded");
}

#[test]
fn scenario_production_test_boundary() {
    let lib_src = include_str!("../src/lib.rs");
    assert!(!lib_src.contains("first_slice_scenario_fixture"));
    assert!(!lib_src.contains("FirstSliceScenarioFixtureSession"));

    for source in [
        include_str!("../src/lib.rs"),
        include_str!("../src/first_slice_mapping_runtime.rs"),
    ] {
        assert!(
            !source.contains("Test-only first-slice scenario fixture session"),
            "production simthing-driver must not contain test-only scenario fixture session"
        );
    }
}

#[test]
fn scenario_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = scenario_spec();
    assert!(!spec.region_field.request_atlas_batching);
    assert_eq!(
        spec.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("MappingExecutionProfile"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let fixture_src = include_str!("phase_m_first_slice_scenario_spec.rs");
    let support_src = include_str!("support/first_slice_scenario_fixture.rs");
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
        SCENARIO_FIXTURE_RON,
        SCENARIO_DISABLED_FIXTURE_RON,
        runtime_src,
        fixture_src,
        support_src,
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
