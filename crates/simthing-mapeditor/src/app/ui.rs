#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::view::window::screenshot::{save_to_disk, Screenshot};
use bevy_egui::{egui, EguiContexts};

use crate::dialog::{
    inactive_control_warning, unimplemented_action_response, StudioAction, WarningDialogModel,
};
use crate::generation::{run_generation, GenerationPreset, GenerationProfile};
use crate::hyperlane_buckets::{
    apply_hyperlane_render_settings_to_meta, hyperlane_visuals_dirty_after_settings_change,
    HyperlaneRenderSettings,
};
use crate::panel_layout::{
    clamp_dialog_rect_away_from_panels, compute_collapsed_panel_tab, compute_floating_panel_layout,
    left_panel_content_scroll_height, left_panel_title, rect_from_xywh, right_panel_rect,
    should_auto_collapse_panel, FloatingDialogBounds,
};
use crate::selection::selected_system_details;
use crate::settings::WindowModeSetting;
use crate::shape_params::spiral_arm_params_active;
use crate::star_render::{
    apply_star_falloff_settings_to_meta, apply_star_render_mode_to_meta,
    star_visuals_dirty_after_settings_change, StarFalloffSettings, StarNameplateSettings,
    StarRenderMode,
};

use super::camera::{reset_camera_after_generation, snap_overhead, StudioCamera};
use super::galaxy_render::{
    mark_hyperlane_render_dirty, mark_star_visual_render_dirty, StarVisualAssets,
};
use super::scenario_io::{
    load_scenario_manual_path_action, open_native_scenario_load_picker, save_scenario_action,
    ScenarioActionResult, ScenarioPickerActionResult,
};
use super::window::{minimize_window, set_window_mode};
use super::{adopt_loaded_scenario_session, adopt_session, GalaxySceneRoot, StudioAppState};
use crate::scenario_runtime_saveload_ui::{
    reopen_candidate_scenario_for_studio_session, save_candidate_scenario_for_studio_create_new,
};
use crate::session::StudioSession;
use crate::studio_frame_phase_gpu_telemetry::{
    apply_diagnostic_minimal_render, capture_normal_render_snapshot,
    restore_normal_render_from_snapshot, PerformanceDiagnosticFlags,
    DIAGNOSTIC_MINIMAL_RENDER_BUTTON, RESTORE_NORMAL_RENDER_BUTTON,
};
use crate::studio_performance_telemetry::performance_settings_section_lines;
use crate::studio_render_loop_dirty_gate::StudioRenderLoopCaches;
use crate::studio_screenshot::next_screenshot_filename;

use super::performance_telemetry::{record_egui_pass_timing, StudioPerformanceTelemetryState};

const SETTINGS_DIALOG_SIZE: egui::Vec2 = egui::vec2(420.0, 720.0);
const TELEMETRY_DIALOG_SIZE: egui::Vec2 = egui::vec2(420.0, 760.0);
const SETTINGS_TITLE_CLOSE_DRAG_GAP: f32 = 6.0;
const TELEMETRY_BUTTON_LABEL: &str = "Telemetry";
const TELEMETRY_TOOLTIP: &str = "Performance Telemetry";
const SETTINGS_BUTTON_LABEL: &str = "⚙";
const SETTINGS_TOOLTIP: &str = "Settings";
/// Top-right window controls, left-to-right (Telemetry immediately before Settings gear).
#[cfg_attr(not(test), allow(dead_code))]
pub const WINDOW_CONTROLS_LEFT_TO_RIGHT: &[&str] = &[
    TELEMETRY_BUTTON_LABEL,
    SETTINGS_BUTTON_LABEL,
    "—",
    "▢",
    "⛶",
    "✕",
];
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SettingsTitleBarRects {
    pub title_rect: egui::Rect,
    pub close_rect: egui::Rect,
    pub drag_rect: egui::Rect,
}

pub fn settings_title_bar_drag_rect(
    title_rect: egui::Rect,
    close_rect: egui::Rect,
    padding: f32,
) -> egui::Rect {
    let max_x = (close_rect.min.x - padding).clamp(title_rect.min.x, title_rect.max.x);
    egui::Rect::from_min_max(title_rect.min, egui::pos2(max_x, title_rect.max.y))
}

pub fn panel_opacity_system(mut state: ResMut<StudioAppState>, time: Res<Time>) {
    let target = if state.left_panel_hovered || state.left_panel_target_opacity > 0.55 {
        0.80
    } else {
        0.50
    };
    state.left_panel_target_opacity = target;
    let speed = 4.0;
    state.left_panel_opacity += (target - state.left_panel_opacity) * speed * time.delta_secs();
}

