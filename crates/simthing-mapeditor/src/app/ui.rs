#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::view::window::screenshot::{save_to_disk, Screenshot};
use bevy_egui::{egui, EguiContexts};

use crate::dialog::{
    inactive_control_warning, unimplemented_action_response, StudioAction, WarningDialogModel,
};
use crate::falloff_ruler_overlay::{draw_falloff_ruler_overlay, FalloffRulerOverlayParams};
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
use crate::studio_antialiasing::StudioAntialiasingModeSource;
use crate::studio_antialiasing::{apply_studio_antialiasing_mode, StudioAntialiasingMode};

use super::camera::{reset_camera_after_generation, snap_overhead, MainCamera, StudioCamera};
use super::galaxy_render::{
    apply_batched_galaxy_scene, apply_batched_galaxy_scene_cleanup, begin_batched_galaxy_scene,
    cancel_batched_galaxy_scene, finish_batched_galaxy_scene, loading_cover_active_for_phase,
    mark_hyperlane_render_dirty, mark_star_visual_render_dirty, phase_after_final_batch_complete,
    phase_after_parent_revealed, prepare_galaxy_scene, reveal_pending_galaxy_scene_parent,
    BatchedGalaxySceneBuild, PreparedGalaxyScene, SceneAdoptionVisibilityPhase, StarVisualAssets,
};
use super::scenario_io::{
    load_scenario_manual_path_action, open_native_scenario_load_picker, save_scenario_action,
    select_native_clause_scenario_path, ScenarioActionResult, ScenarioPickerActionResult,
};
use super::window::{minimize_window, set_window_mode};
use super::{adopt_loaded_scenario_session, adopt_session, GalaxySceneRoot, StudioAppState};
use crate::clause_scenario_picker::{
    run_clause_picker_action_staged, ClausePickerActionResult, ClausePickerSelection,
};
use crate::scenario_runtime_saveload_ui::{
    reopen_candidate_scenario_for_studio_session, save_candidate_scenario_for_studio_create_new,
};
use crate::session::StudioSession;
use crate::studio_frame_phase_gpu_telemetry::{
    apply_diagnostic_minimal_render, capture_normal_render_snapshot,
    restore_normal_render_from_snapshot, PerformanceDiagnosticFlags,
    DIAGNOSTIC_MINIMAL_RENDER_BUTTON, RESTORE_NORMAL_RENDER_BUTTON,
};
use crate::studio_performance_telemetry::{
    falloff_debug_lines, hyperlane_debug_lines, nameplate_debug_lines, performance_summary_lines,
    render_loop_gpu_vram_lines, video_options_debug_lines,
};
use crate::studio_render_loop_dirty_gate::StudioRenderLoopCaches;
use crate::studio_scenario_library_ui::{
    StudioLoaderStage, StudioLoaderStageEvent, StudioLoaderStageStatus,
};
use crate::studio_screenshot::next_screenshot_filename;
use crate::{create_blank_studio_session, StudioSimClockRate, StudioSimClockTransportCommand};

use super::performance_telemetry::{record_egui_pass_timing, StudioPerformanceTelemetryState};
use bevy::ecs::system::SystemParam;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(SystemParam)]
pub(super) struct StudioUiPresentationParams<'w> {
    perf: ResMut<'w, StudioPerformanceTelemetryState>,
    frosted: ResMut<'w, crate::FrostedGlassPanelRegistry>,
}

enum ClauseLoaderWorkerMessage {
    Stage {
        token: u64,
        event: StudioLoaderStageEvent,
    },
    Finished {
        token: u64,
        result: ClausePickerActionResult,
        prepared_scene: Option<PreparedGalaxyScene>,
        runtime_status: Option<crate::StudioScenarioRuntimeSaveLoadStatus>,
        scene_prepare_elapsed: Duration,
    },
}

pub(super) struct PendingClauseLoaderJob {
    token: u64,
    inbox: Arc<Mutex<VecDeque<ClauseLoaderWorkerMessage>>>,
}

pub(super) struct PendingClauseSceneAdoption {
    token: u64,
    session: Option<StudioSession>,
    source_path: Option<std::path::PathBuf>,
    message: String,
    runtime_status: Option<crate::StudioScenarioRuntimeSaveLoadStatus>,
    build: Option<BatchedGalaxySceneBuild>,
    elapsed_before_batches: Duration,
    batch_started: Instant,
    /// After resource swap; parent still Hidden until reveal frame.
    pending_parent_awaiting_reveal: Option<Entity>,
    phase: SceneAdoptionVisibilityPhase,
}

const SETTINGS_DIALOG_SIZE: egui::Vec2 = egui::vec2(420.0, 720.0);
const TELEMETRY_DIALOG_SIZE: egui::Vec2 = egui::vec2(420.0, 760.0);
const SETTINGS_TITLE_CLOSE_DRAG_GAP: f32 = 6.0;
const TELEMETRY_BUTTON_LABEL: &str = "Telemetry";
const TELEMETRY_TOOLTIP: &str = "Performance Telemetry";
const SETTINGS_BUTTON_LABEL: &str = "⚙";
const SETTINGS_TOOLTIP: &str = "Settings";
pub const DEFAULT_STELLARIS_STAR_NAMES_CORPUS_PATH: &str =
    r"C:\Users\mvorm\Clauser\Paradox\vanilla\common\random_names\base\00_random_names.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum GenerationNameCorpusChoice {
    Corpus(String),
    None,
    Cancel,
}

fn apply_generation_name_corpus_choice(
    profile: &mut GenerationProfile,
    dialog_visible: &mut bool,
    choice: GenerationNameCorpusChoice,
) -> bool {
    *dialog_visible = false;
    match choice {
        GenerationNameCorpusChoice::Corpus(path) => {
            profile.star_name_corpus_path = path.trim().to_string();
            true
        }
        GenerationNameCorpusChoice::None => {
            profile.star_name_corpus_path.clear();
            true
        }
        GenerationNameCorpusChoice::Cancel => false,
    }
}

/// Whether `path` is a Stellaris star-name corpus that can actually be read and
/// parsed. The OK button must check this itself: generation runs after the
/// dialog has already closed, so a bad path discovered there can no longer be
/// resolved by user interaction.
fn star_name_corpus_path_is_importable(path: &str) -> bool {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return false;
    }
    match std::fs::read(trimmed) {
        Ok(bytes) => simthing_clausething::parse_stellaris_star_name_catalog(&bytes).is_ok(),
        Err(_) => false,
    }
}
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

fn push_clause_loader_message(
    inbox: &Arc<Mutex<VecDeque<ClauseLoaderWorkerMessage>>>,
    message: ClauseLoaderWorkerMessage,
) {
    inbox
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .push_back(message);
}

