//! Leaf A behavior referees: real dispatch, owner validation, resident samples and coexistence.
#![cfg(windows)]

use bevy::input::{
    keyboard::{Key, KeyboardInput},
    mouse::MouseWheel,
    ButtonState,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiClipboard, EguiContext, EguiInput, PrimaryEguiContext};
use simthing_mapeditor::app::StudioAppState;
use simthing_mapeditor::rehearsal_studio_native_ui::{
    activate_native_control, native_observation_text, NativeControl, NativePane,
    NativePrototypePlugin, NativeReadout,
};

fn harness() -> (App, Entity, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<StudioAppState>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<EguiClipboard>()
        .add_event::<KeyboardInput>()
        .add_event::<MouseWheel>()
        .add_plugins(NativePrototypePlugin);
    let window = app
        .world_mut()
        .spawn((Window::default(), PrimaryWindow))
        .id();
    let context = app
        .world_mut()
        .spawn((EguiContext::default(), PrimaryEguiContext))
        .id();
    app.update();
    // Exercise egui's real full-viewport background registration. It must not own
    // empty map pixels just because layer_id_at returns Some(background).
    let mut egui_context = app.world_mut().get_mut::<EguiContext>(context).unwrap();
    let _ = egui_context.get_mut().run(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..default()
        },
        |_| {},
    );
    app.world_mut()
        .resource_mut::<StudioAppState>()
        .native_ui
        .set_enabled(true);
    // Deterministic layout output fixture, fed to the production physical-pixel hit testing.
    let mut panes = app
        .world_mut()
        .query_filtered::<(&mut ComputedNode, &mut GlobalTransform), With<NativePane>>();
    for (mut node, mut transform) in panes.iter_mut(app.world_mut()) {
        node.size = Vec2::new(400.0, 600.0);
        *transform = GlobalTransform::from_translation(Vec3::new(600.0, 400.0, 0.0));
    }
    (app, window, context)
}

fn key(app: &mut App, window: Entity, key_code: KeyCode, text: Option<&str>) {
    app.world_mut().send_event(KeyboardInput {
        key_code,
        logical_key: Key::Unidentified(bevy::input::keyboard::NativeKey::Unidentified),
        state: ButtonState::Pressed,
        text: text.map(Into::into),
        repeat: false,
        window,
    });
}

fn cursor(app: &mut App, window: Entity, position: Vec2) {
    app.world_mut()
        .get_mut::<Window>(window)
        .unwrap()
        .set_physical_cursor_position(Some(position.into()));
}

#[test]
fn rehearsal_studio_native_dispatch_validates_shared_tps_and_consumes_focus_events() {
    let (mut app, window, context) = harness();
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        activate_native_control(&mut state, NativeControl::MaxTps);
    }
    key(&mut app, window, KeyCode::Digit0, Some("0"));
    key(&mut app, window, KeyCode::Enter, None);
    app.world_mut()
        .get_mut::<EguiInput>(context)
        .unwrap()
        .events = vec![egui::Event::Text("0".into())];
    app.update();
    let state = app.world().resource::<StudioAppState>();
    assert!(state.sim_clock_transport.last_error().is_some());
    assert_eq!(state.sim_clock_transport.readout().max_tps, 10.0);
    assert!(state.native_ui.block_map_keyboard);
    assert!(
        app.world()
            .get::<EguiInput>(context)
            .unwrap()
            .events
            .is_empty(),
        "native text must not also reach egui"
    );

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ControlLeft);
    key(&mut app, window, KeyCode::KeyA, None);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    key(&mut app, window, KeyCode::Digit1, Some("12.5"));
    key(&mut app, window, KeyCode::Enter, None);
    app.update();
    assert_eq!(
        app.world()
            .resource::<StudioAppState>()
            .sim_clock_transport
            .readout()
            .max_tps,
        12.5
    );
    // Egui uses this exact owner; its updates immediately feed the native label/draft too.
    app.world_mut()
        .resource_mut::<StudioAppState>()
        .sim_clock_transport
        .apply(simthing_mapeditor::StudioSimClockTransportCommand::SetMaxTps(7.0))
        .unwrap();
    key(&mut app, window, KeyCode::Tab, None);
    app.update();
    assert_eq!(
        app.world().resource::<StudioAppState>().native_ui.focus,
        Some(NativeControl::ApplyTps)
    );
    key(&mut app, window, KeyCode::Escape, None);
    app.update();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_keyboard,
        "Escape is consumed on the release-focus frame"
    );
    app.update();
    assert!(
        !app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_keyboard
    );
    let state = app.world().resource::<StudioAppState>();
    assert_eq!(state.sim_clock_transport.max_tps_draft(), "7");
}

