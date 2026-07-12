#![cfg(windows)]

mod aa_test_pattern;
mod camera;
mod galaxy_render;
mod labels;
mod performance_telemetry;
mod picking;
mod resources;
pub mod scenario_io;
mod ui;
mod window;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::dialog::{SettingsDialogModel, TelemetryDialogModel};
use crate::generation::GenerationProfile;
use crate::hyperlane_buckets::{apply_hyperlane_render_settings_to_meta, HyperlaneRenderSettings};
use crate::selection::StudioSelectionState;
use crate::session::StudioSession;
use crate::settings::EditorSettings;
use crate::star_render::{
    apply_star_falloff_settings_to_meta, apply_star_render_mode_to_meta, StarFalloffSettings,
    StarRenderMode,
};
use crate::studio_config::{
    apply_studio_config_to_editor_settings, SimThingStudioConfig, StudioConfigLoadOutcome,
    StudioViewModeSetting,
};

use crate::studio_render_loop_dirty_gate::StudioRenderLoopCaches;
use galaxy_render::{
    init_star_visual_assets, mark_hyperlane_render_dirty, mark_star_visual_render_dirty,
    rebuild_galaxy_scene, StarVisualAssets,
};
use resources::{StudioDialog, StudioSettings};

use crate::panel_layout;