fn start_clause_loader_job(state: &mut StudioAppState) {
    if state.scenario_library.is_loading() {
        return;
    }
    enforce_scenario_library_pause(state);
    // Opaque cover for the entire attempt (worker + hidden adopt + reveal frame).
    state.loading_cover_active = true;
    let token = state.scenario_library.begin_load_attempt();
    let selection = ClausePickerSelection {
        clause_path: state.scenario_library.path_text.trim().into(),
        resolver_entries: Default::default(),
        scenario_json_path: None,
    };
    let profile = Some(state.profile.clone());
    let star_falloff_settings = state.star_falloff_settings;
    let star_render_mode = state.star_render_mode;
    let hyperlane_render_settings = state.hyperlane_render_settings;
    let inbox = Arc::new(Mutex::new(VecDeque::new()));
    state.clause_loader_job = Some(PendingClauseLoaderJob {
        token,
        inbox: Arc::clone(&inbox),
    });

    std::thread::spawn(move || {
        let observer_inbox = Arc::clone(&inbox);
        let mut result = run_clause_picker_action_staged(&selection, profile, &mut |event| {
            push_clause_loader_message(
                &observer_inbox,
                ClauseLoaderWorkerMessage::Stage { token, event },
            );
        });
        let scene_prepare_started = Instant::now();
        let (prepared_scene, runtime_status) = match &mut result {
            ClausePickerActionResult::Loaded { session, .. } => {
                push_clause_loader_message(
                    &inbox,
                    ClauseLoaderWorkerMessage::Stage {
                        token,
                        event: StudioLoaderStageEvent::Running(StudioLoaderStage::SceneAdopt),
                    },
                );
                apply_star_falloff_settings_to_meta(
                    &mut session.view_model.render_meta,
                    star_falloff_settings,
                );
                apply_star_render_mode_to_meta(
                    &mut session.view_model.render_meta,
                    star_render_mode,
                );
                apply_hyperlane_render_settings_to_meta(
                    &mut session.view_model.render_meta,
                    hyperlane_render_settings,
                );
                let prepared = prepare_galaxy_scene(session);
                let status = crate::refresh_runtime_saveload_status_from_session(
                    "studio_loaded_session",
                    &session.scenario_authority,
                )
                .ok();
                (Some(prepared), status)
            }
            _ => (None, None),
        };
        push_clause_loader_message(
            &inbox,
            ClauseLoaderWorkerMessage::Finished {
                token,
                result,
                prepared_scene,
                runtime_status,
                scene_prepare_elapsed: scene_prepare_started.elapsed(),
            },
        );
    });
}

