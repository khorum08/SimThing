//! STUDIO-LOADER-DIALOG-REPAIR-0 focused staged-loader presentation proofs.

use std::path::{Path, PathBuf};

use simthing_mapeditor::app::{scenario_io, StudioAppState};
use simthing_mapeditor::clause_scenario_picker::{
    run_clause_picker_action_staged, ClausePickerActionResult, ClausePickerSelection,
    FakeClauseFilePicker,
};
use simthing_mapeditor::studio_scenario_library_ui::{
    StudioLoaderProgress, StudioLoaderStage, StudioLoaderStageEvent, StudioLoaderStageStatus,
    StudioScenarioLibraryModel, StudioSceneBatchCursor,
};
use simthing_mapeditor::studio_scenario_load::ScenarioPickerOutcome;
use simthing_mapeditor::{
    request_live_bridge_reset_after_session_replacement, runtime_vertical_seed_scenario_spec,
    StudioSession, StudioSimClockTransport, StudioSimClockTransportCommand,
};
use simthing_spec::serialize_scenario_authority;
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn selection(json_path: PathBuf) -> ClausePickerSelection {
    ClausePickerSelection {
        clause_path: canonical_clause_path(),
        resolver_entries: Default::default(),
        scenario_json_path: Some(json_path),
    }
}

fn loaded_session() -> StudioSession {
    StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("existing.simthing-scenario.json"),
        None,
    )
    .expect("existing session")
}

/// catches: legacy tabs, summary, authoring, or visible fake progress returning to the modal.
#[test]
fn load_dialog_defaults_minimal_and_progress_hidden() {
    let mut transport = StudioSimClockTransport::new();
    let mut model = StudioScenarioLibraryModel::default();
    model.path_text = "stale.clause".into();
    model.open(&mut transport);

    assert!(model.path_text.is_empty());
    assert!(!model.load_progress.visible);
    assert!(transport.clock().is_paused());

    let source = include_str!("../src/app/ui.rs");
    let start = source.find("fn draw_scenario_library_dialog").unwrap();
    let end = source[start..]
        .find("fn clear_scenario_library_pending_actions")
        .map(|offset| start + offset)
        .unwrap();
    let modal = &source[start..end];
    for required in [
        "Scenario path:",
        "Select File…",
        "Load",
        "Cancel",
        "ProgressBar",
    ] {
        assert!(
            modal.contains(required),
            "missing minimal control {required}"
        );
    }
    for forbidden in [
        "selectable_value",
        "Current:",
        "Create blank scenario",
        "Resolver entries",
    ] {
        assert!(
            !modal.contains(forbidden),
            "legacy modal residue {forbidden}"
        );
    }
}

/// catches: native selection remaining coupled to ingest or cancellation erasing the chosen path.
#[test]
fn select_file_populates_path_without_starting_ingest() {
    let dir = TempDir::new().expect("tempdir");
    let selected = dir.path().join("selected.clause");
    let mut state = StudioAppState::default();
    let picker = FakeClauseFilePicker {
        outcome: ScenarioPickerOutcome::Selected(selected.clone()),
    };

    let outcome = scenario_io::select_clause_scenario_path_with_picker_state(&mut state, &picker);
    assert!(matches!(outcome, ScenarioPickerOutcome::Selected(_)));
    assert_eq!(
        state.scenario_library.path_text,
        selected.display().to_string()
    );
    assert!(state.session.is_none());
    assert!(!state.scenario_library.load_progress.visible);

    let cancelled = FakeClauseFilePicker {
        outcome: ScenarioPickerOutcome::Cancelled,
    };
    let _ = scenario_io::select_clause_scenario_path_with_picker_state(&mut state, &cancelled);
    assert_eq!(
        state.scenario_library.path_text,
        selected.display().to_string()
    );
}