pub fn run_studio() {
    let mut settings = EditorSettings::load();
    let load_outcome = SimThingStudioConfig::load_at_startup();
    let config_load_warning = match &load_outcome {
        StudioConfigLoadOutcome::MissingDefaults => None,
        StudioConfigLoadOutcome::Loaded { config, warnings } => {
            apply_studio_config_to_editor_settings(config, &mut settings);
            if warnings.is_empty() {
                None
            } else {
                Some(format!(
                    "Studio config clamped values on load: {}",
                    warnings.join("; ")
                ))
            }
        }
        StudioConfigLoadOutcome::RejectedDefaults { reason } => Some(format!(
            "Studio config invalid; defaults loaded. ({reason})"
        )),
    };
    let mut app_state = StudioAppState::from_settings(&settings);
    if let StudioConfigLoadOutcome::Loaded { config, .. } = &load_outcome {
        app_state.show_stars = config.view.show_stars;
        app_state.show_hyperlanes = config.view.show_hyperlanes;
        app_state.config_view_mode = config.view.view_mode;
    } else if matches!(load_outcome, StudioConfigLoadOutcome::MissingDefaults) {
        // Seed stable JSON from validated RON/editor defaults so the next launch is consistent.
        let _ = save_current_studio_config(&app_state, &settings, None);
    }
    app_state.config_load_warning = config_load_warning;
    app_state.antialiasing_mode_source = match &load_outcome {
        StudioConfigLoadOutcome::Loaded { .. } => {
            crate::studio_antialiasing::StudioAntialiasingModeSource::LoadedStudioConfig
        }
        StudioConfigLoadOutcome::MissingDefaults
        | StudioConfigLoadOutcome::RejectedDefaults { .. } => {
            crate::studio_antialiasing::StudioAntialiasingModeSource::DefaultFallback
        }
    };
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(StudioSettings(settings.clone()))
        .insert_resource(app_state)
        .insert_non_send_resource(crate::StudioLiveSessionBridge::new())
        .insert_resource(StudioDialog::default())
        .insert_resource(GalaxySceneRoot::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window::primary_window_from_settings(&settings)),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "warn,simthing_mapeditor=info".into(),
                    ..default()
                }),
        )
        // No TonemappingLutFixPlugin: its GpuImage/FallbackImage-mutating "fix" broke egui compositing
        // (black screen, STUDIO-TYPEFACE-STARTUP-FIX-0R). The LUT D2/D3 mismatch only arises when text
        // renders through a tonemapping camera (the offscreen path), which the Studio does not mount;
        // the camera stays at the known-good default. The plugin-internal variant of that fix is now
        // gated off for live windows via SimthingToolsTextPlugin::without_lut_d3_view_fix().
        .add_plugins(EguiPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(performance_telemetry::StudioGpuIdentityInitPlugin)
        .add_plugins(camera::StudioCameraPlugin);
    // Live Studio text shares the primary Camera3d view through the transparent world-text phase.
    // The toolkit must not add a Camera2d or mutate tonemapping LUT resources in this process.
    crate::studio_typeface_shell::mount_studio_typeface_plugins(&mut app);
    app.add_systems(Startup, setup_scene)
        .add_systems(PostStartup, window::apply_initial_window_mode)
        .add_systems(Startup, init_star_visual_assets)
        .add_systems(Startup, init_studio_map_radius_falloff_state)
        .add_systems(
            Startup,
            (
                performance_telemetry::init_studio_performance_telemetry,
                init_render_loop_cache_state,
                init_aa_test_pattern_runtime,
            ),
        )
        .add_systems(EguiPrimaryContextPass, ui::studio_ui_system)
        .add_systems(Update, live_session_bridge_system)
        .add_systems(
            Update,
            (
                window::warn_missing_egui_viewport,
                performance_telemetry::begin_main_update_timing,
                (
                    ui::panel_opacity_system,
                    camera::camera_control_system,
                    camera::sync_studio_antialiasing_system,
                    aa_test_pattern::sync_aa_test_pattern_system,
                    performance_telemetry::update_map_radius_falloff_context_system,
                    camera::camera_hotkeys_system,
                    picking::selection_keyboard_system,
                    picking::star_pick_system,
                    picking::sync_selection_highlight_system,
                    galaxy_render::sync_star_visuals_system,
                    picking::billboard_stars_system,
                    galaxy_render::sync_star_nameplate_settings_system,
                    galaxy_render::sync_star_nameplate_focus_system,
                    galaxy_render::sync_render_debug_visibility_system,
                    performance_telemetry::update_studio_fps_telemetry,
                    performance_telemetry::update_nameplate_diagnostics_system,
                    performance_telemetry::update_studio_vram_telemetry,
                    performance_telemetry::update_studio_window_gpu_context,
                    performance_telemetry::update_studio_antialiasing_video_debug_system,
                    window::persist_settings_on_exit,
                ),
                performance_telemetry::finalize_main_update_timing,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            galaxy_render::sync_hyperlane_colors_system.after(camera::camera_control_system),
        )
        .run();
}

#[derive(Resource, Default)]
pub struct GalaxySceneRoot {
    pub stars: Vec<(u32, Entity)>,
    pub nameplates: Vec<Entity>,
    pub hyperlane_buckets: [Option<Entity>; 3],
    pub highlight_hyperlanes: Option<Entity>,
    pub core_glow: Option<Entity>,
}

#[derive(Resource, Clone, Copy, Default)]
pub struct StudioMapRadiusFalloffState {
    pub context: crate::falloff_metric::StudioMapRadiusFalloffContext,
    pub bounds: crate::falloff_metric::MapPlaneBounds,
    pub valid: bool,
    pub diagnostics: crate::falloff_metric::MapRadiusFalloffDiagnostics,
    pub context_frame: u64,
}

#[derive(Resource)]
pub struct StudioAppState {
    pub profile: GenerationProfile,
    pub session: Option<StudioSession>,
    pub selection: StudioSelectionState,
    pub left_panel_collapsed: bool,
    pub left_panel_width_frac: f32,
    pub left_panel_opacity: f32,
    pub left_panel_target_opacity: f32,
    pub left_panel_hovered: bool,
    pub generation_busy: bool,
    pub generation_error: Option<String>,
    /// Presentation-only modal gate shown before a generated galaxy is adopted.
    pub generation_name_dialog_visible: bool,
    /// Editable corpus path in the pre-generation naming dialog.
    pub generation_name_corpus_path: String,
    pub status_message: String,
    pub show_stars: bool,
    pub show_hyperlanes: bool,
    pub settings_dialog: SettingsDialogModel,
    pub telemetry_dialog: TelemetryDialogModel,
    /// Modal scenario library state (presentation only; never scenario authority).
    pub scenario_library: crate::StudioScenarioLibraryModel,
    pub star_falloff_settings: StarFalloffSettings,
    pub star_render_mode: StarRenderMode,
    pub hyperlane_render_settings: HyperlaneRenderSettings,
    pub config_load_warning: Option<String>,
    pub config_view_mode: StudioViewModeSetting,
    /// Editable scenario file path for Save/Load Scenario UI (presentation only).
    pub scenario_path_text: String,
    /// Last scenario IO status message (presentation only; not persisted in scenario authority).
    pub last_scenario_io_status: String,
    /// Last selected / displayed `.clause` path (presentation only).
    pub clause_path_text: String,
    /// Explicit resolver entries for ClauseScript open: `TOKEN=path` lines (presentation only).
    pub clause_resolver_text: String,
    /// Editable candidate artifact path for Save/Reopen Candidate UI (presentation only).
    pub candidate_path_text: String,
    /// Loaded scenario runtime/candidate status (presentation only; not authority).
    pub runtime_saveload_status: Option<crate::StudioScenarioRuntimeSaveLoadStatus>,
    /// Last runtime/candidate save-reopen status message (presentation only).
    pub last_runtime_saveload_status: String,
    /// Runtime save/load status must refresh before next draw (presentation only).
    pub runtime_saveload_status_dirty: bool,
    /// Cached authority digest from the last successful status refresh.
    pub runtime_saveload_status_source_digest: Option<u64>,
    /// True while the expensive proof/report refresh is running.
    pub runtime_saveload_status_refresh_in_progress: bool,
    /// Elapsed milliseconds for the last successful status refresh.
    pub runtime_saveload_status_last_refresh_ms: Option<u128>,
    /// Bumps on session adopt/scene rebuild; drives render-loop dirty gates (presentation only).
    pub scene_render_revision: u64,
    /// Performance diagnostic: hide main egui panels (presentation only).
    pub performance_diagnostic_hide_panels: bool,
    /// Performance diagnostic: skip camera input updates (presentation only).
    pub performance_diagnostic_freeze_camera: bool,
    /// Performance diagnostic: hide star aura layer entities (presentation only).
    pub performance_diagnostic_hide_star_aura: bool,
    /// Snapshot for Restore Normal Render (presentation only).
    pub performance_normal_render_snapshot:
        Option<crate::studio_frame_phase_gpu_telemetry::PerformanceNormalRenderSnapshot>,
    /// Nameplate LOD debug mode (presentation only).
    pub star_nameplate_debug_mode: crate::star_render::StarNameplateDebugMode,
    /// Star/nameplate falloff progress metric (presentation only).
    pub star_falloff_metric: crate::star_render::StarFalloffMetric,
    /// Diagnostic overlay: visual high-horizon falloff ruler (presentation only; default off).
    pub show_falloff_ruler: bool,
    /// Diagnostic overlay: 3D geometry-edge AA test pattern (presentation only; default off).
    pub show_aa_test_pattern: bool,
    /// Mutually exclusive post-process antialiasing mode (presentation only).
    pub antialiasing_mode: crate::studio_antialiasing::StudioAntialiasingMode,
    /// Where [`Self::antialiasing_mode`] was last set (telemetry/debug).
    pub antialiasing_mode_source: crate::studio_antialiasing::StudioAntialiasingModeSource,
    /// Studio sim clock transport (presentation projection over [`crate::StudioSimClock`]).
    /// Does not execute gameplay or mutate ScenarioSpec.
    pub sim_clock_transport: crate::StudioSimClockTransport,
    /// Send-safe snapshot of the live bridge (updated by NonSend bridge system).
    /// Full [`crate::StudioLiveSessionBridge`] is NonSend (holds SimSession).
    pub live_bridge_readout: crate::StudioLiveSessionBridgeReadout,
    /// One-frame request to detach live execution after replacing ScenarioSpec authority.
    pub live_bridge_reset_requested: bool,
}

impl StudioAppState {
    fn from_settings(settings: &EditorSettings) -> Self {
        let mut selection = StudioSelectionState::default();
        selection.selected_system_id = settings.last_selected_system_id;
        let star_falloff_settings = settings.star_falloff_settings();
        let star_render_mode = settings.star_render_mode();
        let hyperlane_render_settings = settings.hyperlane_render_settings();
        Self {
            profile: settings.last_generation_params.clone(),
            session: None,
            selection,
            left_panel_collapsed: settings.left_panel_collapsed,
            left_panel_width_frac: panel_layout::PANEL_WIDTH_FRAC,
            left_panel_opacity: 0.5,
            left_panel_target_opacity: 0.5,
            left_panel_hovered: false,
            generation_busy: false,
            generation_error: None,
            generation_name_dialog_visible: false,
            generation_name_corpus_path: ui::DEFAULT_STELLARIS_STAR_NAMES_CORPUS_PATH.to_string(),
            status_message: String::new(),
            show_stars: true,
            show_hyperlanes: true,
            settings_dialog: SettingsDialogModel::new(
                settings.settings_dialog_visible,
                settings.settings_dialog_position,
                star_falloff_settings,
                star_render_mode,
                hyperlane_render_settings,
            ),
            telemetry_dialog: TelemetryDialogModel::new(false, [480.0, 96.0]),
            scenario_library: crate::StudioScenarioLibraryModel::default(),
            star_falloff_settings,
            star_render_mode,
            hyperlane_render_settings,
            config_load_warning: None,
            config_view_mode: StudioViewModeSetting::ThreeD,
            scenario_path_text: scenario_io::DEFAULT_SCENARIO_PATH.to_string(),
            last_scenario_io_status: String::new(),
            clause_path_text: String::new(),
            clause_resolver_text: String::new(),
            candidate_path_text: "candidate.simthing-scenario.json".to_string(),
            runtime_saveload_status: None,
            last_runtime_saveload_status: String::new(),
            runtime_saveload_status_dirty: false,
            runtime_saveload_status_source_digest: None,
            runtime_saveload_status_refresh_in_progress: false,
            runtime_saveload_status_last_refresh_ms: None,
            scene_render_revision: 0,
            performance_diagnostic_hide_panels: false,
            performance_diagnostic_freeze_camera: false,
            performance_diagnostic_hide_star_aura: false,
            performance_normal_render_snapshot: None,
            star_nameplate_debug_mode: crate::star_render::StarNameplateDebugMode::default(),
            star_falloff_metric: crate::star_render::StarFalloffMetric::default(),
            show_falloff_ruler: false,
            show_aa_test_pattern: false,
            antialiasing_mode: settings.antialiasing_mode(),
            antialiasing_mode_source:
                crate::studio_antialiasing::StudioAntialiasingModeSource::DefaultFallback,
            sim_clock_transport: crate::StudioSimClockTransport::new(),
            live_bridge_readout: crate::StudioLiveSessionBridgeReadout::default_unattached(),
            live_bridge_reset_requested: false,
        }
    }

    pub(crate) fn bump_scene_render_revision(&mut self) {
        self.scene_render_revision = self.scene_render_revision.saturating_add(1);
    }

    pub(crate) fn runtime_saveload_status_cache_mut(
        &mut self,
    ) -> crate::RuntimeSaveloadStatusCacheMut<'_> {
        crate::RuntimeSaveloadStatusCacheMut {
            status: &mut self.runtime_saveload_status,
            dirty: &mut self.runtime_saveload_status_dirty,
            source_digest: &mut self.runtime_saveload_status_source_digest,
            refresh_in_progress: &mut self.runtime_saveload_status_refresh_in_progress,
            last_refresh_ms: &mut self.runtime_saveload_status_last_refresh_ms,
        }
    }

    pub(crate) fn mark_runtime_saveload_status_dirty(&mut self) {
        self.runtime_saveload_status_dirty = true;
    }

    pub(crate) fn refresh_runtime_saveload_status_if_needed(&mut self, force: bool) -> bool {
        let decision = crate::runtime_saveload_refresh_decision(
            self.session.is_some(),
            self.runtime_saveload_status_dirty,
            force,
            self.runtime_saveload_status_source_digest,
            None,
        );
        match decision {
            crate::RuntimeSaveloadRefreshDecision::Clear => {
                self.runtime_saveload_status = None;
                self.runtime_saveload_status_dirty = false;
                self.runtime_saveload_status_source_digest = None;
                self.runtime_saveload_status_refresh_in_progress = false;
                false
            }
            crate::RuntimeSaveloadRefreshDecision::UseCache => false,
            crate::RuntimeSaveloadRefreshDecision::Refresh => {
                self.runtime_saveload_status_refresh_in_progress = true;
                let started = std::time::Instant::now();
                let refresh_result = match self.session.as_ref() {
                    Some(session) => crate::refresh_runtime_saveload_status_from_session(
                        "studio_loaded_session",
                        &session.scenario_authority,
                    ),
                    None => {
                        self.runtime_saveload_status_refresh_in_progress = false;
                        self.runtime_saveload_status = None;
                        self.runtime_saveload_status_dirty = false;
                        self.runtime_saveload_status_source_digest = None;
                        return false;
                    }
                };
                let elapsed_ms = started.elapsed().as_millis();
                self.runtime_saveload_status_refresh_in_progress = false;
                self.runtime_saveload_status_last_refresh_ms = Some(elapsed_ms);
                match refresh_result {
                    Ok(status) => {
                        self.runtime_saveload_status_source_digest = status.loaded_scenario_digest;
                        self.runtime_saveload_status = Some(status);
                        self.runtime_saveload_status_dirty = false;
                        true
                    }
                    Err(_) => {
                        self.runtime_saveload_status_dirty = true;
                        false
                    }
                }
            }
        }
    }

    pub(crate) fn apply_refreshed_runtime_saveload_status(
        &mut self,
        status: crate::StudioScenarioRuntimeSaveLoadStatus,
        elapsed_ms: Option<u128>,
    ) {
        crate::apply_runtime_saveload_status_to_cache(
            self.runtime_saveload_status_cache_mut(),
            status,
            elapsed_ms,
        );
    }
}