fn poll_clause_loader_jobs(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    settings: &mut super::resources::StudioSettings,
    camera: &mut StudioCamera,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &StarVisualAssets,
    scene_root: &mut GalaxySceneRoot,
    render_caches: &mut StudioRenderLoopCaches,
    presentation: &mut StudioUiPresentationParams,
) {
    if let Some(cleanup) = state.clause_scene_cleanup.first_mut() {
        if apply_batched_galaxy_scene_cleanup(commands, cleanup) {
            state.clause_scene_cleanup.remove(0);
        }
        ctx.request_repaint();
    }

    let pending_messages = state.clause_loader_job.as_ref().map(|job| {
        let mut inbox = job
            .inbox
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        inbox.drain(..).collect::<Vec<_>>()
    });
    let mut worker_finished = false;
    if let Some(messages) = pending_messages {
        for message in messages {
            match message {
                ClauseLoaderWorkerMessage::Stage { token, event } => {
                    state.scenario_library.observe_load_attempt(token, event);
                }
                ClauseLoaderWorkerMessage::Finished {
                    token,
                    result,
                    prepared_scene,
                    runtime_status,
                    scene_prepare_elapsed,
                } => {
                    worker_finished = true;
                    if !state.scenario_library.is_current_load_attempt(token) {
                        continue;
                    }
                    match result {
                        ClausePickerActionResult::Loaded {
                            session,
                            ingest,
                            message,
                        } => {
                            let Some(prepared_scene) = prepared_scene else {
                                let message =
                                    "ClauseScript open failed: scene preparation unavailable"
                                        .to_string();
                                state.last_scenario_io_status = message.clone();
                                state.status_message = message;
                                state.scenario_library.finish_load_attempt(token);
                                state.loading_cover_active = false;
                                continue;
                            };
                            state.clause_scene_adoption = Some(PendingClauseSceneAdoption {
                                token,
                                session: Some(session),
                                source_path: ingest.source_path,
                                message,
                                runtime_status,
                                build: Some(begin_batched_galaxy_scene(commands, prepared_scene)),
                                elapsed_before_batches: scene_prepare_elapsed,
                                batch_started: Instant::now(),
                                pending_parent_awaiting_reveal: None,
                                phase: SceneAdoptionVisibilityPhase::BuildingHidden,
                            });
                        }
                        ClausePickerActionResult::Failed { message }
                        | ClausePickerActionResult::InvalidPath { message } => {
                            state.last_scenario_io_status = message.clone();
                            state.status_message = message;
                            state.scenario_library.finish_load_attempt(token);
                            state.loading_cover_active = false;
                        }
                        ClausePickerActionResult::Cancelled => {
                            state.scenario_library.finish_load_attempt(token);
                            state.loading_cover_active = false;
                        }
                    }
                }
            }
        }
    }
    if worker_finished
        || state
            .clause_loader_job
            .as_ref()
            .is_some_and(|job| !state.scenario_library.is_current_load_attempt(job.token))
    {
        state.clause_loader_job = None;
    }

    if let Some(mut adoption) = state.clause_scene_adoption.take() {
        if !state
            .scenario_library
            .is_current_load_attempt(adoption.token)
        {
            // Stale/cancelled: never reveal pending geometry.
            if let Some(parent) = adoption.pending_parent_awaiting_reveal.take() {
                commands.entity(parent).despawn();
            } else if let Some(build) = adoption.build.take() {
                state
                    .clause_scene_cleanup
                    .push(cancel_batched_galaxy_scene(build));
            }
            state.loading_cover_active = false;
        } else if adoption.phase == SceneAdoptionVisibilityPhase::CommittedAwaitingReveal {
            // Reveal frame: one parent Visibility write; then drop cover and close modal.
            if let Some(parent) = adoption.pending_parent_awaiting_reveal.take() {
                reveal_pending_galaxy_scene_parent(
                    commands,
                    parent,
                    scene_root,
                    state.show_stars,
                    state.show_hyperlanes,
                );
            }
            adoption.phase = phase_after_parent_revealed(adoption.phase);
            state.loading_cover_active = false;
            state.scenario_library.finish_load_attempt(adoption.token);
            state.scenario_library.close();
            // Old committed scene cleanup already queued at commit step.
        } else if let Some(build) = adoption.build.as_mut() {
            if apply_batched_galaxy_scene(commands, meshes, materials, assets, build) {
                // Final batch complete: swap resource + adopt while parent stays Hidden.
                let elapsed = adoption.elapsed_before_batches + adoption.batch_started.elapsed();
                let build = adoption.build.take().expect("build present after complete");
                let (old_cleanup, pending_parent) = finish_batched_galaxy_scene(scene_root, build);
                state.clause_scene_cleanup.push(old_cleanup);
                let session = adoption.session.take().expect("session present for commit");
                adopt_loaded_scenario_session(session, settings, state, adoption.message.clone());
                request_live_bridge_reset_after_session_replacement(state);
                if let Some(status) = adoption.runtime_status.take() {
                    state
                        .apply_refreshed_runtime_saveload_status(status, Some(elapsed.as_millis()));
                }
                if let Some(source) = adoption.source_path.clone() {
                    state.clause_path_text = source.display().to_string();
                }
                mark_hyperlane_render_dirty(&mut render_caches.hyperlane);
                mark_star_visual_render_dirty(&mut render_caches.star_visual);
                render_caches.picking.last_key = None;
                render_caches.picking.cached_projections.clear();
                render_caches.billboard.last_camera_key = None;
                reset_camera_after_generation(camera);
                presentation.perf.vram_dirty = true;
                state.scenario_library.observe_load_attempt(
                    adoption.token,
                    StudioLoaderStageEvent::Passed {
                        stage: StudioLoaderStage::SceneAdopt,
                        elapsed,
                    },
                );
                adoption.pending_parent_awaiting_reveal = Some(pending_parent);
                adoption.phase = phase_after_final_batch_complete(adoption.phase);
                debug_assert!(loading_cover_active_for_phase(adoption.phase));
                // Keep cover + hold adoption for one reveal frame (modal still open / paused).
                state.clause_scene_adoption = Some(adoption);
            } else {
                state.clause_scene_adoption = Some(adoption);
            }
        }
    }

    if state.scenario_library.is_loading()
        || state.clause_scene_adoption.is_some()
        || !state.clause_scene_cleanup.is_empty()
    {
        ctx.request_repaint();
    }
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
    mut presentation: StudioUiPresentationParams,
    mut render_caches: ResMut<StudioRenderLoopCaches>,
    mut ctx_unavailable_logged: Local<bool>,
    main_camera: Query<Entity, With<MainCamera>>,
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
    poll_clause_loader_jobs(
        ctx,
        &mut state,
        &mut settings,
        &mut camera,
        &mut commands,
        &mut meshes,
        &mut materials,
        &assets,
        &mut scene_root,
        &mut render_caches,
        &mut presentation,
    );
    let screen = ctx.screen_rect();
    let screen_w = screen.width();
    let screen_h = screen.height();
    presentation.frosted.begin_frame();

    // Opaque world cover behind modal/telemetry, above 3D scene (OVL: no starmap while loading).
    if state.loading_cover_active
        || state.scenario_library.is_loading()
        || state.clause_scene_adoption.is_some()
    {
        draw_studio_loading_cover(ctx, screen);
    }

    if should_auto_collapse_panel(screen_w) {
        state.left_panel_collapsed = true;
    }
    enforce_scenario_library_pause(&mut state);

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
                &mut presentation.frosted,
            );
            left_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        } else {
            let panel_started = std::time::Instant::now();
            draw_collapsed_tab(ctx, &mut state, screen_w, screen_h);
            left_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        }
        if state.session.is_some() {
            let panel_started = std::time::Instant::now();
            draw_right_panel(
                ctx,
                &mut state,
                screen_w,
                screen_h,
                &mut presentation.frosted,
            );
            right_panel_ms = panel_started.elapsed().as_secs_f64() * 1000.0;
        }
    }
    let settings_started = std::time::Instant::now();
    draw_settings_dialog(
        ctx,
        &mut state,
        &mut settings,
        &mut render_caches,
        &mut commands,
        &main_camera,
        screen_w,
        screen_h,
        &mut presentation.frosted,
    );
    let settings_ms = settings_started.elapsed().as_secs_f64() * 1000.0;
    let telemetry_started = std::time::Instant::now();
    draw_telemetry_dialog(
        ctx,
        &mut state,
        &mut settings,
        &mut commands,
        &presentation.perf.telemetry,
        &mut render_caches,
        screen_w,
        screen_h,
        &mut presentation.frosted,
    );
    draw_studio_ops_telemetry(ctx, &mut state);
    let telemetry_ms = telemetry_started.elapsed().as_secs_f64() * 1000.0;
    if state.show_falloff_ruler
        && state.star_falloff_metric == crate::star_render::StarFalloffMetric::VisualHorizon
    {
        let nameplate_settings = settings.star_nameplate_settings().clamped();
        draw_falloff_ruler_overlay(
            ctx,
            FalloffRulerOverlayParams {
                viewport_width: screen_w,
                viewport_height: screen_h,
                star_falloff_percent: state.star_falloff_settings.falloff_distance_percent,
                nameplate_relative_falloff_percent: nameplate_settings
                    .relative_falloff_distance_percent,
            },
        );
    }
    if !state.performance_diagnostic_hide_panels {
        draw_warning_dialog(ctx, &mut dialog);
    }
    draw_scenario_library_dialog(
        ctx,
        &mut state,
        screen_w,
        screen_h,
        &mut presentation.frosted,
    );
    draw_generation_name_corpus_dialog(ctx, &mut state);
    record_egui_pass_timing(
        &mut presentation.perf,
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
                    presentation.perf.vram_dirty = true;
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
                request_live_bridge_reset_after_session_replacement(&mut state);
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
                presentation.perf.vram_dirty = true;
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
                request_live_bridge_reset_after_session_replacement(&mut state);
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
                presentation.perf.vram_dirty = true;
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
                request_live_bridge_reset_after_session_replacement(&mut state);
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
                presentation.perf.vram_dirty = true;
            }
            _ => {}
        }
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_select_clause_scenario"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_select_clause_scenario")));
        let _ = select_native_clause_scenario_path(&mut state);
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_load_clause_scenario"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_load_clause_scenario")));
        start_clause_loader_job(&mut state);
    }

    if ctx.data(|d| {
        d.get_temp::<bool>(egui::Id::new("do_create_blank_scenario"))
            .unwrap_or(false)
    }) {
        ctx.data_mut(|d| d.remove::<bool>(egui::Id::new("do_create_blank_scenario")));
        let requested_id = state.scenario_library.create_scenario_id.clone();
        match create_blank_studio_session(&requested_id) {
            Ok(session) => {
                let save_path = session.scenario_path.clone();
                let message = format!("Created blank scenario: {}", session.galaxy_name());
                adopt_loaded_scenario_session(session, &mut settings, &mut state, message);
                request_live_bridge_reset_after_session_replacement(&mut state);
                if let Some(path) = save_path {
                    state.scenario_path_text = path.display().to_string();
                }
                enforce_scenario_library_pause(&mut state);
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
                presentation.perf.vram_dirty = true;
            }
            Err(err) => {
                state.last_scenario_io_status = format!("Scenario create failed: {err}");
                state.status_message = state.last_scenario_io_status.clone();
            }
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

fn register_frosted_rect(
    registry: &mut crate::FrostedGlassPanelRegistry,
    rect: egui::Rect,
    screen_w: f32,
    screen_h: f32,
) {
    registry.register_logical_rect(
        [rect.min.x, rect.min.y],
        [rect.max.x, rect.max.y],
        [screen_w, screen_h],
    );
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
}

fn draw_settings_dialog(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
    commands: &mut Commands,
    main_camera: &Query<Entity, With<MainCamera>>,
    screen_w: f32,
    screen_h: f32,
    frosted_panels: &mut crate::FrostedGlassPanelRegistry,
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

    let area = egui::Area::new(egui::Id::new("settings_dialog"))
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
                    .on_hover_text(
                        "Percent of current map-radius view range that remains at base star visibility.",
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
                        .text("Nameplate Relative Size"),
                    )
                    .on_hover_text(
                        "Uniform scale relative to rendered star blur/visual width. 100% = label height equals star blur width; text width preserves natural aspect.",
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
                    .on_hover_text(
                        "Percent of Star Falloff Distance for which nameplates remain at base label visibility.",
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
                ui.separator();
                ui.label(egui::RichText::new("Antialiasing").strong());
                let mut aa_mode = state.antialiasing_mode;
                let mut aa_changed = false;
                for candidate in StudioAntialiasingMode::ALL {
                    aa_changed |= ui
                        .radio_value(&mut aa_mode, candidate, candidate.label())
                        .changed();
                }
                if aa_changed {
                    apply_antialiasing_settings(aa_mode, state, settings, commands, main_camera);
                }
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        reset_settings_dialog_values(
                            state,
                            settings,
                            render_caches,
                            commands,
                            main_camera,
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            close_settings_dialog_from_button(state, settings);
                        }
                    });
                });
            });
        });
    register_frosted_rect(frosted_panels, area.response.rect, screen_w, screen_h);
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
    frosted_panels: &mut crate::FrostedGlassPanelRegistry,
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

    let area = egui::Area::new(egui::Id::new("telemetry_dialog"))
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
                draw_telemetry_scenario_section(ctx, ui, state);
                if ui.button("Show Studio_ops Telemetry").clicked() {
                    state.scenario_library.toggle_studio_ops_telemetry();
                }
                ui.separator();
                egui::CollapsingHeader::new("Nameplate debug")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Nameplate mode:");
                            egui::ComboBox::from_id_salt("nameplate_debug_mode")
                                .selected_text(state.star_nameplate_debug_mode.label())
                                .show_ui(ui, |ui| {
                                    for mode in [
                                        crate::star_render::StarNameplateDebugMode::AllLabelsSettingsDriven,
                                        crate::star_render::StarNameplateDebugMode::AutoLodDebug,
                                        crate::star_render::StarNameplateDebugMode::FocusedOnlyDebug,
                                        crate::star_render::StarNameplateDebugMode::ForceAllDebug,
                                    ] {
                                        ui.selectable_value(
                                            &mut state.star_nameplate_debug_mode,
                                            mode,
                                            mode.label(),
                                        );
                                    }
                                });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Falloff metric:");
                            egui::ComboBox::from_id_salt("star_falloff_metric")
                                .selected_text(state.star_falloff_metric.label())
                                .show_ui(ui, |ui| {
                                    for metric in [
                                        crate::star_render::StarFalloffMetric::MapRadiusPlateau,
                                        crate::star_render::StarFalloffMetric::VisualHorizon,
                                        crate::star_render::StarFalloffMetric::CameraDistanceDebug,
                                    ] {
                                        ui.selectable_value(
                                            &mut state.star_falloff_metric,
                                            metric,
                                            metric.label(),
                                        );
                                    }
                                });
                        });
                        if state.star_nameplate_debug_mode.is_debug_override() {
                            ui.label(
                                "Debug mode — optional LOD/readability assist; Settings sliders remain authoritative in All labels mode.",
                            );
                        }
                        if state.star_nameplate_debug_mode.is_force_all_debug() {
                            ui.colored_label(
                                egui::Color32::YELLOW,
                                "DEBUG: Force all bypasses offscreen/LOD culls; Settings falloff/alpha still apply.",
                            );
                        }
                        for line in nameplate_debug_lines(telemetry) {
                            ui.label(line);
                        }
                    });
                egui::CollapsingHeader::new("Falloff debug")
                    .default_open(false)
                    .show(ui, |ui| {
                        for line in falloff_debug_lines(telemetry) {
                            ui.label(line);
                        }
                    });
                egui::CollapsingHeader::new("Hyperlane debug")
                    .default_open(false)
                    .show(ui, |ui| {
                        for line in hyperlane_debug_lines(telemetry) {
                            ui.label(line);
                        }
                    });
                egui::CollapsingHeader::new("Video Options Debug")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label(
                            "Confirms selected AA mode and active Bevy camera components (FXAA/SMAA/MSAA). Visual quality still requires owner screenshot comparison.",
                        );
                        ui.checkbox(&mut state.show_aa_test_pattern, "Show AA test pattern");
                        if state.show_aa_test_pattern {
                            ui.label(
                                "Compare Off vs MSAA 4x/8x on the diagonal test pattern, not on text labels.",
                            );
                        }
                        for line in video_options_debug_lines(telemetry) {
                            if line == "AA STATE MISMATCH" {
                                ui.colored_label(egui::Color32::YELLOW, &line);
                            } else {
                                ui.label(line);
                            }
                        }
                    });
                egui::CollapsingHeader::new("Performance summary")
                    .default_open(false)
                    .show(ui, |ui| {
                        for line in performance_summary_lines(telemetry) {
                            ui.label(line);
                        }
                    });
                egui::CollapsingHeader::new("Performance isolation")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.checkbox(&mut state.show_falloff_ruler, "Show falloff ruler");
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
                                apply_performance_diagnostic_minimal_render(
                                    state,
                                    settings,
                                    render_caches,
                                );
                            }
                            if ui.button(RESTORE_NORMAL_RENDER_BUTTON).clicked() {
                                restore_performance_normal_render(state, settings, render_caches);
                            }
                        });
                    });
                egui::CollapsingHeader::new("Render loop / GPU / VRAM")
                    .default_open(false)
                    .show(ui, |ui| {
                        for line in render_loop_gpu_vram_lines(telemetry) {
                            ui.label(line);
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
    register_frosted_rect(frosted_panels, area.response.rect, screen_w, screen_h);
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
    super::persist_presentation_settings(state, settings, None);
    if state.status_message.is_empty() {
        state.status_message = "Settings saved".into();
    }
}

fn reset_settings_dialog_values(
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    render_caches: &mut StudioRenderLoopCaches,
    commands: &mut Commands,
    main_camera: &Query<Entity, With<MainCamera>>,
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
    state.antialiasing_mode = defaults.antialiasing_mode;
    state.antialiasing_mode_source = StudioAntialiasingModeSource::DefaultFallback;
    settings.set_star_falloff_settings(star);
    settings.set_star_render_mode(mode);
    settings.set_star_nameplate_settings(nameplate);
    settings.set_hyperlane_render_settings(hyperlane);
    settings.set_antialiasing_mode(defaults.antialiasing_mode);
    if let Some(session) = state.session.as_mut() {
        apply_star_falloff_settings_to_meta(&mut session.view_model.render_meta, star);
        apply_star_render_mode_to_meta(&mut session.view_model.render_meta, mode);
        apply_hyperlane_render_settings_to_meta(&mut session.view_model.render_meta, hyperlane);
    }
    mark_hyperlane_render_dirty(&mut render_caches.hyperlane);
    mark_star_visual_render_dirty(&mut render_caches.star_visual);
    if let Ok(entity) = main_camera.single() {
        apply_studio_antialiasing_mode(&mut commands.entity(entity), state.antialiasing_mode);
    }
    super::persist_presentation_settings(state, settings, None);
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
    super::persist_presentation_settings(state, settings, None);
}

fn apply_antialiasing_settings(
    mode: StudioAntialiasingMode,
    state: &mut StudioAppState,
    settings: &mut crate::settings::EditorSettings,
    commands: &mut Commands,
    main_camera: &Query<Entity, With<MainCamera>>,
) {
    let mode = mode.normalize();
    state.antialiasing_mode = mode;
    state.antialiasing_mode_source = StudioAntialiasingModeSource::CurrentUiState;
    settings.set_antialiasing_mode(mode);
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    if let Ok(entity) = main_camera.single() {
        apply_studio_antialiasing_mode(&mut commands.entity(entity), mode);
    }
    state.status_message = format!("Antialiasing: {}", mode.label());
    super::persist_presentation_settings(state, settings, None);
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
    super::persist_presentation_settings(state, settings, None);
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
    super::persist_presentation_settings(state, settings, None);
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
    frosted_panels: &mut crate::FrostedGlassPanelRegistry,
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
                            if ui.button("Library...").clicked() {
                                open_scenario_library(state);
                            }
                            if ui.button("Generate").clicked() {
                                state.generation_name_corpus_path =
                                    DEFAULT_STELLARIS_STAR_NAMES_CORPUS_PATH.to_string();
                                state.generation_name_dialog_visible = true;
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
                        egui::CollapsingHeader::new("Sim clock transport")
                            .id_salt("left_panel_sim_clock")
                            .default_open(true)
                            .show(ui, |ui| {
                                draw_sim_clock_transport(ui, state);
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
    register_frosted_rect(frosted_panels, area.response.rect, screen_w, screen_h);
}

/// Compact operator transport over [`crate::StudioSimClockTransport`] + live bridge readout.
fn draw_sim_clock_transport(ui: &mut egui::Ui, state: &mut StudioAppState) {
    {
        let transport = &mut state.sim_clock_transport;
        let readout = transport.readout();

        ui.label(egui::RichText::new("Sim clock (transport only)").strong());
        ui.label(
            egui::RichText::new("Schedules admitted ticks; bridge executes via SimSession.")
                .small()
                .weak(),
        );

        ui.horizontal(|ui| {
            if ui
                .add_enabled(readout.playing, egui::Button::new("Pause"))
                .clicked()
            {
                let _ = transport.apply(StudioSimClockTransportCommand::Pause);
            }
            if ui
                .add_enabled(readout.paused, egui::Button::new("Play"))
                .clicked()
            {
                let _ = transport.apply(StudioSimClockTransportCommand::Play);
            }
        });

        ui.horizontal(|ui| {
            for (label, rate, cmd) in [
                (
                    "1×",
                    StudioSimClockRate::Rate1x,
                    StudioSimClockTransportCommand::Rate1x,
                ),
                (
                    "2×",
                    StudioSimClockRate::Rate2x,
                    StudioSimClockTransportCommand::Rate2x,
                ),
                (
                    "4×",
                    StudioSimClockRate::Rate4x,
                    StudioSimClockTransportCommand::Rate4x,
                ),
            ] {
                let selected = readout.rate == rate;
                if ui.selectable_label(selected, label).clicked() {
                    let _ = transport.apply(cmd);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("Max TPS");
            let response = ui.text_edit_singleline(transport.max_tps_draft_mut());
            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let _ = transport.apply_max_tps_draft();
            }
            if ui.button("Apply TPS").clicked() {
                let _ = transport.apply_max_tps_draft();
            }
        });

        let readout = transport.readout();
        let state_label = if readout.paused { "paused" } else { "playing" };
        ui.label(format!(
            "State: {state_label}  ·  Rate: {}  ·  Max TPS: {:.3}  ·  Effective: {:.3}/s  ·  Tick: {}",
            readout.rate_label, readout.max_tps, readout.effective_tps, readout.tick_index
        ));
        if let Some(err) = transport.last_error() {
            ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
        }
    }

    // Live session bridge status (9.3) — snapshot from NonSend bridge system.
    let bridge = &state.live_bridge_readout;
    ui.label(format!(
        "Live bridge: {}  ·  Executed: {}  ·  Last batch: {}",
        bridge.status_label, bridge.executed_ticks, bridge.last_scheduled_batch
    ));
    ui.label(egui::RichText::new(bridge.production_path).small().weak());
    if let Some(err) = bridge.last_error.as_deref() {
        ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
    }

    // Live observation (9.4) — pure projection; does not tick or mutate Spec.
    draw_live_observation(ui, state);
}

/// Compact observation panel over clock + bridge + session (presentation only).
fn draw_live_observation(ui: &mut egui::Ui, state: &StudioAppState) {
    let clock = state.sim_clock_transport.readout();
    let obs = crate::build_studio_live_observation_readout(
        &clock,
        &state.live_bridge_readout,
        state.session.as_ref(),
    );

    ui.separator();
    ui.label(egui::RichText::new("Live observation").strong());
    ui.label(
        egui::RichText::new("Read-only projection; freezes with pause; never steps the sim.")
            .small()
            .weak(),
    );

    let clock_state = if obs.clock_paused {
        "paused"
    } else {
        "playing"
    };
    ui.label(format!(
        "Clock: {clock_state}  ·  Rate: {}  ·  Max TPS: {:.3}  ·  Eff: {:.3}/s  ·  Scheduled tick: {}",
        obs.clock_rate_label, obs.max_tps, obs.effective_tps, obs.scheduled_tick_index
    ));
    ui.label(format!(
        "Live bridge: {}  ·  Executed: {}  ·  Last batch: {}",
        obs.bridge_status_label, obs.bridge_executed_ticks, obs.bridge_last_scheduled_batch
    ));
    if let Some(err) = obs.bridge_last_error.as_deref() {
        ui.colored_label(
            egui::Color32::from_rgb(220, 80, 80),
            format!("Bridge status: {err}"),
        );
    }

    if obs.session_loaded {
        let scenario = obs.scenario_id.as_deref().unwrap_or("(unknown)");
        let systems = obs.system_count.unwrap_or(0);
        let links = obs.link_count.unwrap_or(0);
        let stead = match obs.stead_valid {
            Some(true) => "STEAD valid",
            Some(false) => "STEAD invalid",
            None => "STEAD n/a",
        };
        let rf = match obs.rf_ready {
            Some(true) => "RF ready",
            Some(false) => "RF not ready",
            None => "RF n/a",
        };
        let occupied = obs.occupied_cells.unwrap_or(0);
        ui.label(format!(
            "Session: {scenario}  ·  {}  ·  systems: {systems}  ·  links: {links}",
            obs.source_kind_label
        ));
        ui.label(format!(
            "Summary: {stead}  ·  {rf}  ·  occupied cells: {occupied}"
        ));
        if let Some(msg) = obs.session_status_message.as_deref() {
            ui.label(egui::RichText::new(msg).small().weak());
        }
    } else {
        ui.label("Session: no loaded session  ·  bridge unattached/idle");
    }
}

/// Read-only Scenario section plus existing debug/operator actions rehomed from the load modal.
fn draw_telemetry_scenario_section(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    state: &mut StudioAppState,
) {
    let clock = state.sim_clock_transport.readout();
    let telemetry = crate::build_studio_scenario_telemetry_readout(
        state.session.as_ref(),
        &state.clause_path_text,
        &clock,
    );
    egui::CollapsingHeader::new("Scenario")
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Scenario id: {}", telemetry.scenario_id));
            ui.label(format!("Clause path: {}", telemetry.clause_path));
            ui.label(format!("Source path: {}", telemetry.source_path));
            ui.label(format!(
                "Source resolution: {}",
                telemetry.source_resolution
            ));
            ui.label(format!("Resolver: {}", telemetry.resolver_state));
            ui.label(format!(
                "Systems: {}  ·  Owners: {}  ·  {}",
                telemetry.system_count, telemetry.owner_count, telemetry.stead_label
            ));
            let play = if telemetry.paused {
                "paused"
            } else {
                "playing"
            };
            ui.label(format!("Clock: {play}  ·  Tick: {}", telemetry.tick_index));
            ui.separator();
            ui.label(egui::RichText::new("Debug scenario actions").strong());
            ui.horizontal(|ui| {
                ui.label("Scenario path:");
                ui.text_edit_singleline(&mut state.scenario_path_text);
            });
            ui.horizontal(|ui| {
                if ui.button("Load JSON path").clicked() {
                    let _ = state
                        .sim_clock_transport
                        .apply(StudioSimClockTransportCommand::Pause);
                    ctx.data_mut(|data| {
                        data.insert_temp(egui::Id::new("do_load_scenario_manual"), true)
                    });
                }
                if ui.button("Select JSON file").clicked() {
                    let _ = state
                        .sim_clock_transport
                        .apply(StudioSimClockTransportCommand::Pause);
                    ctx.data_mut(|data| {
                        data.insert_temp(egui::Id::new("do_load_scenario_picker"), true)
                    });
                }
                if ui.button("Save JSON").clicked() {
                    ctx.data_mut(|data| {
                        data.insert_temp(
                            egui::Id::new("do_save_scenario"),
                            std::path::PathBuf::from(&state.scenario_path_text),
                        )
                    });
                }
            });
            ui.horizontal(|ui| {
                ui.label("Blank scenario id:");
                ui.text_edit_singleline(&mut state.scenario_library.create_scenario_id);
                if ui.button("Create blank scenario").clicked() {
                    let _ = state
                        .sim_clock_transport
                        .apply(StudioSimClockTransportCommand::Pause);
                    ctx.data_mut(|data| {
                        data.insert_temp(egui::Id::new("do_create_blank_scenario"), true)
                    });
                }
            });
            ui.separator();
            draw_runtime_candidate_saveload_controls(ctx, ui, state);
        });
}

fn draw_studio_ops_telemetry(ctx: &egui::Context, state: &mut StudioAppState) {
    if !state.scenario_library.studio_ops_telemetry_visible {
        return;
    }
    let mut visible = true;
    egui::Window::new("Studio_ops Telemetry")
        .open(&mut visible)
        .resizable(true)
        .show(ctx, |ui| {
            egui::Grid::new("studio_ops_loader_stages")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    for stage in StudioLoaderStage::ALL {
                        let record = state.scenario_library.load_progress.record(stage);
                        ui.label(stage.label());
                        ui.label(record.status.label());
                        ui.label(
                            record
                                .elapsed
                                .map(|elapsed| format!("{:.3} ms", elapsed.as_secs_f64() * 1000.0))
                                .unwrap_or_else(|| "-- ms".into()),
                        );
                        ui.end_row();
                        if let Some(failure) = record.failure.as_deref() {
                            ui.colored_label(egui::Color32::RED, "Failure");
                            ui.colored_label(egui::Color32::RED, failure);
                            ui.label("");
                            ui.end_row();
                        }
                    }
                });

            // [OVL] STUDIO-FIELD-SESSION-ELEVATE-0 — session path + field accretion samples.
            ui.separator();
            ui.heading("Live session path");
            let bridge = &state.live_bridge_readout;
            let clock = state.sim_clock_transport.readout();
            egui::Grid::new("studio_ops_session_path")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("scenario");
                    ui.label(
                        state
                            .session
                            .as_ref()
                            .and_then(|session| session.scenario_path.as_ref())
                            .map(|path| path.display().to_string())
                            .or_else(|| bridge.scenario_id.clone())
                            .unwrap_or_else(|| "(none)".into()),
                    );
                    ui.end_row();
                    ui.label("tick / transport");
                    ui.label(format!(
                        "{} / {}",
                        bridge.executed_ticks,
                        if clock.paused { "paused" } else { "playing" }
                    ));
                    ui.end_row();
                    ui.label("session path");
                    ui.label(bridge.session_path_label);
                    ui.end_row();
                    ui.label("path preference");
                    ui.label(bridge.path_preference_label);
                    ui.end_row();
                    ui.label("production path");
                    ui.label(bridge.production_path);
                    ui.end_row();
                    ui.label("executed ticks");
                    ui.label(format!("{}", bridge.executed_ticks));
                    ui.end_row();
                    ui.label("decision events (last / cumulative)");
                    ui.label(format!(
                        "{} / {}",
                        bridge.last_decision_event_count, bridge.cumulative_decision_events
                    ));
                    ui.end_row();
                });

            ui.separator();
            ui.heading("Recursive Arena Resource Flow");
            let recursive = &bridge.recursive_rf;
            egui::Grid::new("studio_ops_recursive_rf")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("profile / activity");
                    ui.label(format!(
                        "{} / {}",
                        recursive.execution_profile,
                        if recursive.active {
                            "active"
                        } else {
                            "inactive"
                        }
                    ));
                    ui.end_row();
                    ui.label("arena");
                    ui.label(recursive.arena.as_deref().unwrap_or("--"));
                    ui.end_row();
                    ui.label("named child");
                    ui.label(recursive.named_child.as_deref().unwrap_or("--"));
                    ui.end_row();
                    ui.label("real ancestor / siblings");
                    ui.label(format!(
                        "{} / {} siblings",
                        recursive.ancestor.as_deref().unwrap_or("--"),
                        recursive.sibling_count
                    ));
                    ui.end_row();
                    ui.label("ancestor aggregate (loaded / live)");
                    ui.label(format!(
                        "{} / {}",
                        recursive
                            .ancestor_aggregate_before
                            .map(|value| format!("{value:.6}"))
                            .unwrap_or_else(|| "--".into()),
                        recursive
                            .ancestor_aggregate_after
                            .map(|value| format!("{value:.6}"))
                            .unwrap_or_else(|| "--".into()),
                    ));
                    ui.end_row();
                    ui.label("need / weight_profile");
                    ui.label(
                        recursive
                            .need_profile_id
                            .as_deref()
                            .map(|id| {
                                format!(
                                    "{id} / {} weights=[{}] live={} thr={} status={} field_policy_events={}",
                                    recursive.need_profile_kind.as_deref().unwrap_or("--"),
                                    recursive.need_weight_values.as_deref().unwrap_or("--"),
                                    recursive
                                        .need_live_value
                                        .map(|v| format!("{v:.6}"))
                                        .unwrap_or_else(|| "--".into()),
                                    recursive
                                        .need_threshold
                                        .map(|v| format!("{v:.3}"))
                                        .unwrap_or_else(|| "--".into()),
                                    recursive.need_threshold_result.unwrap_or("--"),
                                    recursive.need_threshold_event_count,
                                )
                            })
                            .unwrap_or_else(|| "not bound (no admitted GameMode binding)".into()),
                    );
                    ui.end_row();
                });

            // [OVL] STUDIO-DISRUPTION-SELECT-SCREEN-0 — selected-star disruption screen.
            ui.separator();
            ui.heading("Selected-star disruption screen");
            let screen = crate::selected_disruption_select_screen(
                state.selection.selected_system_id,
                &bridge.disruption_readout,
            );
            egui::Grid::new("studio_ops_disruption_select_screen")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("selected system id");
                    ui.label(
                        state
                            .selection
                            .selected_system_id
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "--".into()),
                    );
                    ui.end_row();
                    ui.label("raw disruption");
                    ui.label(if state.selection.selected_system_id.is_some() {
                        format!("{:.3}", screen.raw_disruption)
                    } else {
                        "--".into()
                    });
                    ui.end_row();
                    ui.label("blur scale");
                    ui.label(if state.selection.selected_system_id.is_some() {
                        format!("{:.3}", screen.blur_scale)
                    } else {
                        "--".into()
                    });
                    ui.end_row();
                    ui.label("red fraction");
                    ui.label(if state.selection.selected_system_id.is_some() {
                        format!("{:.3}", screen.red_fraction)
                    } else {
                        "--".into()
                    });
                    ui.end_row();
                });

            // [OVL] TP-EMERGENT-TENSION-PROOF-0 — read-only per-owner macro gauges
            // projected from exact admitted property keys (no substring / Studio mutation).
            ui.separator();
            ui.heading("Per-owner macro gauges");
            let latest_exact = |property_key: &str| {
                bridge
                    .field_accretion_samples
                    .iter()
                    .rev()
                    .find(|sample| sample.property_key == property_key)
                    .map(|sample| (sample.tick_index, sample.amount))
            };
            let terran_production =
                latest_exact("tp_economy::terran_shipyard_hulls_quantity");
            let terran_suppression =
                latest_exact("tp_economy::terran_shipyard_disrupted_hulls_quantity");
            let pirate_disruption =
                latest_exact("tp_economy::pirate_outpost_disruption_presence");
            let construction = &bridge.recursive_rf;
            egui::Grid::new("studio_ops_owner_macro_gauges")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("terran production (hulls)");
                    ui.label(
                        terran_production
                            .map(|(tick, amount)| format!("tick {tick}: {amount:.3}"))
                            .unwrap_or_else(|| "--".into()),
                    );
                    ui.end_row();
                    ui.label("terran suppression (disrupted_hulls)");
                    ui.label(
                        terran_suppression
                            .map(|(tick, amount)| format!("tick {tick}: {amount:.3}"))
                            .unwrap_or_else(|| "--".into()),
                    );
                    ui.end_row();
                    ui.label("pirate disruption (presence)");
                    ui.label(
                        pirate_disruption
                            .map(|(tick, amount)| format!("tick {tick}: {amount:.3}"))
                            .unwrap_or_else(|| "--".into()),
                    );
                    ui.end_row();
                    ui.label("construction need profile");
                    ui.label(format!(
                        "id={} kind={}",
                        construction.need_profile_id.as_deref().unwrap_or("--"),
                        construction.need_profile_kind.as_deref().unwrap_or("--"),
                    ));
                    ui.end_row();
                    ui.label("construction live / threshold / last");
                    ui.label(format!(
                        "{} / {} / {}",
                        construction
                            .need_live_value
                            .map(|v| format!("{v:.6}"))
                            .unwrap_or_else(|| "--".into()),
                        construction
                            .need_threshold
                            .map(|v| format!("{v:.3}"))
                            .unwrap_or_else(|| "--".into()),
                        construction.need_threshold_result.unwrap_or("--"),
                    ));
                    ui.end_row();
                    ui.label("construction crossings (event_kind)");
                    ui.label(format!(
                        "last_tick={} cumulative={}",
                        construction.need_threshold_event_count,
                        bridge.cumulative_construction_crossings,
                    ));
                    ui.end_row();
                });

            ui.separator();
            ui.heading("Field accretion samples");
            if bridge.field_accretion_samples.is_empty() {
                ui.label(
                    egui::RichText::new("(no field-bearing samples yet)")
                        .small()
                        .weak(),
                );
            } else {
                egui::Grid::new("studio_ops_field_accretion")
                    .num_columns(4)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("tick");
                        ui.label("property");
                        ui.label("amount");
                        ui.label("decisions");
                        ui.end_row();
                        for sample in &bridge.field_accretion_samples {
                            ui.label(format!("{}", sample.tick_index));
                            ui.label(&sample.property_key);
                            ui.label(format!("{:.3}", sample.amount));
                            ui.label(format!("{}", sample.decision_events));
                            ui.end_row();
                        }
                    });
            }
        });
    state.scenario_library.studio_ops_telemetry_visible = visible;
}

