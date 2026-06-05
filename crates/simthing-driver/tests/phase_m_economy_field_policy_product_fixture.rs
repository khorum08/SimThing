//! Phase M Economy + FIELD_POLICY Product Fixture V1 — boundary economy signal into first-slice commitment.
//!
//! Option A product/acceptance fixture: test-level orchestration only. Discrete economy resolves
//! treasury at boundary; test maps stress to EML weight profiles; FIELD_POLICY commitment emerges through
//! existing GPU-resident reduction + EvalEML + Threshold + EmitEvent.

#[path = "support/daily_economy_session.rs"]
mod daily_economy;
#[path = "support/economy_field_policy_product_fixture.rs"]
mod economy_field_policy_product_fixture;
#[path = "support/first_slice_scenario_fixture.rs"]
mod first_slice_scenario_fixture;

use daily_economy::try_gpu;
use economy_field_policy_product_fixture::{
    assert_deficit_economy_postconditions, assert_surplus_economy_postconditions,
    commitment_scenario_spec, compile_commitment_scenario, compile_residency_scenario,
    run_deficit_economy_boundary_day, run_field_policy_commitment_with_economy_weights,
    run_surplus_economy_boundary_day, ECONOMY_STRESS_TREASURY_THRESHOLD,
    FIELD_POLICY_COMMITMENT_EVENT_KIND, HIGH_STRESS_EML_WEIGHTS, LOW_STRESS_EML_WEIGHTS, SEED,
};
use first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;
use simthing_driver::{
    FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryStatus, FirstSliceTickOptions,
};
use simthing_gpu::GpuContext;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_game_mode_resource_economy_authoring_preview, compile_region_field_preview,
    deserialize_first_slice_scenario_ron, MappingExecutionProfile, ResourceFlowOptInMode,
    SpecError,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