pub(crate) fn view_mode_setting_from_camera(
    camera: &camera::StudioCamera,
) -> StudioViewModeSetting {
    match camera.view_mode() {
        camera::StudioViewMode::ThreeD => StudioViewModeSetting::ThreeD,
        camera::StudioViewMode::OverheadStrategic => StudioViewModeSetting::OverheadStrategic,
    }
}

pub(crate) fn save_current_studio_config(
    state: &StudioAppState,
    settings: &EditorSettings,
    camera: Option<&camera::StudioCamera>,
) -> Result<(), crate::studio_config::StudioConfigError> {
    let view_mode = camera
        .map(view_mode_setting_from_camera)
        .unwrap_or(state.config_view_mode);
    let camera_state = camera
        .map(camera::StudioCamera::to_persisted)
        .or_else(|| Some(settings.last_camera));
    let config = SimThingStudioConfig::from_presentation_state(
        state.settings_dialog.visible,
        state.settings_dialog.position,
        state.star_falloff_settings,
        state.star_render_mode,
        settings.star_nameplate_settings(),
        state.hyperlane_render_settings,
        state.show_stars,
        state.show_hyperlanes,
        view_mode,
        state.antialiasing_mode,
        camera_state,
    );
    config.save_to_default_path()
}

/// Persist presentation sliders/dialog state to both RON and JSON stores.
pub(crate) fn persist_presentation_settings(
    state: &StudioAppState,
    settings: &mut EditorSettings,
    camera: Option<&camera::StudioCamera>,
) {
    settings.set_star_falloff_settings(state.star_falloff_settings);
    settings.set_star_render_mode(state.star_render_mode);
    settings.set_hyperlane_render_settings(state.hyperlane_render_settings);
    settings.set_antialiasing_mode(state.antialiasing_mode);
    settings.settings_dialog_position = state.settings_dialog.position;
    settings.settings_dialog_visible = state.settings_dialog.visible;
    if let Err(err) = settings.save() {
        bevy::log::warn!("failed to save editor settings: {err}");
    }
    if let Err(err) = save_current_studio_config(state, settings, camera) {
        bevy::log::warn!("failed to save studio config: {err}");
    }
}