/// Fully opaque world-area cover while a load attempt is active.
/// Drawn under modal/telemetry layers so progress UI remains visible (source-shape OVL proof).
fn draw_studio_loading_cover(ctx: &egui::Context, screen: egui::Rect) {
    // Order::Middle sits above the central panel / 3D view but below Foreground modals.
    egui::Area::new(egui::Id::new("studio_loading_cover"))
        .order(egui::Order::Middle)
        .fixed_pos(screen.min)
        .interactable(false)
        .show(ctx, |ui| {
            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_rgb(6, 8, 12));
        });
}

fn draw_scenario_library_dialog(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    screen_w: f32,
    screen_h: f32,
    frosted_panels: &mut crate::FrostedGlassPanelRegistry,
) {
    if !state.scenario_library.visible {
        return;
    }

    enforce_scenario_library_pause(state);
    let response = egui::Modal::new(egui::Id::new("scenario_library_modal"))
        .frame(studio_panel_frame(0.88, 10.0))
        .show(ctx, |ui| {
            ui.set_min_width(520.0);
            ui.heading("Load ClauseScript Scenario");
            let loading = state.scenario_library.is_loading();
            ui.horizontal(|ui| {
                ui.label("Scenario path:");
                ui.add_enabled(
                    !loading,
                    egui::TextEdit::singleline(&mut state.scenario_library.path_text),
                );
            });
            let cancel = ui
                .horizontal(|ui| {
                    if ui
                        .add_enabled(!loading, egui::Button::new("Select File…"))
                        .clicked()
                    {
                        ctx.data_mut(|data| {
                            data.insert_temp(egui::Id::new("do_select_clause_scenario"), true)
                        });
                    }
                    if ui
                        .add_enabled(!loading, egui::Button::new("Load"))
                        .clicked()
                    {
                        ctx.data_mut(|data| {
                            data.insert_temp(egui::Id::new("do_load_clause_scenario"), true)
                        });
                    }
                    ui.button("Cancel").clicked()
                })
                .inner;
            if state.scenario_library.load_progress.visible {
                ui.separator();
                let progress = &state.scenario_library.load_progress;
                let active = StudioLoaderStage::ALL
                    .into_iter()
                    .find(|stage| {
                        matches!(
                            progress.record(*stage).status,
                            StudioLoaderStageStatus::Running | StudioLoaderStageStatus::Failed
                        )
                    })
                    .or_else(|| {
                        StudioLoaderStage::ALL.into_iter().rev().find(|stage| {
                            progress.record(*stage).status == StudioLoaderStageStatus::Passed
                        })
                    });
                let text = active
                    .map(|stage| {
                        let record = progress.record(stage);
                        let mut text = format!("{}: {}", stage.label(), record.status.label());
                        if let Some(failure) = record.failure.as_deref() {
                            text.push_str(&format!(" - {failure}"));
                        }
                        text
                    })
                    .unwrap_or_else(|| "Preparing load".into());
                ui.add(egui::ProgressBar::new(progress.completed_fraction()).text(text));
            }
            cancel
        });

    register_frosted_rect(frosted_panels, response.response.rect, screen_w, screen_h);

    if response.inner || response.should_close() {
        cancel_scenario_library(state);
        clear_scenario_library_pending_actions(ctx);
    }
}