#[test]
fn economy_field_policy_fixtures_admit_and_preview() {
    let surplus_preview = compile_game_mode_resource_economy_authoring_preview(
        &daily_economy::surplus_game_mode(),
        &simthing_core::EmlExpressionRegistry::new(),
    )
    .expect("surplus economy preview");
    let deficit_preview = compile_game_mode_resource_economy_authoring_preview(
        &daily_economy::deficit_game_mode(),
        &simthing_core::EmlExpressionRegistry::new(),
    )
    .expect("deficit economy preview");

    assert!(!surplus_preview.report.resource_flow_enabled);
    assert!(!deficit_preview.report.resource_flow_enabled);
    assert_eq!(surplus_preview.report.transfer_count, 2);
    assert_eq!(deficit_preview.report.threshold_emit_count, 1);

    let scenario = commitment_scenario_spec();
    assert_eq!(
        scenario.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    let preview = compile_commitment_scenario();
    assert!(preview.region_field.commitment.is_some());
    assert_eq!(
        preview.region_field.parent_formula_class.as_deref(),
        Some("field_urgency")
    );

    let mut atlas_request = scenario.region_field.clone();
    atlas_request.request_atlas_batching = true;
    let err = compile_region_field_preview(&atlas_request).expect_err("atlas rejected");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(reason.contains("atlas"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn surplus_economy_produces_no_field_policy_commitment() {
    with_gpu(|ctx| {
        let economy = run_surplus_economy_boundary_day();
        assert_surplus_economy_postconditions(&economy);

        let preview = compile_commitment_scenario();
        let threshold = preview
            .region_field
            .commitment
            .as_ref()
            .expect("commitment")
            .threshold;
        let field_policy =
            run_field_policy_commitment_with_economy_weights(ctx, &preview, &economy);

        assert!(field_policy.threat.is_finite() && field_policy.threat > 0.0);
        assert!(field_policy.urgency.is_finite());
        assert!(
            field_policy.urgency < threshold,
            "surplus/low-stress urgency should stay below threshold: {} >= {}",
            field_policy.urgency,
            threshold
        );
        assert!(field_policy.report.threshold_events.is_empty());
        assert_eq!(field_policy.report.mapping.reduction_stencil_readbacks, 0);
        println!(
            "surplus treasury={} stress={} weights={:?} urgency={} threshold={} field_policy_events={}",
            field_policy.treasury,
            economy_field_policy_product_fixture::economy_stress_signal(field_policy.treasury),
            field_policy.eml_weights,
            field_policy.urgency,
            threshold,
            field_policy.report.threshold_events.len()
        );
    });
}

#[test]
fn deficit_economy_produces_one_field_policy_commitment() {
    with_gpu(|ctx| {
        let economy = run_deficit_economy_boundary_day();
        assert_deficit_economy_postconditions(&economy);

        let preview = compile_commitment_scenario();
        let commitment = preview
            .region_field
            .commitment
            .as_ref()
            .expect("commitment");
        let field_policy =
            run_field_policy_commitment_with_economy_weights(ctx, &preview, &economy);

        assert!(field_policy.threat.is_finite() && field_policy.threat > 0.0);
        assert!(field_policy.urgency.is_finite());
        assert!(
            field_policy.urgency > commitment.threshold,
            "deficit/high-stress urgency should cross threshold: {} <= {}",
            field_policy.urgency,
            commitment.threshold
        );
        assert_eq!(field_policy.report.threshold_events.len(), 1);
        let event = field_policy.report.threshold_events[0];
        assert_eq!(event.event_kind, FIELD_POLICY_COMMITMENT_EVENT_KIND);
        assert_eq!(event.event_kind, commitment.event_kind);
        assert_eq!(event.slot, commitment.parent_slot);
        assert_eq!(event.col, commitment.urgency_col);
        assert!((event.value - field_policy.urgency).abs() < 0.001);
        assert_eq!(field_policy.report.mapping.reduction_stencil_readbacks, 0);
        println!(
            "deficit treasury={} stress={} weights={:?} urgency={} threshold={} field_policy_events={}",
            field_policy.treasury,
            economy_field_policy_product_fixture::economy_stress_signal(field_policy.treasury),
            field_policy.eml_weights,
            field_policy.urgency,
            commitment.threshold,
            field_policy.report.threshold_events.len()
        );
    });
}

#[test]
fn economy_field_policy_product_fixture_is_deterministic() {
    with_gpu(|ctx| {
        let preview = compile_commitment_scenario();

        let run_surplus = || {
            let economy = run_surplus_economy_boundary_day();
            run_field_policy_commitment_with_economy_weights(ctx, &preview, &economy)
        };
        let surplus_a = run_surplus();
        let surplus_b = run_surplus();
        assert_eq!(surplus_a.treasury, surplus_b.treasury);
        assert_eq!(surplus_a.eml_weights, surplus_b.eml_weights);
        assert!((surplus_a.urgency - surplus_b.urgency).abs() < 0.001);
        assert_eq!(
            surplus_a.report.threshold_events,
            surplus_b.report.threshold_events
        );
        assert_eq!(
            surplus_a.report.mapping.total_dispatches,
            surplus_b.report.mapping.total_dispatches
        );

        let run_deficit = || {
            let economy = run_deficit_economy_boundary_day();
            run_field_policy_commitment_with_economy_weights(ctx, &preview, &economy)
        };
        let deficit_a = run_deficit();
        let deficit_b = run_deficit();
        assert_eq!(deficit_a.treasury, deficit_b.treasury);
        assert_eq!(deficit_a.eml_weights, deficit_b.eml_weights);
        assert!((deficit_a.urgency - deficit_b.urgency).abs() < 0.001);
        assert_eq!(
            deficit_a.report.threshold_events,
            deficit_b.report.threshold_events
        );
        assert_eq!(
            deficit_a.report.mapping.total_dispatches,
            deficit_b.report.mapping.total_dispatches
        );
    });
}

#[test]
fn summary_validity_and_residency_not_broken_with_economy_derived_weights() {
    with_gpu(|ctx| {
        let economy = run_surplus_economy_boundary_day();
        assert_eq!(economy.eml_weights, LOW_STRESS_EML_WEIGHTS);

        let preview = compile_residency_scenario();
        let mut session = FirstSliceScenarioFixtureSession::open(ctx, &preview).unwrap();
        session.queue_seeds(&[SEED]).unwrap();

        let hot = session
            .tick_with_scenario_commitment(
                ctx,
                FirstSliceTickOptions::hot_path(),
                economy.eml_weights,
            )
            .unwrap();
        assert_eq!(
            hot.mapping.residency.status,
            FirstSliceResidencyStatus::HotExecutedThisTick
        );
        assert_eq!(
            hot.mapping.summary.status,
            FirstSliceSummaryStatus::FreshThisTick
        );
        assert!(hot.mapping.summary.has_gpu_parent_summary);
        assert_eq!(hot.mapping.reduction_stencil_readbacks, 0);
        assert!(hot.threshold_events.is_empty());

        let cached = session
            .tick_with_scenario_commitment(
                ctx,
                FirstSliceTickOptions::hot_path(),
                economy.eml_weights,
            )
            .unwrap();
        assert_eq!(
            cached.mapping.residency.status,
            FirstSliceResidencyStatus::ResidentCached
        );
        assert_eq!(
            cached.mapping.summary.status,
            FirstSliceSummaryStatus::Cached { age_ticks: 1 }
        );
        assert!(!cached.mapping.summary.summary_used_for_commitment_scan);
        assert!(cached.threshold_events.is_empty());

        session
            .queue_seeds(&[FirstSliceSeed {
                row: 3,
                col: 3,
                value: 90.0,
            }])
            .unwrap();
        let refresh = session
            .tick_with_scenario_commitment(
                ctx,
                FirstSliceTickOptions::hot_path(),
                economy.eml_weights,
            )
            .unwrap();
        assert_eq!(
            refresh.mapping.residency.status,
            FirstSliceResidencyStatus::HotExecutedThisTick
        );
        assert_eq!(
            refresh.mapping.summary.status,
            FirstSliceSummaryStatus::FreshThisTick
        );
        assert_eq!(refresh.mapping.total_dispatches, 9);
    });
}

#[test]
fn economy_field_policy_product_fixture_posture_preserved() {
    let sim_sources = [
        include_str!("../../simthing-sim/src/lib.rs"),
        include_str!("../../simthing-sim/src/boundary.rs"),
    ];
    for text in sim_sources {
        assert!(
            !text.contains("DailyResolutionBoundary"),
            "forbidden DailyResolutionBoundary in simthing-sim"
        );
        assert!(
            !text.contains("struct Calendar")
                && !text.contains("enum Calendar")
                && !text.contains("struct Season")
                && !text.contains("enum Season"),
            "forbidden calendar/season types in simthing-sim"
        );
    }

    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowOptInMode::default(),
        ResourceFlowOptInMode::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let scenario = deserialize_first_slice_scenario_ron(
        economy_field_policy_product_fixture::COMMITMENT_SCENARIO_RON,
    )
    .expect("scenario parses");
    assert_eq!(
        scenario.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    assert!(!scenario.region_field.request_atlas_batching);

    assert!(
        daily_economy::SURPLUS_RON.contains("daily")
            || daily_economy::SURPLUS_RON.contains("ticks_per_day")
    );
    assert!(!daily_economy::SURPLUS_RON.contains("DailyResolutionBoundary"));

    assert_eq!(ECONOMY_STRESS_TREASURY_THRESHOLD, 95.0);
    assert_eq!(LOW_STRESS_EML_WEIGHTS, (0.2, 0.1));
    assert_eq!(HIGH_STRESS_EML_WEIGHTS, (0.9, 0.1));
}
