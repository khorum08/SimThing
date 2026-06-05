//! Phase M Economy + FIELD_POLICY product fixture orchestration (Option A).
//!
//! Test-level wiring only: run discrete economy at boundary, read resolved treasury,
//! map treasury stress to authored EML weight profiles, then drive the existing
//! first-slice GPU-resident commitment path. CPU does not compute urgency or emit FIELD_POLICY events.

#![allow(dead_code)]

use super::daily_economy::{
    deficit_game_mode, open_daily_economy_session, open_daily_economy_session_with_thresholds,
    run_days_collecting_events, run_days_with_full_boundary, surplus_game_mode, treasury_amount,
    DEFICIT_DAILY_NET, INITIAL_TREASURY, LOW_STORAGE_EVENT_KIND, SURPLUS_DAILY_NET,
};
use super::first_slice_scenario_fixture::FirstSliceScenarioFixtureSession;
use simthing_driver::{FirstSliceCommitmentReport, FirstSliceSeed, FirstSliceTickOptions};
use simthing_gpu::{GpuContext, ThresholdEvent};
use simthing_spec::{
    compile_first_slice_scenario_preview, deserialize_first_slice_scenario_ron,
    CompiledFirstSliceScenarioPreview, FirstSliceScenarioSpec, GameModeSpec,
};

pub const COMMITMENT_SCENARIO_RON: &str =
    include_str!("../fixtures/first_slice_product_commitment_scenario.ron");
pub const RESIDENCY_SCENARIO_RON: &str =
    include_str!("../fixtures/first_slice_product_summary_validity_scenario.ron");

pub const FIELD_POLICY_COMMITMENT_EVENT_KIND: u32 = 0x5345_4144;
pub const SEED: FirstSliceSeed = FirstSliceSeed {
    row: 4,
    col: 4,
    value: 120.0,
};

/// Treasury stress threshold aligned with deficit fixture `emit_on_threshold` (95.0).
pub const ECONOMY_STRESS_TREASURY_THRESHOLD: f32 = 95.0;

/// Proven low-urgency profile from first-slice commitment fixture tests.
pub const LOW_STRESS_EML_WEIGHTS: (f32, f32) = (0.2, 0.1);
/// Proven high-urgency profile from first-slice commitment fixture tests.
pub const HIGH_STRESS_EML_WEIGHTS: (f32, f32) = (0.9, 0.1);