pub fn studio_ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<StudioAppState>,
    mut settings: ResMut<super::resources::StudioSettings>,
    mut dialog: ResMut<super::resources::StudioDialog>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    mut exit: EventWriter<AppExit>,
    mut camera: ResMut<StudioCamera>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_root: ResMut<GalaxySceneRoot>,
    assets: Res<StarVisualAssets>,
    mut perf_telemetry: ResMut<StudioPerformanceTelemetryState>,
    mut render_caches: ResMut<StudioRenderLoopCaches>,
    mut ctx_unavailable_logged: Local<bool>,
) {
    let egui_started = std::time::Instant::now();
    let mut left_panel_ms = 0.0f64;
    let mut right_panel_ms = 0.0f64;

    let Ok(ctx) = contexts.ctx_mut() else {
        if !*ctx_unavailable_logged {
            bevy::log::warn!("studio egui primary context unavailable; UI pass skipped");
            *ctx_unavailable_logged = true;
        }
        return;
    };
    let screen = ctx.screen_rect();
    let screen_w = screen.width();
    let screen_h = screen.height();

    if should_auto_collapse_panel(screen_w) {
        state.left_panel_collapsed = true;
    }

    if !state.performance_diagnostic_hide_panels {
        draw_window_controls(ctx, &mut state, &mut settings, &mut windows, &mut exit);
        if !state.left_panel_collapsed {
            let panel_started = std::time::Instant::now();
            draw_left_panel(
                ctx,
                &mut state,
                &mut dialog,
                &mut camera,
                screen_w,
                screen_h,
            );
            left_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        } else {
            let panel_started = std::time::Instant::now();
            draw_collapsed_tab(ctx, &mut state, screen_w, screen_h);
            left_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        }
        if state.session.is_some() {
            let panel_started = std::time::Instant::now();
            draw_right_panel(ctx, &mut state, screen_w, screen_h);
            right_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        }
    }
    let settings_started = std::time::Instant::now();
    draw_settings_dialog(
        ctx,
        &mut state,
        &mut settings,
        &mut render_caches,
        screen_w,
        screen_h,
    );
    let settings_ms = settings_started.elapsed().as_secs_f64() * 1000.0;
    let telemetry_started = std::time::Instant::now();
    draw_telemetry_dialog(
        ctx,
        &mut state,
        &mut settings,
        &mut commands,
        &perf_telemetry.telemetry,
        &mut render_caches,
        screen_w,
        screen_h,
    );
    let telemetry_ms = telemetry_started.elapsed().as_secs_f64() * 1000.0;
    if !state.performance_diagnostic_hide_panels {
        draw_warning_dialog(ctx, &mut dialog);
    }
    record_egui_pass_timing(
        &mut perf_telemetry,
        egui_started.elapsed().as_secs_f64() * 1000.0,
        settings_ms + telemetry_ms,
        left_panel_ms,
        right_panel_ms,
    );

    if state.generation_busy {
        return;
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_generate"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_generate")));
        state.generation_busy = true;
        let profile = state.profile.clone();
        match run_generation(&profile) {
            Ok(output) => match StudioSession::from_generation(profile, output) {
                Ok(session) => {
                    adopt_session(session, &mut settings, &mut state);
                    super::rebuild_session_scene(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &assets,
                        &mut scene_root,
                        &mut state,
                        &mut render_caches,
                    );
                    reset_camera_after_generation(&mut camera);
                    perf_telemetry.vram_dirty = true;
                    let _ = settings.save();
                }
                Err(err) => {
                    state.generation_error = Some(err.to_string());
                }
            },
            Err(err) => {
                state.generation_error = Some(err.to_string());
            }
        }
        state.generation_busy = false;
    }

    if let Some(path) =
        ctx.data(|d| d.get_temp::<std::path::PathBuf>(egui::Id::new("do_save_scenario")))
    {
        ctx.data_mut(|d| d.remove::<std::path::PathBuf>(egui::Id::new("do_save_scenario")));
        save_scenario_action(&mut state, &path);
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_save_candidate"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_save_candidate")));
        execute_save_candidate_action(&mut state);
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_reopen_candidate"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_reopen_candidate")));
        if let Some(adoption) = execute_reopen_candidate_action(&mut state) {
            if let (Some(session), Some(status)) = (adoption.session, adoption.status) {
                adopt_loaded_scenario_session(
                    session,
                    &mut settings,
                    &mut state,
                    adoption.message.clone(),
                );
                state.apply_refreshed_runtime_saveload_status(status, None);
                state.last_runtime_saveload_status = adoption.message;
                super::rebuild_session_scene(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &assets,
                    &mut scene_root,
                    &mut state,
                    &mut render_caches,
                );
                reset_camera_after_generation(&mut camera);
                perf_telemetry.vram_dirty = true;
            }
        }
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_load_scenario_manual"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_load_scenario_manual")));
        match load_scenario_manual_path_action(&mut state) {
            ScenarioActionResult::Loaded { session, message } => {
                adopt_loaded_scenario_session(session, &mut settings, &mut state, message);
                state.refresh_runtime_saveload_status_if_needed(false);
                super::rebuild_session_scene(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &assets,
                    &mut scene_root,
                    &mut state,
                    &mut render_caches,
                );
                reset_camera_after_generation(&mut camera);
                perf_telemetry.vram_dirty = true;
            }
            _ => {}
        }
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_load_scenario_picker"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_load_scenario_picker")));
        match open_native_scenario_load_picker(&mut state) {
            ScenarioPickerActionResult::Loaded { session, message } => {
                adopt_loaded_scenario_session(session, &mut settings, &mut state, message);
                state.refresh_runtime_saveload_status_if_needed(false);
                super::rebuild_session_scene(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &assets,
                    &mut scene_root,
                    &mut state,
                    &mut render_caches,
                );
                reset_camera_after_generation(&mut camera);
                perf_telemetry.vram_dirty = true;
            }
            _ => {}
        }
    }
}

fn studio_panel_frame(opacity: f32, corner_radius: f32) -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            12,
            18,
            32,
            (opacity * 210.0) as u8,
        ))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(70, 110, 170, (opacity * 180.0) as u8),
        ))
        .inner_margin(12.0)
        .corner_radius(corner_radius)
}

fn inactive_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(egui::Color32::from_gray(120)))
            .fill(egui::Color32::from_rgba_unmultiplied(28, 32, 42, 120))
            .stroke(egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(60, 70, 90, 100),
            )),
    )
}

