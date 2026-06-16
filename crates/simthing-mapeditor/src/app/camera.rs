#![cfg(windows)]

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use crate::camera_control::{
    apply_orbit_delta, reset_camera_after_generation as reset_orbit, snap_overhead as snap_orbit,
    OrbitCameraState, DEFAULT_ORBIT_SENSITIVITY,
};
use crate::settings::PersistedCameraState;

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
        }
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

    pub fn apply_persisted(&mut self, persisted: &PersistedCameraState) {
        self.apply_orbit_state(OrbitCameraState::from(*persisted));
    }

    pub fn to_persisted(&self) -> PersistedCameraState {
        PersistedCameraState::from(&self.to_orbit_state())
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(40.0, 35.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}

pub fn reset_camera_after_generation(camera: &mut StudioCamera) {
    let mut state = camera.to_orbit_state();
    reset_orbit(&mut state);
    camera.apply_orbit_state(state);
}

pub fn snap_overhead(camera: &mut StudioCamera) {
    let mut state = camera.to_orbit_state();
    snap_orbit(&mut state);
    camera.apply_orbit_state(state);
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
}

pub fn camera_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll: EventReader<MouseWheel>,
    mut camera: ResMut<StudioCamera>,
    mut transforms: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
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
    } else {
        for _ in mouse_motion.read() {}
    }

    for ev in scroll.read() {
        camera.orbit_distance = (camera.orbit_distance - ev.y * 4.0).clamp(25.0, 220.0);
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
