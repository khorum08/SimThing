//! 0088 C1a escaped instrument-gap witness through the ordinary loader/admission doors.
#![cfg(windows)]

use simthing_driver::AnchorTableSnapshot;
use simthing_mapeditor::app::StudioAppState;
use simthing_mapeditor::clause_scenario_picker::{
    run_clause_picker_action_staged, ClausePickerActionResult, ClausePickerSelection,
};
use simthing_mapeditor::settings::EditorSettings;
use simthing_mapeditor::studio_scenario_library_ui::{StudioLoaderStage, StudioLoaderStageStatus};
use simthing_mapeditor::StudioLiveSessionBridge;
use std::path::Path;
use std::time::{Duration, Instant};

#[test]
fn rehearsal_studio_source_to_ready_tracks_successful_resident_admission() {
    let output = tempfile::tempdir().unwrap();
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause");
    let bundle_paths = [
        source.clone(),
        source.with_extension("base.json"),
        source.with_extension("dependencies.json"),
    ];
    let original_bytes = bundle_paths.each_ref().map(|p| std::fs::read(p).unwrap());
    let mut app = StudioAppState::default();
    let mut settings = EditorSettings::default();
    let mut bridge = StudioLiveSessionBridge::default();
    app.scenario_library.select_path(source.clone());
    let before_start = Instant::now();
    let token = app.scenario_library.begin_load_attempt();
    let after_start = Instant::now();
    let selection = ClausePickerSelection {
        clause_path: source.clone(),
        scenario_json_path: Some(output.path().join("source.simthing-scenario.json")),
        ..Default::default()
    };
    let worker = std::thread::spawn(move || {
        let mut events = Vec::new();
        let result = run_clause_picker_action_staged(&selection, None, &mut |e| events.push(e));
        (result, events)
    });
    let (result, events) = worker.join().unwrap();
    let ClausePickerActionResult::Loaded {
        session, ingest, ..
    } = result
    else {
        panic!("ordinary source load failed: {result:?}");
    };
    let authored_profile_before = format!("{:?}", session.authored_live_profile);
    for event in events {
        assert!(app.scenario_library.observe_load_attempt(token, event));
    }
    let stages_before_admission = app.scenario_library.load_progress.clone();
    assert_eq!(
        stages_before_admission
            .record(StudioLoaderStage::Projection)
            .status,
        StudioLoaderStageStatus::Passed
    );
    assert!(app.scenario_library.source_to_ready.is_none());
    assert!(!bridge.readout().attached);

    // Test-only delay at the existing worker-result -> main-thread admission boundary.
    // No production hook/sleep and no Bevy substitute application or timing baseline.
    std::thread::sleep(Duration::from_millis(25));
    let minimum_elapsed = after_start.elapsed();
    assert!(app
        .try_adopt_clause_load_attempt(
            token,
            session.clone(),
            &mut settings,
            &mut bridge,
            "ready".into()
        )
        .unwrap());
    let maximum_elapsed = before_start.elapsed();
    let sample = app
        .scenario_library
        .source_to_ready
        .clone()
        .expect("resident ready sample");
    assert!(
        sample.started_at >= before_start && sample.started_at <= after_start,
        "the retained start must belong to the accepted attempt, never restart at admission"
    );
    assert!(
        sample.elapsed >= minimum_elapsed,
        "must include worker and delayed main-thread admission"
    );
    assert!(
        sample.elapsed <= maximum_elapsed,
        "one monotonic interval, not a sum of stages"
    );
    assert!(sample.elapsed > Duration::ZERO);
    assert_eq!(sample.attempt_token, token);
    assert_eq!(sample.source_path, source.display().to_string());
    assert_eq!(
        sample.source_identity.as_deref(),
        Some("fnv1a64:ee4e4df9e8c9fbd9:5798")
    );
    assert_eq!(
        sample.profile_identity.as_deref(),
        Some("fnv1a64:bfcbc44323b304bf:23530")
    );
    assert_eq!(
        sample.dependencies,
        ingest.source_cache.as_ref().unwrap().dependencies
    );
    assert_eq!(
        app.scenario_library.load_progress, stages_before_admission,
        "historical stages unchanged"
    );
    assert_eq!(
        format!("{:?}", app.session.as_ref().unwrap().authored_live_profile),
        authored_profile_before
    );
    assert!(bridge.readout().attached);
    assert!(bridge.sim_session().is_some());
    assert_eq!(bridge.executed_ticks(), 0);
    assert_eq!(bridge.sim_session().unwrap().coord.day_index(), 0);
    assert!(
        app.scenario_library.is_current_load_attempt(token),
        "sample exists before reveal finishes attempt"
    );
    eprintln!("READY_BEFORE_REVEAL: {sample:?}");

    let admitted_identity = bridge.sim_session().unwrap().persisted_execution_identity();
    assert!(!app
        .try_adopt_clause_load_attempt(
            token,
            session.clone(),
            &mut settings,
            &mut bridge,
            "duplicate".into()
        )
        .unwrap());
    assert_eq!(app.scenario_library.source_to_ready.as_ref(), Some(&sample));
    assert_eq!(
        bridge.sim_session().unwrap().persisted_execution_identity(),
        admitted_identity
    );
    // The actual reveal branch finishes/closes the attempt. A later reveal must not
    // extend or replace the completed source-to-ready sample.
    std::thread::sleep(Duration::from_millis(25));
    assert!(app.scenario_library.finish_load_attempt(token));
    app.scenario_library.close();
    assert_eq!(app.scenario_library.source_to_ready.as_ref(), Some(&sample));
    assert!(
        bridge.consume_scheduled_ticks(1).is_ok(),
        "installed resident must be usable"
    );

    let document_before =
        serde_json::to_value(&app.session.as_ref().unwrap().scenario_authority).unwrap();
    let settings_before = serde_json::to_value(&settings).unwrap();
    let identity_before = bridge.sim_session().unwrap().persisted_execution_identity();
    let anchors_before = AnchorTableSnapshot::from_session(bridge.sim_session().unwrap());
    let readout_before = app.live_bridge_readout.clone();
    let failed = app.scenario_library.begin_load_attempt();
    assert!(
        app.scenario_library.source_to_ready.is_none(),
        "new attempt cannot show old success"
    );
    // Existing invalid-candidate admission boundary; never mutate source or valid session.
    let mut rejected = session.clone();
    rejected
        .authored_live_profile
        .as_mut()
        .unwrap()
        .game_mode
        .overlays[0]
        .targets_property = "missing::resource".into();
    let error = app
        .try_adopt_clause_load_attempt(
            failed,
            rejected,
            &mut settings,
            &mut bridge,
            "rejected".into(),
        )
        .expect_err("real resident admission must refuse");
    assert!(
        app.scenario_library.source_to_ready.is_none(),
        "failed admission must never publish success"
    );
    assert!(app.scenario_library.finish_load_attempt(failed));
    assert_eq!(
        serde_json::to_value(&app.session.as_ref().unwrap().scenario_authority).unwrap(),
        document_before
    );
    assert_eq!(serde_json::to_value(&settings).unwrap(), settings_before);
    assert_eq!(
        bridge.sim_session().unwrap().persisted_execution_identity(),
        identity_before
    );
    assert_eq!(
        AnchorTableSnapshot::from_session(bridge.sim_session().unwrap()).rows(),
        anchors_before.rows()
    );
    assert_eq!(app.live_bridge_readout, readout_before);
    assert_eq!(bridge.executed_ticks(), 1);
    assert_eq!(app.status_message, "ready");
    eprintln!("FAILED_ADMISSION_NO_SAMPLE_PRIOR_RESIDENT_PRESERVED: {error}");

    let cancelled = app.scenario_library.begin_load_attempt();
    app.scenario_library.cancel_load_attempt();
    assert!(!app
        .try_adopt_clause_load_attempt(
            cancelled,
            session.clone(),
            &mut settings,
            &mut bridge,
            "cancelled".into()
        )
        .unwrap());
    assert!(app.scenario_library.source_to_ready.is_none());
    assert_eq!(
        bridge.sim_session().unwrap().persisted_execution_identity(),
        identity_before
    );

    let stale = app.scenario_library.begin_load_attempt();
    let current = app.scenario_library.begin_load_attempt();
    assert_ne!(stale, current);
    assert!(!app
        .try_adopt_clause_load_attempt(
            stale,
            session.clone(),
            &mut settings,
            &mut bridge,
            "stale".into()
        )
        .unwrap());
    assert!(app.scenario_library.source_to_ready.is_none());
    assert_eq!(
        bridge.sim_session().unwrap().persisted_execution_identity(),
        identity_before
    );
    assert!(app
        .try_adopt_clause_load_attempt(
            current,
            session.clone(),
            &mut settings,
            &mut bridge,
            "current".into()
        )
        .unwrap());
    let current_sample = app.scenario_library.source_to_ready.clone().unwrap();
    assert_eq!(current_sample.attempt_token, current);
    assert_eq!(current_sample.source_identity, sample.source_identity);
    assert_eq!(current_sample.profile_identity, sample.profile_identity);
    assert!(!app
        .try_adopt_clause_load_attempt(
            stale,
            session,
            &mut settings,
            &mut bridge,
            "late stale".into()
        )
        .unwrap());
    assert!(!app.scenario_library.finish_load_attempt(stale));
    assert_eq!(
        app.scenario_library.source_to_ready.as_ref(),
        Some(&current_sample)
    );
    assert_eq!(app.status_message, "current");
    assert!(app.scenario_library.finish_load_attempt(current));
    assert_eq!(
        bundle_paths.each_ref().map(|p| std::fs::read(p).unwrap()),
        original_bytes
    );
    eprintln!("EXACTLY_ONCE_CANCEL_STALE_PROVENANCE_AND_UNCHANGED_BUNDLE: PASS");
}
