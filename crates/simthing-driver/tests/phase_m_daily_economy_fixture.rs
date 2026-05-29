//! Phase M Daily Economy Fixture V1 — Clausewitz-style daily banking over existing
//! boundary cadence and discrete ResourceEconomySpec authoring.

#[path = "support/daily_economy_session.rs"]
mod support;

use simthing_driver::{open_replay_with_spec, read_spec_replay_file};
use simthing_sim::PipelineFlags;
use simthing_spec::{MappingExecutionProfile, ResourceFlowOptInMode};
use support::{
    assert_mapping_and_resource_flow_posture, daily_economy_scenario, deficit_game_mode,
    open_daily_economy_session, open_daily_economy_session_with_thresholds,
    run_days_collecting_events, run_days_with_full_boundary, surplus_game_mode, treasury_amount, try_gpu, DEFICIT_DAILY_NET, INITIAL_TREASURY, LOW_STORAGE_EVENT_KIND,
    SURPLUS_DAILY_NET, DEFICIT_RON, SURPLUS_RON,
};

#[test]
fn daily_economy_ron_admits_and_compiles() {
    let game_mode = surplus_game_mode();
    assert_eq!(game_mode.resource_economy.as_ref().unwrap().transfers.len(), 2);
    assert_eq!(game_mode.resource_economy.as_ref().unwrap().recipes.len(), 1);
    assert_eq!(game_mode.resource_economy.as_ref().unwrap().emissions.len(), 0);

    let scenario = daily_economy_scenario(1, 5);
    assert_eq!(scenario.ticks_per_day, 1);

    if try_gpu() {
        let session = open_daily_economy_session(&game_mode, 1, 1);
        let registry = session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .expect("economy registry after install");
        assert_eq!(registry.registrations.transfers.len(), 2);
        assert_eq!(registry.registrations.recipes.len(), 1);
        assert_eq!(registry.registrations.emissions.len(), 0);
        assert!(registry.registrations.emit_on_threshold.is_empty());
    }

    assert_eq!(
        game_mode
            .resource_flow
            .as_ref()
            .map(|s| s.opt_in_mode)
            .unwrap_or(ResourceFlowOptInMode::Disabled),
        ResourceFlowOptInMode::Disabled
    );
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
}

#[test]
fn one_day_surplus_banks_into_treasury() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = surplus_game_mode();
    let mut session = open_daily_economy_session(&game_mode, 1, 5);

    assert!(session.proto.flags.use_accumulator_transfer);
    assert!(!session.proto.flags.use_accumulator_emission);
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_transfer_active);
    assert!(
        !session.state.accumulator_emission_active,
        "daily banking uses discrete transfers/recipes, not EmitEvent emission"
    );
    assert_mapping_and_resource_flow_posture(&session);

    run_days_with_full_boundary(&mut session, 1);
    assert_eq!(session.coord.day_index(), 1);

    let treasury = treasury_amount(&session);
    let expected = INITIAL_TREASURY + SURPLUS_DAILY_NET;
    assert!(
        (treasury - expected).abs() < 1e-4,
        "treasury after one day: expected {expected}, got {treasury}"
    );
}

#[test]
fn multi_day_accumulation_is_deterministic() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = surplus_game_mode();
    let run_once = || {
        let mut session = open_daily_economy_session(&game_mode, 1, 5);
        let mut treasury_trace = Vec::new();
        for _ in 0..5 {
            run_days_with_full_boundary(&mut session, 1);
            treasury_trace.push(treasury_amount(&session));
        }
        (treasury_trace, treasury_amount(&session))
    };

    let (trace_a, treasury_a) = run_once();
    let (trace_b, treasury_b) = run_once();

    let expected_trace: Vec<f32> = (1..=5)
        .map(|day| INITIAL_TREASURY + day as f32 * SURPLUS_DAILY_NET)
        .collect();

    assert_eq!(trace_a, expected_trace, "daily treasury trace A");
    assert_eq!(trace_b, expected_trace, "daily treasury trace B");
    assert_eq!(treasury_a.to_bits(), treasury_b.to_bits());
}

#[test]
fn multi_day_replay_matches_storage_trajectory() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = surplus_game_mode();
    let scenario = daily_economy_scenario(1, 3);

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("daily_economy.replay.ldjson");
    let mut record_session = open_daily_economy_session(&game_mode, 1, 3);
    let summary = record_session
        .record_to_path(&path, 3)
        .expect("record three-day run");
    let live_treasury = treasury_amount(&record_session);

    let loaded = read_spec_replay_file(&path).expect("read replay");
    assert_eq!(loaded.frames.len(), 3);
    assert!(
        loaded.spec_snapshot.is_some(),
        "daily economy replay must carry spec_snapshot for reinstall"
    );

    let (mut replay_session, _, frames) =
        open_replay_with_spec(&path, &game_mode, scenario).expect("replay open");
    assert_eq!(frames.len(), 3);
    run_days_with_full_boundary(&mut replay_session, 3);
    let replay_treasury = treasury_amount(&replay_session);

    assert_eq!(summary.boundaries_run, 3);
    assert_eq!(replay_treasury.to_bits(), live_treasury.to_bits());
}

#[test]
fn deficit_upkeep_emits_low_storage_threshold_event() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = deficit_game_mode();
    let mut session = open_daily_economy_session_with_thresholds(&game_mode, 1, 3);

    let registry = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("economy registry");
    assert_eq!(registry.registrations.emit_on_threshold.len(), 1);
    assert_eq!(
        registry.registrations.emit_on_threshold[0].event_kind,
        LOW_STORAGE_EVENT_KIND
    );

    let (boundaries_run, events) = run_days_collecting_events(&mut session, 1);
    assert_eq!(boundaries_run, 1);

    let treasury = treasury_amount(&session);
    let expected = INITIAL_TREASURY + DEFICIT_DAILY_NET;
    assert!(
        (treasury - expected).abs() < 1e-4,
        "deficit treasury: expected {expected}, got {treasury}"
    );

    let low_storage: Vec<_> = events
        .iter()
        .filter(|e| e.event_kind == LOW_STORAGE_EVENT_KIND)
        .collect();
    assert_eq!(
        low_storage.len(),
        1,
        "threshold substrate must emit exactly one low_storage_event"
    );
    assert!(low_storage[0].value < 95.0);
}

#[test]
fn sub_day_boundary_cadence_documented_not_daily_amount_scaling() {
    // Discrete resource economy executes per substrate tick. Sub-day boundary cadence
    // (ticks_per_day=N) without per-tick amount scaling is covered by
    // phase_m_boundary_cadence_doctrine; this fixture intentionally uses ticks_per_day=1.
    let scenario = daily_economy_scenario(4, 1);
    assert_eq!(scenario.ticks_per_day, 4);
}

#[test]
fn posture_preserved_no_new_daily_semantics() {
    let sources = [
        include_str!("../../simthing-sim/src/lib.rs"),
        include_str!("../../simthing-sim/src/boundary.rs"),
        include_str!("../src/session.rs"),
        SURPLUS_RON,
        DEFICIT_RON,
    ];
    for text in sources {
        assert!(
            !text.contains("DailyResolutionBoundary"),
            "forbidden DailyResolutionBoundary primitive"
        );
    }

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("Calendar"));
    assert!(!sim_lib.contains("PauseState"));

    let flags = PipelineFlags::default();
    assert!(!flags.use_accumulator_resource_flow);
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
}