#[test]
fn rehearsal_studio_native_drag_origin_modal_disable_and_dpi_are_exclusive() {
    let (mut app, window, context) = harness();
    cursor(&mut app, window, Vec2::new(600.0, 300.0));
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .press(MouseButton::Right);
    app.world_mut()
        .get_mut::<EguiInput>(context)
        .unwrap()
        .events = vec![egui::Event::PointerButton {
        pos: egui::pos2(600.0, 300.0),
        button: egui::PointerButton::Secondary,
        pressed: true,
        modifiers: default(),
    }];
    app.update();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer
    );
    assert!(!app
        .world()
        .get::<EguiInput>(context)
        .unwrap()
        .events
        .iter()
        .any(|e| matches!(e, egui::Event::PointerButton { .. })));
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .clear();
    cursor(&mut app, window, Vec2::new(100.0, 100.0));
    app.update();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer,
        "native drag retains ownership over map"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .release(MouseButton::Right);
    app.update();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer,
        "release goes to origin"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .clear();
    app.update();
    assert!(
        !app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer
    );

    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .press(MouseButton::Right);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .clear();
    cursor(&mut app, window, Vec2::new(600.0, 300.0));
    app.update();
    assert!(
        !app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer,
        "map drag retains ownership across pane"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .reset_all();
    app.update();

    // A scale change alters logical cursor coordinates, not physical UI ownership.
    app.world_mut()
        .get_mut::<Window>(window)
        .unwrap()
        .resolution
        .set_scale_factor_override(Some(1.5));
    app.update();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer
    );
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        activate_native_control(&mut state, NativeControl::Play);
        let StudioAppState {
            scenario_library,
            sim_clock_transport,
            ..
        } = &mut *state;
        scenario_library.open(sim_clock_transport);
    }
    key(&mut app, window, KeyCode::Enter, None);
    app.update();
    let state = app.world().resource::<StudioAppState>();
    assert!(state.sim_clock_transport.readout().paused);
    assert!(state.native_ui.focus.is_none());
    assert!(state.native_ui.block_map_pointer && state.native_ui.block_map_keyboard);
    let mut panes = app.world_mut().query_filtered::<&Node, With<NativePane>>();
    assert!(panes
        .iter(app.world())
        .all(|node| node.display == Display::None));
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        state.scenario_library.close();
        state.native_ui.set_enabled(false);
    }
    app.update();
    assert!(
        !app.world()
            .resource::<StudioAppState>()
            .native_ui
            .block_map_pointer
    );
    app.world_mut()
        .resource_mut::<StudioAppState>()
        .sim_clock_transport
        .apply(simthing_mapeditor::StudioSimClockTransportCommand::Play)
        .unwrap();
    assert!(
        app.world()
            .resource::<StudioAppState>()
            .sim_clock_transport
            .readout()
            .playing,
        "egui fallback owner remains functional"
    );
}

#[test]
fn rehearsal_studio_native_meridian_readout_tracks_resident_and_current_selection() {
    use simthing_mapeditor::clause_scenario_picker::{
        run_clause_picker_action_staged, ClausePickerActionResult, ClausePickerSelection,
    };
    let (mut app, window, _) = harness();
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
        scenario_json_path: Some(output.path().join("native-ui.simthing-scenario.json")),
        ..default()
    };
    let result =
        std::thread::spawn(move || run_clause_picker_action_staged(&selection, None, &mut |_| {}))
            .join()
            .unwrap();
    let ClausePickerActionResult::Loaded { session, .. } = result else {
        panic!("load failed: {result:?}");
    };
    let mut bridge = simthing_mapeditor::StudioLiveSessionBridge::default();
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        let token = state.scenario_library.begin_load_attempt();
        state
            .try_adopt_clause_load_attempt(
                token,
                session,
                &mut Default::default(),
                &mut bridge,
                "native proof".into(),
            )
            .unwrap();
        state.scenario_library.finish_load_attempt(token);
        state.scenario_library.close();
        assert_eq!(state.session.as_ref().unwrap().view_model.stars.len(), 7);
        assert_eq!(
            state
                .scenario_library
                .source_to_ready
                .as_ref()
                .unwrap()
                .profile_identity
                .as_deref(),
            Some("fnv1a64:bfcbc44323b304bf:23530")
        );
    }
    app.update();
    let execution_identity = bridge.sim_session().unwrap().persisted_execution_identity();
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        activate_native_control(&mut state, NativeControl::Next);
        activate_native_control(&mut state, NativeControl::Row(2));
        let selected = state.session.as_ref().unwrap().view_model.stars[5].system_id;
        assert_eq!(state.selection.selected_system_id, Some(selected));
    }
    key(&mut app, window, KeyCode::ArrowDown, None);
    app.update();
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        let selected = state.session.as_ref().unwrap().view_model.stars[6].system_id;
        assert_eq!(state.selection.selected_system_id, Some(selected));
        activate_native_control(&mut state, NativeControl::Play);
        let StudioAppState {
            sim_clock_transport,
            session,
            ..
        } = &mut *state;
        bridge
            .tick_from_clock(sim_clock_transport.clock_mut(), session.as_ref(), 0.1)
            .unwrap();
    }
    let ticks = bridge.executed_ticks();
    assert!(ticks > 0);
    {
        let mut state = app.world_mut().resource_mut::<StudioAppState>();
        state.live_bridge_readout = bridge.readout();
        activate_native_control(&mut state, NativeControl::Pause);
        let text = native_observation_text(&state);
        for sample in &state.live_bridge_readout.field_accretion_samples {
            assert!(text.contains(&sample.property_key));
        }
        assert!(text.contains(&format!("resident generation {ticks}")));
        eprintln!("NATIVE_RESIDENT_PROOF\n{text}");
        state.scene_render_revision += 1;
        state.selection.clear();
        activate_native_control(&mut state, NativeControl::Row(0));
        assert!(
            state.selection.selected_system_id.is_none(),
            "stale rendered rows cannot select across replacement"
        );
    }
    app.update();
    let text = native_observation_text(app.world().resource::<StudioAppState>());
    let mut readouts = app
        .world_mut()
        .query_filtered::<&Text, With<NativeReadout>>();
    assert!(
        readouts
            .iter(app.world())
            .all(|rendered| rendered.0 == text),
        "real Bevy text receives the shared snapshot"
    );
    app.update();
    assert_eq!(
        bridge.executed_ticks(),
        ticks,
        "presentation never advances the resident"
    );
    assert_eq!(
        bridge.sim_session().unwrap().persisted_execution_identity(),
        execution_identity
    );
    assert_eq!(paths.each_ref().map(|p| std::fs::read(p).unwrap()), before);
}