fn clear_scenario_library_pending_actions(ctx: &egui::Context) {
    ctx.data_mut(|data| {
        data.remove::<std::path::PathBuf>(egui::Id::new("do_save_scenario"));
        data.remove::<bool>(egui::Id::new("do_load_scenario_manual"));
        data.remove::<bool>(egui::Id::new("do_load_scenario_picker"));
        data.remove::<bool>(egui::Id::new("do_open_clause_scenario_picker"));
        data.remove::<bool>(egui::Id::new("do_select_clause_scenario"));
        data.remove::<bool>(egui::Id::new("do_load_clause_scenario"));
        data.remove::<bool>(egui::Id::new("do_create_blank_scenario"));
    });
}

fn open_scenario_library(state: &mut StudioAppState) {
    let StudioAppState {
        scenario_library,
        sim_clock_transport,
        ..
    } = state;
    scenario_library.open(sim_clock_transport);
}

fn enforce_scenario_library_pause(state: &mut StudioAppState) {
    let StudioAppState {
        scenario_library,
        sim_clock_transport,
        ..
    } = state;
    scenario_library.enforce_pause(sim_clock_transport);
}

fn cancel_scenario_library(state: &mut StudioAppState) {
    let StudioAppState {
        scenario_library,
        sim_clock_transport,
        loading_cover_active,
        ..
    } = state;
    scenario_library.cancel(sim_clock_transport);
    // Cover clears once the poller sees the invalidated token and despawns pending geometry.
    // Keep true until that cleanup runs so a partial tree cannot flash.
    let _ = loading_cover_active;
}

