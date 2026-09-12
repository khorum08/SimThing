//! 0088-UI-PROTOTYPE-0 Leaf A. Disposable native presentation of existing Studio owners.
//!
//! Deletion endpoint if migration is declined: remove this module, its plugin/state/toggle
//! mounts and the native input gates in app/{camera,picking}. No persisted prototype state,
//! session, observer, camera, render resource, or dependency is introduced.
#![cfg(windows)]

use bevy::input::{keyboard::KeyboardInput, mouse::MouseWheel, ButtonState};
use bevy::prelude::*;
use bevy::ui::{CalculatedClip, UiSystem};
use bevy::window::PrimaryWindow;
use bevy_egui::{
    egui, EguiClipboard, EguiContext, EguiInput, EguiPreUpdateSet, PrimaryEguiContext,
};

use crate::app::StudioAppState;
use crate::StudioSimClockTransportCommand as ClockCommand;

const ROWS: usize = 3;

/// Widget identities only; operations still belong to StudioAppState's existing owners.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeControl {
    Pause,
    Play,
    Rate1,
    Rate2,
    Rate4,
    MaxTps,
    ApplyTps,
    Previous,
    Next,
    Row(usize),
    Disable,
}

const TAB_ORDER: [NativeControl; 13] = [
    NativeControl::Pause,
    NativeControl::Play,
    NativeControl::Rate1,
    NativeControl::Rate2,
    NativeControl::Rate4,
    NativeControl::MaxTps,
    NativeControl::ApplyTps,
    NativeControl::Previous,
    NativeControl::Next,
    NativeControl::Row(0),
    NativeControl::Row(1),
    NativeControl::Row(2),
    NativeControl::Disable,
];

#[derive(Debug, Default, Clone)]
pub struct NativePrototypeState {
    pub enabled: bool,
    pub focus: Option<NativeControl>,
    pub first_row: usize,
    // This is caret/selection presentation, not another numeric draft.
    caret: usize,
    select_all: bool,
    capture: Option<bool>, // Some(true) = native; Some(false) = external drag origin.
    keyboard_capture: bool,
    pub block_map_pointer: bool,
    pub block_map_keyboard: bool,
    displayed_revision: u64,
}

impl NativePrototypeState {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.focus = None;
        self.select_all = false;
        // Keep a drag's owner through its release, even when Disable was clicked.
    }
}

#[derive(Component)]
pub struct NativePane;
#[derive(Component)]
pub struct NativeReadout;
#[derive(Component)]
struct ControlText(NativeControl);
#[derive(Component)]
struct SystemList;

pub struct NativePrototypePlugin;

impl Plugin for NativePrototypePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_native_pane)
            .add_systems(
                PreUpdate,
                route_native_input
                    .after(UiSystem::Focus)
                    .after(EguiPreUpdateSet::ProcessInput)
                    .before(EguiPreUpdateSet::BeginPass),
            )
            .add_systems(
                Update,
                sync_native_pane.after(crate::app::live_session_bridge_system),
            );
    }
}

fn label(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.94, 1.0)),
        Node {
            flex_shrink: 0.0,
            ..default()
        },
    )
}

fn spawn_native_pane(mut commands: Commands) {
    // Bevy UI selects the existing primary Camera3d. Never mount another camera.
    commands.spawn((
        NativePane,
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            left: Val::Percent(30.0), right: Val::Percent(30.0),
            top: Val::Px(52.0), bottom: Val::Px(24.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ScrollPosition::default(),
        GlobalZIndex(100),
        BackgroundColor(Color::srgb(0.025, 0.04, 0.075)),
    )).with_children(|pane| {
        pane.spawn(label("Native prototype | Simulation + observation"));
        pane.spawn(label("F8: toggle | Tab: focus | Esc: release focus\nWheel: scroll pane / systems | egui dialogs take priority"));
        for controls in [
            vec![NativeControl::Pause, NativeControl::Play, NativeControl::Rate1, NativeControl::Rate2, NativeControl::Rate4],
            vec![NativeControl::MaxTps, NativeControl::ApplyTps],
            vec![NativeControl::Previous, NativeControl::Next],
        ] {
            pane.spawn(Node { flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, column_gap: Val::Px(6.0), row_gap: Val::Px(4.0), flex_shrink: 0.0, ..default() })
                .with_children(|row| {
                    for control in controls { spawn_button(row, control); }
                });
        }
        pane.spawn((SystemList, Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(4.0), flex_shrink: 0.0, ..default() }))
            .with_children(|list| {
                for row in 0..ROWS { spawn_button(list, NativeControl::Row(row)); }
            });
        pane.spawn((NativeReadout, label("No resident observation yet")));
        spawn_button(pane, NativeControl::Disable);
    });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, control: NativeControl) {
    parent
        .spawn((
            Button,
            control,
            Node {
                padding: UiRect::axes(Val::Px(9.0), Val::Px(6.0)),
                min_height: Val::Px(32.0),
                flex_shrink: 0.0,
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.18, 0.26)),
        ))
        .with_children(|button| {
            button.spawn((ControlText(control), label("")));
        });
}