impl Default for StudioAppState {
    fn default() -> Self {
        Self::from_settings(&EditorSettings::default())
    }
}

fn init_render_loop_cache_state(mut commands: Commands) {
    commands.init_resource::<StudioRenderLoopCaches>();
}

fn init_aa_test_pattern_runtime(mut commands: Commands) {
    commands.init_resource::<crate::studio_aa_test_pattern::AaTestPatternRuntime>();
}

fn init_studio_map_radius_falloff_state(mut commands: Commands) {
    commands.init_resource::<StudioMapRadiusFalloffState>();
}

/// Drive production live bridge from wall elapsed + StudioSimClock (not frame-count authority).
///
/// Bevy `Time` supplies wall `delta_secs` for schedule demand only; tick count authority is
/// [`crate::StudioSimClock::advance`]. Bridge is NonSend because `SimSession` is !Sync.
fn live_session_bridge_system(
    mut bridge: NonSendMut<crate::StudioLiveSessionBridge>,
    mut state: ResMut<StudioAppState>,
    time: Res<Time>,
) {
    if crate::apply_live_bridge_reset_before_tick(
        &mut state.live_bridge_reset_requested,
        &mut bridge,
    ) {
        state.live_bridge_readout = bridge.readout();
    }
    let StudioAppState {
        scenario_library,
        sim_clock_transport,
        ..
    } = &mut *state;
    scenario_library.enforce_pause(sim_clock_transport);
    let elapsed = time.delta_secs_f64();
    if !elapsed.is_finite() || elapsed <= 0.0 {
        state.live_bridge_readout = bridge.readout();
        return;
    }
    let StudioAppState {
        session,
        sim_clock_transport,
        live_bridge_readout,
        ..
    } = &mut *state;
    let clock = sim_clock_transport.clock_mut();
    let _ = bridge.tick_from_clock(clock, session.as_ref(), elapsed);
    *live_bridge_readout = bridge.readout();
}