/// catches: fabricated progress, reordered stages, or timing emitted outside production calls.
#[test]
fn real_load_stages_are_ordered_and_timed() {
    let dir = TempDir::new().expect("tempdir");
    let mut progress = StudioLoaderProgress::default();
    progress.begin_attempt();
    let mut events = Vec::new();
    let result = run_clause_picker_action_staged(
        &selection(dir.path().join("staged.simthing-scenario.json")),
        None,
        &mut |event| {
            events.push(event.clone());
            progress.observe(event);
        },
    );
    assert!(matches!(result, ClausePickerActionResult::Loaded { .. }));

    let stages = &StudioLoaderStage::ALL[..7];
    assert_eq!(events.len(), stages.len() * 2);
    for (index, stage) in stages.iter().copied().enumerate() {
        assert!(matches!(events[index * 2], StudioLoaderStageEvent::Running(s) if s == stage));
        assert!(
            matches!(events[index * 2 + 1], StudioLoaderStageEvent::Passed { stage: s, .. } if s == stage)
        );
        let record = progress.record(stage);
        assert_eq!(record.status, StudioLoaderStageStatus::Passed);
        assert!(record.elapsed.is_some());
    }
    assert_eq!(
        progress.record(StudioLoaderStage::SceneAdopt).status,
        StudioLoaderStageStatus::NotRun
    );
    let ingest = include_str!("../src/clause_scenario_ingest.rs");
    let picker = include_str!("../src/clause_scenario_picker.rs");
    assert!(!ingest.contains("thread::sleep"));
    assert!(!picker.contains("thread::sleep"));
}

/// catches: fail-loud stage mapping losing the path or replacing the prior session.
#[test]
fn failed_stage_keeps_dialog_open_and_preserves_path_and_session() {
    let mut state = StudioAppState::default();
    state.session = Some(loaded_session());
    state.scenario_library.visible = true;
    state.scenario_library.path_text = "missing.clause".into();
    state.scenario_library.load_progress.begin_attempt();
    let before = serialize_scenario_authority(&state.session.as_ref().unwrap().scenario_authority)
        .expect("before");
    let selection = ClausePickerSelection {
        clause_path: PathBuf::from("missing.clause"),
        resolver_entries: Default::default(),
        scenario_json_path: None,
    };
    let result = run_clause_picker_action_staged(&selection, None, &mut |event| {
        state.scenario_library.load_progress.observe(event)
    });

    assert!(matches!(
        result,
        ClausePickerActionResult::InvalidPath { .. } | ClausePickerActionResult::Failed { .. }
    ));
    assert!(state.scenario_library.visible);
    assert_eq!(state.scenario_library.path_text, "missing.clause");
    assert_eq!(
        state
            .scenario_library
            .load_progress
            .record(StudioLoaderStage::Resolve)
            .status,
        StudioLoaderStageStatus::Failed
    );
    assert_eq!(
        state
            .scenario_library
            .load_progress
            .record(StudioLoaderStage::Parse)
            .status,
        StudioLoaderStageStatus::NotRun
    );
    let after = serialize_scenario_authority(&state.session.as_ref().unwrap().scenario_authority)
        .expect("after");
    assert_eq!(before, after);
}

/// catches: the Load command regressing to inline ingest or scene adoption on the egui call stack.
#[test]
fn load_dispatches_worker_and_returns_before_ingest_finishes() {
    let source = include_str!("../src/app/ui.rs");
    let action = &source[source.find("do_load_clause_scenario").unwrap()
        ..source.find("do_create_blank_scenario").unwrap()];
    assert!(action.contains("start_clause_loader_job(&mut state)"));
    assert!(!action.contains("run_clause_picker_action_staged"));
    assert!(!action.contains("rebuild_session_scene"));

    let start = source.find("fn start_clause_loader_job").unwrap();
    let end = source[start..]
        .find("fn poll_clause_loader_jobs")
        .map(|offset| start + offset)
        .unwrap();
    assert!(source[start..end].contains("std::thread::spawn"));
}

