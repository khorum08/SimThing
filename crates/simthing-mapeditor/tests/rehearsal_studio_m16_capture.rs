//! B0a instrument validity, deliberately not a performance comparison.
#![cfg(windows)]
use bevy::{prelude::*, time::TimeUpdateStrategy};
use simthing_mapeditor::{app::StudioAppState, rehearsal_studio_m16_capture::*};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

fn config(path: std::path::PathBuf, max_samples: usize) -> CaptureConfig {
    CaptureConfig {
        output: path,
        max_samples,
        metadata: [
            "condition",
            "repetition",
            "code_revision",
            "instrument_revision",
            "seed",
            "hardware",
            "backend",
            "driver",
            "compiler",
            "build_flags",
            "cache_state",
            "warmup",
            "exact_command",
            "operation_sequence",
            "workload_cardinalities",
            "presentation_settings",
            "subscriptions",
        ]
        .into_iter()
        .map(|k| {
            (
                k.into(),
                "synthetic validity fixture; no benchmark claim".into(),
            )
        })
        .collect::<BTreeMap<_, _>>(),
    }
}

#[test]
fn rehearsal_studio_m16_real_admission_replacement_has_distinct_publication_epoch() {
    use simthing_mapeditor::clause_scenario_picker::{
        run_clause_picker_action_staged, ClausePickerActionResult, ClausePickerSelection,
    };
    let output = tempfile::tempdir().unwrap();
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/stellaristhing_base.clause");
    let paths = [
        source.clone(),
        source.with_extension("base.json"),
        source.with_extension("dependencies.json"),
    ];
    let before = paths.each_ref().map(|p| std::fs::read(p).unwrap());
    let selection = ClausePickerSelection {
        clause_path: source,
        scenario_json_path: Some(output.path().join("capture.simthing-scenario.json")),
        ..default()
    };
    let loaded =
        std::thread::spawn(move || run_clause_picker_action_staged(&selection, None, &mut |_| {}))
            .join()
            .unwrap();
    let ClausePickerActionResult::Loaded { session, .. } = loaded else {
        panic!("load failed: {loaded:?}");
    };
    let mut state = StudioAppState::default();
    let mut bridge = simthing_mapeditor::StudioLiveSessionBridge::default();
    state
        .m16
        .start(
            config(output.path().join("admission.json"), 100),
            serde_json::json!({}),
        )
        .unwrap();
    for _ in 0..2 {
        state
            .try_adopt_loaded_scenario_session(
                session.clone(),
                &mut Default::default(),
                &mut bridge,
                "capture validity".into(),
            )
            .unwrap();
        assert_eq!(bridge.executed_ticks(), 0);
        consume_projection(&mut state, Client::Egui);
        consume_projection(&mut state, Client::Native);
    }
    let publications: Vec<_> = state
        .m16
        .capture
        .as_ref()
        .unwrap()
        .samples
        .iter()
        .filter_map(|s| match s {
            Sample::Publication { stamp } => Some(*stamp),
            _ => None,
        })
        .collect();
    assert_eq!(publications.len(), 2);
    assert_eq!(publications[0].generation, publications[1].generation);
    assert_ne!(publications[0].scene, publications[1].scene);
    assert_ne!(
        publications[0].resident_epoch,
        publications[1].resident_epoch
    );
    assert_eq!(
        state
            .m16
            .capture
            .as_ref()
            .unwrap()
            .samples
            .iter()
            .filter(|s| matches!(s, Sample::Consumption { .. }))
            .count(),
        4
    );
    assert_eq!(paths.each_ref().map(|p| std::fs::read(p).unwrap()), before);
}

#[test]
fn rehearsal_studio_m16_raw_tails_stop_export_and_qualification() {
    let output = tempfile::tempdir().unwrap();
    let path = output.path().join("raw.json");
    let mut capture = M16Capture::default();
    let mut invalid = config(path.clone(), 2);
    invalid.metadata.remove("warmup");
    assert!(capture.start(invalid, serde_json::json!({})).is_err());
    let cfg = config(path.clone(), 2);
    capture
        .start(cfg.clone(), serde_json::json!({"phase":"fixture"}))
        .unwrap();
    assert!(
        capture.start(cfg.clone(), serde_json::json!({})).is_err(),
        "never silently discard retained samples"
    );
    let mut time = Time::<Real>::default();
    let base = Instant::now() + Duration::from_secs(1);
    let strategy = TimeUpdateStrategy::ManualDuration(Duration::from_millis(1));
    time.update_with_instant(base);
    capture.frame(&time, &strategy);
    time.update_with_instant(base + Duration::from_millis(1));
    capture.frame(&time, &strategy);
    capture.frame(&time, &strategy); // same frame must not gain statistical weight
    time.update_with_instant(base + Duration::from_millis(201));
    capture.frame(&time, &strategy);
    assert!(!capture.active());
    let c = capture.capture.as_ref().unwrap();
    assert_eq!(c.samples.len(), 2);
    let deltas: Vec<_> = c
        .samples
        .iter()
        .map(|s| match s {
            Sample::Frame {
                start_ns,
                end_ns,
                delta_ns,
                time_strategy,
            } => {
                assert_eq!(end_ns - start_ns, *delta_ns);
                assert!(time_strategy.contains("ManualDuration"));
                *delta_ns
            }
            _ => panic!("unexpected sample"),
        })
        .collect();
    assert_eq!(
        deltas,
        [1_000_000, 200_000_000],
        "raw tail cannot be smoothed or clipped"
    );
    assert!(c
        .stop_reason
        .as_ref()
        .unwrap()
        .starts_with("capacity reached"));
    std::fs::write(&path, b"existing evidence").unwrap();
    assert!(capture.export().is_err());
    assert_eq!(std::fs::read(&path).unwrap(), b"existing evidence");
    assert_eq!(capture.capture.as_ref().unwrap().samples.len(), 2);
    // Move this test's colliding file, preserving it, then retry the retained capture.
    std::fs::rename(&path, output.path().join("prior.json")).unwrap();
    capture.export().unwrap();
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(json["samples"][1]["delta_ns"], 200_000_000);
    assert_eq!(json["config"]["metadata"]["warmup"], cfg.metadata["warmup"]);
    capture
        .start(
            config(output.path().join("next.json"), 2),
            serde_json::json!({}),
        )
        .unwrap();
    assert!(capture.capture.as_ref().unwrap().samples.is_empty());
}

