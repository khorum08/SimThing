#![cfg(windows)]

mod camera;
mod galaxy_render;
mod resources;
mod ui;
mod window;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::generation::GenerationProfile;
use crate::session::StudioSession;
use crate::settings::EditorSettings;

use resources::{StudioDialog, StudioSettings};

pub fn run_studio() {
    let settings = EditorSettings::load();
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(StudioSettings(settings.clone()))
        .insert_resource(StudioAppState::default())
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
        .add_systems(EguiPrimaryContextPass, ui::studio_ui_system)
        .add_systems(
            Update,
            (
                ui::panel_opacity_system,
                camera::camera_control_system,
                camera::camera_hotkeys_system,
                galaxy_render::sync_hyperlane_colors_system,
                window::persist_settings_on_exit,
            ),
        )
        .run();
}

#[derive(Resource, Default)]
pub struct GalaxySceneRoot {
    pub stars: Vec<Entity>,
    pub hyperlanes: Option<Entity>,
    pub core_glow: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct StudioAppState {
    pub profile: GenerationProfile,
    pub session: Option<StudioSession>,
    pub left_panel_collapsed: bool,
    pub left_panel_width_frac: f32,
    pub left_panel_opacity: f32,
    pub left_panel_target_opacity: f32,
    pub left_panel_hovered: bool,
    pub generation_busy: bool,
    pub generation_error: Option<String>,
    pub status_message: String,
}

fn setup_scene(
    mut commands: Commands,
    settings: Res<StudioSettings>,
    mut state: ResMut<StudioAppState>,
) {
    state.profile = settings.last_generation_params.clone();
    state.left_panel_collapsed = settings.left_panel_collapsed;
    state.left_panel_width_frac = settings.last_panel_width.clamp(0.2, 0.5);
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
    session: StudioSession,
    settings: &mut EditorSettings,
    state: &mut StudioAppState,
) {
    settings.last_generation_params = session.profile.clone();
    state.profile = session.profile.clone();
    state.generation_error = None;
    state.status_message = format!(
        "Generated {} systems — quality {}",
        session.report().output.system_count,
        session.report().output.map_quality_status
    );
    state.session = Some(session);
}
