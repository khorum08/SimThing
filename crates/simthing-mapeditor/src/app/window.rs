#![cfg(windows)]

use bevy::prelude::*;
use bevy::window::{MonitorSelection, VideoModeSelection, WindowMode, WindowResolution};

use super::camera::StudioCamera;
use super::resources::StudioSettings;
use super::StudioAppState;
use crate::settings::WindowModeSetting;

pub fn primary_window_from_settings(settings: &crate::settings::EditorSettings) -> Window {
    let (w, h) = (settings.last_window_size[0], settings.last_window_size[1]);
    Window {
        title: "SimThing Studio".into(),
        resolution: WindowResolution::new(w as f32, h as f32),
        decorations: false,
        resizable: true,
        ..default()
    }
}

pub fn apply_initial_window_mode(
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    settings: Res<StudioSettings>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };
    apply_mode(&mut window, settings.window_mode);
}

pub fn apply_mode(window: &mut Window, mode: WindowModeSetting) {
    match mode {
        WindowModeSetting::Windowed => {
            window.mode = WindowMode::Windowed;
        }
        WindowModeSetting::BorderlessFullscreen => {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
        }
        WindowModeSetting::ExclusiveFullscreen => {
            window.mode =
                WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current);
        }
    }
}

pub fn set_window_mode(
    windows: &mut Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    settings: &mut crate::settings::EditorSettings,
    mode: WindowModeSetting,
) {
    settings.window_mode = mode;
    if let Ok(mut window) = windows.single_mut() {
        apply_mode(&mut window, mode);
    }
}

pub fn minimize_window(windows: &mut Query<&mut Window, With<bevy::window::PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.set_minimized(true);
    }
}

pub fn persist_settings_on_exit(
    mut exit_events: EventReader<AppExit>,
    settings: Res<StudioSettings>,
    state: Res<StudioAppState>,
    camera: Res<StudioCamera>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if exit_events.read().next().is_none() {
        return;
    }
    let mut copy = settings.0.clone();
    copy.last_generation_params = state.profile.clone();
    copy.left_panel_collapsed = state.left_panel_collapsed;
    copy.last_panel_width = state.left_panel_width_frac;
    copy.last_selected_system_id = state.selection.selected_system_id;
    copy.last_camera = camera.to_persisted();
    copy.set_star_falloff_settings(state.star_falloff_settings);
    copy.set_star_render_mode(state.star_render_mode);
    copy.settings_dialog_position = state.settings_dialog.position;
    copy.settings_dialog_visible = state.settings_dialog.visible;
    if let Ok(window) = windows.single() {
        copy.last_window_size = [
            window.resolution.width() as u32,
            window.resolution.height() as u32,
        ];
    }
    let _ = copy.save();
}