fn setup_scene(
    mut commands: Commands,
    settings: Res<StudioSettings>,
    mut state: ResMut<StudioAppState>,
    mut camera: ResMut<camera::StudioCamera>,
) {
    state.profile = settings.last_generation_params.clone();
    state.left_panel_collapsed = settings.left_panel_collapsed;
    state.left_panel_width_frac = panel_layout::PANEL_WIDTH_FRAC;
    if let Some(warning) = state.config_load_warning.clone() {
        state.status_message = warning;
    }
    camera.apply_persisted(&settings.last_camera);
    camera.apply_loaded_view_mode(state.config_view_mode);
    commands.spawn((
        DirectionalLight {
            illuminance: 800.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.55, 0.95),
            intensity: 1200.0,
            ..default()
        },
        Transform::from_xyz(0.0, 40.0, 0.0),
    ));
}

pub fn adopt_session(
    mut session: StudioSession,
    settings: &mut EditorSettings,
    state: &mut StudioAppState,
) {
    state.last_scenario_io_status.clear();
    let status_message = session.status_message();
    adopt_session_with_status(&mut session, settings, state, status_message);
    state.session = Some(session);
}

pub fn adopt_loaded_scenario_session(
    mut session: StudioSession,
    settings: &mut EditorSettings,
    state: &mut StudioAppState,
    status_message: String,
) {
    adopt_session_with_status(&mut session, settings, state, status_message.clone());
    state.last_scenario_io_status = status_message;
    state.session = Some(session);
}

