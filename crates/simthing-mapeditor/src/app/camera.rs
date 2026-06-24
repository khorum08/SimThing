#![cfg(windows)]

use bevy::diagnostic::FrameCount;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::view::Msaa;

use crate::camera_control::{
    apply_orbit_delta, apply_scroll_zoom, reset_camera_after_generation as reset_orbit,
    snap_overhead as snap_orbit, OrbitCameraState, DEFAULT_ORBIT_SENSITIVITY,
};
use crate::settings::PersistedCameraState;
use crate::studio_config::StudioViewModeSetting;

pub struct StudioCameraPlugin;

impl Plugin for StudioCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StudioCamera::default())
            .add_systems(Startup, spawn_camera);
    }
}

#[derive(Resource)]
pub struct StudioCamera {
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub orbit_distance: f32,
    pub orbit_target: Vec3,
    pub overhead: bool,
    pub move_speed: f32,
    pub rmb_held: bool,
    view_mode: StudioViewMode,
    saved_three_d_state: Option<OrbitCameraState>,
    saved_overhead_state: Option<OrbitCameraState>,
}

impl Default for StudioCamera {
    fn default() -> Self {
        let orbit = OrbitCameraState::default();
        Self {
            orbit_yaw: orbit.orbit_yaw,
            orbit_pitch: orbit.orbit_pitch,
            orbit_distance: orbit.orbit_distance,
            orbit_target: Vec3::from_array(orbit.orbit_target),
            overhead: orbit.overhead,
            move_speed: 40.0,
            rmb_held: false,
            view_mode: StudioViewMode::ThreeD,
            saved_three_d_state: None,
            saved_overhead_state: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioViewMode {
    ThreeD,
    OverheadStrategic,
}

impl StudioViewMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::ThreeD => "3D",
            Self::OverheadStrategic => "Strategic overhead",
        }
    }

    pub fn hyperlane_render_path(self) -> HyperlaneRibbonRenderPath {
        match self {
            Self::ThreeD => HyperlaneRibbonRenderPath::CameraFacing3D,
            Self::OverheadStrategic => HyperlaneRibbonRenderPath::OverheadLegibility,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlaneRibbonRenderPath {
    CameraFacing3D,
    OverheadLegibility,
}

pub fn toggle_studio_view_mode(mode: StudioViewMode) -> StudioViewMode {
    match mode {
        StudioViewMode::ThreeD => StudioViewMode::OverheadStrategic,
        StudioViewMode::OverheadStrategic => StudioViewMode::ThreeD,
    }
}

impl StudioCamera {
    fn to_orbit_state(&self) -> OrbitCameraState {
        OrbitCameraState {
            orbit_yaw: self.orbit_yaw,
            orbit_pitch: self.orbit_pitch,
            orbit_distance: self.orbit_distance,
            orbit_target: self.orbit_target.to_array(),
            overhead: self.overhead,
        }
    }

    fn apply_orbit_state(&mut self, state: OrbitCameraState) {
        self.orbit_yaw = state.orbit_yaw;
        self.orbit_pitch = state.orbit_pitch;
        self.orbit_distance = state.orbit_distance;
        self.orbit_target = Vec3::from_array(state.orbit_target);
        self.overhead = state.overhead;
    }

    pub fn view_mode(&self) -> StudioViewMode {
        self.view_mode
    }

    pub fn toggle_view_mode(&mut self) {
        match self.view_mode {
            StudioViewMode::ThreeD => {
                self.saved_three_d_state = Some(self.to_orbit_state());
                self.view_mode = toggle_studio_view_mode(self.view_mode);
                let mut state = self
                    .saved_overhead_state
                    .unwrap_or_else(|| strategic_overhead_state(self.orbit_target.to_array()));
                state.orbit_target = self.orbit_target.to_array();
                self.apply_orbit_state(state);
            }
            StudioViewMode::OverheadStrategic => {
                self.saved_overhead_state = Some(self.to_orbit_state());
                self.view_mode = toggle_studio_view_mode(self.view_mode);
                let state = self.saved_three_d_state.unwrap_or_default();
                self.apply_orbit_state(state);
            }
        }
    }

    pub fn apply_persisted(&mut self, persisted: &PersistedCameraState) {
        self.apply_orbit_state(OrbitCameraState::from(*persisted));
        self.view_mode = StudioViewMode::ThreeD;
        self.saved_three_d_state = None;
        self.saved_overhead_state = None;
    }

    pub fn apply_loaded_view_mode(&mut self, mode: StudioViewModeSetting) {
        match mode {
            StudioViewModeSetting::ThreeD => {
                self.view_mode = StudioViewMode::ThreeD;
            }
            StudioViewModeSetting::OverheadStrategic => {
                self.view_mode = StudioViewMode::OverheadStrategic;
                self.apply_orbit_state(strategic_overhead_state(self.orbit_target.to_array()));
            }
        }
    }

    pub fn to_persisted(&self) -> PersistedCameraState {
        PersistedCameraState::from(&self.to_orbit_state())
    }
}

pub fn strategic_overhead_state(target: [f32; 3]) -> OrbitCameraState {
    OrbitCameraState {
        orbit_yaw: 0.0,
        orbit_pitch: std::f32::consts::FRAC_PI_2 - 0.001,
        orbit_distance: 180.0,
        orbit_target: target,
        overhead: true,
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(40.0, 35.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
        MainCamera,
    ));
}

pub fn reset_camera_after_generation(camera: &mut StudioCamera) {
    let mut state = camera.to_orbit_state();
    reset_orbit(&mut state);
    camera.apply_orbit_state(state);
    camera.view_mode = StudioViewMode::ThreeD;
    camera.saved_three_d_state = None;
    camera.saved_overhead_state = None;
}

pub fn snap_overhead(camera: &mut StudioCamera) {
    let mut state = camera.to_orbit_state();
    snap_orbit(&mut state);
    camera.apply_orbit_state(state);
    camera.view_mode = StudioViewMode::ThreeD;
}

pub fn camera_hotkeys_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<StudioCamera>,
) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        snap_overhead(&mut camera);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        reset_camera_after_generation(&mut camera);
    }
    if keyboard.just_pressed(KeyCode::Tab) {
        camera.toggle_view_mode();
    }
}