fn draw_window_controls(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    windows: &mut Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    exit: &mut EventWriter<AppExit>,
) {
    egui::Area::new(egui::Id::new("window_controls"))
        .fixed_pos(egui::pos2(ctx.screen_rect().max.x - 280.0, 8.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button(TELEMETRY_BUTTON_LABEL)
                    .on_hover_text(TELEMETRY_TOOLTIP)
                    .clicked()
                {
                    state.telemetry_dialog.toggle_visible();
                }
                if ui
                    .small_button(SETTINGS_BUTTON_LABEL)
                    .on_hover_text(SETTINGS_TOOLTIP)
                    .clicked()
                {
                    state.settings_dialog.toggle_visible();
                    settings.settings_dialog_visible = state.settings_dialog.visible;
                    let _ = settings.save();
                }
                if ui.button("—").clicked() {
                    minimize_window(windows);
                }
                if ui.button("▢").clicked() {
                    set_window_mode(windows, settings, WindowModeSetting::BorderlessFullscreen);
                }
                if ui.button("⛶").clicked() {
                    settings.exclusive_fullscreen_enabled = true;
                    set_window_mode(windows, settings, WindowModeSetting::ExclusiveFullscreen);
                }
                if ui.button("✕").clicked() {
                    exit.write(AppExit::Success);
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_button_is_gear_icon_with_settings_tooltip() {
        assert_eq!(SETTINGS_BUTTON_LABEL, "⚙");
        assert_eq!(SETTINGS_TOOLTIP, "Settings");
    }

    #[test]
    fn telemetry_button_is_left_of_settings_button_with_tooltip() {
        assert_eq!(TELEMETRY_BUTTON_LABEL, "Telemetry");
        assert_eq!(TELEMETRY_TOOLTIP, "Performance Telemetry");
        let controls = WINDOW_CONTROLS_LEFT_TO_RIGHT;
        let telemetry_idx = controls
            .iter()
            .position(|label| *label == TELEMETRY_BUTTON_LABEL)
            .expect("telemetry control");
        let settings_idx = controls
            .iter()
            .position(|label| *label == SETTINGS_BUTTON_LABEL)
            .expect("settings control");
        assert_eq!(settings_idx, telemetry_idx + 1);
    }
    #[test]
    fn settings_title_drag_rect_does_not_overlap_close_rect() {
        let title_rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(380.0, 32.0));
        let close_rect = egui::Rect::from_min_max(egui::pos2(340.0, 4.0), egui::pos2(372.0, 28.0));
        let drag_rect = settings_title_bar_drag_rect(title_rect, close_rect, 6.0);
        assert!(drag_rect.max.x <= close_rect.min.x - 6.0);
        assert!(!drag_rect.intersects(close_rect));
    }

    #[test]
    fn settings_title_drag_rect_covers_title_area() {
        let title_rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(380.0, 32.0));
        let close_rect = egui::Rect::from_min_max(egui::pos2(340.0, 4.0), egui::pos2(372.0, 28.0));
        let drag_rect = settings_title_bar_drag_rect(title_rect, close_rect, 6.0);
        assert_eq!(drag_rect.min, title_rect.min);
        assert_eq!(drag_rect.max.y, title_rect.max.y);
        assert!(drag_rect.contains(egui::pos2(24.0, 16.0)));
        assert!(drag_rect.max.x < title_rect.max.x);
    }
}

fn draw_settings_dialog(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
    screen_w: f32,
    screen_h: f32,
) {
    if !state.settings_dialog.visible {
        return;
    }

    let bounds = settings_dialog_bounds(ctx, state, screen_w, screen_h);
    let desired = egui::Rect::from_min_size(
        egui::pos2(
            state.settings_dialog.position[0],
            state.settings_dialog.position[1],
        ),
        SETTINGS_DIALOG_SIZE,
    );
    let clamped = clamp_dialog_rect_away_from_panels(desired, &bounds);
    state.settings_dialog.position = [clamped.min.x, clamped.min.y];
    settings.settings_dialog_position = state.settings_dialog.position;

    egui::Area::new(egui::Id::new("settings_dialog"))
        .order(egui::Order::Foreground)
        .fixed_pos(clamped.min)
        .show(ctx, |ui| {
            studio_panel_frame(0.82, 10.0).show(ui, |ui| {
                ui.set_width(SETTINGS_DIALOG_SIZE.x - 24.0);
                ui.set_min_height(SETTINGS_DIALOG_SIZE.y - 24.0);
                let mut close_rect = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO);
                let title_response = ui
                    .horizontal(|ui| {
                        ui.heading("Settings");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_response = ui.button("X");
                            close_rect = close_response.rect;
                            if close_response.clicked() {
                                close_settings_dialog_from_icon(state, settings);
                            }
                        });
                    })
                    .response;
                let title_rects = SettingsTitleBarRects {
                    title_rect: title_response.rect,
                    close_rect,
                    drag_rect: settings_title_bar_drag_rect(
                        title_response.rect,
                        close_rect,
                        SETTINGS_TITLE_CLOSE_DRAG_GAP,
                    ),
                };
                let title_drag = ui.interact(
                    title_rects.drag_rect,
                    ui.id().with("settings_title_drag"),
                    egui::Sense::drag(),
                );
                if title_drag.dragged() {
                    let delta = ctx.input(|input| input.pointer.delta());
                    let moved = clamped.translate(delta);
                    let clamped_moved = clamp_dialog_rect_away_from_panels(moved, &bounds);
                    state.settings_dialog.position = [clamped_moved.min.x, clamped_moved.min.y];
                    settings.settings_dialog_position = state.settings_dialog.position;
                }
                ui.separator();
                ui.label(egui::RichText::new("Star rendering").strong());
                let mut values = state.settings_dialog.star_render;
                let mut mode = state.settings_dialog.star_render_mode;
                let mut changed = false;
                egui::ComboBox::from_label("Render mode")
                    .selected_text(mode.label())
                    .show_ui(ui, |ui| {
                        for candidate in StarRenderMode::ALL {
                            changed |= ui
                                .selectable_value(&mut mode, candidate, candidate.label())
                                .changed();
                        }
                    });
                changed |= ui
                    .add(
                        egui::Slider::new(&mut values.base_blur_radius, 0.0..=1.0)
                            .text("Base Star Blur Radius"),
                    )
                    .changed();
                changed |= ui
                    .add(
                        egui::Slider::new(&mut values.falloff_distance_percent, 1.0..=100.0)
                            .suffix("%")
                            .text("Falloff Distance"),
                    )
                    .changed();
                changed |= ui
                    .add(
                        egui::Slider::new(&mut values.falloff_blur_radius_percent, 0.0..=100.0)
                            .suffix("%")
                            .text("Falloff Star Blur Radius"),
                    )
                    .changed();
                changed |= ui
                    .add(
                        egui::Slider::new(&mut values.falloff_opacity_percent, 0.0..=100.0)
                            .suffix("%")
                            .text("Falloff Star Opacity"),
                    )
                    .changed();
                if changed {
                    apply_star_render_settings(
                        values,
                        mode,
                        state,
                        settings,
                        &mut render_caches.star_visual,
                    );
                }
                ui.separator();
                ui.label(egui::RichText::new("Star nameplates").strong());
                let mut nameplate_values = settings.star_nameplate_settings();
                let mut nameplate_changed = false;
                nameplate_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut nameplate_values.relative_width_percent,
                            20.0..=200.0,
                        )
                        .suffix("%")
                        .text("Nameplate Relative Width"),
                    )
                    .changed();
                nameplate_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut nameplate_values.base_transparency_percent,
                            0.0..=100.0,
                        )
                        .suffix("%")
                        .text("Base Transparency"),
                    )
                    .changed();
                nameplate_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut nameplate_values.relative_falloff_distance_percent,
                            5.0..=100.0,
                        )
                        .suffix("%")
                        .text("Relative Falloff Distance"),
                    )
                    .changed();
                nameplate_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut nameplate_values.relative_falloff_transparency_percent,
                            0.0..=100.0,
                        )
                        .suffix("%")
                        .text("Relative Falloff Transparency"),
                    )
                    .changed();
                if nameplate_changed {
                    apply_nameplate_render_settings(nameplate_values, state, settings);
                }
                ui.separator();
                ui.label(egui::RichText::new("Hyperlane rendering").strong());
                let mut hyperlane_values = state.settings_dialog.hyperlane_render;
                let mut hyperlane_changed = false;
                hyperlane_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut hyperlane_values.base_thickness_percent_of_star,
                            1.0..=25.0,
                        )
                        .suffix("%")
                        .text("Base Hyperlane Line Thickness"),
                    )
                    .changed();
                hyperlane_changed |= ui
                    .add(
                        egui::Slider::new(&mut hyperlane_values.base_opacity_percent, 0.0..=100.0)
                            .suffix("%")
                            .text("Base Hyperlane Opacity"),
                    )
                    .changed();
                hyperlane_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut hyperlane_values.falloff_distance_percent,
                            1.0..=100.0,
                        )
                        .suffix("%")
                        .text("Falloff Distance"),
                    )
                    .changed();
                hyperlane_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut hyperlane_values.falloff_thickness_percent,
                            0.0..=100.0,
                        )
                        .suffix("%")
                        .text("Falloff Thickness"),
                    )
                    .changed();
                hyperlane_changed |= ui
                    .add(
                        egui::Slider::new(
                            &mut hyperlane_values.falloff_opacity_percent,
                            0.0..=100.0,
                        )
                        .suffix("%")
                        .text("Falloff Opacity"),
                    )
                    .changed();
                if hyperlane_changed {
                    apply_hyperlane_render_settings(
                        hyperlane_values,
                        state,
                        settings,
                        &mut render_caches.hyperlane,
                    );
                }
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        reset_settings_dialog_values(state, settings, render_caches);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            close_settings_dialog_from_button(state, settings);
                        }
                    });
                });
            });
        });
}