#[derive(Clone, Debug, PartialEq)]
pub struct EconomyBoundaryOutcome {
    pub treasury: f32,
    pub boundaries_run: u64,
    pub economy_events: Vec<ThresholdEvent>,
    pub high_stress: bool,
    pub eml_weights: (f32, f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPolicyCommitmentOutcome {
    pub report: FirstSliceCommitmentReport,
    pub threat: f32,
    pub urgency: f32,
    pub eml_weights: (f32, f32),
    pub treasury: f32,
    pub high_stress: bool,
}

pub fn commitment_scenario_spec() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(COMMITMENT_SCENARIO_RON)
        .expect("commitment scenario RON parses")
}

pub fn residency_scenario_spec() -> FirstSliceScenarioSpec {
    deserialize_first_slice_scenario_ron(RESIDENCY_SCENARIO_RON)
        .expect("residency scenario RON parses")
}

pub fn compile_commitment_scenario() -> CompiledFirstSliceScenarioPreview {
    compile_first_slice_scenario_preview(&commitment_scenario_spec()).expect("scenario admits")
}

pub fn compile_residency_scenario() -> CompiledFirstSliceScenarioPreview {
    compile_first_slice_scenario_preview(&residency_scenario_spec()).expect("scenario admits")
}

/// Map resolved treasury storage to fixture-authored EML weight profiles.
///
/// This is fixture orchestration only — not production runtime wiring.
pub fn eml_weights_from_treasury_stress(treasury: f32) -> ((f32, f32), bool) {
    if treasury <= ECONOMY_STRESS_TREASURY_THRESHOLD {
        (HIGH_STRESS_EML_WEIGHTS, true)
    } else {
        (LOW_STRESS_EML_WEIGHTS, false)
    }
}

pub fn economy_stress_signal(treasury: f32) -> f32 {
    (ECONOMY_STRESS_TREASURY_THRESHOLD - treasury).max(0.0)
}

pub fn run_surplus_economy_boundary_day() -> EconomyBoundaryOutcome {
    let game_mode = surplus_game_mode();
    let mut session = open_daily_economy_session(&game_mode, 1, 1);
    run_days_with_full_boundary(&mut session, 1);
    let treasury = treasury_amount(&session);
    let (eml_weights, high_stress) = eml_weights_from_treasury_stress(treasury);
    EconomyBoundaryOutcome {
        treasury,
        boundaries_run: 1,
        economy_events: Vec::new(),
        high_stress,
        eml_weights,
    }
}

pub fn run_deficit_economy_boundary_day() -> EconomyBoundaryOutcome {
    let game_mode = deficit_game_mode();
    let mut session = open_daily_economy_session_with_thresholds(&game_mode, 1, 1);
    let (boundaries_run, economy_events) = run_days_collecting_events(&mut session, 1);
    let treasury = treasury_amount(&session);
    let (eml_weights, high_stress) = eml_weights_from_treasury_stress(treasury);
    EconomyBoundaryOutcome {
        treasury,
        boundaries_run,
        economy_events,
        high_stress,
        eml_weights,
    }
}

pub fn run_economy_boundary_day(game_mode: &GameModeSpec) -> EconomyBoundaryOutcome {
    if game_mode.id.contains("deficit") {
        run_deficit_economy_boundary_day()
    } else {
        run_surplus_economy_boundary_day()
    }
}

pub fn open_seeded_commitment_session(
    ctx: &GpuContext,
    preview: &CompiledFirstSliceScenarioPreview,
) -> FirstSliceScenarioFixtureSession {
    let mut session = FirstSliceScenarioFixtureSession::open(ctx, preview).unwrap();
    session.queue_seeds(&[SEED]).unwrap();
    session
}

pub fn run_field_policy_commitment_with_economy_weights(
    ctx: &GpuContext,
    preview: &CompiledFirstSliceScenarioPreview,
    economy: &EconomyBoundaryOutcome,
) -> FieldPolicyCommitmentOutcome {
    let _commitment = preview
        .region_field
        .commitment
        .as_ref()
        .expect("commitment binding");
    let mut session = open_seeded_commitment_session(ctx, preview);
    let report = session
        .tick_with_scenario_commitment(ctx, FirstSliceTickOptions::hot_path(), economy.eml_weights)
        .unwrap();
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert!(report.mapping.reduction_executed);
    assert!(report.mapping.eml_executed);
    assert_eq!(report.mapping.reduction_stencil_readbacks, 0);

    let (threat, urgency) = session
        .diagnostic_readback_reduction_eml(ctx, economy.eml_weights)
        .unwrap();

    FieldPolicyCommitmentOutcome {
        report,
        threat,
        urgency,
        eml_weights: economy.eml_weights,
        treasury: economy.treasury,
        high_stress: economy.high_stress,
    }
}

pub fn assert_surplus_economy_postconditions(economy: &EconomyBoundaryOutcome) {
    let expected = INITIAL_TREASURY + SURPLUS_DAILY_NET;
    assert!(
        (economy.treasury - expected).abs() < 1e-4,
        "surplus treasury expected {expected}, got {}",
        economy.treasury
    );
    assert!(!economy.high_stress);
    assert_eq!(economy.eml_weights, LOW_STRESS_EML_WEIGHTS);
    assert!(economy_stress_signal(economy.treasury) <= 0.0);
}

pub fn assert_deficit_economy_postconditions(economy: &EconomyBoundaryOutcome) {
    let expected = INITIAL_TREASURY + DEFICIT_DAILY_NET;
    assert!(
        (economy.treasury - expected).abs() < 1e-4,
        "deficit treasury expected {expected}, got {}",
        economy.treasury
    );
    assert!(economy.high_stress);
    assert_eq!(economy.eml_weights, HIGH_STRESS_EML_WEIGHTS);
    assert!(economy_stress_signal(economy.treasury) > 0.0);
    assert!(
        economy
            .economy_events
            .iter()
            .any(|e| e.event_kind == LOW_STORAGE_EVENT_KIND),
        "deficit economy substrate should emit low_storage threshold event"
    );
}