pub fn camera_control_system(
    app_state: Res<super::StudioAppState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll: EventReader<MouseWheel>,
    mut camera: ResMut<StudioCamera>,
    mut transforms: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    if app_state.performance_diagnostic_freeze_camera {
        return;
    }
    camera.rmb_held = mouse.pressed(MouseButton::Right);
    let dt = time.delta_secs();
    let forward = Vec3::new(-camera.orbit_yaw.sin(), 0.0, -camera.orbit_yaw.cos());
    let right = Vec3::new(forward.z, 0.0, -forward.x);
    let mut delta = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        delta += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        delta -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        delta -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        delta += right;
    }
    if delta.length_squared() > 0.0 {
        let speed = camera.move_speed;
        camera.orbit_target += delta.normalize() * speed * dt;
    }

    if camera.rmb_held {
        let mut motion = Vec2::ZERO;
        for ev in mouse_motion.read() {
            motion += ev.delta;
        }
        let mut state = camera.to_orbit_state();
        apply_orbit_delta(&mut state, motion.x, motion.y, DEFAULT_ORBIT_SENSITIVITY);
        camera.apply_orbit_state(state);
        if camera.view_mode == StudioViewMode::OverheadStrategic {
            camera.overhead = true;
            camera.orbit_pitch = std::f32::consts::FRAC_PI_2 - 0.001;
        }
    } else {
        for _ in mouse_motion.read() {}
    }

    for ev in scroll.read() {
        camera.orbit_distance = apply_scroll_zoom(camera.orbit_distance, ev.y);
    }

    let pitch = if camera.overhead {
        std::f32::consts::FRAC_PI_2 - 0.001
    } else {
        camera.orbit_pitch
    };
    let yaw = camera.orbit_yaw;
    let dist = camera.orbit_distance;
    let target = camera.orbit_target;
    let offset = Vec3::new(
        dist * pitch.cos() * yaw.sin(),
        dist * pitch.sin(),
        dist * pitch.cos() * yaw.cos(),
    );
    for mut transform in &mut transforms {
        transform.translation = target + offset;
        transform.look_at(target, Vec3::Y);
    }
}