fn draw_telemetry_dialog(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    commands: &mut Commands,
    telemetry: &crate::studio_performance_telemetry::StudioPerformanceTelemetry,
    render_caches: &mut StudioRenderLoopCaches,
    screen_w: f32,
    screen_h: f32,
) {
    if !state.telemetry_dialog.visible {
        return;
    }

    let bounds = settings_dialog_bounds(ctx, state, screen_w, screen_h);
    let desired = egui::Rect::from_min_size(
        egui::pos2(
            state.telemetry_dialog.position[0],
            state.telemetry_dialog.position[1],
        ),
        TELEMETRY_DIALOG_SIZE,
    );
    let clamped = clamp_dialog_rect_away_from_panels(desired, &bounds);
    state.telemetry_dialog.position = [clamped.min.x, clamped.min.y];

    egui::Area::new(egui::Id::new("telemetry_dialog"))
        .order(egui::Order::Foreground)
        .fixed_pos(clamped.min)
        .show(ctx, |ui| {
            studio_panel_frame(0.82, 10.0).show(ui, |ui| {
                ui.set_width(TELEMETRY_DIALOG_SIZE.x - 24.0);
                ui.set_min_height(TELEMETRY_DIALOG_SIZE.y - 24.0);
                let mut close_rect = egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO);
                let title_response = ui
                    .horizontal(|ui| {
                        ui.heading("Performance Telemetry");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_response = ui.button("X");
                            close_rect = close_response.rect;
                            if close_response.clicked() {
                                state.telemetry_dialog.close_icon();
                            }
                        });
                    })
                    .response;
                let title_rects = SettingsTitleBarRects {
                    title_rect: title_response.rect,
                    close_rect,
                    drag_rect: settings_title_bar_drag_rect(
                        title_response.rect,
                        close_rect,
                        SETTINGS_TITLE_CLOSE_DRAG_GAP,
                    ),
                };
                let title_drag = ui.interact(
                    title_rects.drag_rect,
                    ui.id().with("telemetry_title_drag"),
                    egui::Sense::drag(),
                );
                if title_drag.dragged() {
                    let delta = ctx.input(|input| input.pointer.delta());
                    let moved = clamped.translate(delta);
                    let clamped_moved = clamp_dialog_rect_away_from_panels(moved, &bounds);
                    state.telemetry_dialog.position = [clamped_moved.min.x, clamped_moved.min.y];
                }
                ui.separator();
                let performance_lines = performance_settings_section_lines(telemetry);
                if let Some(title) = performance_lines.first() {
                    ui.label(egui::RichText::new(title).strong());
                }
                for line in performance_lines.iter().skip(1) {
                    ui.label(line);
                }
                ui.separator();
                ui.label(egui::RichText::new("Performance isolation").strong());
                let mut hide_stars = !state.show_stars;
                if ui.checkbox(&mut hide_stars, "Hide stars").changed() {
                    state.show_stars = !hide_stars;
                }
                let mut hide_hyperlanes = !state.show_hyperlanes;
                if ui
                    .checkbox(&mut hide_hyperlanes, "Hide hyperlanes")
                    .changed()
                {
                    state.show_hyperlanes = !hide_hyperlanes;
                }
                ui.checkbox(
                    &mut state.performance_diagnostic_hide_star_aura,
                    "Disable star aura layer",
                );
                let mut force_crisp = state.star_render_mode == StarRenderMode::CrispCircle;
                if ui
                    .checkbox(&mut force_crisp, "Force crisp/no-bloom star render")
                    .changed()
                {
                    state.star_render_mode = if force_crisp {
                        StarRenderMode::CrispCircle
                    } else {
                        StarRenderMode::BloomStarburst
                    };
                    if let Some(session) = state.session.as_mut() {
                        session
                            .view_model
                            .apply_star_render_mode(state.star_render_mode);
                    }
                    mark_star_visual_render_dirty(&mut render_caches.star_visual);
                }
                ui.checkbox(
                    &mut state.performance_diagnostic_hide_panels,
                    "Hide main egui panels",
                );
                ui.checkbox(
                    &mut state.performance_diagnostic_freeze_camera,
                    "Freeze camera update",
                );
                ui.horizontal(|ui| {
                    if ui.button(DIAGNOSTIC_MINIMAL_RENDER_BUTTON).clicked() {
                        apply_performance_diagnostic_minimal_render(state, settings, render_caches);
                    }
                    if ui.button(RESTORE_NORMAL_RENDER_BUTTON).clicked() {
                        restore_performance_normal_render(state, settings, render_caches);
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Screenshot").clicked() {
                        let cwd = std::env::current_dir().unwrap_or_else(|_| ".".into());
                        if let Some(filename) = next_screenshot_filename(&cwd) {
                            commands
                                .spawn(Screenshot::primary_window())
                                .observe(save_to_disk(filename.clone()));
                            state.status_message = format!("Screenshot requested: {filename}");
                        } else {
                            state.status_message =
                                "Screenshot failed: could not allocate filename".into();
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            state.telemetry_dialog.close_button();
                        }
                    });
                });
            });
        });
}

fn close_settings_dialog_from_icon(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
) {
    close_settings_dialog(state, settings, SettingsDialogCloseSource::Icon);
}