pub fn native_suspended(state: &StudioAppState) -> bool {
    state.scenario_library.visible
        || state.settings_dialog.visible
        || state.telemetry_dialog.visible
        || state.scenario_library.studio_ops_telemetry_visible
        || state.generation_name_dialog_visible
        || state.loading_cover_active
        || state.performance_diagnostic_hide_panels
}

fn system_count(state: &StudioAppState) -> usize {
    state
        .session
        .as_ref()
        .map_or(0, |s| s.view_model.stars.len())
}

fn scroll_rows(state: &mut StudioAppState, delta: isize) {
    state.native_ui.first_row = state
        .native_ui
        .first_row
        .saturating_add_signed(delta)
        .min(system_count(state).saturating_sub(ROWS));
}

/// Called synchronously from input dispatch; no queued command can cross a scene replacement.
pub fn activate_native_control(state: &mut StudioAppState, control: NativeControl) {
    if !state.native_ui.enabled || native_suspended(state) {
        return;
    }
    if state.native_ui.focus == Some(NativeControl::MaxTps)
        && !matches!(control, NativeControl::MaxTps | NativeControl::ApplyTps)
    {
        let _ = state.sim_clock_transport.apply_max_tps_draft();
    }
    state.native_ui.focus = Some(control);
    let command = match control {
        NativeControl::Pause => Some(ClockCommand::Pause),
        NativeControl::Play => Some(ClockCommand::Play),
        NativeControl::Rate1 => Some(ClockCommand::Rate1x),
        NativeControl::Rate2 => Some(ClockCommand::Rate2x),
        NativeControl::Rate4 => Some(ClockCommand::Rate4x),
        NativeControl::MaxTps => {
            state.native_ui.caret = state.sim_clock_transport.max_tps_draft().chars().count();
            state.native_ui.select_all = true;
            None
        }
        NativeControl::ApplyTps => {
            let _ = state.sim_clock_transport.apply_max_tps_draft();
            None
        }
        NativeControl::Previous => {
            scroll_rows(state, -(ROWS as isize));
            None
        }
        NativeControl::Next => {
            scroll_rows(state, ROWS as isize);
            None
        }
        NativeControl::Row(row) => {
            // A rendered row from an old generation must never select a different current system.
            if state.native_ui.displayed_revision == state.scene_render_revision {
                if let Some(star) = state
                    .session
                    .as_ref()
                    .and_then(|s| s.view_model.stars.get(state.native_ui.first_row + row))
                {
                    crate::selection::apply_star_click(&mut state.selection, star.system_id);
                }
            }
            None
        }
        NativeControl::Disable => {
            state.native_ui.set_enabled(false);
            None
        }
    };
    if let Some(command) = command {
        let _ = state.sim_clock_transport.apply(command);
    }
}