fn request_live_bridge_reset_after_session_replacement(state: &mut StudioAppState) {
    crate::request_live_bridge_reset_after_session_replacement(
        &mut state.live_bridge_reset_requested,
    );
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
            ui.checkbox(&mut state.show_falloff_ruler, "Show falloff ruler");
            ui.checkbox(&mut state.show_aa_test_pattern, "Show AA test pattern");
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

fn draw_right_panel(
    ctx: &egui::Context,
    state: &mut StudioAppState,
    screen_w: f32,
    screen_h: f32,
    frosted_panels: &mut crate::FrostedGlassPanelRegistry,
) {
    let Some(session) = state.session.as_ref() else {
        return;
    };
    let width = 320.0;
    let (_, margin_y) = crate::panel_layout::panel_margin(screen_w, screen_h);
    let area = egui::Area::new(egui::Id::new("right_panel"))
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
    register_frosted_rect(frosted_panels, area.response.rect, screen_w, screen_h);
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

fn draw_generation_name_corpus_dialog(ctx: &egui::Context, state: &mut StudioAppState) {
    if !state.generation_name_dialog_visible {
        return;
    }

    egui::Window::new("Generate Galaxy — Star Names")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("Stellaris Star Names corpus:");
            ui.add(
                egui::TextEdit::singleline(&mut state.generation_name_corpus_path)
                    .desired_width(620.0),
            );
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    let path = state.generation_name_corpus_path.clone();
                    let choice = if star_name_corpus_path_is_importable(&path) {
                        GenerationNameCorpusChoice::Corpus(path)
                    } else {
                        // Import failed: fall back to exactly the Cancel effect rather
                        // than letting a bad path reach generation as a hard error.
                        GenerationNameCorpusChoice::Cancel
                    };
                    if apply_generation_name_corpus_choice(
                        &mut state.profile,
                        &mut state.generation_name_dialog_visible,
                        choice,
                    ) {
                        ctx.data_mut(|data| data.insert_temp(egui::Id::new("do_generate"), true));
                    }
                }
                if ui.button("None").clicked() {
                    if apply_generation_name_corpus_choice(
                        &mut state.profile,
                        &mut state.generation_name_dialog_visible,
                        GenerationNameCorpusChoice::None,
                    ) {
                        ctx.data_mut(|data| data.insert_temp(egui::Id::new("do_generate"), true));
                    }
                }
                if ui.button("Cancel").clicked() {
                    apply_generation_name_corpus_choice(
                        &mut state.profile,
                        &mut state.generation_name_dialog_visible,
                        GenerationNameCorpusChoice::Cancel,
                    );
                }
            });
        });
}