fn close_settings_dialog_from_button(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
) {
    close_settings_dialog(state, settings, SettingsDialogCloseSource::Button);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsDialogCloseSource {
    Icon,
    Button,
}

fn close_settings_dialog(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    source: SettingsDialogCloseSource,
) {
    match source {
        SettingsDialogCloseSource::Icon => state.settings_dialog.close_icon(),
        SettingsDialogCloseSource::Button => state.settings_dialog.close_button(),
    }
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    settings.set_star_falloff_settings(state.star_falloff_settings);
    settings.set_star_render_mode(state.star_render_mode);
    settings.set_hyperlane_render_settings(state.hyperlane_render_settings);
    let _ = settings.save();
    if let Err(err) = super::save_current_studio_config(state, settings, None) {
        state.status_message = format!("Studio config save failed: {err}");
    }
}

fn reset_settings_dialog_values(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
) {
    let defaults = crate::studio_config::SimThingStudioConfig::default();
    let (star, mode) = defaults.star_rendering.clone().to_star_settings();
    let nameplate = defaults.nameplate_rendering.to_nameplate_settings();
    let hyperlane = defaults.hyperlane_rendering.to_hyperlane_settings();
    state.star_falloff_settings = star;
    state.star_render_mode = mode;
    state.hyperlane_render_settings = hyperlane;
    state.settings_dialog.star_render = star;
    state.settings_dialog.star_render_mode = mode;
    state.settings_dialog.hyperlane_render = hyperlane;
    settings.set_star_falloff_settings(star);
    settings.set_star_render_mode(mode);
    settings.set_star_nameplate_settings(nameplate);
    settings.set_hyperlane_render_settings(hyperlane);
    if let Some(session) = state.session.as_mut() {
        apply_star_falloff_settings_to_meta(&mut session.view_model.render_meta, star);
        apply_star_render_mode_to_meta(&mut session.view_model.render_meta, mode);
        apply_hyperlane_render_settings_to_meta(&mut session.view_model.render_meta, hyperlane);
    }
    mark_hyperlane_render_dirty(&mut render_caches.hyperlane);
    mark_star_visual_render_dirty(&mut render_caches.star_visual);
}

fn settings_dialog_bounds(
    ctx: &egui::Context,
    state: &StudioAppState,
    screen_w: f32,
    screen_h: f32,
) -> FloatingDialogBounds {
    let left_panel = if state.left_panel_collapsed {
        None
    } else {
        let layout = compute_floating_panel_layout(screen_w, screen_h, false);
        Some(rect_from_xywh(
            layout.x,
            layout.y,
            layout.width,
            layout.height,
        ))
    };
    let right_panel = if state.session.is_some() {
        let (x, y, width, height) = right_panel_rect(screen_w, screen_h);
        Some(rect_from_xywh(x, y, width, height))
    } else {
        None
    };
    FloatingDialogBounds {
        viewport: ctx.screen_rect(),
        left_panel,
        right_panel,
    }
}

fn apply_star_render_settings(
    values: StarFalloffSettings,
    mode: StarRenderMode,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    star_cache: &mut crate::studio_render_loop_dirty_gate::StarVisualSyncCacheState,
) {
    let values = values.clamped();
    let dirty = star_visuals_dirty_after_settings_change(
        state.star_falloff_settings,
        values,
        state.star_render_mode,
        mode,
    );
    state.star_falloff_settings = values;
    state.star_render_mode = mode;
    state.settings_dialog.set_star_render(values);
    state.settings_dialog.set_star_render_mode(mode);
    settings.set_star_falloff_settings(values);
    settings.set_star_render_mode(mode);
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    if dirty {
        state.status_message = "Updated star render settings".into();
        mark_star_visual_render_dirty(star_cache);
    }
    if let Some(session) = state.session.as_mut() {
        session.view_model.apply_star_falloff_settings(values);
        session.view_model.apply_star_render_mode(mode);
    }
    let _ = settings.save();
}

fn apply_nameplate_render_settings(
    values: StarNameplateSettings,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
) {
    settings.set_star_nameplate_settings(values);
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    state.status_message = "Updated star nameplate settings".into();
    let _ = settings.save();
}

fn performance_diagnostic_flags(state: &StudioAppState) -> PerformanceDiagnosticFlags {
    PerformanceDiagnosticFlags {
        hide_panels: state.performance_diagnostic_hide_panels,
        freeze_camera: state.performance_diagnostic_freeze_camera,
        hide_star_aura: state.performance_diagnostic_hide_star_aura,
    }
}

fn apply_performance_diagnostic_minimal_render(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
) {
    if state.performance_normal_render_snapshot.is_none() {
        state.performance_normal_render_snapshot = Some(capture_normal_render_snapshot(
            state.show_stars,
            state.show_hyperlanes,
            state.star_render_mode,
            performance_diagnostic_flags(state),
        ));
    }
    let mut flags = performance_diagnostic_flags(state);
    apply_diagnostic_minimal_render(
        &mut state.show_stars,
        &mut state.show_hyperlanes,
        &mut state.star_render_mode,
        &mut flags,
    );
    state.performance_diagnostic_hide_star_aura = flags.hide_star_aura;
    state
        .settings_dialog
        .set_star_render_mode(state.star_render_mode);
    settings.set_star_render_mode(state.star_render_mode);
    if let Some(session) = state.session.as_mut() {
        session
            .view_model
            .apply_star_render_mode(state.star_render_mode);
    }
    mark_hyperlane_render_dirty(&mut render_caches.hyperlane);
    mark_star_visual_render_dirty(&mut render_caches.star_visual);
    state.status_message = "Applied diagnostic minimal render preset".into();
}

fn restore_performance_normal_render(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
) {
    let Some(snapshot) = state.performance_normal_render_snapshot.take() else {
        return;
    };
    let mut flags = performance_diagnostic_flags(state);
    restore_normal_render_from_snapshot(
        snapshot,
        &mut state.show_stars,
        &mut state.show_hyperlanes,
        &mut state.star_render_mode,
        &mut flags,
    );
    state.performance_diagnostic_hide_panels = flags.hide_panels;
    state.performance_diagnostic_freeze_camera = flags.freeze_camera;
    state.performance_diagnostic_hide_star_aura = flags.hide_star_aura;
    state
        .settings_dialog
        .set_star_render_mode(state.star_render_mode);
    settings.set_star_render_mode(state.star_render_mode);
    if let Some(session) = state.session.as_mut() {
        session
            .view_model
            .apply_star_render_mode(state.star_render_mode);
    }
    mark_hyperlane_render_dirty(&mut render_caches.hyperlane);
    mark_star_visual_render_dirty(&mut render_caches.star_visual);
    state.status_message = "Restored normal render preset".into();
}

fn apply_hyperlane_render_settings(
    values: HyperlaneRenderSettings,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    hyperlane_cache: &mut crate::studio_render_loop_dirty_gate::HyperlaneRenderCacheState,
) {
    let values = values.clamped();
    let dirty =
        hyperlane_visuals_dirty_after_settings_change(state.hyperlane_render_settings, values);
    state.hyperlane_render_settings = values;
    state.settings_dialog.set_hyperlane_render(values);
    settings.set_hyperlane_render_settings(values);
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    if dirty {
        state.status_message = "Updated hyperlane render settings".into();
        mark_hyperlane_render_dirty(hyperlane_cache);
    }
    if let Some(session) = state.session.as_mut() {
        session.view_model.apply_hyperlane_render_settings(values);
    }
    let _ = settings.save();
}

fn draw_collapsed_tab(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    screen_w: f32,
    screen_h: f32,
) {
    let tab = compute_collapsed_panel_tab(screen_w, screen_h);
    egui::Area::new(egui::Id::new("left_collapsed"))
        .fixed_pos(egui::pos2(tab.x, tab.y))
        .show(ctx, |ui| {
            if ui.button(">>").clicked() {
                state.left_panel_collapsed = false;
            }
        });
}

fn draw_left_panel(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    dialog: &mut WarningDialogModel,
    camera: &mut StudioCamera,
    screen_w: f32,
    screen_h: f32,
) {
    let layout = compute_floating_panel_layout(screen_w, screen_h, false);
    let opacity = state.left_panel_opacity;
    let title = left_panel_title(state.session.as_ref().map(|s| s.galaxy_name()));

    let scroll_height = left_panel_content_scroll_height(&layout);
    let area = egui::Area::new(egui::Id::new("left_panel"))
        .fixed_pos(egui::pos2(layout.x, layout.y))
        .show(ctx, |ui| {
            studio_panel_frame(opacity, layout.corner_radius).show(ui, |ui| {
                ui.set_width(layout.width);
                ui.set_max_height(layout.height);
                ui.horizontal(|ui| {
                    if title.is_empty() {
                        ui.label("");
                    } else {
                        ui.heading(title);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("<<").clicked() {
                            state.left_panel_collapsed = true;
                        }
                    });
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("left_panel_scroll")
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if inactive_button(ui, "New").clicked() {
                                *dialog = unimplemented_action_response(StudioAction::New);
                            }
                            if inactive_button(ui, "Load").clicked() {
                                *dialog = unimplemented_action_response(StudioAction::Load);
                            }
                            if inactive_button(ui, "Save").clicked() {
                                *dialog = unimplemented_action_response(StudioAction::Save);
                            }
                            if ui.button("Generate").clicked() {
                                ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_generate"), true));
                            }
                        });
                        ui.separator();
                        // PERF: collapse heavy sections by default. A collapsed egui CollapsingHeader
                        // does not lay out its children, so the per-frame egui layout/tessellation cost
                        // of the left panel drops dramatically (the cause of the FPS collapse). See
                        // docs/simthing-bevy-performance.md.
                        egui::CollapsingHeader::new("Presets")
                            .id_salt("left_panel_presets")
                            .default_open(false)
                            .show(ui, |ui| {
                                for preset in GenerationPreset::all() {
                                    let active = preset.is_active();
                                    let label = preset.label();
                                    if active {
                                        if ui
                                            .selectable_label(
                                                state.profile.preset_id == preset.id(),
                                                label,
                                            )
                                            .clicked()
                                        {
                                            state.profile = preset.to_profile();
                                        }
                                    } else if inactive_button(ui, label).clicked() {
                                        *dialog = unimplemented_action_response(
                                            StudioAction::InactivePreset(preset.id().into()),
                                        );
                                    }
                                }
                            });
                        egui::CollapsingHeader::new("Active generation controls")
                            .id_salt("left_panel_generation")
                            .default_open(false)
                            .show(ui, |ui| {
                                generation_fields(ui, &mut state.profile, dialog);
                            });
                        egui::CollapsingHeader::new("Scenario / runtime save-load")
                            .id_salt("left_panel_scenario")
                            .default_open(false)
                            .show(ui, |ui| {
                                draw_scenario_io_controls(ctx, ui, state);
                            });
                        ui.separator();
                        render_debug_controls(ui, state);
                        ui.separator();
                        ui.label("Camera");
                        ui.horizontal(|ui| {
                            ui.label(format!("View: {}", camera.view_mode().label()));
                            if ui.button("Toggle (Tab)").clicked() {
                                camera.toggle_view_mode();
                            }
                        });
                        if ui.button("Overhead (O)").clicked() {
                            snap_overhead(camera);
                        }
                        if ui.button("Reset (R)").clicked() {
                            reset_camera_after_generation(camera);
                        }
                        if let Some(err) = &state.generation_error {
                            ui.colored_label(egui::Color32::RED, err);
                        } else if !state.last_scenario_io_status.is_empty() {
                            ui.label(&state.last_scenario_io_status);
                        } else if !state.status_message.is_empty() {
                            ui.label(&state.status_message);
                        }
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("SimThing Studio").small().weak());
                    });
            });
        });
    state.left_panel_hovered = area.response.hovered();
}