fn adopt_session_with_status(
    session: &mut StudioSession,
    settings: &mut EditorSettings,
    state: &mut StudioAppState,
    status_message: String,
) {
    let profile = session.profile();
    settings.last_generation_params = profile.clone();
    state.profile = profile;
    apply_star_falloff_settings_to_meta(
        &mut session.view_model.render_meta,
        state.star_falloff_settings,
    );
    apply_star_render_mode_to_meta(&mut session.view_model.render_meta, state.star_render_mode);
    apply_hyperlane_render_settings_to_meta(
        &mut session.view_model.render_meta,
        state.hyperlane_render_settings,
    );
    state.generation_error = None;
    state.selection.clear();
    state.status_message = status_message;
    state.mark_runtime_saveload_status_dirty();
    state.bump_scene_render_revision();
}

pub fn rebuild_session_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &StarVisualAssets,
    root: &mut GalaxySceneRoot,
    state: &mut StudioAppState,
    caches: &mut StudioRenderLoopCaches,
) {
    if let Some(session) = state.session.as_ref() {
        rebuild_galaxy_scene(commands, meshes, materials, assets, root, session);
        galaxy_render::rebuild_highlight_hyperlanes(
            commands,
            meshes,
            materials,
            root,
            session,
            state.selection.selected_system_id,
        );
        mark_hyperlane_render_dirty(&mut caches.hyperlane);
        mark_star_visual_render_dirty(&mut caches.star_visual);
        caches.picking.last_key = None;
        caches.picking.cached_projections.clear();
        caches.billboard.last_camera_key = None;
    }
}
