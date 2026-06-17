#![cfg(windows)]

mod camera;
mod galaxy_render;
mod picking;
mod resources;
mod ui;
mod window;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::dialog::SettingsDialogModel;
use crate::generation::GenerationProfile;
use crate::selection::StudioSelectionState;
use crate::session::StudioSession;
use crate::settings::EditorSettings;
use crate::star_render::{
    apply_star_falloff_settings_to_meta, apply_star_render_mode_to_meta, StarFalloffSettings,
    StarRenderMode,
};

use galaxy_render::{init_star_visual_assets, rebuild_galaxy_scene, StarVisualAssets};
use resources::{StudioDialog, StudioSettings};

use crate::panel_layout;

pub fn run_studio() {
    let settings = EditorSettings::load();
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(StudioSettings(settings.clone()))
        .insert_resource(StudioAppState::from_settings(&settings))
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
        .add_plugins(EguiPlugin::default())
        .add_plugins(camera::StudioCameraPlugin)
        .add_systems(Startup, (window::apply_initial_window_mode, setup_scene))
        .add_systems(Startup, init_star_visual_assets)
        .add_systems(EguiPrimaryContextPass, ui::studio_ui_system)
        .add_systems(
            Update,
            (
                ui::panel_opacity_system,
                camera::camera_control_system,
                camera::camera_hotkeys_system,
                picking::selection_keyboard_system,
                picking::star_pick_system,
                picking::sync_selection_highlight_system,
                picking::sync_star_visuals_system,
                picking::billboard_stars_system,
                galaxy_render::sync_hyperlane_colors_system,
                galaxy_render::sync_render_debug_visibility_system,
                window::persist_settings_on_exit,
            ),
        )
        .run();
}

#[derive(Resource, Default)]
pub struct GalaxySceneRoot {
    pub stars: Vec<(u32, Entity)>,
    pub hyperlane_buckets: [Option<Entity>; 3],
    pub highlight_hyperlanes: Option<Entity>,
    pub core_glow: Option<Entity>,
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
    pub status_message: String,
    pub show_stars: bool,
    pub show_hyperlanes: bool,
    pub settings_dialog: SettingsDialogModel,
    pub star_falloff_settings: StarFalloffSettings,
    pub star_render_mode: StarRenderMode,
}

impl StudioAppState {
    fn from_settings(settings: &EditorSettings) -> Self {
        let mut selection = StudioSelectionState::default();
        selection.selected_system_id = settings.last_selected_system_id;
        let star_falloff_settings = settings.star_falloff_settings();
        let star_render_mode = settings.star_render_mode();
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
            status_message: String::new(),
            show_stars: true,
            show_hyperlanes: true,
            settings_dialog: SettingsDialogModel::new(
                settings.settings_dialog_visible,
                settings.settings_dialog_position,
                star_falloff_settings,
                star_render_mode,
            ),
            star_falloff_settings,
            star_render_mode,
        }
    }
}

impl Default for StudioAppState {
    fn default() -> Self {
        Self::from_settings(&EditorSettings::default())
    }
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
    camera.apply_persisted(&settings.last_camera);
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
    settings.last_generation_params = session.profile.clone();
    state.profile = session.profile.clone();
    apply_star_falloff_settings_to_meta(
        &mut session.view_model.render_meta,
        state.star_falloff_settings,
    );
    apply_star_render_mode_to_meta(&mut session.view_model.render_meta, state.star_render_mode);
    state.generation_error = None;
    state.selection.clear();
    state.status_message = format!(
        "Generated {} systems — quality {}",
        session.report().output.system_count,
        session.report().output.map_quality_status
    );
    state.session = Some(session);
}

pub fn rebuild_session_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &StarVisualAssets,
    root: &mut GalaxySceneRoot,
    state: &mut StudioAppState,
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
    }
}