fn draw_scenario_io_controls(ctx: &egui::Context, ui: &mut egui::Ui, state: &mut StudioAppState) {
    ui.label(egui::RichText::new("Scenario (model authority)").strong());
    ui.label(
        egui::RichText::new("Separate from simthing-studio-config.json")
            .small()
            .weak(),
    );
    ui.horizontal(|ui| {
        ui.label("Scenario path:");
        ui.text_edit_singleline(&mut state.scenario_path_text);
    });
    ui.horizontal(|ui| {
        let save_enabled = state.session.is_some();
        if save_enabled {
            if ui.button("Save Scenario").clicked() {
                if let Ok(path) = super::scenario_io::scenario_path_from_state(state) {
                    ctx.data_mut(|d| {
                        d.insert_temp(egui::Id::new("do_save_scenario"), path);
                    });
                } else {
                    state.last_scenario_io_status =
                        "Scenario save failed: invalid scenario path".into();
                    state.status_message = state.last_scenario_io_status.clone();
                }
            }
        } else if inactive_button(ui, "Save Scenario").clicked() {
            state.last_scenario_io_status = "Scenario save failed: no active session".into();
            state.status_message = state.last_scenario_io_status.clone();
        }
        if ui.button("Load Scenario...").clicked() {
            ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_load_scenario_picker"), true));
        }
        if ui.button("Manual Load Path").clicked() {
            ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_load_scenario_manual"), true));
        }
    });
    ui.separator();
    draw_runtime_candidate_saveload_controls(ctx, ui, state);
}

fn execute_save_candidate_action(state: &mut StudioAppState) {
    let session_before = state.session.clone();
    let status_before = state.runtime_saveload_status.clone();
    let path = match super::scenario_io::validate_scenario_path_text(&state.candidate_path_text) {
        Ok(path) => path,
        Err(reason) => {
            state.last_runtime_saveload_status =
                format!("Save Candidate failed: invalid candidate path ({reason})");
            return;
        }
    };
    let Some(session) = session_before.as_ref() else {
        state.last_runtime_saveload_status =
            "Save Candidate failed: no active loaded session".into();
        return;
    };
    let json =
        match crate::canonical_json_from_loaded_scenario_authority(&session.scenario_authority) {
            Ok(json) => json,
            Err(_) => {
                state.last_runtime_saveload_status =
                    "Save Candidate failed: could not serialize loaded scenario authority".into();
                state.session = session_before;
                state.runtime_saveload_status = status_before;
                return;
            }
        };
    match save_candidate_scenario_for_studio_create_new("studio_save_candidate", &json, &path) {
        Ok(result) => {
            state.last_runtime_saveload_status = result.message.clone();
            state.session = session_before;
            if result.saved {
                state.mark_runtime_saveload_status_dirty();
                state.refresh_runtime_saveload_status_if_needed(false);
            } else {
                state.runtime_saveload_status = status_before;
            }
            return;
        }
        Err(_) => {
            state.last_runtime_saveload_status =
                "Save Candidate failed: candidate save/reopen plan unavailable".into();
        }
    }
    state.session = session_before;
    state.runtime_saveload_status = status_before;
}