fn edit_text(state: &mut StudioAppState, insert: &str, backspace: bool, delete: bool) {
    let mut text: Vec<char> = state.sim_clock_transport.max_tps_draft().chars().collect();
    let ui = &mut state.native_ui;
    ui.caret = ui.caret.min(text.len());
    if ui.select_all {
        text.clear();
        ui.caret = 0;
        ui.select_all = false;
    } else if backspace && ui.caret > 0 {
        ui.caret -= 1;
        text.remove(ui.caret);
    } else if delete && ui.caret < text.len() {
        text.remove(ui.caret);
    }
    // Only single-line presentation filtering. Numeric validation remains the clock owner's job.
    for ch in insert.chars().filter(|ch| !ch.is_control()) {
        text.insert(ui.caret, ch);
        ui.caret += 1;
    }
    *state.sim_clock_transport.max_tps_draft_mut() = text.into_iter().collect();
}

fn handle_key(
    state: &mut StudioAppState,
    event: &KeyboardInput,
    ctrl: bool,
    shift: bool,
    clipboard: &mut EguiClipboard,
) {
    if event.state != ButtonState::Pressed {
        return;
    }
    let focus = state.native_ui.focus;
    match event.key_code {
        KeyCode::Escape => {
            if focus == Some(NativeControl::MaxTps) {
                let _ = state.sim_clock_transport.apply_max_tps_draft();
            }
            state.native_ui.focus = None;
        }
        KeyCode::Tab => {
            if focus == Some(NativeControl::MaxTps) {
                let _ = state.sim_clock_transport.apply_max_tps_draft();
            }
            let index = focus.and_then(|f| TAB_ORDER.iter().position(|c| *c == f));
            let next = if shift {
                index.map_or(TAB_ORDER.len() - 1, |i| {
                    (i + TAB_ORDER.len() - 1) % TAB_ORDER.len()
                })
            } else {
                index.map_or(0, |i| (i + 1) % TAB_ORDER.len())
            };
            state.native_ui.focus = Some(TAB_ORDER[next]);
            if TAB_ORDER[next] == NativeControl::MaxTps {
                state.native_ui.select_all = true;
            }
        }
        KeyCode::Enter | KeyCode::Space if focus != Some(NativeControl::MaxTps) => {
            if !event.repeat {
                if let Some(control) = focus {
                    activate_native_control(state, control);
                }
            }
        }
        KeyCode::Enter => {
            let _ = state.sim_clock_transport.apply_max_tps_draft();
        }
        KeyCode::ArrowDown | KeyCode::ArrowUp if matches!(focus, Some(NativeControl::Row(_))) => {
            let Some(NativeControl::Row(row)) = focus else {
                return;
            };
            let index = state.native_ui.first_row + row;
            let next = if event.key_code == KeyCode::ArrowDown {
                index
                    .saturating_add(1)
                    .min(system_count(state).saturating_sub(1))
            } else {
                index.saturating_sub(1)
            };
            if next < state.native_ui.first_row {
                state.native_ui.first_row = next;
            }
            if next >= state.native_ui.first_row + ROWS {
                state.native_ui.first_row = next + 1 - ROWS;
            }
            activate_native_control(state, NativeControl::Row(next - state.native_ui.first_row));
        }
        _ if focus == Some(NativeControl::MaxTps) => match event.key_code {
            KeyCode::KeyA if ctrl => state.native_ui.select_all = true,
            KeyCode::KeyC | KeyCode::KeyX if ctrl => {
                if state.native_ui.select_all {
                    clipboard.set_text(state.sim_clock_transport.max_tps_draft());
                    if event.key_code == KeyCode::KeyX {
                        edit_text(state, "", false, false);
                    }
                }
            }
            KeyCode::KeyV if ctrl => {
                if let Some(text) = clipboard.get_text() {
                    edit_text(state, &text, false, false);
                }
            }
            KeyCode::Backspace => edit_text(state, "", true, false),
            KeyCode::Delete => edit_text(state, "", false, true),
            KeyCode::Home => {
                state.native_ui.caret = 0;
                state.native_ui.select_all = false;
            }
            KeyCode::End => {
                state.native_ui.caret = state.sim_clock_transport.max_tps_draft().chars().count();
                state.native_ui.select_all = false;
            }
            KeyCode::ArrowLeft => {
                state.native_ui.caret = state.native_ui.caret.saturating_sub(1);
                state.native_ui.select_all = false;
            }
            KeyCode::ArrowRight => {
                state.native_ui.caret = (state.native_ui.caret + 1)
                    .min(state.sim_clock_transport.max_tps_draft().chars().count());
                state.native_ui.select_all = false;
            }
            _ if !ctrl => {
                if let Some(text) = &event.text {
                    edit_text(state, text, false, false);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

// UI layout and cursor are both physical pixels, including the Windows scale factor.
fn hit(
    node: &ComputedNode,
    transform: &GlobalTransform,
    clip: Option<&CalculatedClip>,
    cursor: Option<Vec2>,
) -> bool {
    let rect = Rect::from_center_size(transform.translation().truncate(), node.size());
    let rect = clip.map_or(rect, |clip| rect.intersect(clip.clip));
    node.size().cmpgt(Vec2::ZERO).all() && cursor.is_some_and(|p| rect.contains(p))
}

/// One per-frame input decision, before egui begins its pass and before map Update systems.
fn route_native_input(
    mut state: ResMut<StudioAppState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keys: EventReader<KeyboardInput>,
    mut wheels: EventReader<MouseWheel>,
    mut contexts: Query<(&mut EguiContext, &mut EguiInput), With<PrimaryEguiContext>>,
    mut clipboard: ResMut<EguiClipboard>,
    dialog: Option<Res<crate::app::resources::StudioDialog>>,
    pane: Query<(&ComputedNode, &GlobalTransform), With<NativePane>>,
    controls: Query<(
        &NativeControl,
        &ComputedNode,
        &GlobalTransform,
        Option<&CalculatedClip>,
    )>,
    list: Query<(&ComputedNode, &GlobalTransform, Option<&CalculatedClip>), With<SystemList>>,
    mut scroll: Query<(&ComputedNode, &mut ScrollPosition), With<NativePane>>,
) {
    let Ok(window) = windows.single() else {
        keys.clear();
        wheels.clear();
        return;
    };
    let cursor = window.physical_cursor_position();
    let mut egui_pointer = false;
    let mut egui_keyboard = false;
    for (mut context, _) in &mut contexts {
        let ctx = context.get_mut();
        // Egui registers a full-viewport background even over the map. Only actual
        // floating areas/windows preempt the native pane (Studio's panels are Areas).
        egui_pointer |= window.cursor_position().is_some_and(|p| {
            ctx.layer_id_at(egui::pos2(p.x, p.y))
                .is_some_and(|layer| layer.order != egui::Order::Background)
        });
        egui_keyboard |= ctx.wants_keyboard_input();
    }
    let suspended =
        native_suspended(&state) || dialog.as_ref().is_some_and(|d| d.visible) || !window.focused;
    if suspended {
        state.native_ui.focus = None;
        state.native_ui.capture = None;
        state.native_ui.keyboard_capture = false;
    }
    let enabled = state.native_ui.enabled && !suspended;
    let inside = enabled && !egui_pointer && pane.iter().any(|(n, t)| hit(n, t, None, cursor));
    if mouse.get_just_pressed().next().is_some() && state.native_ui.capture.is_none() {
        state.native_ui.capture = Some(inside);
    }
    let owns_pointer = state.native_ui.capture.unwrap_or(inside) && !suspended;
    let mut owns_keyboard = enabled && state.native_ui.focus.is_some();
    if mouse.get_just_pressed().next().is_some() && !owns_pointer {
        if state.native_ui.focus == Some(NativeControl::MaxTps) {
            let _ = state.sim_clock_transport.apply_max_tps_draft();
        }
        state.native_ui.focus = None;
        owns_keyboard = false;
    }
    if enabled && owns_pointer && mouse.just_pressed(MouseButton::Left) {
        if let Some(control) = controls
            .iter()
            .find_map(|(c, n, t, clip)| hit(n, t, clip, cursor).then_some(*c))
        {
            activate_native_control(&mut state, control);
        } else {
            state.native_ui.focus = Some(NativeControl::Pause);
        }
        owns_keyboard = true;
    }
    let ctrl = keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if owns_keyboard && keyboard.get_pressed().next().is_some() {
        state.native_ui.keyboard_capture = true;
    }
    owns_keyboard |= state.native_ui.keyboard_capture;
    let mut toggled = false;
    for key in keys.read() {
        if key.key_code == KeyCode::F8
            && key.state == ButtonState::Pressed
            && !key.repeat
            && !suspended
            && !egui_keyboard
        {
            let enabled = !state.native_ui.enabled;
            state.native_ui.set_enabled(enabled);
            toggled = true;
        } else if owns_keyboard && state.native_ui.enabled {
            handle_key(&mut state, key, ctrl, shift, &mut clipboard);
        }
    }
    for wheel in wheels.read() {
        if !enabled || !owns_pointer {
            continue;
        }
        if list.iter().any(|(n, t, clip)| hit(n, t, clip, cursor)) {
            if wheel.y != 0.0 {
                scroll_rows(&mut state, if wheel.y > 0.0 { -1 } else { 1 });
            }
        } else {
            for (node, mut scroll) in &mut scroll {
                let delta = if wheel.unit == bevy::input::mouse::MouseScrollUnit::Line {
                    wheel.y * 32.0
                } else {
                    wheel.y
                };
                let max = ((node.content_size().y - node.size().y) * node.inverse_scale_factor())
                    .max(0.0);
                scroll.offset_y = (scroll.offset_y - delta).clamp(0.0, max);
            }
        }
    }
    state.native_ui.block_map_pointer = owns_pointer || egui_pointer || suspended;
    state.native_ui.block_map_keyboard = owns_keyboard || egui_keyboard || suspended || toggled;
    // The release belongs to the drag origin. Hand off only on the following frame.
    if mouse.get_pressed().next().is_none() {
        state.native_ui.capture = None;
    }
    if keyboard.get_pressed().next().is_none() {
        state.native_ui.keyboard_capture = false;
    }
    for (mut context, mut input) in &mut contexts {
        if owns_keyboard {
            context.get_mut().memory_mut(|memory| {
                if let Some(id) = memory.focused() {
                    memory.surrender_focus(id);
                }
            });
        }
        input.events.retain(|event| {
            !native_consumes_egui_event(event, owns_pointer, owns_keyboard, toggled)
        });
        if owns_pointer {
            input.events.push(egui::Event::PointerGone);
        }
    }
}

pub fn native_consumes_egui_event(
    event: &egui::Event,
    pointer: bool,
    keyboard: bool,
    toggle: bool,
) -> bool {
    match event {
        egui::Event::PointerMoved(_)
        | egui::Event::PointerButton { .. }
        | egui::Event::MouseMoved(_)
        | egui::Event::MouseWheel { .. }
        | egui::Event::Touch { .. }
        | egui::Event::Zoom(_) => pointer,
        egui::Event::Key {
            key: egui::Key::F8, ..
        } if toggle => true,
        // Release keys that egui may have acquired before focus moved to native.
        egui::Event::Key { pressed: false, .. } => false,
        egui::Event::Key { .. }
        | egui::Event::Text(_)
        | egui::Event::Copy
        | egui::Event::Cut
        | egui::Event::Paste(_)
        | egui::Event::Ime(_) => keyboard,
        _ => false,
    }
}

/// Reads precisely the same snapshots as egui; never opens, advances or subscribes a session.
pub fn native_observation_text(state: &StudioAppState) -> String {
    if state.live_bridge_reset_requested {
        return "Session replacement pending; prior resident observation withheld".into();
    }
    let clock = state.sim_clock_transport.readout();
    let bridge = &state.live_bridge_readout;
    let obs = crate::build_studio_live_observation_readout(&clock, bridge, state.session.as_ref());
    let mut text = format!(
        "{} | {} | max {:.3} TPS | effective {:.3}/s\nScheduled tick {} | resident generation {}\nBridge: {} | {}\nScenario: {}\nSystems: {} | selected: {:?}\n",
        if obs.clock_paused { "Paused" } else { "Playing" }, obs.clock_rate_label.replace("×", "x"),
        obs.max_tps, obs.effective_tps, obs.scheduled_tick_index, obs.bridge_executed_ticks,
        obs.bridge_status_label, bridge.session_path_label,
        obs.scenario_id.as_deref().unwrap_or("(none)"), system_count(state), state.selection.selected_system_id,
    );
    if state.live_bridge_reset_requested {
        text.push_str("Session replacement pending; prior observation withheld\n");
    } else {
        let mut latest = std::collections::BTreeMap::new();
        for sample in &bridge.field_accretion_samples {
            latest.insert(&sample.property_key, sample);
        }
        if latest.is_empty() {
            text.push_str("No resident property samples yet\n");
        }
        for (key, sample) in latest {
            text.push_str(&format!(
                "{key}\n  generation {}: {:.3}\n",
                sample.tick_index, sample.amount
            ));
        }
    }
    if let Some(error) = state.sim_clock_transport.last_error() {
        text.push_str(&format!("TPS refused: {error}\n"));
    }
    if let Some(error) = &bridge.last_error {
        text.push_str(&format!("Bridge: {error}\n"));
    }
    text
}

fn control_label(state: &StudioAppState, control: NativeControl) -> String {
    let focused = state.native_ui.focus == Some(control);
    let label = match control {
        NativeControl::Pause => "Pause".into(),
        NativeControl::Play => "Play".into(),
        NativeControl::Rate1 => "1x".into(),
        NativeControl::Rate2 => "2x".into(),
        NativeControl::Rate4 => "4x".into(),
        NativeControl::MaxTps => {
            let mut draft: Vec<_> = state.sim_clock_transport.max_tps_draft().chars().collect();
            if focused && !state.native_ui.select_all {
                draft.insert(state.native_ui.caret.min(draft.len()), '|');
            }
            format!(
                "Max TPS: {}{}",
                draft.into_iter().collect::<String>(),
                if focused && state.native_ui.select_all {
                    " [all]"
                } else {
                    ""
                }
            )
        }
        NativeControl::ApplyTps => "Apply TPS".into(),
        NativeControl::Previous => "Systems: previous".into(),
        NativeControl::Next => "next".into(),
        NativeControl::Row(row) => state
            .session
            .as_ref()
            .and_then(|s| s.view_model.stars.get(state.native_ui.first_row + row))
            .map_or_else(
                || "(no system)".into(),
                |s| {
                    format!(
                        "{} {} | {}",
                        if state.selection.selected_system_id == Some(s.system_id) {
                            "*"
                        } else {
                            " "
                        },
                        s.system_id,
                        s.display_name
                    )
                },
            ),
        NativeControl::Disable => "Disable native prototype (F8)".into(),
    };
    format!("{}{label}", if focused { "> " } else { "" })
}

fn sync_native_pane(
    mut state: ResMut<StudioAppState>,
    dialog: Option<Res<crate::app::resources::StudioDialog>>,
    mut pane: Query<&mut Node, With<NativePane>>,
    mut labels: Query<(&ControlText, &mut Text), Without<NativeReadout>>,
    mut readouts: Query<&mut Text, With<NativeReadout>>,
    mut buttons: Query<(&NativeControl, &Interaction, &mut BackgroundColor)>,
) {
    let visible = state.native_ui.enabled
        && !native_suspended(&state)
        && !dialog.as_ref().is_some_and(|d| d.visible);
    for mut node in &mut pane {
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }
    if !visible {
        return;
    }
    if state.native_ui.displayed_revision != state.scene_render_revision {
        state.native_ui.first_row = 0;
        state.native_ui.focus = None;
        state.native_ui.displayed_revision = state.scene_render_revision;
    }
    state.native_ui.first_row = state
        .native_ui
        .first_row
        .min(system_count(&state).saturating_sub(ROWS));
    for (control, mut text) in &mut labels {
        text.0 = control_label(&state, control.0);
    }
    let observation = native_observation_text(&state);
    for mut text in &mut readouts {
        if text.0 != observation {
            text.0.clone_from(&observation);
        }
    }
    for (control, interaction, mut color) in &mut buttons {
        color.0 = if state.native_ui.focus == Some(*control) {
            Color::srgb(0.13, 0.34, 0.48)
        } else if *interaction == Interaction::Hovered {
            Color::srgb(0.18, 0.25, 0.34)
        } else {
            Color::srgb(0.12, 0.18, 0.26)
        };
    }
}