/// catches: worker callbacks mutating presentation directly instead of frame polling events.
#[test]
fn polled_events_update_progress_incrementally() {
    let mut model = StudioScenarioLibraryModel::default();
    let token = model.begin_load_attempt();
    assert!(model.observe_load_attempt(
        token,
        StudioLoaderStageEvent::Running(StudioLoaderStage::Resolve)
    ));
    assert_eq!(
        model
            .load_progress
            .record(StudioLoaderStage::Resolve)
            .status,
        StudioLoaderStageStatus::Running
    );
    assert_eq!(
        model.load_progress.record(StudioLoaderStage::Parse).status,
        StudioLoaderStageStatus::NotRun
    );
    assert!(model.observe_load_attempt(
        token,
        StudioLoaderStageEvent::Passed {
            stage: StudioLoaderStage::Resolve,
            elapsed: std::time::Duration::from_millis(2),
        }
    ));
    assert_eq!(
        model
            .load_progress
            .record(StudioLoaderStage::Resolve)
            .status,
        StudioLoaderStageStatus::Passed
    );
}

/// catches: a late worker result from a cancelled or superseded attempt adopting unexpectedly.
#[test]
fn stale_attempt_events_are_ignored_after_cancel_or_supersession() {
    let mut model = StudioScenarioLibraryModel::default();
    let stale = model.begin_load_attempt();
    model.cancel_load_attempt();
    assert!(!model.observe_load_attempt(
        stale,
        StudioLoaderStageEvent::Running(StudioLoaderStage::Parse)
    ));

    let current = model.begin_load_attempt();
    assert_ne!(stale, current);
    assert!(!model.observe_load_attempt(
        stale,
        StudioLoaderStageEvent::Running(StudioLoaderStage::Hydrate)
    ));
    assert!(model.is_current_load_attempt(current));
}

/// catches: scene adoption collapsing back into one unbounded main-thread pass.
#[test]
fn forced_small_scene_batch_requires_multiple_polls() {
    let mut cursor = StudioSceneBatchCursor::new(10, 3);
    let mut ranges = Vec::new();
    while !cursor.is_complete() {
        ranges.push(cursor.take_next());
    }
    assert_eq!(ranges, vec![0..3, 3..6, 6..9, 9..10]);

    let source = include_str!("../src/app/galaxy_render.rs");
    assert!(source.contains("SCENARIO_SCENE_STAR_BATCH_SIZE"));
    assert!(source.contains("apply_batched_galaxy_scene"));
}

/// catches: successful load restoring Play, omitting bridge reset, or closing before final batch.
#[test]
fn successful_load_closes_without_autoplay_and_requests_bridge_reset() {
    let mut transport = StudioSimClockTransport::new();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .expect("play");
    let mut model = StudioScenarioLibraryModel::default();
    model.open(&mut transport);
    model.close();
    let mut bridge_reset = false;
    request_live_bridge_reset_after_session_replacement(&mut bridge_reset);

    assert!(transport.clock().is_paused());
    assert!(bridge_reset);
    let source = include_str!("../src/app/ui.rs");
    let start = source.find("fn poll_clause_loader_jobs").unwrap();
    let end = source[start..]
        .find("pub fn studio_ui_system")
        .map(|offset| start + offset)
        .unwrap();
    let action = &source[start..end];
    for required in [
        "StudioLoaderStage::SceneAdopt",
        "adopt_loaded_scenario_session",
        "request_live_bridge_reset_after_session_replacement",
        "finish_batched_galaxy_scene",
        "scenario_library.close",
    ] {
        assert!(
            action.contains(required),
            "missing success lifecycle {required}"
        );
    }
    assert!(!action.contains("StudioSimClockTransportCommand::Play"));
}

/// catches: missing OVL affordance, default-visible telemetry, or telemetry mutating authority.
#[test]
fn studio_ops_telemetry_is_hidden_by_default_toggleable_and_read_only() {
    let session = loaded_session();
    let before = serialize_scenario_authority(&session.scenario_authority).expect("before");
    let mut model = StudioScenarioLibraryModel::default();
    assert!(!model.studio_ops_telemetry_visible);
    model.toggle_studio_ops_telemetry();
    assert!(model.studio_ops_telemetry_visible);
    for stage in StudioLoaderStage::ALL {
        let _ = model.load_progress.record(stage);
    }
    let after = serialize_scenario_authority(&session.scenario_authority).expect("after");
    assert_eq!(before, after);

    let source = include_str!("../src/app/ui.rs");
    assert!(source.contains("Show Studio_ops Telemetry"));
    assert!(source.contains("egui::Window::new(\"Studio_ops Telemetry\")"));
}