fn execute_reopen_candidate_action(
    state: &mut StudioAppState,
) -> Option<crate::StudioReopenCandidateAdoptionResult> {
    let path = match super::scenario_io::validate_scenario_path_text(&state.candidate_path_text) {
        Ok(path) => path,
        Err(reason) => {
            state.last_runtime_saveload_status =
                format!("Reopen Candidate failed: invalid candidate path ({reason})");
            return None;
        }
    };
    match reopen_candidate_scenario_for_studio_session(&path) {
        Ok(adoption) if adoption.adopted => Some(adoption),
        Ok(adoption) => {
            state.last_runtime_saveload_status = adoption.message;
            None
        }
        Err(_) => {
            state.last_runtime_saveload_status =
                "Reopen Candidate failed: could not load canonical candidate JSON".into();
            None
        }
    }
}

fn draw_runtime_candidate_saveload_controls(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    state: &mut StudioAppState,
) {
    ui.label(egui::RichText::new("Runtime / Candidate Save-Reopen (presentation only)").strong());
    ui.label(
        egui::RichText::new(
            "UI state, Bevy ECS, runtime reports, and GPU buffers are not authority",
        )
        .small()
        .weak(),
    );

    // PERF + authority: scenario status is expensive (canonical serialize + STEAD/RF/report/candidate
    // evaluation). It is computed ONLY on an explicit Refresh Runtime Status click (or on load/save/reopen
    // events that mark it dirty). The draw path must NEVER serialize/evaluate scenario state — doing so per
    // frame caused ~550 ms frames. When dirty, the UI shows "click Refresh Runtime Status" (cached display
    // only). See docs/simthing-bevy-performance.md.
    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_refresh_runtime_status"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_refresh_runtime_status")));
        state.refresh_runtime_saveload_status_if_needed(true);
    }

    if state.runtime_saveload_status_dirty {
        ui.label(
            egui::RichText::new("Status refresh pending — click Refresh Runtime Status")
                .weak()
                .italics(),
        );
    } else if state.runtime_saveload_status_refresh_in_progress {
        ui.label(
            egui::RichText::new("Refreshing runtime status…")
                .weak()
                .italics(),
        );
    }
    if let Some(ms) = state.runtime_saveload_status_last_refresh_ms {
        ui.label(format!("Last status refresh: {ms} ms"));
    }
    if let Some(digest) = state.runtime_saveload_status_source_digest {
        ui.label(format!("Cached digest: {digest}"));
    }

    if let Some(status) = &state.runtime_saveload_status {
        ui.label(format!(
            "Loaded digest: {}",
            status
                .loaded_scenario_digest
                .map(|digest| digest.to_string())
                .unwrap_or_else(|| "n/a".into())
        ));
        ui.label(format!(
            "STEAD/link/tree validation: {}",
            if status.stead_validation_ready {
                "ready"
            } else {
                "not ready"
            }
        ));
        ui.label(format!(
            "Recursive RF runtime: {}",
            if status.recursive_rf_runtime_ready {
                "ready"
            } else {
                "not ready"
            }
        ));
        ui.label(format!(
            "Runtime report chain: {}",
            if status.runtime_report_chain_ready {
                "ready"
            } else {
                "not ready"
            }
        ));
        ui.label(format!(
            "Candidate ready: {} (digest: {})",
            if status.candidate_ready { "yes" } else { "no" },
            status
                .candidate_digest
                .map(|digest| digest.to_string())
                .unwrap_or_else(|| "n/a".into())
        ));
    } else {
        ui.label("Load a scenario to evaluate runtime/candidate readiness.");
    }

    ui.horizontal(|ui| {
        ui.label("Candidate path:");
        ui.text_edit_singleline(&mut state.candidate_path_text);
    });
    ui.horizontal(|ui| {
        let save_enabled = state
            .runtime_saveload_status
            .as_ref()
            .is_some_and(|status| status.candidate_save_ready);
        if save_enabled {
            if ui.button("Save Candidate").clicked() {
                ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_save_candidate"), true));
            }
        } else if inactive_button(ui, "Save Candidate").clicked() {
            state.last_runtime_saveload_status =
                "Save Candidate unavailable: load scenario and wait for candidate readiness".into();
        }
        if ui.button("Reopen Candidate").clicked() {
            ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_reopen_candidate"), true));
        }
        if ui.button("Refresh Runtime Status").clicked() {
            ctx.data_mut(|d| d.insert_temp(egui::Id::new("do_refresh_runtime_status"), true));
        }
    });
    if !state.last_runtime_saveload_status.is_empty() {
        ui.label(&state.last_runtime_saveload_status);
    }
}

fn render_debug_controls(ui: &mut egui::Ui, state: &mut StudioAppState) {
    egui::CollapsingHeader::new("Render debug")
        .default_open(false)
        .show(ui, |ui| {
            ui.checkbox(&mut state.show_stars, "Show stars");
            ui.checkbox(&mut state.show_hyperlanes, "Show hyperlanes");
            ui.horizontal(|ui| {
                if ui.button("Stars only").clicked() {
                    state.show_stars = true;
                    state.show_hyperlanes = false;
                }
                if ui.button("Hyperlanes only").clicked() {
                    state.show_stars = false;
                    state.show_hyperlanes = true;
                }
                if ui.button("Both").clicked() {
                    state.show_stars = true;
                    state.show_hyperlanes = true;
                }
            });
        });
}

