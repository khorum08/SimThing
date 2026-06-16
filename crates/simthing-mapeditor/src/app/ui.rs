#![cfg(windows)]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::dialog::{
    inactive_control_warning, unimplemented_action_response, StudioAction, WarningDialogModel,
};
use crate::generation::{run_generation, GenerationPreset, GenerationProfile};
use crate::panel_layout::{
    compute_collapsed_panel_tab, compute_floating_panel_layout, left_panel_title,
    should_auto_collapse_panel,
};
use crate::selection::selected_system_details;
use crate::settings::WindowModeSetting;
use crate::shape_params::spiral_arm_params_active;

use super::camera::{reset_camera_after_generation, snap_overhead, StudioCamera};
use super::galaxy_render::{rebuild_galaxy_scene, StarVisualAssets};
use super::window::{minimize_window, set_window_mode};
use super::{adopt_session, GalaxySceneRoot, StudioAppState};
use crate::session::StudioSession;

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
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let screen = ctx.screen_rect();
    let screen_w = screen.width();
    let screen_h = screen.height();

    if should_auto_collapse_panel(screen_w) {
        state.left_panel_collapsed = true;
    }

    draw_window_controls(ctx, &mut settings, &mut windows, &mut exit);
    if !state.left_panel_collapsed {
        draw_left_panel(
            ctx,
            &mut state,
            &mut dialog,
            &mut camera,
            screen_w,
            screen_h,
        );
    } else {
        draw_collapsed_tab(ctx, &mut state, screen_w, screen_h);
    }
    if state.session.is_some() {
        draw_right_panel(ctx, &mut state, screen_w, screen_h);
    }
    draw_warning_dialog(ctx, &mut dialog);

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
            Ok(output) => {
                let session = StudioSession::from_generation(profile, output);
                adopt_session(session, &mut settings, &mut state);
                if let Some(session) = state.session.as_ref() {
                    rebuild_galaxy_scene(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &assets,
                        &mut scene_root,
                        session,
                    );
                }
                reset_camera_after_generation(&mut camera);
                let _ = settings.save();
            }
            Err(err) => {
                state.generation_error = Some(err.to_string());
            }
        }
        state.generation_busy = false;
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
    settings: &mut crate::settings::EditorSettings,
    windows: &mut Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    exit: &mut EventWriter<AppExit>,
) {
    egui::Area::new(egui::Id::new("window_controls"))
        .fixed_pos(egui::pos2(ctx.screen_rect().max.x - 140.0, 8.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
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

    let area = egui::Area::new(egui::Id::new("left_panel"))
        .fixed_pos(egui::pos2(layout.x, layout.y))
        .show(ctx, |ui| {
            studio_panel_frame(opacity, layout.corner_radius).show(ui, |ui| {
                ui.set_width(layout.width);
                ui.set_min_height(layout.height);
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
                ui.label("Presets");
                for preset in GenerationPreset::all() {
                    let active = preset.is_active();
                    let label = preset.label();
                    if active {
                        if ui
                            .selectable_label(state.profile.preset_id == preset.id(), label)
                            .clicked()
                        {
                            state.profile = preset.to_profile();
                        }
                    } else if inactive_button(ui, label).clicked() {
                        *dialog = unimplemented_action_response(StudioAction::InactivePreset(
                            preset.id().into(),
                        ));
                    }
                }
                ui.separator();
                generation_fields(ui, &mut state.profile, dialog);
                ui.separator();
                ui.label("Camera");
                if ui.button("Overhead (O)").clicked() {
                    snap_overhead(camera);
                }
                if ui.button("Reset (R)").clicked() {
                    reset_camera_after_generation(camera);
                }
                if let Some(err) = &state.generation_error {
                    ui.colored_label(egui::Color32::RED, err);
                } else if !state.status_message.is_empty() {
                    ui.label(&state.status_message);
                }
                ui.add_space(8.0);
                ui.label(egui::RichText::new("SimThing Studio").small().weak());
            });
        });
    state.left_panel_hovered = area.response.hovered();
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
    let report = session.report();
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