pub fn sync_studio_antialiasing_system(
    app_state: Res<super::StudioAppState>,
    mut last_applied: Local<Option<crate::studio_antialiasing::StudioAntialiasingMode>>,
    camera: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
    mut apply_state: ResMut<crate::studio_antialiasing::StudioAntialiasingApplyState>,
    frame: Res<FrameCount>,
) {
    let mode = app_state.antialiasing_mode;
    if last_applied
        .map(|previous| previous == mode)
        .unwrap_or(false)
    {
        return;
    }
    let Ok(entity) = camera.single() else {
        return;
    };
    crate::studio_antialiasing::apply_and_record_studio_antialiasing_mode(
        &mut commands.entity(entity),
        mode,
        &mut apply_state,
        frame.0.into(),
    );
    *last_applied = Some(mode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hyperlane_buckets::{compute_hyperlane_visual, HyperlaneRenderSettings};

    #[test]
    fn tab_toggles_view_mode_between_three_d_and_overhead() {
        assert_eq!(
            toggle_studio_view_mode(StudioViewMode::ThreeD),
            StudioViewMode::OverheadStrategic
        );
        assert_eq!(
            toggle_studio_view_mode(StudioViewMode::OverheadStrategic),
            StudioViewMode::ThreeD
        );
        let mut camera = StudioCamera::default();
        assert_eq!(camera.view_mode(), StudioViewMode::ThreeD);
        camera.toggle_view_mode();
        assert_eq!(camera.view_mode(), StudioViewMode::OverheadStrategic);
        assert!(camera.overhead);
        camera.toggle_view_mode();
        assert_eq!(camera.view_mode(), StudioViewMode::ThreeD);
    }

    #[test]
    fn overhead_mode_uses_legibility_render_path() {
        assert_eq!(
            StudioViewMode::OverheadStrategic.hyperlane_render_path(),
            HyperlaneRibbonRenderPath::OverheadLegibility
        );
        assert_eq!(
            StudioViewMode::ThreeD.hyperlane_render_path(),
            HyperlaneRibbonRenderPath::CameraFacing3D
        );
    }

    #[test]
    fn settings_hyperlane_sliders_affect_three_d_mode() {
        let mode = StudioViewMode::ThreeD;
        let base = compute_hyperlane_visual(0.0, 10.0, &HyperlaneRenderSettings::default(), true);
        let adjusted = compute_hyperlane_visual(
            0.0,
            10.0,
            &HyperlaneRenderSettings {
                base_thickness_percent_of_star: 16.0,
                base_opacity_percent: 40.0,
                ..Default::default()
            },
            true,
        );
        assert_eq!(
            mode.hyperlane_render_path(),
            HyperlaneRibbonRenderPath::CameraFacing3D
        );
        assert!(adjusted.thickness_world > base.thickness_world);
        assert!(adjusted.core_opacity < base.core_opacity);
    }

    #[test]
    fn settings_hyperlane_sliders_affect_overhead_mode() {
        let mode = StudioViewMode::OverheadStrategic;
        let settings = HyperlaneRenderSettings {
            falloff_distance_percent: 50.0,
            ..Default::default()
        };
        let base = compute_hyperlane_visual(100.0, 10.0, &settings, true);
        let adjusted = compute_hyperlane_visual(
            100.0,
            10.0,
            &HyperlaneRenderSettings {
                falloff_distance_percent: 50.0,
                falloff_thickness_percent: 5.0,
                falloff_opacity_percent: 5.0,
                ..Default::default()
            },
            true,
        );
        assert_eq!(
            mode.hyperlane_render_path(),
            HyperlaneRibbonRenderPath::OverheadLegibility
        );
        assert!(adjusted.thickness_world < base.thickness_world);
        assert!(adjusted.core_opacity < base.core_opacity);
    }
}