fn generation_fields(
    ui: &mut egui::Ui,
    profile: &mut GenerationProfile,
    dialog: &mut WarningDialogModel,
) {
    ui.label("Active generation controls");
    egui::Grid::new("gen_grid").show(ui, |ui| {
        ui.label("Shape");
        let old_shape = profile.shape.clone();
        ui.text_edit_singleline(&mut profile.shape);
        if profile.shape != old_shape {
            profile.init_shape_param_storage();
            let new_shape = profile.shape.clone();
            profile.switch_shape(&old_shape, &new_shape);
        }
        ui.end_row();
        ui.label("Stars");
        ui.add(egui::DragValue::new(&mut profile.star_count).range(1..=10000));
        ui.end_row();
        ui.label("Lattice edge");
        ui.add(egui::DragValue::new(&mut profile.lattice_edge).range(8..=1000));
        ui.end_row();
        ui.label("Seed");
        ui.add(egui::DragValue::new(&mut profile.seed));
        ui.end_row();
        ui.label("Target hyperlanes");
        ui.add(egui::DragValue::new(&mut profile.target_hyperlanes).range(1..=20000));
        ui.end_row();
        ui.label("Max lane distance");
        ui.add(
            egui::DragValue::new(&mut profile.max_hyperlane_distance)
                .speed(0.1)
                .range(1.0..=64.0),
        );
        ui.end_row();
        ui.checkbox(&mut profile.ensure_connected, "Ensure connected");
        ui.end_row();
        ui.checkbox(&mut profile.allow_disconnected, "Allow disconnected");
        ui.end_row();
        ui.checkbox(&mut profile.draw_core, "Draw core glow");
        ui.end_row();
        ui.checkbox(&mut profile.render_lanes, "Render lanes");
        ui.end_row();
        let spiral_active = spiral_arm_params_active(&profile.shape);
        ui.label("arm_width");
        if spiral_active {
            ui.add(egui::DragValue::new(&mut profile.arm_width).speed(0.1));
        } else if inactive_button(ui, &format!("{:.1}", profile.arm_width)).clicked() {
            *dialog = inactive_control_warning("arm_width (inactive for selected shape)");
        }
        ui.end_row();
        ui.label("arm_tightness");
        if spiral_active {
            ui.add(egui::DragValue::new(&mut profile.arm_tightness).speed(0.05));
        } else if inactive_button(ui, &format!("{:.2}", profile.arm_tightness)).clicked() {
            *dialog = inactive_control_warning("arm_tightness (inactive for selected shape)");
        }
        ui.end_row();
        ui.label("jitter");
        if spiral_active || profile.shape == "elliptical" {
            ui.add(egui::DragValue::new(&mut profile.jitter).speed(0.1));
        } else if inactive_button(ui, &format!("{:.1}", profile.jitter)).clicked() {
            *dialog = inactive_control_warning("jitter (inactive for selected shape)");
        }
        ui.end_row();
    });
    if !spiral_arm_params_active(&profile.shape) {
        ui.label(
            egui::RichText::new("Spiral arm params inactive for selected shape — not submitted.")
                .small()
                .weak(),
        );
    }
    ui.separator();
    ui.label("Deferred (visible, inactive)");
    for label in [
        "Import / Export settings",
        "Simulation session settings",
        "Layer toggles",
        "Clausewitz UI import experiment",
    ] {
        if inactive_button(ui, label).clicked() {
            *dialog = inactive_control_warning(label);
        }
    }
}

fn draw_right_panel(ctx: &egui::Context, state: &mut StudioAppState, screen_w: f32, screen_h: f32) {
    let Some(session) = state.session.as_ref() else {
        return;
    };
    let width = 320.0;
    let (_, margin_y) = crate::panel_layout::panel_margin(screen_w, screen_h);
    egui::Area::new(egui::Id::new("right_panel"))
        .fixed_pos(egui::pos2(
            screen_w - width - screen_w * 0.03,
            margin_y.max(48.0),
        ))
        .show(ctx, |ui| {
            let corner = crate::panel_layout::corner_radius_for_panel_width(width);
            studio_panel_frame(0.72, corner).show(ui, |ui| {
                ui.set_width(width);
                if let Some(selected_id) = state.selection.selected_system_id {
                    ui.heading("Selected system");
                    ui.separator();
                    if let Some(details) = selected_system_details(&session.view_model, selected_id)
                    {
                        ui.label(format!("System id: {}", details.system_id));
                        ui.label(format!(
                            "Structural grid: col {}, row {}",
                            details.structural_col, details.structural_row
                        ));
                        ui.label(format!(
                            "Render height (render-only): {:.3}",
                            details.render_height
                        ));
                        ui.label(format!("Hyperlane degree: {}", details.degree));
                        if details.incident_neighbor_ids.is_empty() {
                            ui.label("Incident neighbors: (none)");
                        } else {
                            ui.label("Incident neighbors:");
                            for neighbor in &details.incident_neighbor_ids {
                                ui.label(format!("  • {neighbor}"));
                            }
                        }
                        ui.label(egui::RichText::new(details.render_only_note).small().weak());
                        if ui.button("Clear selection (Esc)").clicked() {
                            state.selection.clear();
                        }
                    }
                    ui.separator();
                }
                ui.heading("Galaxy status");
                ui.separator();
                ui.label(format!("Galaxy: {}", session.galaxy_name()));
                if session.is_generated() {
                    if let Some(report) = session.report() {
                        ui.label(format!("Shape: {}", report.request.shape));
                        ui.label(format!("Seed: {}", report.generator.seed));
                        ui.label(format!("Systems: {}", report.output.system_count));
                        ui.label(format!(
                            "Grid: {}×{}",
                            report.request.lattice_width, report.request.lattice_height
                        ));
                        ui.label(format!(
                            "Base hyperlanes: {}",
                            report.output.base_hyperlane_count
                        ));
                        ui.label(format!(
                            "Topology hyperlanes: {}",
                            report.output.actual_topology_hyperlanes
                        ));
                        ui.label(format!(
                            "Connectivity bridges: {}",
                            report.output.connectivity_bridge_count
                        ));
                        ui.label(format!("Components: {}", report.output.component_count));
                        ui.label(format!(
                            "Average degree: {:.2}",
                            report.output.average_degree
                        ));
                        ui.label(format!(
                            "Isolated systems: {}",
                            report.output.isolated_system_count
                        ));
                        ui.label(format!("Map quality: {}", report.output.map_quality_status));
                        if !report.output.map_quality_warnings.is_empty() {
                            for warn in &report.output.map_quality_warnings {
                                ui.colored_label(egui::Color32::YELLOW, warn);
                            }
                        }
                    }
                } else {
                    let summary = &session.scenario_summary;
                    ui.label("Source: loaded scenario authority");
                    ui.label(format!("Systems: {}", summary.system_count));
                    ui.label(format!("Links: {}", summary.link_count));
                    ui.label(format!(
                        "Grid: {}×{} ({} occupied)",
                        summary.grid_width, summary.grid_height, summary.occupied_cells
                    ));
                    ui.label(format!(
                        "STEAD: {}",
                        if summary.stead_valid {
                            "valid"
                        } else {
                            "invalid"
                        }
                    ));
                    ui.label(format!(
                        "RF ready: {}",
                        if summary.rf_ready { "yes" } else { "no" }
                    ));
                    ui.label(format!(
                        "Heatmap readiness: {:?}",
                        summary.heatmap_readiness
                    ));
                    let gpu = &session.gpu_residency_readiness;
                    ui.label(format!(
                        "GPU index ready: {}",
                        if gpu.dense_location_index_ready {
                            "yes"
                        } else {
                            "no"
                        }
                    ));
                    if gpu.atlas_required {
                        ui.label("Atlas required for dense MF execution");
                    }
                }
                if let Some(path) = &session.report_path {
                    ui.label(format!("Report: {}", path.display()));
                }
                if let Some(path) = &session.scenario_path {
                    ui.label(format!("Scenario: {}", path.display()));
                }
            });
        });
}

fn draw_warning_dialog(ctx: &egui::Context, dialog: &mut WarningDialogModel) {
    if !dialog.visible {
        return;
    }
    egui::Window::new(&dialog.title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(&dialog.message);
            if ui.button("OK").clicked() {
                dialog.dismiss();
            }
        });
}