#[test]
fn rehearsal_studio_m16_freshness_refuses_scene_resident_and_pending_reset() {
    let output = tempfile::tempdir().unwrap();
    let mut capture = M16Capture::default();
    capture
        .start(
            config(output.path().join("fresh.json"), 100),
            serde_json::json!({}),
        )
        .unwrap();
    capture.publish(4, 7, true);
    capture.consume(Client::Egui, 4, 7, false, true);
    capture.publish(4, 7, true); // repeated paused publication is not a new observation
    capture.consume(Client::Egui, 4, 7, false, true);
    capture.consume(Client::Native, 4, 7, false, true);
    capture.consume(Client::Native, 5, 7, false, true);
    capture.consume(Client::Egui, 4, 7, true, true);
    capture.resident_reset(); // even same source, scene and generation cannot reuse old stamp
    capture.consume(Client::Egui, 4, 7, false, true);
    capture.publish(4, 7, true);
    capture.consume(Client::Egui, 4, 7, false, true);
    capture.publish(4, 7, false);
    capture.consume(Client::Native, 4, 7, false, false);
    let samples = &capture.capture.as_ref().unwrap().samples;
    let publications: Vec<_> = samples
        .iter()
        .filter_map(|s| match s {
            Sample::Publication { stamp } => Some(stamp),
            _ => None,
        })
        .collect();
    assert_eq!(publications.len(), 2);
    assert_ne!(
        publications[0].resident_epoch,
        publications[1].resident_epoch
    );
    let consumed: Vec<_> = samples
        .iter()
        .filter_map(|s| match s {
            Sample::Consumption {
                stamp, consumed_ns, ..
            } => {
                assert!(*consumed_ns >= stamp.published_ns);
                Some(stamp)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        consumed,
        [publications[0], publications[0], publications[1]]
    );
    assert_eq!(
        samples
            .iter()
            .filter(|s| matches!(s, Sample::Rejected { .. }))
            .count(),
        4
    );
}

#[test]
fn rehearsal_studio_m16_native_adapter_and_shared_cpu_scope_use_current_snapshot() {
    use bevy::input::{keyboard::KeyboardInput, mouse::MouseWheel};
    use bevy::window::PrimaryWindow;
    use bevy_egui::{EguiClipboard, EguiContext, PrimaryEguiContext};
    use simthing_mapeditor::rehearsal_studio_native_ui::{NativePrototypePlugin, NativeReadout};
    let output = tempfile::tempdir().unwrap();
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<StudioAppState>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<EguiClipboard>()
        .add_event::<KeyboardInput>()
        .add_event::<MouseWheel>()
        .add_plugins((NativePrototypePlugin, M16CapturePlugin));
    app.world_mut().spawn((Window::default(), PrimaryWindow));
    app.world_mut()
        .spawn((EguiContext::default(), PrimaryEguiContext));
    app.update();
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        state
            .m16
            .start(
                config(output.path().join("cpu.json"), 100),
                serde_json::json!({}),
            )
            .unwrap();
        state.native_ui.set_enabled(true);
        // Explicit synthetic snapshot; real resident/admission is covered by the existing referee.
        state.live_bridge_readout.attached = true;
        state.live_bridge_readout.executed_ticks = 17;
        state.m16.publish(0, 17, true);
        let e = measured_projection(&mut state, Client::Egui);
        let n = measured_projection(&mut state, Client::Native);
        assert_eq!(e, n);
        assert_eq!(e.bridge_executed_ticks, 17);
    }
    app.update();
    let mut texts = app
        .world_mut()
        .query_filtered::<&Text, With<NativeReadout>>();
    assert!(texts
        .iter(app.world())
        .any(|t| t.0.contains("resident generation 17")));
    let state = app.world().resource::<StudioAppState>();
    let samples = &state.m16.capture.as_ref().unwrap().samples;
    for client in [Client::Egui, Client::Native] {
        assert!(samples.iter().any(|s| matches!(s, Sample::Cpu {client: c, scope, start_ns, end_ns}
            if *c == client && scope == "shared_clock_and_observation_projection" && end_ns >= start_ns)));
    }
    assert!(samples.iter().any(|s| matches!(s, Sample::Consumption {client: Client::Native, stamp, ..} if stamp.generation == 17)));
    app.world_mut()
        .resource_mut::<StudioAppState>()
        .live_bridge_reset_requested = true;
    app.update();
    assert!(texts
        .iter(app.world())
        .all(|t| t.0.contains("prior resident observation withheld")));
    let state = app.world().resource::<StudioAppState>();
    assert!(state
        .m16
        .capture
        .as_ref()
        .unwrap()
        .samples
        .iter()
        .any(|s| matches!(
            s,
            Sample::Rejected {
                client: Client::Native,
                ..
            }
        )));
    assert_eq!(
        state.live_bridge_readout.executed_ticks, 17,
        "instrumentation has no tick authority"
    );
}
